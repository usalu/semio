//! 🧱️ Tree, table, world3d and document window kits of the plugin SDK.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "../🌳️tree/🦀️.rs"]
pub mod tree_kit;
#[path = "../📊️table/🦀️.rs"]
pub mod table_kit;
#[path = "../🌍️world3d/🦀️.rs"]
pub mod world3d_host;
#[path = "../📃️document/🦀️.rs"]
pub mod document_kit;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
