
extern crate libc;

#[link(name = "calculator")]
extern "C" {
    fn multiply(input: i32) -> i32;
}

fn run_calc() {
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);

    let input = 4;
    let output = unsafe {
        multiply(input)
    };
    println!("{} * 2 = {}", input, output);
}


fn main() {
    println!("Hello, world!");

    run_calc();
}
