fn num(min: i32, max: i32) -> i32 {
    let pair;
    let ans;
    if max%2 == 0 {
        pair = max/2;
        ans = (min+max)*pair;
    } else {
        pair = (max-1)/2;
        ans = ((min+max-1)*pair)+max;
    }
    ans
}

fn main() {
    println!("Hello, world!");
    let start = 1;
    let end = 100;
    println!("{} + ... + {} = {}",start,end,num(start,end));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let input_start = 1;
        let input_end = 100;
        let expected_output = 5050;
        assert_eq!(num(input_start,input_end), expected_output);
    }

    #[test]
    fn test_num2() {
        let input_start = 1;
        let input_end = 101;
        let expected_output = 5151;
        assert_eq!(num(input_start,input_end), expected_output);
    }
}