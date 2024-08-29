use dotenvy::dotenv;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    color_eyre::install()?;

    let var = "NUMBER_IN_ENV";

    let key = std::env::var(var)?;
    let number = key.parse::<i32>()?;

    println!("\"NUMBER_IN_ENV\" is {}", number);

    Ok(())
}
