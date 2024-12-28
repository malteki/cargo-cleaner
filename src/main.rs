use std::{env, path::PathBuf, time::Instant};

use cargo_cleaner::{analyze, clean};
use tracing::info;

fn main() -> miette::Result<()> {
  let start = Instant::now();
  if let Err(err) = dotenvy::dotenv() {
    println!("failed to load .env file ({err})")
  };
  tracing_subscriber::fmt::init();
  let dir = PathBuf::from(env::var("PROJECTS_DIR").expect("$PROJECTS_DIR must be set"));

  let clean_dur = Instant::now();
  let results = clean(dir);
  let clean_dur = clean_dur.elapsed();

  let process = Instant::now();
  info!("processing");
  analyze(results);
  let process = process.elapsed();

  info!(
    "finished after {:?} (clean: {clean_dur:?} + process: {process:?})",
    start.elapsed()
  );

  Ok(())
}
