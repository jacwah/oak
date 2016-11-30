#[macro_use]
extern crate clap;
extern crate ntree;

use std::path::Path;
use std::process;
use std::io::{Write, stderr};
use std::fmt::Display;
use ntree::print_processor::{PrintProcessor, SummaryFormat};
use ntree::tree;
use ntree::filters::{FilterAggregate, filter_hidden_files, filter_non_dirs, GitignoreFilter};

fn die(message: &Display) -> ! {
    writeln!(&mut stderr(), "error: {}", message).expect("Failed to write to stderr");
    process::exit(1);
}

fn main() {
    let argv_matches = clap::App::new("ntree")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tree for the modern user.")
        .arg(clap::Arg::with_name("DIR")
            .help("The directory to list, defaults to cwd")
            .index(1))
        .arg(clap::Arg::with_name("show-hidden")
             .help("Show hidden files")
             .short("a"))
        .arg(clap::Arg::with_name("only-dirs")
             .help("List directories only")
             .short("d"))
        .arg(clap::Arg::with_name("no-git-ignore")
             .help("Do not exclude gitignored files")
             .long("no-git")
             .short("g"))
        .get_matches();

    let dir = Path::new(argv_matches.value_of("DIR").unwrap_or("."));
    let mut filters = FilterAggregate::default();
    let mut procor = PrintProcessor::new();

    if !argv_matches.is_present("show-hidden") {
        filters.push(filter_hidden_files);
    }

    if argv_matches.is_present("only-dirs") {
        filters.push(filter_non_dirs);
        procor.set_summary_format(SummaryFormat::DirCount);
    }

    if !argv_matches.is_present("no-git-ignore") {
        match GitignoreFilter::new(dir) {
            Ok(filter) => {
                filters.push(filter);
            },
            Err(err) => {
                die(&err);
            },
        }
    }

    // if let Err(err) = tree::process(&dir, &mut procor, &filters) {
    //     die(&err);
    // }
    if let Err(err) = tree::process(dir, filters) {
        die(&err);
    }
}
