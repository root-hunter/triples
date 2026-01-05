const N: usize = 100_000;

#[inline(always)]
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn print_triples(vec: &Vec<[usize; 3]>) {
    for triple in vec {
        println!("{}^2 + {}^2 = {}^2", triple[0], triple[1], triple[2]);
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let n_log = (N as f64).log2();
    let vec_size = (N as f64 * n_log) as usize;

    let mut vec = Vec::with_capacity(vec_size);
    let m_max = ((N - 1) as f64).sqrt() as usize;

    for m in 2..=m_max {
        let m2 = m * m;
        if m2 + 1 >= N {
            break;
        }

        // Start n at 1 if m is even, else start at 2
        let start_n = if m & 1 == 0 { 1 } else { 2 };

        for n in (start_n..m).step_by(2) {
            // Ensure m and n have opposite parity
            if (m & 1) == (n & 1) {
                continue;
            }

            if gcd(m, n) != 1 {
                continue;
            }

            // Generate primitive triple
            let n2 = n * n;
            let a = m2 - n2;
            let b = 2 * m * n;
            let c = m2 + n2;

            // Generate multiples of the primitive triple
            // and store them if c <= N

            let mut ka = a;
            let mut kb = b;
            let mut kc = c;

            while kc <= N {
                vec.push([ka, kb, kc]);

                ka += a;
                kb += b;
                kc += c;
            }
        }
    }
    let duration = start_time.elapsed();

    // print_triples(&vec);

    println!("Find {} results", vec.len());
    println!("Time elapsed: {:?}", duration);
}
