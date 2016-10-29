extern crate git2;

use std::path::Path;
use std::os::unix::ffi::OsStrExt;
use self::git2::Repository;

pub struct GitignoreFilter {
    repo: Repository,
}

impl GitignoreFilter {
    pub fn new(path: &Path) -> Result<Self, git2::Error> {
        match Repository::discover(path) {
            Ok(repo) => Ok(GitignoreFilter { repo: repo }),
            Err(err) => Err(err),
        }
    }

    pub fn filter(&self, path: &Path) -> bool {
        // ./filename paths doesn't seem to work with should_ignore
        let path = path.canonicalize().unwrap();
        match self.repo.status_should_ignore(&path) {
            Ok(result) => !result,
            Err(_) => false,
        }
    }
}

pub fn filter_hidden_files(path: &Path) -> bool {
    // Is this implementation sound?
    static DOT: u8 = '.' as u8;
    let maybe_name = path.file_name();

    match maybe_name {
        Some(name) => name.as_bytes()[0] != DOT,
        _ => false,
    }
}

pub fn filter_non_dirs(path: &Path) -> bool {
    match path.metadata() {
        Ok(data) => data.is_dir(),
        Err(_) => false,
    }
}

