use std::path::Path;
use super::TreeProcessor;

#[allow(dead_code)]
pub struct DummyProcessor;

impl DummyProcessor {

    #[allow(dead_code)]
    pub fn new() -> DummyProcessor {
        DummyProcessor{}
    }

}

impl TreeProcessor for DummyProcessor {

    fn open_dir(&mut self, path: &Path, num_entries: usize) {
        println!("open_dir({}, {})", path.display(), num_entries);
    }

    fn close_dir(&mut self) {
        println!("close_dir()");
    }

    fn file(&mut self, path: &Path) {
        println!("file({})", path.display());
    }

}
