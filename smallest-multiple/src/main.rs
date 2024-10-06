fn main() {
    let x = (1..20).fold(1, |a, b| a * b / gcd(a, b));
    println!("{}", x);
    
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
