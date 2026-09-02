//! 📜️ 📜️ Trinity Rewrite app command — `reset-rule`.

use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfigMutation;
use semio_framework_plugin::{Emit, Fault};

/// 🧬️ `resetRule` is a genuine whole-document reset (back to the blank default rule) — not
/// expressible as a granular mutation, so it routes through `Effect::LoadDocument` (outside
/// undo history) via `editor::rewrite::reset_document_effect`, mirroring `set_active_example`/
/// `set_fixture_json` conventions elsewhere in this ticket.
pub(crate) fn reset_rule(state: &RewriteSnapshot) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let next = crate::editor::rewrite::default_rule_state();
    let camera = crate::editor::rewrite::seed_before_pane_camera(&next);
    let config_mutations = vec![RewriteConfigMutation::SetBeforePaneCamera { camera }];
    if &next == state {
        Ok(Emit::config(config_mutations))
    } else {
        Ok(Emit { effects: vec![crate::editor::rewrite::reset_document_effect(&next)], config_mutations, ..Default::default() })
    }
}
