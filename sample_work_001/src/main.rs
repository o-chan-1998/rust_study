fn main() {
    println!("Hello, world!");
    run();
}

pub fn run() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    println!("Stack address of v1 is: {:p}", &v1);  // Stack address of v1 is: 0x7ffe5b788440
    println!("Stack address of v2 is: {:p}", &v2);  // Stack address of v2 is: 0x7ffe5b788458
    println!("Heap memory address of v1: {:?}", v1.as_ptr());   // Heap memory address of v1: 0x557144e1eae0
    println!("Len of v1 is: {}", v1.len()); // Len of v1 is: 4
    println!("Capacity of v1 is: {}", v1.capacity());   // Capacity of v1 is: 4
}
