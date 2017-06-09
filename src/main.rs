use std::env;

fn main() {
    let target_primes: usize = env::args().nth(1).unwrap_or("1000000".to_string()).parse().expect("Invalid number");
    let mut map = vec![false; 300_000_000];
    let mut primes: Vec<usize> = Vec::with_capacity(target_primes);
    primes.push(2);
    let mut x = 3usize;

    loop {
        if map[x] {
            x += 2;
            continue;
        }

        primes.push(x);

        if primes.len() == target_primes {
            break;
        }

        let mut n = x*x;
        loop {
            if n >= map.len() {
                break;
            }

            map[n] = true;
            n += x;
        }

        x += 2;
    }

    println!("The #{} prime: {}", target_primes, primes.last().unwrap());
}
