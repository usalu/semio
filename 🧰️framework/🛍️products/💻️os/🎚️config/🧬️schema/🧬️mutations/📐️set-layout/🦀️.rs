//! 📐️ Sets or clears the OS-wide chrome layout preference.
use super::super::super::{SettingEdit, UiChromeLayout, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLayout {
    pub layout: Option<UiChromeLayout>,
}
pub fn set_layout(layout: Option<UiChromeLayout>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetLayout(SetLayout { layout })
}
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetLayout {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layout", kind: "set-layout", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.layout == self.layout {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "layout is already selected.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { layout: Some(SettingEdit::new(self.layout.clone())), ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetLayout(Self { layout: base.layout.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layout to {:?}", self.layout), &format!("Layout auf {:?} setzen", self.layout))
    }

    fn target(&self) -> Vec<String> {
        vec!["layout".to_string()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-layout/🦀️.rs"]
mod tests_sets_layout;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-layout/🦀️.rs"]
mod tests_keeps_layout;
//#endregion 🧪️Tests
