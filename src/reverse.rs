pub mod reverse {
    use unicode_segmentation::UnicodeSegmentation;

    pub fn reverse(input: &str) -> String {
        //input.chars().rev().collect::<String>()
        input.graphemes(true).rev().collect::<String>()
    }
}

#[test]
fn test_empty_string() {
    assert_eq!("cool", reverse::reverse("looc"));
    assert_eq!("uüu", reverse::reverse("uüu"));
}
