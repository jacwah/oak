use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::iter::Peekable;
use super::filters::FileFilter;

#[derive(Debug)]
pub struct Entry {
    path: PathBuf,
    has_next_sibling: bool,
}

/// An iterator yielding only the entries in dir where `file_filter` returns true.
struct FilteredDir {
    file_filter: Box<FileFilter>,
    dir: fs::ReadDir,
}

impl Iterator for FilteredDir {
    type Item = Result<fs::DirEntry, Box<Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = match self.dir.next() {
                Some(result) => result,
                None => return None,
            };

            let entry = match result {
                Ok(entry) => entry,
                Err(err) => return Some(Err(From::from(err))),
            };

            let should_yield = match self.file_filter.filter(entry.path().as_path()) {
                Ok(should_yield) => should_yield,
                Err(err) => return Some(Err(From::from(err))),
            };

            if should_yield {
                return Some(Ok(entry));
            }
        }
    }
}

pub struct TreeIter {
    dir_stack: Vec<Peekable<FilteredDir>>,
}

impl TreeIter {
    pub fn new<P, F>(path: P, file_filter: F) -> Result<Self, Box<Error>> where
        P: AsRef<Path>,
        F: FileFilter + 'static
    {
        fs::read_dir(path)
            .map(|dir| {
                let filtered = FilteredDir {
                    file_filter: Box::new(file_filter),
                    dir: dir,
                };
                TreeIter { dir_stack: vec![filtered.peekable()] }
            })
            .map_err(From::from)
    }
}

fn has_next_sibling<T, E, I: Iterator<Item=Result<T, E>>>(dir: &mut Peekable<I>) -> bool {
    loop {
        match dir.peek() {
            Some(result) => {
                if result.is_ok() {
                    return true;
                }
            },
            None => {
                return false;
            }
        }
    }
}

impl Iterator for TreeIter {
    type Item = Result<Entry, Box<Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        // recurse etc
        if let Some(current_dir) = self.dir_stack.as_mut_slice().last_mut() {
            current_dir.next()
                .map(|result| {
                    result.map(|entry| Entry {
                        path: entry.path(),
                        has_next_sibling: has_next_sibling(current_dir),
                     })
                    .map_err(From::from)
                })
        } else {
            None
        }
    }
}

pub fn process<F: FileFilter + 'static>(dir: &Path, file_filter: F) -> Result<(), Box<Error>> {
    for entry in try!(TreeIter::new(dir, file_filter)) {
        println!("{:?}", try!(entry));
    }
    Ok(())
}
