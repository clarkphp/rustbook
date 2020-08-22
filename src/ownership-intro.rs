fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    takes_ownership(s); // move s to the function, which deallocates it
                        // println!("{}", s);

    let x = 5;
    let y = x; // y is a true copy of x; both live on the stack
    println!("x: {} and y: {}", x, y);

    makes_copy(x);
    println!("x: {} and y: {}", x, y);

    let s1 = String::from("hello");
    //    let s2 = s1; // move
    let s2 = s1.clone(); // deep copy; heap allocation
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let string_one = gives_ownership(); // move return val of function into variable
    println!("{}", string_one);

    let string_three = takes_and_gives_back(s2); // move rtn val into string_three
    println!("{}", string_three);

    let (string_four, len) = calc_length(string_three);
    println!("length of {} is {}", string_four, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // takes ownership of argument, deallocates it at func end
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("A new string"); // new string in scope
    some_string // moves to caller of this function
}

fn takes_and_gives_back(a_string: String) -> String {
    // argument is moved to this function
    a_string // moves to caller of this function
}

fn calc_length(input: String) -> (String, usize) {
    // take ownership of argument
    let length = input.len(); // create new variable in scope
    (input, length) // move argument back to caller, along with local variable
}
