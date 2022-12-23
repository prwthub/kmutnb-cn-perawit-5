use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Failed to parse input");

    for i in 0..number{
        print!("{} ",fibonacci_function(i));
        if i == number-1{
            print!("...")
        }
    }
}

fn fibonacci_function(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_function(n-1) + fibonacci_function(n-2),
    }
}