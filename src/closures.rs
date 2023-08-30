use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>  where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

pub fn run_closures() {
    println!("Closures in rust");
    //
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 6;
    //
    // generate_workout(
    //     simulated_user_specified_value, 
    //     simulated_random_number
    // );
    //
    // simulated_expensive_calculation(10);
    //
    let x = vec![1,2,3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x over here: {:?}", x);
    let y = vec![1,2,3];

    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // we are unnecessarily runnning this function. It might not be required to run this
    // function in the inner if block of outer else statement
    //
    // if we call it wherever it is necessary we are just calling the same code multiple times
    // This is use case of closures!
    let mut expensive_result = Cacher::new( |num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }else {
        if random_number == 3 {
            println!("Take a break today! remember to stay hydrated!");
        }else {
            println!("Today run for {} minutes!", expensive_result.value(intensity));
        }
    }
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

