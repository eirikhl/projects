fn main() {
    let s = String::from("Hello world");

    // Send &s in order to keep s valid in this scope. If s, then s would be dropped by the function
    let word = first_word_loc(&s);

    let first = &s[0..word];
    println!("{}", first);

    println!("{}", first_word(&s));
}

// Returns the location of the end of the first word in a string
fn first_word_loc(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Returns the first word outright
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
