fn main() {
    let mut s = String::from("A new test string");
    println!("s is {}", s);

    let str_len = calc_length(&s);
    println!("length of '{}' is {}", s, str_len);

    change(&mut s);
    println!("s is {}", s);

    let str_len = calc_length(&s);
    println!("length of '{}' is {}", s, str_len);

    let ref_1 = &s;
    let ref_2 = &s;
    println!("{}, {}", ref_1, ref_2);

    let ref_3 = &mut s;
    println!("{}", ref_3);

    let not_a_ref_to_nothing = no_dangle();
    println!("{}", not_a_ref_to_nothing);
}

fn calc_length(input: &String) -> usize {
    // argument is a reference; function now owns the address of input
    input.len() // move return value to caller
}

fn change(some_string: &mut String) {
    // reference is dropped, not the data it points to
    some_string.push_str(", with appended text");
}

fn no_dangle() -> String {
    // scope of s is this function only
    let s = String::from("Rust prevents dangling references!");

    // return the string, not a ref to it; offers ownership to the caller
    s
}
