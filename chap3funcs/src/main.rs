// main() is common entry point. Snake case is conventional naming style.

fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 6);
    some_expression();

    let x = five();

    println!("x is {}", x);

    let y = plus_one(5);

    println!("y + 1 is {}", y);
}

// Rust doesn't care where functions are defined
fn another_function() {
    println!("Another function.");
}

// type annotation is required
// Compiler almost never needs you to provide type elsewhere in code
fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Func Bodies made up of statements optionally ending in an express
// Rust is an expression-based language. Ending expression different from express as part of a statement
// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resulting value

// let y = 6; is a statement
// Function definitions are also statements. i.e. the main() function is a statement
// You cannot, for example: let x = (let y = 6); - statements don't return a value
// This is different than other languages like C and Ruby where the assignment returns its own value
// x = y = 6 does not work in Rust

// 5 + 6 math operation is an Expression. Expressions can be a part of statements
// let y = 6; -- 6 is an expressions that evaluates to 6. Calling a func or macro is an expression
// {} scope blocks are expressions

fn some_expression() {
    // Compiler #[warn(unused_variables)] - suggested adding underscore
    // Underscore tells compiler we don't care if the variable is unused???
    let _x = 5;

    let y = {
        let x = 3;
        // No semicolon. Expressions do not including ending semicolons
        // Would be a statement with a semicolon, which will not return a value
        x + 1
    };

    println!("The value of y is: {}", y);
}

// Funcs can return values to the code that calls them
// Return values aren't named but declare their type after the -> arrow
// return value is synonymous with the value of the final express in the body block of the func

// Can return early with the return keyword but most often return the last expression implicitly

fn five() -> i32 {
    // no func calls, macros, or let statements
    // just the number 5 expression by itself
    5
}

fn plus_one(x: i32) -> i32 {
    // if we added a semicolon, this would error with mismatched types
    // "expected i32, found ()"
    // statements dont evaluate to a value, which is expressed by (), an empty tuple
    x + 1
}