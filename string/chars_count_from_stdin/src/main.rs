use unicode_segmentation::UnicodeSegmentation;

fn read_line_stdin() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Input error.");
    s
}

fn main() {
    let s = read_line_stdin();
    let s2 = s.trim_end_matches('\n');
    println!("{}", s2.len());
    println!("{}", s2.chars().count());
    println!("{}", s2.graphemes(true).count());
}
