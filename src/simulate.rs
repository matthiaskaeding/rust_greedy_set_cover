use rand::Rng;
use std::{collections::HashSet, usize};
pub type Set = HashSet<u32>;
pub type SetCollection = Vec<Set>;

/// Generates a random integer between 0 and `max` (inclusive).
///
/// This function uses the thread-local random number generator.
///
/// # Arguments
///
/// * `max` - The upper bound for the random number generation (inclusive).
///
/// # Returns
///
/// A random `u32` between 0 and `max`, inclusive.
///
/// # Examples
///
/// ```
/// let random_number = draw_single_int(10);
/// assert!(random_number <= 10);
/// ```
fn draw_single_int(max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=max)
}

/// Generates a collection of sets, each containing random elements.
///
/// This function creates a specified number of sets and populates them
/// with random elements. The total number of elements across all sets
/// is controlled by the `n_elements` parameter.
///
/// # Arguments
///
/// * `n_sets` - The number of sets to create.
/// * `n_elements` - The total number of elements to distribute across all sets.
/// * `max_elements` - The maximum value for any element in the sets.
///
/// # Returns
///
/// A `SetCollection` (Vec<HashSet<u32>>) containing the generated sets.
///
/// # Examples
///
/// ```
/// let collection = draw_set_collection(3, 10, 100);
/// assert_eq!(collection.len(), 3);
/// ```
///
/// # Note
///
/// The distribution of elements across sets is random. Some sets may
/// end up with more elements than others, and some may be empty.
pub fn draw_set_collection(n_sets: u32, n_elements: usize, max_elements: u32) -> SetCollection {
    let mut sets: SetCollection = vec![Set::new(); n_sets as usize];
    for _ in 0..n_elements {
        let element = draw_single_int(max_elements);
        let i_set = draw_single_int(n_sets - 1) as usize;
        sets[i_set].insert(element);
    }

    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_single_int() {
        let max = 10;
        let mut seen_max = false;
        for _ in 0..1000 {
            let result = draw_single_int(max);
            assert!(
                result <= max,
                "Generated number should be less than or equal to max"
            );
            if result == max {
                seen_max = true;
            }
        }
        assert!(
            seen_max,
            "Max value should be generated at least once in 1000 tries"
        );
    }

    #[test]
    fn test_draw_set_collection_size() {
        let n_sets = 5;
        let n_elements = 20;
        let max_elements = 100;
        let result = draw_set_collection(n_sets, n_elements, max_elements);
        assert_eq!(
            result.len(),
            n_sets as usize,
            "Number of sets should match n_sets"
        );
    }

    #[test]
    fn test_draw_set_collection_element_range() {
        let n_sets = 3;
        let n_elements = 50;
        let max_elements = 30;
        let result = draw_set_collection(n_sets, n_elements, max_elements);
        for set in result {
            for &element in set.iter() {
                assert!(
                    element <= max_elements,
                    "Elements should be less than or equal to max_elements"
                );
            }
        }
    }

    #[test]
    fn test_draw_set_collection_total_elements() {
        let n_sets = 4;
        let n_elements = 100;
        let max_elements = 200;
        let result = draw_set_collection(n_sets, n_elements, max_elements);
        let mut n_counted_elements: u32 = 0;
        for set in &result {
            for _ in set.iter() {
                n_counted_elements += 1
            }
        }
        assert!(
            n_counted_elements > 0,
            "There should be at least one counted element"
        );
        assert!(
            n_counted_elements as usize <= n_elements,
            "There should be less or equal elements than argument n_elements"
        );
        //let result_len: *usize = result.len();
        assert_eq!(result.len(), n_sets as usize);
    }
}
