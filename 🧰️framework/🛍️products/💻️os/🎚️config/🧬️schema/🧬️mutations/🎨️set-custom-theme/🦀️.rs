//! 🎨️ Sets or removes one OS-wide custom theme.
use super::super::super::{KeyedEdit, UiPreferences, UiPreferencesDiff, UiTheme};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCustomTheme {
    pub theme_id: String,
    pub theme: Option<UiTheme>,
}
pub fn set_custom_theme(theme_id: impl Into<String>, theme: Option<UiTheme>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetCustomTheme(SetCustomTheme { theme_id: theme_id.into(), theme })
}
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetCustomTheme {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "custom-theme", kind: "set-custom-theme", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.custom_themes.get(&self.theme_id) == self.theme.as_ref() {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "custom theme already has the requested value.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { custom_themes: vec![KeyedEdit::new(self.theme_id.clone(), self.theme.clone())], ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetCustomTheme(Self { theme_id: self.theme_id.clone(), theme: base.custom_themes.get(&self.theme_id).cloned() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set custom theme {:?}", self.theme_id), &format!("Eigenes Design {:?} setzen", self.theme_id))
    }

    fn target(&self) -> Vec<String> {
        vec!["custom-themes".to_string(), self.theme_id.clone()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-custom-theme/🦀️.rs"]
mod tests_sets_custom_theme;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-custom-theme/🦀️.rs"]
mod tests_keeps_custom_theme;
//#endregion 🧪️Tests
