//! 🌉️ Central apply entry points: the only code of this artifact that turns a mutation's diff into the next snapshot with `protocol::apply_diff`; the schema and mutation leaves only build diffs and inverses.
#![allow(unused_imports)]

use crate::schema::mutations::SourcingMutation;
use crate::CurationSnapshot;

/// ▶️ Applies `mutation` in place and returns every diagnostic it raised as `(code, severity)`
/// pairs, so the committed `🎯️outcome/🔣️.json`'s claim is checkable from outside this
/// crate rather than only inside its own leaf tests.
pub fn apply_sourcing_mutation_reporting(snapshot: &mut CurationSnapshot, mutation: &SourcingMutation) -> Vec<(String, String)> {
    let outcome = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), &*snapshot) {
        *snapshot = next;
    }
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}
