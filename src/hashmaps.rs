use std::collections::HashMap;

pub fn run_hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    print_hashmaps(&scores);

    takes_ownership(&mut scores);

    print_hashmaps(&scores);

    get_and_print(&scores, "Green");
}

fn print_hashmaps(hm: &HashMap<String, i32>) {
    for (k, v) in hm.iter() {
        println!("{}: {}", k, v);
    }
}

fn takes_ownership(hm: &mut HashMap<String, i32>) {
    let s = String::from("Green");

    hm.insert(s, 50);
}

fn get_and_print(hm: &HashMap<String, i32>, s: &str) {
    let i_val = hm.get(s).unwrap();

    println!("{} -> {}", s, i_val);
}
