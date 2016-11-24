use std::io;
use std::fs;
use std::path::Path;
use super::tree_processor::TreeProcessor;
use super::filters::FileFilter;

pub fn process<T, F>(dir: &Path, procor: &mut T, filter: &F) -> io::Result<()>
    where T: TreeProcessor,
          F: FileFilter,
{
    let read_entries = try!(fs::read_dir(dir));

    let mut entries: Vec<_> = try!(read_entries.collect());
    entries.retain(|x| filter.filter(&x.path()));
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
