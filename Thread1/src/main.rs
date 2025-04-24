use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    let mut handles = vec![];
    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("All threads completed.");
}