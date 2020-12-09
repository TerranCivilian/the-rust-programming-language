fn main() {
    // borrowing by using a reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("`{}` length: {}", s1, len);

    // mutable references are also possible
    let mut s2 = String::from("world");
    add_exclamation(&mut s2);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}
