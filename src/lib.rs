use std::{
  ops::AddAssign,
  path::PathBuf,
  process::{Command, ExitStatus},
};

use byte_unit::Byte;
use clap::Parser;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use tracing::{debug, info, trace, warn};
use walkdir::WalkDir;

pub mod cli;

//

lazy_static::lazy_static! {
  static ref REGEX: Regex = Regex::new(r"Removed (\d+) files(?:, ([\d.]+[A-Za-z]+) total)?").expect("\"regex failed");
}

// functions

pub fn parse_args() -> cli::Args {
  cli::Args::parse()
}

pub fn clean(dir: PathBuf) -> Vec<FileResult> {
  info!("walking {}", dir.display());
  WalkDir::new(dir)
    .into_iter()
    .par_bridge()
    .filter_map(|r| r.ok())
    .filter(|entry| entry.file_type().is_file())
    .filter_map(|file| {
      let filename = file.file_name().to_str();
      if filename == Some("Cargo.toml") || filename == Some("cargo.toml") {
        let out = Command::new("cargo")
          .arg("clean")
          .arg("--manifest-path")
          .arg(format!("{}", file.path().display()))
          .output();
        match out {
          Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let exit_status = out.status;
            trace!(
              "cleaned \"{}\" | stdout: {stdout:?} | stderr: {stderr:?} | exit-status: {exit_status}",
              file.path().display(),
            );
            Some(FileResult::Ok { manifest_path: file.path().to_path_buf() , stdout: out.stdout, stderr: out.stderr, exit_status })
          }
          Err(err) => {
            warn!(
              "Command failed:\n path: {}\n err: {err}",
              file.path().display()
            );
            Some(FileResult::CmdErr {
              err: err.to_string(),
              manifest_path: file.path().to_path_buf(),
            })
          }
        }
      } else {
        None
      }
    })
    .collect::<Vec<_>>()
}

pub fn process_results(collected_files: Vec<FileResult>) {
  let mut file_count = 0usize;
  let mut success_count = 0usize;
  let mut removed = CleanOutput {
    files_removed: 0,
    bytes_removed: 0,
  };

  for x in collected_files.into_iter() {
    file_count += 1;
    match x {
      FileResult::CmdErr { manifest_path, err } => {
        warn!(
          "failed to run Command with {} ({err})",
          manifest_path.display()
        )
      }
      FileResult::Ok {
        manifest_path,
        stdout: _,
        stderr,
        exit_status,
      } => {
        let stderr = String::from_utf8_lossy(&stderr);
        match exit_status.success() {
          true => {
            success_count += 1;
            match CleanOutput::parse(&stderr) {
              Some(parsed) => {
                removed += parsed;
                // debug!("{parsed:?}");
              }
              None => {
                debug!("failed to parse {stderr:?}")
              }
            };
          }
          false => debug!(
            "'cargo clean' exited with error:\n {stderr}\n path: {}",
            manifest_path.display()
          ),
        }
      }
    }
  }

  info!("ran successfully on {success_count}/{file_count} cargo.toml files");

  let adjusted =
    Byte::from_u64(removed.bytes_removed).get_appropriate_unit(byte_unit::UnitType::Decimal);

  info!("removed {} files ({adjusted:.3})", removed.files_removed);
}

//

#[derive(Debug)]
struct CleanOutput {
  files_removed: u32,
  bytes_removed: u64,
}

impl AddAssign for CleanOutput {
  fn add_assign(&mut self, rhs: Self) {
    self.files_removed += rhs.files_removed;
    self.bytes_removed += rhs.bytes_removed;
  }
}

impl CleanOutput {
  fn parse(from: &str) -> Option<Self> {
    let regex_match = REGEX.captures(from)?;

    let files = regex_match.get(1)?.as_str().parse::<u32>().ok()?;
    let size = regex_match
      .get(2)
      .map(|matched| Byte::parse_str(matched.as_str(), false).ok())
      .flatten()
      .map_or(0, |byte| byte.as_u64());

    Some(Self {
      files_removed: files,
      bytes_removed: size,
    })
  }
}

#[allow(unused)]
#[derive(Debug)]
pub enum FileResult {
  CmdErr {
    manifest_path: PathBuf,
    err: String,
  },
  Ok {
    manifest_path: PathBuf,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    exit_status: ExitStatus,
  },
}
