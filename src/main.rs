fn main() {
    let key = std::env::var("NUMBER_IN_ENV").unwrap();
    let number = key.parse::<i32>().unwrap();

    println!("\"NUMBER_IN_ENV\" is {}", number);
}
