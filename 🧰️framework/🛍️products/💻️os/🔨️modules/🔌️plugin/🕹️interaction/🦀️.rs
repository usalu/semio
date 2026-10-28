//! 🕹️ Exact local interaction ownership, query, and publication components.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
#[cfg(test)]
use semio_framework_os_kernel::io;

#[path = "🔐️authority/🦀️.rs"]
pub mod authority;

#[path = "📡️live/🦀️.rs"]
pub mod live;

#[path = "📖️capture/🦀️.rs"]
mod capture;
pub use capture::LocalInteractionCaptureCursor;

#[path = "📃️query/🦀️.rs"]
pub mod query;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

#[path = "🧭️topology/🦀️.rs"]
pub mod interaction_topology;

/// 🧩 Fallible fixed-capacity UI assembly result shared by plugin render boundaries.
pub type UiAssemblyResult<T> = Result<T, PluginAssemblyError>;

pub fn ui_assembly_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}"))
}

#[path="🌱️genesis/🦀️.rs"]
pub mod original_genesis;
