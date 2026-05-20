use color_eyre::eyre::Context;
use color_eyre::eyre::Ok;
use color_eyre::eyre::Result;

mod crossterm;

fn main() -> Result<()> {
    println!("Hello, world!");
    color_eyre::install()?;
    let _ = ratatui::run(crossterm::run).context("failed to run app");
    Ok(())
}
