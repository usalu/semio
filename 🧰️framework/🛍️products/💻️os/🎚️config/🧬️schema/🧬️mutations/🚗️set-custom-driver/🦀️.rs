//! 🚗️ Sets or removes one OS-wide custom driver.
use super::super::super::{UiDriver, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCustomDriver {
    pub driver_id: String,
    pub driver: Option<UiDriver>,
}
pub fn set_custom_driver(driver_id: impl Into<String>, driver: Option<UiDriver>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetCustomDriver(SetCustomDriver { driver_id: driver_id.into(), driver })
}
keyed_setting_impl!(SetCustomDriver, SetCustomDriver, custom_drivers, driver_id, driver, "set-custom-driver", "custom-driver", "custom driver", "custom-drivers");
