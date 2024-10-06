fn main() {
    let mut sum: u128 = 0;
    let lim = 4000000;
    let (mut a, mut b) = (0, 1);

    while b < lim {
        a = a + b; 
        if a % 2 == 0 {
            sum += a
        }
        (a, b) = (b, a)
    }

    println!("{sum}")
}
