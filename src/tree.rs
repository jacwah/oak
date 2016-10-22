use std::io;
use std::fs;
use std::ffi;
use std::path::Path;
use processor::TreeProcessor;

pub fn process<T: TreeProcessor>(dir: &Path, procor: &mut T) -> io::Result<()> {
    let entries = try!(fs::read_dir(dir));
    let entries: Vec<_> = entries.collect();

    let dir_name = match dir.file_name() {
        Some(name) => name,
        None => ffi::OsStr::new("?"),
    };

    let dir_name = match dir_name.to_str() {
        Some(name) => name,
        None => "?",
    };

    procor.open_dir(dir_name, entries.len());

    for entry in entries {
        let entry = try!(entry);
        let path = entry.path();

        let file_type = try!(entry.file_type());

        if file_type.is_dir() {
            try!(process(&path, procor));
        } else {
            match path.file_name() {
                Some(name) => procor.file(name.to_str().unwrap_or("?")),
                None => (),
            }
        }
    }

    procor.close_dir();

    Ok(())
}
