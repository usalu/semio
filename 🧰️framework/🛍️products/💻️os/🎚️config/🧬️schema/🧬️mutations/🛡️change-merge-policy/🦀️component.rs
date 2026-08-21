//! 🛡️ Merge-policy config facet — `os.config.merge-policy`: the authority-local `MergePolicy`
//! (`LaissezFaire | Normal | Vigilant`, `📡️spr/🧾️wire` region `🔖️Policies`) a user has chosen for
//! how strict conflict quarantine is on ingest. Schema + dispatch enum are folded into THIS file
//! (rather than split across a parent `🧬️schema`/`🧬️mutations` pair the way `OpeningPreferences`
//! is) because this triad's lease is scoped to `🧬️mutations/🛡️change-merge-policy/**` only — the
//! sibling `📌️set-default-app`/`🧹clear-default-app` triads and their shared parent files belong to
//! the concurrent `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket and are never touched here (see
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
//! 📋️ownership-and-handoffs.md`). Matches the same unwired-facet precedent those triads set — see
//! their parent `../../🦀️component.rs` doc comment ("NOT YET wired into any crate's `📦️glue.rs`").
//! Shell settings persist this event-sourced, then forward the resolved value to the channel as
//! `AppCommand::SetMergePolicy` (`📋️contract-freeze.md` §C8, tag 30) — see `Shell`/`ChromePanels`.

use protocol::{Mutation, MutationDiff, MutationOutcome};
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🛡️ `os.config.merge-policy` — the one authority-local `MergePolicy` choice, OS-wide.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MergePolicySetting {
    pub policy: protocol::MergePolicy,
}

/// 🪪️ The schema id this facet is registered under.
pub const MERGE_POLICY_CONFIG_SCHEMA: &str = "os.config.merge-policy";

/// 🧮️ Whole-record diff for `MergePolicyConfigMutation` — `apply` ignores `base` entirely, since
/// `ChangeMergePolicy`'s `🔺️diff` leaf already returns the full post-op setting (matches
/// `OpeningPreferences`'s precedent for a config facet this small).
impl MutationDiff<MergePolicySetting> for MergePolicySetting {
    fn apply(&self, _base: &MergePolicySetting) -> protocol::MutationApplyResult<MergePolicySetting> {
        Ok(*self)
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutations
/// @emoji 🛡️ Typed, invertible merge-policy mutation vocabulary — one kind today; more may join
/// this enum later without touching `ChangeMergePolicy` itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum MergePolicyConfigMutation {
    ChangeMergePolicy(ChangeMergePolicy),
}

impl Mutation<MergePolicySetting> for MergePolicyConfigMutation {
    type Diff = MergePolicySetting;

    fn diff(&self, base: &MergePolicySetting) -> MutationOutcome<MergePolicySetting> {
        match self {
            MergePolicyConfigMutation::ChangeMergePolicy(op) => op.diff(base),
        }
    }

    fn inverse(&self, base: &MergePolicySetting) -> Vec<Self> {
        match self {
            MergePolicyConfigMutation::ChangeMergePolicy(op) => op.inverse(base),
        }
    }
}

/// 🧮️ Diff-first apply — `operation.diff(base).diff().apply(base)`, matching every other migrated
/// config facet (`apply_opening_config_mutation`'s precedent).
pub fn apply_merge_policy_config_mutation(snapshot: &mut MergePolicySetting, mutation: &MergePolicyConfigMutation) -> protocol::MutationApplyResult<()> {
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

pub fn inverse_merge_policy_config_mutation(snapshot: &MergePolicySetting, mutation: &MergePolicyConfigMutation) -> Vec<MergePolicyConfigMutation> {
    mutation.inverse(snapshot)
}

pub use super::change_merge_policy::mutation::{change_merge_policy, ChangeMergePolicy};

/// 🧪️ Handcrafted mutation fixture for this facet's one kind (contract D1, ticket
/// `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Declared HERE rather than in a crate's `📦️glue.rs`
/// because this facet is still the deliberately unwired, self-contained file its module doc
/// describes — the fixture travels with it and starts running the moment the facet is mounted.
#[cfg(test)]
#[path = "🧪️tests/tightens-the-authority-to-vigilant/🦀️component.rs"]
mod tests_tightens_the_authority_to_vigilant;
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_policy_setting_default_is_normal() {
        assert_eq!(MergePolicySetting::default(), MergePolicySetting { policy: protocol::MergePolicy::Normal });
    }
}
//#endregion 🧪️Tests
