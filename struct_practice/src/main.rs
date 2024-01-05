
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Home {
    name: String,
    address: String,
    price: u64,
    sold: bool,
}

fn main() {
    println!("struct");
    
    let mut user1 = User {
        name: String::from("Ravi"),
        email: String::from("ravi@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let mut home1 = Home {
        name: String::from("Home1"),
        address: String::from("Address1"),
        price: 100000,
        sold: false,
    };

    user1.active = false;
    println!("user1: {}", user1.name);
    println!("home1: {}", home1.name);  
}

