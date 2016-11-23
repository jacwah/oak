use std::io;
use std::fs;
use std::path::Path;
use std::ops::Fn;
use super::tree_processor::TreeProcessor;

pub fn process<T, F>(dir: &Path, procor: &mut T, filters: &[F]) -> io::Result<()>
    where T: TreeProcessor,
          F: Fn(&Path) -> bool
{
    let read_entries = try!(fs::read_dir(dir));

    let mut entries: Vec<_> = try!(read_entries.collect());
    entries.retain(|x| filters.iter().all(|f| f(&x.path())));
    procor.open_dir(dir, entries.len());

    for entry in entries {
        let path = entry.path();

        let file_type = try!(entry.file_type());

        if file_type.is_dir() {
            try!(process(&path, procor, filters));
        } else {
            procor.file(&path);
        }
    }

    procor.close_dir();

    Ok(())
}
