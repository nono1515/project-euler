fn main() {
    let mut primes = vec![];
    let prime_index = 10001;

    let mut i = 2;
    while primes.len() < prime_index {
        if primes.iter().all(|p| i % p != 0) {
            primes.push(i);
        }

        i += 1;
    }

    println!("{}", primes[prime_index - 1]);
}
