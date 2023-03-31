use regex::Regex;

#[derive(Debug)]
pub struct IntegerPattern {
    pub url: String,
    pub start_index: usize,
    pub end_index: usize,
    pub start_int: i32,
    pub end_int: i32,
    pub pad: usize,
}
impl IntegerPattern {
    fn count_pad(integer_str: &str) -> usize {
        for (i, char) in integer_str.chars().enumerate() {
            if char != '0' {
                return i;
            }
        }
        integer_str.len()
    }
    fn new(raw_pattern: &str, start_index: usize, end_index: usize) -> Self {
        let (start_str, end_str) = raw_pattern[start_index..end_index]
            .trim_matches(['[', ']'].as_slice())
            .split_once(':')
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
pub fn parse_patterns(url: &str) -> Vec<IntegerPattern> {
    Regex::new(r"\[[0-9]+:[0-9]+\]")
        .unwrap()
        .find(url)
        .into_iter()
        .map(|needle| IntegerPattern::new(url, needle.start(), needle.end()))
        .collect()
}
