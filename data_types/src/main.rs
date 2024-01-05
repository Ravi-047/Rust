fn main() {
    println!("Hello, world!");

    // Scalar Types
    // Integer Types
    // String Types
    // Boolean Types
    // Character Types
    // Floating-Point Types

    // length 8bit, 16bit, 32bit, 64bit, 128bit arch signed(i32) and unsigned (u32)

    // scalar types - types that represent a single value
    let num: i32 = 10;
    println!("The value of num is: {}", num);

    let is_male: bool = true;
    println!("{}", is_male);

    let character: char = 'a';
    println!("{}", character);

    let float: f32 = 2.3;
    println!("{}", float);

    // compound types - types where we can group multiple values into one type
    // tuple
    // arrays
    // distionary

    // tuple
    let tup = (1, 2, 3, 4, 5);
    let mut tup2: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}", tup.0); // single value
    println!("{:?}", tup2); // all values

    tup2.0 = 1000;
    println!("{:?}", tup2);

    //Arrays
    let aray = [1, 2, 3, 4, 5];
    println!("{:?}", aray); // all values
    println!("{}", aray[0]); // single value

    let aray2: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", aray2);
}
