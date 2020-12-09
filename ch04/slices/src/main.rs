fn main() {
    let test_string = String::from("this is a test string!");
    let first_word = get_first_word(&test_string[..]);
    println!("first word = `{}`", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
