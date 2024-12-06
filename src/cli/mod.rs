mod csv;
mod gen_pass;
mod base64;
mod text;

use std::path::{Path, PathBuf};
use clap::Parser;
pub use csv::{CsvOpts, OutputFormat};
pub use gen_pass::{GenPassOpts};
pub use base64::{Base64SubCommand, Base64Format};
pub use text::{TextSubCommand, TextSignFormat};



#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

fn verify_file(file_name: &str) -> Result<String, &'static str>{
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
         Err("File not found")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str>{
    let p = Path::new(path);
    if Path::new(path).exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("path not exist")
    }
}