fn main() {
    let number: u64 = 600851475143;
    // let number = 13195;
    let factors = prime_factors(number);
    println!("{factors:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM1: u64 = 13195;
    const FACTORS1: [u64; 4] = [5, 7, 13, 29];
    const NUM2: u64 = 600851475143;
    const FACTORS2: [u64; 4] = [71, 839, 1471, 6857];

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(NUM1), FACTORS1.to_vec());
        assert_eq!(prime_factors(NUM2), FACTORS2.to_vec());
    }
}

// https://stackoverflow.com/a/412942
pub fn prime_factors(x: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut d = 2;
    let mut n = x;
    while n > 1 {
        // Just check if d divide n
        while n % d == 0 {
            // If so, add it to factors and update n
            factors.push(d);
            n /= d;
        }
        // Increase d up to sqrt(n), then return
        d += 1;
        if d ^ 2 > n {
            if n > 1 {
                factors.push(n);
            }
            break;
        }
    }
    return factors;
}
