//! 📜️ 📜️ Trinity Rewriting app command — `set-lhs-json`.

use crate::artifacts::rewriting::rewriting_snapshot_mutations;
use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_lhs_json(state: &RewritingSnapshot, value: &str) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    let mut next = state.clone();
    next.lhs_json = value.to_string();
    Ok(Emit::mutations(rewriting_snapshot_mutations(state, &next)))
}
