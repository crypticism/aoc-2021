use std::time::Instant;

use aoc_2021::days::day1;
fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let now = Instant::now();

    println!("day 1: {:?}", day1::run()?);

    let end_time = now.elapsed();

    if end_time.as_millis() == 0 {
        println!("\nRunning all days took {}us", end_time.as_micros());
    } else {
        println!(
            "\nRunning all days took {}s {}ms",
            end_time.as_secs(),
            end_time.subsec_millis()
        );
    }
    Ok(())
}
