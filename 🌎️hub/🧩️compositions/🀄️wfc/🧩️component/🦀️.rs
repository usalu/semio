//! 🀄️ The `wfc` plugin component: the one cdylib of `semio-hub-wfc`, holding only its entry point.

semio_plugin::plugin_exports!(semio_hub_wfc::WFC_MOUNTED_OWNER_POLICY, semio_hub_wfc::plugin, semio_hub_wfc::WfcApps);

#[cfg(test)]
#[path = "🧪️tests/🎟️mounted-owner-policy/🦀️.rs"]
mod mounted_owner_policy_tests;
