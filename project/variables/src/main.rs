fn main() {
    //Variables and mutable
    let mut x = 2019;
    println!("The value of x is: {}", x);
    x = 2020; 
    println!("The value of x is: {}", x);
    let cat = '😻';
    println!("The value of x is: {}", cat);

    //Tuple and destructuring
    let tup: (i32, f64, u8) = (500, 6.42, 12);
    let (a, b, c) = tup;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of first is: {}", five_hundred);
    println!("The value of second is: {}", six_point_four);
    println!("The value of third is: {}", one);
    //Array and accession
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first in arr is: {}", first);
    println!("The value of second in arr is: {}", second);
}
