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
    match list_dir(Path::new(".")) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
