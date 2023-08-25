use std::fmt::Display;

pub fn run_lifetimes() {
    simple_ex();
    
    let s1 = String::from("hello world i'm groot");
    let s2 = "small world";

    let s3 = longest(&s1, s2);
    println!("longest string is '{}'", s3);

    let s4 = longest_string(&s1, s2);
    println!("longest string is {}", s4); 

    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence: &str = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}", i);
    println!("Novel: {}", novel);

    let s5 = ImportantExcerpt::longest_with_an_announcement(&s1, s2, "Hello world!");
    println!("Longest: {}", s5);
}

fn simple_ex() {
    let x = 5;

    let r = &x;

    println!("{}", r);
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn longest_string(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()
    }else {
        y.to_string()
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl <'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn longest_with_an_announcement<'c, T> (x: &'c str, y: &'c str, ann: T) -> &'c str 
        where T: Display {
            println!("Announcement: {}", ann);
            if x.len() > y.len() {
                x
            }else {
                y
            }
        }
}
