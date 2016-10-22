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

fn main() {
    let argv_matches = clap::App::new("etree")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Extended tree")
        .arg(clap::Arg::with_name("DIR")
            .help("The directory to list")
            .index(1))
        .get_matches();

    let dir = Path::new(argv_matches.value_of("DIR").unwrap_or("."));

    match tree::process(dir,
                        &mut processor::PrintProcessor::new(),
                        filter_hidden_files) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
