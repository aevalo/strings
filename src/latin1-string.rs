use core::result::Result;
use encoding::all::ISO_8859_1;
use encoding::{EncoderTrap, Encoding};
use std::borrow::Cow;

fn read_latin1_string() -> Result<Vec<u8>, Cow<'static, str>> {
    ISO_8859_1.encode("äÄöÖåÅ", EncoderTrap::Strict)
}

fn latin1_to_string(latin1_data: &[u8]) -> String {
    latin1_data.iter().map(|&c| c as char).collect()
}

fn main() {
    // Read Latin-1 data into a Vec<u8>
    let latin1_data: Vec<u8> = match read_latin1_string() {
        Ok(vec) => vec,
        Err(str) => {
            println!(r#"Failed to encode "{str}""#);
            return;
        }
    };

    // Convert Latin-1 data to a UTF-8 Rust String
    let utf8_string: String = latin1_to_string(&latin1_data);

    println!("Converted string: {utf8_string}");
}
