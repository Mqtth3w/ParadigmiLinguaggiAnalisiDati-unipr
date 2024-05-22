
fn get_phone_number(name: String, my_phone_book: &[(String, i64)]) -> Result<i64, String> {
    for (n, p) in my_phone_book {
        if *n == name { // for return references -> * to deference
            return Ok(*p);
        }
    }
    return Err(String::from("Name not found"));
}

fn main() {
    let phone_book = &[(String::from("skj"), 5415), (String::from("mlml"), 9484)];
    let r = get_phone_number(String::from("skj"), phone_book);
    match r {
        Ok(n) => println!("found {}" , n),
        Err(e) => println!("{}", e),
    }
}
