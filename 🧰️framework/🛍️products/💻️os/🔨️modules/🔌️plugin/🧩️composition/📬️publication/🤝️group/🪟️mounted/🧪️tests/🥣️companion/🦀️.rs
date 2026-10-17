//! 🥣️ Companion tests of the mounted publication: the receipt byte, operation framing, host receipt and handle issuer laws, run against the public surface of `semio-framework-plugin-publication-mounted`.
//!
//! 🧭️ The receipt laws sit below a wrapper of the receipt module that restates its imports; every fixture is read at run time from the mounted folder, so no document is compiled into the test closure.
#![cfg(test)]

use semio_framework_test_fixtures::fixture_path;

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

/// 📥️ Reads a committed language-agnostic fixture of the mounted laws at run time, by its path relative to the mounted folder, so the test closure stays inside the source gate instead of embedding the fixtures with `include_str!`.
/// @see <https://doc.rust-lang.org/std/macro.include_str.html>
pub(crate) fn fixture_json(relative: &str) -> serde_json::Value {
    let path = fixture_path!("../../../..").join(relative);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
}

#[path = "🧾️receipt/🦀️.rs"]
mod mounted_child_group_receipt;
