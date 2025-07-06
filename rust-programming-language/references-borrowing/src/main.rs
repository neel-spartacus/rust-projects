/*fn main() {
    let mut x: Box<i32> = Box::new(1);
    println!("x: {}",x);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    println!("a: {}",a);
    *x += 1;                 // *x on the left-side modifies the heap value,
    println!("x after increment: {x}");
    //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    println!("r1: {}",r1);
    let b: i32 = **r1;       // two dereferences get us to the heap value
    println!("b: {}",b);
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    println!("r2 :{}",r2);
    let c: i32 = *r2;    // so only one dereference is needed to read it
    println!("c: {}",c);
}*/
use std::boxed::Box;
use std::convert::From;
use std::mem::drop;
use std::string::String;
use std::vec::Vec;

// Taking ownership of the Box
fn process_owned(data: Box<i32>) -> Box<i32> {
    println!("Processing: {}", data);
    Box::new(*data * 2) // Create new Box
}

// Borrowing the Box
fn process_borrowed(data: &Box<i32>) -> i32 {
    println!("Processing borrowed: {}", data);
    **data * 2 // Return the value, not a Box
}

// More idiomatic: just borrow the inner value
fn process_inner(data: &i32) -> i32 {
    println!("Processing inner: {}", data);
    *data * 2
}

/*fn main() {
    let original = Box::new(5);

    // Using reference - original is still accessible
    let result1 = process_borrowed(&original);
    println!("Result: {}, Original: {}", result1, original);

    // More idiomatic approach
    let result2 = process_inner(&original);
    println!("Result: {}, Original: {}", result2, original);

    // Taking ownership - original is moved
    let new_box = process_owned(original);
    // original is no longer accessible
    println!("New box: {}", new_box);
}*/

/*fn main() {
    let boxed = Box::new(42i32);
    let box_ref = &boxed;

    // Different ways to access the value
    println!("Via Box: {}", boxed);        // Automatic deref
    println!("Via &Box: {}", box_ref);     // Automatic deref
    println!("Manual deref &Box: {}", *box_ref);   // Deref to Box<i32>
    println!("Double deref: {}", **box_ref);       // Deref to i32

    // The types at each level:
    // boxed: Box<i32>
    // box_ref: &Box<i32>
    // *box_ref: Box<i32>
    // **box_ref: i32
}*/


//Implicit-Explicit dereference

/*fn main()  {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
    }*/

/*fn main() {
    let x = 0;
    let mut x_ref = &x;
    println!("{x_ref} {x}");
}*/
/*fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    v.push(4);
   // println!("Third element is {}", *num);
}*/

/*fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    println!("Third element is {}", *num);
}*/

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num+=1;
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
}


/*fn main() {
    let mut x = 1;
    let y = &x;
    let z = *y;
    println!("{z}");
    x += z;
    println!("{x}")
}*/

/*fn first_or<'a, 'b, 'c>(strings: &'a Vec<String>, default: &'b String) -> &'c String {
    if strings.len() > 0 {
        &strings[0]
    } else {
        default
    }
}*/

//Missing lifetime specifier

/*fn return_a_string() -> &String {
    let s = String::from("Hello world");
    let s_ref = &s;
    s_ref
}*/

/*fn main() {
    let strings = vec![];
    let default = String::from("default");
    let s = first_or(&strings, &default);
    drop(default);
    println!("{}", s);
}*/





/*
In Rust, Box<i32> and &Box<i32> represent fundamentally different concepts related to ownership and borrowing.
Let me break down the differences in detail.
Box<i32> - Owned Smart Pointer
Box<i32> is an owned smart pointer that allocates memory on the heap and has exclusive ownership of that memory.
Key characteristics:

Ownership: You own the boxed value completely
Memory management: Automatically deallocates when it goes out of scope
Mutability: Can be moved, and if declared as mut, the contents can be modified
Size: Takes up one pointer-sized word on the stack (8 bytes on 64-bit systems)

&Box<i32> - Borrowed Reference to a Box
&Box<i32> is a reference (borrow) to a Box<i32>. It doesn't own the Box but can access it.
Key characteristics:

Borrowing: You're borrowing access to someone else's Box
No ownership: Cannot move or consume the Box
Lifetime-bound: Must respect Rust's borrowing rules
Size: Also one pointer-sized word, but points to the Box on the stack


When to Use Each
Use Box<i32> when:

You want to own heap-allocated data
You need to transfer ownership
You're building data structures like linked lists or trees
You want to avoid stack allocation for large data

Use &Box<i32> when:

You want to temporarily access a Box without taking ownership
You're passing Boxes to functions that don't need ownership
However, in most cases, &i32 (borrowing the inner value) is more idiomatic.

Important Note on Idiomatic Rust
In practice, &Box<T> is rarely used because it's usually better to use &T directly:

// Less idiomatic
fn process_box_ref(data: &Box<i32>) -> i32 {
    **data * 2
}

// More idiomatic
fn process_value_ref(data: &i32) -> i32 {
    *data * 2
}

fn main() {
    let boxed = Box::new(10);

    // Both work, but the second is preferred
    let result1 = process_box_ref(&boxed);
    let result2 = process_value_ref(&boxed); // Box derefs to &i32 automatically
}

 */