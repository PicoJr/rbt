extern crate clap;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

use clap::{App, Arg};

use rbt::pattern::Pattern;

const BUFFER_SIZE: usize = 4000; // 4KB

fn remove_leading_zeros(v: &[u8]) -> Vec<u8> {
    let mut non_zero_found = false;
    let mut rv: Vec<u8> = vec![];
    for &e in v {
        if e != 0 {
            non_zero_found = true;
            rv.push(e);
        } else if e == 0 && non_zero_found {
            rv.push(e);
        }
    }
    rv
}

fn main() {
    let matches = App::new("ReplaceBytesTool")
        .version("1.0")
        .author("PicoJr")
        .about("Replace bytes periodically in file")
        .arg(Arg::with_name("INPUT").help("input file").required(true))
        .arg(Arg::with_name("OUTPUT").help("output file").required(true))
        .arg(Arg::with_name("pattern").multiple(true).required(true))
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
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
        let mut pattern_init = p.split(',');
        let int_value: usize =
            (pattern_init.next().unwrap().parse::<usize>()).expect("could not parse pattern");
        let value: [u8; 8] = int_value.to_be_bytes();
        let value = remove_leading_zeros(&value.to_vec());
        let int_mask: usize =
            (pattern_init.next().unwrap().parse::<usize>()).expect("could not parse pattern");
        let mask: [u8; 8] = int_mask.to_be_bytes();
        let mask = remove_leading_zeros(&mask.to_vec());
        let periodicity: usize =
            (pattern_init.next().unwrap().parse::<usize>()).expect("could not parse pattern");
        let offset: usize =
            (pattern_init.next().unwrap().parse::<usize>()).expect("could not parse pattern");

        patterns.push(Pattern::new(value, mask, periodicity, offset));
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
