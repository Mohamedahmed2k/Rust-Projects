fn main() {

    // borrow -> mut 
    
    let mut x =10;
    let y = &mut x;
    println!("{}",*y);
    println!("{}",x);
    println!("{}",*y); // error
}
