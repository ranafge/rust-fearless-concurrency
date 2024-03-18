use std::{sync::Mutex, thread, vec};



fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 1..10 {
        // Counter value is moved here 
        let handle = thread::spawn(move  || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result : {}", *counter.lock().unwrap() );
    // counter value borrowed here after move the thread

}