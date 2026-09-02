//! 📜️ 📜️ Trinity Rewriting app command — `reset-rule`.

use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

/// 🧬️ `resetRule` is a genuine whole-document reset (back to the blank default rule) — not
/// expressible as a granular mutation, so it routes through `Effect::LoadDocument` (outside
/// undo history) via `editor::rewriting::reset_document_effect`, mirroring `set_active_example`/
/// `set_fixture_json` conventions elsewhere in this ticket.
pub(crate) fn reset_rule(state: &RewritingSnapshot) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    let next = crate::editor::rewriting::default_rule_state();
    let camera = crate::editor::rewriting::seed_before_pane_camera(&next);
    let config_mutations = vec![RewritingConfigMutation::SetBeforePaneCamera { camera }];
    if &next == state {
        Ok(Emit::config(config_mutations))
    } else {
        Ok(Emit { effects: vec![crate::editor::rewriting::reset_document_effect(&next)], config_mutations, ..Default::default() })
    }
}
