#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Seattle,
}

#[allow(dead_code)]
enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(UsState),
}

impl Message {
    fn call(&self) {
        println!("calling enum");
    }
} 

pub fn run_enums() {
    let m  = Message::Write(String::from("hello"));
    m.call();

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    // let sum = x + y;

    // println!("{}", sum);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    // match with all possible values to limit the bugs that may be caused in the future by
    // calling some method on a value that doesn't exist( Null in other languages)
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}
