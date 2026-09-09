//! 📜️ 📜️ Trinity Rewriting app command — `set-lhs-json`.

use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

pub(crate) fn set_lhs_json(state: &RewritingSnapshot, value: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    let mut next = state.clone();
    next.lhs_json = value.to_string();
    Emit::mutations(rewriting_snapshot_mutations(state, &next))
}
