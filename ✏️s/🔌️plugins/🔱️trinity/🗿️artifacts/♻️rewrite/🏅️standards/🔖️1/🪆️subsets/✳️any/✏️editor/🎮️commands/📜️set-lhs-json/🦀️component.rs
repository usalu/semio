//! 📜️ 📜️ Trinity Rewrite app command — `set-lhs-json`.

use crate::editor::rewrite::config::RewriteConfigMutation;
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn set_lhs_json(state: &RewriteSnapshot, value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut next = state.clone();
    next.lhs_json = value.to_string();
    Ok(Emit::mutations(rewrite_snapshot_mutations(state, &next)))
}
