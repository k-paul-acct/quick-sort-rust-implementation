use rand::Rng;
use std::time::Instant;

mod sort;

fn main() {
    let elements = 1_000_000;
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (0..elements).map(|_| rng.gen()).collect();
    let mut numbers_copy = numbers.clone();

    let mut start = Instant::now();
    numbers.sort_unstable();
    let mut end = Instant::now();

    println!("elements in vector: {}", elements);
    println!(
        "sorting time (default sort): {} ms",
        (end - start).as_millis()
    );

    start = Instant::now();
    sort::quick_sort(&mut numbers_copy);
    end = Instant::now();

    println!(
        "sorting time (custom sort): {} ms",
        (end - start).as_millis()
    );

    std::io::stdin().read_line(&mut String::new()).unwrap();
}
