pub trait TreeProcessor {
    fn open_dir(&mut self, name: &str, num_entries: usize);
    fn close_dir(&mut self);
    fn file(&mut self, name: &str);
}
