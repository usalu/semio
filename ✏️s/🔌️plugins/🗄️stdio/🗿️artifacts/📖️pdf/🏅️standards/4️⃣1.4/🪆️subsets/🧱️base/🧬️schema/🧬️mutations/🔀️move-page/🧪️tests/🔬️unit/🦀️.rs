use super::*;
use protocol::Mutation;

/// 🚫️ The refusal branch the committed vector cannot express, because a refused mutation
/// produces no after-state to commit: addressed against a document with no pages at all,
/// `move-page` must raise, leave the document untouched, and offer no undo.
#[test]
fn missing_page_refuses_without_inverse_or_state_change() {
    let mutation: PdfMutation = dsl::json::from_json_str(include_str!("../🔄️round-trips-the-concrete-inverse/🦠️mutation/🔣️.json")).expect("committed move-page payload decodes");
    let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
    let mut state = base.clone();
    assert!(!mutation.diff(&state).apply_to(&mut state).messages().is_empty(), "move-page: an unaddressable page must be refused");
    assert_eq!(state, base, "move-page: a refused mutation must leave the document untouched");
    assert!(mutation.inverse(&base).is_empty(), "move-page: a refused mutation has nothing to undo");
}
