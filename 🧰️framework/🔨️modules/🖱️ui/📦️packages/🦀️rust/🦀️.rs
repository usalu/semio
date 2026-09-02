//! 🖱️ Handcrafted semio UI. One crate, one target per feature: `tui` renders to terminal cells,
//! `wgpu` renders declarative components (and, with `wgpu-engine`, a retained-mode GPU engine).
//! The two share `ui_styling` tokens and the co-located `🧱️elements/<Element>/` widget sources but
//! nothing else — hence features rather than sibling crates, so a wasm32-wasip2 program component
//! pulling in the component types never links winit/parley/swash.

/// 🧬️ Code-side `dsl` alias for `semio-framework-os-kernel`. It cannot be a Cargo rename, because
/// the `ToValue`/`FromValue` derives expand to the literal `::semio_framework_os_kernel::` path and
/// Cargo rejects the same package appearing twice under two names.
#[cfg(feature = "wgpu")]
extern crate semio_framework_os_kernel as dsl;

#[cfg(feature = "tui")]
#[path = "🎯️targets/⌨️tui/🦀️.rs"]
pub mod tui;

#[cfg(feature = "wgpu")]
#[path = "🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod wgpu;
