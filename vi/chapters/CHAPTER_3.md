# Common Programming Concepts

## Variables and mutability

**By default variabels are immutable**

```rust
fn main(){
    let x = 5;
    x = 6;
}
```

Will get the compiler error because `x` is immutable.

But can make the variable mutable via `mut` key word.

```rust
fn main(){
    let mut x = 5;
    x = 6;
}
```

### Constant

Constants, of course, always immutable and cannot use with the `mut` key word.

To define a constant, use the `const` key word instead of `let`.

```rust
fn main(){
    const DAYS_OF_WEEK : u32 = 7;
}
```

**Constants can be defined in any scrope**.

Constants naming convesion in Rust is `ALL_UPPERCASE_WITH_UNDERSCORDE_BETWEEN_WORDS`.

### Shadowing

*Shadowing* is the ability that we can create variables with the same name using `let` key word.

```rust
fn main(){
    let x = 5;
    let x = x + 1; // this x overshadow the first one
    {
        let x = x * 2; // this x overshadow the second one
        printnl!("x in scope {x}");
    }

    println!("x out scope {x}");
}

// x in scope 12
// x out scope 6
```

Shadowing helps us to perform some changes on the variable and make sure the shadowed one will not be change accidentally because the shadowing will end when out of the scope (or being shadowed by an other one).

Because we using the `let` key word to perform a shadowing, so we can change the type of the variable.

```rust
fn main(){
    let x = String::new(); // String
    let x = x.len(); // usize
}
```

## Data type

> Every value in Rust is of a certain data type, which tells Rust what kind of
data is being specified so it knows how to work with that data. We’ll look at
two data type subsets: `scalar` and `compound`.

Rust is a *statically typed* language, it needs to know the type of the variable at the compile time. Often, the compiler can infer the type from the value, but in the situation that many types have chance, you need to specific the type of the variable.

### Scala Types

> **Scala Types**: represents a single value. Rust has 4 primary scala types: Interger, Floating-point numbers, Boolean and Character.

### Interger types

**Numbers wihout fractional component.**

- Signed integer: i8, i32, i64, i128, isize
- Unsinged integer: u8, u32, u64, u128, usize

`isize`, `usize`, are depend on the architecture that the program run in. If it is an 64bit, they will be equal to i64, and u64.

You can write integer in any of the below forms.

```rust
fn main(){
    let x: i32 = 10_000; // 10,000 - Decimal
    let x: i32 = 0x2710; // 10,000 - Hex
    let x: i32 = 0o23420; // 10,000 - Octal
    let x: i32 = 0b0010_0111_0001_0000; // 10,000 - Binary
    let x: i32 = b'A'; // u8 only - Byte
}
```

#### Integer overflow

In the case that you put a value greater than defined type, let's say you put 256 into an `u8` (0->255) variable. **Integer overflow** will be occurred.

In this case: Rust do 2 different behaviors:
- In debug mode: panic and cannot receover
- In release mode: it will wrapp the value (using modulo), in this case 256 -> 0.


Default integer type is `i32`.

### Floating-point types

Rust has 2 floating point types: `f32` and `f64`. `f64` is the default type.


### Numeric operations

```rust
fn main(){
    let sum = 5 + 10;
    let substact = 9.2 - 3.2;
    let multiply = 4 * 5;
    
    let devision = 9.2 / 2.3;
    let truncate = -5 / 3; // result =  -1
    let remainder = 12 % 5; // 2
}
```


### Boolean type

The boolean type has 2 possible values: `true` and `false`. This is one byte type and use `bool` keywrord to defined.

The main use of the boolean type is for conditions and control flows.


### Character type

Character in Rust represents a unicode scalar value, using single quote `'`, 4 bytes size, using `char` keyword to define.

```rust
fn main(){
    let c: char = 'X';
    let c: char = 'B';
    let c: char = '🤔';
}
```

## Compound types

### Tuples type

> A tuple is a general way of grouping together a number of values with a
variety of types into one compound type. Tuples have a fixed length: once
declared, they cannot grow or shrink in size.

> We create a tuple by writing a comma-separated list of values inside
parentheses.

```rust
fn main(){
    let tup: (i32, char, bool) = (2, 'A', false);

    // Pattern destructuring

    let (x, z, y) = tup; // x = 2, y = 'A', z = false

    // Period

    let num = tup.0; // num = 2
    let c = tup.1; // c = 'A'
    let b = tup.2; // b = false


}
```

**The tuple without any values has a special name, _unit_**. This value and its
corresponding type are both written () and represent an empty value or an
empty return type. *Expressions implicitly return the unit value if they don’t
return any other value.*

**Tuples are fixed size**.

### Array type

Fixed length, and same type collection of data.

```rust
fn main(){
    let arr = [1,2,3,4];
}
```

> Arrays are useful when you want your data allocated on the stack rather
than the heap.

A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector. 

```rust
fn main(){
    let arr1: [i32, 5] = []; // A fixed 5 elements in i32

    let arr2 = [3; 5]; // [3,3,3,3,3]

    let x = arr2[0]; // 3
}
```

## Function

Use `fn` keyword to define functions.

Rust convension is to use `snake_case` for functions and vairables name.

### Parametter

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

This is the valid function, return a value.

Note that is we add the ";" to the function at `a + b`. We will get an error type mismatch, because adding ";" will make this become a statement and statements don't return value (return "()" unit).

```rust
fn add(a: i32, b:i32) -> i32{
    a + b; // Compile error
}
```

## Comments

Use `//` to comment

## Control flows

```rust
fn main (){
    let x = 5;
    if x <= 5 {
        println!("Less than or equal to 5");
    } else {
        println!("Greater than 5");
    }
}
```

Because `if` is an expression so it can be used to return value in the let statement.

```rust
fn main(){
    let x = 5;
    let c = if x <= 5 {'A'} else {'B'};
}
```

## Repetition (Loop)

`loop` is used for repetitive tasks

```rust
loop {
    println!("Hello");
}
```

This will print "Hello" forever.

`while` is used for looping with condition

```rust
fn main(){
    let mut x: i32 = 10;
    while x < 10 {
        println!("Hello");
        x += 1;
    }

    println!("Out");
}
```

`for` a concise way to loop over an collection

```rust
fn main(){
    let arr = [9; 5];
    for item in arr {
        println!("Item = {item}");
    }
}
```