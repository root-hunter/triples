use std::io::Write;

// Binary GCD algorithm
#[inline(always)]
pub fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 { return v; }
    if v == 0 { return u; }

    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 {
            return u << shift;
        }
    }
}

pub fn triples(limit: usize, count: &mut usize, mut buf: Option<&mut impl Write>) {
    *count = 0;

    let m_max = ((limit as f32).sqrt() as usize) + 1;

    for m in 2..m_max {
        let m2 = m * m;

        if m2 + 1 > limit {
            break;
        }

        let start_n = 1 + (m & 1);

        for n in (start_n..=m).step_by(2) {
            if gcd(m, n) == 1 {
                let n2 = n * n;
                let a = m2 - n2;
                let b = 2 * m * n;
                let c = m2 + n2;

                let k_max = limit / c;
                *count += k_max;

                if let Some(out) = buf.as_deref_mut() {
                    let mut ka = a;
                    let mut kb = b;
                    let mut kc = c;

                    for _ in 1..=k_max {
                        writeln!(out, "(EUCLID) {}^2 + {}^2 = {}^2", ka, kb, kc).unwrap();
                        ka += a;
                        kb += b;
                        kc += c;
                    }
                }
            }
        }
    }
}
