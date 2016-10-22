use std::path::Path;
use std::borrow::Cow;
use super::TreeProcessor;

struct Dir {
    num_entries: usize,
    num_processed: usize,
}

impl Dir {
    
    fn new(num_entries: usize) -> Dir {
        Dir { num_entries: num_entries, num_processed: 0 }
    }
 
}

pub struct PrintProcessor {
    dir_stack: Vec<Dir>,
    num_dirs: usize,
    num_files: usize,
}

impl PrintProcessor {

    pub fn new() -> PrintProcessor {
        PrintProcessor {
            dir_stack: Vec::new(),
            num_dirs: 0,
            num_files: 0,
        }
    }

    fn print_entry(&mut self, name: &str) {
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

        println!("\n{} directories, {} files", dirs, self.num_files);
    }

}

fn file_name_from_path(path: &Path) -> Cow<str> {
    // Using unwrap here should be safe as long as all paths processed by this
    // function are generated from read_dir
    path.file_name().unwrap().to_string_lossy()
}

impl TreeProcessor for PrintProcessor {

    fn open_dir(&mut self, path: &Path, num_entries: usize) {
        let rel_pathstr = &format!("{}", path.display());
        let mut file_name = file_name_from_path(path);
        let file_name = file_name.to_mut();

        // Print the relative path to the root dir
        let name = if self.dir_stack.is_empty() {
            rel_pathstr
        } else {
            file_name
        };

        self.print_entry(name);
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
        
