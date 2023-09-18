use std::ffi::OsString;

fn main() -> std::io::Result<()> {
    let paths = std::fs::read_dir(".")?;

    for path in paths {
        match path {
            Ok(entry) => {
                let os_string: OsString = entry.file_name();
                match os_string.into_string() {
                    Ok(string) => println!("Found a file: {}", string),
                    Err(os_string) => println!("Found a non-UTF-8 filename: {:?}", os_string)
                }
            },
            Err(_) => println!("Couldn't read the path.")
        }
    }

    Ok(())
}