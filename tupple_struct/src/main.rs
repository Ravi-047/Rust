fn main() {
    println!("tupple struct");
    first();
}


fn first(){
    struct User(u8,u8,u8);
    let mut user1 = User(1,2,3);
    println!("user1: {:?}", user1);

    user1.0 = 10;
    println!("user1: {:?}", user1);

    struct user_details (String, i16, bool);
    let user2 = user_details(String::from("user2"), 20, true);
    println!("user2: {:?}", user2);
}
