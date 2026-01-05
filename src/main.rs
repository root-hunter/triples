const N: usize = 100_000;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn main() {
    let start_time = std::time::Instant::now();

    let n_log = (N as f64).log2();
    let vec_size = (N as f64 * n_log) as usize;

    let mut vec = Vec::with_capacity(vec_size);

    let mut count = 0;

    for m in 2.. {
        let m2 = m*m;
        if m2 + 1 >= N { break; }

        for n in 1..m {
            if (m - n) % 2 == 1 && gcd(m,n) == 1 {
                let n2 = n*n;
                let a = m2 - n2;
                let b = 2*m*n;
                let c = m2 + n2;

                let mut ka = a;
                let mut kb = b;
                let mut kc = c;
   
                while kc <= N {
                    vec.push((ka, kb, kc));

                    count += 1;
                    ka += a;
                    kb += b;
                    kc += c;
                }
            }
        }
    }

    for (a, b, c) in vec {
        println!("{}^2 + {}^2 = {}^2", a, b, c);
    }

    println!("Find {} results", count);
    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
}