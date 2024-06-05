use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    let rev = input.graphemes(true).rev().collect::<String>();
    rev
}
