
use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;

fn estimate_pi_seq(samples: u64) -> f64 {
    let mut inside_circle = 0;

    let mut rng = rand::thread_rng();
    for _ in 0..samples {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    4.0 * (inside_circle as f64) / (samples as f64)
}

fn estimate_pi_par(samples: u64) -> f64 {
    let inside_circle: u64 = (0..samples)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            if x * x + y * y <= 1.0 {
                1
            } else {
                0
            }
        })
        .sum();

    4.0 * (inside_circle as f64) / (samples as f64)
}

fn main() {
    let all_samples = [100, 500_000, 1_000_000, 3_000_000, 5_000_000];
    for samples in all_samples {
        println!("Samples: {}", samples);
        //execute sequential
        let mut start = Instant::now();
        let mut pi = estimate_pi_seq(samples);
        let mut duration = start.elapsed();
        println!("Estimated Pi (sequential): {}", pi);
        println!("Time taken: {:?}", duration);
        //execute parallel
        start = Instant::now();
        pi = estimate_pi_par(samples);
        duration = start.elapsed();
        println!("Estimated Pi (parallel): {}", pi);
        println!("Time taken: {:?}", duration);
    }
}
