mod greedy_set_cover;

use crate::greedy_set_cover::greedy_set_cover_0;
use std::collections::HashMap;

fn main() {
    // Example with string slices as set identifiers and integers as elements.
    let mut sets_to_cover: HashMap<&str, Vec<i32>> = HashMap::new();
    sets_to_cover.insert("S1", vec![1, 2, 3, 6]);
    sets_to_cover.insert("S2", vec![2, 4]);
    sets_to_cover.insert("S3", vec![3, 5]);
    sets_to_cover.insert("S4", vec![4, 5]);
    sets_to_cover.insert("S5", vec![6, 7]);

    let cover = greedy_set_cover_0(&sets_to_cover);
    println!("Selected sets to cover all elements: {:?}", cover);

    // Example with integer keys and character elements.
    let mut sets_to_cover_2: HashMap<i32, Vec<char>> = HashMap::new();
    sets_to_cover_2.insert(1, vec!['a', 'b']);
    sets_to_cover_2.insert(2, vec!['b', 'c', 'd']);
    sets_to_cover_2.insert(3, vec!['d', 'e']);

    let cover_2 = greedy_set_cover_0(&sets_to_cover_2);
    println!("Selected sets for the second example: {:?}", cover_2);
}
