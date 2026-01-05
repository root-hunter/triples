mod euclide;
mod berggren;

use std::io::{BufWriter, Write};

const N: usize = 5_000_000;

fn test_euclide(limit: usize, handle: Option<&mut impl Write>) {
    let mut count = 0;
    let start_time = std::time::Instant::now();
    euclide::stream_triples(limit, &mut count, handle);
    let duration = start_time.elapsed();

    println!("(EUCLIDE) Found {} results [limit={}]", count, limit);
    println!("(EUCLIDE) Time elapsed: {:?} [limit={}]", duration, limit);
}

fn test_berggren(limit: usize, handle: Option<&mut impl Write>) {
    let mut count = 0;
    let start_time = std::time::Instant::now();
    berggren::stream_triples(limit, &mut count, handle, [3, 4, 5]);
    let duration = start_time.elapsed();

    println!("(BERGGREN) Found {} results [limit={}]", count, limit);
    println!("(BERGGREN) Time elapsed: {:?} [limit={}]", duration, limit);
}

fn main() {
    let limits = [
        100, 
        500,
        1_000,
        2_000,
        10_000,
        50_000,
        100_000,
        1_000_000,
        5_000_000
    ];

    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    for &limit in &limits {
        let handle = None::<&mut BufWriter<std::io::StdoutLock>>;
        test_euclide(limit, handle);
    
        let handle = None::<&mut BufWriter<std::io::StdoutLock>>;
        test_berggren(limit, handle);
    }
}
