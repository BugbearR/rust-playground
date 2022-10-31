use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].trim().parse().expect("Invalid u32");
    println!("{}", n);
}
