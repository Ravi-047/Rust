fn main() {
    println!("Types of loop");
    first();
    second();
    third();
}

fn first(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

// example of while loop
fn second(){
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn third(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// example of for loop
fn fourth(){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the value is: {}", element);
    }
}

// example of for loop with range
fn fifth(){
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn sixth(){
    for x in 1..11{
        if x == 3 {
            continue;
        }
        println!("x is {}", x);
    }
}