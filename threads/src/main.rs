use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    /* Showcasing threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("Number {} from main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
    */

    /* Showcasing the ability to `move` data into a thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();
    */

    /* Showcasing messaging through channels
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        //let val = String::from("hi");
        //tx.send(val).unwrap();
        //println!("We cannot use {} now because it has been moved", val);

        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec!["this", "is", "another", "sender"];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //let received = rx.recv().unwrap();
    // We don't have to call recv() because we treat rx as an iterator
    for received in rx {
        println!("Got '{}'", received);
    }
    */

    /* Demostrating how to lock data
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    */

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
