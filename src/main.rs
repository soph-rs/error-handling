use anyhow::Result;
fn main() -> Result<()> {
    let key = std::env::var("NUMBER_IN_ENV")?;
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
