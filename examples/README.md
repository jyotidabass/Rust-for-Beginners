# Rust Examples

This directory contains practical code examples demonstrating the concepts covered in the main README.

## Running the Examples

### Option 1: Compile and Run Individual Files

```bash
rustc 01_hello_world.rs
./01_hello_world
```

### Option 2: Run Directly with rustc

```bash
rustc 01_hello_world.rs && ./01_hello_world
```

### Option 3: Use Cargo Script (if you have a Cargo project)

If you initialize this as a Cargo project, you can place these in the `examples/` directory and run with:

```bash
cargo run --example 01_hello_world
```

## Examples List

1. **01_hello_world.rs** - Basic Hello World program
2. **02_variables.rs** - Variables, mutability, constants, and shadowing
3. **03_data_types.rs** - Scalar and compound data types
4. **04_functions.rs** - Function definitions and return values
5. **05_control_flow.rs** - If/else, loops (loop, while, for)
6. **06_ownership.rs** - Ownership, borrowing, and references
7. **07_structs.rs** - Structs and methods
8. **08_enums.rs** - Enums and pattern matching
9. **09_collections.rs** - Vectors, strings, and hash maps

## Tips

- Read the comments in each file for more context
- Experiment by modifying the code and running it again
- Compare the output with what's described in the main README
- Try combining concepts from different examples

Happy learning! 🦀
