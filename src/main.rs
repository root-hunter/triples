
fn main() {
    let n: usize = 20000;

    // a^2 + b^2 = c^2
    // c = sqrt(a^2 + b^2)

    let start_time = std::time::Instant::now();

    for a in 1..n {
        let x = a * a;
        
        for b in 1..n {
            let c_squared = x + b * b;
            let c = (c_squared as f64).sqrt();
            if c.fract() == 0.0 && c as usize <= n {
                println!("{}^2+{}^2={}^2", a, b, c as usize);
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}