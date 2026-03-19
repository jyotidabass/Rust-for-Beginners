// Example 3: Data Types
// Run with: rustc 03_data_types.rs && ./03_data_types

fn main() {
    // Scalar types
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let is_rust_fun: bool = true;
    let letter: char = 'R';
    let emoji: char = '🦀';
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", is_rust_fun);
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    
    // Compound types
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("Tuple first element: {}", tuple.0);
    
    let array = [1, 2, 3, 4, 5];
    println!("Array first element: {}", array[0]);
}
