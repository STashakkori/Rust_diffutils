// Rust Coreutil Diff
// By: Sina Tashakkori, QVLx Labs

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, BufReader};

// Detect binary data by checking for null bytes (0x00)
fn is_binary(data: &[u8]) -> bool {
  return data.iter().any(|&byte| byte == 0);
}

fn compare_textual_files(file1: &str, file2: &str) -> io::Result<()> {
  let reader1 = BufReader::new(File::open(file1)?);
  let reader2 = BufReader::new(File::open(file2)?);

  let mut line_number = 1;

  for (line1, line2) in reader1.lines().zip(reader2.lines()) {
      let line1 = line1?;
      let line2 = line2?;

      if line1 != line2 {
          println!("{}c{}{}", line_number, line_number + 1, line1);
          println!("---");
          println!("{}{}", line_number, line2);
      }

      line_number += 1;
  }
  return Ok(());
}

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
      eprintln!("Usage: {} <file1> <file2>", &args[0]);
      std::process::exit(1);
  }

  let file1_path = &args[1];
  let file2_path = &args[2];

  let mut data1 = Vec::new();
  let mut data2 = Vec::new();

  File::open(file1_path)?.read_to_end(&mut data1)?;
  File::open(file2_path)?.read_to_end(&mut data2)?;

  if !is_binary(&data1) && !is_binary(&data2) {
      // If both files are not binary (textual), do textual comparison
      compare_textual_files(file1_path, file2_path)?;
  } else if data1 != data2 {
      // If files are binary and different, print the binary difference message
      println!("Binary files {} and {} differ.", file1_path, file2_path);
  }
  return Ok(());
}
