use std::thread;

fn main() {
    println!("Entering Main Thread");

    let t1 = thread::spawn(callback);
    let t2 = thread::spawn(callback);
    let t3 = thread::spawn(callback);

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    println!("Exiting Main Thread");
}

fn callback() {
    println!("Thread ID: {:#?}", thread::current().id());
}
