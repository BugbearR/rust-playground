fn my_vec2<T: std::default::Default>() -> Vec<T> {
    // vec![0, 0]
    vec![Default::default(), Default::default()]
}

fn main() {
    let myvec1: Vec<i32> = my_vec2();
    let myvec2: Vec<u32> = my_vec2();
    println!("{:?}", myvec1);
    println!("{:?}", myvec2);
}
