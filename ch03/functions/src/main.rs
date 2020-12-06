fn main() {
    another_function(5, 6);

    let a = seven();
    println!("The value of a is: {}", a);

    let b = plus_one(a);
    println!("The value of b is: {}", b);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn seven() -> i32 {
    7
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
