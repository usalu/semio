use super::super::super::{MiniDoc, MiniMutation};
use super::RenameMini;

/// ➕️ L3: renaming to a new name, to the same name and to the empty name sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = MiniDoc { name: "alpha".into() };
    for new_name in ["beta", "alpha", ""] {
        semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law(&MiniMutation::RenameMini(RenameMini { new_name: new_name.into() }), &base).await;
    }
}
