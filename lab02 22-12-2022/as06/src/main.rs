use std::io::{self, Write};

fn main() {
    let mut arr:[i32;8] = [1,2,0,4,3,0,5,0];
    zeroback_function(&mut arr);
}

fn zeroback_function(arr: &mut [i32]){
    print!("[");
    let mut count = 0;
    for num in arr.iter(){
        if num != &0 {
            print!("{},",num);
        }
        else {
            count = count + 1;
        }
    }
    for c in 0..count{
        if c == count-1 {
            print!("0");
        }
        else{
            print!("0,")
        }
        
    }
    print!("]");
}