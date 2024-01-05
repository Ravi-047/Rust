fn main() {
    println!("Vector Practice");

    let mut abc: Vec<i32> = vec![1, 2, 3];
    println!("abc: {:?}", abc);

    abc.push(4);
    println!("abc: {:?}", abc);
    println!("abc[0]: {}", abc[0]);

    let mut xyz: Vec<i32> = Vec::new(); // empty vector
    xyz.push(1);
    println!("xyz: {:?}", xyz);
}
