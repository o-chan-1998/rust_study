fn main() {
    println!("Hello, world!");
    run();
}

pub fn run() {
    let a1: [u8; 9000000] = [1; 9000000];
}

// [root@localhost sample_work_001]# cargo run
// warning: unused variable: `a1`
//  --> src/main.rs:7:9
//   |
// 7 |     let a1: [u8; 9000000] = [1; 9000000];
//   |         ^^ help: if this is intentional, prefix it with an underscore: `_a1`
//   |
//   = note: `#[warn(unused_variables)]` on by default

// warning: `sample_work_001` (bin "sample_work_001") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//      Running `target/debug/sample_work_001`
// Hello, world!

// thread 'main' has overflowed its stack
// fatal runtime error: stack overflow
// 中止 (コアダンプ)