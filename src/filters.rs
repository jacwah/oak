use std::path::Path;
use std::os::unix::ffi::OsStrExt;

pub fn filter_hidden_files(path: &Path) -> bool {
    // Is this implementation sound?
    static DOT: u8 = '.' as u8;
    let maybe_name = path.file_name();

    match maybe_name {
        Some(name) => name.as_bytes()[0] != DOT,
        _ => false,
    }
}

pub fn filter_non_dirs(path: &Path) -> bool {
    match path.metadata() {
        Ok(data) => data.is_dir(),
        Err(_) => false,
    }
}

