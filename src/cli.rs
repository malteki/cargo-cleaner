use std::path::PathBuf;

use tracing::Level;

#[derive(Debug, clap::Parser)] // requires `derive` feature
#[command(name = "cargo-cleaner")]
#[command(about = "simple and fast tool to 'cargo clean' all your packages recursively", long_about = None)]
pub struct Args {
  #[arg(
    short = 'd',
    alias = "path",
    env = "CARGO_CLEANER_DIR",
    default_value = ".",
    help = "the directory to start cleaning in"
  )]
  pub dir: PathBuf,

  #[arg(
    short = 'l',
    env = "RUST_LOG",
    default_value = "info",
    help = "level of verbosity (for tracing)"
  )]
  pub log: Level,

  #[arg(
    short = 't',
    default_value_t = false,
    help = "output timing data, requires log=info (or higher)"
  )]
  pub timings: bool,

  #[arg(
    short = 'D',
    alias = "depth",
    help = "maximum depth for cleaning recursively"
  )]
  pub max_depth: Option<usize>,
}
