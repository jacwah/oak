#[macro_use]
extern crate clap;

use std::io;
use std::fs;
use std::path::Path;

fn list_dir(dir: &Path) -> io::Result<()> {
    for entry in try!(fs::read_dir(dir)) {
        let entry = try!(entry);
        let path = entry.path();
        match path.file_name() {
            Some(name) => println!("{}", name.to_string_lossy()),
            None => (),
        }
    }
    Ok(())
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
 
    match list_dir(dir) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
