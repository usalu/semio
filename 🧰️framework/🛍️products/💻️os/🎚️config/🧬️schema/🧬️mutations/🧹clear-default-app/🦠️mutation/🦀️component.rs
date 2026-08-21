//! 🧹 Opening-preferences mutation — `ClearDefaultApp` payload: unpins a `(dialect, role)`
//! coordinate, falling back to the `OpeningResolver`'s owner/router order.

use super::super::super::OpeningPreferences;
use super::super::OpeningConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework::{AppRole, ArtifactDialect};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧹 Removes the pinned default for `(dialect, role)`, if any. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearDefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn clear_default_app(dialect: ArtifactDialect, role: AppRole) -> OpeningConfigMutation {
    OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role })
}

impl MutationKind<OpeningPreferences, OpeningConfigMutation> for ClearDefaultApp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "default-app", kind: "clear-default-app", record: "Cleared" };

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
        format!("Clear default {} for \"{}\"", role, self.dialect.to_coordinate())
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
    fn clear_default_app_label_names_role_and_dialect() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let payload = ClearDefaultApp { dialect: dialect.clone(), role: AppRole::Viewer };
        assert_eq!(MutationKind::<OpeningPreferences, OpeningConfigMutation>::label(&payload), "Clear default viewer for \"s.cad.cad@1/*\"");
    }
}
//#endregion 🧪️Tests
