fn main() {
    let limit = 999;
    println!("{}", sum_of_multiples(3, limit) + sum_of_multiples(5, limit) - sum_of_multiples(15, limit));
}

fn sum_of_multiples(num: u64, limit: u64) -> u64 {
    return num * sum_of_int(limit / num)
}

fn sum_of_int(num: u64) -> u64 {
    return num * (num + 1) / 2
}
