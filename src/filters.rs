extern crate git2;

use std::path::Path;
use self::git2::Repository;

pub trait FileFilter {
    fn filter(&self, path: &Path) -> bool;
}

impl<F> FileFilter for F
    where F: Fn(&Path) -> bool
{
    fn filter(&self, path: &Path) -> bool {
        (self)(path)
    }
}

pub struct FilterAggregate {
    filters: Vec<Box<FileFilter>>,
}

impl FilterAggregate {
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
    fn filter(&self, path: &Path) -> bool {
        self.filters.iter().all(|f| f.filter(path))
    }
}

pub struct GitignoreFilter {
    repo: Repository,
}

impl GitignoreFilter {
    pub fn new(path: &Path) -> Result<Self, git2::Error> {
        Repository::discover(path)
            .map(|repo| GitignoreFilter { repo: repo })
    }
}

impl FileFilter for GitignoreFilter {
    fn filter(&self, path: &Path) -> bool {
        // ./filename paths doesn't seem to work with should_ignore
        let path = path.canonicalize().expect("Failed to canonicalize path");
        !self.repo.status_should_ignore(&path).unwrap_or(true)
    }
}

pub fn filter_hidden_files(path: &Path) -> bool {
    // Default to not filter if filename can't be retrieved or converted to utf-8
    path.file_name()
        .map(|name| {
            name.to_str()
                .map(|str| !str.starts_with("."))
                .unwrap_or(true)
            })
        .unwrap_or(true)
}

pub fn filter_non_dirs(path: &Path) -> bool {
    path.metadata()
        .map(|data| data.is_dir())
        .unwrap_or(false)
}

