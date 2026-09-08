
use super::*;

#[semio_framework_async_macros::async_test]
async fn counts_elements_depth_and_text_over_nested_structure() {
    let root = HtmlNode::Element { name: "html".into(), attributes: vec![], children: vec![HtmlNode::Element { name: "body".into(), attributes: vec![], children: vec![HtmlNode::Text { text: "hello".into() }] }] };
    let snapshot = HtmlSnapshot { schema: "stdio.html".into(), doctype: None, root };
    let outline = HtmlOutline::compute(&snapshot);
    assert_eq!(outline.element_count, 2);
    assert_eq!(outline.max_depth, 2);
    assert_eq!(outline.text_length, 5);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_one_element_and_no_text() {
    let outline = HtmlOutline::compute(&HtmlSnapshot::default());
    assert_eq!(outline.element_count, 1);
    assert_eq!(outline.max_depth, 1);
    assert_eq!(outline.text_length, 0);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = HtmlSnapshot::default();
    assert_eq!(HtmlOutline::compute(&snapshot), HtmlOutline::compute(&snapshot));
}
