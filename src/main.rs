fn main() {
    let mut s = String::from("A new test string");
    println!("s is {}", s);

    let str_len = calc_length(&s);
    println!("length of '{}' is {}", s, str_len);

    change(&mut s);
    println!("s is {}", s);
}

fn calc_length(input: &String) -> usize {
    // argument is a reference; function now owns the address of input
    input.len() // move return value to caller
}

fn change(some_string: &mut String) {
    some_string.push_str(", with appended text"); // reference is dropped, not the data it points to
}
