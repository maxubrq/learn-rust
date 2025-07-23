# Struct

We can use struct (structure) to group related data, like `Tuple` type.

```rust
struct User {
    active: bool, // each line is called a field.
    username: String,
    email: String,
    sign_in_account: u64,
}
```

To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.

```rust
fn main() {
    let user = User {
        active: true,
        username: String::from("maxubrq"),
        email: String::from("hungtp.play@gmail.com"),
        sign_in_account: 1,
    }
}
```

To get a specific field value, we can use `.` operation.

```rust
fn main() {
    let user = User {
        active: true,
        username: String::from("maxubrq"),
        email: String::from("hungtp.play@gmail.com"),
        sign_in_account: 1,
    }

    println!("Username: {}", user.username);
}
```

We can also set/update a value of a field, of couse this behavior can only available when the user variable is mutable.

```rust
fn main() {
    let mut user = User {
        active: true,
        username: String::from("maxubrq"),
        email: String::from("hungtp.play@gmail.com"),
        sign_in_account: 1,
    }

    user.username = String::from("huntp.play+12@gmail.com");
}
```

> Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

### Field init shorthand

```rust
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,   // Shorthanded
        email,      // Shorthanded
        sign_in_account: 1,
    }
}
```

### Struct update syntax

Create instance from another instance.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("email"),
        sign_in_account: 1,
    }

    let user2 = User {
        username: String::from("username2"),
        ..user1 // All other fields will have the same value in the user1
    }

    // User 1 is no longer valid
    // Because email, a String, will be moved to the user2.
    // Other fields: active nad sign_in_account will be copied to user2
    
    let user3 = User {
        active: true,
        username: String::from("username"),
        email: String::from("email"),
        sign_in_account: 1,
    }

    let user4 = User {
        username: String::from("username"),
        email: String::from("email"),
        ..user3,
    }

    // In this case, email and username are not moved from user4
    // Other fields implemented the Copy trait.
    // So, user3 is still valid, and we can use it
}
```

### Struct without named fields

We can define struct without named field, it look like a tuple.

```rust
struct Color(i32, i32, i32);
```

An we can also give a name for a unit-like struct

```rust
struct UnitLike; // ()
```

### Ownership of Struct Data

First, we want the struct to own it data, so we use `String` in stead of `&str`.

But struct can store references in it. The only thing we need to ensure in this case is the references must live as long as the struct is and this required a `lifetime` genertic.

## Example of struct

```rust
#[derive(Debug)] // with derive(Debug) annotation
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // use :?
}

// rect1 is {width: 30, height: 50}
```

## Method syntax

```rust
#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize
}

impl Rectangle {
    fn new(width: usize, height: usize) -> Self { // associated function without self
        Self {
            width,
            height,
        }
    }

    fn area(&self) -> usize { // associated functions wit &self (method)
        self.width * self.height
    }

    fn width(&self) -> bool { // allow method with the same name with field
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn give(self) -> Self {
        self
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 11,
        width: 11,
    }

    println!("area = {}", rect1.area());

    let rect2 = Rectangle::new(11,11);

    println!("width = {}, width = {}", rect1.width, rect1.width());

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));

    let rect3 = rect1.give();

    // rect1 now becomes invalid, because using self instead of &self
    // comes from the situation where we want to tranform the data
    // and give the ownership to the other variable
}
```

