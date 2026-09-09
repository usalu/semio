//! 📜️ 📜️ Trinity Rewriting app command — `reset-rule`.

use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

/// 🧬️ `resetRule` is a genuine whole-document reset (back to the blank default rule) — not
/// expressible as a granular mutation, so it routes through `Effect::LoadDocument` (outside
/// undo history) via `editor::rewriting::reset_document_effect`, mirroring `set_active_example`/
/// `set_fixture_json` conventions elsewhere in this ticket.
pub(crate) fn reset_rule(state: &RewritingSnapshot) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    let next = crate::editor::rewriting::default_rule_state();
    if &next == state {
        Emit::default()
    } else {
        Emit { effects: vec![crate::editor::rewriting::reset_document_effect(&next)], ..Default::default() }
    }
}
