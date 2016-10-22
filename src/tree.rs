use std::io;
use std::fs;
use std::path::Path;
use processor::TreeProcessor;

pub fn process<T: TreeProcessor>(dir: &Path, procor: &T) -> io::Result<()> {
    for entry in try!(fs::read_dir(dir)) {
        let entry = try!(entry);
        let path = entry.path();
        match path.file_name() {
            Some(name) => procor.file(name.to_str().unwrap_or("?")),
            None => (),
        }
    }
    Ok(())
}
