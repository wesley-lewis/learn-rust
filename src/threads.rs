use std::thread;
use std::time::Duration;

pub fn run_threads() {
    println!("Threads running");
    
    // return type of thread::spawn is a JoinHandle 
    // The join() method can be called to wait for the newly generated thread to complete
    // before the main thread exits.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // the join() method will block the current thread currently running until the thread
    // represented by the handle terminates.
    handle.join().unwrap();

    closure_with_move_thread();
}

fn closure_with_move_thread() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// message passing
