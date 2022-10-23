mod generator;

use crate::generator::Generator;

fn main() {
    println!("Linear Congruential Generator (seed = 0)");

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
    let mut prng = Generator::new_with_seed(0);

    for i in 0..100 {
        println!("{:3}: {:.20}", i + 1, prng.next_random())
    }
}

/// Test 2: Mean and Standard Deviation of first 100 numbers
fn test_2() {
    let mut prng = Generator::new_with_seed(0);
}

/// Test 3: Mean and Standard Deviation of first 1,000,000 numbers
fn test_3() {
    let mut prng = Generator::new_with_seed(0);
}

/// Test 4: Runs test for runs 1 through 10
fn test_4() {
    let mut prng = Generator::new_with_seed(0);
}

/// Test 5: Regions test for 10 regions
fn test_5() {
    let mut prng = Generator::new_with_seed(0);
}
