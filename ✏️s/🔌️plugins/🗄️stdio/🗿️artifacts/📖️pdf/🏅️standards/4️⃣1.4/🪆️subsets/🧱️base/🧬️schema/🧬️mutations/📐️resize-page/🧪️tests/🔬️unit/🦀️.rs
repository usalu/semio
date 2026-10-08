use super::*;
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
use crate::standards::v1_4::subsets::base::io::binary::mutations as binary;
use protocol::Mutation;

/// 🚫️ The refusal branch the committed vector cannot express, because a refused mutation
/// produces no after-state to commit: addressed against a document with no pages at all,
/// `resize-page` must raise, leave the document untouched, and offer no undo.
#[test]
fn missing_page_refuses_without_inverse_or_state_change() {
    let mutation: PdfMutation = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️resize-page/🔄️round/🦠️mutation/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed resize-page payload decodes");
    let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
    let mut state = base.clone();
    assert!(!crate::standards::v1_4::subsets::base::io::mutation_bridge::apply_outcome(mutation.diff(&state), &mut state).messages().is_empty(), "resize-page: an unaddressable page must be refused");
    assert_eq!(state, base, "resize-page: a refused mutation must leave the document untouched");
    assert!(mutation.inverse(&base).expect("valid retained mutation inverse fixture").is_empty(), "resize-page: a refused mutation has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "first".to_string() }, PageDoc { width: 100.0, height: 200.0, text: "second".to_string() }], ..Default::default() };
    assert_mutation_inverse_sum_law(&PdfMutation::ResizePage(ResizePage { index: 1, width: 300.0, height: 400.0 }), &base).await;
}
