use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, usize> {
    let mut letter_counts = HashMap::new();

    for c in text.to_lowercase().chars() {
        if c.is_alphabetic() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }
    }

    letter_counts
}

fn main() {
    let input_text = "Hello, World! 123 ABC";
    let letter_counts = count_letters(input_text);

    for (letter, count) in letter_counts {
        println!("{} => {}", letter, count);
    }
}