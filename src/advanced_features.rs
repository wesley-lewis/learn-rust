use std::slice;

pub fn run_advanced() {
    println!("Advanced features");
    deref_raw_pointer();
    call_dangerous();
    let mut my_slice = vec![1,2,3,4,5,6,6,7];
    let (part1, part2) = split_at_mut(&mut my_slice, 2);
    println!("Part1: {:?} | Part2: {:?}", part1, part2);
    call_extern();
    add_to_count(5);
    unsafe {
        println!("Static variable: {}", COUNTER);
    }
}

// Unsafe Rust
// 4 actions in unsafe rust called unsafe superpowers
// 1) Dereference a raw pointer
// 2) Call an unsafe function or method
// 3) Access or modify a mutable static varaible
// 4) Implement an unsafe trait

// 1) Dereferencing a raw pointer
fn deref_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// 2)Calling an unsafe function or method
unsafe fn dangerous() {
    println!("Dangerous function");
}

fn call_dangerous() {
    unsafe {
        dangerous();
    }
}
// Creating a safe abstraction over unsafe code
// Rust's borrow checker can't understand that we're borrowing different parts of the slice; it
// only knows that we are borrowing from the same slice twice.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

// Using extern functions to call external code
extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_extern() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// let us call this function from C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust Function from C!");
}

// 3) Accessing or modifying a mutable static variable

static HELLO_WORLD: &str = "Hello, World";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
