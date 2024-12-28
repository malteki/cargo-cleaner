use std::path::PathBuf;

#[derive(Debug, clap::Parser)] // requires `derive` feature
#[command(name = "cargo-cleaner")]
#[command(about = "", long_about = None)]
pub struct Args {
  pub path: PathBuf,
}
