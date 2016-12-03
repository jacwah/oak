use std::error::Error;
use std::path::Path;
use super::tree::{TreeIter, Entry, Event};

pub trait TreeProcessor {
    fn open_dir(&mut self, entry: &Entry);
    fn close_dir(&mut self);
    fn file(&mut self, entry: &Entry);
    fn root(&mut self, path: &Path);

    fn process(&mut self, tree: &mut TreeIter) -> Option<Box<Error>> {
        for result in tree {
            match result {
                Ok(event) => {
                    match event {
                        Event::OpenDir(ref entry) => self.open_dir(entry),
                        Event::File(ref entry) => self.file(entry),
                        Event::CloseDir => self.close_dir(),
                    };
                },
                Err(err) => return Some(err),
            };
        };

        None
    }
}
