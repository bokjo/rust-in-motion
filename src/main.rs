fn main() {
    let name = "Rust in motion";
    let big_name = name.to_uppercase();

    println!("{}", big_name);

    let mut x: i64 = 5;
    x += 1;

    let y: i32 = 42;
    let z = x + y as i64;

    println!("z is {}", z);
}
