//! 🧰️ Gateway capability declarations and the core catalog.

extern crate semio_framework_os_kernel as store;

#[path = "../💡️inference/🧰️capability-declarations/🦀️.rs"]
pub mod capability_declarations;

#[cfg(test)]
#[path = "../🗂️catalog/🧪️tests/🔬️quick/🦀️.rs"]
pub mod catalog_quick;

#[path = "../📇️registry/🧰️catalog-source-discovery/🦀️.rs"]
pub mod catalog_source_discovery;

#[cfg(test)]
#[path = "../🧪️conformance/🧪️tests/🔬️quick/🦀️.rs"]
pub mod conformance_quick;

#[path = "../🗒️core-capabilities/🦀️.rs"]
pub mod core_capabilities;

#[cfg(test)]
#[path = "../📇️registry/🧪️tests/🔬️quick/🦀️.rs"]
pub mod registry_quick;

#[cfg(test)]
#[path = "../🔎️search/🧪️tests/🔬️long/🦀️.rs"]
pub mod search_long;

#[cfg(test)]
#[path = "../🔎️search/🧪️tests/🔬️quick/🦀️.rs"]
pub mod search_quick;

#[cfg(any(test, feature = "testkit"))]
#[path = "../🧪️tests/🧱️source-builders/🦀️.rs"]
pub mod source_builders;

#[cfg(test)]
#[path = "../🧪️tests/🧱️source-builders-quick/🦀️.rs"]
pub mod source_builders_quick;
