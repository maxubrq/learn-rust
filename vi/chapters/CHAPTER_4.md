# Chapter 4: Ownership

Ownership rules:
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable scope
```rust
fn main(){
    {
        let s = String::new(); // initial a string s
    } // the scope is end - s is no longer valid
}
```

When the variable is go out of scope, Rust will call the `drop` function, and it will free the memory in Heap. 

## Data interaction with move

```rust
{
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership is now move from s1 to s2
                 // And s1 is no longer valid

    println!("String is: {s1}"); // <-- Compile error
}
```

But why?

The `s1` is assigned the value not the `Hello` but with this structure, called string data:

- pointer: point to the the address where the actual value,`Hello`, is stored in Heap.
- len: the current size of the string in bytes.
- capacity: the maximum size of the string in bytes.

When we assign `s1` to `s2`, Rust doesn't copy the value of `Hello` to s2, instead, it copys the string data to s2.

When the `s2` and `s1` go out of scope, Rust will need to call `drop` two times, this called the `Double free` problem. And freeing the memory two times can lead to **memory corruption** which can lead to security vulnerabilities.

To ensure the memory is safe, in this situation, Rust considers that the `s1` is no longer valid, therefore, s1 cannot be used anymore.

In this situation, we call the ownership is moved from s1 to s2. It is equal to `shallow copy` in other languages.

### Deep copy

Use `clone` method to make a deep copy.

```rust
fn main(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    printnl!("Hi there!, {s1} there"); // <-- No error
}
```

But deep copy can be an expensive operation.

### Stack data only - Copy

```rust
fn main(){
    let a = 12;
    let b = a;

    println!("{a}"); // <-- No error
}
```

**Why?**

Because the `a`, `b` size are known at compile time,so, their values will be store in stack in stead of heap.

Rust has a `Copy` trait for variable types that store in stack, and the actual value of these variables will be copy.

Rust won't let use annotate the `Copy` trait with the types that has `Drop` trait.

## Ownership and function

The mechanics of passing a value to a function are similar to those when
assigning a value to a variable.

```rust
fn main(){
    let s = String::from("Hello");
    take_ownership(s);

    println!("{s} world!"); // Compile error
}

fn take_ownership(s: String) -> String{
    s
}
```

As you can see in the above code, we copy the String data to the function `take_ownership` so that the `s` variable is now no longer valid. If we try to use it after the take_ownership function, a compile error will be occurred.

But this below code is fine because the x's value is copy (it is stored in stack) using the `Copy` trait.

```rust
fn main(){
    let x: isize = 5;
    make_copy(x);

    println!("x={x}");
}

fn make_copy(x: isize) -> isize {
    x
}
```

## Return value and Scope

```rust
fn main(){
    let s1 = give_ownership();          // Give ownership to s1
    let s2 = String::from("World");     // S2 assgined and take ownership
    let s3 = take_and_give_back(s2);    // s2 ownership is now gone, it is 
                                        // invalid now, but the function if the ownership back to s3
}   // s3 is out of scope -> s3 call drop
    // s2 is moved -> nothing happen
    // s1 is out of scope -> s1 call drop

fn take_and_give_back(s: String)->String{
    s
}

fn give_ownership() -> String{
    let s = String::from("Hello");
    s
}
```

## Reference & Borrowing

> A reference is like a
pointer in that it’s an address we can follow to access the data stored at that
address; that data is owned by some other variable. Unlike a pointer, a refer-
ence is guaranteed to point to a valid value of a particular type for the life
of that reference.

```rust
fn main(){
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");
}

fn calculate_length(s: &String) -> isize {
    s.len()
}   // s is out of scope -> drop
    // but it is a reference to string type (s1) not take the ownership of 
    // "Hello" from s1. So, in this code, s1 is still valid because it
    // hold the ownership of the data.
```

We call the action of creating a reference __borrowing__. As in real life, if a
person owns something, you can borrow it from them. When you’re done,
you have to give it back. You don’t own it.

And, of course, if you borrow somethings, you have the right to use it, but the right to change it is not, unless, the owner approve you to change it. So, Rust has the same thing.

```rust
fn main(){
    let mut s1 = String::from("Hello");
    change(&s1); // Compile error, you don't have the right to change the data
}

fn change(s: &String){
    s.push_str(", world");
}
```

We can change the data of the reference via `&mut` a mutable reference.

```rust
fn main(){
    let mut s1 = String::from("Hello");
    change(&mut s1);   
}

fn change(s: &mut String){
    s.push_str(" wolrd");
}
```

There are one rule with mutable reference: **Only one mutable reference at a time**. Becasue if there are more than one things can change the data at a time, they can lead to `data race` and give us unstable result.


```rust
fn main(){
    let mut s1 = String.from("Hello");
    let s2 = &mut s1;
    let s3 = &mut s1;
} // Compile error
```

And you also need to choose either reference or mutable reference for the variable.

```rust
fn main(){
    let s1 = String::from("Hello");
    let s2 = &s1; // No problem
    let s3 = &s1; // No problem
    let s4 = &mut s1; // Problem
}
```

Because you cannot have both immutable reference and mutable reference at the same time.

The scope of the references will be span to the last time the reference is used.

```rust
fn main(){
    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;
    println!("s2={s2}, s3={s3}"); // s2, s3 is no longer used, they are droped

    let s4 = &mut s1; // Ok then
}
```

## Dangling reference

>In languages with pointers, it’s easy to erroneously create a dangling pointer—
a pointer that references a location in memory that may have been given to
someone else—by freeing some memory while preserving a pointer to that
memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference
to the data does.

```rust
fn main(){
    let s = reference_to_nothing(); // Compile error
}

fn reference_to_nothing() -> &String{
    let s = String::from("Hello");
    &s
}   // s is out of scope -> dropped
    // &s is now a dangling reference
```

### The rules of references

- At any given time, you can have either one mutable reference or any
number of immutable references.
- References must always be valid.

## Slice type

> Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

### String slice

String slice is annotated by `&str`. `..` is range operator.

```rust
fn main(){
    let s = String::from("This is a long sentence");
    let s1 = &s[0..3];  // "This"
    let s2 = &s[..2];   // "Th"
    let s3 = &s[4..];   // " is a long sentence"
    let s4 = &s[..];    // "This is a long sentence"
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

String literals are always string slices.

```rust
fn main(){
    let s = "This is a string literal";
    let s: &str = "This is also a string literal";
}
```

### Other slices

Other slice types are annotated by `&[<T>]`, for example a slice of `i32` is `&[i32]`.