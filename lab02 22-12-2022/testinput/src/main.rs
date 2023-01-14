use std::io;
use std::io::Write;

fn main() {
    print!("Enter x value : ");
    io::stdout().flush().unwrap(); // flush the output buffer

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    
    println!();
    println!("x = {}", x);
}
