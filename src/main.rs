use std::io;

fn main() {
    let mut user_string = String::new();

    io::stdin()
        .read_line(&mut user_string)
        .expect("Failed to read user input");

    println!("{}", find_largest_palindrome(&user_string));
}

fn expand_around_center(string: &str, mut left_center: usize, mut right_center: usize) -> &str {
    let mut best_word: &str = &string[0..0];
    let mut best_length = 0;

    while right_center < string.len() && string.chars().nth(left_center) == string.chars().nth(right_center) {
        let word = &string[left_center..right_center + 1];
        if word.len() > best_length {
            best_word = word;
            best_length = best_word.len();
        }

        if left_center != 0 { left_center -= 1; } else { break; }
        right_center += 1;
    }

    best_word
}

fn find_largest_palindrome(string: &str) -> &str {
    let mut best_word = &string[0..0];

    for i in 1..(string.len() - 1) {
        let word = expand_around_center(&string, i, i);

        if word.len() > best_word.len() { best_word = word; }
    }
    for i in 1..(string.len() - 1) {
        let word = expand_around_center(&string, i, i + 1);

        if word.len() > best_word.len() { best_word = word; }
    }
    
    best_word
}