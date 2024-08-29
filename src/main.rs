use dotenvy::dotenv;
use eyre::{eyre, Result, WrapErr};
fn main() -> Result<()> {
    dotenv().ok();
    color_eyre::install()?;

    let var = "NUMBER_IN_ENV";

    let key = std::env::var(var).context(eyre!("Failed to get env var {var}"))?;
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
