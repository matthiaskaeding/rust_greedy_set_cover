use std::{collections::HashMap, collections::HashSet, hash::RandomState};
pub type Set = HashSet<u32>;
pub type SetVec = Vec<Set>;
use std::collections::hash_set::Intersection;

fn make_universe(sets: &SetVec) -> Set {
    let universe: Set = sets.iter().flatten().cloned().collect();
    universe
}

/// First greedy set cover algorithm.
/// Directly implements whats in virtually every textbook.
///
/// # Arguments
///
/// * `sets` - A reference to a  SetCollection Vec<HashSet<u32>>;
///
/// # Returns
///
/// A SetCollection of sets covering the universe defined by the input sets
pub fn greedy_set_cover_0(sets: &SetVec) -> SetVec {
    // At start, every element is uncovered
    let mut uncovered_elements = make_universe(sets);
    //let mut covered_elements = Set::new();
    // Output: a collection of sets usually smaller than the input collection
    let mut covering_sets = SetVec::new();
    let n_sets: usize = sets.len();

    for _ in 0..n_sets {
        if uncovered_elements.is_empty() {
            break;
        }
        // Find biggest intersection of uncovered elements and each set
        let mut len_biggest: usize = 0;
        let mut i_biggest: usize = 0;
        let mut intersection: Intersection<u32, RandomState>;
        for i in 0..n_sets {
            intersection = uncovered_elements.intersection(&sets[i]);
            let len_intersection = intersection.count();
            if len_intersection > len_biggest {
                i_biggest = i;
                len_biggest = len_intersection;
            }
        }
        let biggest_set: Set = sets[i_biggest].clone();
        // Remove covered elements from "uncovered"
        uncovered_elements.retain(|&x| !&biggest_set.contains(&x));

        covering_sets.push(biggest_set);
    }

    covering_sets
}

// Second version, closer to RcppGreedySetCover implementation
// Here we map every element to the sets. Then when we cover an element,
// we can remove that from the mapping

fn map_elements_to_sets(sets: &SetVec) -> HashMap<u32, Vec<usize>> {
    let mut el_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (i_set, set) in sets.iter().enumerate() {
        for &el in set.iter() {
            el_map.entry(el).or_default().push(i_set);
        }
    }
    el_map
}

#[test]
fn test_make_universe() {
    let mut sets: SetVec = SetVec::new();
    sets.push(Set::from([1, 2, 3]));
    sets.push(Set::from([2, 3, 4]));
    sets.push(Set::from([11, 2, 800]));

    let universe: Set = make_universe(&sets);
    assert!(universe.contains(&1));
    assert!(universe.contains(&2));
    assert!(universe.contains(&3));
    assert!(universe.contains(&11));
    assert!(universe.contains(&800));
    let desired_length: usize = 6;
    assert_eq!(universe.len(), desired_length);

    // Empty set
    let mut sets: SetVec = SetVec::new();
    assert_eq!(make_universe(&sets).len(), 0);
    // Some more
    sets.push(Set::from([1, 2, 3]));
    sets.push(Set::from([2, 3, 4, 5, 6, 7, 8]));
    sets.push(Set::from([999]));
    assert_eq!(
        make_universe(&sets),
        Set::from([1, 2, 3, 4, 5, 6, 7, 8, 999])
    )
}

#[cfg(test)]
mod tests {
    use crate::simulate::draw_set_vec;

    use super::*;

