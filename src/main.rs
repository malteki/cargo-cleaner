use std::time::Instant;

use cargo_cleaner::{analyze, clean, parse_args};
use tracing::info;

fn main() -> miette::Result<()> {
  let start = Instant::now();
  let _ = dotenvy::dotenv(); // this is optional and we dont care if it fails
  let args = parse_args();
  if let Err(err) = tracing_subscriber::fmt()
    .with_max_level(args.log_level)
    .with_target(false)
    .try_init()
  {
    println!("failed to init tracing_subscriber ({err})")
  };

  let clean_dur = Instant::now();
  let results = clean(args.dir);
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
