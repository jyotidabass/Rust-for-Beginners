// Example 5: Control Flow
// Run with: rustc 05_control_flow.rs && ./05_control_flow

fn main() {
    // If/Else
    let number = 7;
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }
    
    // Loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // For loop
    for number in 1..4 {
        println!("For loop number: {}", number);
    }
}
