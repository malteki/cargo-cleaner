use std::time::Instant;

use cargo_cleaner::{clean, parse_args, process_results};
use comfy_table::{presets, Cell, CellAlignment};
use tracing::info;

fn main() -> miette::Result<()> {
  let total_dur = Instant::now();

  // init
  let init_dur = Instant::now();
  let _ = dotenvy::dotenv(); // this is optional and we dont care if it fails
  let args = parse_args();
  if let Err(err) = tracing_subscriber::fmt()
    .without_time()
    .with_max_level(args.log)
    .with_target(false)
    .try_init()
  {
    println!("failed to init tracing_subscriber ({err})")
  };
  let init_dur = init_dur.elapsed();

  // clean
  let clean_dur = Instant::now();
  let results = clean(args.dir, args.max_depth);
  let clean_dur = clean_dur.elapsed();

  // process
  let process_dur = Instant::now();
  if !args.skip_processing {
    info!("processing");
    process_results(results);
  }
  let process_dur = process_dur.elapsed();

  if args.timings {
    let total_dur = total_dur.elapsed();
    let table = comfy_table::Table::new()
      .set_header([
        Cell::new("TOTAL").set_alignment(CellAlignment::Left),
        Cell::new(format!("{:.2}ms", total_dur.as_secs_f64() * 1000.0))
          .set_alignment(CellAlignment::Right),
      ])
      .add_rows(vec![
        [
          Cell::new("init").set_alignment(CellAlignment::Left),
          Cell::new(format!("{:.1}ms", init_dur.as_secs_f64() * 1000.0))
            .set_alignment(CellAlignment::Right),
        ],
        [
          Cell::new("clean").set_alignment(CellAlignment::Left),
          Cell::new(format!("{:.1}ms", clean_dur.as_secs_f64() * 1000.0))
            .set_alignment(CellAlignment::Right),
        ],
        [
          Cell::new("process").set_alignment(CellAlignment::Left),
          Cell::new(format!("{:.1}ms", process_dur.as_secs_f64() * 1000.0))
            .set_alignment(CellAlignment::Right),
        ],
      ])
      .load_preset(presets::NOTHING)
      .to_string();

    info!("\n{table}");
  }

  Ok(())
}
