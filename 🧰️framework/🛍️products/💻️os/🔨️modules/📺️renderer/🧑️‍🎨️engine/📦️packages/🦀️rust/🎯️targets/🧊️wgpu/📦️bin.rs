#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use semio_framework_os_renderer_wgpu::{run_native, run_smoke};
    use std::env;
    use std::path::PathBuf;

    let plugin_filter = env::args().position(|arg| arg == "--plugin").and_then(|index| env::args().nth(index + 1)).unwrap_or_else(|| "studio".to_string());
    let modules_root = env::var("SEMIO_PLUGIN_MODULES").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../dev/js/plugin-modules"));
    // 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — `--smoke` boots the shell (real
    // identity/directory/plugin path, no GPU/window) and dumps its widget tree as JSON instead of
    // opening a real window, for environments that cannot drive one.
    if env::args().any(|arg| arg == "--smoke") {
        std::process::exit(run_smoke(&plugin_filter, modules_root));
    }
    run_native(&plugin_filter, modules_root);
}
