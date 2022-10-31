use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "\u{304b}\u{3099}";
    println!("{}", s.len());
    println!("{}", s.chars().count());
    println!("{}", s.graphemes(true).count());
}
