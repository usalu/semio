use super::*;

#[semio_framework_async_macros::async_test]
async fn counts_lines_words_and_chars() {
    let snapshot = TxtSnapshot { schema: "stdio.txt".into(), lines: vec!["hello world".into(), "one two three".into()], trailing_newline: true, line_ending: Default::default() };
    let outline = TxtOutline::compute(&snapshot);
    assert_eq!(outline.line_count, 2);
    assert_eq!(outline.word_count, 5);
    assert_eq!(outline.char_count, "hello world".chars().count() as u32 + "one two three".chars().count() as u32);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = TxtSnapshot::default();
    assert_eq!(TxtOutline::compute(&snapshot), TxtOutline::compute(&snapshot));
}
