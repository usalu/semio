//! 📌️ `SetDefaultApp` is the authoritative direct leaf for pinning one viewer/editor default.

use super::super::{DefaultApp, OpeningPreferences};
use super::clear_default_app::ClearDefaultApp;
use super::OpeningConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework::{AppRef, AppRole, ArtifactDialect};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 📌️ Pins `app` for one `(dialect, role)` coordinate, replacing any prior pin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SetDefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
    pub app: AppRef,
}

/// 🏗️ Wraps a set-default-app payload in the opening-config dispatch enum.
pub fn set_default_app(dialect: ArtifactDialect, role: AppRole, app: AppRef) -> OpeningConfigMutation {
    OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect, role, app })
}

impl MutationKind<OpeningPreferences, OpeningConfigMutation> for SetDefaultApp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "default-app", kind: "set-default-app", record: "Set" };

    fn diff(&self, base: &OpeningPreferences) -> MutationOutcome<OpeningPreferences> {
        if base.defaults.iter().any(|entry| entry.dialect == self.dialect && entry.role == self.role && entry.app == self.app) {
            let role = role_name(self.role);
            return MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" is already the default {} for \"{}\".", self.app.app_id, role, self.dialect.to_coordinate()));
        }
        let mut defaults: Vec<DefaultApp> = base.defaults.iter().filter(|entry| !(entry.dialect == self.dialect && entry.role == self.role)).cloned().collect();
        defaults.push(DefaultApp { dialect: self.dialect.clone(), role: self.role, app: self.app.clone() });
        MutationOutcome::new(OpeningPreferences { defaults })
    }

    fn inverse(&self, base: &OpeningPreferences) -> Vec<OpeningConfigMutation> {
        match base.defaults.iter().find(|entry| entry.dialect == self.dialect && entry.role == self.role) {
            Some(prior) => vec![OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: self.dialect.clone(), role: self.role, app: prior.app.clone() })],
            None => vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect: self.dialect.clone(), role: self.role })],
        }
    }

    fn label(&self) -> String {
        format!("Set default {} for \"{}\"", role_name(self.role), self.dialect.to_coordinate())
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
#[path = "🧪️tests/repins-the-cad-editor-to-the-drafting-app/🦀️.rs"]
mod tests_repins_the_cad_editor_to_the_drafting_app;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_names_role_and_dialect() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = SetDefaultApp { dialect, role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
        assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), "Set default editor for \"s.cad.cad@1/*\"");
    }

    #[test]
    fn unpinned_coordinate_inverts_to_clear() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
        assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::inverse(&payload, &OpeningPreferences::default()), vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role: AppRole::Editor })]);
    }

    #[test]
    fn already_pinned_app_is_a_warned_no_op() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
        let base = OpeningPreferences { defaults: vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }] };
        let payload = SetDefaultApp { dialect, role: AppRole::Editor, app };
        let outcome = MutationKind::<OpeningPreferences, OpeningConfigMutation>::diff(&payload, &base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &base);
    }
}
//#endregion 🧪️Tests
