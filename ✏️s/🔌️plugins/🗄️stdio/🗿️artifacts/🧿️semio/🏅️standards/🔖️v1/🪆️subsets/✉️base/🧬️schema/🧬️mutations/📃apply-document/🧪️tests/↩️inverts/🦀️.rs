//! 🧪️ `apply-document` inverse law on its committed wrapped-arm vector: the wrapped mutation is undone by its own inverse rows, replayed
//! last-to-first, and those rows' diffs sum to the negative of the forward diff.

use crate::standards::v1::subsets::base::io::text::mutations::decode_semio_mutation_json;
use crate::standards::v1::subsets::base::io::text::snapshot::decode_semio_snapshot_json;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/📑️apply-document-applied/⬅️before.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/📑️apply-document-applied/🦠️mutation.json");

#[semio_framework_async_macros::async_test]
async fn the_wrapped_document_mutation_inverts_and_sums_to_the_negative_diff() {
    let base = decode_semio_snapshot_json(BEFORE).expect("committed before-envelope decodes");
    let mutation = decode_semio_mutation_json(MUTATION).expect("committed wrapped mutation decodes");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
