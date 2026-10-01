//! 📜️ 📜️ Trinity Rewriting app command — `set-rhs-json`.

use crate::standards::v1::subsets::any::schema::mutations::edit_rhs;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

/// 👉️ The rule's right-hand side replaced by `value` as ONE `edit-rhs` leaf, followed by the parameter-binding leaves that reset
/// every binding to the new side's declared defaults ([`crate::editor::rewriting::parameter_binding_mutations`]); the unchanged
/// side moves nothing.
pub(crate) fn set_rhs_json(state: &RewritingSnapshot, value: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    if state.rhs_json == value {
        return Emit::default();
    }
    let defaults = crate::editor::rewriting::default_parameter_bindings(value);
    Emit::mutations(std::iter::once(edit_rhs(value.to_string())).chain(crate::editor::rewriting::parameter_binding_mutations(&state.parameter_bindings, &defaults)).collect())
}
