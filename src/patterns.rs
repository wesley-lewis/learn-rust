pub fn run_patterns() {
    // Literal, Destructured Arrays, enums, structs or tuples, variables, wildcards,
    // placeholders
    if_let_else();    
    while_let();
    destructuring_structs();
    ref_borrow();
    match_guard();
}

fn if_let_else() {
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _>  = "34".parse();

    if let Some(color) = fav_color {
        println!("Using your favourite color, {} as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }else {
            println!("using orange as the background color");
        }
    } else { 
        println!("Using blue as the backgroud color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };
    let Point{ x: a, y: b} = p;

    println!("a={} b={}", a, b);
}

fn ref_borrow() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("Robot name is still valid: {}", robot_name.unwrap());
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less that five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}


// fn refutable_pattern() {
//     let val: Option<i32> = None;
//     let Some(val) = val;
// }
