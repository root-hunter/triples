use plotters::prelude::*;
use std::{io::BufWriter, time::Instant};
use triples::{berggren, euclid};

const LIMITS: [usize; 26] = [
    100,
    500,
    1_000,
    2_000,
    10_000,
    50_000,
    100_000,
    250_000,
    500_000,
    750_000,
    1_000_000,
    2_500_000,
    5_000_000,
    7_500_000,
    10_000_000,
    25_000_000,
    50_000_000,
    100_000_000,
    250_000_000,
    500_000_000,
    750_000_000,
    1_000_000_000,
    1_250_000_000,
    1_500_000_000,
    1_750_000_000,
    2_000_000_000,
];

fn measure_euclide(limit: usize) -> f64 {
    let mut count = 0;
    let start_time = Instant::now();
    euclid::generate(
        limit,
        &mut count,
        None::<&mut BufWriter<std::io::StdoutLock>>,
    );
    let duration = start_time.elapsed();
    println!("Euclide found {} triples up to {}", count, limit);
    duration.as_secs_f64()
}

fn measure_berggren(limit: usize) -> f64 {
    let mut count = 0;
    let start_time = Instant::now();
    berggren::generate(
        limit,
        &mut count,
        None::<&mut BufWriter<std::io::StdoutLock>>,
        berggren::INITIAL_TRIPLE,
    );
    let duration = start_time.elapsed();
    println!("Berggren found {} triples up to {}", count, limit);
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

    // Disegna il grafico
    let root = BitMapBackend::new("benchmark.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = times_euclide
        .iter()
        .chain(times_berggren.iter())
        .cloned()
        .fold(0. / 0., f64::max); // massimo tra tutti i tempi

    let mut chart = ChartBuilder::on(&root)
        .caption("Benchmark Euclide vs Berggren", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(LIMITS[0]..LIMITS[LIMITS.len() - 1], 0f64..max_time * 1.1)?;

    chart
        .configure_mesh()
        .x_desc("Limit")
        .y_desc("Time (seconds)")
        .draw()?;

    // Euclide line
    chart
        .draw_series(LineSeries::new(
            LIMITS
                .iter()
                .zip(times_euclide.iter())
                .map(|(&x, &y)| (x, y)),
            &RED,
        ))?
        .label("Euclide")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Berggren line
    chart
        .draw_series(LineSeries::new(
            LIMITS
                .iter()
                .zip(times_berggren.iter())
                .map(|(&x, &y)| (x, y)),
            &BLUE,
        ))?
        .label("Berggren")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("Chart saved to benchmark.png");

    Ok(())
}
