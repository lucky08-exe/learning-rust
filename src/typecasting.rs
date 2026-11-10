use std::io;

fn main() {
    
    let x:u32 = 23;

    //let mut y = x;
    //y = -23;
    // doing this creates copy of x, it INHERITS data type too
    // so putting -ve value in u32 ie unsigned data type -> gives error

    let y:i32 = -46;

    //let z = x + y;
    //since x & y are diff data types we can't add

    let z: f32 = (x as f32) / y as f32;
    print!("{}", z);


    //CONVERTING STRING TO INT

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("msg");

    let int_input:i64 = input.trim().parse().unwrap();

    print!("{}", int_input);

}