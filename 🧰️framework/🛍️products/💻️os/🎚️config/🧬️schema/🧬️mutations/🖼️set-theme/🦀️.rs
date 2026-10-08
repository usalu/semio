//! 🖼️ Sets or clears the OS-wide theme selection.
use super::super::super::{SettingEdit, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTheme {
    pub theme_id: Option<String>,
}
pub fn set_theme(theme_id: Option<String>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetTheme(SetTheme { theme_id })
}
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetTheme {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "theme", kind: "set-theme", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.theme_id == self.theme_id {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "theme is already selected.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { theme_id: Some(SettingEdit::new(self.theme_id.clone())), ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetTheme(Self { theme_id: base.theme_id.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set theme to {:?}", self.theme_id), &format!("Design auf {:?} setzen", self.theme_id))
    }

    fn target(&self) -> Vec<String> {
        vec!["theme-id".to_string()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-theme/🦀️.rs"]
mod tests_sets_theme;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-theme/🦀️.rs"]
mod tests_keeps_theme;
//#endregion 🧪️Tests
