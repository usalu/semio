//! 🕹️ Sets or clears the OS-wide driver selection.
use super::super::super::{UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDriver {
    pub driver_id: Option<String>,
}
pub fn set_driver(driver_id: Option<String>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetDriver(SetDriver { driver_id })
}
optional_setting_impl!(SetDriver, SetDriver, driver_id, "set-driver", "driver", "driver", "driver-id");
