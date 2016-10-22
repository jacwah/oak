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
}

impl PrintProcessor {

    pub fn new() -> PrintProcessor {
        PrintProcessor { dir_stack: Vec::new() }
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

}

impl TreeProcessor for PrintProcessor {
    
    fn open_dir(&mut self, name: &str, num_entries: usize) {
        self.print_entry(name);
        // incr processed
        self.dir_stack.push(Dir::new(num_entries));
    }

    fn close_dir(&mut self) {
        self.dir_stack.pop();
    }

    fn file(&mut self, name: &str) {
        self.print_entry(name);
    }

}
        
