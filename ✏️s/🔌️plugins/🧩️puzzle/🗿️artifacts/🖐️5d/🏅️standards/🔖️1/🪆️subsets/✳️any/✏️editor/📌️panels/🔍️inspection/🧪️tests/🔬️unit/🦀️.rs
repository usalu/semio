
use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;

#[test]
fn empty_selection_renders_the_document_summary() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-inspector.empty"));
}
