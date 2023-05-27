// Rust usees the snake case as the conevntional style fo function and variable names
fn main() {
    println!("Hello, world!");

    another_function();

    function_with_parameters(5);

    function_with_multiple_paraneters(5, 'h');

    return_y();

    let x = five();
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

// function with parameters
fn function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

// function with multiple parameters
fn function_with_multiple_paraneters(value:i32, unit_label:char){
    println!("The measurement is: {value}{unit_label}");
}

// exploring expressions and statements
//* generally adding a semicolon to an expression makes it a statement
//* expressions return a value
fn return_y(){
    let y = {
        let x = 1;
        x+1
    };
    println!("The value of y is: {y}");
}

// functions that return values
fn five() -> i32{
    5
}
