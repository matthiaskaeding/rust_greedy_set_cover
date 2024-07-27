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
