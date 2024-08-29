use eyre::{Result, WrapErr};
fn main() -> Result<()> {
    let var = "NUMBER_IN_ENV";

    let key = std::env::var(var)
        .wrap_err_with(|| format!("Failed to get env var {var}"))
        .expect("more error info");
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
