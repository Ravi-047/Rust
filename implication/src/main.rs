struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");
    let rect1:Reactangle = Reactangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
