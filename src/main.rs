#[macro_use]
extern crate clap;

mod tree;
mod processor;

use std::path::Path;

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
 
    match tree::process(dir, &processor::DummyProcessor::new()) {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
    }
}
