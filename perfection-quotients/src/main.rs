use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    let limit = BigUint::parse_bytes(b"1000000000000000000", 10).unwrap();
    let mut sum = BigUint::zero();

    for a in 1..60 { // 2^60 > 10^18, so this is sufficient
        let power_of_two = BigUint::from(2u32).pow(a);
        let sigma_power_of_two = &power_of_two * 2u32 - 1u32;

        let mut q = BigUint::one();
        loop {
            let n = &power_of_two * &q * &q;
            if n > limit {
                break;
            }

            let sigma_q = sigma_odd_square(&q);
            if &sigma_power_of_two * &sigma_q == &n + &q * &q {
                sum += n;
            }

            q += 1u32;
        }
    }

    println!("Sum: {}", sum);
}

fn sigma_odd_square(q: &BigUint) -> BigUint {
    let q_squared = q * q;
    let mut result = &q_squared + q + 1u32;
    let mut d = BigUint::from(3u32);
    while &d * &d <= *q {
        if q % &d == BigUint::zero() {
            result += &d + q / &d;
        }
        d += 2u32;
    }
    result
}