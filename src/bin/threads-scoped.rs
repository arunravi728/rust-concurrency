use std::thread;

fn main() {
    println!("Entering Main Thread");

    let numbers = Vec::from_iter(1..=10);

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {:#?}", numbers.len());
        });
        scope.spawn(|| {
            println!("Sum: {:#?}", numbers.iter().sum::<usize>());
        });
        scope.spawn(|| {
            println!("Max: {:#?}", numbers.iter().max());
        });
        scope.spawn(|| {
            println!("Min: {:#?}", numbers.iter().min());
        });
    });

    println!("Exiting Main Thread");
}
