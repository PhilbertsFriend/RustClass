use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));
    for i in 1..=5 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        handles.push(handle);
    }
    let counter_clone = Arc::clone(&counter);
    for handle in handles{
        for i in 1..=10 {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }
    }
    println!("Final Count: {}", *counter.lock().unwrap());
    println!("All threads completed.");
}

/* 
Assignment 1:
Spawn() return JoinHandler 
JoinHandler supports join() method
Need collection to keep all JoinHandlers | Vector

For each joinhandler.join()

Assignment 2:
To make data accessible - 
Use ARC for multiple readers
To update w/ Arc, protect critical section with Mutex() or RwLock()

Assignment 3:
1. Multiple Thread Management
2. Tasks get sent through the channel
3. Each thread waits ("Spins") in infinite loop for instructions
4. Need to send stop signal to break infinite loop. 
Known as clean shutdown, because we are releasing resources back to the OS.
*/