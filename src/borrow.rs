pub fn borrow_fn() {
    let s1 = String::from("Hello");
    let mut s2 = s1;
    // a move similar to shallow copy occurs
    println!("s2: {}", s2);
    // error as s2 becomes the new owner of the value
    // println!("s1: {}", s1);
    let num: i32 = 10; 
    passing_value_to_fn1(num);
    passing_value_to_fn2(s2);
    // if I try to access s2 after this i'll get an error
    // println!("{}", s2);
    s2 = String::from("New string");
    takes_ownership(s2);

    s2 = String::from("Another new string");
    s2 = takes_and_gives_back(s2);
    println!("{}", s2);

    println!("Length of string: {}", calculate_length(&s2));
}

fn passing_value_to_fn1(num: i32) {
    println!("Value is copied: {}", num); 
}

fn passing_value_to_fn2(inp: String) {
    println!("Shallow copy along with move: {}", inp);
}

fn takes_ownership(inp: String) {
    println!("--------------------------");
    println!("{}", inp);
    println!("Takes ownership of the data");
    println!("--------------------------");
}

fn takes_and_gives_back(inp: String) -> String {
    inp
}

// References
fn calculate_length(s1: &str) -> usize {
    // let mut length = 0;
    // for _i in 0..s1.len() {
    //     length = length + 1; 
    // }
    //
    // length

    s1.len()
}