    #[test]
    fn test_basic_case() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::from([100]));
        sets.push(Set::from([15]));
        sets.push(Set::from([1, 2]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_with_empty_set() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::new());
        sets.push(Set::from([3, 4, 5]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_all_sets_needed() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1]));
        sets.push(Set::from([2]));
        sets.push(Set::from([3]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(sets.len(), set_cover.len());
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_one_set_covers_all() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3, 4, 5]));
        sets.push(Set::from([1, 2]));
        sets.push(Set::from([3, 4]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(set_cover.len(), 1);
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_overlapping_sets() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::from([3, 4, 5]));
        sets.push(Set::from([5, 6, 7]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_large_numbers() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1000000, 2000000, 3000000]));
        sets.push(Set::from([4000000, 5000000]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_duplicate_sets() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::from([4, 5, 6]));
        let set_cover = greedy_set_cover_0(&sets);
        assert!(set_cover.len() < sets.len());
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_one_element_per_set() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1]));
        sets.push(Set::from([2]));
        sets.push(Set::from([3]));
        sets.push(Set::from([4]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(sets.len(), set_cover.len());
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_nested_sets() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3, 4, 5]));
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::from([1, 2]));
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(set_cover.len(), 1);
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_large_number_of_small_sets() {
        let mut sets: SetVec = SetVec::new();
        for i in 0..100 {
            sets.push(Set::from([i, i + 1]));
        }
        let set_cover: Vec<HashSet<u32>> = greedy_set_cover_0(&sets);
        assert!(set_cover.len() < sets.len());
        assert_eq!(make_universe(&sets), make_universe(&set_cover));
    }

    #[test]
    fn test_random_sets() {
        let set_vec = draw_set_vec(10, 100, 20);
        let set_cover: Vec<HashSet<u32>> = greedy_set_cover_0(&set_vec);
        assert_eq!(make_universe(&set_vec), make_universe(&set_cover));

        let set_vec = draw_set_vec(30, 70, 50);
        let set_cover: Vec<HashSet<u32>> = greedy_set_cover_0(&set_vec);
        assert_eq!(make_universe(&set_vec), make_universe(&set_cover));

        let set_vec = draw_set_vec(200, 1000, 200);
        let set_cover: Vec<HashSet<u32>> = greedy_set_cover_0(&set_vec);
        assert_eq!(make_universe(&set_vec), make_universe(&set_cover));
    }

    #[test]
    fn test_map_elements_to_sets() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2, 3]));
        sets.push(Set::from([2, 3, 4]));
        sets.push(Set::from([3, 4, 5]));
        sets.push(Set::from([5, 6, 7]));
        let mp = map_elements_to_sets(&sets);

        assert_eq!(mp.get(&1), Some(&vec![0]));
        assert_eq!(mp.get(&2), Some(&vec![0, 1]));
        assert_eq!(mp.get(&3), Some(&vec![0, 1, 2]));
        assert_eq!(mp.get(&4), Some(&vec![1, 2]));
        assert_eq!(mp.get(&5), Some(&vec![2, 3]));
        assert_eq!(mp.get(&6), Some(&vec![3]));
        assert_eq!(mp.get(&7), Some(&vec![3]));
    }

    #[test]
    fn test_map_elements_to_sets_with_empty_set() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1, 2]));
        sets.push(Set::new()); // Empty set
        sets.push(Set::from([2, 3]));
        let mp = map_elements_to_sets(&sets);

        assert_eq!(mp.get(&1), Some(&vec![0]));
        assert_eq!(mp.get(&2), Some(&vec![0, 2]));
        assert_eq!(mp.get(&3), Some(&vec![2]));
        assert_eq!(mp.get(&4), None);
    }

    #[test]
    fn test_map_elements_to_sets_single_element() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1]));
        sets.push(Set::from([1]));
        sets.push(Set::from([2]));
        let mp = map_elements_to_sets(&sets);

        assert_eq!(mp.get(&1), Some(&vec![0, 1]));
        assert_eq!(mp.get(&2), Some(&vec![2]));
        assert_eq!(mp.get(&3), None);
    }

    #[test]
    fn test_map_elements_to_sets_large_numbers() {
        let mut sets: SetVec = SetVec::new();
        sets.push(Set::from([1000000, 2000000]));
        sets.push(Set::from([2000000, 3000000]));
        let mp = map_elements_to_sets(&sets);

        assert_eq!(mp.get(&1000000), Some(&vec![0]));
        assert_eq!(mp.get(&2000000), Some(&vec![0, 1]));
        assert_eq!(mp.get(&3000000), Some(&vec![1]));
        assert_eq!(mp.get(&4000000), None);
    }

    #[test]
    fn test_map_elements_to_sets_all_empty() {
        let sets: SetVec = vec![Set::new(), Set::new(), Set::new()];
        let mp = map_elements_to_sets(&sets);

        assert!(mp.is_empty());
    }
}
