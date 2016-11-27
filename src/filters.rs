extern crate git2;

use std::path::Path;
use self::git2::Repository;
use std::result;
use std::error::Error;

type Result = result::Result<bool, Box<Error>>;

pub trait FileFilter {
    fn filter(&self, path: &Path) -> Result;
}

impl<F> FileFilter for F
    where F: Fn(&Path) -> Result
{
    fn filter(&self, path: &Path) -> Result {
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
    fn filter(&self, path: &Path) -> Result {
        for f in self.filters.iter() {
            if try!(f.filter(path)) == false {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

pub struct GitignoreFilter {
    repo: Repository,
}

impl GitignoreFilter {
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
            .map_err(|err| From::from(err))
    }
}

pub fn filter_hidden_files(path: &Path) -> Result {
    path.file_name()
        .and_then(|name| {
            name.to_str()
                .map(|str| !str.starts_with("."))
            })
        .ok_or(From::from("No file name."))
}

pub fn filter_non_dirs(path: &Path) -> Result {
    path.metadata()
        .map(|data| data.is_dir())
        .map_err(|err| From::from(err))
}

