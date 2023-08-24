pub fn run_strings() {
    let data = "initial_contents";
    let s = data.to_string();
    println!("{}", s);
    let mut st = String::from("foo");

    println!("Before: {}", st);
    st.push_str("bar");

    println!("After {}", st);

    println!("+ : {}", s + &st);

    let st = print_tic_tac_toe(String::from("tic"), "tac", "toe");
    println!("{}", st);

    indexing_str(&st[0..5]);
    iterate_str(&st[..]);
}

fn print_tic_tac_toe(s1: String, s2: &str, s3: &str) -> String {
    format!("{}-{}-{}", s1, s2, s3)
}

fn indexing_str(s: &str) {
    println!("{}", s); 
}

fn iterate_str(s: &str) {
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!("");

    for c in s.chars() {
        print!("{} ", c);
    } 
    println!("");
}


