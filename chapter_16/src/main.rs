use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

fn main() {
    let vector = vec![1, 2, 3, 45, 7, 777];

    let handle = thread::spawn(move || {
        for i in vector {
            println!("Here value {} from vector", i);
        }
    });

    handle.join().unwrap();

    working_with_channels();
    working_with_mutex();
    working_with_shared_mutex();


}

fn working_with_channels() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got message: {}", received);
    }
    // let received = rx.recv().unwrap();
    // println!("Got message: {}", received);
}

fn working_with_mutex(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn working_with_shared_mutex(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
           let mut num = counter.lock().unwrap();

           *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
