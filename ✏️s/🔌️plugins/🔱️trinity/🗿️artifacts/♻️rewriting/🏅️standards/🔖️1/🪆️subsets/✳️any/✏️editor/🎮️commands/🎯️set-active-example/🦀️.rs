//! 🎬️ 🎬️ Trinity Rewriting app command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};
use store::ArtifactDsl;

/// 🎬️ The ids this verb answers. The navbar example picker dispatches a REGISTERED example id, and
/// `demo` is the only example this subset registers, so that id must resolve or every navbar pick is
/// inert. `default` names the blank rule `resetRule` builds, for the catalogue's own rows.
pub(crate) fn set_active_example_document(example_id: &str) -> Option<RewritingSnapshot> {
    match example_id {
        "default" | "blank" => Some(crate::editor::rewriting::default_rule_state()),
        id if id == crate::examples::demo::ID => RewritingSnapshot::parse_dsl(crate::examples::demo::PRIMARY_TEXT).ok(),
        _ => None,
    }
}

/// 🧬️ Whole-document swap, so it routes through `Effect::LoadDocument` exactly as `reset_rule` does —
/// `SetState` is forbidden vocabulary in the mutation enum.
pub(crate) fn set_active_example(example_id: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    match set_active_example_document(example_id) {
        Some(next) => Emit { effects: vec![crate::editor::rewriting::reset_document_effect(&next)], ..Default::default() },
        None => Emit::default(),
    }
}
