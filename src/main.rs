mod greedy_set_cover;
mod simulate;

use crate::greedy_set_cover::{greedy_set_cover_0, Set, SetVecMap};
use std::collections::HashMap;

fn main() {
    let mut sets: SetVecMap = HashMap::new();
    sets.insert(
        "category1".to_string(),
        vec![Set::from([1, 2]), Set::from([1, 2, 3])],
    );
    sets.insert(
        "category2".to_string(),
        vec![Set::from([4, 5]), Set::from([6, 7, 8]), Set::from([3])],
    );

    let set_cover = greedy_set_cover_0(&sets);
    let len_set_cover = set_cover.len();
    println!("Set cover achieved by {len_set_cover} sets");
    for (i, set) in set_cover.iter().enumerate() {
        for el in set.iter() {
            println!("Set {i} contains {el}");
        }
    }
}
