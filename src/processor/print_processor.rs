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

    fn print_entry(&self, name: &str) {
        for _ in 0..self.dir_stack.len() {
            print!("    ");
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
        
