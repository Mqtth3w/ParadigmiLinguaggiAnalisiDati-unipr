
fn get_phone_number(name: String, my_phone_book: Vec<(String, i64)>) -> Result<i64, String> {
    for (n, p) in my_phone_book {
        if n == name {
            return Ok(p);
        }
    }
    return Err("Name not found".to_string());
}

fn main() {
    let phone_book = vec![("skj".to_string(), 5415), ("mlml".to_string(), 9484)];
    let r = get_phone_number("skj".to_string(), phone_book);
    match r {
        Ok(n) => println!("found {}" , n),
        Err(e) => println!("{}", e),
    }
}
