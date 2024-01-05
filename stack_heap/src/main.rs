fn main() {
    println!("stack and heap");

    stack_memory();
    heap_memory();
}

// stack memory
fn stack_memory() {
    let x = 5;
    let y = 8;

    let sum = add(x, y);
    println!("sum = {}", sum);
    println!("The sum of x {} and y {} is {}", x, y, sum);

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// heap memory
fn heap_memory() {
   let mut v:Vec<i32> = Vec::new(); // dynamic memory allocation for vector v

   v.push(5);
   v.push(6);
   v.push(7);

   println!("v = {:?}", v);

}