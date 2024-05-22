
fn multiple_any(x: i64, ys: &[i64]) -> bool {
    ys.iter().any(|&k| x % k == 0)
}

fn main() {
    let multiples = [3, 5, 7];
    let sum: i64 = (1..)
        .map(|x| x * x)
        .take_while(|&sqx| sqx < 5000)
        .filter(|stuff| multiple_any(*stuff, &multiples))
        .sum();
    println!("Total: {}", sum);
}
