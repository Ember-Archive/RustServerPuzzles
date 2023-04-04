use rand::Rng;

fn mod_exp(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn miller_rabin(n: i64, k: usize) -> bool {
    if n <= 3 {
        return n > 1;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut rng = rand::thread_rng();

    let mut d = n - 1;
    let mut s = 0;

    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    'witness: for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = mod_exp(a, d, n);

        if x == 1 || x == n - 1 {
            continue 'witness;
        }

        for _ in 0..s - 1 {
            x = mod_exp(x, 2, n);

            if x == n - 1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}

// Find the primes that satisfy the given condition
pub fn find_primes(number: i64, k: usize) -> (i64, i64, i64) {
    // Iterate through even gaps starting from 4
    for gap in (4..).step_by(2) {
        let prime1 = number - gap;
        let prime2 = gap / 2;
        
        // Check if both prime1 and prime2 are prime using the Miller-Rabin test
        if miller_rabin(prime1, k) && miller_rabin(prime2, k) {
            // If both are prime, return the primes as a tuple
            return (prime1, prime2, prime2);
        }
    }
    // This line should never be reached since there should always be a solution
    unreachable!("Compiler, sweetie, baby, how you doin'?");
}