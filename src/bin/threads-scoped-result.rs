use std::thread;

fn main() {
    let result: Vec<i32> = thread::scope(|scope| {
        let handles: Vec<_> = (1..=5).map(|i| scope.spawn(move || i * i)).collect();

        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    });

    println!("Results: {:#?}", result);
}
