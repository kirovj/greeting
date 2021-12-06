use std::{
    fs::File,
    io::{self, Read},
};

fn read_file_as_name() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let name = read_file_as_name().unwrap();
    print!("name: {}", &name);
}
