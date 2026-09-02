//! 🧹 `ClearDefaultApp` is the authoritative direct leaf for unpinning one viewer/editor default.

use super::super::OpeningPreferences;
use super::set_default_app::SetDefaultApp;
use super::OpeningConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework::{AppRole, ArtifactDialect};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧹 Removes the pinned default for one `(dialect, role)` coordinate, if present.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ClearDefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
}

/// 🏗️ Wraps a clear-default-app payload in the opening-config dispatch enum.
pub fn clear_default_app(dialect: ArtifactDialect, role: AppRole) -> OpeningConfigMutation {
    OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role })
}

impl MutationKind<OpeningPreferences, OpeningConfigMutation> for ClearDefaultApp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "default-app", kind: "clear-default-app", record: "Cleared" };

    fn diff(&self, base: &OpeningPreferences) -> MutationOutcome<OpeningPreferences> {
        if !base.defaults.iter().any(|entry| entry.dialect == self.dialect && entry.role == self.role) {
            let role = role_name(self.role);
            return MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" has no pinned default {} to clear.", self.dialect.to_coordinate(), role));
        }
        let defaults = base.defaults.iter().filter(|entry| !(entry.dialect == self.dialect && entry.role == self.role)).cloned().collect();
        MutationOutcome::new(OpeningPreferences { defaults })
    }

    fn inverse(&self, base: &OpeningPreferences) -> Vec<OpeningConfigMutation> {
        base.defaults
            .iter()
            .find(|entry| entry.dialect == self.dialect && entry.role == self.role)
            .map(|prior| vec![OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: self.dialect.clone(), role: self.role, app: prior.app.clone() })])
            .unwrap_or_default()
    }

    fn label(&self) -> String {
        format!("Clear default {} for \"{}\"", role_name(self.role), self.dialect.to_coordinate())
    }

    fn target(&self) -> Vec<String> {
        vec![self.dialect.to_coordinate(), role_name(self.role).to_string()]
    }
}

fn role_name(role: AppRole) -> &'static str {
    match role {
        AppRole::Viewer => "viewer",
        AppRole::Editor => "editor",
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/unpins-the-cad-editor-and-keeps-the-viewer-pin/🦀️.rs"]
mod tests_unpins_the_cad_editor_and_keeps_the_viewer_pin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_names_role_and_dialect() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = ClearDefaultApp { dialect, role: AppRole::Viewer };
        assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), "Clear default viewer for \"s.cad.cad@1/*\"");
    }

    #[test]
    fn absent_coordinate_has_no_inverse() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = ClearDefaultApp { dialect, role: AppRole::Viewer };
        assert!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::inverse(&payload, &OpeningPreferences::default()).is_empty());
    }

    #[test]
    fn absent_coordinate_is_a_warned_no_op() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = ClearDefaultApp { dialect, role: AppRole::Viewer };
        let base = OpeningPreferences::default();
        let outcome = MutationKind::<OpeningPreferences, OpeningConfigMutation>::diff(&payload, &base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &base);
    }
}
//#endregion 🧪️Tests
