//! 🏇️ Mounted member groups, receipts and members of the plugin SDK publication.

use semio_framework_test_fixtures::fixture_path;

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "🧾️receipt/📦️group/📚️command/🪟️mounted/🦀️.rs"]
pub mod command_mounted;
#[path = "📦️owner/🦀️.rs"]
pub mod owner;
#[path = "🪪️identity/🦀️.rs"]
pub mod mounted_child_group_identity;
#[path = "🧾️receipt/🦀️.rs"]
pub mod mounted_child_group_receipt;
#[path = "🧾️receipt/📦️group/🦀️.rs"]
pub mod mounted_child_group_receipt_group;

/// 📥️ Reads a committed language-agnostic fixture of the mounted laws at run time, by its path relative to this folder, so the test closure stays inside the source gate instead of embedding the fixtures with `include_str!`.
/// @see <https://doc.rust-lang.org/std/macro.include_str.html>
#[cfg(test)]
pub(crate) fn fixture_json(relative: &str) -> serde_json::Value {
    let path = fixture_path!("../..").join(relative);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
}
