use std::fs::File;
use std::io::{Write, Read};
use std::{io, fs};

fn main(){
     match open_file() {
         Ok(res) => println!("{}", res),
         Err(e) => panic!(e),
     }
}
fn open_file()-> Result<String, io::Error> {
    let path = "./plain.txt";
    fs::write(path,"hahaha");
    let mut f = File::open("./plain.txt").unwrap_or_else(|e| {
        File::create(path).unwrap()
    });
    let mut buffer  = String::new();
    let s = f.read_to_string(&mut buffer)?;
    Ok(buffer)
}