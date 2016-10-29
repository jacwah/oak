#[macro_use]
extern crate clap;
extern crate ntree;

use std::path::Path;
use std::process;
use ntree::print_processor::{PrintProcessor, SummaryFormat};
use ntree::tree;
use ntree::filters::{filter_hidden_files, filter_non_dirs, GitignoreFilter};

fn main() {
    let argv_matches = clap::App::new("ntree")
        .version(crate_version!())
        .author(crate_authors!())
        .about("New tree -- a modern reimplementation of tree.")
        .arg(clap::Arg::with_name("DIR")
            .help("The directory to list")
            .index(1))
        .arg(clap::Arg::with_name("a")
             .help("Show hidden files")
             .short("a"))
        .arg(clap::Arg::with_name("d")
             .help("List directories only")
             .short("d"))
        .arg(clap::Arg::with_name("git-ignore")
             .help("Do not list git ignored files")
             .short("g"))
        .get_matches();

    let dir = Path::new(argv_matches.value_of("DIR").unwrap_or("."));

    let filter_hidden_files_ref = &filter_hidden_files;
    let filter_non_dirs_ref = &filter_non_dirs;
    let filter_gitignore_maybe = GitignoreFilter::new(dir);
    let filter_gitignore: GitignoreFilter;
    let filter_gitignore_clos;
    let filter_gitignore_ref;

    let mut filters: Vec<&Fn(&Path) -> bool> = Vec::new();

    let mut procor = PrintProcessor::new();

    if !argv_matches.is_present("a") {
        //filters.push(&filter_hidden_files);
        filters.push(filter_hidden_files_ref);
    }

    if argv_matches.is_present("d") {
        //filters.push(&filter_non_dirs);
        filters.push(filter_non_dirs_ref);
        procor.set_summary_format(SummaryFormat::DirCount);
    }

    if argv_matches.is_present("git-ignore") {
        match filter_gitignore_maybe {
            Ok(_) => {
                filter_gitignore = filter_gitignore_maybe.unwrap();
                filter_gitignore_clos = |p: &Path| filter_gitignore.filter(p);
                filter_gitignore_ref = &filter_gitignore_clos;
                filters.push(filter_gitignore_ref);
            },
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            },
        }
    }

    match tree::process(&dir,
                        &mut procor,
                        &filters) {
        Ok(_) => (),
        Err(err) => {
            println!("error: {}", err);
            process::exit(1);
        }
    }
}
