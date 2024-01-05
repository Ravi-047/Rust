trait Printable{
    fn print(&self);
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: u32,
    height: u32,
}

struct Square {
    side: f64,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn main() {
    println!("Traits Practice!");
    let rect1 = Rectangle { width: 30, height: 50 };
    rect1.print();

    let sq1: Square = Square { side: 3.0 };
    print_area(shape:sq1);
}
