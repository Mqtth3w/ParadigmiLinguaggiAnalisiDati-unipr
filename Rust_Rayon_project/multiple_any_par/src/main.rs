
use std::time::Instant;
use rayon::prelude::*;

fn multiple_any(x: i64, ys: &[i64]) -> bool {
    ys.iter().any(|&k| x % k == 0)
}

fn main() {
    let sum_limit = 5000;
    let multiples = [3, 5, 7];
    //sequential
    let mut start_time = Instant::now(); 
    let sum: i64 = (1..)
        .map(|x| x * x)
        .take_while(|&sqx| sqx < sum_limit)
        .filter(|&sqx| multiple_any(sqx, &multiples))
        .sum();
    let mut elapsed_time = start_time.elapsed();
    println!("Total: {}", sum); 
    println!("Sequential time: {:?}", elapsed_time); 
    //parallel
    start_time = Instant::now(); 
    let sum_par: i64 = (1..1000000000000000)
        .into_par_iter()
        .map(|x| x * x)
        .take_while(|&sqx| sqx < sum_limit) //not working, not feasible in //
        .filter(|&sqx| multiple_any(sqx, &multiples))  
        .sum();
    elapsed_time = start_time.elapsed();
    println!("Total in parallel: {}", sum_par); 
    println!("Parallel time: {:?}", elapsed_time); 
}
