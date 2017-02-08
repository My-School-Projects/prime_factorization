
fn main() {
    let n = 13195;
    println!("The factors of {} are {:?}", n, prime_factors(n));
    let n = 600851475143;
    println!("The factors of {} are {:?}", n, prime_factors(n));
}

fn prime_factors(n: u64) -> Vec<u64>
{
    let n = n as f64;
    let root_n = n.sqrt() as u64;

    let mut v = vec![];

    for j in 2..root_n+1 {
        if is_integer(n / j as f64) {
            // j is a factor of n.
            // Find the prime factors of j (recursively), and append them to v.
            // If there are none (j is prime), append j.
            let factors = prime_factors(j);
            if factors.len() == 0 {
                if !v.contains(&j) {
                    v.push(j);
                }
            } else {
                for i in factors {
                    if !v.contains(&i) {
                        v.push(i);
                    }
                }
            }
            // For every j < sqrt(n), there is another factor of n, k = n / j
            // We do the same thing for k.
            let k = n as u64/ j;
            if k != j {
                let factors = prime_factors(k);
                if factors.len() == 0 {
                    if !v.contains(&k) {
                        v.push(k);
                    }
                } else {
                    for i in factors {
                        if !v.contains(&i) {
                            v.push(i);
                        }
                    }
                }
            }
        }
    }
    v
}

fn is_integer(n: f64) -> bool
{
    n == n.floor()
}
