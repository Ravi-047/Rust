fn main() { // main function is the entry point of the program
    println!("Hello, world!");
    first_fn(); // calling the first_fn function inside the main function
    second_fn(10); // calling the second_fn function inside the main function with single parameter
    third_fn(10, 'A'); // calling the third_fn function inside the main function with multiple parameters
    expression_fn(); // calling the expression_fn function inside the main function

    let x = return_value(10); // calling the return_value function inside the main function
    println!("The value of x is: {}", x);
}


fn first_fn(){
    println!("This is my first function");
}

// example of statement

// passing single parameter to the function
fn second_fn(x: i32){
    println!("The value of x is: {}", x);
}

// passing multiple parameters to the function
fn third_fn(x: i32, y: char){ // char is a single character
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// expression 
fn expression_fn(){
    let x = 10;
    let y = {
        let x = 5;
        x + 1 // expression
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


// return value from function
fn return_value(x: i32) -> i32 {
    x + 1 // expression
}