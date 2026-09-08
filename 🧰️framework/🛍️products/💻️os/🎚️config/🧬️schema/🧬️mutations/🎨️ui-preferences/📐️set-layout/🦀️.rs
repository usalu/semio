//! 📐️ Sets or clears the OS-wide chrome layout preference.
use super::super::super::UiPreferencesDiff;
use super::super::UiPreferencesConfigMutation;
use semio_framework_os_shell::{UiChromeLayout, UiPreferences};
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
