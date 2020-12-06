#![allow(unused)]
fn main() {
    //
    // Scalar Types: integers, floating-point, Booleans, characters
    //

    // integers
    let a: u8 = 2;
    let b = -12_345i32;
    let c = 0xffff; // default type: i32
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    // floating-point numbers
    let d: f32 = 1.01;
    let e = d * 2.0;
    let f = 34.03; // default type: f64
    println!("d = {}", d);
    println!("e = {}", e);
    println!("f = {}", f);

    // Booleans
    let g = true;
    let h: bool = false;
    println!("g = {}", g);
    println!("h = {}", h);

    // characters
    let i = 'i';
    let j: char = 'j';
    println!("i = {}", i);
    println!("j = {}", j);

    //
    // Compound Types: tuples, arrays
    //

    // tuples
    let k: (i32, f32, bool) = (1, 2.2, true);
    let (l, m, _) = k;
    println!("k = ({}, {}, {})", l, m, k.2);

    // arrays

    let n: [i32; 5] = [1, 2, 3, 4, 5];
    let o: [3; 5]; // [3, 3, 3, 3, 3]
}
