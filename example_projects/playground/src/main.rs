use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        tx.send(String::from("cool")).unwrap();
    });

    loop {
        match rx.recv() {
            Ok(message) => println!("Cool: {}", message),
            _ => break,
        }
    }
}
