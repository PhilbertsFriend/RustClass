use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Terminating all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} received a job.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} terminating.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

fn checkWebsite(
    url: String,
    timeout: Duration,
    retries: u32,
    tx: mpsc::Sender<WebsiteStatus>,
) {
    let client = Client::builder()
        .timeout(timeout)
        .build()
        .unwrap();

    let mut attempts = 0;
    let mut result: Result<StatusCode, String> = Err("Unreachable".to_string());
    let start = Instant::now();

    while attempts <= retries {
        match client.get(&url).send() {
            Ok(response) => {
                result = Ok(response.status());
                break;
            }
            Err(e) => {
                result = Err(format!("Error: {}", e));
                attempts += 1;
                if attempts > retries {
                    break;
                }
            }
        }
    }

    let elapsed = start.elapsed();
    let status = WebsiteStatus {
        url,
        status: result.map(|s| s.as_u16()).map_err(|e| e.to_string()),
        response_time: elapsed,
        timestamp: SystemTime::now(),
    };

    tx.send(status).unwrap();
}

fn parse_urls_from_args() -> io::Result<Vec<String>> {
    let mut args = env::args().skip(1);
    let mut urls = Vec::new();

    while let Some(arg) = args.next() {
        if arg == "--file" {
            if let Some(path) = args.next() {
                let file = File::open(&path)?;
                for line in io::BufReader::new(file).lines() {
                    let line = line?.trim().to_string();
                    if !line.is_empty() && !line.starts_with('#') {
                        urls.push(line);
                    }
                }
            }
        } else {
            urls.push(arg);
        }
    }

    Ok(urls)
}

fn write_statuses_to_json(statuses: &[WebsiteStatus]) -> io::Result<()> {
    let mut file = File::create("status.json")?;
    writeln!(file, "[")?;

    for (i, status) in statuses.iter().enumerate() {
        let json = format!(
            "  {{\n    \"url\": \"{}\",\n    \"status\": {},\n    \"response_time_ms\": {},\n    \"timestamp\": \"{:?}\"\n  }}{}",
            status.url,
            match &status.status {
                Ok(code) => code.to_string(),
                Err(e) => format!("\"{}\"", e),
            },
            status.response_time.as_millis(),
            status.timestamp,
            if i + 1 == statuses.len() { "" } else { "," }
        );
        writeln!(file, "{}", json)?;
    }

    writeln!(file, "]")
}

fn main() {
     let urls = match parse_urls_from_args() {
        Ok(urls) => urls,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            return;
        }
    };

    if urls.is_empty() {
        println!("No URLs provided.");
        return;
    }

    let thread_count = 10;
    let timeout = Duration::from_secs(5);
    let max_retries = 2;

    let pool = ThreadPool::new(thread_count);
    let (status_tx, status_rx) = mpsc::channel();

    for url in urls {
        let tx = status_tx.clone();
        let url = url.to_string();
        pool.execute(move || {
            checkWebsite(url, timeout, max_retries, tx);
        });
    }

    drop(status_tx);

    let mut results = Vec::new();
    for status in status_rx.iter() {
        println!("{:?}", status);
        results.push(status);
    }

    if let Err(e) = write_statuses_to_json(&results) {
        eprintln!("Failed to write status.json: {}", e);
    } else {
        println!("Results written to status.json.");
    }

    println!("Done checking websites.");
}