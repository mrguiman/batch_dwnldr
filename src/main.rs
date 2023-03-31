use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, Error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to download from. Supports the following patterns:
    /// [start:end]: where start and end are an integer range. For each number within the range, the pattern will be replaced by the number and download will occur.
    /// [1:10]: 1,2,3,4,5,6,7,9,10
    /// [01:10]: 01,02,03,04,05,06,07,08,09,10
    #[clap(verbatim_doc_comment)]
    url: String,
}
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let patterns = parse_patterns(&args.url);

    // TODO figure out what to do with multiple patterns
    if let Some(pattern) = patterns.get(0) {
        for x in pattern.start_int..=pattern.end_int {
            let mut download_url = pattern.url.clone();
            download_url.replace_range(
                pattern.start_index..pattern.end_index,
                &format!("{:0pad$}", x, pad = pattern.pad),
            );

            if let Err(e) = download_content(&download_url) {
                println!("Error while downloading file at {}: {}", download_url, e);
            }
        }
    }
    Ok(())
}
#[derive(Debug)]
struct IntegerPattern {
    url: String,
    start_index: usize,
    end_index: usize,
    start_int: i32,
    end_int: i32,
    pad: usize,
}
impl IntegerPattern {
    fn count_pad(integer_str: &str) -> usize {
        for (i, char) in integer_str.chars().enumerate() {
            if char != '0' {
                return i;
            }
        }
        return integer_str.len();
    }
    fn new(raw_pattern: &str, start_index: usize, end_index: usize) -> Self {
        let (start_str, end_str) = raw_pattern[start_index..end_index]
            .trim_matches(['[', ']'].as_slice())
            .split_once(":")
            .expect("Invalid pattern format");

        IntegerPattern {
            url: raw_pattern.to_string(),
            start_index,
            end_index,
            start_int: start_str.parse::<i32>().unwrap_or(0),
            end_int: end_str.parse::<i32>().unwrap_or(0),
            pad: IntegerPattern::count_pad(start_str),
        }
    }
}
fn parse_patterns(url: &str) -> Vec<IntegerPattern> {
    let integer_pattern_regex: Regex = Regex::new(r"\[[0-9]+:[0-9]+\]").unwrap();
    integer_pattern_regex
        .find(url)
        .into_iter()
        .map(|needle| IntegerPattern::new(url, needle.start(), needle.end()))
        .collect()
}

fn download_content(url: &str) -> Result<(), Error> {
    let file_name = url.split('/').last().unwrap();
    let mut file_buffer = File::create(file_name)?;
    let mut response = reqwest::blocking::get(url).unwrap();
    assert!(response.status().is_success());
    io::copy(&mut response, &mut file_buffer).expect("Unable to copy data");
    Ok(())
}
