use rayon::prelude::*;
use std::time::Instant;

fn main() {
    /*
    let now = Instant::now();
    let range: std::ops::Range<u32> = 0..10_000_000;
    range.into_par_iter().for_each(|x| {
        let parsed: Vec<char> = x.to_string().chars().collect();
        let mut sum = 0;

        parsed.into_par_iter().for_each(|c| {
            let num = c.to_digit(10).unwrap();
            let mut result = 1;
            for _ in 0..num {
                result *= num;
            }

            sum += result;
        });

        if sum == x {
            println!("{} is a winner!", x);
        }
    });

    println!("Elapsed: {:?}", now.elapsed());
    */

    let parsed: Vec<char> = 3435.to_string().chars().collect();

    /*
    let mut sum = 0;
    for c in parsed {
        let num = c.to_digit(10).unwrap();
        let mut result = 1;
        for _ in 0..num {
            result *= num;
        }

        sum += result;
    }
    */

    let sum = parsed.into_par_iter().fold(
        || 0,
        |acc, c| {
            acc + {
                let num = c.to_digit(10).unwrap();
                let mut result = 1;
                for _ in 0..num {
                    result *= num;
                }

                result
            }
        },
    );

    println!("SUM: {:?}", sum);
}
