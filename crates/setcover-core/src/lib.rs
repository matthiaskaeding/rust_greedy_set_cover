use ahash::{AHashMap, AHashSet};
use bitvec::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
#[allow(dead_code)] // This function is used by tests and the Python module

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
pub fn greedy_set_cover_1<K, T>(sets: &HashMap<K, Vec<T>>) -> Vec<K>
where
    K: Clone + Hash + Eq + std::fmt::Debug,
    T: Clone + Hash + Eq + std::fmt::Debug,
{
    // Create the element-to-integer mapping directly in a single pass.
    // This is much faster as it avoids allocating an intermediate HashSet.
    let mut mapping: AHashMap<T, usize> = AHashMap::new();
    let mut next_id = 0;
    // `sets.values().flatten()` creates an iterator over every single element
    // in all of the sets provided.
    for element in sets.values().flatten() {
        // The `.entry()` API is perfect for this. It finds the entry for a key
        // and allows us to insert a value only if the key is not already present.
        mapping.entry(element.clone()).or_insert_with(|| {
            // This code only runs the FIRST time we see a new element.
            let id = next_id;
            next_id += 1;
            id
        });
    }

    let universe_size = mapping.len();
    let mut bit_sets: AHashMap<K, BitVec> = AHashMap::new();
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
    let mut cover: AHashSet<K> = AHashSet::new();

    let mut intersection_buffer = BitVec::with_capacity(universe_size);

    for _ in 0..sets.len() {
        if uncovered_elements.not_any() {
            break;
        }

        let mut best_set_key: Option<K> = None;
        let mut best_set_covered_count = 0;
        let mut best_intersection: Option<BitVec> = None;

        for (key, bit_set) in &bit_sets {
            if cover.contains(key) {
                continue;
            }
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
    cover.into_iter().collect()
}

/// Finds an approximate solution to the set cover problem using a greedy algorithm.
/// Allows choosing between different implementations (0: HashSet-based, 1: BitVec-based).
///
/// # Arguments
///
/// * `sets`: A `HashMap` where keys are the identifiers of the sets and values are vectors
///   of the elements in each set.
/// * `algo`: An integer specifying which implementation to use (0 or 1).
///
/// # Type Parameters
///
/// * `K`: The type of the set identifiers (keys in the HashMap). Must be cloneable, hashable,
///   and equatable.
/// * `T`: The type of the elements within the sets. Must be cloneable, hashable, and equatable.
///
/// # Returns
///
/// A `HashSet` containing the keys of the sets that form the cover.
///
/// # Panics
///
/// Panics if the input sets do not collectively cover all of their unique elements,
/// or if an invalid algorithm choice is provided.
pub fn greedy_set_cover<K, T>(sets: &HashMap<K, Vec<T>>, algo: i16) -> Vec<K>
where
    K: Clone + Hash + Eq + std::fmt::Debug,
    T: Clone + Hash + Eq + std::fmt::Debug,
{
    match algo {
        0 => greedy_set_cover_0(sets),
        1 => greedy_set_cover_1(sets),
        _ => panic!("Wrong algo choice, must be 0 or 1"),
    }
}

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
pub fn greedy_set_cover_0<K, T>(sets: &HashMap<K, Vec<T>>) -> Vec<K>
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
    cover.into_iter().collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn make_universe<K, T>(sets: &HashMap<K, Vec<T>>) -> HashSet<T>
    where
        T: Clone + Hash + Eq,
    {
        sets.values().flatten().cloned().collect()
    }
    #[test]
    fn test_greedy_set_cover() {
        let mut sets = HashMap::new();
        sets.insert("A".to_string(), vec![1, 2, 3]);
        sets.insert("B".to_string(), vec![1, 2]);
        sets.insert("C".to_string(), vec![2]);

        let result_0 = greedy_set_cover(&sets, 0);
        let result_1 = greedy_set_cover(&sets, 1);
        let direct_0 = greedy_set_cover_0(&sets);
        let direct_1 = greedy_set_cover_1(&sets);

        assert_eq!(result_0, direct_0);
        assert_eq!(result_1, direct_1);
    }

    #[test]
    fn test_basic_case() {
        let mut sets = HashMap::new();
        sets.insert("A".to_string(), vec![1, 2, 3]);
        sets.insert("B".to_string(), vec![1, 2]);
        sets.insert("C".to_string(), vec![2]);

        // Test both versions
        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);
        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<String>,
            sets: &HashMap<String, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<String, Vec<i32>> = cover
                .iter()
                .map(|key| (key.clone(), sets.get(key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_with_empty_set() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1, 2, 3]);
        sets.insert(2, vec![]);
        sets.insert(3, vec![3, 4, 5]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);
        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_all_sets_needed() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1]);
        sets.insert(2, vec![2]);
        sets.insert(3, vec![3]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        assert_eq!(sets.len(), set_cover_0.len());
        assert_eq!(sets.len(), set_cover_1.len());

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_one_set_covers_all() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1, 2, 3, 4, 5]);
        sets.insert(2, vec![1, 2]);
        sets.insert(3, vec![3, 4]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        assert_eq!(set_cover_0.len(), 1);
        assert_eq!(set_cover_1.len(), 1);

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_overlapping_sets() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1, 2, 3]);
        sets.insert(2, vec![3, 4, 5]);
        sets.insert(3, vec![5, 6, 7]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        // The greedy algorithm might pick 2 or 3 sets, but the universe must be covered
        assert!(set_cover_0.len() >= 2 && set_cover_0.len() <= 3);
        assert!(set_cover_1.len() >= 2 && set_cover_1.len() <= 3);

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_large_numbers() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1_000_000, 2_000_000, 3_000_000]);
        sets.insert(2, vec![4_000_000, 5_000_000]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);
        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_duplicate_sets() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1, 2, 3]);
        sets.insert(2, vec![1, 2, 3]);
        sets.insert(3, vec![4, 5, 6]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        assert!(set_cover_0.len() < sets.len());
        assert!(set_cover_1.len() < sets.len());

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_one_element_per_set() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1]);
        sets.insert(2, vec![2]);
        sets.insert(3, vec![3]);
        sets.insert(4, vec![4]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        assert_eq!(sets.len(), set_cover_0.len());
        assert_eq!(sets.len(), set_cover_1.len());

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_nested_sets() {
        let mut sets = HashMap::new();
        sets.insert(1, vec![1, 2, 3, 4, 5]);
        sets.insert(2, vec![1, 2, 3]);
        sets.insert(3, vec![1, 2]);

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        assert_eq!(set_cover_0.len(), 1);
        assert_eq!(set_cover_1.len(), 1);

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_large_number_of_small_sets() {
        let mut sets = HashMap::new();
        for i in 0..50 {
            sets.insert(i, vec![i, i + 1]);
        }

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        assert!(set_cover_0.len() < sets.len());
        assert!(set_cover_1.len() < sets.len());

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }

    #[test]
    fn test_complex_deterministic_cases() {
        // Case 1: A clear greedy choice path
        let mut sets = HashMap::new();
        sets.insert(1, vec![1, 2, 3, 4, 5, 6]); // S1 (Best initial choice)
        sets.insert(2, vec![1, 2, 7]); // S2
        sets.insert(3, vec![3, 4, 8]); // S3
        sets.insert(4, vec![5, 6, 9]); // S4
        sets.insert(5, vec![7, 8, 9, 10]); // S5 (Best second choice)

        let set_cover_0 = greedy_set_cover_0(&sets);
        let set_cover_1 = greedy_set_cover_1(&sets);

        // The optimal greedy cover is 2 sets: {1,2,3,4,5,6} and {7,8,9,10}
        assert_eq!(set_cover_0.len(), 2);
        assert_eq!(set_cover_1.len(), 2);

        let universe = make_universe(&sets);

        // Helper function to check coverage
        fn check_coverage(
            cover: &HashSet<i32>,
            sets: &HashMap<i32, Vec<i32>>,
            universe: &HashSet<i32>,
        ) {
            let covered_sets: HashMap<i32, Vec<i32>> = cover
                .iter()
                .map(|&key| (key, sets.get(&key).unwrap().clone()))
                .collect();
            let covered_universe = make_universe(&covered_sets);
            assert_eq!(universe, &covered_universe);
        }

        check_coverage(&set_cover_0, &sets, &universe);
        check_coverage(&set_cover_1, &sets, &universe);
    }
}
