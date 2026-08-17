//! 📜️ 📜️ Trinity Rewrite app command — `set-rhs-json`.

use crate::editor::rewrite::config::RewriteConfigMutation;
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_rhs_json(state: &RewriteSnapshot, value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut next = state.clone();
    next.rhs_json = value.to_string();
    next.parameter_bindings = crate::editor::rewrite::default_parameter_bindings(&next.rhs_json);
    Ok(Emit::mutations(rewrite_snapshot_mutations(state, &next)))
}
