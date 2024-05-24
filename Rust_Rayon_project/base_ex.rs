
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (0..100).collect();
    let sum: i32 = numbers.par_iter().sum();
    println!("The sum is: {}", sum);
}

