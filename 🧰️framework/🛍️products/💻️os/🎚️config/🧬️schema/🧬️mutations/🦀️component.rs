//! 🧬️ Opening-preferences config mutation dispatch enum — `os.config.opening`'s two handcrafted
//! kinds, each a single-field tuple wrapping its `🧬️mutations/<slug>/` triad payload. Hand-written
//! dispatch (not `#[derive(dsl::Mutations)]`) because this facet's `Diff` is the whole
//! `OpeningPreferences` value (see `📌️set-default-app`/`🧹clear-default-app`'s `🔺️diff` leaves),
//! matching `📕️norm`'s `NormConfigMutation` precedent rather than a sparse per-field diff type.

use super::{DefaultApp, OpeningPreferences};
use protocol::{Mutation, MutationDiff, MutationKind};
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

    fn diff(&self, base: &OpeningPreferences) -> OpeningPreferences {
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
pub fn apply_opening_config_mutation(snapshot: &mut OpeningPreferences, mutation: &OpeningConfigMutation) {
    *snapshot = mutation.diff(snapshot).apply(snapshot);
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
        let after_set = set_op.diff(&base).apply(&base);
        assert_eq!(after_set.defaults, vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }]);

        let undo = set_op.inverse(&base);
        assert_eq!(undo, vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect: dialect.clone(), role: AppRole::Editor })]);
        let restored = undo[0].diff(&after_set).apply(&after_set);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
