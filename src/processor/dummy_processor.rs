use super::TreeProcessor;

pub struct DummyProcessor;

impl DummyProcessor {

    pub fn new() -> DummyProcessor {
        DummyProcessor{}
    }

}

impl TreeProcessor for DummyProcessor {

    fn open_dir(&self, name: &str, num_entries: usize) {
        println!("open_dir({}, {})", name, num_entries);
    }

    fn close_dir(&self) {
        println!("close_dir()");
    }

    fn file(&self, name: &str) {
        println!("file({})", name);
    }

}
