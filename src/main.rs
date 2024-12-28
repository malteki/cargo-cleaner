use std::time::Instant;

use cargo_cleaner::{clean, parse_args, process_results};
use tracing::info;

fn main() -> miette::Result<()> {
  let start = Instant::now();

  // init
  let _ = dotenvy::dotenv(); // this is optional and we dont care if it fails
  let args = parse_args();
  if let Err(err) = tracing_subscriber::fmt()
    .with_max_level(args.log)
    .with_target(false)
    .try_init()
  {
    println!("failed to init tracing_subscriber ({err})")
  };

  // clean
  let clean_dur = Instant::now();
  let results = clean(args.dir, args.max_depth);
  let clean_dur = clean_dur.elapsed();

  // process
  let process = Instant::now();
  if !args.skip_processing {
    info!("processing");
    process_results(results);
  }
  let process = process.elapsed();

  if args.timings {
    info!(
      "timings:\n total: {:.3?}\n  clean: {clean_dur:.3?}\n  process: {process:.3?}",
      start.elapsed()
    );
  }

  Ok(())
}
