// Creating a new thread with spawn


use std::thread;
use std::time::Duration;

fn main() { 
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // After finished the execution of main thread, it will not wait for other thread to finished the execution.
    for i in 1..5{
        println!("Hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}