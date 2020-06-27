use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
fn main() {
    let res = read_data_from_file().expect("some error");
    println!("{}",res);


}

fn read_data_from_file() -> Result<String, io::Error> {
    // MATCH FOR DAYS
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // };

    // SHORTHAND FOR MATCH
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // CHAINING SHORTHANDS
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // CONVENIENT READ OPERATION
    fs::read_to_string("hello.txt")
}
