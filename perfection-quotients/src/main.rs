use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();

    // let n = 1e18 as u64;
    let n = 1e7 as u64;
    // let n = 6 as u64;
    // let sum: u64 = (1..=n).filter(|i| perfect_quotient(*i, &mut cache) % 1.0 == 0.5).sum();
    let divs = divisors(50, &mut cache);
    println!("{:?}", cache);
    println!("{}", cache.len());
    println!("{}, {:?}", 50, divs);
    let divs = divisors(100, &mut cache);
    println!("{:?}", cache);
    println!("{}", cache.len());
    let divs = divisors(1e10 as u64, &mut cache);
    println!("{}", cache.len());

    // for i in 1..=20 {
    //     println!("{}: {}", i, perfect_quotient(i, &mut cache));
    // }

    // println!("{:?}", perfect_quotient(n, &mut cache));
    // println!("{:?}", divisors);
    // println!("{}", sum);
}

fn perfect_quotient(n: u64, cache: &mut HashMap<u64, Vec<u64>>) -> f64 {
    sigma(n, cache) as f64 / n as f64
}

fn sigma(n: u64, cache: &mut HashMap<u64, Vec<u64>>) -> u64 {
    divisors(n, cache).iter().sum()
}


fn divisors(n: u64, cache: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if let Some(cached_result) = cache.get(&n) {
        return cached_result.clone();
    }

    let mut divs = vec![1, n];
    let sqrt_n = (n as f64).sqrt().ceil() as u64;

    for i in 2..=sqrt_n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
                divs.append(&mut divisors(n / i, cache));
                break;
            }
        }
    }

    divs.sort();
    divs.dedup(); // Remove duplicates
    
    cache.insert(n, divs.clone());
    divs
}
