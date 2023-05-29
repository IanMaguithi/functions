// Rust usees the snake case as the conevntional style fo function and variable names
fn main() {
    println!("Hello, world!");

    another_function();

    function_with_parameters(5);

    function_with_multiple_paraneters(5, 'h');

    return_y();

    let mut x = five();
    function_with_parameters(x);

    x = plus_one(5);
    function_with_parameters(x);

    //? ownership
    let s = String::from("hello");
    // clone is used to create a deep copy of the data on the heap
    let s2 = s.clone();
    takes_ownership(s);
    println!("{}", s2); // this will throw an error because s was moved to the function
}

fn another_function() {
    println!("Another function.");
}

// function with parameters
fn function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

// function with multiple parameters
fn function_with_multiple_paraneters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// exploring expressions and statements
//* generally adding a semicolon to an expression makes it a statement
//* expressions return a value
fn return_y() {
    let y = {
        let x = 1;
        x + 1
    };
    println!("The value of y is: {y}");
}

// functions that return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//? ownership
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
