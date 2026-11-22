//! 🪜️ Companion test crate hosting the mounted browser grant archive publication law, which depends only on public fixture and runtime APIs.
//!
//! 🧭️ The law source stays in `🧪️tests/🔬️mounted-grant-archive-publication` and is included here.

#[cfg(test)]
extern crate semio_os_kernel as store;
#[cfg(test)]
extern crate semio_os_kernel as protocol;

#[cfg(test)]
mod mounted_grant_archive_publication {
    use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{TestCommand, contract_composed_app_raw, drain_and_close_composed_fixture, meta, register_test_child};
    use semio_plugin_reactor_app::PluginApp;
    use semio_plugin_testing_laws::artifact_app_laws;
    use serde_json::Value;
    use semio_os_kernel as protocol;

    include!("../🧪️tests/🔬️mounted-grant-archive-publication/🦀️.rs");
}
