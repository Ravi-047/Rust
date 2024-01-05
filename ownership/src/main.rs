fn main() {
    println!("ownership practice");
    first();
}

fn first(){
    let x = 5;
    println!("The value of x is: {}", x);

    let y = x;
    println!("The value of y is: {}", y);

    //---- string
    let s1 = String::from("hello");
    //String::from is a function that returns a String type
    let b = s1.clone();
    //clone is a method of String type

    println!("{}, world!", b);
}
