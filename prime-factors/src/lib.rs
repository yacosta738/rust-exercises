pub fn factors(n: u64) -> Vec<u64> {
    // This should calculate the prime factors of n
    println!("This should calculate the prime factors of {}", n);
    let mut factors: Vec<u64> = Vec::new();
    let mut i: u64 = 2;
    let mut n_clone: u64 = n;
    while i <= n_clone {
        if n_clone % i == 0 {
            factors.push(i);
            n_clone = n_clone / i;
        } else {
            i = i + 1;
        }
    }
    factors
}
