// main.rs - Application that uses the functions from the library

use std::io::{self, Write};
use your_library::{reverse_string, is_palindrome, most_common_characters};

fn main() {
    // Prompt for user input
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim(); // Remove trailing newline/whitespace

    // Reverse the string and print
    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);

    // Check if the string is a palindrome
    if is_palindrome(input) {
        println!("The string is a palindrome.");
    } else {
        println!("The string is not a palindrome.");
    }

    // Find and print the most common character(s)
    let common_chars = most_common_characters(input);
    if common_chars.len() == 1 {
        let (ch, count) = common_chars[0];
        println!("The most common character is '{}' with {} occurrences.", ch, count);
    } else {
        println!("The most common characters are:");
        for (ch, count) in common_chars {
            println!("'{}' with {} occurrences.", ch, count);
        }
    }
}
