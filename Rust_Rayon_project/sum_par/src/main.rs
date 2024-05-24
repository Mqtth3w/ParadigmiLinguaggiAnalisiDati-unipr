use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (0..100).collect();
    //parallel
    let mut start_time = Instant::now(); 
    let mut sum: i32 = numbers.par_iter().sum();
    let mut elapsed_time = start_time.elapsed();
    println!("The sum is: {}", sum);
    println!("Elapsed time: {:?}", elapsed_time);

    //sequential
    start_time = Instant::now(); 
    sum = numbers.iter().sum();
    elapsed_time = start_time.elapsed();
    println!("The parallel sum is: {}", sum);
    println!("Elapsed parallel time: {:?}", elapsed_time);
}