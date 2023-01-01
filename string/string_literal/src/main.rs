fn type_name<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let s = "Hello, world!";
    println!("{}", s);
    println!("{}", type_name(s));
    println!("{}", type_name("Hello, world!"));
    println!("{}", &s[..3]);
    println!("{}", &s[..=3]);
    println!("{}", &s[1..3]);
    let ss = s.to_string();
    println!("{}", type_name(ss));
    // println!("{}", &ss[1..3]);
}
