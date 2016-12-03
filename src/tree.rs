use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::iter::Peekable;
use std::rc::Rc;
use std::fmt;
use super::filters::FileFilter;

/// Represents an entry in the file system.
pub struct Entry {
    path: PathBuf,
    /// Whether the iterator that yielded this entry has more sibling (same directory) entries.
    has_next_sibling: bool,
    /// A cached metadata entry for this file. It's probably better to use this than
    /// calling `fs::metadata` on `path`.
    metadata: fs::Metadata,
}

impl Entry {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn has_next_sibling(&self) -> bool {
        self.has_next_sibling
    }

    pub fn metadata(&self) -> &fs::Metadata {
        &self.metadata
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.debug_struct("Entry")
            .field("path", &self.path)
            .field("has_next_sibling", &self.has_next_sibling)
            .field("is_dir", &self.metadata.is_dir())
            .finish()
    }
}

/// An iterator yielding only the entries in dir where `file_filter` returns true.
struct FilteredDir {
    file_filter: Rc<FileFilter>,
    dir: fs::ReadDir,
}

impl FilteredDir {
    pub fn new<P>(path: P, file_filter: Rc<FileFilter>) -> Result<Self, Box<Error>> where
        P: AsRef<Path>,
    {
        fs::read_dir(path)
            .map(|dir| {
                FilteredDir {
                    file_filter: file_filter,
                    dir: dir,
                }
            })
            .map_err(From::from)
    }
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

/// A filtered recursive directory iterator.
///
/// The iterator descends the tree depth first. This means that all of a directory's children
/// will immediately follow thier parent. This essentially mirrors the output of this program.
///
/// # Example
/// Given the following directory structure, the items would be returned from `TreeIter` in the
/// same order.
///
/// ```text
/// .
/// ├── a
/// ├── b
/// │   ├── 1
/// │   └── 2
/// ├── c
/// ├── d
/// │   ├── 1
/// │   └── 2
/// └── e
/// ```
pub struct TreeIter {
    dir_stack: Vec<Peekable<FilteredDir>>,
    file_filter: Rc<FileFilter>,
}

impl TreeIter {
    /// Create a new iterator with `path` as root.
    pub fn new<P, F>(path: P, file_filter: F) -> Result<Self, Box<Error>> where
        P: AsRef<Path>,
        F: FileFilter + 'static
    {
        let rc_filter = Rc::new(file_filter);

        fs::read_dir(path)
            .map(|dir| {
                let filtered = FilteredDir {
                    file_filter: rc_filter.clone(),
                    dir: dir,
                };
                TreeIter {
                    dir_stack: vec![filtered.peekable()],
                    file_filter: rc_filter,
                }
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

fn next_entry(dir: &mut Peekable<FilteredDir>) -> Option<Result<Entry, Box<Error>>> {
    let entry = match dir.next() {
        Some(Ok(entry)) => entry,
        Some(Err(err)) => return Some(Err(From::from(err))),
        None => return None,
    };

    let has_next_sibling = has_next_sibling(dir);
    let metadata = match entry.metadata() {
        Ok(metadata) => metadata,
        Err(err) => return Some(Err(From::from(err))),
    };
    let path = entry.path();

    Some(Ok(Entry {
        path: path,
        metadata: metadata,
        has_next_sibling: has_next_sibling,
    }))
}

impl Iterator for TreeIter {
    type Item = Result<Entry, Box<Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry;

        loop {
            let mut should_pop = false;

            match self.dir_stack.as_mut_slice().last_mut() {
                Some(dir) => {
                    match next_entry(dir) {
                        Some(Ok(the_entry)) => {
                            entry = the_entry;
                            break;
                        },
                        Some(Err(err)) => return Some(Err(err)),
                        None => {
                            // Top dir is empty, go down a level
                            should_pop = true;
                        },
                    }
                },
                // We reached top of dir stack
                None => return None,
            };

            if should_pop {
                self.dir_stack.pop();
            }
        };

        if entry.metadata.is_dir() {
            match FilteredDir::new(&entry.path, self.file_filter.clone()) {
                Ok(dir) => self.dir_stack.push(dir.peekable()),
                Err(err) => return Some(Err(From::from(err))),
            };
        };

        Some(Ok(entry))
    }
}

pub fn process<F: FileFilter + 'static>(dir: &Path, file_filter: F) -> Result<(), Box<Error>> {
    for entry in try!(TreeIter::new(dir, file_filter)) {
        println!("{:?}", try!(entry));
    }
    Ok(())
}
