use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let arg1 = &args[1];
    println!("arg1:{}", arg1);
}
