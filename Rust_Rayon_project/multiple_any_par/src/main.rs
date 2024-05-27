
use std::time::Instant;
use rayon::prelude::*;

fn multiple_any(x: i64, ys: &[i64]) -> bool {
    ys.iter().any(|&k| x % k == 0)
}

fn main() {
    let sum_limit = 10000000000;
    let multiples = [3, 5, 7];
    //sequential
    let mut start_time = Instant::now(); 
    let mut sum: i64 = (1..)
        .map(|x| x * x)
        .take_while(|&sqx| sqx < sum_limit)
        .filter(|&sqx| multiple_any(sqx, &multiples))
        .sum();
    let mut elapsed_time = start_time.elapsed();
    println!("Total: {}", sum); 
    println!("Sequential time: {:?}", elapsed_time); 
    //println!("Sequential time: {:?}", elapsed_time.as_millis());
    //parallel (is not really parallel the seq is faster)
    start_time = Instant::now(); 
    sum = (1..)
        .map(|x| x * x)
        .take_while(|&sqx| sqx < sum_limit)
        .filter(|&sqx| multiple_any(sqx, &multiples))  
        .par_iter() // mmm?
        .sum();
    elapsed_time = start_time.elapsed();
    println!("Total in parallel: {}", sum); 
    println!("Parallel time: {:?}", elapsed_time); 
    //println!("Sequential time: {:?}", elapsed_time.as_millis());
}
