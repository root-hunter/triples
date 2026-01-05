use std::io::Write;

// Binary GCD algorithm
// Get it from https://github.com/frewsxcv/rust-gcd/blob/master/src/lib.rs
#[inline(always)]
fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        #[allow(clippy::manual_swap)]
        if u > v {
            // mem::swap(&mut u, &mut v);
            let temp = u;
            u = v;
            v = temp;
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}

pub fn triples(limit: usize, count: &mut usize, mut buf: Option<&mut impl Write>) {
    *count = 0;
    let m_max = ((limit as f64).sqrt() as usize) + 1;

    for m in 2..=m_max {
        let m2 = m * m;

        if m2 + 1 > limit {
            break;
        }

        // Start n at 1 if m is even, else start at 2
        let start_n = if m & 1 == 0 { 1 } else { 2 };

        let mut n = start_n;

        loop {
            if n >= m {
                break;
            }

            if gcd(m, n) == 1 {
                let n2 = n * n;
                let a = m2 - n2;
                let b = 2 * m * n;
                let c = m2 + n2;

                let k_max = limit / c;
                *count += k_max;

                if let Some(out) = buf.as_deref_mut() {
                    for k in 1..=k_max {
                        writeln!(out, "{}^2 + {}^2 = {}^2", a * k, b * k, c * k).unwrap();
                    }
                }
            }
            n += 2;
        }
    }
}
