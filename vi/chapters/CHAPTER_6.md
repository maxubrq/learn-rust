# Enum & Pattern Marching

> Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum.

```rust
enum IpAddressKind {
    V4,
    V6,
}

fn main() {
    let v4 = IpAddressKind::V4;
    let v6 = IpAddressKind::V6;
}
```

Enums can hold value

```rust
enum IpAddressKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

Enum give us the flexibility to handle states with different type of data

```rust
enum Message {
    Quit,                           // Unit ()
    Move { x: i32, y: i32 },        // A struct with x, y fields
    Write(String),                  // A string
    ChangeColor(i32, i32, i32),     // A tuple
}
```

## `Option` enum

There is special enum named `Option` it has the type of `Option<T>`, and look like this in the implementation.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

> The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>

Where `Some` variant tells us that there is some value that may be interesting. And `None` tells that there is no value at all.

## Pattern Matching

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Aha, a penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // Match variable holds value
            println!("The quarter from {state:?}");
            25
        },
    }
}
```

When use match, we should catch all the variants of the enum, otherwise the compiler will throw an error.

## Match `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // if None return None
        Some(i) => Some(i + 1), // If has some value, return +1
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## Catch-All Patterns and the `_` Placeholder

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(), // rest of the case go here
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

```

## Concise Control Flow with `if let` and `let...else`

### `if let`

```rust
// ---- Version 1: Match ----
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}

// ---- Version 2: if let ---
// Do exactly the same thing like the version 1
// Print if there is some value
// Do nothing for the rest cases (None) in this situation
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}


// ---- if let ... else ---
// ---- Version 1: Match ----
// Print the state of the quarter if it is a Quarter variant
// Increase the counter by 1 for counting
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}

// ---- Version 2: if let ... else ---
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

### `let ... else`

```rust
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

// --- Version 1 -- use if let ----
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// --- Version 2 -- use if let with expression return ----
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// --- Version 3 -- use let ... else ---
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

```