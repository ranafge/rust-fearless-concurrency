// Creating a new thread with spawn


use std::thread;
use std::time::Duration;

fn main() { 

    let vec = vec![1,2,3,4,5];
    // os will do the spawn thread put it in backgourd with out executing code.
    thread::spawn(move ||  {
        println!("{:?}", vec);
    });
    // in here vec is drop immediately so in the above spanw thread compile does not find the vec
    // drop(vec);
    // the thread spawn is return JoinHandle that owned value used with join method next.
    let joinhandler = thread::spawn(|| {
        for i in 1..30 {
            println!("Hi number {} from spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // After finished the execution of main thread, it will not wait for other thread to finished the execution.
    for i in 1..5{
        println!("Hi number {} from main thread!", i);
        // thread sleep time duration is allowing the other thread executing time.
        thread::sleep(Duration::from_millis(100));
    }
    joinhandler.join().expect("join handler join is not workded");
}