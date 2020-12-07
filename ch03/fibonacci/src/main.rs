fn main() {
    println!("1st Fibonacci number: {}", fib(0));
    println!("2nd Fibonacci number: {}", fib(1));
    println!("3rd Fibonacci number: {}", fib(2));
    println!("4th Fibonacci number: {}", fib(3));
    println!("5th Fibonacci number: {}", fib(4));
    println!("6th Fibonacci number: {}", fib(5));
    println!("7th Fibonacci number: {}", fib(6));
    println!("8th Fibonacci number: {}", fib(7));
    println!("9th Fibonacci number: {}", fib(8));
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    }
    else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
