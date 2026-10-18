//! 🍒️ Companion test crate tests-11 of the plugin SDK partition: hosts the test roots `original_codec_binding_laws`, `contributed_mutation_wire_tests_host`, `reactor_tasks_test_hosts`, `instance_ladders_test_hosts`, `plugin_builder_contract_tests_6`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/🪪️codec-binding/🦀️.rs"]
pub mod original_codec_binding_laws;
#[cfg(test)]
#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/📡️contributed-mutation-wire/🦀️.rs"]
pub mod contributed_mutation_wire_tests_host;
#[cfg(test)]
#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/⏲️reactor-tasks/🦀️.rs"]
pub mod reactor_tasks_test_hosts;
#[cfg(test)]
#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/🪜️instance-ladders/🦀️.rs"]
pub mod instance_ladders_test_hosts;

