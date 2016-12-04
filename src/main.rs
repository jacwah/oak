#[macro_use]
extern crate clap;
extern crate oak;

use std::path::Path;
use std::process;
use std::io::{Write, stderr};
use std::fmt::Display;
use oak::tree_processor::TreeProcessor;
use oak::print_processor::{PrintProcessorBuilder, SummaryFormat};
use oak::tree;
use oak::filters::{FilterAggregate, filter_hidden_files, filter_non_dirs, GitignoreFilter, GlobFilter};

fn die(message: &Display) -> ! {
    writeln!(&mut stderr(), "error: {}", message).expect("Failed to write to stderr");
    process::exit(1);
}

fn main() {
    let argv_matches = clap::App::new("Oak")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A recursive directory listing utility for the modern age.")
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
        .arg(clap::Arg::with_name("glob-include")
            .help("Include only files matching a glob pattern")
            .short("P")
            .conflicts_with("glob-exclude")
            .multiple(true)
            .number_of_values(1)
            .takes_value(true))
        .arg(clap::Arg::with_name("glob-exclude")
            .help("Exclude files matching a glob pattern")
            .short("I")
            .multiple(true)
            .number_of_values(1)
            .takes_value(true))
        .get_matches();

    let dir = Path::new(argv_matches.value_of("DIR").unwrap_or("."));
    let mut filters = FilterAggregate::default();
    let mut procor = PrintProcessorBuilder::new(From::from(dir));

    if !argv_matches.is_present("show-hidden") {
        filters.push(filter_hidden_files);
    }

    if argv_matches.is_present("only-dirs") {
        filters.push(filter_non_dirs);
        procor.summary(SummaryFormat::DirCount);
    }

    if !argv_matches.is_present("no-git-ignore") {
        match GitignoreFilter::new(dir) {
            Some(Ok(filter)) => {
                filters.push(filter);
            },
            Some(Err(err)) => {
                die(&err);
            },
            None => {},
        }
    }

    // This is a mess!!
    if let Some(patterns) = argv_matches.values_of("glob-include") {
        let result = GlobFilter::from(patterns.map(From::from), false);

        match result {
            Ok(filter) => filters.push(filter),
            Err(err) => die(&err),
        };
    } else if let Some(patterns) = argv_matches.values_of("glob-exclude") {
        let result = GlobFilter::from(patterns.map(From::from), true);

        match result {
            Ok(filter) => filters.push(filter),
            Err(err) => die(&err),
        };
    }

    let mut tree_iter = tree::TreeIter::new(dir, filters).unwrap_or_else(|err| die(&err));
    if let Some(err) = procor.build().process(&mut tree_iter) {
        die(&err);
    }
}
