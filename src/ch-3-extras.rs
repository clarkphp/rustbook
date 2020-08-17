fn main() {
    for n in 0..13 {
        println!("F({}) = {}", n, fibonacci(n));
    }

    for temp in [-40.0, 0.0, 32.0, 98.6, 212.0].iter() {
        println!("{} deg F = {} deg C", temp, fahrenheit_to_celsius(*temp));
    }

    for temp in [-40.0, -17.7, 0.0, 37.0, 100.0].iter() {
        println!("{} deg C = {} deg F", temp, celsius_to_fahrenheit(*temp));
    }

    twelve_days_of_christmas();
}

fn fibonacci(n: i32) -> i32 {
    // F0 = 0, F1 = 1,
    // Fn = Fn−1 + Fn−2, n > 1
    let mut fib = n;
    if n > 1 {
        fib = fibonacci(n - 1) + fibonacci(n - 2);
    }
    fib
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut balance = 0;
    for number in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me\n{}",
            days[number], gifts[number]
        );

        if number > 0 {
            balance = number - 1
        };

        while balance > 0 {
            println!("{}", gifts[balance]);
            balance -= 1;
        }

        if number > 0 {
            println!("and {}", gifts[balance])
        };
    }
}
