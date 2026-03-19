// Example 2: Variables and Mutability
// Run with: rustc 02_variables.rs && ./02_variables

fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Mutable variable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("Now the value of y is: {}", y);
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
    
    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);
}
