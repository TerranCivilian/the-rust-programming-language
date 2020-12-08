fn main() {
    // primative stack objects are copied on assignment
    let x = 5;
    let mut y = x;
    y += 1;
    println!("x = {}\ny = {}", x, y); // x and y are two distinct memory locations

    // allocated objects transfer ownership (i.e. move) on assignment
    let s = String::from("hello");
    let t = s;
    println!("{}", t);
    //println!("{}", s); // illegal--s was moved to t
    
    // use clone for a "deep copy"
    let c = t.clone();
    println!("s = {}\nc = {}", t, c);

    // pass ownership to functions
    pass_ownership(t); // can no longer use t
    // and receive ownership from functions
    let r = receive_ownership();
    println!("{}: ownership received from function", r);
}

fn pass_ownership(s: String) {
    println!("{}: ownership transferred to function", s);
}

fn receive_ownership() -> String {
    let tmp = String::from("world");
    tmp // tmp would not be usable if this function was longer
}
