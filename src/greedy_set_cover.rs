use std::{collections::HashMap, collections::HashSet, hash::RandomState};
pub type Set = HashSet<u32>;
pub type SetVec = Vec<Set>;
pub type SetVecMap = HashMap<String, SetVec>;
use std::collections::hash_set::Intersection;

fn make_universe(sets: &SetVecMap) -> Set {
    let universe: Set = sets.values().flatten().flatten().cloned().collect();
    universe
}

/// First greedy set cover algorithm.
/// Directly implements whats in virtually every textbook.
///
/// # Arguments
///
/// * `sets` - A reference to a HashMap<String, SetVec> where each key is a category and the value is a SetVec;
///
/// # Returns
///
/// A SetCollection of sets covering the universe defined by the input sets
pub fn greedy_set_cover_0(sets: &SetVecMap) -> SetVec {
    // At start, every element is uncovered
    let mut uncovered_elements = make_universe(sets);
    // Output: a collection of sets usually smaller than the input collection
    let mut covering_sets = SetVec::new();

    // Flatten all sets into a single vector for processing
    let all_sets: SetVec = sets.values().flatten().cloned().collect();
    let n_sets: usize = all_sets.len();

    for _ in 0..n_sets {
        if uncovered_elements.is_empty() {
            break;
        }
        // Find biggest intersection of uncovered elements and each set
        let mut len_biggest: usize = 0;
        let mut i_biggest: usize = 0;
        let mut intersection: Intersection<u32, RandomState>;
        for i in 0..n_sets {
            intersection = uncovered_elements.intersection(&all_sets[i]);
            let len_intersection = intersection.count();
            if len_intersection > len_biggest {
                i_biggest = i;
                len_biggest = len_intersection;
            }
        }
        let biggest_set: Set = all_sets[i_biggest].clone();
        // Remove covered elements from "uncovered"
        uncovered_elements.retain(|&x| !&biggest_set.contains(&x));

        covering_sets.push(biggest_set);
    }

    covering_sets
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulate::draw_set_vec;

    #[test]
    fn test_basic_case() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3]), Set::from([100])],
        );
        sets.insert(
            "category2".to_string(),
            vec![Set::from([15]), Set::from([1, 2])],
        );
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_with_empty_set() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3]), Set::new()],
        );
        sets.insert("category2".to_string(), vec![Set::from([3, 4, 5])]);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_all_sets_needed() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1]), Set::from([2])],
        );
        sets.insert("category2".to_string(), vec![Set::from([3])]);
        let set_cover = greedy_set_cover_0(&sets);
        let all_sets: SetVec = sets.values().flatten().cloned().collect();
        assert_eq!(all_sets.len(), set_cover.len());
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_one_set_covers_all() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert("category1".to_string(), vec![Set::from([1, 2, 3, 4, 5])]);
        sets.insert(
            "category2".to_string(),
            vec![Set::from([1, 2]), Set::from([3, 4])],
        );
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(set_cover.len(), 1);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_overlapping_sets() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3]), Set::from([3, 4, 5])],
        );
        sets.insert("category2".to_string(), vec![Set::from([5, 6, 7])]);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_large_numbers() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1000000, 2000000, 3000000])],
        );
        sets.insert("category2".to_string(), vec![Set::from([4000000, 5000000])]);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_duplicate_sets() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3]), Set::from([1, 2, 3])],
        );
        sets.insert("category2".to_string(), vec![Set::from([4, 5, 6])]);
        let set_cover = greedy_set_cover_0(&sets);
        let all_sets: SetVec = sets.values().flatten().cloned().collect();
        assert!(set_cover.len() < all_sets.len());
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_one_element_per_set() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1]), Set::from([2])],
        );
        sets.insert(
            "category2".to_string(),
            vec![Set::from([3]), Set::from([4])],
        );
        let set_cover = greedy_set_cover_0(&sets);
        let all_sets: SetVec = sets.values().flatten().cloned().collect();
        assert_eq!(all_sets.len(), set_cover.len());
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_nested_sets() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3, 4, 5]), Set::from([1, 2, 3])],
        );
        sets.insert("category2".to_string(), vec![Set::from([1, 2])]);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(set_cover.len(), 1);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_large_number_of_small_sets() {
        let mut sets: SetVecMap = HashMap::new();
        let mut category1_sets = Vec::new();
        let mut category2_sets = Vec::new();
        for i in 0..50 {
            category1_sets.push(Set::from([i, i + 1]));
            category2_sets.push(Set::from([i + 50, i + 51]));
        }
        sets.insert("category1".to_string(), category1_sets);
        sets.insert("category2".to_string(), category2_sets);
        let set_cover = greedy_set_cover_0(&sets);
        let all_sets: SetVec = sets.values().flatten().cloned().collect();
        assert!(set_cover.len() < all_sets.len());
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_random_sets() {
        let set_vec = draw_set_vec(10, 100, 20);
        let mut sets: SetVecMap = HashMap::new();
        sets.insert("category1".to_string(), set_vec);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );

        let set_vec = draw_set_vec(30, 70, 50);
        let mut sets: SetVecMap = HashMap::new();
        sets.insert("category1".to_string(), set_vec);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );

        let set_vec = draw_set_vec(200, 1000, 200);
        let mut sets: SetVecMap = HashMap::new();
        sets.insert("category1".to_string(), set_vec);
        let set_cover = greedy_set_cover_0(&sets);
        assert_eq!(
            make_universe(&sets),
            make_universe(&HashMap::from([("all".to_string(), set_cover.clone())]))
        );
    }

    #[test]
    fn test_make_universe() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3]), Set::from([2, 3, 4])],
        );
        sets.insert("category2".to_string(), vec![Set::from([11, 2, 800])]);

        let universe: Set = make_universe(&sets);
        assert!(universe.contains(&1));
        assert!(universe.contains(&2));
        assert!(universe.contains(&3));
        assert!(universe.contains(&11));
        assert!(universe.contains(&800));
        let desired_length: usize = 6;
        assert_eq!(universe.len(), desired_length);

        // Empty set
        let mut sets: SetVecMap = HashMap::new();
        assert_eq!(make_universe(&sets).len(), 0);
        // Some more
        sets.insert(
            "category1".to_string(),
            vec![Set::from([1, 2, 3]), Set::from([2, 3, 4, 5, 6, 7, 8])],
        );
        sets.insert("category2".to_string(), vec![Set::from([999])]);
        assert_eq!(
            make_universe(&sets),
            Set::from([1, 2, 3, 4, 5, 6, 7, 8, 999])
        )
    }
}
