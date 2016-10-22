use std::path::Path;

pub trait TreeProcessor {
    fn open_dir(&mut self, path: &Path, num_entries: usize);
    fn close_dir(&mut self);
    fn file(&mut self, path: &Path);
}
