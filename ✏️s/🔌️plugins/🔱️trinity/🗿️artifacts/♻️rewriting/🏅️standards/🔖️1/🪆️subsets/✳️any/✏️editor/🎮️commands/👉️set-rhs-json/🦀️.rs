//! 📜️ 📜️ Trinity Rewriting app command — `set-rhs-json`.

use crate::artifacts::rewriting::rewriting_snapshot_mutations;
use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_rhs_json(state: &RewritingSnapshot, value: &str) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    let mut next = state.clone();
    next.rhs_json = value.to_string();
    next.parameter_bindings = crate::editor::rewriting::default_parameter_bindings(&next.rhs_json);
    Ok(Emit::mutations(rewriting_snapshot_mutations(state, &next)))
}
