fn main() {
    // let number: u64 = 600851475143;  // [71, 839, 1471, 6857];
    let number = 13195;  // [5, 7, 13, 29];
    // let factors = trial_division(number);
    let factors = fermat_factorization(number);
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
    fn test_trial_division() {
        assert_eq!(trial_division(NUM1), FACTORS1.to_vec());
        assert_eq!(trial_division(NUM2), FACTORS2.to_vec());
    }

    #[test]
    fn test_fermat_factorization() {
        assert_eq!(fermat_factorization(NUM1), FACTORS1.to_vec());
        assert_eq!(fermat_factorization(NUM2), FACTORS2.to_vec());
    }
}

// https://stackoverflow.com/a/412942
pub fn trial_division(x: u64) -> Vec<u64> {
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

pub fn fermat_factorization(x: u64) -> Vec<u64> {
    // Divide by 2 until x is odd
    let mut partial_factors = vec![];
    let mut factors = vec![];
    let mut x = x;
    while x % 2 == 0 {
        factors.push(2);
        x /= 2;
    }
    let x = x; // x is odd and immutable
    partial_factors.push(x);
    while partial_factors.len() > 0 {
        let x = partial_factors.pop().unwrap();
        let (a, b) = fermat_factor(x);
        match a {
            1 => {
                factors.push(b);
            },
            _ => {
                partial_factors.push(a);
                partial_factors.push(b);
            }
        }
    }
    factors.sort();
    factors
}

pub fn fermat_factor(x: u64) -> (u64, u64) {
    assert_eq!(x % 2, 1);
    let mut a = (x as f64).sqrt().ceil();
    let mut b2 = a * a - x as f64;
    while b2.sqrt() % 1.0 != 0.0 {
        a += 1.0;
        b2 = a * a - x as f64;
    }
    let a = a as u64;
    let b = b2.sqrt() as u64;
    (a - b, a + b)
}
