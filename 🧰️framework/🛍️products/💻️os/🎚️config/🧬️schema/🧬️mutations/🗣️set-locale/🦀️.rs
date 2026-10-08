//! 🗣️ Sets or clears the OS-wide locale preference.
use super::super::super::{SettingEdit, UiLocale, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLocale {
    pub locale: Option<UiLocale>,
}
pub fn set_locale(locale: Option<UiLocale>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetLocale(SetLocale { locale })
}
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.locale == self.locale {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "locale is already selected.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { locale: Some(SettingEdit::new(self.locale.clone())), ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetLocale(Self { locale: base.locale.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set locale to {:?}", self.locale), &format!("Sprache auf {:?} setzen", self.locale))
    }

    fn target(&self) -> Vec<String> {
        vec!["locale".to_string()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-locale/🦀️.rs"]
mod tests_sets_locale;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-locale/🦀️.rs"]
mod tests_keeps_locale;
//#endregion 🧪️Tests
