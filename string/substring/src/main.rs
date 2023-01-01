use unicode_segmentation::UnicodeSegmentation;

fn substring_by_ascii() {
    let text = "ABCDEFG";
    let s = &text[2..5].to_string();
    println!("{}", s); 
}

fn substring_by_char() {
    let text = "🍣🍺大好き💖💖";
    let s: String = text.chars().skip(2).take(3).collect();
    println!("{}", s);
}

fn substring_by_vec_char() {
    let text = "🍣🍺大好き💖💖";
    let cs = text.chars().collect::<Vec<_>>();
    let s = String::from_iter(&cs[2..5]);
    println!("{}", s); 
}

fn substring_by_grapheme() {
    let text = "กำ🤷🏽‍♀️กำ🤷🏽‍♀️กำ🤷🏽‍♀️";
    let s = text.graphemes(true).skip(2).take(3).collect::<Vec<_>>().join("");
    println!("{}", s); 
}

fn substring_by_vec_grapheme() {
    let text = "กำ🤷🏽‍♀️กำ🤷🏽‍♀️กำ🤷🏽‍♀️";
    let cs = text.graphemes(true).collect::<Vec<_>>();
    let s = &cs[2..5].join("");
    println!("{}", s); 
}

fn main() {
    substring_by_ascii();
    substring_by_char();
    substring_by_vec_char();
    substring_by_grapheme();
    substring_by_vec_grapheme();
}
