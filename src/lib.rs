/// Reverses the given string.
/// 
/// # Arguments
/// * `input` - A string slice that holds the string to be reversed.
/// 
/// # Returns
/// A new String that is the reverse of the input string.
/// 
/// # Example
/// ```
/// let result = reverse_string("hello");
/// assert_eq!(result, "olleh");
/// ```
pub fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();
    for ch in input.chars().rev() {
        reversed.push(ch);
    }
    reversed
}

/// Checks if the input string is a palindrome (reads the same forward and backward).
/// 
/// # Arguments
/// * `input` - A string slice to be checked for palindrome property.
/// 
/// # Returns
/// A boolean indicating whether the string is a palindrome or not.
/// 
/// # Example
/// ```
/// let result = is_palindrome("madam");
/// assert!(result);
/// ```
pub fn is_palindrome(input: &str) -> bool {
    let reversed = reverse_string(input);
    input == reversed
}

/// Finds the most commonly occurring character(s) in the string and their frequency.
/// 
/// # Arguments
/// * `input` - A string slice to analyze.
/// 
/// # Returns
/// A vector of tuples, each containing a character and its frequency.
/// 
/// # Example
/// ```
/// let result = most_common_characters("aabbbc");
/// assert_eq!(result, vec![('b', 3)]);
/// ```
pub fn most_common_characters(input: &str) -> Vec<(char, usize)> {
    let mut frequency_map = std::collections::HashMap::new();

    // Count frequencies of each character
    for ch in input.chars() {
        let counter = frequency_map.entry(ch).or_insert(0);
        *counter += 1;
    }

    // Find the maximum frequency
    let max_frequency = frequency_map.values().cloned().max().unwrap_or(0);

    // Collect characters with the maximum frequency
    frequency_map
        .into_iter()
        .filter(|&(_, count)| count == max_frequency)
        .collect()
}

// lib.rs - Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("world"), "dlrow");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("madam"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_most_common_characters() {
        let input = "aabbbc";
        let result = most_common_characters(input);
        assert_eq!(result, vec![('b', 3)]);

        let input = "aabbcc";
        let result = most_common_characters(input);
        assert_eq!(result, vec![('a', 2), ('b', 2), ('c', 2)]);
    }
}
