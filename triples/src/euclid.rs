use std::io::Write;

// Binary GCD algorithm - versione ottimizzata
#[inline(always)]
fn gcd(mut u: usize, mut v: usize) -> usize {
    // Caso base: uno dei due è zero
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    // Trova il numero di zeri trailing comuni
    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;

    // Rimuovi zeri trailing da u
    u >>= u.trailing_zeros();

    loop {
        // Rimuovi zeri trailing da v
        v >>= v.trailing_zeros();

        // Swap se necessario per mantenere u <= v
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

    // Pre-calcola il limite per m
    let m_max = ((limit as f64).sqrt() as usize) + 1;

    for m in 2..=m_max {
        let m2 = m * m;

        // Early exit: se m^2 + 1 > limit, nessuna tripla è possibile
        if m2 + 1 > limit {
            break;
        }

        // Ottimizzazione: start_n basato sulla parità di m
        // Se m è pari, n deve essere dispari (start_n=1)
        // Se m è dispari, n deve essere pari (start_n=2)
        let start_n = 1 + (m & 1);

        // Loop manuale invece di step_by per più controllo
        let mut n = start_n;
        while n < m {
            // Check GCD - skip se non sono coprimi
            if gcd(m, n) == 1 {
                let n2 = n * n;
                let a = m2 - n2;
                let b = 2 * m * n;
                let c = m2 + n2;

                // Calcola direttamente quanti multipli esistono
                let k_max = limit / c;
                *count += k_max;

                // Scrivi output solo se necessario
                if let Some(out) = buf.as_deref_mut() {
                    // Versione ottimizzata: calcola tutti i multipli
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

            n += 2;
        }
    }
}
