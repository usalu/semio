
use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;

#[test]
fn document_tree_lists_the_seeded_parts_section() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-document.parts"));
}
