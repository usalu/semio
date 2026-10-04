use super::*;
use protocol::Mutation;

/// 🚫️ The refusal branch the committed vector cannot express, because a refused mutation
/// produces no after-state to commit: addressed against a document with no pages at all,
/// `replace-page-text` must raise, leave the document untouched, and offer no undo.
#[test]
fn missing_page_refuses_without_inverse_or_state_change() {
    let mutation: PdfMutation = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/♻️replace-page-text/🔄️round/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-page-text payload decodes");
    let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
    let mut state = base.clone();
    assert!(!mutation.diff(&state).apply_to(&mut state).messages().is_empty(), "replace-page-text: an unaddressable page must be refused");
    assert_eq!(state, base, "replace-page-text: a refused mutation must leave the document untouched");
    assert!(mutation.inverse(&base).expect("valid retained mutation inverse fixture").is_empty(), "replace-page-text: a refused mutation has nothing to undo");
}
