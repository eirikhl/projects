use read_input::prelude::*;

fn main() {
    let x: f64 = input().msg("Please enter a number \n").get();
    println!("Your first number is {}", x);

    let y: f64 = input().msg("Please enter a number \n").get();

    println!("Your second number is {}", y);

    let op: char = input().msg("Please enter an operation \n").get();
    match op {
        '+' => println!("The result is: {}", x+y),
        '-' => println!("The result is: {}", x-y),
        '*' => println!("The result is: {}", x*y),
        '/' => println!("The result is: {}", x/y),
        _ => println!("Now that was not very nice of you."),
    }
}
