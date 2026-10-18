//! 🫐️ Companion test crate tests-09 of the plugin SDK partition: hosts the test roots `segmented_download_contract`, `app_commands_tests_host`, `job_tool_run_host_2`, `testing_fixtures_test_hosts`, `plugin_builder_contract_tests_5`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🧪️tests/📤️segmented-download/🦀️.rs"]
pub mod segmented_download_contract;
#[cfg(test)]
#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/🎯️app-commands/🦀️.rs"]
pub mod app_commands_tests_host;
#[cfg(test)]
#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/👷️job/🧱️unit-5/🦀️.rs"]
pub mod job_tool_run_host_2;
#[cfg(test)]
#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/🎟️testing-fixtures/🦀️.rs"]
pub mod testing_fixtures_test_hosts;

