// Returns prime numbers until n
pub fn primes(n: usize) -> Vec<usize> {
    let mut arr = vec![true; n + 1];
    let mut primes = vec![];
    arr[0] = false; // 0 is NOT a prime number
    arr[1] = false; // 1 is NOT a prime number
    for p in 2..=n {
        if !arr[p] {
            continue;
        }

        primes.push(p);

        let mut mul_p = 2 * p;
        while mul_p <= n {
            arr[mul_p] = false;
            mul_p += p;
        }
    }

    primes
}

// Returns prime factors of n
pub fn prime_factors(n: usize) -> std::collections::HashMap<usize, usize> {
    let mut n = n;
    let mut pf = std::collections::HashMap::new();
    let ps = primes((n as f64).sqrt() as usize + 2);

    for p in ps {
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }

        if count > 0 {
            pf.insert(p, count);
        }
    }

    // for case n is a prime number
    if n > 1 {
        pf.insert(n, 1);
    }

    pf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        let ps = primes(100);
        assert_eq!(
            ps,
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn test_prime_factors() {
        let pattern = vec![
            (12, vec![(2, 2), (3, 1)]),
            (1024, vec![(2, 10)]),
            (1, vec![]),
            (17, vec![(17, 1)]),
        ];

        for (n, factors) in pattern {
            let pf = prime_factors(n);
            let mut ans = std::collections::HashMap::new();
            for (p, k) in factors {
                ans.insert(p, k);
            }
            assert_eq!(pf, ans);
        }
    }
}
