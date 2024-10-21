use std::{io, result};
use std::fs::{remove_file, File, OpenOptions};
use std::io::prelude::*;
use std::fs;

fn main() {
    let result = divide(10, 2);
    match result{
        Ok(value) => println!("value: {}",value),
        Err(msg)=> println!("ERROR: {}",msg)
    }



    let result_f = file_open("src/ftest.txt");
    match result_f {
        Ok(cont)=> println!("{}",cont),
        Err(e)=> println!("{}",e)
    }
}
// method expect , unwrap
// unrecoverable panic!
// recoverable error
// ? operator
fn divide (x: i32,y: i32) -> Result< i32, String> {
    if (y == 0 ){
        return Err(String::from("cannot divided by Zero"));
    }
    Ok(x/y)

}

fn file_open(path:&str) -> Result<String,std::io::Error> {
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content);
    Ok(content)
}