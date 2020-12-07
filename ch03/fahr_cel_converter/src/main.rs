fn main() {
    let f = 32.0;
    println!("{} degrees Fahrenheit = {} degrees Celsius.", f, f_to_c(f));
    let f = 57.0;
    println!("{} degrees Fahrenheit = {} degrees Celsius.", f, f_to_c(f));

    let c = 0.0;
    println!("{} degrees Celsius = {} degrees Fahrenheit.", c, c_to_f(c));
    let c = 24.0;
    println!("{} degrees Celsius = {} degrees Fahrenheit.", c, c_to_f(c));
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn c_to_f(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}
