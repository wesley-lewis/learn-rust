use std::thread;
use std::rc::Rc;
use std::sync::{Mutex, Arc};

pub fn run_mutexes() {
    println!("Mutexes in rust"); 
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    atomic_references();
}

//
// fn sharing_bw_multiple_threads() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];
//
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//     println!("Result: {}", *counter.lock().unwrap());
// }

fn atomic_references() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
