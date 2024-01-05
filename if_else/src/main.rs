fn main() {
    println!("If, else, else If");

    frist();
    second();
    third();
}

fn first(){
    let num = 3;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn second(){
    let num = 3;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }
}

fn third (){
    let condition = true;
    let num = if condition {
        5
    } else {
        6
    };

    println!("The value of num is: {}", num);
}
