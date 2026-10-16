// 🐚️ HUB WP4: mount hub of WP4 Shell components: shell-vocab, shell-async, the thirteen component crates and the shell-state composer.
// This file is spliced into the crate root by `include!` from the renderer root, so every item below stays a crate-root item;
// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the renderer folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-RENDERER-WGPU-CRATES.md, section 10.3).

extern crate semio_framework_os_renderer_wgpu_shell_vocab as shell_vocab;

#[path = "../../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod shell;
