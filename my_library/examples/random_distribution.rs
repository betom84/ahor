use my_library::RandomNumberGenerator;

fn main() {
    let mut rng = RandomNumberGenerator::new();
    let mut results = vec![0; 16];

    for _ in 0..1_000 {
        let roll: usize = rng.range(1..=6) + rng.range(1..=6) + rng.range(1..=6);
        results[roll - 3] += 1;
    }

    println!("Distriubutio  of 3d6 rolls:");
    for (i, count) in results.iter().enumerate() {
        print!("{: >2} : ", i + 3);
        (0..*count).for_each(|_| print! {"#"});
        println!();
    }
}
