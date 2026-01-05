const N: usize = 10000;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut mat = Vec::with_capacity(N * 2);
    let start_time = std::time::Instant::now();

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
                    mat.push([k * a, k * b, k * c]);
                    k += 1;
                }
            }
        }
        if m2 + 1 >= N {
            break;
        }
    }


    for t in &mat {
    }

    let duration = start_time.elapsed();
    println!("Found {} triples.", mat.len());
    println!("Time elapsed: {:?}", duration);
}