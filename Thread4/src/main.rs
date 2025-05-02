use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let prod = item_count / 2;

    for i in 1..=prod {
        let rand = rand::thread_rng().gen_range(0..100);
        println!("Producer {} produced {}.", id, rand);
        tx.send(rand).unwrap();
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop{
        let rec = rx.lock().unwrap().recv().unwrap();
        if rec == TERMINATION_SIGNAL {
            println!("Consumer {} terminated.", id);
            break;
        }

        else if rec > 0 && rec < 101 {
            println!("Consumer {} processing {}.", id, rec);
            thread::sleep(Duration::from_millis(200));
        }
    }
}


fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut producers = vec![];
    let mut consumers = vec![];
    
    // TODO: Create 2 producer threads
    for i in 1..=2 {
        let txclone = tx.clone();
        let thread = thread::spawn(move ||{
            producer(i, txclone, ITEM_COUNT);    
        });
        producers.push(thread);
    }
    
    // TODO: Create 3 consumer threads
    let mut con_num = 0;
    for i in 1..=3{
        let rxclone = Arc::clone(&rx);
        let thread = thread::spawn(move || {
            consumer(i, rxclone);
        });
        consumers.push(thread);
        con_num += 1;
    }

    // TODO: Wait for all threads to finish
    for thread in producers{
        thread.join().unwrap();
    }
    
    for _ in 0..=con_num {
        let term = tx.clone();
        term.send(TERMINATION_SIGNAL).unwrap();
    }

    drop(tx);

    for thread in consumers{
        thread.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}