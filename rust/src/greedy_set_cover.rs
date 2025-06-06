use bitvec::prelude::*;
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
pub fn greedy_set_cover_0<K, T>(sets: &HashMap<K, Vec<T>>) -> HashSet<K>
where
    K: Clone + Hash + Eq + std::fmt::Debug, // Added Debug for error message
    T: Clone + Hash + Eq + std::fmt::Debug, // Added Debug for error message
{
    let mut uncovered_elements: HashSet<T> = sets.values().flatten().cloned().collect();
    let mut cover = HashSet::new();

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
            cover.insert(key.clone());
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

/// Creates a mapping from unique elements to consecutive integers (0, 1, 2...).
///
/// This function iterates through a collection of elements and assigns a unique `usize`
/// identifier to each unique element it encounters.
///
/// # Type Parameters
///
/// * `T`: The type of the elements. It must be hashable and equatable to be used
///   as a key in the resulting `HashMap`, and cloneable to be owned by the map.
/// * `I`: An iterator that yields references to elements of type `T`.
///
/// # Arguments
///
/// * `elements`: An iterator providing the elements to be mapped. Duplicates are handled
///   gracefully; they will all point to the same integer ID.
///
/// # Returns
///
/// A `HashMap` where each key is a unique element and the value is its assigned integer ID.
pub fn map_elements_to_integers<T, I>(elements: I) -> HashMap<T, usize>
where
    T: Hash + Eq + Clone,
    I: IntoIterator<Item = T>,
{
    let mut mapping = HashMap::new();
    let mut next_id = 0;
    for element in elements {
        // The `entry` API is efficient: it only performs one hash lookup.
        // `or_insert_with` only executes the closure if the key is new.
        mapping.entry(element).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
    }
    mapping
}

/// Inverts a mapping from elements to integers.
///
/// Given a `HashMap<T, usize>`, this function creates a `HashMap<usize, T>`,
/// allowing for quick lookups of the original element from an integer ID.
///
/// # Type Parameters
///
/// * `T`: The type of the elements, which must be cloneable to be used as a value
///   in the new map.
///
/// # Arguments
///
/// * `mapping`: A reference to the forward mapping (`element -> integer`).
///
/// # Returns
///
/// A `HashMap` where each key is an integer ID and the value is the original element.
pub fn revert_integer_mapping<T: Clone>(mapping: &HashMap<T, usize>) -> HashMap<usize, T> {
    mapping
        .iter()
        .map(|(element, &id)| (id, element.clone()))
        .collect()
}

/// Creates a mapping from unique elements to consecutive integers (0, 1, 2...).
///
/// This function iterates through a collection of elements and assigns a unique `usize`
/// identifier to each unique element it encounters.
///
/// # Type Parameters
///
/// * `T`: The type of the elements. It must be hashable and equatable to be used
///   as a key in the resulting `HashMap`, and cloneable to be owned by the map.
/// * `I`: An iterator that yields references to elements of type `T`.
///
/// # Arguments
///
/// * `elements`: An iterator providing the elements to be mapped. Duplicates are handled
///   gracefully; they will all point to the same integer ID.
///
/// # Returns
///
/// A `HashMap` where each key is a unique element and the value is its assigned integer ID.
pub fn map_elements_to_integers_owned<T, I>(elements: I) -> HashMap<T, usize>
where
    T: Hash + Eq + Clone,
    I: IntoIterator<Item = T>,
{
    let mut mapping = HashMap::new();
    let mut next_id = 0;
    for element in elements {
        mapping.entry(element).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
    }
    mapping
}

/// Creates a HashSet containing all unique elements from the input sets.
///
/// # Arguments
///
/// * `sets`: A `HashMap` where keys are the identifiers of the sets and values are vectors
///   of the elements in each set.
///
/// # Type Parameters
///
/// * `K`: The type of the set identifiers (keys in the HashMap).
/// * `T`: The type of the elements within the sets. Must be cloneable, hashable, and equatable.
///
/// # Returns
///
/// A `HashSet` containing all unique elements from the input sets.
fn make_universe<K, T>(sets: &HashMap<K, Vec<T>>) -> HashSet<T>
where
    T: Clone + Hash + Eq,
{
    sets.values().flatten().cloned().collect()
}

/// Finds an approximate solution to the set cover problem using a greedy algorithm.
/// Maps all elements to integer first, then leveraging set operation on integers
/// This incurs cost at the beginning but is faster later, so if this better than
/// algorithm 0 depends on the number of sets and elements and number of needed sets -
/// so it will be hard to say in advance
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
pub fn greedy_set_cover_1<K, T>(sets: &HashMap<K, Vec<T>>) -> HashSet<K>
where
    K: Clone + Hash + Eq + std::fmt::Debug,
    T: Clone + Hash + Eq + std::fmt::Debug,
{
    // ... (preprocessing and bit_sets creation is identical)
    let universe = make_universe(sets);
    let mapping = map_elements_to_integers_owned(universe.into_iter());
    let universe_size = mapping.len();
    let mut bit_sets: HashMap<K, BitVec> = HashMap::new();
    for (key, elements) in sets {
        let mut bv = bitvec![0; universe_size];
        for element in elements {
            if let Some(&id) = mapping.get(element) {
                bv.set(id, true);
            }
        }
        bit_sets.insert(key.clone(), bv);
    }

    let mut uncovered_elements = bitvec![1; universe_size];
    let mut cover: HashSet<K> = HashSet::new();

    // OPTIMIZATION: Create a reusable buffer for intersection calculations.
    // We allocate it once here, outside all loops that use it.
    let mut intersection_buffer = BitVec::with_capacity(universe_size);

    for _ in 0..sets.len() {
        if uncovered_elements.not_any() {
            break;
        }

        let mut best_set_key: Option<K> = None;
        let mut best_set_covered_count = 0;
        let mut best_intersection: Option<BitVec> = None;

        for (key, bit_set) in &bit_sets {
            // OPTIMIZATION: Instead of `clone`, use `clone_from` to reuse the
            // buffer's allocation. This turns a potentially slow allocation
            // into a much faster memory copy.
            intersection_buffer.clone_from(bit_set);
            intersection_buffer &= &uncovered_elements;

            let covered_count = intersection_buffer.count_ones();

            if covered_count > best_set_covered_count {
                best_set_key = Some(key.clone());
                best_set_covered_count = covered_count;
                // We still need to clone here to save the result for later,
                // as the buffer will be overwritten in the next iteration.
                best_intersection = Some(intersection_buffer.clone());
            }
        }

        if let Some(key) = best_set_key {
            if let Some(elements_to_remove) = best_intersection {
                uncovered_elements &= &!elements_to_remove;
            }
            cover.insert(key);
        } else if uncovered_elements.any() {
            panic!("Error: Unable to find a set to cover remaining elements.");
        }
    }

    if uncovered_elements.any() {
        panic!("Error: Could not cover all elements.");
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

    // --- Rewritten Tests ---

    #[test]
    fn test_basic_case() {
        let mut sets = HashMap::new();
        sets.insert("A".to_string(), vec![1, 2, 3]);
        sets.insert("B".to_string(), vec![1, 2]);
        sets.insert("C".to_string(), vec![2]);

        let set_cover = greedy_set_cover_1(&sets);
        let universe = make_universe(&sets);

        let covered_sets: HashMap<String, Vec<i32>> = set_cover
            .iter()
            .map(|key| (key.clone(), sets.get(key).unwrap().clone()))
            .collect();

        let covered_universe = make_universe(&covered_sets);

        assert_eq!(universe, covered_universe);
    }

    // #[test]
    // fn test_with_empty_set() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1, 2, 3]), Set::new()]);
    //     sets.insert(2, vec![Set::from([3, 4, 5])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_all_sets_needed() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1]), Set::from([2])]);
    //     sets.insert(2, vec![Set::from([3])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert_eq!(function_input.len(), set_cover.len());
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_one_set_covers_all() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1, 2, 3, 4, 5])]);
    //     sets.insert(2, vec![Set::from([1, 2]), Set::from([3, 4])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert_eq!(set_cover.len(), 1);
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_overlapping_sets() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1, 2, 3]), Set::from([3, 4, 5])]);
    //     sets.insert(2, vec![Set::from([5, 6, 7])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     // The greedy algorithm might pick 2 or 3 sets, but the universe must be covered.
    //     assert!(set_cover.len() >= 2 && set_cover.len() <= 3);
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_large_numbers() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1_000_000, 2_000_000, 3_000_000])]);
    //     sets.insert(2, vec![Set::from([4_000_000, 5_000_000])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_duplicate_sets() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1, 2, 3]), Set::from([1, 2, 3])]);
    //     sets.insert(2, vec![Set::from([4, 5, 6])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert!(set_cover.len() < function_input.len());
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_one_element_per_set() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1]), Set::from([2])]);
    //     sets.insert(2, vec![Set::from([3]), Set::from([4])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert_eq!(function_input.len(), set_cover.len());
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_nested_sets() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     sets.insert(1, vec![Set::from([1, 2, 3, 4, 5]), Set::from([1, 2, 3])]);
    //     sets.insert(2, vec![Set::from([1, 2])]);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert_eq!(set_cover.len(), 1);
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_large_number_of_small_sets() {
    //     let mut sets: SetVecMap = HashMap::new();
    //     let mut category1_sets = Vec::new();
    //     let mut category2_sets = Vec::new();
    //     for i in 0..50 {
    //         category1_sets.push(Set::from([i, i + 1]));
    //         category2_sets.push(Set::from([i + 50, i + 51]));
    //     }
    //     sets.insert(1, category1_sets);
    //     sets.insert(2, category2_sets);

    //     let function_input = transform_input(&sets);
    //     let set_cover = greedy_set_cover_0(&function_input);

    //     assert!(set_cover.len() < function_input.len());
    //     assert_eq!(
    //         make_universe_from_test_input(&sets),
    //         make_universe_from_cover_output(&set_cover)
    //     );
    // }

    // #[test]
    // fn test_complex_deterministic_cases() {
    //     // Case 1: A clear greedy choice path
    //     let mut sets1: SetVecMap = HashMap::new();
    //     sets1.insert(
    //         1,
    //         vec![
    //             Set::from([1, 2, 3, 4, 5, 6]), // S1 (Best initial choice)
    //             Set::from([1, 2, 7]),          // S2
    //             Set::from([3, 4, 8]),          // S3
    //             Set::from([5, 6, 9]),          // S4
    //             Set::from([7, 8, 9, 10]),      // S5 (Best second choice)
    //         ],
    //     );
    //     let function_input1 = transform_input(&sets1);
    //     let set_cover1 = greedy_set_cover_0(&function_input1);
    //     // The optimal greedy cover is 2 sets: {1,2,3,4,5,6} and {7,8,9,10}
    //     assert_eq!(set_cover1.len(), 2);
    //     assert_eq!(
    //         make_universe_from_test_input(&sets1),
    //         make_universe_from_cover_output(&set_cover1)
    //     );

    //     // Case 2: A less obvious greedy path
    //     let mut sets2: SetVecMap = HashMap::new();
    //     sets2.insert(
    //         1,
    //         vec![
    //             Set::from([1, 2, 3]),     // S1
    //             Set::from([4, 5, 6]),     // S2
    //             Set::from([7, 8, 9]),     // S3
    //             Set::from([1, 4, 7]),     // S4
    //             Set::from([2, 5, 8]),     // S5
    //             Set::from([3, 6, 9, 10]), // S6 (covers one unique element '10')
    //         ],
    //     );
    //     let function_input2 = transform_input(&sets2);
    //     let set_cover2 = greedy_set_cover_0(&function_input2);
    //     // An optimal solution is 3 sets (e.g., S1,S2,S3), but a greedy one might take more.
    //     // For example S4, S5, S6, S1. The only guarantee is coverage.
    //     assert_eq!(
    //         make_universe_from_test_input(&sets2),
    //         make_universe_from_cover_output(&set_cover2)
    //     );
    // }

    // #[test]
    // fn test_with_different_types() {
    //     // Case 1: Using string slices for both keys and elements
    //     let mut sets_str: HashMap<&str, Vec<&str>> = HashMap::new();
    //     sets_str.insert("Set A", vec!["apple", "banana", "cherry"]);
    //     sets_str.insert("Set B", vec!["banana", "date"]);
    //     sets_str.insert("Set C", vec!["cherry", "fig", "grape"]);
    //     sets_str.insert("Set D", vec!["fig", "grape"]);

    //     let cover_str = greedy_set_cover_0(&sets_str);

    //     let original_universe_str: HashSet<&str> = sets_str.values().flatten().cloned().collect();
    //     let covered_universe_str: HashSet<&str> = cover_str.values().flatten().cloned().collect();

    //     // The greedy choice should be "Set A" and "Set C"
    //     assert_eq!(cover_str.len(), 3);
    //     assert_eq!(original_universe_str, covered_universe_str);
    //     assert!(cover_str.contains_key("Set A"));
    //     assert!(cover_str.contains_key("Set C"));

    //     // ---

    //     // Case 2: Using integers for keys and characters for elements
    //     let mut sets_char: HashMap<i32, Vec<char>> = HashMap::new();
    //     sets_char.insert(1, vec!['a', 'b', 'c']);
    //     sets_char.insert(2, vec!['c', 'd']);
    //     sets_char.insert(3, vec!['e', 'f']);
    //     sets_char.insert(4, vec!['a', 'd', 'e']);

    //     let cover_char = greedy_set_cover_0(&sets_char);

    //     let original_universe_char: HashSet<char> = sets_char.values().flatten().cloned().collect();
    //     let covered_universe_char: HashSet<char> = cover_char.values().flatten().cloned().collect();

    //     // A possible greedy cover is sets 1 and 4, or 1 and 3 and 2...
    //     // The most important thing is that the universe is covered.
    //     assert_eq!(original_universe_char, covered_universe_char);
    // }

    #[test]
    fn test_mapping() {
        let data = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "apple".to_string(),
        ];

        let forward_map = map_elements_to_integers(data.iter());
        assert_eq!(forward_map.len(), 3);
        assert!(forward_map.contains_key(&"apple".to_string()));
        assert!(forward_map.contains_key(&"banana".to_string()));
        assert!(forward_map.contains_key(&"cherry".to_string()));

        let values: HashSet<usize> = forward_map.values().cloned().collect();
        assert_eq!(values, HashSet::from([0, 1, 2]));
    }
}
