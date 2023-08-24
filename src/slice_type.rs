pub fn run_slice_fns() {
    let s1 = String::from("Hello, world");
    println!("{}", first_word(&s1));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Rules of References: 
// 1) At any given time, you can have either but not both of the following: one mutable reference
//    or any number of immutable references
//    2) References must always be valid
