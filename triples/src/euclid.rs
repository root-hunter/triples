use std::io::Write;

// Binary GCD algorithm
#[inline(always)]
pub fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

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

                    for _ in 1..k_max {
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

pub struct EuclidIter {
    limit: usize,
    m: usize,
    n: usize,
    k: usize,

    m_max: usize,

    a: usize,
    b: usize,
    c: usize,
    k_max: usize,
}

impl EuclidIter {
    pub fn new(limit: usize) -> Self {
        let m_max = ((limit as f64).sqrt() as usize) + 1;
        EuclidIter {
            limit,
            m: 2,
            n: 1,
            k: 1,
            m_max,
            a: 0,
            b: 0,
            c: 0,
            k_max: 0,
        }
    }
}

impl Iterator for EuclidIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.k <= self.k_max {
                let res = (self.a * self.k, self.b * self.k, self.c * self.k);
                self.k += 1;
                return Some(res);
            }

            self.n += 2;
            while self.n >= self.m {
                self.m += 1;
                if self.m > self.m_max {
                    return None;
                }
                self.n = 1 + (self.m & 1);
            }

            let m = self.m;
            let n = self.n;

            if gcd(m, n) != 1 {
                continue;
            }

            let m2 = m * m;
            let n2 = n * n;

            let c = m2 + n2;
            if c > self.limit {
                continue;
            }

            self.a = m2 - n2;
            self.b = 2 * m * n;
            self.c = c;
            self.k_max = self.limit / c;
            self.k = 1;
        }
    }
}
