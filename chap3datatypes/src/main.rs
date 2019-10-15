// Every value has a data type so rust knows how to work with that data
// Two types: scalar and compound

// Rust is statically typed, it must know all types at compile time
// Can usually infer types

// let guess: u32 = "42".parse().expect("Not a number");
// type annotation needed since many types are possible

//-----

// Scalar Types
// Represents a single value. Four primary types.
// Integers, floating-point, Booleans, characters

// Integer types
// Number without fractional component. i8, i16, u32, u64, u128, usize (arch), etc
// Signed numbers are stored with two's complement representation

// -(2 to the n -1) to 2 to the n -1 -1 inclusive range, where n is bit number
// i8 = -(2 to the 7) to 2 to the 7 -1 = -128 to 127
// Unsigned variants store 0 to 2 to the n - 1. u8 = 0 to 255

// usize and isize depend on computer. 64 bit architecture or 32 bit.

// Can write integer literals. Number literals	Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

// If you're unsure, trust Rust defaults. Integers default to i32. Generally the fastest even on 64bit systems
// Primary case for isize/usize is indexing some sort of collection

// Integer Overflow
// Try to save value outside of 256 to a u8, integer overflow occurs
// Rust rules will panic at runtime when compiling in debug mode
// When compiling with --release flag, Rust does NOT check for integer overflow
// No panic, Rust performs two's complement wrapping. Integers wrap back to 0
// Wrapping standard library if you explicitly want to wrap

// Floating-Point Types
// f32 and f64, roughly the same speed but f64 is capable of more precision
// IEEE-754 standard. f32 is single precision float, f64 is double precision

// Boolean
// True, false. One byte in size 

// Character Type
// Most primtive alphabetic type. char literals specificied by single quotes, string literals are double quotes
// Four bytes and represents Unicode Scalar Value, so can represent more than ASCII

//-----

// Compound Types
// Can group multiple values into one type. Rust has two primitive compound types, tuples and array

// Tuple Type
// General way of grouping values with a variety of types into one compound type
// Fixed length, cannot grow or shrink after declared
// Created by comma separated list of values inside paratheses. Each position has a type.

// Array Type
// Every element must have same type. Fixed length.
// Comma separated list into square brackets.

// Useful for data allocation to stack rather than heap or to ensure fixed number of elements
// Less flexible than vectors -- growable arrays.
// One example of array over vector: months of the year
// x: [i32; 5] - type and size
// Alternative init: x = [3; 5] - same value in each element

// Single chunk of memory allocated on stack

// Accessing out of bounds index causes runtime error
// Safety principles in action. Many low-level languages don't do this check.
// Prevents invalid memory access. Rust immediately exits.


fn main() {
    // Floating points
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Numeric Operations

     // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Character
     let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching to pull out values with destructuring
    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup2;

    println!("The value of y is: {}", y);

    // Can also access element with a period/dot syntax
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Array
    let a = [1, 2, 3, 4, 5];
}
