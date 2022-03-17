use std::thread;

fn main() {
    let s = String::from("Hello world!");
    let handler = thread::spawn(move || {
        println!("moved: {:?}", s);
    });
    handler.join().unwrap();
}
