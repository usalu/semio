/// ⏱️ Frame pumps and edit ledger: remote and liveness probes, the frame pump, document opening settle, paired pumping and the applied-edit and membership ledgers.

#[cfg(not(target_arch = "wasm32"))]
fn remote_of(shell: &ShellState) -> String {
    shell.sync.sync_status.as_ref().map_or_else(|| "none".to_string(), |status| format!("{:?}", status.remote))
}

#[cfg(not(target_arch = "wasm32"))]
fn is_live(shell: &ShellState) -> bool {
    matches!(shell.sync.sync_status.as_ref().map(|status| &status.remote), Some(RemoteState::Live { .. }))
}

/// 🖼️ What one painted frame does for an open document, headless, answering how many renderer I/O
/// sessions it advanced: the present loop's renderer I/O and
/// worker-retirement slots (`present_step_inner`; a detached request such as the open's app instance
/// waits on them), the frame pump (`pump_sync_events`: directory lane, auto check-in, the frame-pumped
/// open, document actor events), the chrome walk's presence phase, which arms the maintenance lane's
/// heartbeat step (`ShellChromeFramePhase::Presence` → `advance_presence_preview_step`), and one settle
/// step when the settle lane is owed one. Without the presence half a headless shell never beat, so the
/// hub saw no peer and dropped each idle document socket after its presence lease (two-user gate run 13).
#[cfg(not(target_arch = "wasm32"))]
fn frame_pump(shell: &mut ShellState) -> usize {
    let advanced = semio_framework_os_renderer_wgpu_host_base::pump_renderer_io_sessions(1);
    let _ = semio_framework_job_worker::pump_worker_job_retirements(1, 1, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);
    drive(PointerCtx::of(&mut shell).pump_sync_events());
    shell.sync.request_presence_preview(&mut shell.chrome);
    if shell.chrome.chrome_present.maintenance.presence_requested {
        ChromeViewCtx::of(&mut shell).advance_presence_preview_step();
    }
    if PointerCtx::of(&mut shell).settle_pump_pending() {
        drive(PointerCtx::of(&mut shell).settle_pump_step());
    }
    advanced
}

/// ⏱️ How one frame-pumped open went, frame by frame: the frames while its steps were out and the
/// frames that then rendered what it owed, each with its count and its longest single frame.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
struct OpenFrames {
    opening: usize,
    opening_longest: std::time::Duration,
    rendering: usize,
    rendering_longest: std::time::Duration,
    elapsed: std::time::Duration,
}

/// 🚪️ Pumps frames ([`frame_pump`]) until the shell's frame-pumped document open settles and the
/// settle lane has rendered what it owed, timing every frame. Like the present loop, it idles only when
/// no renderer I/O session advanced. A settled failure or cancellation is left for the caller to assert.
#[cfg(not(target_arch = "wasm32"))]
fn settle_document_opening(shell: &mut ShellState) -> OpenFrames {
    let started = std::time::Instant::now();
    let mut frames = OpenFrames::default();
    while shell.opening.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        let advanced = frame_pump(shell);
        frames.opening += 1;
        frames.opening_longest = frames.opening_longest.max(frame.elapsed());
        if advanced == 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    while PointerCtx::of(&mut shell).settle_pump_pending() && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        let _ = frame_pump(shell);
        frames.rendering += 1;
        frames.rendering_longest = frames.rendering_longest.max(frame.elapsed());
    }
    frames.elapsed = started.elapsed();
    frames
}

/// 🔁️ Drives both shells' frames ([`frame_pump`]) until `done` holds or the budget ends, recording
/// every distinct remote state either shell passed through.
#[cfg(not(target_arch = "wasm32"))]
fn pump_pair(a: &mut ShellState, b: &mut ShellState, budget: std::time::Duration, trail: &mut Vec<String>, done: impl Fn(&ShellState, &ShellState) -> bool) -> Option<std::time::Duration> {
    let started = std::time::Instant::now();
    while started.elapsed() < budget {
        let _ = frame_pump(a);
        let _ = frame_pump(b);
        let observed = format!("A={} B={}", remote_of(a), remote_of(b));
        if trail.last() != Some(&observed) {
            trail.push(observed);
        }
        if done(a, b) {
            return Some(started.elapsed());
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn applied_edits(shell: &mut ShellState) -> Vec<(String, bool)> {
    drive(SessionCtx::of(&mut shell).refresh_history_snapshot());
    history_rows_oldest_first(&shell.history.history_entries).into_iter().filter(|entry| entry.kind == "mutation").map(|entry| (entry.action_id.clone(), entry.applied)).collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn author_edit(shell: &mut ShellState, action: &str) -> (Result<(), String>, std::time::Duration) {
    let controller_id = shell.docs.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
    let started = std::time::Instant::now();
    let outcome = drive(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id, action: action.to_string(), args: None }));
    (outcome, started.elapsed())
}

#[cfg(not(target_arch = "wasm32"))]
fn online_members(shell: &mut ShellState, space_id: &str) -> Vec<(String, bool)> {
    drive(shell.directory.reload_hub_members(space_id));
    shell.directory.hub_workspace.members.iter().map(|member| (member.display_name.clone(), member.online)).collect()
}
