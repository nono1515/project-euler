fn modular_pow(base: &u64, exp: &u64, modulus: &u64) -> u64 {
    if *modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = *exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    return result;
}

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n <= 1 || n % 2 == 0 {
        return false;
    }

    let (mut r, mut d) = (0, n - 1);
    while d % 2 == 0 {
        r += 1;
        d /= 2;
    }

    let basis = match n {
        ..2047 => vec![2],
        ..1373653 => vec![2, 3],
        ..9080191 => vec![31, 73],
        ..25326001 => vec![2, 3, 5],
        ..3215031751 => vec![2, 3, 5, 7],
        ..4759123141 => vec![2, 7, 61],
        ..1122004669633 => vec![2, 13, 23, 1662803],
        ..2152302898747 => vec![2, 3, 5, 7, 11],
        ..3474749660383 => vec![2, 3, 5, 7, 11, 13],
        ..341550071728321 => vec![2, 3, 5, 7, 11, 13, 17],
        ..3825123056546413051 => vec![2, 3, 5, 7, 11, 13, 17, 19, 23],
        _ => vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
    };

    let try_composite = |a: u64| -> bool {
        if modular_pow(&a, &d, &n) == 1 {
            return false;
        }
        for i in 0..r {
            if modular_pow(&a, &(2u64.pow(i) * d), &n) == n - 1 {
                return false;
            }
        }
        return true;
    };

    return !basis.iter().any(|a| try_composite(*a));
}

fn main() {
    let lim = 2_000_000;
    println!("{}", (1..=lim).filter(|x| is_prime(*x)).sum::<u64>());

    // for i in 1..=lim {
    //     println!("{} {}", i, is_prime(i));
    // }
}
