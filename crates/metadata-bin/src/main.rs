mod cli;

#[cfg(not(debug_assertions))]
fn main() {
    cli::main();
}

#[cfg(debug_assertions)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    cli::main();

    Ok(())
}
