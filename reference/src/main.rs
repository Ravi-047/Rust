fn main() {
    println!("reference practice");
}

fn first(){
    let mut x = 10;
    let y = & mut x; // y is a reference to x
    {
        let z = & mut y; // z is a reference to y
        *z += 1; // dereference z and increment by 1
        println!("z: {}", z);
    }
    println!("x: {}", x);
    println!("y: {}", y);
}

