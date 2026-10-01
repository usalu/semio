//! 📜️ 📜️ Trinity Rewriting app command — `set-lhs-json`.

use crate::standards::v1::subsets::any::schema::mutations::edit_lhs;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

/// 👈️ The rule's left-hand side replaced by `value` as ONE `edit-lhs` leaf (its input is the whole pattern JSON, structured text
/// applied explicitly); the unchanged side moves nothing.
pub(crate) fn set_lhs_json(state: &RewritingSnapshot, value: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    match state.lhs_json == value {
        true => Emit::default(),
        false => Emit::mutations(vec![edit_lhs(value.to_string())]),
    }
}
