//! 🧬️ Opening-preferences config mutation dispatch enum — `os.config.opening`'s two handcrafted
//! kinds, each a single-field tuple wrapping its `🧬️mutations/<slug>/` triad payload. Hand-written
//! dispatch (not `#[derive(dsl::Mutations)]`) because this facet's `Diff` is the whole
//! `OpeningPreferences` value (see `📌️set-default-app`/`🧹clear-default-app`'s `🔺️diff` leaves),
//! matching `📕️norm`'s `NormConfigMutation` precedent rather than a sparse per-field diff type.

// 🩹️ W1-A wiring fix (this facet was authored "not yet wired into any crate's glue.rs" — see the
// module doc above): one hop short. Under the mount this facet's own leaf files already assume
// (`🧬️mutations` a real module wrapping this dispatch enum, itself inside the schema module — the
// SAME depth `📌️set-default-app`/`🧹clear-default-app`'s `use super::super::OpeningConfigMutation;`
// already require two supers to reach), this file's own module sits ONE level inside `🧬️mutations`,
// so `DefaultApp`/`OpeningPreferences` (declared at the schema level) need `super::super`, not `super`.
use super::super::OpeningPreferences;
use protocol::{Mutation, MutationDiff, MutationKind, MutationOutcome};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// @emoji 🧬️ Typed, invertible opening-preferences mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum OpeningConfigMutation {
    SetDefaultApp(SetDefaultApp),
    ClearDefaultApp(ClearDefaultApp),
}

impl Mutation<OpeningPreferences> for OpeningConfigMutation {
    type Diff = OpeningPreferences;

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): both leaves already return `MutationOutcome<OpeningPreferences>` (`Self::Diff`
    /// here IS `OpeningPreferences`, so no `.map` needed), forwarded as-is.
    fn diff(&self, base: &OpeningPreferences) -> MutationOutcome<OpeningPreferences> {
        match self {
            OpeningConfigMutation::SetDefaultApp(op) => op.diff(base),
            OpeningConfigMutation::ClearDefaultApp(op) => op.diff(base),
        }
    }

    fn inverse(&self, base: &OpeningPreferences) -> Vec<Self> {
        match self {
            OpeningConfigMutation::SetDefaultApp(op) => op.inverse(base),
            OpeningConfigMutation::ClearDefaultApp(op) => op.inverse(base),
        }
    }
}

/// 🧮️ Diff-first apply — `operation.diff(base).apply(base)`, matching every other migrated facet.
pub fn apply_opening_config_mutation(snapshot: &mut OpeningPreferences, mutation: &OpeningConfigMutation) -> protocol::MutationApplyResult<()> {
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

pub fn inverse_opening_config_mutation(snapshot: &OpeningPreferences, mutation: &OpeningConfigMutation) -> Vec<OpeningConfigMutation> {
    mutation.inverse(snapshot)
}

pub use super::set_default_app::mutation::{set_default_app, SetDefaultApp};
pub use super::clear_default_app::mutation::{clear_default_app, ClearDefaultApp};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::{AppRef, AppRole, ArtifactDialect};

    #[test]
    fn set_default_app_and_clear_default_app_invert_each_other() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
        let base = OpeningPreferences::default();

        let set_op = OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() });
        let after_set = set_op.diff(&base).diff().apply(&base).expect("valid set-default diff");
        assert_eq!(after_set.defaults, vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }]);

        let undo = set_op.inverse(&base);
        assert_eq!(undo, vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect: dialect.clone(), role: AppRole::Editor })]);
        let restored = undo[0].diff(&after_set).diff().apply(&after_set);
        assert_eq!(restored, Ok(base));
    }

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// `set`/`clear` have no missing-target or Fatal-domain case (an upsert and an idempotent
    /// removal never reference an external entity that could be absent or invariant-violating), so
    /// the only real outcome-law surface here is `mutation.no-op` on the idempotent path.
    #[test]
    fn set_default_app_already_pinned_is_no_op() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
        let base = OpeningPreferences { defaults: vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }] };
        let outcome = OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect, role: AppRole::Editor, app }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &base);
    }

    #[test]
    fn clear_default_app_without_a_pin_is_no_op() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let base = OpeningPreferences::default();
        let outcome = OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role: AppRole::Viewer }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &base);
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
