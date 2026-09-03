#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use semio_framework_os_renderer_wgpu::{run_native, run_smoke};
    use semio_framework_os_kernel::os_directory::identity::{claim_inherited_local_hub_credential, IdentityEnv};
    use std::env;
    use std::path::PathBuf;

    fn arg_value(flag: &str) -> Option<String> {
        env::args().position(|arg| arg == flag).and_then(|index| env::args().nth(index + 1))
    }

    fn drive_entrypoint<F: std::future::Future>(future: F) -> F::Output {
        semio_framework_async::block_on(future)
    }

    #[cfg(unix)]
    fn inherited_credential_fd_is_closed() -> bool {
        unsafe extern "C" {
            fn fcntl(fd: i32, command: i32, ...) -> i32;
        }
        unsafe { fcntl(3, 1) } < 0
    }

    #[cfg(windows)]
    fn inherited_credential_fd_is_closed() -> bool {
        unsafe extern "C" {
            fn _get_osfhandle(fd: i32) -> isize;
        }
        unsafe { _get_osfhandle(3) } == -1
    }

    fn protected_credential_environment_is_absent() -> bool {
        env::vars_os().all(|(key, value)| {
            let key = key.to_string_lossy().to_ascii_uppercase();
            if key == "S_LOCAL_CREDENTIAL_FD" {
                return value == "3";
            }
            !(key == "S_USER"
                    || key == "VITE_S_USER"
                    || key == "S_HUB_URL"
                    || key.contains("TOKEN")
                    || key.contains("SESSION")
                    || key.contains("CREDENTIAL")
                    || key.contains("BEARER")
                    || key.contains("CAPABILITY")
                    || key.contains("AUTHORIZATION")
                    || key.contains("COOKIE"))
        })
    }

    fn benign_direct_child_environment_is_preserved() -> bool {
        env::var("SEMIO_DIRECT_CHILD_BENIGN").ok().as_deref() == Some("preserved")
    }

    if !protected_credential_environment_is_absent() {
        eprintln!("[DEBUG] native protected credential environment rejected");
        std::process::exit(1);
    }
    if IdentityEnv::from_process_env().is_some() && claim_inherited_local_hub_credential("native").is_err() {
        eprintln!("[DEBUG] native local credential claim failed");
        std::process::exit(1);
    }
    if env::args().any(|arg| arg == "--assert-no-local-credential-state") {
        std::process::exit(if inherited_credential_fd_is_closed() && protected_credential_environment_is_absent() && benign_direct_child_environment_is_preserved() { 0 } else { 1 });
    }
    if env::args().any(|arg| arg == "--credential-probe") {
        if !protected_credential_environment_is_absent() || !benign_direct_child_environment_is_preserved() {
            std::process::exit(1);
        }
        let status = env::current_exe()
            .ok()
            .and_then(|executable| std::process::Command::new(executable).arg("--assert-no-local-credential-state").env_remove("S_LOCAL_CREDENTIAL_FD").status().ok());
        if status.is_some_and(|status| status.success()) {
            println!("native-credential-probe-ok");
            std::process::exit(0);
        }
        std::process::exit(1);
    }
    if env::args().any(|arg| arg == "--socket-grant-probe") {
        let status = drive_entrypoint(semio_framework_os_renderer_wgpu::run_socket_grant_probe());
        if status == 0 {
            println!("native-socket-grant-probe-ok");
        }
        std::process::exit(status);
    }
    let plugin_filter = arg_value("--plugin").unwrap_or_else(|| "studio".to_string());
    let modules_root = env::var("SEMIO_PLUGIN_MODULES").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../🧑️‍💻️dev/🔌️plugin-modules"));
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
