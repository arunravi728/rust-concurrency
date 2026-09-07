use std::thread;

fn main() {
    println!("Entering Main Thread");

    thread::spawn(callback);
    thread::spawn(callback);
    thread::spawn(callback);

    println!("Exiting Main Thread");
}

fn callback() {
    println!("Thread ID: {:#?}", thread::current().id());
}
