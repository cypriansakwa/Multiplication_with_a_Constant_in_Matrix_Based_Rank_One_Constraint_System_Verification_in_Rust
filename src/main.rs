use ndarray::{array, Array1};
use rand::{rng, Rng}; // Use `rng` instead of `thread_rng`

// Function to generate random values for x and y
fn generate_values() -> (i32, i32, i32) {
    let mut rng = rng(); // Use `rng()` instead of `thread_rng()`
    let x = rng.random_range(1..=1000); // Use `random_range()`
    let y = rng.random_range(1..=1000);
    let z = 2 * x * x + y; // Compute z based on the equation

    (x, y, z)
}

// Function to verify R1CS constraints
fn verify_r1cs(x: i32, y: i32, z: i32) {
    // Define the constraint matrices
    let l = array![[0, 0, 2, 0]];
    let r = array![[0, 0, 1, 0]];
    let o = array![[0, 1, 0, -1]];

    // Define the witness vector
    let a: Array1<i32> = array![1, z, x, y];

    // Compute matrix operations
    let lhs = o.dot(&a);
    let rhs = (l.dot(&a) * r.dot(&a))[0]; // Scalar multiplication

    // Check the constraint
    assert_eq!(lhs[0], rhs, "Constraint failed!");

    println!("Constraint satisfied with x = {}, y = {}, z = {}", x, y, z);
}

fn main() {
    let (x, y, z) = generate_values();
    verify_r1cs(x, y, z);
}
