fn main() {
    let my_string: String =
        String::from("This is a long string that I don't intend to modify further.");

    // Convert the String into a Box<str>
    let my_boxed_str: Box<str> = my_string.into_boxed_str();

    // Now you can use `my_boxed_str` just like a &str, but it's owned.
    println!("My boxed str: {my_boxed_str}",);
}
