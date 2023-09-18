use regex::Regex;

fn main() {
    let text1 = "He said \"goodbye\" and left.";
    println!("{text1}");
    let text2 = r#"He said "goodbye" and left."#;
    println!("{text2}");

    let re = Regex::new(r#"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})"#).unwrap();
    let re_input = "2010-03-14";
    println!("Input: {re_input}");
    let caps = re.captures(re_input).unwrap();
    println!(
        "Year: {}, month: {}, day: {}",
        &caps["year"], &caps["month"], &caps["day"]
    );

    let http_ok = b"HTTP/1.1 200 OK\r\n";
    println!("{http_ok:?}");
    // The PNG signature
    let png_signature = br#"\x89PNG\r\n\x1a\n"#;
    println!("{png_signature:?}");
}
