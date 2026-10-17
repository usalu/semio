use protocol::MutationKind;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_framework_test_fixtures::fixture_bytes;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
#[allow(unused_imports)]
use protocol::{MutationOutcome, SemanticDescriptor};
use crate::standards::v1_7::subsets::base::schema::mutations::insert_content::*;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::testkit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {
    assert_eq!(<InsertContent as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "insert-content");
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🖋️insert-content/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🖋️insert-content/➡️after.pdf"));
    let (at, _, content) = testkit::content_edit(&before, &after);
    assert_mutation_inverse_sum_law(&PdfMutation::InsertContent(InsertContent { index: 0, at, content }), &before).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let (mut base, after) = testkit::snapshots(fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🖋️insert-content/⬅️before.pdf"), fixture_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🖋️insert-content/➡️after.pdf"));
    let (at, _, content) = testkit::content_edit(&base, &after);
    let op = content[0].clone();
    base.pages[0].content.insert(0, op.clone());
    base.pages[0].content.push(op);
    assert_mutation_inverse_sum_law(&PdfMutation::InsertContent(InsertContent { index: 0, at: at + 1, content }), &base).await;
}
