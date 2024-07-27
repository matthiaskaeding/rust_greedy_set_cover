mod simulate;

use crate::simulate::{draw_set_collection, SetCollection};

fn main() {
    let n_sets: u32 = 10;
    let n_elements: usize = 50;
    let max_elements: u32 = 100;
    let setcollection: SetCollection = draw_set_collection(n_sets, n_elements, max_elements);

    for v in setcollection[0].iter() {
        println!("{v}")
    }
    println!("\n");
    for v in setcollection[1].iter() {
        println!("{v}")
    }
}
