// 🌱️ HUB WP1: mount hub of WP1 non-store leaves and hubs: os_dsl, os_pack, list_delta, os_spr, os_plugin_module_schema, os_semio, os_extension, native_catalog.
// This file is spliced into the crate root by `include!` from the kernel root, so every item below stays a crate-root item;
// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the kernel package folder.
// Carving a crate removes its unit here and puts the facade `pub use` lines of the crate in its place (PLAN-OS-KERNEL-CRATES.md, section 10.2).

/// 🗣️ The OS product grammars live in `semio-framework-os-dsl`; every historical `os_dsl::` path keeps resolving here.
pub use semio_framework_os_dsl as os_dsl;
pub use semio_framework_os_dsl::{original_ordinal_operation_receiving, original_tagged_operation_receiving};

pub mod os_pack {
    #[cfg(not(target_arch = "wasm32"))]
    pub use semio_framework_os_pack_tools::control;
    pub use semio_framework_os_pack::*;

    #[cfg(not(target_arch = "wasm32"))]
    pub use semio_framework_os_pack_tools as cli;

    // 🎒️ The `.spk` container (header/footer/segments/manifest/recovery/sources) and the schema-driven record
    // codec are owned by `🧰️framework/🔨️modules/🎒️pack` (`pack::record`), and its codec floor by the replication
    // crate. What stays os-side is only the pack runtime bridge, the stored-hash read and the test-only laws.
    pub use pack::async_;
    pub use pack::codec;
    pub use pack::codec::ids;
    pub use pack::format;
    pub use pack::http;
    #[cfg(not(target_arch = "wasm32"))]
    pub use pack::io;
    pub use semio_framework_pack_source as source;

    // 🎾️ The flat codec/ids/source surface arrives through `semio_framework_os_pack`'s `pub use pack::*` above —
    // re-exporting it a second time here would make every primitive an ambiguous glob.
}

/// 🪡️ Generic positional list deltas — `{removed, inserted, moved, patched}` rows with base/after coordinates, never an id order list.
pub use semio_framework_os_list_delta as list_delta;
pub use semio_framework_os_list_delta::{list_delta, plain_list_delta, row_patch};

/// 📡️ The spr component, the replication aliases and the command line live in `semio-framework-os-spr` and `-os-spr-cli`.
pub mod os_spr {
    pub use semio_framework_os_spr::*;

    #[cfg(not(target_arch = "wasm32"))]
    pub use semio_framework_os_spr_cli as cli;
}

// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C1: directory event log
// schema + pure read model (`DirectoryEvent`/`DirectoryReadModel`/`fold`) — plain serde data, no
// cross-crate dependency, so it mounts cleanly unlike `🔁️workflow` below.
pub use semio_framework_os_plugin_registry_schema as os_plugin_module_schema;
pub use semio_framework_os_plugin_registry_schema::native_catalog;

pub use semio_framework_os_semio as os_semio;

// 🧩️ `.sxt` extension package pack/unpack/verify is host-only: installing a runtime extension is
// native-host tooling (only caller repo-wide: `semio-framework-os`'s host crate), never something a
// guest component does to itself. `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so the
// dependency is narrowed to exclude the WASI component target with the full
// `not(all(target_arch = "wasm32", target_env = "p2"))` form — a bare arch gate would also exclude
// the browser wasm32 target, which does not need to lose this capability. See
// `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`'s matching `zip` target-gate.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use semio_framework_os_extension as os_extension;
