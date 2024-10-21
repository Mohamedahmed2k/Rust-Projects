

use std::io::{self, stdin};

fn main() {
    println!("Rust calculator");
    println!("Enter value of X");
    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x);
    let x : i32 = input_x.trim().parse().expect("input not an integer!");
    let float_x = x as f32 ;

    println!("Enter value of Y");
    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y);
    let y : i32 = input_y.trim().parse().expect("input not an integer!");
    let float_y = y as f32 ;  

    println!("choose operator: + ,* ,- ,/ ,%");
    let mut input_ope = String::new();
    io::stdin().read_line(&mut input_ope);
    let ope  = input_ope.trim();
    
    match ope {
        "+" => add(x,y),
        "-" => sub(x, y),
        "/" => divi(float_x, float_y),
        "*" => multi(x, y),
        "%" => rem(x, y),
        _   => println!("not supported"),
    };

}

fn add(x: i32 , y: i32){
    println!("{} + {} = {}",x,y,x+y);
}
fn multi(x: i32 , y: i32){
    println!("{} X {} = {}",x,y,x*y);
}
fn sub(x: i32 , y: i32){
    println!("{} - {} = {}",x,y,x-y);
}
fn divi(x: f32 , y: f32){
    println!("{} / {} = {}",x,y,x/y);
}
fn rem(x: i32 , y: i32){
    println!("{} % {} = {}",x,y,x%y);
}