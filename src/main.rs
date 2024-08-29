use eyre::{eyre, Result, WrapErr};
fn main() -> Result<()> {
    let var = "NUMBER_IN_ENV";

    let key = std::env::var(var).wrap_err(eyre!("Failed to get env var {var}"))?;
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
