//! `PrintProcessor` and supporting types.
//!
//! This processor is designed to output in a format inspired by the classic `tree` command line
//! utility.

use std::borrow::Cow;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use super::tree_processor::TreeProcessor;
use super::tree::Entry;

/// A summary format for `PrintProcessor`.
#[derive(Clone)]
pub enum SummaryFormat {
    /// Print only number of directories, e.g. "2 directories".
    ///
    /// This is useful if no files are expected, such as when using `filters::filter_non_dirs()`.
    DirCount,
    /// Print both number of directories and files, e.g. "2 directories, 10 files".
    DirAndFileCount,
}

/// Builder for `PrintProcessor`.
///
/// One of the benefits of a separate builder struct is deferring the printing of root until after
/// configuration. Thus no text is printed if a setup step fails.
pub struct PrintProcessorBuilder {
    summary_format: SummaryFormat,
    root: PathBuf,
}

impl PrintProcessorBuilder {
    /// Create a new builder.
    pub fn new(root: PathBuf) -> Self {
        PrintProcessorBuilder {
            summary_format: SummaryFormat::DirAndFileCount,
            root: root,
        }
    }

    /// Set the summary format.
    pub fn summary(&mut self, format: SummaryFormat) -> &mut Self{
        self.summary_format = format;
        self
    }

    /// Build a `PrintProcessor`.
    ///
    /// This method also prints the root, which sets up for subsequent output from the processor.
    pub fn build(&self) -> PrintProcessor {
        println!("{}", self.root.display());

        PrintProcessor {
            dir_has_next: vec![true],
            num_dirs: 0,
            num_files: 0,
            summary_format: self.summary_format.clone(),
        }
    }
}

/// A `TreeProcessor` for printing the events in a clasic `tree`-like format.
///
/// # Example
/// This is an example of the output of this processor.
///
/// ```text
/// ├── a
/// ├── b
/// │   ├── 1
/// │   └── 2
/// ├── c
/// └── d
/// ```
pub struct PrintProcessor {
    dir_has_next: Vec<bool>,
    num_dirs: usize,
    num_files: usize,
    summary_format: SummaryFormat,
}

impl PrintProcessor {
    fn print_entry<D: Display>(&mut self, name: &D) {
        let vertical_line = "│   ";
        let branched_line = "├── ";
        let terminal_line = "└── ";
        let empty_line    = "    ";

        let len = self.dir_has_next.len();

        for (i, has_next) in self.dir_has_next.iter().enumerate() {
            if i < len - 1 {
                if *has_next {
                    print!("{}", vertical_line);
                } else {
                    print!("{}", empty_line);
                }
            } else if *has_next {
                print!("{}", branched_line);
            } else {
                print!("{}", terminal_line);
            }
        }

        println!("{}", name);
    }

    fn print_summary(&self) {
        match self.summary_format {
            SummaryFormat::DirAndFileCount => {
                println!("\n{} directories, {} files", self.num_dirs, self.num_files)
            },
            SummaryFormat::DirCount => {
                println!("\n{} directories", self.num_dirs)
            },
        }
    }

}

fn file_name_from_path(path: &Path) -> Cow<str> {
    // Using unwrap here should be safe as long as all paths processed by this
    // function are generated from read_dir
    path.file_name().unwrap().to_string_lossy()
}

impl TreeProcessor for PrintProcessor {
    fn open_dir(&mut self, entry: &Entry) {
        self.dir_has_next.pop();
        self.dir_has_next.push(entry.has_next_sibling());

        // Print the relative path to the root dir
        if self.dir_has_next.is_empty() {
            self.print_entry(&entry.path().display());
        } else {
            self.print_entry(file_name_from_path(entry.path()).to_mut());
        };

        self.dir_has_next.push(true);
        self.num_dirs += 1;
    }

    fn close_dir(&mut self) {
        self.dir_has_next.pop().expect("Number of calls to close_dir exceeds open_dir");

        if self.dir_has_next.is_empty() {
            self.print_summary();
        }
    }

    fn file(&mut self, entry: &Entry) {
        self.dir_has_next.pop();
        self.dir_has_next.push(entry.has_next_sibling());

        self.print_entry(file_name_from_path(entry.path()).to_mut());
        self.num_files += 1;
    }
}
