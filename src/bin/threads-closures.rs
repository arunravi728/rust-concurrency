use std::thread;

fn main() {
    let numbers = Vec::from_iter(1..=10);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        (sum as f64 / len as f64) as f64
    });

    println!("Average: {:#?}", t.join().unwrap() as f64);
}
