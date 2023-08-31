use std::ops::Deref;
use std::rc::Rc;

pub fn run_smart_pointers() {
    println!("Smart pointers");

    // let list = List::Cons(1, 
    //                 Box::new(List::Cons(2,
    //                               Box::new(List::Cons(3,
    //                                             Box::new(List::Nil))))));
    //
    // let x = 5;
    // let y = MyBox::new(x);
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    //
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("Other stuff") };
    drop(c);
    println!("CustomSmartPointer created.");

    using_rct_share_data();
}

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref trait 
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn using_rct_share_data() {
    use crate::smart_pointers::List::{ Cons, Nil };

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
    }
    println!("count after creating a = {}", Rc::strong_count(&a));
}
