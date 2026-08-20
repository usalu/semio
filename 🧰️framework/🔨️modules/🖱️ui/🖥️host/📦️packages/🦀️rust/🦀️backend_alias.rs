//! @emoji 🔌️ The per-target concrete `ActiveBackend` alias — the shape `ui_render::backend`'s own
//! docstring prescribes for this exact crate (U3: `dyn GraphicsBackend` is banned, and there is
//! nothing here for a vtable to erase in the first place — exactly one of the four hand-written
//! backends compiles per real target, so the seam resolves at *compile* time via a `cfg`-selected
//! type alias to a **concrete** type, never an enum or a `Box<dyn _>`).
//!
//! A host or a generic frame driver never names a concrete backend type — it is generic over
//! `<B: ui_render::GraphicsBackend>` (or over [`ActiveBackend`] directly), so retargeting a platform
//! costs exactly one line here.
//!
//! ⚠️ **UNVERIFIED until the four sibling backend packets land.** Checked 2026-08-20: `backend-metal`,
//! `backend-d3d12`, `backend-vulkan` and `backend-webgpu` are all still scaffolds — each `📦️glue.rs`
//! is an empty `//#region Backend` block with no `pub type`/`pub struct` of its own yet. The four type
//! names below (`MetalBackend`, `D3d12Backend`, `VulkanBackend`, `WebGpuBackend`) come from this
//! packet's own brief and from `ui_render::backend`'s docstring, not from a real `impl GraphicsBackend`
//! anywhere yet — this file will not compile on any real target until its matching backend packet
//! defines that type under this exact name. Per ruling U4 this agent does not run `cargo`; `sol`
//! verifies per target once each backend packet lands.

//#region 🔖️Host

#[cfg(target_arch = "wasm32")]
pub type ActiveBackend = backend_webgpu::WebGpuBackend;

#[cfg(target_os = "macos")]
pub type ActiveBackend = backend_metal::MetalBackend;

#[cfg(target_os = "windows")]
pub type ActiveBackend = backend_d3d12::D3d12Backend;

#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
pub type ActiveBackend = backend_vulkan::VulkanBackend;

//#endregion 🔖️Host
