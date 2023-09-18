use regex::{Match, Regex};
use std::borrow::Cow;

// "Please contact me at john.doe@example.com or jane.doe@foo.bar."
fn anonymize_emails(s: &mut str) {
    let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap();
    // Collect matches
    let temp = s.to_owned();
    let matches: Vec<Match> = re.find_iter(temp.as_str()).collect();
    // Now that we have the matches, perform the replacements.
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    for range in matches {
        let replacement = vec![b'*'; range.end() - range.start()];
        s_bytes[range.start()..range.end()].copy_from_slice(&replacement);
    }
}

fn sanitize(input: &str) -> Cow<str> {
    if input.contains("badword") {
        let sanitized: String = input.replace("badword", "****");
        return Cow::Owned(sanitized);
    }
    Cow::Borrowed(input)
}

fn main() {
    let mut input = String::from("Please contact me at john.doe@example.com or jane.doe@foo.bar.");
    println!("{input}");
    anonymize_emails(input.as_mut());
    println!("{input}");

    println!("{}", sanitize("goodword"));
    println!("{}", sanitize("badword"));
}
