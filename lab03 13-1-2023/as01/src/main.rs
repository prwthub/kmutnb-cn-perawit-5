fn word(s: &str) -> Vec<String> {
    // s = variable name
    s.split_whitespace().map(|s| s.to_string()).collect()
    // s.split แยก sting ด้วย space ตามชื่อ method
    // s.map(|s| s.to_string) ใช้ปิดค่า sting ย่อย ที่แยกออกมาได้จาก s.split
    // s.collect รวบรวม element ที่ทำการแยกออกมา เอาไปใส้ใน Vec<String>
}

fn unique(words: Vec<String>) -> Vec<String> {
    // words = variable name
    let mut unique_words = Vec::new();
    // ทำการสร้าง array เปล่า
    for word in words {
        if !unique_words.contains(&word) {
        // ถ้า unique_words ไม่มี word 
            unique_words.push(word);
            // ทำการใส่คำนั้นลงไปใน unique_words
        }
    }
    unique_words
    // return unique_words
}

fn count(words: Vec<String>) -> usize {
    words.len()
}


fn main() {
    
    let input_string = "this cat this bat this rat";
    println!("string : {}",input_string);

    let words = word(input_string);
    println!("word : {:?}",words);

    let unique_words = unique(words);
    println!("unique : {:?}",unique_words);
    
    let count_unique = count(unique_words);
    println!("count : {}", count_unique);

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word() {
        let input_string = "this cat this bat this rat";
        let expected_output = vec!["this", "cat", "this", "bat", "this", "rat"];
        assert_eq!(word(input_string), expected_output);
    }

    #[test]
    fn test_unique() {
        let input_words = vec!["this", "cat", "this", "bat", "this", "rat"];
        let input_words: Vec<String> = input_words.iter().map(|s| s.to_string()).collect();
        // เพื่อแปลง input_words ให้เป็น Vec<String>
        let expected_output = vec!["this", "cat", "bat", "rat"];
        assert_eq!(unique(input_words), expected_output);
    }

    #[test]
    fn test_count() {
        let input_words = vec!["this", "cat", "bat", "rat"];
        let input_words: Vec<String> = input_words.iter().map(|s| s.to_string()).collect();
        // เพื่อแปลง input_words ให้เป็น Vec<String>
        let expected_output = 4;
        assert_eq!(count(input_words), expected_output);
    }
    
    // -------------------------------------------------------------------------------------------------

    #[test]
    fn test_word_2() {
        let input_string = "Hello Hello World this is is my first program program";
        let expected_output = vec!["Hello","Hello","World","this","is","is","my","first","program","program"];
        assert_eq!(word(input_string), expected_output);
    }

    #[test]
    fn test_unique_2() {
        let input_words = vec!["Hello","Hello","World","this","is","is","my","first","program","program"];
        let input_words: Vec<String> = input_words.iter().map(|s| s.to_string()).collect();
        // เพื่อแปลง input_words ให้เป็น Vec<String>
        let expected_output = vec!["Hello","World","this","is","my","first","program"];
        assert_eq!(unique(input_words), expected_output);
    }

    #[test]
    fn test_count_2() {
        let input_words = vec!["Hello","World","this","is","my","first","program"];
        let input_words: Vec<String> = input_words.iter().map(|s| s.to_string()).collect();
        // เพื่อแปลง input_words ให้เป็น Vec<String>
        let expected_output = 7;
        assert_eq!(count(input_words), expected_output);
    }
}

