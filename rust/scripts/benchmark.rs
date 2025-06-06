use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Use the library part of your crate to import the function
use set_cover::greedy_set_cover::greedy_set_cover_0;
use set_cover::greedy_set_cover::greedy_set_cover_1;

#[derive(Debug, Deserialize)]
struct Record {
    set: i32,
    element: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Starting Independent Benchmark ---");

    // 1. Read and Process the CSV data
    println!("Reading and processing data.csv...");
    let mut sets_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rdr = csv::Reader::from_path("data.csv")?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        sets_map.entry(record.set).or_default().push(record.element);
    }

    println!("Finished processing. Found {} unique sets.", sets_map.len());

    // 2. Run Algorithm 0 and Record Time
    println!("\nRunning greedy_set_cover_0 algorithm...");
    let start_time = Instant::now();
    let set_cover_0 = greedy_set_cover_0(&sets_map);
    let duration_0 = start_time.elapsed();

    // 3. Run Algorithm 1 and Record Time
    println!("\nRunning greedy_set_cover_1 algorithm...");
    let start_time = Instant::now();
    let set_cover_1 = greedy_set_cover_1(&sets_map);
    let duration_1 = start_time.elapsed();

    // 4. Print the Results
    println!("\n--- Benchmark Results ---");
    println!("Original number of sets: {}", sets_map.len());

    println!("\nAlgorithm 0 (greedy_set_cover_0):");
    println!("Time taken: {:?}", duration_0);
    println!("Number of sets in cover: {}", set_cover_0.len());

    println!("\nAlgorithm 1 (greedy_set_cover_1):");
    println!("Time taken: {:?}", duration_1);
    println!("Number of sets in cover: {}", set_cover_1.len());

    Ok(())
}
