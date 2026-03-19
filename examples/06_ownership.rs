// Example 6: Ownership and Borrowing
// Run with: rustc 06_ownership.rs && ./06_ownership

fn main() {
    // Ownership transfer
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
    
    // References and borrowing
    let s3 = String::from("hello");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);
    
    // Mutable references
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("{}", s4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
