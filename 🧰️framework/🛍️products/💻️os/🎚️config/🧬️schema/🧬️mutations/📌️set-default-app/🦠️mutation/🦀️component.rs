//! 📌️ Opening-preferences mutation — `SetDefaultApp` payload: pins one viewer/editor default for
//! a `(dialect, role)` coordinate.

use super::super::super::OpeningPreferences;
use super::super::OpeningConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework::{AppRef, AppRole, ArtifactDialect};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📌️ Pins `app` as the default `role` surface for `dialect`, replacing any prior pin for the
/// same coordinate. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
    pub app: AppRef,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_default_app(dialect: ArtifactDialect, role: AppRole, app: AppRef) -> OpeningConfigMutation {
    OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect, role, app })
}

impl MutationKind<OpeningPreferences, OpeningConfigMutation> for SetDefaultApp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "default-app", kind: "set-default-app", record: "Set" };

    fn diff(&self, base: &OpeningPreferences) -> MutationOutcome<OpeningPreferences> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &OpeningPreferences) -> Vec<OpeningConfigMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        let role = match self.role {
            AppRole::Viewer => "viewer",
            AppRole::Editor => "editor",
        };
        format!("Set default {} for \"{}\"", role, self.dialect.to_coordinate())
    }

    fn target(&self) -> Vec<String> {
        let role = match self.role {
            AppRole::Viewer => "viewer",
            AppRole::Editor => "editor",
        };
        vec![self.dialect.to_coordinate(), role.to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_default_app_label_names_role_and_dialect() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() } };
        assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), "Set default editor for \"s.cad.cad@1/*\"");
    }
}
//#endregion 🧪️Tests
