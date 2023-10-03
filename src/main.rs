use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};
#[derive(Parser)]
struct Options{
    #[clap(default_value = "Meow!")]
    message: String, // [1]
    /// What does the cat say?
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,    
    
}
fn main() -> Result<()>{
    let options = Options::parse(); // [2]
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    };
    match &options.catfile {
        Some(path) => {
            std::fs::read_to_string(path).with_context(
                || format!("could not read file {:?}", path)
            )?; // [3]
            let cat_template = std::fs::read_to_string(path)?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", cat_picture);
        },
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye=eye.red().bold());
            println!("    =( I )=");
        }
    };
    Ok(())
}
