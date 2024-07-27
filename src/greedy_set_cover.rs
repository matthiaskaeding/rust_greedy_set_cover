use std::collections::HashSet;
pub type Set = HashSet<u32>;
pub type SetVec = Vec<Set>;

fn make_universe(sets: &SetVec) -> Set {
    let mut universe: Set = Set::new();
    for set in sets.iter() {
        for &element in set.iter() {
            universe.insert(element);
        }
    }
    return universe;
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

        for i in 0..n_sets {
            let intersection: Set = uncovered_elements.intersection(&sets[i]).cloned().collect();
            let len_intersection = intersection.len();
            if len_intersection > len_biggest {
                i_biggest = i;
                len_biggest = len_intersection;
            }
        }
        let biggest_set: Set = sets[i_biggest].clone();
        uncovered_elements.retain(|&x| !&biggest_set.contains(&x));

        covering_sets.push(biggest_set);
        // Remove covered elements from "uncovered"
    }

    covering_sets
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
}
