use std::fs::File;
use std::io::{self, Error};

fn main() -> Result<(), Error> {
    for x in 1..=34 {
        let url = format!("http://www.dspguide.com/CH{}.PDF", x);
        let file_name = url.split('/').last().unwrap();

        let mut file_buffer = File::create(file_name)?;
        let mut response = reqwest::blocking::get(url).unwrap();

        assert!(response.status().is_success());

        io::copy(&mut response, &mut file_buffer).expect("Unable to copy data");
    }

    Ok(())
}
