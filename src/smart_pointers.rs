use std::ops::Deref;

pub fn run_smart_pointers() {
    println!("Smart pointers");

    let list = List::Cons(1, 
                    Box::new(List::Cons(2,
                                  Box::new(List::Cons(3,
                                                Box::new(List::Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

enum List {
    Cons(i32, Box<List>),
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
