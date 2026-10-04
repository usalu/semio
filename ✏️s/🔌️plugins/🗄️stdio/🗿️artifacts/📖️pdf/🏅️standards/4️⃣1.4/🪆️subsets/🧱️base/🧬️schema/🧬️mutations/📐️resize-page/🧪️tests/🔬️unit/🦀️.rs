use super::*;
use protocol::Mutation;

/// 🚫️ The refusal branch the committed vector cannot express, because a refused mutation
/// produces no after-state to commit: addressed against a document with no pages at all,
/// `resize-page` must raise, leave the document untouched, and offer no undo.
#[test]
fn missing_page_refuses_without_inverse_or_state_change() {
    let mutation: PdfMutation = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️resize-page/🔄️round/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed resize-page payload decodes");
    let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
    let mut state = base.clone();
    assert!(!mutation.diff(&state).apply_to(&mut state).messages().is_empty(), "resize-page: an unaddressable page must be refused");
    assert_eq!(state, base, "resize-page: a refused mutation must leave the document untouched");
    assert!(mutation.inverse(&base).expect("valid retained mutation inverse fixture").is_empty(), "resize-page: a refused mutation has nothing to undo");
}
