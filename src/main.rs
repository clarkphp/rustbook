fn main() {
    let mut test_string = String::from("This is a test string.");
    let word_end_index = first_word(&test_string);
    println!(
        "Input string: '{}'\nWord 1: {}",
        test_string, word_end_index
    );
    test_string.clear(); // index to end of word is now invalid!

    println!(
        "Input string: '{}'\nWord 1: {}",
        test_string, word_end_index
    );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
