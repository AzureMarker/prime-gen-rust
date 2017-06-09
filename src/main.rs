use std::env;

fn main() {
    let target_primes = env::args().nth(1).unwrap_or("1000000".to_string()).parse().expect("Invalid number");
    let mut primes = Vec::with_capacity(target_primes);
    primes.push(2u32);

    for x in (3..).filter(|num| num % 2 == 1) {
        let mut found = false;

        let sqrt = (x as f64).sqrt() as u32;
        for factor in primes.iter() {
            if *factor > sqrt {
                break;
            }

            if x % factor == 0 {
                found = true;
                break;
            }
        }

        if !found {
            primes.push(x);

            if primes.len() == target_primes {
                break;
            }
        }
        else {
            continue;
        }
    }

    println!("{} is the #{} prime", primes.last().unwrap(), primes.len());
}
