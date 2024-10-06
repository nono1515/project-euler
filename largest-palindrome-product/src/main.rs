fn main() {

    let low_lim = 100;
    let high_lim = 1000;

    println!("{}", find_palindrome(low_lim, high_lim));
    println!("{}", find_palindrome_fast(low_lim, high_lim));
    println!("{}", find_palindrome_smart(low_lim, high_lim));  // the last method only works for specific limits
}

fn find_palindrome(low_lim: u64, high_lim: u64) -> u64 {
    let mut largest_palindrome = 0;
    for i in low_lim..high_lim {
        for j in low_lim..high_lim {
            let n = i * j;
            if is_palindrome(n) && n > largest_palindrome {
                largest_palindrome = n;
            }
        }
    }
    largest_palindrome
}

fn find_palindrome_fast(low_lim: u64, high_lim: u64) -> u64 {
    let mut largest_palindrome = 0;
    // search downward so we are more likely to find the largest palindrome quickly
    for i in (low_lim..high_lim).rev() {
        for j in (low_lim..i).rev() {
            let n = i * j;
            if n <= largest_palindrome {
                break;
            } // break the loop if i and j are too small
            if is_palindrome(n) && n > largest_palindrome {
                largest_palindrome = n;
            }
        }
    }
    largest_palindrome
}

fn find_palindrome_smart(low_lim: u64, high_lim: u64) -> u64 {
    let mut largest_palindrome = 0;
    for i in (low_lim..high_lim).rev() {
        let (db, start) = match i % 11 {
            0 => (1, 999),
            _ => (11, 990),
        };
        for j in (i..=start).rev().step_by(db) {
            let n = i * j;
            if n <= largest_palindrome {
                break;
            } // break the loop if i and j are too small
            if is_palindrome(n) && n > largest_palindrome {
                largest_palindrome = n;
            }
        }
    }
    largest_palindrome
}

fn is_palindrome(n: u64) -> bool {
    n.to_string() == n.to_string().chars().rev().collect::<String>()
}
