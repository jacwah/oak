use std::error::Error;
use super::tree::{TreeIter, Entry, Event};

/// A generic trait for processing the output of `TreeIter`.
pub trait TreeProcessor {
    /// Called for each `OpenDir` event.
    fn open_dir(&mut self, entry: &Entry);
    /// Called for each `CloseDir` event.
    fn close_dir(&mut self);
    /// Called for each `File` event.
    fn file(&mut self, entry: &Entry);

    /// Iterates thorugh a `TreeIter`, delegating each event to its respective method.
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
