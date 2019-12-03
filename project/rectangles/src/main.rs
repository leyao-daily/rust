fn main() {
    let width2 = 40;
    let height2 = 40;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width2, height2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

