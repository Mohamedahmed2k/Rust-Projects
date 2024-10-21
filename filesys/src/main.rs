use std::io;
use std::fs::{remove_file, File, OpenOptions};
use std::io::prelude::*;
use std::fs;

fn main() {
    //let mut file = File::create("src/hello.txt").expect("no file create");
    //file.write_all("hello, world!".as_bytes()).expect("failed to write");

    //let mut file = OpenOptions::new().append(true).open("src/hello.txt").expect("failed");
    //file.write_all("hello again".as_bytes()).expect("faild");
/* 
    let mut file = File::open("src/hello.txt").expect("fail");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("{}",content);
    
    */

    fs::remove_file("src/hello.txt").expect("fail");
}
