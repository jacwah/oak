#[macro_use]
extern crate clap;

mod tree;
mod processor;

use std::path::Path;
use std::os::unix::ffi::OsStrExt;

fn filter_hidden_files(path: &Path) -> bool {
    // Is this implementation sound?
    static DOT: u8 = '.' as u8;
    let maybe_name = path.file_name();

    match maybe_name {
        Some(name) => name.as_bytes()[0] != DOT,
        _ => false,
    }
}

fn filter_non_dirs(path: &Path) -> bool {
    match path.metadata() {
        Ok(data) => data.is_dir(),
        Err(_) => false,
    }
}

fn main() {
    let argv_matches = clap::App::new("etree")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Extended tree")
        .arg(clap::Arg::with_name("DIR")
            .help("The directory to list")
            .index(1))
        .arg(clap::Arg::with_name("a")
             .help("Show hidden files")
             .short("a"))
        .arg(clap::Arg::with_name("d")
             .help("List directories only")
             .short("d"))
        .get_matches();

    let dir = Path::new(argv_matches.value_of("DIR").unwrap_or("."));

    let filter_hidden_files_ref = &filter_hidden_files;
    let filter_non_dirs_ref = &filter_non_dirs;

    let mut filters: Vec<&Fn(&Path) -> bool> = Vec::new();

    if !argv_matches.is_present("a") {
        //filters.push(&filter_hidden_files);
        filters.push(filter_hidden_files_ref);
    }

    if argv_matches.is_present("d") {
        //filters.push(&filter_non_dirs);
        filters.push(filter_non_dirs_ref);
    }

    match tree::process(&dir,
                        &mut processor::PrintProcessor::new(),
                        &filters) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
