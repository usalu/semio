//! 🔗 Host wasm embedder stub for `semio_plugin_bundle_installer_link_shim` when no plugin crate
//! overrides it. This crate is dual-purpose (`crate-type = ["cdylib", "rlib"]`): built alone as its
//! own wasm module, nothing else provides this symbol, so the stub must exist. Pulled in as an rlib
//! dependency of an actual plugin crate (e.g. `animate`, transitively via `kernel_3d_engine`), that
//! plugin's own `plugin_exports!` expansion already defines the same `#[no_mangle]` symbol name — a
//! plain strong definition here is a link-time one-definition-rule violation between the two.
//!
//! 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: `codegen-units = 1` (root `Cargo.toml`'s
//! `[profile.wasm-release]`) surfaced this as a hard `animate` link failure — under the workspace's
//! native `codegen-units = 256` default, this stub's own tiny compilation unit rarely got pulled from
//! the archive once a consumer's real definition was already resolved elsewhere, hiding the conflict
//! by accident. `linkage = "weak"` (nightly, already the pinned toolchain workspace-wide) is the
//! actual fix: it tells the linker this definition may be silently discarded in favor of a strong
//! one, which is exactly the "no plugin crate overrides it" semantics this docstring already
//! promised. (A separate, unrelated, pre-existing failure in `block`'s standalone `-engine` sub-crate
//! builds — confirmed via a diagnostic build under the untouched native profile, same failure either
//! way — was initially misattributed to this change; it is not.)

#[unsafe(no_mangle)]
#[linkage = "weak"]
pub extern "C" fn semio_plugin_bundle_installer_link_shim() {}
