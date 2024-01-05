enum CarTypes {
    Sedan,
    SUV,
    Hatchback,
    Coupe,
    Convertible,
}


fn main() {
    println!("enum");
}

fn print_cars() {
    let car1 = CarTypes::Sedan;
    let car2 = CarTypes::SUV;
    let car3 = CarTypes::Hatchback;
    let car4 = CarTypes::Coupe;
    let car5 = CarTypes::Convertible;

    println!("car1: {:?}", car1);
    println!("car2: {:?}", car2);
    println!("car3: {:?}", car3);
    println!("car4: {:?}", car4);
    println!("car5: {:?}", car5);
}

// car: CarTypes
// match car {}
fn car_print(){
    CarTypes::Sedan => {
        println!("Sedan");
    };
    CarTypes::SUV => {
        println!("SUV");
    };
}