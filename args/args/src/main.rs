use std::env;

fn main() {
    let mut args_raw = env::args();
    println!("{:?}", args_raw);
    println!("{:?}", args_raw.next());
    println!("{:?}", args_raw.next());
    println!("{:?}", args_raw.next());

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let arg1 = &args[1];
    println!("arg1:{}", arg1);
}
