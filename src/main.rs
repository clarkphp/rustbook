fn main() {
    another_function(5, 6);
    let x = five();
    println!("x: {}", x);

    let x = plus_one(3);
    println!("x from plus_one: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("x = {}, and y = {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
