//! 📐️ Sets or clears the OS-wide chrome layout preference.
use super::super::super::{UiChromeLayout, UiPreferences, UiPreferencesDiff};
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
optional_setting_impl!(SetLayout, SetLayout, layout, "set-layout", "layout", "layout", "layout");

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-layout/🦀️.rs"]
mod tests_sets_layout;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-layout/🦀️.rs"]
mod tests_keeps_layout;
//#endregion 🧪️Tests
