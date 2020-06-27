use std::fs;
use std::io;
fn main() {
    let res = read_data_from_file().expect("some error");
    println!("{}",res);


}

fn read_data_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
