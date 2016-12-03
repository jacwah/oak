//! This modules contains various file filters and abstractions for working with them.

extern crate git2;

use std::path::Path;
use self::git2::Repository;
use std::result;
use std::error::Error;

type Result = result::Result<bool, Box<Error>>;

/// A filter used to decide whether to include a file in a collection.
pub trait FileFilter {
    /// `Ok(true)` means the file should be included and vice versa.
    fn filter(&self, path: &Path) -> Result;
}

impl<F> FileFilter for F
    where F: Fn(&Path) -> Result
{
    fn filter(&self, path: &Path) -> Result {
        (self)(path)
    }
}

/// A collection of filters acting as one.
pub struct FilterAggregate {
    filters: Vec<Box<FileFilter>>,
}

impl FilterAggregate {
    /// Add a filter to the collection.
    pub fn push<F>(&mut self, filter: F)
        where F: FileFilter + 'static
    {
        self.filters.push(Box::new(filter));
    }
}

impl Default for FilterAggregate {
    fn default() -> Self {
        FilterAggregate { filters: Vec::new() }
    }
}

impl FileFilter for FilterAggregate {
    fn filter(&self, path: &Path) -> Result {
        for f in &self.filters {
            if !try!(f.filter(path)) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// Exclude files ignored by git.
pub struct GitignoreFilter {
    repo: Repository,
}

impl GitignoreFilter {
    /// Create a new filter rooted at `path`.
    pub fn new(path: &Path) -> result::Result<Self, git2::Error> {
        Repository::discover(path)
            .map(|repo| GitignoreFilter { repo: repo })
    }
}

impl FileFilter for GitignoreFilter {
    fn filter(&self, path: &Path) -> Result {
        // ./filename paths doesn't seem to work with should_ignore
        let path = try!(path.canonicalize());
        self.repo.status_should_ignore(&path)
            .map(|x| !x)
            .map_err(From::from)
    }
}

/// Exclude hidden files.
///
/// This function relies on the Unix convention of denoting hidden files with a leading dot (`.`).
pub fn filter_hidden_files(path: &Path) -> Result {
    path.file_name()
        .and_then(|name| {
            name.to_str()
                .map(|str| !str.starts_with('.'))
            })
        .ok_or_else(|| From::from("No file name."))
}

/// Exclude non directory files.
pub fn filter_non_dirs(path: &Path) -> Result {
    path.metadata()
        .map(|data| data.is_dir())
        .map_err(From::from)
}
