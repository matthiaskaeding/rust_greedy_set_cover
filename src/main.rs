mod simulate;

use crate::simulate::Set;
use simulate::{draw_uniform_discrete, draw_vector};

// fn simulate_sets(n_sets: usize, max_elements_per_set: usize) -> SetCollection {
//     // Simulate n_sets sets, where max_elements is the maximal number of
//     // elements
//     let mut rng = rand::thread_rng();
//     let mut sets: SetCollection = SetCollection::with_capacity(n_sets);
//     for _ in 0..n_sets {
//         int n_draws = rng.gen_range(0..1000)
//         let set: Set = draw_uniform_discrete(max_elements_per_set * 2);
//         sets.push(HashSet::new());
//     }
//     sets;
// }

fn main() {
    let n_draws: usize = 10;
    let int_u: i32 = 100000;
    let _set: Set = draw_uniform_discrete(int_u, n_draws);
    let vec = draw_vector(5, 10000);

    for v in vec.iter() {
        println!("{v}")
    }

    // for val in set.iter() {
    //     println!("Value = {val}");
    // }
}
