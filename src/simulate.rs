use rand::Rng;
use std::{collections::HashSet, usize};
pub type Set = HashSet<i32>;
// type SetCollection = Vec<Set>;
use rand::distributions::{Distribution, Uniform};

pub fn draw_uniform_discrete(int_u: i32, n_draws: usize) -> Set {
    let between = Uniform::from(0..int_u);
    let mut set: Set = Set::new();
    let mut rng = rand::thread_rng();

    for _ in 0..n_draws {
        let draw: i32 = between.sample(&mut rng);
        set.insert(draw);
    }

    return set;
}

pub fn draw_vector(n: usize, max: i32) -> Vec<i32> {
    let mut out: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut rng = rand::thread_rng();
        out[i] = rng.gen_range(0..max);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_vector() {
        let n = 100;
        let max = 50;
        let result = draw_vector(n, max);

        // Test 1: Check if the vector has the correct length
        assert_eq!(result.len(), n, "Vector length should be {}", n);

        // Test 2: Check if all elements are within the correct range
        for &value in &result {
            assert!(
                value >= 0 && value < max,
                "Value {} is out of range [0, {})",
                value,
                max
            );
        }

        // Test 3: Check if the vector contains at least some variation
        let unique_values: HashSet<i32> = result.iter().cloned().collect();
        assert!(
            unique_values.len() > 1,
            "Vector should contain varied values"
        );

        // Test 4: Check if the function produces different results on subsequent calls
        let another_result = draw_vector(n, max);
        assert_ne!(
            result, another_result,
            "Subsequent calls should produce different vectors"
        );
    }
}
