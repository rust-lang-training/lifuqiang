use std::sync::mpsc;
use std::thread;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let max = 1_000_000;
    let (tx, rx) = mpsc::channel();

    let num_threads = 8;
    let chunk_size = max / num_threads;

    let mut threads = Vec::new();
    for i in 0..num_threads {
        let start = i * chunk_size + 1;
        let end = if i == num_threads - 1 { max } else { (i + 1) * chunk_size };
        let sender = tx.clone();
        threads.push(thread::spawn(move || {
            for n in start..=end {
                if is_prime(n) {
                    sender.send(n).unwrap();
                }
            }
        }));
    }

    let mut primes = Vec::new();
    for _ in 0..num_threads * chunk_size {
        primes.push(rx.recv().unwrap());
    }

    println!("Prime numbers up to 1,000,000:");
    println!("{:?}", primes);
}
