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
    println!("slice {:#?}", s1)
}
