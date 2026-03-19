// Example 8: Enums and Pattern Matching
// Run with: rustc 08_enums.rs && ./08_enums

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value: {} cents", value_in_cents(coin));
    
    // Using Option
    let some_number = Some(5);
    match some_number {
        Some(i) => println!("Got a number: {}", i),
        None => println!("No number"),
    }
}
