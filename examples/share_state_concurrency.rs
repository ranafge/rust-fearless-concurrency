use std::{rc::Rc, sync::{Arc, Mutex}, thread, vec};

fn main() {
    let m = Arc::new( Mutex::new(5));
    let mut handles = vec![];
    for _ in 0..10 {
        let m =Arc::clone(&m);
        println!("Ref count {}", Arc::strong_count(&m));
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {:?}", *m.lock().unwrap());

    // {   // mutex lock is smart pointer. MutextGuard implemnet drop trait when mutext guared is out of scope
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // };
    // println!("m = {:?}", m);
}