fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if number > 5 {
        println!("condition was false");
    } else {
        println!("both conditions were false");
    }

    let condition = true;
    let number_two = if condition { 5 } else { 6 };
    println!("number_two is {}", number_two);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {}", result);

    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("val: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("val: {}", element);
    }

    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");
}
