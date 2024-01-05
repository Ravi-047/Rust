mod my_module {
    pub fn personal () { // pub makes this function public
        println!("personal function");
    }
}

fn main() {
    println!("Hello, world!");
    my_module::personal(); // call the function from the module
}
