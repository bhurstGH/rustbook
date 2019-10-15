// Vars are immutable by default. 
// Nudges toward taking advantage of safety and easy concurrency

// Once immutable variable has bound a value, the value can't be changed.

fn main() {
// Won't compile, but the compiler gives helpful messages
// Compiler guarantees immutable values won't change
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

// Mutability is useful. Conveys intent to change value. 
    let mut x = 5;
    println!("Value of x is: {}", x);
    x = 6;
    println!("Value of x is: {}", x);

    // Trade offs to consider.
    // Large data structures, mutating an instance in place may be faster than allocating new instances
    // Small data structures, creating new instances in a more functional style may be easier to think through. Lower performance for clarity.

// Vars and Consts are different
// No mut with consts. Always immutable
// const must always be annotated with type
// const can be declared in any scope
// Can only be set as a constant expression. Not result of func call or values computed at run time
// All uppercase and underscore convention

// underscores can also be inserted in numberic literals for readability
// const MAX_POINTS: u32 = 100_000;

// const valid for entire run time in their scope

// Shadowing
// declare a new variable with the same name as existing variable
// First variable is shadowed by second variable

let y = 5;
let y = y + 1;
let y = y * 2;
println!("Value of y is: {}", y);

// Shadowing is different from mut
// let keyword necessary for shadowing -- allows transformations before immutability
// Effectively creating new var, can change the type but keep same name

let spaces = "   ";
let spaces = spaces.len();

// if we use mut, this doesn't work. Can't mutate variable type.

}
