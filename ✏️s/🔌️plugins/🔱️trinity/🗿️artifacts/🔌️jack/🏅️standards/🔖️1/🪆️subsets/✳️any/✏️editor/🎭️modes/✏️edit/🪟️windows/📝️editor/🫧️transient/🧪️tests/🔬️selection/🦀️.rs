//! 🧪️ `set-editor-selection` keeps its sparse diff and concrete inverse in step.

use super::*;

/// ➕️ The inverse row sums to exactly the negative of the diff (law L3), for a set and for a clear.
#[semio_framework_async_macros::async_test]
async fn set_editor_selection_inverse_sums_to_the_negative_diff() {
    let base = JackEditorWindowTransient { selection: Some(JackEditorSelection { start: 1, end: 4 }) };
    for selection in [Some(JackEditorSelection { start: 2, end: 9 }), None] {
        let mutation: JackEditorWindowTransientMutation = SetEditorSelection { selection }.into();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
