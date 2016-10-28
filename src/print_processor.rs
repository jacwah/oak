use std::borrow::Cow;
use std::fmt::Display;
use std::path::Path;
use super::tree_processor::TreeProcessor;

struct Dir {
    num_entries: usize,
    num_processed: usize,
}

impl Dir {
    
    fn new(num_entries: usize) -> Dir {
        Dir { num_entries: num_entries, num_processed: 0 }
    }
 
}

pub enum SummaryFormat {
    DirCount,
    DirAndFileCount,
}

pub struct PrintProcessor {
    dir_stack: Vec<Dir>,
    num_dirs: usize,
    num_files: usize,
    summary_format: SummaryFormat,
}

impl PrintProcessor {

    pub fn new() -> PrintProcessor {
        PrintProcessor {
            dir_stack: Vec::new(),
            num_dirs: 0,
            num_files: 0,
            summary_format: SummaryFormat::DirAndFileCount,
        }
    }

    pub fn set_summary_format(&mut self, format: SummaryFormat) {
        self.summary_format = format;
    }

    fn print_entry<T: Display>(&mut self, name: &T) {
        let vertical_line = "│   ";
        let branched_line = "├── ";
        let terminal_line = "└── ";
        let empty_line    = "    ";

        for (i, dir) in self.dir_stack.iter().enumerate() {
            if dir.num_processed == dir.num_entries {
                print!("{}", empty_line);
            } else if i == self.dir_stack.len() - 1 { // if the leaf dir
                if dir.num_processed == dir.num_entries - 1 {
                    print!("{}", terminal_line);
                } else {
                    print!("{}", branched_line);
                }
            } else {
                print!("{}", vertical_line);
            }
        }

        if let Some(leaf_dir) = self.dir_stack.last_mut() {
            leaf_dir.num_processed += 1;
        }

        println!("{}", name);
    }

    fn print_summary(&self) {
        // Do not count the root dir or underflow
        let dirs = match self.num_dirs {
            0 => 0,
            n @ _ => n - 1,
        };

        match self.summary_format {
            SummaryFormat::DirAndFileCount => println!("\n{} directories, {} files", dirs, self.num_files),
            SummaryFormat::DirCount => println!("\n{} directories", dirs),
        }
    }

}

fn file_name_from_path(path: &Path) -> Cow<str> {
    // Using unwrap here should be safe as long as all paths processed by this
    // function are generated from read_dir
    path.file_name().unwrap().to_string_lossy()
}

impl TreeProcessor for PrintProcessor {

    fn open_dir(&mut self, path: &Path, num_entries: usize) {
        // Print the relative path to the root dir
        if self.dir_stack.is_empty() {
            self.print_entry(&path.display());
        } else {
            self.print_entry(file_name_from_path(path).to_mut());
        };

        self.dir_stack.push(Dir::new(num_entries));
        self.num_dirs += 1;
    }

    fn close_dir(&mut self) {
        self.dir_stack.pop();

        if self.dir_stack.is_empty() {
            self.print_summary();
        }
    }

    fn file(&mut self, path: &Path) {
        self.print_entry(file_name_from_path(path).to_mut());
        self.num_files += 1;
    }

}

