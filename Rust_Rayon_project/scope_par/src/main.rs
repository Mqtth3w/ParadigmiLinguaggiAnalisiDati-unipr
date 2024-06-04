use rayon::scope;
use rand::Rng;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    scope(|s| {
        let num_threads = 10;
        for i in 0..num_threads {
            let counter = Arc::clone(&counter);
            s.spawn(move |_| {
                let random_value = rand::thread_rng().gen_range(1..=10);
                counter.fetch_add(random_value, Ordering::SeqCst);
                println!("Thread# {} increases by: {}", i, random_value);
            });
        }
    });
    println!("Counter final value: {}", counter.load(Ordering::SeqCst));
}