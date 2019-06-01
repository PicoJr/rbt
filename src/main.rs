extern crate clap;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

use clap::{App, Arg};

use rbt::pattern::Pattern;

const BUFFER_SIZE: usize = 4000; // 4KB

fn main() {
    let matches = App::new("ReplaceBytesTool")
        .version("1.0")
        .author("PicoJr")
        .about("Replace bytes periodically in file")
        .arg(Arg::with_name("INPUT").help("input file").required(true))
        .arg(Arg::with_name("OUTPUT").help("output file").required(true))
        .arg(
            Arg::with_name("pattern")
                .help("<value>,<mask>,<periodicity>,<offset> ")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();
    let input_file = File::open(input_file_path);

    let output_file_path = matches.value_of("OUTPUT").unwrap();
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_path);

    let pattern_string = matches.values_of("pattern").unwrap();
    let mut patterns: Vec<Pattern> = vec![];
    for p in pattern_string {
        match Pattern::from_string(p) {
            Ok(pattern) => {
                patterns.push(pattern);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    match (input_file, output_file) {
        (Err(e), _) => println!("error while opening {}: {}", input_file_path, e),
        (_, Err(e)) => println!("error while opening {}: {}", output_file_path, e),
        (Ok(input), Ok(output)) => {
            let mut reader = BufReader::new(input);
            let mut writer = BufWriter::new(output);

            let mut position = 0;
            let mut buffer = [0; BUFFER_SIZE];

            while let Ok(size) = reader.read(&mut buffer) {
                if size == 0 {
                    break;
                }
                for (i, item) in buffer.iter_mut().enumerate().take(size) {
                    for pattern in &patterns {
                        *item = pattern.compute_pattern(position + i, *item);
                    }
                }
                position += size;
                writer.write_all(&buffer[..size]).expect("write error");
            }
        }
    }
}
