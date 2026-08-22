#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use semio_framework_os_renderer_wgpu::{run_native, run_smoke};
    use std::env;
    use std::path::PathBuf;

    fn arg_value(flag: &str) -> Option<String> {
        env::args().position(|arg| arg == flag).and_then(|index| env::args().nth(index + 1))
    }

    fn drive_entrypoint<F: std::future::Future>(future: F) -> F::Output {
        semio_framework_async::block_on(future)
    }

    let plugin_filter = arg_value("--plugin").unwrap_or_else(|| "studio".to_string());
    let modules_root = env::var("SEMIO_PLUGIN_MODULES").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../dev/js/plugin-modules"));
    // 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b-bench) — `--scale <registry.json>
    // --scale-wasm <fixture.wasm> --report <out.json> [--shards <K>]` bypasses ShellState/GPU/winit
    // entirely and drives `semio_framework_actor::Kernel` + `WasmtimeRuntime` directly against the
    // scale fixture's real wasm component; see `scale_bench::run`'s own doc comment for what it
    // measures and its honest single-shard-loop scope note.
    if let Some(registry_path) = arg_value("--scale") {
        let Some(wasm_path) = arg_value("--scale-wasm") else {
            eprintln!("[DEBUG] --scale requires --scale-wasm <fixture.wasm>");
            std::process::exit(1);
        };
        let Some(report_path) = arg_value("--report") else {
            eprintln!("[DEBUG] --scale requires --report <out.json>");
            std::process::exit(1);
        };
        let shard_count: u16 = arg_value("--shards").and_then(|v| v.parse().ok()).unwrap_or(8);
        std::process::exit(drive_entrypoint(semio_framework_os_renderer_wgpu::scale_bench::run(PathBuf::from(registry_path), PathBuf::from(wasm_path), shard_count, PathBuf::from(report_path))));
    }
    // 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — `--smoke` boots the shell (real
    // identity/directory/plugin path, no GPU/window) and dumps its widget tree as JSON instead of
    // opening a real window, for environments that cannot drive one.
    if env::args().any(|arg| arg == "--smoke") {
        std::process::exit(drive_entrypoint(run_smoke(&plugin_filter, modules_root)));
    }
    run_native(&plugin_filter, modules_root);
}
