fn main() {
    let limit = 100;
    let sum_of_squares = (1..=limit).fold(0, |a, b| a + b * b);
    let squares_of_sum = (1..=limit).sum::<u64>().pow(2);

    println!("{}", squares_of_sum - sum_of_squares);
}
