pub fn run_vectors() {
    // creating a new Vec obj
    let mut v: Vec<i32> = Vec::new();
    for i in 0..10 {
        v.push(i);
    }

    println!("{:?}", v);
    iterate_vec(&v);
    // creating a vec using vec! macro
    v = vec![1,2,3];
    println!("{:?}", v);

    println!("{}",v.get(2).unwrap());
    iterate_vec(&v);

    store_multiple_values();
}

fn iterate_vec(v: &Vec<i32>) {
    print!("[");
    for i in v {
        print!("{} ", i);
    }
    println!("]");
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn store_multiple_values() {
    let row = vec![
        SpreadsheetCell::Int(45),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];    
    
    print_enum(&row);
} 

fn print_enum(v: &Vec<SpreadsheetCell>) {
    for i in v {
        match i {
            SpreadsheetCell::Int(val) => println!("{}", val),
            SpreadsheetCell::Text(val) => println!("{}", val),
            SpreadsheetCell::Float(val) => println!("{}", val),
        }
    }
}
