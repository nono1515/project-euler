fn gen_pythagorean_triplet(m: u64, n: u64) -> (u64, u64, u64) {
    let a = m.pow(2) - n.pow(2);
    let b = 2 * m * n;
    let c = m.pow(2) + n.pow(2);
    (a, b, c)
}

fn main() {
    for n in 1..1000 {
        for m in (n + 1)..1000 {
            let (a, b, c) = gen_pythagorean_triplet(m, n);
            if b >= 1000 || c >= 1000 {
                break;
            }
            if a + b + c == 1000 {
                println!("{} {} {}", a, b, c);
                println!("{}", a * b * c);
                return;
            }
        }
    }
}
