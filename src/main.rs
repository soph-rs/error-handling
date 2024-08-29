use anyhow::{Context, Result};
fn main() -> Result<()> {
    let var = "NUMBER_IN_ENV";

    let key = std::env::var(var).with_context(|| format!("Failed to get env var {var}"))?;
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
