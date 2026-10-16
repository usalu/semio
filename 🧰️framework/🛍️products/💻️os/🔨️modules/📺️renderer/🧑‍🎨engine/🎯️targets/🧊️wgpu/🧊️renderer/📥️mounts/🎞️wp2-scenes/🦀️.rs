// 🎞️ HUB WP2: mount hub of WP2 EngineCanvas, Scenes, IconRenderHost and Canvas2dHost: engine-*, scene-*, canvas2d-host, icon-host.
// This file is spliced into the crate root by `include!` from the renderer root, so every item below stays a crate-root item;
// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the renderer folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-RENDERER-WGPU-CRATES.md, section 10.3).

#[path = "../../../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod engine_canvas;

#[path = "../../../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod scenes;

extern crate semio_framework_os_renderer_wgpu_canvas2d_host as canvas2d_gumball;

extern crate semio_framework_os_renderer_wgpu_icon_host as icon_atlas;
