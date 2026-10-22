//! 🧪️ `set-map-entry` inverse law: every demo mutation of this kind that the committed base accepts is undone by its own inverse
//! rows, replayed last-to-first, and those rows' diffs sum to the negative of the forward diff.

use crate::base_fixture;
use protocol::{ColdMutation, Mutation};
use semio_s_artifact_stdio_semio_value_mutations::schema::mutations::{demo_mutation_cases, SemioValueMutation};

#[semio_framework_async_macros::async_test]
async fn demo_mutations_of_this_kind_invert_and_sum_to_the_negative_diff() {
    let base = base_fixture();
    let mut checked = 0usize;
    for mutation in demo_mutation_cases().into_iter().filter(|mutation| matches!(mutation, SemioValueMutation::SetMapEntry(_))) {
        let (_, messages) = <SemioValueMutation as ColdMutation<_>>::diff(&mutation, &base).into_parts();
        if messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
            continue;
        }
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
        checked += 1;
    }
    assert!(checked > 0, "no demo mutation of this kind applies cleanly to the committed base, so the inverse law was not exercised");
}
