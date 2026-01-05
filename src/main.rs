#![feature(portable_simd)]
use std::simd::{Mask, Simd, StdFloat, cmp::{SimdPartialEq, SimdPartialOrd}};

// const N: usize = 8192;
const N: usize = 8192 * 2;
const LANE: usize = 8;

fn main() {
    let mut mat = Vec::with_capacity(N * 2);
    let start_time = std::time::Instant::now();

    let n_simd = Simd::splat(N as f64);

    for a in 2..N {
        let x = (a * a) as f64;
        let x_simd = Simd::splat(x);

        let mut b = a;
        while b <= N {
            // batch di 8 b
            let mut b_values = [0f64; LANE];
            let mut max_b = LANE;

            for i in 0..LANE {
                if b + i < N {
                    b_values[i] = (b + i) as f64;
                } else {
                    b_values[i] = 0.0;
                    max_b = i;
                    break;
                }
            }

            let b_simd = Simd::<f64, 8>::from_array(b_values);
            let c2_simd = b_simd * b_simd + x_simd;
            let c_simd = c2_simd.sqrt();

            // Maschera per verificare quali c sono interi e <= n
            let mask = (c_simd.fract().simd_eq(Simd::splat(0.0))) 
                & (c_simd.simd_lt(n_simd)) & b_simd.simd_lt(c_simd);

            let c_array = c_simd.to_array();
            let b_array = b_simd.to_array();

            for i in 0..LANE {
                if mask.test(i) && i < max_b {
                    mat.push([a, b_array[i] as usize, c_array[i] as usize]);
                }
            }

            b += LANE;
        }
    }

    for t in &mat {
        println!("{}^2+{}^2={}^2", t[0], t[1], t[2]);
    }

    let duration = start_time.elapsed();
    println!("Found {} triples.", mat.len());
    println!("Time elapsed: {:?}", duration);
}