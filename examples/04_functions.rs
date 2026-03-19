// Example 4: Functions
// Run with: rustc 04_functions.rs && ./04_functions

fn main() {
    greet();
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let (sum, product) = calculate(4, 5);
    println!("Sum: {}, Product: {}", sum, product);
}

fn greet() {
    println!("Hello from a function!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
