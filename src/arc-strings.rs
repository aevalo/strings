use std::sync::Arc;
use std::thread;

fn main() {
    // Create a String, which doesn't have 'static lifetime
    let text_string = String::from("This is some text that multiple threads will read.");
    let text_slice = &text_string[..]; // This also doesn't have 'static lifetime

    // Convert it to an Arc<str>
    let shared_text: Arc<str> = Arc::from(text_slice);

    let mut handles = vec![];

    for _ in 0..3 {
        let text_ref = Arc::clone(&shared_text);
        let handle = thread::spawn(move || {
            println!("Thread is reading: {text_ref}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
