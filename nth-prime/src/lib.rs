pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let primes: Vec<u32> = n_primes(n);
    primes[n as usize]

}

fn n_primes(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![];
    let mut i: u32 = 2;
    let mut count: u32 = 0;

    while count <= n {
        if is_prime(i) {
            primes.push(i);
            count += 1;
        }
        i += 1;
    }

    primes
}

fn is_prime(number: u32) -> bool {
    let mut i: u32 = 2;
    let mut is_prime: bool = true;

    while i < number {
        if number % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }

    is_prime
}

