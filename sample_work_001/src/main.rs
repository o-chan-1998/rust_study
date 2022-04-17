fn main() {
    println!("Hello, world!");
    run();
}

pub fn run() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("Stack address of v1 is: {:p}", &v1);  // Stack address of v1 is: 0x7ffe5b788440
    println!("Heap memory address of v1: {:?}", v1.as_ptr());   // Heap memory address of v1: 0x557144e1eae0
    println!("Len of v1 is: {}", v1.len()); // Len of v1 is: 4
    println!("Capacity of v1 is: {}", v1.capacity());   // Capacity of v1 is: 4
    println!("{:?}", v1);

    v1.insert(1, 10);
    println!("------------------------------");
    println!("Stack address of v1 is: {:p}", &v1);  // Stack address of v1 is: 0x7ffe5b788440
    println!("Heap memory address of v1: {:?}", v1.as_ptr());   // Heap memory address of v1: 0x557144e1eae0
    println!("Len of v1 is: {}", v1.len()); // Len of v1 is: 4
    println!("Capacity of v1 is: {}", v1.capacity());   // Capacity of v1 is: 4
    println!("{:?}", v1);

    v1.remove(0);
    println!("------------------------------");
    println!("Stack address of v1 is: {:p}", &v1);  // Stack address of v1 is: 0x7ffe5b788440
    println!("Heap memory address of v1: {:?}", v1.as_ptr());   // Heap memory address of v1: 0x557144e1eae0
    println!("Len of v1 is: {}", v1.len()); // Len of v1 is: 4
    println!("Capacity of v1 is: {}", v1.capacity());   // Capacity of v1 is: 4
    println!("{:?}", v1);

    let mut v3 = vec![9, 10];
    v1.append(&mut v3);
    println!("------------------------------");
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    println!("{:?}", v1);
    println!("Stack address of v3 is: {:p}", &v3);
    println!("Heap memory address of v3: {:?}", v3.as_ptr());
    println!("Len of v3 is: {}", v3.len());
    println!("Capacity of v3 is: {}", v3.capacity());
    println!("{:?}", v3);
}
