mod cli;

#[cfg(any(
    target = "aarch64-unknown-linux-musl",
    target = "x86_64-unknown-linux-musl"
))]
#[global_allocator]
static GLOBAL_ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

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
