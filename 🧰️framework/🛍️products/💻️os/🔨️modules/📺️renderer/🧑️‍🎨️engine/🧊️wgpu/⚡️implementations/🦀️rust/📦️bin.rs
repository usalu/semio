#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use semio_framework_renderer_wgpu::run_native;
    use std::env;
    use std::path::PathBuf;

    let plugin_filter = env::args().position(|arg| arg == "--plugin").and_then(|index| env::args().nth(index + 1)).unwrap_or_else(|| "studio".to_string());
    let modules_root = env::var("SEMIO_PLUGIN_MODULES").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../dev/js/plugin-modules"));
    run_native(&plugin_filter, modules_root);
}
