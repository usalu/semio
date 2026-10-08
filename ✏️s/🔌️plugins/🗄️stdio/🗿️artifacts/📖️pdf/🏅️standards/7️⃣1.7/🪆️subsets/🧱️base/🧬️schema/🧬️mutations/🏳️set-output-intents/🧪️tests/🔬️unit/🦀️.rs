use super::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<SetOutputIntents as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "set-output-intents");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = support::snapshots(include_bytes!("../../../../../🧫️fixtures/🏳️set-output-intents/⬅️before.pdf"), include_bytes!("../../../../../🧫️fixtures/🏳️set-output-intents/➡️after.pdf"));
    assert_mutation_inverse_sum_law(&PdfMutation::SetOutputIntents(SetOutputIntents { intents: after.output_intents.clone() }), &before).await;
}
