use std::thread;

fn main() {
    println!(" Channel passes data between thread");
    // Tx is call upstream rx is call downstream.
    // channel will be drop or close either one of tx or rx drop

    let (tx, rx) = std::sync::mpsc::channel();
   
    thread::spawn(move || {
        let val = String::from("hi");
        // send is return a Result when it is drop its return an error
        tx.send(val).unwrap();
    });

    // println!("Received value is {}", rx.recv().unwrap());

    // try_recv() does not wait for the recinving value except recv

    // Then try to send multiple value

    let (tx, rx) = std::sync::mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let val = vec![1, 2, 3, 4, 5, 6, 7,8];
        // send is return a Result when it is drop its return an error
        tx.send(val).unwrap();
        
    });
    
    thread::spawn(move || {
        let val = vec![1, 2, 3, 4, 5, 6, 7,111111118];
        // send is return a Result when it is drop its return an error
        tx2.send(val).unwrap();
    }).join().unwrap();
    // here need join() method because main thread completed before the completion complet the above thread


    
    for v_item in rx.recv().unwrap() {
        println!("Received value is {:?}", v_item);
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_main() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            // send is return a Result when it is drop its return an error
            tx.send(val).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), "hi");

        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            let val = vec![1, 2, 3, 4, 5, 6, 7];
            // send is return a Result when it is drop its return an error
            tx.send(val).unwrap();
        });

        assert_eq!(rx.recv().unwrap(),  vec![1, 2, 3, 4, 5, 6, 7])
    }
}
// https://doc.rust-lang.org/book/ch16-03-shared-state.html