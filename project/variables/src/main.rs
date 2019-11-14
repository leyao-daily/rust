fn main() {
    //Variables and mutable
    let mut x = 2019;
    println!("The value of x is: {}", x);
    x = 2020; 
    println!("The value of x is: {}", x);
    let cat = '😻';
    println!("The value of x is: {}", cat);

    //Tuple and destructuring
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //Array and accession
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
}
