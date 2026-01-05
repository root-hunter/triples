mod euclide;
mod berggren;

use std::io::{BufWriter};

const N: usize = 100000;

fn main() {
    let start_time = std::time::Instant::now();

    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let mut count = 0;
    let handle = None::<&mut BufWriter<std::io::Stdout>>;
    euclide::stream_triples(N, &mut count, handle);
    println!("(EUCLIDE) Found {} results", count);

    let mut count = 0;
    let handle = None::<&mut BufWriter<std::io::Stdout>>;
    berggren::stream_triples(N, &mut count, handle, [3, 4, 5]);
    println!("(BERGGREN) Found {} results", count);

    let duration = start_time.elapsed();

    // print_triples(&vec);

    // println!("Find {} results", vec.len());
    println!("Time elapsed: {:?}", duration);
}
