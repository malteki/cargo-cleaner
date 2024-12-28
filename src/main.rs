use std::time::Instant;

use cargo_cleaner::{clean, parse_args, process_results};
use tracing::info;

fn main() -> miette::Result<()> {
  let start = Instant::now();

  // init
  let _ = dotenvy::dotenv(); // this is optional and we dont care if it fails
  let args = parse_args();
  if let Err(err) = tracing_subscriber::fmt()
    .with_max_level(args.log_level)
    .with_target(false)
    .try_init()
  {
    println!("failed to init tracing_subscriber ({err})")
  };

  // clean
  let clean_dur = Instant::now();
  let results = clean(args.dir);
  let clean_dur = clean_dur.elapsed();

  // process
  let process = Instant::now();
  info!("processing");
  process_results(results);
  let process = process.elapsed();

  info!(
    "finished after {:?} (clean: {clean_dur:?} + process: {process:?})",
    start.elapsed()
  );

  Ok(())
}
