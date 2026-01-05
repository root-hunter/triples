use std::io::Write;

#[inline(always)]
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

pub fn stream_triples(limit: usize, count: &mut usize, mut buf: Option<&mut impl Write>) {
    *count = 0;
    let m_max = ((limit - 1) as f64).sqrt() as usize;

    for m in 2..=m_max {
        let m2 = m * m;
        if m2 + 1 >= limit {
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

            while kc <= limit {
                if let Some(out) = buf.as_deref_mut() {
                    writeln!(out, "{}^2 + {}^2 = {}^2", ka, kb, kc).unwrap();
                }
                *count += 1;

                ka += a;
                kb += b;
                kc += c;
            }
        }
    }
}