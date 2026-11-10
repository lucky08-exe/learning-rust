use std::io;

fn main() {
    print!("enter text:\n");
    
    // strings are complex idk :(
    let mut input = String::new();

    //just (input) would create a copy and not do anything to original
    // just (&input) wouldnt let us make changes
    // so we come to (&mut input) to make a mutable reference to original variable
    // similar to how & in C changes value at memory lever through pointer
    io::stdin().read_line(&mut input).expect("failed to read");

    println!("{}", input);


}