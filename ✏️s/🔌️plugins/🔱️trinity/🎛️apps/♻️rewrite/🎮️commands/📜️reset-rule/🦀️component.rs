//! 📜️ 📜️ Trinity Rewrite app command — `reset-rule`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::{Graph, JackSnapshot, PropertyValue};
use crate::artifacts::rewrite::schema::{ParameterKind, Rhs};
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};
use serde_json::Value;

/// 🧬️ `resetRule` is a genuine whole-document reset (back to the blank default rule) — not
/// expressible as a granular mutation, so it routes through `HostEffect::LoadDocument` (outside
/// undo history) via `apps::rewrite::reset_document_effect`, mirroring `set_active_example`/
/// `set_fixture_json` conventions elsewhere in this ticket.
pub(crate) fn reset_rule(state: &RewriteSnapshot) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let next = crate::apps::rewrite::default_rule_state();
    let camera = crate::apps::rewrite::seed_before_pane_camera(&next);
    let config_mutations = vec![RewriteConfigMutation::SetBeforePaneCamera { camera }];
    if &next == state {
        Ok(Emit::config(config_mutations))
    } else {
        Ok(Emit { effects: vec![crate::apps::rewrite::reset_document_effect(&next)], config_mutations, ..Default::default() })
    }
}
