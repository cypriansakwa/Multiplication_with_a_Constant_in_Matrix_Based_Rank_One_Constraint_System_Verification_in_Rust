# Multiplication with a Constant in Matrix-Based Rank-One Constraint System Verification in Rust

This project implements a simple Rank-One Constraint System (R1CS) verification in Rust using matrix representations. The program randomly generates values, computes a constraint equation, and verifies the result using matrix multiplication.

## Features
- Uses `ndarray` for matrix operations.
- Implements R1CS verification with a predefined constraint.
- Generates random values using `rand` crate.
- Asserts correctness using matrix multiplication.

## Installation
Ensure you have Rust installed. If not, install it using [Rustup](https://rustup.rs/).

Clone the repository:
```sh
clone git https://github.com/cypriansakwa/Multiplication_with_a_Constant_in_Matrix_Based_Rank_One_Constraint_System_Verification_in_Rust.git
cd Multiplication_with_a_Constant_in_Matrix_Based_Rank_One_Constraint_System_Verification_in_Rust
```

## Usage
Run the program with:
```csharp
cargo run
```

## Dependencies
This project uses the following dependencies:
- `ndarray` for matrix operations
- `rand` for random number generation

Ensure these dependencies are included in `Cargo.toml`:

```markdown
[dependencies]
rand = "0.9.0"
ndarray = "0.16.1"
```

## Code Overview
The core logic includes:
- Generating random values `x` and `y`
- Computing `z = 2 * x * x + y`
- Defining constraint matrices
- Verifying the constraint with matrix operations

## Example Output
Constraint satisfied with $x = 917, y = 877, z = 1682655$
