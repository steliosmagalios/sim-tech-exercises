mod generator;

use crate::generator::Generator;

const TESTING_SEED: u64 = 0;

fn main() {
    println!("Linear Congruential Generator (seed = {})", TESTING_SEED);

    println!("\n{:=^60}", "Test 01");
    println!("{:=^60}\n", "Display the first 100 random numbers");
    test_1();

    println!("\n{:=^60}", "Test 02");
    println!(
        "{:=^60}\n",
        "Mean and Standard Deviation of first 100 numbers"
    );
    test_2();

    println!("\n{:=^60}", "Test 03");
    println!(
        "{:=^60}\n",
        "Mean and Standard Deviation of first 1,000,000 numbers"
    );
    test_3();

    println!("\n{:=^60}", "Test 04");
    println!("{:=^60}\n", "Runs test for runs 1 through 10");
    test_4();

    println!("\n{:=^60}", "Test 05");
    println!("{:=^60}\n", "Regions test for 10 regions");
    test_5();
}

/// Test 1: Display the first 100 random numbers
fn test_1() {
    let mut prng: Generator = Generator::new_with_seed(TESTING_SEED);

    for i in 0..100 {
        println!("{:3}: {:.17}", i + 1, prng.next_random())
    }
}

/// Test 2: Mean and Standard Deviation of first 100 numbers
fn test_2() {
    let mut prng: Generator = Generator::new_with_seed(TESTING_SEED);

    let mut mean: f64 = 0.0;
    let mut ssq: f64 = 0.0;
    let mut sqs: f64 = 0.0;

    for _ in 0..100 {
        let curr_num: f64 = prng.next_random();

        // Calculate mean
        mean += curr_num / 100.0;

        // Calculate variables needed for standard deviation
        // Thanks to this math.stackexchange answer fot the formula
        // https://math.stackexchange.com/q/3045127
        ssq += curr_num.powf(2.0);
        sqs += curr_num;
    }

    let standard_deviation: f64 = (ssq / 100.0 - (sqs / 100.0).powf(2.0)).sqrt();

    // Printing
    println!("              Mean: {:.6}", mean);
    println!("Standard Deviation: {:.6}", standard_deviation);
}

/// Test 3: Mean and Standard Deviation of first 1,000,000 numbers
fn test_3() {
    let mut prng: Generator = Generator::new_with_seed(TESTING_SEED);

    let mut mean: f64 = 0.0;
    let mut ssq: f64 = 0.0;
    let mut sqs: f64 = 0.0;

    for _ in 0..1_000_000 {
        let curr_num: f64 = prng.next_random();

        // Calculate mean
        mean += curr_num / 1_000_000.0;

        // Calculate variables needed for standard deviation
        // Thanks to this math.stackexchange answer fot the formula
        // https://math.stackexchange.com/q/3045127
        ssq += curr_num.powf(2.0);
        sqs += curr_num;
    }

    let standard_deviation: f64 = (ssq / 1_000_000.0 - (sqs / 1_000_000.0).powf(2.0)).sqrt();

    // Printing
    println!("              Mean: {:.6}", mean);
    println!("Standard Deviation: {:.6}", standard_deviation);
}

/// Test 4: Runs test for runs 1 through 10
fn test_4() {
    let mut prng: Generator = Generator::new_with_seed(TESTING_SEED);
    let mut runs: [u64; 10] = [0; 10];

    let mut run_above: bool = prng.next_random() >= 0.5;
    let mut current_run: u64 = 1;

    for _ in 1..1_000_000 {
        match (run_above, prng.next_random() >= 0.5) {
            // Run continues
            (true, true) => current_run += 1,
            (false, false) => current_run += 1,

            // Run ended
            _ => {
                // Store the run that ended
                runs[(current_run - 1).min(9) as usize] += 1;

                // Start new run
                current_run = 1;
                run_above = !run_above;
            }
        }
    }

    // Printing
    println!("{:^60}", format!("{:^15} {:^15}", "Run", "Probability"));
    for (i, occurencies) in runs.iter().enumerate() {
        println!(
            "{:^60}",
            format!(
                "{:^15} {:^15.6}",
                format!("[{:.1}, {:.1})", i, (i + 1) as f64 / 10.0),
                *occurencies as f64 / 1_000_000.0
            )
        );
    }
    // println!("{:?}", runs);
}

/// Test 5: Regions test for 10 regions
fn test_5() {
    let mut prng: Generator = Generator::new_with_seed(TESTING_SEED);
    let mut regions: [u64; 10] = [0; 10];

    for _ in 0..1_000_000 {
        // Get the region by multiplying and the taking only the
        // decimal part. For 10 regions, this sould go from 0 to 9.
        let region: usize = (prng.next_random() * 10.0) as usize;

        // Increase the region count
        regions[region] += 1;
    }

    // Printing
    println!("{:^60}", format!("{:^15} {:^15}", "Region", "Probability"));
    for (i, occurencies) in regions.iter().enumerate() {
        println!(
            "{:^60}",
            format!(
                "{:^15} {:^15.6}",
                format!("[{:.1}, {:.1})", i, (i + 1) as f64 / 10.0),
                *occurencies as f64 / 1_000_000.0
            )
        );
    }
}
