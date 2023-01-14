use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Failed to parse input");

    factor_function(number);
}

fn factor_function(x: i32){
    let mut num = x;
    let mut check = true;
    let prime = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    for p in prime.iter(){
        while check {  
            if num%p == 0{
                if num == *p{
                    print!("{}",p);
                }
                else{
                    print!("{}*",p); 
                }
                num = num/p;
            }
            else{
                check = false;  
            }
        }
        check = true;
    }
}