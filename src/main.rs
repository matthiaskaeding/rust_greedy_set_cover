mod greedy_set_cover;
mod simulate;

use crate::greedy_set_cover::{greedy_set_cover_0, Set, SetVec};

fn main() {
    let mut sets: SetVec = SetVec::new();
    sets.push(Set::from([1, 2]));
    sets.push(Set::from([1, 2, 3]));
    sets.push(Set::from([4, 5]));
    sets.push(Set::from([6, 7, 8]));
    sets.push(Set::from([3]));

    let set_cover = greedy_set_cover_0(&sets);
    let len_set_cover = set_cover.len();
    println!("Set cover achieved by {len_set_cover} sets");
    for (i, set) in set_cover.iter().enumerate() {
        for el in set.iter() {
            println!("Set {i} contains {el}");
        }
    }
}
