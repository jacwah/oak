use std::fs;
use std::path::Path;
use std::error::Error;
use super::tree_processor::TreeProcessor;
use super::filters::FileFilter;

pub fn process<T, F>(dir: &Path, procor: &mut T, filter: &F) -> Result<(), Box<Error>>
    where T: TreeProcessor,
          F: FileFilter,
{
    let read_entries = try!(fs::read_dir(dir));

    let entries: Vec<_> = try!(read_entries.collect());
    let filter_results: Vec<_> = try!(entries.iter()
        .map(|entry| filter.filter(&entry.path()))
        .collect());
    let entries: Vec<_> = entries.into_iter()
        .zip(filter_results.into_iter())
        .filter(|t| t.1)
        .map(|t| t.0)
        .collect();
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
