use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    initial: T,
    trigger: bool,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(initial: T) -> Self {
        ComputeCache {
            initial,
            trigger: false,
        }
    }

    fn get_result(&mut self) -> String {
        match self.trigger {
            true => {
                println!("Retrieving from cache.");
                let copy = &self.initial;
                copy().to_string()
               // "Done!".to_string()
            }

            false => {
            println!("Searching cache (this will take 10 seconds)...");
            thread::sleep(Duration::from_secs(10));
            self.trigger = true;
            "Found.".to_string()
            }
        }
    }
}

fn Compute_Cache() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take about 5 seconds)...");
        thread::sleep(Duration::from_secs(5));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}

fn operation(){
    let ex1 = |x:i32, y:i32| x * y;
    println!("x * y = {}", ex1(5, 10));
}

fn track_changes(){
    let mut inc = 0;
    let mut update = || {
        inc += 1;
        println!("Count = {}", inc);
    };
    update();
    update();
}

fn process_vector_map<T>(vec: Vec<i32>, t: T) -> Vec<i32>
where 
    T: Fn(i32) -> i32,
{
vec.into_iter().map(t).collect()
}

fn process_vector_forloop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}

fn process_both_vectors(){
    let numbers = vec![5, 10, 15];
    let doublem = process_vector_map(numbers.clone(), |x| {x * 2});
    let replacem = process_vector_map(numbers.clone(), |x| {
        if x > 2 {
            let x = 0;
            x
        }
        else {x}
    });
    let doublef = process_vector_forloop(numbers.clone(), |x| {x * 2});
    let replacef = process_vector_forloop(numbers, |x| {
        if x > 2 {
            let x = 0;
            x
        }
        else {x}
    });
    println!("Doubled Map Vector: {:?}", doublem);
    println!("Replaced Map Vector: {:?}", replacem);
    println!("Doubled Loop Vector: {:?}", doublef);
    println!("Replaced Loop Vector: {:?}", replacef);
}



fn main() {
    operation();
    track_changes();
    process_both_vectors();
    Compute_Cache();
}
