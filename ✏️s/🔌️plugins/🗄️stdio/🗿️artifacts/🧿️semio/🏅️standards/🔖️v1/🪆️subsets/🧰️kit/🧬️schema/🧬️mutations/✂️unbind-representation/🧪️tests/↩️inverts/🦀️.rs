//! 🧪️ `unbind-representation` inverse law: unbinding ANY representation (first, middle, last) is undone by one `bind-representation` at its original
//! index, and the inverse rows' diffs sum to the negative of the forward diff.

use crate::standards::v1::subsets::kit::schema::mutations::{unbind_representation, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::demo_kit_snapshot;

#[semio_framework_async_macros::async_test]
async fn unbinding_any_representation_inverts_at_its_index() {
    let mut base = demo_kit_snapshot();
    let template = base.representations[0].clone();
    for n in 2..=3 {
        let mut link = template.clone();
        link.target.artifact_id = format!("repr-{n}");
        base.representations.push(link);
    }
    for index in 0..base.representations.len() {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index }), &base).await;
    }
}
