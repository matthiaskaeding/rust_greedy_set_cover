use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Finds an approximate solution to the set cover problem using a greedy algorithm.
///
/// # Arguments
///
/// * `sets`: A `HashMap` where keys are the identifiers of the sets and values are vectors
///   of the elements in each set.
///
/// # Type Parameters
///
/// * `K`: The type of the set identifiers (keys in the HashMap). Must be cloneable, hashable,
///   and equatable.
/// * `T`: The type of the elements within the sets. Must be cloneable, hashable, and equatable.
///
/// # Returns
///
/// A `HashMap` containing the sets that form the cover.
///
/// # Panics
///
/// Panics if the input sets do not collectively cover all of their unique elements.
pub fn greedy_set_cover_0<K, T>(sets: &HashMap<K, Vec<T>>) -> HashMap<K, Vec<T>>
where
    K: Clone + Hash + Eq + std::fmt::Debug, // Added Debug for error message
    T: Clone + Hash + Eq + std::fmt::Debug, // Added Debug for error message
{
    let mut uncovered_elements: HashSet<T> = sets.values().flatten().cloned().collect();
    let mut cover = HashMap::new();

    for _ in 0..sets.len() {
        if uncovered_elements.is_empty() {
            break;
        }

        let mut best_set_key: Option<K> = None;
        let mut best_set_covered: HashSet<T> = HashSet::new();

        // Iterate through all the provided sets to find the one that covers the most
        // currently uncovered elements.
        for (key, set_elements) in sets {
            let covered_by_this_set: HashSet<T> = set_elements
                .iter()
                .filter(|element| uncovered_elements.contains(element))
                .cloned()
                .collect();

            if covered_by_this_set.len() > best_set_covered.len() {
                best_set_key = Some(key.clone());
                best_set_covered = covered_by_this_set;
            }
        }

        // If a best set was found, add it to the cover and remove its elements from the universe.
        if let Some(key) = best_set_key {
            uncovered_elements.retain(|e| !best_set_covered.contains(e));
            cover.insert(key.clone(), sets.get(&key).unwrap().clone());
        } else if !uncovered_elements.is_empty() {
            panic!(
                "Error: Unable to find a set to cover the remaining elements: {:?}",
                uncovered_elements
            );
        }
    }

    if !uncovered_elements.is_empty() {
        panic!(
            "Error: Could not cover all elements after iterating through all sets. Remaining elements: {:?}",
            uncovered_elements
        );
    }

    cover
}

#[cfg(test)]
mod tests {
    use super::*; // Imports greedy_set_cover from the parent module
    use std::collections::{HashMap, HashSet};

    type Set = HashSet<i32>;
    type SetVec = Vec<Set>;
    type SetVecMap = HashMap<i32, SetVec>;

    fn make_universe_from_test_input(sets: &SetVecMap) -> Set {
        sets.values()
            .flatten()
            .flat_map(|s| s.iter().cloned())
            .collect()
    }

    fn make_universe_from_cover_output(cover: &HashMap<usize, Vec<i32>>) -> Set {
        cover.values().flatten().cloned().collect()
    }

    fn transform_input(sets: &SetVecMap) -> HashMap<usize, Vec<i32>> {
        let mut function_input = HashMap::new();
        let mut set_id_counter = 0;
        for vec_of_sets in sets.values() {
            for set in vec_of_sets {
                // We only consider non-empty sets for the cover problem.
                if !set.is_empty() {
                    function_input.insert(set_id_counter, set.iter().cloned().collect());
                    set_id_counter += 1;
                }
            }
        }
        function_input
    }

    // --- Rewritten Tests ---

    #[test]
    fn test_basic_case() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1, 2, 3]), Set::from([100])]);
        sets.insert(2, vec![Set::from([15]), Set::from([1, 2])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_with_empty_set() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1, 2, 3]), Set::new()]);
        sets.insert(2, vec![Set::from([3, 4, 5])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_all_sets_needed() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1]), Set::from([2])]);
        sets.insert(2, vec![Set::from([3])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(function_input.len(), set_cover.len());
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_one_set_covers_all() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1, 2, 3, 4, 5])]);
        sets.insert(2, vec![Set::from([1, 2]), Set::from([3, 4])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(set_cover.len(), 1);
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_overlapping_sets() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1, 2, 3]), Set::from([3, 4, 5])]);
        sets.insert(2, vec![Set::from([5, 6, 7])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        // The greedy algorithm might pick 2 or 3 sets, but the universe must be covered.
        assert!(set_cover.len() >= 2 && set_cover.len() <= 3);
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_large_numbers() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1_000_000, 2_000_000, 3_000_000])]);
        sets.insert(2, vec![Set::from([4_000_000, 5_000_000])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_duplicate_sets() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1, 2, 3]), Set::from([1, 2, 3])]);
        sets.insert(2, vec![Set::from([4, 5, 6])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert!(set_cover.len() < function_input.len());
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_one_element_per_set() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1]), Set::from([2])]);
        sets.insert(2, vec![Set::from([3]), Set::from([4])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(function_input.len(), set_cover.len());
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_nested_sets() {
        let mut sets: SetVecMap = HashMap::new();
        sets.insert(1, vec![Set::from([1, 2, 3, 4, 5]), Set::from([1, 2, 3])]);
        sets.insert(2, vec![Set::from([1, 2])]);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert_eq!(set_cover.len(), 1);
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
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
        sets.insert(1, category1_sets);
        sets.insert(2, category2_sets);

        let function_input = transform_input(&sets);
        let set_cover = greedy_set_cover_0(&function_input);

        assert!(set_cover.len() < function_input.len());
        assert_eq!(
            make_universe_from_test_input(&sets),
            make_universe_from_cover_output(&set_cover)
        );
    }

    #[test]
    fn test_complex_deterministic_cases() {
        // Case 1: A clear greedy choice path
        let mut sets1: SetVecMap = HashMap::new();
        sets1.insert(
            1,
            vec![
                Set::from([1, 2, 3, 4, 5, 6]), // S1 (Best initial choice)
                Set::from([1, 2, 7]),          // S2
                Set::from([3, 4, 8]),          // S3
                Set::from([5, 6, 9]),          // S4
                Set::from([7, 8, 9, 10]),      // S5 (Best second choice)
            ],
        );
        let function_input1 = transform_input(&sets1);
        let set_cover1 = greedy_set_cover_0(&function_input1);
        // The optimal greedy cover is 2 sets: {1,2,3,4,5,6} and {7,8,9,10}
        assert_eq!(set_cover1.len(), 2);
        assert_eq!(
            make_universe_from_test_input(&sets1),
            make_universe_from_cover_output(&set_cover1)
        );

        // Case 2: A less obvious greedy path
        let mut sets2: SetVecMap = HashMap::new();
        sets2.insert(
            1,
            vec![
                Set::from([1, 2, 3]),     // S1
                Set::from([4, 5, 6]),     // S2
                Set::from([7, 8, 9]),     // S3
                Set::from([1, 4, 7]),     // S4
                Set::from([2, 5, 8]),     // S5
                Set::from([3, 6, 9, 10]), // S6 (covers one unique element '10')
            ],
        );
        let function_input2 = transform_input(&sets2);
        let set_cover2 = greedy_set_cover_0(&function_input2);
        // An optimal solution is 3 sets (e.g., S1,S2,S3), but a greedy one might take more.
        // For example S4, S5, S6, S1. The only guarantee is coverage.
        assert_eq!(
            make_universe_from_test_input(&sets2),
            make_universe_from_cover_output(&set_cover2)
        );
    }
}
