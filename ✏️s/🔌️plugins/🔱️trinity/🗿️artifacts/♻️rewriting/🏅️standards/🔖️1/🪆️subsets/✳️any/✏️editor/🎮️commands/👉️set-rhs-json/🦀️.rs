//! 📜️ 📜️ Trinity Rewriting app command — `set-rhs-json`.

use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

pub(crate) fn set_rhs_json(state: &RewritingSnapshot, value: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    let mut next = state.clone();
    next.rhs_json = value.to_string();
    next.parameter_bindings = crate::editor::rewriting::default_parameter_bindings(&next.rhs_json);
    Emit::mutations(rewriting_snapshot_mutations(state, &next))
}
