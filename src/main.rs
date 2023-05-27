// Rust usees the snake case as the conevntional style fo function and variable names
fn main() {
    println!("Hello, world!");

    another_function();

    function_with_parameters(5);
}

fn another_function() {
    println!("Another function.");
}

// function with parameters
fn function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}


