use unicode_segmentation::UnicodeSegmentation;

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn get_char_by_ascii() {
    let text = "ABCDEFG";
    let c = text.as_bytes()[2];
    let ch = std::char::from_u32(c.into()).unwrap();
    println!("{} {}", c, ch);
    print_typename(c);
    print_typename(ch);
}

fn get_char_by_char() {
    let text = "🍣🍺大好き💖💖";
    let c = text.chars().nth(2).unwrap();
    println!("{}", c);
}

fn get_char_by_vec_char() {
    let text = "🍣🍺大好き💖💖";
    let cs = text.chars().collect::<Vec<_>>();
    let c = &cs[2];
    println!("{}", c); 
}

fn get_char_by_grapheme() {
    let text = "กำ🤷🏽‍♀️กำ🤷🏽‍♀️กำ🤷🏽‍♀️";
    let c = text.graphemes(true).nth(2).unwrap();
    println!("{}", c); 
}

fn get_char_by_vec_grapheme() {
    let text = "กำ🤷🏽‍♀️กำ🤷🏽‍♀️กำ🤷🏽‍♀️";
    let cs = text.graphemes(true).collect::<Vec<_>>();
    let c = &cs[2];
    println!("{}", c); 
}

fn main() {
    get_char_by_ascii();
    get_char_by_char();
    get_char_by_vec_char();
    get_char_by_grapheme();
    get_char_by_vec_grapheme();
}
