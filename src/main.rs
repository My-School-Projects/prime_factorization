
fn main() {

}

fn find_prime_factors(n: u64) -> Vec<u64>
{
    let v = find_factors(n);

    let done = false;

    while !done {
        for i in v {
            if !is_prime(i) {
                let v2 = find_factors(i);
                // replace i with the elements of v2
            }
        }
    }
}

fn find_factors(n: u64) -> Vec<u64>
{
    let mut v = vec![];

    let mut f = 1;

    loop {
        match find_smallest_factor(n, f+1) {
            Some(x) => {
                v.push(x);
                if n / x != x {
                    v.push(n / x);
                }
                f = x;
            },
            None => return v
        }
    }
}

/// Finds the smallest factor of n that is greater than i.
fn find_smallest_factor(n: u64, i: u64) -> Option<u64>
{
    let n = n as f64;
    let root_n = n.sqrt() as u64;

    for k in i..root_n+1 {
        if is_integer(n / k as f64) {
            return Some(k);
        }
    }
    None
}

fn is_prime(n: u64) -> bool
{
    match find_smallest_factor(n, 2) {
        Some(_) => false,
        None => true
    }
}

fn is_integer(n: f64) -> bool
{
    n == n.floor()
}
