fn main() {
    let test_string = String::from("This is a test string.");

    let word = first_word(&test_string[..]);

    println!("Input string: '{}'\nWord 1: {}", test_string, word);
    //test_string.clear();

    let a_string_literal = "literal string";
    let word_one = first_word(a_string_literal);
    println!("Input string: '{}'\nWord 1: {}", a_string_literal, word_one);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice); // [2,3]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
