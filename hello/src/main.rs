use std::io;
fn main() {
/*
    COMPOUND Types - Tuples , Arrays 
*/
    let student_a = ("mohamed",22,'A',3.5);
    println!("my name is {} my age {} my grade {} my GPA {}",student_a.0,student_a.1,student_a.2,student_a.3);

    let (name , age ,grade , gpa )= student_a;
    println!("my name is {} my age {} my grade {} my GPA {}",name , age ,grade , gpa );

    let  students=["mohamed","ahmed","mazen","amr","omar"];

    let slice = &students[1..4];//slice from 1 to 3

    println!("{:?}",slice);// compiler suggest

    let mut namel=String::from("mohamed"); // string
    namel.push_str("ahmed");

    let mut game = String::new();
    io::stdin().read_line(&mut game);
    println!("I love {}",game);

    let x =10;
    let x_float = x as f64;

    // crates.io fo dep

    let ages = 33;

    match ages {
        1..=24 => println!("a"),
        24..=25 =>println!("b"),
        _ => println!("z")
    };

    // loop {} // inf loop

println!("{}" ,add_multi(5, 6).0);
let (add,multi) = add_multi(5, 6);


    struct car {
        make:String,
        model:String,
        year: u32,
        price:f64,
    };

    let huracan = car{
        make:String::from("lambo"),
        model:String::from("huracan"),
        year: 2020,
        price:  320000.00,
    };
    println!("price of {} {} {} is {}",huracan.make,huracan.model,huracan.year,huracan.year);

    struct rectangle{
        width: u32,
        height: u32,
    }

    impl rectangle {
        fn area(&self) -> u32{
            self.height * self.width
        }
        
    }
    let D = rectangle {width:30,height:50};
    println!("area is {}",D.area());

    #[derive(Debug)]
    enum shape {
        circle(f32),
        rect(f32,f32),
    }

    let rect = shape::rect(10.0, 15.0);
    println!("{:?}",rect);

    struct items <T,U>{
        x: T,
        y: U,
    }

    let useg = items {x:10,y:"hello"};
    
}

// fn return 2 argu
fn add_multi (x: i32 , y: i32) -> (i32  , i32){
    return (x+y,x*y);
}
 
// genaric type templete
fn sum<T: std::ops::Add<Output=T>> (a:T,b:T) ->T {
    a + b
}