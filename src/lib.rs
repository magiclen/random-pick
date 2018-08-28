//! # Random Pick
//! Pick an element from a slice randomly by given weights.
//!
//! ## Example
//!
//! ```
//! extern crate random_pick;
//!
//! enum Prize {
//!     Legendary,
//!     Rare,
//!     Enchanted,
//!     Common,
//! }
//!
//! let prize_list = [Prize::Legendary, Prize::Rare, Prize::Enchanted, Prize::Common]; // available prizes
//!
//! let slice = &prize_list;
//! let weights = [1, 5, 15, 30]; // a scale of chance of picking each kind of prize
//!
//! let n = 1000000;
//! let mut counter = [0usize; 4];
//!
//! for _ in 0..n {
//!     let picked_item = random_pick::pick_from_slice(slice, &weights).unwrap();
//!
//!     match picked_item {
//!         Prize::Legendary=>{
//!             counter[0] += 1;
//!            }
//!         Prize::Rare=>{
//!             counter[1] += 1;
//!         }
//!         Prize::Enchanted=>{
//!             counter[2] += 1;
//!         }
//!         Prize::Common=>{
//!             counter[3] += 1;
//!         }
//!     }
//! }
//!
//! println!("{}", counter[0]); // Should be close to 20000
//! println!("{}", counter[1]); // Should be close to 100000
//! println!("{}", counter[2]); // Should be close to 300000
//! println!("{}", counter[3]); // Should be close to 600000
//! ```
//!
//! The length of the slice is usually an integral multiple (larger than zero) of that of weights.

extern crate rand;

use rand::Rng;

const MAX_U64: u64 = u64::max_value();

/// Pick an element from a slice randomly by given weights.
pub fn pick_from_slice<'a, T>(slice: &'a [T], weights: &'a [usize]) -> Option<&'a T> {
    let weights_len = weights.len();

    let index = gen_usize_with_weights(weights_len, weights)?;

    Some(&slice[index])
}

/// Get a usize value by given weights.
pub fn gen_usize_with_weights(high: usize, weights: &[usize]) -> Option<usize> {
    let weights_len = weights.len();

    if weights_len == 0 {
        return None;
    }

    let mut weights_sum = 0f64;
    let mut max_weight = 0;

    for &w in weights {
        weights_sum += w as f64;
        if w > max_weight {
            max_weight = w;
        }
    }

    if max_weight == 0 {
        return None;
    }

    let normal_weight = MAX_U64 - 1;

    let index_scale = (high as f64) / (weights_len as f64);

    let weights_scale = (normal_weight as f64) / weights_sum;

    let rnd = random_u64(0, normal_weight) as f64;

    let mut temp = 0f64;

    for (i, &w) in weights.iter().enumerate() {
        temp += (w as f64) * weights_scale;
        if temp > rnd {
            let index = ((i as f64) * index_scale) as u64;

            return Some(random_u64(index, ((((i + 1) as f64) * index_scale) - 1f64) as u64) as usize);
        }
    }

    None
}

fn random_u64(a: u64, b: u64) -> u64 {
    let rnd: u64 = rand::thread_rng().gen();

    if b >= a {
        (rnd % (b - a + 1)) + a
    } else {
        (rnd % (a - b + 1)) + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_u64() {
        let mut result = Vec::new();

        let n = 1000000;

        let nn = n / 10;

        for _ in 0..n {
            result.push(random_u64(0, 9));
        }

        let mut counter = [0usize; 10];

        for i in result {
            counter[i as usize] += 1;
        }

        let mut errs = [0f64; 10];

        for (i, &c) in counter.iter().enumerate() {
            errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / (nn as f64);
        }

        for &err in errs.iter() {
            assert!(err < 0.02);
        }
    }

    #[test]
    fn test_gen_index_with_weights_1() {
        let mut result = Vec::new();

        let n = 1000000;
        let weights = [5, 10];

        for _ in 0..n {
            result.push(gen_usize_with_weights(2, &weights).unwrap());
        }

        let mut counter = [0usize; 2];

        for i in result {
            counter[i] += 1;
        }

        let a = (counter[0] as f64) * (weights[1] as f64) / (weights[0] as f64);

        let b = counter[1] as f64;

        let err = (b - a).abs() / b;

        assert!(err < 0.02);
    }

    #[test]
    fn test_gen_index_with_weights_2() {
        let mut result = Vec::new();

        let n = 1000000;
        let weights = [5, 10, 15, 20, 25];

        for _ in 0..n {
            result.push(gen_usize_with_weights(5, &weights).unwrap());
        }

        let mut counter = [0usize; 5];

        for i in result {
            counter[i] += 1;
        }

        for i in 0..5 {
            for j in i..5 {
                let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

                let b = counter[j] as f64;

                let err = (b - a).abs() / b;

                assert!(err < 0.02);
            }
        }
    }

    #[test]
    fn test_gen_index_with_weights_3() {
        let mut result = Vec::new();

        let n = 1000000;
        let weights = [5, 10];

        for _ in 0..n {
            result.push(gen_usize_with_weights(10, &weights).unwrap());
        }

        let mut counter = [0usize; 2];

        for i in result {
            if i < 5 {
                counter[0] += 1;
            } else {
                counter[1] += 1;
            }
        }

        let a = (counter[0] as f64) * (weights[1] as f64) / (weights[0] as f64);

        let b = counter[1] as f64;

        let err = (b - a).abs() / b;

        assert!(err < 0.02);
    }

    #[test]
    fn test_gen_index_with_weights_4() {
        let mut result = Vec::new();

        let n = 1000000;
        let weights = [5, 10, 15, 20, 25];

        for _ in 0..n {
            result.push(gen_usize_with_weights(10, &weights).unwrap());
        }

        let mut counter = [0usize; 5];

        for i in result {
            if i < 2 {
                counter[0] += 1;
            } else if i < 4 {
                counter[1] += 1;
            } else if i < 6 {
                counter[2] += 1;
            } else if i < 8 {
                counter[3] += 1;
            } else {
                counter[4] += 1;
            }
        }

        for i in 0..5 {
            for j in i..5 {
                let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

                let b = counter[j] as f64;

                let err = (b - a).abs() / b;

                assert!(err < 0.02);
            }
        }
    }

    #[test]
    fn test_pick_from_slice() {
        enum Prize {
            Legendary,
            Rare,
            Enchanted,
            Common,
        }

        let prize_list = [Prize::Legendary, Prize::Rare, Prize::Enchanted, Prize::Common];

        let weights = [1, 5, 15, 30];


        let n = 1000000;
        let mut result = Vec::new();

        for _ in 0..n {
            let picked_item = pick_from_slice(&prize_list, &weights).unwrap();

            result.push(picked_item);
        }

        let mut counter = [0usize; 4];

        for ref picked_item in result {
            match picked_item {
                Prize::Legendary=>{
                    counter[0] += 1;
                }
                Prize::Rare=>{
                    counter[1] += 1;
                }
                Prize::Enchanted=>{
                    counter[2] += 1;
                }
                Prize::Common=>{
                    counter[3] += 1;
                }
            }
        }

        for i in 0..4 {
            for j in i..4 {
                let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

                let b = counter[j] as f64;

                let err = (b - a).abs() / b;

                assert!(err < 0.02);
            }
        }
    }
}
