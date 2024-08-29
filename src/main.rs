fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("NUMBER_IN_ENV").map_err(|err| format!("{err}"))?;
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
