use super::*;

#[semio_framework_async_macros::async_test]
async fn jack_diff_absorb_merges_content() {
    let mut diff = JackDiff::default();
    let other = diff_replace_content(vec![Node { id: "x".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: Default::default(), ports: vec![] }], vec![]);
    diff.absorb(other.clone());
    assert_eq!(diff.content, other.content);
}
