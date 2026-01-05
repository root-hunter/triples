fn main() {
    let n: usize = 20000;

    // a^2 + b^2 = c^2
    // c = sqrt(a^2 + b^2)

    let start_time = std::time::Instant::now();

    let mut mat = vec![vec![0; 3]; n * 2];

    println!("Calculating Pythagorean triples up to {}...", n);

    let mut count = 0;

    for a in 2..n {
        let x = a * a;

        for b in a..n {
            let y = b * b;
            let c_squared = (x + y) as f64;

            let z = c_squared.sqrt();

            if z.fract() != 0.0 {
                continue;
            }

            if z as usize > n {
                break;
            }

            mat[count][0] = a;
            mat[count][1] = b;
            mat[count][2] = z as usize;

            count += 1;
        }
    }

    for i in 0..count {
        let a = mat[i][0];
        let b = mat[i][1];
        let z = mat[i][2];

        println!("{}^2+{}^2={}^2", a, b, z);
    }

    println!("Found {} Pythagorean triples.", count);

    let duration = start_time.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
