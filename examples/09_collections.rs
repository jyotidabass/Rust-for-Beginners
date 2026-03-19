// Example 9: Collections (Vectors, Strings, HashMaps)
// Run with: rustc 09_collections.rs && ./09_collections

use std::collections::HashMap;

fn main() {
    // Vectors
    let v = vec![1, 2, 3, 4, 5];
    println!("The third element is {}", v[2]);
    
    for i in &v {
        println!("Vector element: {}", i);
    }
    
    // Strings
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
    
    // HashMaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
