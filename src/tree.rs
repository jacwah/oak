use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use processor::TreeProcessor;

pub fn process<T: TreeProcessor>(dir: &Path,
                                 procor: &mut T,
                                 filter: fn(&Path) -> bool,
                                 ) -> io::Result<()> {
    let read_entries = try!(fs::read_dir(dir));

    let mut entries: Vec<DirEntry> = Vec::new();

    for entry in read_entries {
        entries.push(try!(entry));
    }

    entries.retain(|x| filter(&x.path()));
    procor.open_dir(dir, entries.len());

    for entry in entries {
        let path = entry.path();

        let file_type = try!(entry.file_type());

        if file_type.is_dir() {
            try!(process(&path, procor, filter));
        } else {
            procor.file(&path);
        }
    }

    procor.close_dir();

    Ok(())
}
