use std::{io::BufWriter, time::Instant};
use triples::{berggren, euclid};

use std::io::Write;

const LIMITS: [usize; 1] = [128];

fn measure_euclide(limit: usize) -> f64 {
    let mut count = 0;
    let handle = std::io::stdout();
    let mut writer = BufWriter::new(handle.lock());

    let start_time = Instant::now();
    euclid::triples(limit, &mut count, Some(&mut writer));
    let duration = start_time.elapsed();
    writeln!(writer, "Euclide found {} triples up to {}", count, limit).unwrap();
    duration.as_secs_f64()
}

fn measure_berggren(limit: usize) -> f64 {
    let mut count = 0;
    let handle = std::io::stdout();
    let mut writer = BufWriter::new(handle.lock());

    let start_time = Instant::now();
    berggren::triples(
        limit,
        &mut count,
        Some(&mut writer),
        berggren::INITIAL_TRIPLE,
    );
    let duration = start_time.elapsed();
    writeln!(writer, "Berggren found {} triples up to {}", count, limit).unwrap();
    duration.as_secs_f64()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Calcola i tempi
    let mut times_euclide = Vec::new();
    let mut times_berggren = Vec::new();

    for &limit in &LIMITS {
        let t_e = measure_euclide(limit);
        println!("Euclide limit {} -> {:.4} s", limit, t_e);
        times_euclide.push(t_e);

        let t_b = measure_berggren(limit);
        println!("Berggren limit {} -> {:.4} s", limit, t_b);
        times_berggren.push(t_b);

        println!("-----------------------------------");
    }
    Ok(())
}
