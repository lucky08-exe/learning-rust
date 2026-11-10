fn main() {
    let x = 5;
    print!("{}", x);
    {
        let x = x + 1;
        print!("{}", x);
    }
    let x = 4;
    print!("{}", x);

    // to change value we define variable as "mut" mutable

    //normally variables cannot be edited, so redefined using 'let'
    // on redefining we can change type
    let x = "hello";
    print!("{}", x);

    let tup:(i32, bool, char) = (3, false, 'h');
    print!("{}", tup.0);

    let arr :[i32; 5] = [0,3,4,5,5];
    print!("{}", arr[4]);
    

}
