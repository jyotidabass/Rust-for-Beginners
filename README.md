# 🦀 Rust for Beginners

A hands-on collection of Rust examples designed for **absolute beginners** — no prior Rust experience needed. Each file focuses on one core concept, is fully commented, and can be run independently.

---

## 📋 Table of Contents

1. [What is Rust?](#what-is-rust)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [How to Run the Examples](#how-to-run-the-examples)
5. [Examples Overview](#examples-overview)
   - [01 – Hello World](#01--hello-world)
   - [02 – Variables](#02--variables)
   - [03 – Data Types](#03--data-types)
   - [04 – Functions](#04--functions)
   - [05 – Control Flow](#05--control-flow)
   - [06 – Ownership & Borrowing](#06--ownership--borrowing)
   - [07 – Structs](#07--structs)
   - [08 – Enums & Pattern Matching](#08--enums--pattern-matching)
   - [09 – Collections](#09--collections)
6. [Recommended Learning Order](#recommended-learning-order)
7. [Common Errors & Fixes](#common-errors--fixes)
8. [Tips for Learning Rust](#tips-for-learning-rust)
9. [Next Steps](#next-steps)
10. [Blog Post](#blog-post)

---

## What is Rust?

Rust is a **systems programming language** that focuses on three goals:

- ⚡ **Performance** — as fast as C and C++
- 🔒 **Safety** — no null pointer crashes, no memory bugs
- 🔧 **Concurrency** — fearless multi-threading

Rust is used to build operating systems, web servers, game engines, command-line tools, and more. It's consistently voted the **most loved programming language** in developer surveys.

---

## Prerequisites

You don't need to know any Rust. Basic familiarity with programming concepts (variables, loops, functions) in any language is helpful but not required.

You'll need:
- A computer running **Windows**, **macOS**, or **Linux**
- A terminal / command prompt
- An internet connection (just for the one-time installation)

---

## Installation

### Step 1 — Install Rust

The official and recommended way is via `rustup`, the Rust toolchain installer.

**On macOS / Linux**, open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Windows**, download and run the installer from:
👉 https://www.rust-lang.org/tools/install

Follow the on-screen prompts and choose the default installation when asked.

### Step 2 — Restart your terminal

After installation, close and reopen your terminal so the new commands are available.

### Step 3 — Verify installation

```bash
rustc --version
cargo --version
```

You should see output like:
```
rustc 1.78.0 (9b00956e5 2024-04-29)
cargo 1.78.0 (74ea67456 2024-04-29)
```

If you see version numbers, you're all set! 🎉

> 💡 **What's the difference between `rustc` and `cargo`?**
> - `rustc` is the Rust **compiler** — it turns `.rs` files into programs.
> - `cargo` is Rust's **build tool and package manager** — it handles larger projects. We'll mostly use `rustc` for these simple examples.

---

## How to Run the Examples

### Clone or download this repository

```bash
git clone https://github.com/your-username/Rust-for-Beginners.git
cd Rust-for-Beginners/examples
```

Or download the ZIP from GitHub and extract it.

### Running a single example

Each example is a standalone `.rs` file. To compile and run any of them:

```bash
# Step 1: Compile the file
rustc 01_hello_world.rs

# Step 2: Run the compiled program
./01_hello_world          # macOS / Linux
01_hello_world.exe        # Windows
```

You can do both steps in one line:

```bash
rustc 01_hello_world.rs && ./01_hello_world
```

### Quick reference — run all examples

```bash
# From the examples/ directory:
for file in *.rs; do
  name="${file%.rs}"
  echo "=== Running $file ==="
  rustc "$file" -o "$name" && ./"$name"
  echo ""
done
```

---

## Examples Overview

---

### 01 – Hello World

**File:** `examples/01_hello_world.rs`

The classic starting point. This program prints text to your screen.

**What you'll learn:**
- The `fn main()` entry point — every Rust program starts here
- The `println!()` macro for printing output (note the `!` — that means it's a macro, not a regular function)

**Code:**
```rust
fn main() {
    println!("Hello, World!");
    println!("Welcome to Rust programming!");
}
```

**Expected output:**
```
Hello, World!
Welcome to Rust programming!
```

**Run it:**
```bash
rustc 01_hello_world.rs && ./01_hello_world
```

---

### 02 – Variables

**File:** `examples/02_variables.rs`

In Rust, variables behave differently from most languages. By default they are **immutable** (cannot be changed). You must explicitly opt into mutability.

**What you'll learn:**
- `let` — declares an immutable variable
- `let mut` — declares a mutable variable (can be changed)
- `const` — a true constant, known at compile time, MUST have a type annotation
- **Shadowing** — re-declaring a variable with the same name to transform it

**Code:**
```rust
fn main() {
    // Immutable variable — cannot be changed after this
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable — can be reassigned
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("Now the value of y is: {}", y);

    // Constant — must have a type, cannot be `mut`
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    // Shadowing — z is redeclared each time (not mutation)
    let z = 5;
    let z = z + 1;   // z is now 6
    let z = z * 2;   // z is now 12
    println!("The value of z is: {}", z);
}
```

**Expected output:**
```
The value of x is: 5
The value of y is: 5
Now the value of y is: 6
The maximum points are: 100000
The value of z is: 12
```

> ⚠️ **Gotcha:** If you try to do `x = 10;` after `let x = 5;`, Rust will refuse to compile and show an error. This is intentional — Rust protects you from accidental mutation!

**Run it:**
```bash
rustc 02_variables.rs && ./02_variables
```

---

### 03 – Data Types

**File:** `examples/03_data_types.rs`

Rust is a **statically typed** language — every value has a type that's known at compile time. Rust can often infer the type, but you can always be explicit.

**What you'll learn:**
- **Scalar types** — single values: integers, floats, booleans, characters
- **Compound types** — multiple values: tuples and arrays

**Key types at a glance:**

| Type | Example | Notes |
|------|---------|-------|
| `i32` | `42` | Signed 32-bit integer (most common) |
| `u32` | `100` | Unsigned 32-bit integer (no negatives) |
| `f64` | `3.14` | 64-bit float (most common float) |
| `bool` | `true` | `true` or `false` only |
| `char` | `'R'` | Single character, uses single quotes |

**Code:**
```rust
fn main() {
    // Scalar types
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let is_rust_fun: bool = true;
    let letter: char = 'R';
    let emoji: char = '🦀';       // chars support Unicode!

    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", is_rust_fun);
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);

    // Tuple — fixed-size, can mix types, access with .0 .1 .2 ...
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("Tuple first element: {}", tuple.0);

    // Array — fixed-size, same type for all elements, access with [index]
    let array = [1, 2, 3, 4, 5];
    println!("Array first element: {}", array[0]);
}
```

**Expected output:**
```
Integer: 42
Float: 3.14159
Boolean: true
Letter: R
Emoji: 🦀
Tuple first element: 500
Array first element: 1
```

> 💡 **Tuple vs Array:** Tuples can hold mixed types `(42, "hello", true)`. Arrays must hold one type `[1, 2, 3, 4]`.

**Run it:**
```bash
rustc 03_data_types.rs && ./03_data_types
```

---

### 04 – Functions

**File:** `examples/04_functions.rs`

Functions are building blocks of every Rust program. In Rust, the **last expression** in a function (without a semicolon) is automatically the return value.

**What you'll learn:**
- Defining functions with `fn`
- Parameters and their required type annotations
- Return types with `->`
- Returning multiple values using a tuple
- The implicit return (expression without `;`)

**Code:**
```rust
fn main() {
    greet();                                  // call a simple function

    let result = add(5, 3);
    println!("5 + 3 = {}", result);           // prints: 5 + 3 = 8

    let (sum, product) = calculate(4, 5);     // destructure tuple return
    println!("Sum: {}, Product: {}", sum, product);
}

// No parameters, no return value
fn greet() {
    println!("Hello from a function!");
}

// Takes two i32s, returns one i32
// Note: `a + b` has NO semicolon — it's the return value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Returns a tuple of two i32s
fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
```

**Expected output:**
```
Hello from a function!
5 + 3 = 8
Sum: 9, Product: 20
```

> ⚠️ **Semicolons matter!** In Rust, `a + b` (no semicolon) is a **return expression**. `a + b;` (with semicolon) is a **statement** that returns nothing. This is a common source of beginner confusion.

**Run it:**
```bash
rustc 04_functions.rs && ./04_functions
```

---

### 05 – Control Flow

**File:** `examples/05_control_flow.rs`

Control flow lets your program make decisions and repeat actions. Rust has three loop types: `loop` (infinite), `while` (condition-based), and `for` (iterator-based).

**What you'll learn:**
- `if` / `else if` / `else` conditions
- `loop` — runs forever until you `break` (and can return a value!)
- `while` — runs while a condition is true
- `for` — iterates over a range or collection
- Ranges with `..` (exclusive) and `..=` (inclusive)

**Code:**
```rust
fn main() {
    // If / else if / else
    let number = 7;
    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }

    // loop — infinite loop, break returns a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;   // returns 20
        }
    };
    println!("Loop result: {}", result);

    // while — countdown
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for — iterates 1, 2, 3 (4 is excluded with ..)
    for number in 1..4 {
        println!("For loop number: {}", number);
    }
}
```

**Expected output:**
```
Number is greater than 5
Loop result: 20
3!
2!
1!
LIFTOFF!!!
For loop number: 1
For loop number: 2
For loop number: 3
```

> 💡 **Range syntax:**
> - `1..4` → 1, 2, 3 (excludes 4)
> - `1..=4` → 1, 2, 3, 4 (includes 4)

**Run it:**
```bash
rustc 05_control_flow.rs && ./05_control_flow
```

---

### 06 – Ownership & Borrowing

**File:** `examples/06_ownership.rs`

This is Rust's **most unique and most important concept**. Ownership is what makes Rust memory-safe without a garbage collector. Take your time with this one.

**The 3 Rules of Ownership:**
1. Every value has exactly **one owner**
2. When the owner goes out of scope, the value is **dropped** (memory freed)
3. There can only be **one owner at a time**

**What you'll learn:**
- Ownership and why moving values matters
- `.clone()` to make a deep copy when you need two owners
- **References** (`&`) — borrowing a value without taking ownership
- **Mutable references** (`&mut`) — borrowing with permission to modify

**Code:**
```rust
fn main() {
    // s1 is moved into s2 — but we use .clone() to keep both
    let s1 = String::from("hello");
    let s2 = s1.clone();             // deep copy — s1 is still valid
    println!("s1: {}, s2: {}", s1, s2);

    // Pass a reference (&) so the function borrows — we keep ownership
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // &s3 = borrow, not move
    println!("The length of '{}' is {}.", s3, len);   // s3 still valid!

    // Mutable reference — borrow with permission to change
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("{}", s4);   // prints: hello, world
}

fn calculate_length(s: &String) -> usize {
    s.len()   // borrows s, doesn't own it
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Expected output:**
```
s1: hello, s2: hello
The length of 'hello' is 5.
hello, world
```

> ⚠️ **Without `.clone()`:** If you wrote `let s2 = s1;`, then `s1` would be **moved** into `s2`, and `s1` would no longer be valid. Rust enforces this at compile time.

> 💡 **The Golden Rules of References:**
> - You can have **many** immutable references (`&T`) at once
> - OR exactly **one** mutable reference (`&mut T`)
> - But **never both at the same time**

**Run it:**
```bash
rustc 06_ownership.rs && ./06_ownership
```

---

### 07 – Structs

**File:** `examples/07_structs.rs`

Structs let you group related data together into a named type. You can add behavior to structs using `impl` blocks (similar to methods in OOP languages).

**What you'll learn:**
- Defining a `struct` with named fields
- Creating instances of a struct
- Adding methods with `impl`
- `&self` — a method that reads the struct
- `&other` — passing another struct as a parameter

**Code:**
```rust
// Define the struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Add methods to the struct
impl Rectangle {
    // &self gives read-only access to this Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Compare this rectangle to another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
```

**Expected output:**
```
Area of rect1: 1500
Can rect1 hold rect2? true
```

> 💡 **`self` vs `Self`:** Inside `impl`, lowercase `self` refers to the current instance (like `this` in JavaScript/Java). Uppercase `Self` refers to the type itself.

**Run it:**
```bash
rustc 07_structs.rs && ./07_structs
```

---

### 08 – Enums & Pattern Matching

**File:** `examples/08_enums.rs`

Enums let you define a type that can be one of several **named variants**. Combined with `match`, they're incredibly powerful — Rust's `match` must handle every possible case (it's exhaustive).

**What you'll learn:**
- Defining an `enum` with variants
- `match` — like a `switch` statement, but exhaustive and more powerful
- `Option<T>` — Rust's way of handling "maybe a value, maybe nothing" (no null!)
- `Some(value)` and `None`

**Code:**
```rust
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
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value: {} cents", value_in_cents(coin));

    // Option<T> — either Some(value) or None
    let some_number = Some(5);
    match some_number {
        Some(i) => println!("Got a number: {}", i),
        None    => println!("No number"),
    }
}
```

**Expected output:**
```
Lucky penny!
Value: 1 cents
Got a number: 5
```

> 💡 **Why `Option` instead of null?** Languages with `null` let you accidentally use a null value and crash at runtime. Rust forces you to explicitly handle `None` at compile time — no more null pointer exceptions!

**Run it:**
```bash
rustc 08_enums.rs && ./08_enums
```

---

### 09 – Collections

**File:** `examples/09_collections.rs`

Collections store multiple values. Unlike arrays and tuples, collections live on the **heap** and can grow or shrink at runtime.

**What you'll learn:**
- `Vec<T>` (Vector) — a growable list
- `String` — a growable, heap-allocated string
- `HashMap<K, V>` — a key-value store (dictionary)
- Iterating over collections with `for` and `&`

**Code:**
```rust
use std::collections::HashMap;   // must import HashMap

fn main() {
    // --- Vectors ---
    let v = vec![1, 2, 3, 4, 5];    // vec! macro creates a Vector
    println!("The third element is {}", v[2]);   // index starts at 0

    for i in &v {                    // &v borrows the vector
        println!("Vector element: {}", i);
    }

    // --- Strings ---
    let mut s = String::from("Hello");
    s.push_str(", world!");          // append to the string
    println!("{}", s);

    // --- HashMaps ---
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),   10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

**Expected output:**
```
The third element is 3
Vector element: 1
Vector element: 2
Vector element: 3
Vector element: 4
Vector element: 5
Hello, world!
Blue: 10
Yellow: 50
```

> ⚠️ **HashMap output order** may vary — HashMaps do not guarantee order. Your Blue/Yellow lines might appear in either order.

> 💡 **`&str` vs `String`:**
> - `&str` — a string **slice**, fixed, usually used for string literals like `"hello"`
> - `String` — a heap-allocated, growable string you can modify

**Run it:**
```bash
rustc 09_collections.rs && ./09_collections
```

---

## Recommended Learning Order

Work through the examples in order — each one builds on the previous:

```
01 Hello World  →  02 Variables  →  03 Data Types  →  04 Functions
→  05 Control Flow  →  06 Ownership  →  07 Structs  →  08 Enums  →  09 Collections
```

**Spend extra time on Example 06 (Ownership).** It's Rust's most unique concept and the foundation of everything that makes Rust special. If it feels confusing at first, that's completely normal — re-read it, experiment with the code, and let it sink in.

---

## Common Errors & Fixes

Here are the most frequent errors beginners encounter, with solutions:

---

**Error: `cannot assign twice to immutable variable`**

```
error[E0384]: cannot assign twice to immutable variable `x`
```
**Fix:** Change `let x = 5;` to `let mut x = 5;`

---

**Error: `use of moved value`**

```
error[E0382]: use of moved value: `s1`
```
**Fix:** Use `s1.clone()` to make a copy, or pass a reference `&s1` instead.

---

**Error: `rustc` command not found**

```
rustc: command not found
```
**Fix:** Rust isn't installed, or your terminal needs to be restarted after installation. Run `source ~/.cargo/env` (macOS/Linux) or restart your terminal.

---

**Error: Permission denied running the compiled file (macOS/Linux)**

```
bash: ./01_hello_world: Permission denied
```
**Fix:** Run `chmod +x ./01_hello_world` then try again.

---

**Error: `cannot find type X in this scope`**

```
error[E0412]: cannot find type `HashMap` in this scope
```
**Fix:** Add `use std::collections::HashMap;` at the top of your file (like in Example 09).

---

**Error: `mismatched types`**

```
error[E0308]: mismatched types — expected i32, found f64
```
**Fix:** Rust does not automatically convert between number types. Use `as` to cast: `let x = 3.14_f64 as i32;`

---

## Tips for Learning Rust

- **Read the error messages carefully.** Rust's compiler errors are among the best in any language. They almost always tell you exactly what's wrong and often suggest a fix.

- **Experiment freely.** Modify the example code, break it on purpose, and see what happens. You can't cause any harm — just re-run `rustc` to compile again.

- **The borrow checker is your friend.** When the compiler rejects your code, it's preventing a real bug. Try to understand *why* it's complaining, not just how to silence it.

- **Don't rush ownership.** Example 06 is the hardest concept for most beginners. It's okay to move forward and come back to it. Ownership will "click" with practice.

- **Use Rust Playground** for quick experiments without any setup: 👉 https://play.rust-lang.org

---

## Next Steps

Once you're comfortable with all 9 examples, here's where to go next:

- 📘 **The Rust Book** (free, official): https://doc.rust-lang.org/book/
- 🏋️ **Rustlings** — small interactive exercises: https://github.com/rust-lang/rustlings
- 🎮 **Exercism Rust track** — practice problems with mentorship: https://exercism.org/tracks/rust
- 📖 **Rust by Example** — code-first learning: https://doc.rust-lang.org/rust-by-example/

---

## Blog Post

A detailed plain-English walkthrough of all 9 examples in this repository is available as a Medium article:

**[Rust for Beginners: A Complete Plain-English Guide to Your First 9 Rust Programs](https://medium.com/tech-ai-made-easy/rust-for-beginners-a-complete-plain-english-guide-to-your-first-9-rust-programs-b9c863172e52?sk=d853f2fef7a88484a11ece2af72261b4)**

The article covers every example with in-depth explanations — ideal if you prefer reading a narrative walkthrough alongside the code.

---

## Repository Structure

```
Rust-for-Beginners/
├── README.md                  ← You are here
└── examples/
    ├── README.md              ← Quick reference for running examples
    ├── 01_hello_world.rs      ← println! and main()
    ├── 02_variables.rs        ← let, mut, const, shadowing
    ├── 03_data_types.rs       ← integers, floats, bool, char, tuples, arrays
    ├── 04_functions.rs        ← fn, parameters, return types
    ├── 05_control_flow.rs     ← if/else, loop, while, for
    ├── 06_ownership.rs        ← ownership, clone, references, borrowing
    ├── 07_structs.rs          ← struct, impl, methods
    ├── 08_enums.rs            ← enum, match, Option
    └── 09_collections.rs      ← Vec, String, HashMap
```

---

Happy coding! 🦀 The Rust community is welcoming to beginners — don't hesitate to ask questions on the [Rust Users Forum](https://users.rust-lang.org/) or the [Rust Discord](https://discord.gg/rust-lang).
