use prime_factorization::Factorization;

fn main() {
    // navie approach, much too slow
    // let n = 50;
    // println!("{:?}", get_euler_divisors(n))

    // From https://en.wikipedia.org/wiki/Divisor_function#Other_properties_and_identities
    // Similarly, the number σ1(n) is odd if and only if n is a square or twice a square.[9]
    // [9]: Gioia, A. A.; Vaidya, A. M. (1967), "Amicable numbers with opposite parity"
    let mut sum: u128 = 0;
    let twice_a_square_lim = (10_u64.pow(18) as f64 / 2.0).sqrt().ceil() as u128;
    for i in 1..10_u128.pow(9) {
        if i < twice_a_square_lim {
            let divs = Factorization::run(2 * i.pow(2));
            // divs.prime_factor_repr();
            // println!("{:?}", divs.prime_factor_repr());
            let sigma = sigma_from_primes(divs.prime_factor_repr());
            if (2 * sigma / i) % 2 == 1 {
                sum += i as u128
            }
        }
        if i % 2 == 0 {
            let divs = Factorization::run(i.pow(2));
            // divs.prime_factor_repr();
            let sigma = sigma_from_primes(divs.prime_factor_repr());
            if (2 * sigma / i) % 2 == 1 {
                sum += i as u128
            }
        }
    }

    println!("{}", sum)
}

fn sigma_from_primes(divs: Vec<(u128, u32)>) -> u128 {
    divs.iter().fold(1, |curr, (p, k)| {
        // println!("{curr}, {p}^{k}");
        curr * (p.pow(k + 1) - 1) / (p - 1)
    })
}

fn get_euler_divisors(lim: usize) -> Vec<i32> {
    let mut divisor_function = vec![];

    for n in 1..lim {
        let mut i: i32 = 1;
        let mut sigma = 0;
        loop {
            let i1 = n as i32 - (3 * i.pow(2) - i) / 2;
            if i1 < 0 {
                break;
            }
            let i2 = n as i32 - (3 * i.pow(2) + i) / 2;
            let sigma1 = match i1 {
                0 => n as i32,
                1.. => divisor_function[i1 as usize - 1],
                _ => 0,
            };
            let sigma2 = match i2 {
                0 => n as i32,
                1.. => divisor_function[i2 as usize - 1],
                _ => 0,
            };
            match i % 2 {
                1 => sigma += sigma1 + sigma2,
                0 => sigma -= sigma1 + sigma2,
                _ => panic!(),
            };
            i += 1;
        }
        divisor_function.push(sigma);
    }

    divisor_function
}
