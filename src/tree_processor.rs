use std::error::Error;
use std::path::Path;
use super::tree::{TreeIter, Entry};

pub trait TreeProcessor {
    fn open_dir(&mut self, entry: &Entry);
    fn close_dir(&mut self);
    fn file(&mut self, entry: &Entry);
    fn root(&mut self, path: &Path);

    fn process(&mut self, tree: &mut TreeIter) -> Option<Box<Error>> {
        for result in tree {
            match result {
                Ok(entry) => {
                    if entry.metadata().is_dir() {
                        println!("open_dir");
                        self.open_dir(&entry);
                    } else {
                        println!("file");
                        self.file(&entry);
                    };

                    if !entry.has_next_sibling() {
                        println!("close_dir");
                        self.close_dir();
                    }
                },
                Err(err) => return Some(err),
            };
        };

        None
    }
}
