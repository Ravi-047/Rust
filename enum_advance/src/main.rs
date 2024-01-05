#[derive(Debug)]

enum GenderCat {
    Male,
    Female,
    Transgender,
}

struct Person {
    name:String,
    email:String,
    age:u64,
    gender:GenderCat,
}

fn main() {
    println!("enum advance");
    person_1();
    
    let result = calculate(10);
    println!("result: {:?}", result);
}

fn person_1(){
    let person1 = Person {
        name:String::from("John"),
        email:String::from("john@gmail.com"),
        age:45,
        gender:GenderCat::Male
    };
    println!("person1: {:?}", person1);
}

//option enum
// enum Option<T> {}

fn calculate(num:i32) -> Option<i32> {
    if num % 2 == 0 {
        Some(num)
    } else {
        None
    }
}