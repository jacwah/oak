pub trait TreeProcessor {
    fn open_dir(&self, name: &str, num_entries: usize);
    fn close_dir(&self);
    fn file(&self, name: &str);
    fn done(&self);
}
