//! 📌️ `SetDefaultApp` is the authoritative direct leaf for pinning one viewer/editor default.

use super::super::{DefaultAppPin, OpeningDiff, OpeningPreferences};
use super::clear_default_app::ClearDefaultApp;
use super::OpeningConfigMutation;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use {semio_framework::AppRef,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 📌️ Pins `app` for one `(dialect, role)` coordinate, replacing any prior pin.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
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

    fn diff(&self, base: &OpeningPreferences) -> MutationOutcome<OpeningDiff> {
        if base.defaults.iter().any(|entry| entry.dialect == self.dialect && entry.role == self.role && entry.app == self.app) {
            let role = role_name(self.role);
            return MutationOutcome::new(OpeningDiff::default()).warning("mutation.no-op", format!("\"{}\" is already the default {} for \"{}\".", self.app.app_id, role, format!("{} ({}, {})",self.dialect.artifact_kind,self.dialect.standard,self.dialect.subset)));
        }
        MutationOutcome::new(OpeningDiff { pins: vec![DefaultAppPin { dialect: self.dialect.clone(), role: self.role, app: Some(self.app.clone()) }] })
    }

    fn inverse(&self, base: &OpeningPreferences) -> Result<Vec<OpeningConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        match base.defaults.iter().find(|entry| entry.dialect == self.dialect && entry.role == self.role) {
            Some(prior) => vec![OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: self.dialect.clone(), role: self.role, app: prior.app.clone() })],
            None => vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect: self.dialect.clone(), role: self.role })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set default {} for \"{}\"", role_name(self.role), format!("{} ({}, {})",self.dialect.artifact_kind,self.dialect.standard,self.dialect.subset)), &format!("Standard-{} für \"{}\" festlegen", role_name_de(self.role), format!("{} ({}, {})",self.dialect.artifact_kind,self.dialect.standard,self.dialect.subset)))
    }

    fn target(&self) -> Vec<String> {
        vec![self.dialect.artifact_kind.clone(),self.dialect.standard.clone(),self.dialect.subset.clone(),role_name(self.role).to_string()]
    }
}

fn role_name(role: AppRole) -> &'static str {
    match role {
        AppRole::Viewer => "viewer",
        AppRole::Editor => "editor",
    }
}
/// 🌐️ German twin of `role_name` — the role word is authored copy, not data, so it is translated
/// rather than broadcast untranslated into the German cell.
fn role_name_de(role: AppRole) -> &'static str {
    match role {
        AppRole::Viewer => "Betrachter",
        AppRole::Editor => "Editor",
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️repins-the-cad-editor-to-the-drafting-app/🦀️.rs"]
mod tests_repins_the_cad_editor_to_the_drafting_app;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
