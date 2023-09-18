use std::rc::Rc;

fn main() {
    let some_large_text: &'static str = "This is some large text that we want to work with.";

    // Extract the subsection that multiple parts of the program will need to reference.
    let subsection: Rc<str> = Rc::from(&some_large_text[5..24]);

    // Simulate multiple owners by cloning the Rc.
    let another_reference = Rc::clone(&subsection);
    let yet_another_reference = Rc::clone(&subsection);

    println!("First reference: {subsection}");
    println!("Second reference: {another_reference}");
    println!("Third reference: {yet_another_reference}");
}
