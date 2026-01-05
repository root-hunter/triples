const N: usize = 100000;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let mut count = 0;

    for m in 2.. {
        let m2 = m * m;
        for n in 1..m {
            if (m - n) % 2 == 1 && gcd(m, n) == 1 {
                let n2 = n * n;
                let a = m2 - n2;
                let b = 2 * m * n;
                let c = m2 + n2;

                if c >= N {
                    break;
                }

                let mut k = 1;
                while k * c <= N {
                    println!("{}^2 + {}^2 = {}^2", k * a, k * b, k * c);
                    count += 1;
                    k += 1;
                }
            }
        }
        if m2 + 1 >= N {
            break;
        }
    }

    println!("Trovati {} risultati", count);
    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
}
