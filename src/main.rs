use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};
use std::io::{self, Read};
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
    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
    
}
fn main() -> Result<()>{
    let options = Options::parse(); 
    let mut message = String::new(); // [2]
    let eye = if options.dead { "x" } else { "o" };
    if options.stdin {
        io::stdin().read_to_string(&mut message)?; // [2]
    } else {
        message = options.message;
    };
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
