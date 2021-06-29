use std::io;

fn main() {
    let name = "Rust in motion";
    let big_name = name.to_uppercase();

    println!("{}", big_name);

    let mut x: i64 = 5;
    x += 1;

    let y: i32 = 42;
    let z = x + y as i64;

    println!("z is {}", z);

    println!("Builtin basic types...");
    println!("1. Bool");
    let t = true;
    let nt = !t;
    println!("{} != {}", t, nt);

    println!("2. Integers");
    let i_8: i8 = 127;
    let i_16: i16 = 1;
    let i_32: i32 = 1;
    let i_64: i64 = 1;
    let i_128: i128 = 1;

    println!("i8 is {}", i_8);

    println!("3. Floating point numbers");
    let f_32: f32 = 2.2;
    let f_64: f64 = 4.2;

    println!("f_32: {} + {} = {}", f_32, f_64, f_32 + f_64 as f32);
    println!("f_64: {} + {} = {}", f_32, f_64, f_32 as f64 + f_64);

    println!("4. Characters");
    let char_a = 'a';
    let char_b = 'b';

    println!("Builtin compound types...");
    println!("1. Tuples");

    let my_tup = (1, 'c', true);
    println!("MyTuple: {:#?}", my_tup);
    println!("first from MyTuple: {}", my_tup.0);
    println!("second from MyTuple: {}", my_tup.1);

    println!(" destructuring example with tuple");

    let (aa, bb, cc) = my_tup;

    println!("2. Arrays");

    let my_arr = [1, 0, -1];
    println!("MyArray: {:#?}", my_arr);

    println!("3. Slices");
    let a1 = [100, 200, 300];
    let s1 = &a1[0..2];
    println!("slice {:#?}", s1);

    println!("Functions...");
    next_birthday("Torko", 12);
    println!("Sum: {}", add(12, 1));
    println!("Control flow in Rust...");
    println!("1. IF-ELSE");
    discount(3);
    if_else_test(2);

    println!("2. Looping...");
    my_loop_fn(42);
    secret_word_with_loop();
    secret_word_with_while();
    my_for_loop_example();

    println!("3. Match...");
    println!("[MATCH] Pattern Matching...");
    let three = 3;

    match three {
        1 => println!("matched 1"),
        2 => println!("matched 2"),
        3 => println!("matched 3"),
        _ => println!("matched whatever"),
    }

    let dice1 = 1;
    let dice2 = 5;

    match (dice1, dice2) {
        (1, 1) => println!("Shame, rolled lowest!"),
        (5, _) | (_, 5) => {
            println!("Great, rolled at lest one 5... please move and roll again!")
        }
        _ => print!("Just move, no extra roll!"),
    }

    println!("[MATCH] Exhaustiveness checking...");
    let is_confirmed = true;
    let is_active = false;

    match (is_confirmed, is_active) {
        (true, true) => println!("This account is in good standing."),
        (false, true) => println!("Please confirm your account!"),
        // (true, false) => {
        //     println!("If not implemented the compiler will yell for not covered pattern! if default match not provided!")
        // }
        (false, false) => println!("This account will be deactivated!"),
        _ => {}
    }
}

fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("Hello {}, bla bla {}!", name, next_age);
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 { 50 } else { 10 };

    println!("Your discount is {}!", amount);
}

fn if_else_test(num: u8) {
    if num % 2 == 0 {
        println!("{} is even number!", num);
    } else {
        println!("{} is odd number!", num);
    }
}

fn my_loop_fn(count: u8) {
    let mut until = 0;
    loop {
        println!("Hello count: {}", until);
        until += 1;
        if until == count {
            break;
        };
    }
}

fn secret_word_with_loop() {
    loop {
        println!("What is the secret word?");
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        if word.trim() == "rust" {
            break;
        }
    }

    println!("Great you guess correctly, 10 points");
}

fn secret_word_with_while() {
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What is the secret word?");
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
    }

    println!("Great you guess correctly, 10 points");
}

fn my_for_loop_example() {
    for i in 0..10 {
        println!("Serving #nr {}", i)
    }
}
