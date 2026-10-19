/// 🪢️ Severable relay and ledger: the loopback relay whose connections can be cut, the collaboration ledger and the native drive and sign-in helpers.

/// ✂️ A loopback TCP relay in front of the hub whose every live connection can be cut at once,
/// then healed: the second actor reaches the hub ONLY through it, so severing it is a real
/// network loss on that actor's sockets — the document socket and the directory transport alike —
/// while the first actor's own connections stay untouched.
#[cfg(not(target_arch = "wasm32"))]
struct SeverableRelay {
    origin: String,
    live: std::sync::Arc<std::sync::Mutex<Vec<std::net::TcpStream>>>,
    open: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SeverableRelay {
    fn start(upstream_origin: &str) -> Self {
        let upstream = upstream_origin.trim_start_matches("http://").trim_end_matches('/').to_string();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("relay binds a loopback port");
        let origin = format!("http://{}", listener.local_addr().expect("relay address"));
        let live = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let open = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let (accepted, admitting) = (live.clone(), open.clone());
        std::thread::spawn(move || {
            for inbound in listener.incoming().flatten() {
                let outbound = match std::net::TcpStream::connect(&upstream) {
                    Ok(outbound) if admitting.load(std::sync::atomic::Ordering::SeqCst) => outbound,
                    _ => {
                        let _ = inbound.shutdown(std::net::Shutdown::Both);
                        continue;
                    }
                };
                let mut retained = accepted.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                retained.extend([inbound.try_clone(), outbound.try_clone()].into_iter().flatten());
                drop(retained);
                let pipe = |mut from: std::net::TcpStream, mut to: std::net::TcpStream| {
                    std::thread::spawn(move || {
                        let _ = std::io::copy(&mut from, &mut to);
                        let _ = to.shutdown(std::net::Shutdown::Write);
                    });
                };
                if let (Ok(from), Ok(to)) = (inbound.try_clone(), outbound.try_clone()) {
                    pipe(from, to);
                }
                pipe(outbound, inbound);
            }
        });
        Self { origin, live, open }
    }

    fn sever(&self) -> usize {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        let streams = std::mem::take(&mut *self.live.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
        streams.iter().for_each(|stream| {
            let _ = stream.shutdown(std::net::Shutdown::Both);
        });
        streams.len()
    }

    fn heal(&self) {
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

/// 📒️ One named step of the two-user journey and what the shells' own state showed for it.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct CollaborationLedger {
    steps: Vec<(&'static str, bool, String)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl CollaborationLedger {
    fn record(&mut self, step: &'static str, passed: bool, detail: String) -> bool {
        println!("g7w-live step={step} {} {detail}", if passed { "PASS" } else { "FAIL" });
        self.steps.push((step, passed, detail));
        passed
    }
}

/// 🔂️ Polls one shell future to completion while pumping what the GPU present loop pumps in the
/// real binary (`present_step_inner`): retained renderer I/O sessions and worker-job retirements.
/// Without it a headless caller parks forever on the first `read_renderer_source_bytes` (the native runtime
/// manifest read inside `load_wasm_plugins`), because nothing else drives those slots.
#[cfg(not(target_arch = "wasm32"))]
fn drive<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        let advanced = semio_framework_os_renderer_wgpu_host_base::pump_renderer_io_sessions(1);
        let _ = semio_framework_job_worker::pump_worker_job_retirements(1, 1, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);
        if advanced == 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn live_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} is required; see the two-user law's doc comment"))
}

/// 🔑️ Signs one shell in through its own hub lane, clicking Sign in a second time exactly like a
/// human would when the first attempt failed (a debug hub's password hash can outlast the lane's
/// 5 s command deadline under load); answers the first attempt's refusal, if any.
#[cfg(not(target_arch = "wasm32"))]
fn sign_in_live(shell: &mut ShellState, origin: &str, email: &str, password: &str) -> Option<String> {
    drive(ActionsCtx::of(&mut shell).apply_os_command("os.openHub", None)).expect("the hub route opens");
    hub_verb(shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_ADDRESS, &[("value", origin)]);
    hub_verb(shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::ADD_CONNECTION, &[]);
    let mut first_refusal = None;
    for _ in 0..2 {
        hub_verb(shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_EMAIL, &[("value", email)]);
        hub_verb(shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_PASSWORD, &[("value", password)]);
        hub_verb(shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_IN, &[]);
        if shell.directory.hub_workspace.session.phase == HubSessionPhase::SignedIn {
            break;
        }
        first_refusal.get_or_insert_with(|| format!("{:?}", shell.directory.hub_workspace.session.error));
    }
    first_refusal
}

#[cfg(not(target_arch = "wasm32"))]
fn shell_command(shell: &mut ShellState, action_id: &str, args: &[(&str, &str)]) {
    let args = DslValue::Object(args.iter().map(|(key, value)| ((*key).to_string(), DslValue::String((*value).to_string()))).collect());
    drive(ActionsCtx::of(&mut shell).handle_replay_shell_command(action_id, Some(&args)));
}
