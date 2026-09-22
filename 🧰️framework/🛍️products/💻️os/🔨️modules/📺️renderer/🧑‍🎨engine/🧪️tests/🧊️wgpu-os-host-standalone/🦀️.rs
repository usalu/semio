#[cfg(test)]
fn terminal_os_host_retirement_state() -> OsHostRetirementState {
    OsHostRetirementState {
        runtime: None,
        presenter: None,
        scheduler: None,
        clock: None,
        caret: None,
        hot_swap: None,
        events: None,
        ui_token: None,
        snapshot_sink: None,
        frame_build: None,
        surface_resize: None,
        engine_surfaces: PairedEngineSurfaceClose {
            operation: semio_framework_trace::allocate_operation_id(),
            sequence: 0,
            scan: crate::engine_canvas::ENGINE_SURFACE_CAPACITY,
            token: None,
            cpu_present: false,
            gpu_present: false,
            phase: PairedEngineSurfaceClosePhase::Terminal,
            faulted: false,
        },
        raster_uploads: None,
        cursor_wake_requested: None,
        #[cfg(not(target_arch = "wasm32"))]
        kernel_progress_close: None,
    }
}

#[cfg(test)]
pub(crate) fn finish_retired_cpu_only_map_fixture_close() {
    finish_cpu_only_fixture_component_close(|owner| assert_eq!(owner.kind, ui_wgpu::wgpu::SurfaceKind::TiledMap));
}

#[cfg(test)]
pub(crate) fn finish_cpu_only_fixture_component_close(close: impl FnOnce(&crate::interpreter::ScenePointerTarget)) {
    let Some(request) = take_component_surface_close_request() else { return };
    assert!(matches!(request.owner.kind, ui_wgpu::wgpu::SurfaceKind::TiledMap | ui_wgpu::wgpu::SurfaceKind::NodeGraph | ui_wgpu::wgpu::SurfaceKind::Board2d));
    close(&request.owner);
    assert!(crate::engine_canvas::engine_surface_token(&request.owner.host_id).is_none());
    if let Some(token) = request.engine_token {
        assert_eq!(crate::engine_canvas::engine_surface_token_at(usize::from(token.slot)), Ok(None));
    }
    assert!(publish_component_surface_close_terminal(request.token));
}

#[cfg(test)]
#[test]
fn interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one() {
    let token = reserve_os_host_retirement_abandonment().expect("fixed host retirement reservation");
    let stale = OsHostRetirementAbandonment { slot: token.slot, generation: token.generation.checked_add(1).expect("test generation") };
    assert!(!release_os_host_retirement_abandonment(stale));
    drop(OsHostRetirement { state: Some(Box::new(terminal_os_host_retirement_state())), abandonment: Some(token) });
    assert_eq!(OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire), 1);
    let mut turns = 0usize;
    while !OsHostRetirement::close_abandoned_step() {
        turns += 1;
        assert!(turns <= OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY);
    }
    assert_eq!(OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire), 0);
    let mut reservations = [None; OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY];
    for reservation in &mut reservations {
        *reservation = reserve_os_host_retirement_abandonment();
        assert!(reservation.is_some());
    }
    assert!(reserve_os_host_retirement_abandonment().is_none());
    for reservation in reservations.into_iter().flatten() {
        assert!(release_os_host_retirement_abandonment(reservation));
    }
}

#[cfg(test)]
fn runtime_with_component_worlds() -> RuntimeMailbox {
    runtime_with_component_world_asset_urls("/component-world-a.glb", "/component-world-b.glb")
}

#[cfg(test)]
fn runtime_with_component_world_asset_urls(world_a_url: &str, world_b_url: &str) -> RuntimeMailbox {
    let mut shell = crate::shell::ShellState::new(Vec::new(), "component-asset-close".to_string());
    for (host_id, surface_id, url) in [("component-world-a", "document-world-a", world_a_url), ("component-world-b", "document-world-b", world_b_url)] {
        let mut state = infinite_world::world::World3dState::new(surface_id.to_string(), format!("{surface_id}.controller"));
        infinite_world::world::reserve_world3d_asset_request(&mut state, infinite_world::world::WorldAssetRequestKind::Glb, url).expect("the exact World admits its asset request");
        assert!(shell.world3d_states.try_insert(host_id.to_string(), state).is_ok(), "the component World host is admitted");
    }
    runtime_with_shell(shell)
}

#[cfg(test)]
fn runtime_with_shell(shell: crate::shell::ShellState) -> RuntimeMailbox {
    RuntimeMailbox::new(crate::AppRuntime {
        atlas: crate::FontAtlas::builtin(),
        icons: crate::IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: Some(crate::AppInteractionState {
            shell,
            input: crate::InputState::default(),
            theme: crate::Theme::default(),
            theme_dark: false,
            last_pointer_x: 0.0,
            last_pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            pointer_capture: crate::shell::PointerCapture::default(),
            modifiers: crate::PointerModifiers::default(),
            space_pressed: false,
            wheel_zoom_deadline_ms: 0.0,
            caret_blink_at_ms: 0.0,
            caret_blink_visible: true,
            text_streams: std::array::from_fn(|_| None),
            text_fault: None,
            frame_fault: None,
            text_cancel_pending: false,
            last_sync_pump_ms: 0.0,
        }),
        checkout: Default::default(),
        draw: crate::DrawList::default(),
        overlay: crate::DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: crate::FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_plugin_mtimes: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_modified: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    })
}

#[cfg(all(test, not(target_arch = "wasm32")))]
struct StalledNativeAssetServer {
    address: std::net::SocketAddr,
    stalled_request: std::sync::mpsc::Receiver<()>,
    stalled_head: std::sync::mpsc::Receiver<()>,
    sibling_request: std::sync::mpsc::Receiver<()>,
    release_stalled: Option<std::sync::mpsc::SyncSender<()>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

#[cfg(all(test, not(target_arch = "wasm32")))]
impl StalledNativeAssetServer {
    fn new(cleanup_deadline: std::time::Duration, response_bytes: usize) -> Self {
        Self::with_head_mode(cleanup_deadline, response_bytes, false)
    }

    fn withholding_head(cleanup_deadline: std::time::Duration, response_bytes: usize) -> Self {
        Self::with_head_mode(cleanup_deadline, response_bytes, true)
    }

    fn with_head_mode(cleanup_deadline: std::time::Duration, response_bytes: usize, withhold_head: bool) -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("local native asset listener");
        let address = listener.local_addr().expect("local native asset address");
        let (stalled_request_tx, stalled_request) = std::sync::mpsc::sync_channel(1);
        let (stalled_head_tx, stalled_head) = std::sync::mpsc::sync_channel(1);
        let (sibling_request_tx, sibling_request) = std::sync::mpsc::sync_channel(1);
        let (release_stalled, release_stalled_rx) = std::sync::mpsc::sync_channel(1);
        let thread = std::thread::spawn(move || {
            let (stalled, _) = listener.accept().expect("stalled native asset request");
            let stalled_thread = std::thread::spawn(move || {
                let mut stalled = stalled;
                stalled.set_read_timeout(Some(cleanup_deadline)).expect("stalled request read deadline");
                let mut request = [0; 1024];
                let _ = std::io::Read::read(&mut stalled, &mut request);
                let _ = stalled_request_tx.send(());
                if withhold_head {
                    let _ = release_stalled_rx.recv_timeout(cleanup_deadline);
                }
                let head = format!("HTTP/1.1 200 OK\r\nContent-Length: {response_bytes}\r\nConnection: close\r\n\r\n");
                std::io::Write::write_all(&mut stalled, head.as_bytes()).expect("stalled response head");
                std::io::Write::flush(&mut stalled).expect("stalled response head flush");
                let _ = stalled_head_tx.send(());
                if !withhold_head {
                    let _ = release_stalled_rx.recv_timeout(cleanup_deadline);
                }
                let _ = std::io::Write::write_all(&mut stalled, &vec![41; response_bytes]);
            });
            listener.set_nonblocking(true).expect("nonblocking sibling listener");
            let deadline = std::time::Instant::now() + cleanup_deadline;
            while std::time::Instant::now() < deadline {
                match listener.accept() {
                    Ok((mut sibling, _)) => {
                        sibling.set_read_timeout(Some(cleanup_deadline)).expect("sibling request read deadline");
                        let mut request = [0; 1024];
                        let _ = std::io::Read::read(&mut sibling, &mut request);
                        let _ = sibling_request_tx.send(());
                        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {response_bytes}\r\nConnection: close\r\n\r\n");
                        let _ = std::io::Write::write_all(&mut sibling, response.as_bytes());
                        let _ = std::io::Write::write_all(&mut sibling, &vec![43; response_bytes]);
                        break;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => std::thread::yield_now(),
                    Err(_) => break,
                }
            }
            let _ = stalled_thread.join();
        });
        Self { address, stalled_request, stalled_head, sibling_request, release_stalled: Some(release_stalled), thread: Some(thread) }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{path}", self.address)
    }

    fn release_stalled(&mut self) {
        if let Some(release) = self.release_stalled.take() {
            let _ = release.send(());
        }
    }

    fn finish(&mut self) {
        self.release_stalled();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
impl Drop for StalledNativeAssetServer {
    fn drop(&mut self) {
        self.finish();
    }
}

#[cfg(test)]
fn retire_component_world_fixture(runtime: &RuntimeMailbox) {
    for (host_id, surface_id, key) in [("component-world-a", "document-world-a", "world-a"), ("component-world-b", "document-world-b", "world-b")] {
        let mut target = crate::interpreter::fixture_scene_pointer_target("component-window", surface_id, key);
        target.host_id = host_id.into();
        target.kind = ui_wgpu::wgpu::SurfaceKind::World3d;
        let mut asset_terminal = false;
        for _ in 0..262_144 {
            asset_terminal = runtime.close_component_asset_step(&target);
            let _ = crate::pump_renderer_io_sessions(1);
            if asset_terminal {
                break;
            }
        }
        assert!(asset_terminal, "{host_id}'s asset authority reaches terminal before whole-World retirement");
        let mut runtime_owner = runtime.try_lock().expect("the fixture runtime returns for exact cleanup");
        let interaction = runtime_owner.interaction.as_mut().expect("the actual interaction owner");
        let Some(token) = interaction.shell.world3d_states.token(host_id) else { continue };
        let mut owner = interaction.shell.world3d_states.take_exact_to_retirement(token).expect("the exact World transfers to bounded cleanup");
        infinite_world::world::begin_world3d_dynamic_retirement(&mut owner.value);
        let mut sequence = 0;
        for _ in 0..262_144 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1),
                semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut sequence,
            );
            if infinite_world::world::step_world3d_dynamic_retirement(&mut owner.value, &mut context) && infinite_world::world::world3d_dynamic_retirement_terminal_is_empty(&owner.value) {
                break;
            }
        }
        assert!(infinite_world::world::world3d_dynamic_retirement_terminal_is_empty(&owner.value));
        drop(owner);
        interaction.shell.world3d_states.acknowledge_retired_owner();
    }
}

#[cfg(test)]
#[test]
fn a_temporarily_checked_out_runtime_is_busy_not_cancelled_for_its_exact_world_asset() {
    let runtime = runtime_with_component_worlds();
    let fetch = runtime.take_renderer_asset_step().expect("World A's real asset owner is checked out");
    let interaction = {
        let mut runtime_owner = runtime.try_lock().expect("the fixture runtime is locally available");
        runtime_owner.interaction.take().expect("the exact interaction owner checks out")
    };
    let current_while_checked_out = runtime.renderer_asset_current(&fetch);
    let cancelled_while_checked_out = runtime.renderer_asset_cancelled(&fetch);
    {
        let mut runtime_owner = runtime.try_lock().expect("the fixture runtime accepts exact handback");
        assert!(runtime_owner.interaction.replace(interaction).is_none());
    }
    assert!(runtime.return_renderer_asset_owner(fetch).is_ok(), "the exact asset owner returns after runtime handback");
    retire_component_world_fixture(&runtime);
    assert_eq!(current_while_checked_out, None, "a checked-out runtime reports temporary unavailability");
    assert!(!cancelled_while_checked_out, "temporary runtime unavailability is not an asset cancellation verdict");
}

#[cfg(test)]
#[test]
fn a_component_world_close_begins_with_its_exact_checked_out_asset_before_world_transfer() {
    let runtime = runtime_with_component_worlds();
    let fetch_a = runtime.take_renderer_asset_step().expect("World A's real asset owner is checked out");
    let token_a = match &fetch_a {
        crate::RendererAssetFetchOwner::World { surface_token, .. } => *surface_token,
        crate::RendererAssetFetchOwner::Shared(_) => panic!("the component fixture must own the fetched asset"),
    };
    let (expected_a, token_b) = {
        let runtime = runtime.try_lock().expect("the fixture runtime is locally available");
        let interaction = runtime.interaction.as_ref().expect("the actual interaction owner");
        (interaction.shell.world3d_states.token("component-world-a").expect("World A token"), interaction.shell.world3d_states.token("component-world-b").expect("World B token"))
    };
    assert_eq!(token_a, expected_a);
    assert_ne!(token_a, token_b);
    let mut target = crate::interpreter::fixture_scene_pointer_target("component-window", "document-world-a", "world-a");
    target.host_id = "component-world-a".to_string();
    target.kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let request = ComponentSurfaceCloseRequest { token: ComponentSurfaceCloseToken(1), owner: target, engine_token: None };
    let mut close = ComponentSurfaceCloseOwner::new(request);
    close.close_asset_world_step(&runtime);
    let phase_while_checked_out = format!("{:?}", close.phase);
    let (a_live_while_checked_out, b_unchanged_while_a_closes) = {
        let runtime = runtime.try_lock().expect("the fixture runtime is locally available");
        let interaction = runtime.interaction.as_ref().expect("the actual interaction owner");
        (interaction.shell.world3d_states.token("component-world-a") == Some(token_a), interaction.shell.world3d_states.token("component-world-b") == Some(token_b))
    };
    assert!(runtime.return_renderer_asset_owner(fetch_a).is_ok(), "World A's exact checked-out owner returns to its still-live closing authority");
    let mut close_turns = 0usize;
    while close.phase != ComponentSurfaceClosePhase::Terminal && close_turns < 262_144 {
        close.close_asset_world_step(&runtime);
        close_turns += 1;
    }
    let (a_retired, b_survived) = {
        let runtime = runtime.try_lock().expect("the fixture runtime is locally available");
        let interaction = runtime.interaction.as_ref().expect("the actual interaction owner");
        (interaction.shell.world3d_states.token("component-world-a").is_none(), interaction.shell.world3d_states.token("component-world-b") == Some(token_b))
    };
    let mut fetch_b = None;
    for _ in 0..crate::scenes::SCENE_SURFACE_CAPACITY {
        if let Some(owner) = runtime.take_renderer_asset_step() {
            fetch_b = Some(owner);
            break;
        }
    }
    let fetch_b = fetch_b.expect("a bounded surface scan finds World B's surviving request");
    let fetched_b_exactly = matches!(&fetch_b, crate::RendererAssetFetchOwner::World { surface_token, .. } if *surface_token == token_b);
    match &fetch_b {
        crate::RendererAssetFetchOwner::World { surface_token, .. } => assert_eq!(*surface_token, token_b),
        crate::RendererAssetFetchOwner::Shared(_) => panic!("World B's request remains component-owned"),
    }
    assert!(runtime.return_renderer_asset_owner(fetch_b).is_ok(), "World B's owner returns unchanged");
    retire_component_world_fixture(&runtime);
    assert_eq!(phase_while_checked_out, "Asset", "World transfer waits while World A's exact asset owner is checked out");
    assert!(a_live_while_checked_out, "the checked-out asset keeps its exact World reachable for handback");
    assert!(b_unchanged_while_a_closes && b_survived && fetched_b_exactly, "World B keeps its token, request, and owner while World A closes");
    assert!(close_turns < 262_144 && close.terminal_is_empty() && a_retired, "World A reaches exact terminal retirement after its asset owner returns");
}

/// 📬️ A busy seal or handback retains the exact native response until its World authority returns.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_asset_handoff_preserves_its_exact_response_across_seal_and_return_contention() {
    let runtime = runtime_with_component_worlds();
    let mut fetch = runtime.take_renderer_asset_step().expect("World A request");
    assert!(matches!(runtime.reserve_renderer_asset_response(&mut fetch, 7), crate::RendererAssetSealStep::Granted));
    fetch.owner_mut().push_page(crate::WorldAssetResponsePage::try_from_owned(vec![41; 7]).unwrap()).unwrap();
    let token = fetch.owner().token();
    runtime.retain_native_asset_handoff(fetch, None, true);
    let interaction = runtime.try_lock().unwrap().interaction.take().unwrap();
    let busy_seal = runtime.pump_native_asset_handoff_step(false);
    let retained_before_seal = {
        let slot = runtime.0.native_asset_handoff.lock().unwrap();
        let owner = slot.as_ref().unwrap().fetch.as_ref().unwrap().owner();
        (owner.token(), owner.received_bytes(), owner.is_sealed())
    };
    runtime.try_lock().unwrap().interaction = Some(interaction);
    let seal = runtime.pump_native_asset_handoff_step(false);
    let interaction = runtime.try_lock().unwrap().interaction.take().unwrap();
    let busy_return = runtime.pump_native_asset_handoff_step(false);
    let retained_after_seal = {
        let slot = runtime.0.native_asset_handoff.lock().unwrap();
        let owner = slot.as_ref().unwrap().fetch.as_ref().unwrap().owner();
        (owner.token(), owner.received_bytes(), owner.is_sealed())
    };
    runtime.try_lock().unwrap().interaction = Some(interaction);
    let returned = runtime.pump_native_asset_handoff_step(false);
    let mut completed = (0..crate::scenes::SCENE_SURFACE_CAPACITY).find_map(|_| runtime.take_completed_renderer_asset_step()).expect("the same sealed response returns to its decoder lane");
    let completed_token = completed.owner().token();
    completed.begin_close();
    for _ in 0..4096 {
        if completed.close_step() {
            break;
        }
    }
    assert!(runtime.finish_renderer_asset_owner(completed, None).is_ok());
    let fault = runtime.0.frame_fault.lock().unwrap().clone();
    retire_component_world_fixture(&runtime);
    assert_eq!((busy_seal, seal, busy_return, returned), (Some(false), Some(true), Some(false), Some(true)));
    assert_eq!(retained_before_seal, (token, 7, false));
    assert_eq!(retained_after_seal, (token, 7, true));
    assert_eq!(completed_token, token);
    assert!(fault.is_none());
    eprintln!("[DEBUG] native handoff retained token {token:?} and all seven bytes across two busy ownership boundaries");
}

/// 🛑️ Closing one component returns its pending native response without faulting or cancelling its sibling.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_asset_handoff_cancellation_returns_the_closing_request_and_preserves_its_sibling() {
    let runtime = runtime_with_component_worlds();
    let mut fetch = runtime.take_renderer_asset_step().expect("World A request");
    assert!(matches!(runtime.reserve_renderer_asset_response(&mut fetch, 7), crate::RendererAssetSealStep::Granted));
    fetch.owner_mut().push_page(crate::WorldAssetResponsePage::try_from_owned(vec![41; 7]).unwrap()).unwrap();
    runtime.retain_native_asset_handoff(fetch, None, true);
    let mut target = crate::interpreter::fixture_scene_pointer_target("component-window", "document-world-a", "world-a");
    target.host_id = "component-world-a".into();
    target.kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let close_with_checked_out_response = runtime.close_component_asset_step(&target);
    let cancelled = runtime.pump_native_asset_handoff_step(false);
    let returned = runtime.pump_native_asset_handoff_step(false);
    let mut terminal = false;
    for _ in 0..4096 {
        if runtime.close_component_asset_step(&target) {
            terminal = true;
            break;
        }
    }
    let sibling = (0..crate::scenes::SCENE_SURFACE_CAPACITY).find_map(|_| runtime.take_renderer_asset_step()).expect("World B request survives");
    let sibling_current = runtime.renderer_asset_current(&sibling);
    assert!(runtime.return_renderer_asset_owner(sibling).is_ok());
    let fault = runtime.0.frame_fault.lock().unwrap().clone();
    retire_component_world_fixture(&runtime);
    assert!(!close_with_checked_out_response);
    assert_eq!((cancelled, returned), (Some(true), Some(true)));
    assert!(terminal && fault.is_none());
    assert_eq!(sibling_current, Some(true));
    eprintln!("[DEBUG] native component cancellation returned its pending response and preserved the sibling without a frame fault");
}

/// 🛑️ A component close interrupts its exact stalled native body before the sibling enters the single-fetch lane.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_stalled_native_component_body_is_aborted_before_its_sibling_enters_the_single_fetch_lane() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).expect("native asset contract");
    let transport = &law["transportCancellation"];
    let observation_deadline = std::time::Duration::from_millis(transport["observationDeadlineMs"].as_u64().expect("observation deadline"));
    let cleanup_deadline = std::time::Duration::from_millis(transport["cleanupDeadlineMs"].as_u64().expect("cleanup deadline"));
    let mut server = StalledNativeAssetServer::new(cleanup_deadline, transport["responseBytes"].as_u64().expect("response bytes") as usize);
    let runtime = runtime_with_component_world_asset_urls(&server.url("/world-a.glb"), &server.url("/world-b.glb"));
    assert!(runtime.pump_native_asset(), "World A enters the real native fetch lane");
    let stalled_head_observed = server.stalled_head.recv_timeout(cleanup_deadline).is_ok();
    let mut target = crate::interpreter::fixture_scene_pointer_target("component-window", "document-world-a", "world-a");
    target.host_id = transport["stalledHostId"].as_str().expect("stalled host id").to_string();
    target.kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let deadline = std::time::Instant::now() + observation_deadline;
    let mut sibling_started_while_stalled = false;
    let mut close_terminal = false;
    while std::time::Instant::now() < deadline {
        close_terminal |= runtime.close_component_asset_step(&target);
        let _ = runtime.pump_native_asset();
        if server.sibling_request.try_recv().is_ok() {
            sibling_started_while_stalled = true;
            break;
        }
        std::thread::yield_now();
    }

    server.release_stalled();
    let cleanup_limit = std::time::Instant::now() + cleanup_deadline;
    let mut sibling_started = sibling_started_while_stalled;
    while std::time::Instant::now() < cleanup_limit {
        close_terminal |= runtime.close_component_asset_step(&target);
        let _ = runtime.pump_native_asset();
        sibling_started |= server.sibling_request.try_recv().is_ok();
        let fetch_idle = !runtime.0.native_asset_fetching.load(Ordering::Acquire);
        let handoff_idle = runtime.0.native_asset_handoff.lock().expect("native handoff cleanup").is_none();
        if close_terminal && sibling_started && fetch_idle && handoff_idle {
            break;
        }
        std::thread::yield_now();
    }
    server.finish();
    for _ in 0..4096 {
        let _ = runtime.pump_native_asset();
        if !runtime.0.native_asset_fetching.load(Ordering::Acquire) && runtime.0.native_asset_handoff.lock().expect("native handoff terminal").is_none() {
            break;
        }
        std::thread::yield_now();
    }
    let frame_faults = usize::from(runtime.0.frame_fault.lock().expect("native frame fault").is_some());
    let terminal_native_owners = usize::from(runtime.0.native_asset_fetching.load(Ordering::Acquire)) + usize::from(runtime.0.native_asset_handoff.lock().expect("native handoff owner").is_some()) + runtime.native_asset_transport_live_count();
    retire_component_world_fixture(&runtime);

    assert!(stalled_head_observed, "World A reaches a real response body before close");
    assert!(sibling_started_while_stalled, "closing World A must interrupt its stalled body before World B enters the one-fetch lane");
    assert!(close_terminal && sibling_started, "World A reaches terminal component close and World B resumes");
    assert_eq!(frame_faults, transport["frameFaults"].as_u64().expect("frame faults") as usize);
    assert_eq!(terminal_native_owners, transport["terminalTransportLeases"].as_u64().expect("terminal transport owners") as usize);
}

/// 🛡️ Closing one component interrupts its exact native request even before the response head arrives.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_withheld_native_component_head_is_cancelled_before_its_sibling_enters_the_single_fetch_lane() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).expect("native asset contract");
    let transport = &law["transportCancellation"];
    let observation_deadline = std::time::Duration::from_millis(transport["observationDeadlineMs"].as_u64().expect("observation deadline"));
    let cleanup_deadline = std::time::Duration::from_millis(transport["cleanupDeadlineMs"].as_u64().expect("cleanup deadline"));
    let mut server = StalledNativeAssetServer::withholding_head(cleanup_deadline, transport["responseBytes"].as_u64().expect("response bytes") as usize);
    let runtime = runtime_with_component_world_asset_urls(&server.url("/world-a.glb"), &server.url("/world-b.glb"));
    assert!(runtime.pump_native_asset(), "World A enters the real native fetch lane");
    let withheld_request_observed = server.stalled_request.recv_timeout(cleanup_deadline).is_ok();
    let mut target = crate::interpreter::fixture_scene_pointer_target("component-window", "document-world-a", "world-a");
    target.host_id = transport["stalledHostId"].as_str().expect("stalled host id").to_string();
    target.kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let deadline = std::time::Instant::now() + observation_deadline;
    let mut sibling_started_while_head_withheld = false;
    let mut close_terminal = false;
    while std::time::Instant::now() < deadline {
        close_terminal |= runtime.close_component_asset_step(&target);
        let _ = runtime.pump_native_asset();
        if server.sibling_request.try_recv().is_ok() {
            sibling_started_while_head_withheld = true;
            break;
        }
        std::thread::yield_now();
    }

    server.release_stalled();
    let released_head_observed = server.stalled_head.recv_timeout(cleanup_deadline).is_ok();
    let cleanup_limit = std::time::Instant::now() + cleanup_deadline;
    let mut sibling_started = sibling_started_while_head_withheld;
    while std::time::Instant::now() < cleanup_limit {
        close_terminal |= runtime.close_component_asset_step(&target);
        let _ = runtime.pump_native_asset();
        sibling_started |= server.sibling_request.try_recv().is_ok();
        let fetch_idle = !runtime.0.native_asset_fetching.load(Ordering::Acquire);
        let handoff_idle = runtime.0.native_asset_handoff.lock().expect("native handoff cleanup").is_none();
        if close_terminal && sibling_started && fetch_idle && handoff_idle {
            break;
        }
        std::thread::yield_now();
    }
    server.finish();
    for _ in 0..4096 {
        let _ = runtime.pump_native_asset();
        if !runtime.0.native_asset_fetching.load(Ordering::Acquire) && runtime.0.native_asset_handoff.lock().expect("native handoff terminal").is_none() {
            break;
        }
        std::thread::yield_now();
    }
    let frame_faults = usize::from(runtime.0.frame_fault.lock().expect("native frame fault").is_some());
    let terminal_native_owners = usize::from(runtime.0.native_asset_fetching.load(Ordering::Acquire)) + usize::from(runtime.0.native_asset_handoff.lock().expect("native handoff owner").is_some()) + runtime.native_asset_transport_live_count();
    retire_component_world_fixture(&runtime);

    assert!(withheld_request_observed && released_head_observed, "World A reaches the server before close and cleanup releases its withheld head");
    assert!(sibling_started_while_head_withheld, "closing World A must interrupt its withheld response head before World B enters the one-fetch lane");
    assert!(close_terminal && sibling_started, "World A reaches terminal component close and World B resumes");
    assert_eq!(frame_faults, transport["frameFaults"].as_u64().expect("frame faults") as usize);
    assert_eq!(terminal_native_owners, transport["terminalTransportLeases"].as_u64().expect("terminal transport owners") as usize);
}

/// 🗃️ A local page returned after component cancellation enters bounded handback before its World owner.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_cancelled_local_component_page_is_retained_for_bounded_handback_without_response_publication() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).expect("native asset contract");
    let local = &law["localPageCancellation"];
    let page_bytes = local["pageBytes"].as_u64().expect("local page bytes") as usize;
    let path = std::env::temp_dir().join(format!("semio-native-component-page-{}.bin", std::process::id()));
    std::fs::write(&path, vec![47; page_bytes]).expect("write exact local page fixture");
    let url = format!("file://{}", path.to_string_lossy());
    let runtime = runtime_with_component_world_asset_urls(&url, &url);
    let surface_token = {
        let owner = runtime.try_lock().expect("local page fixture runtime");
        owner.interaction.as_ref().expect("local page fixture interaction").shell.world3d_states.token("component-world-a").expect("World A surface token")
    };
    let barrier = std::sync::Arc::new(crate::NativeAssetPageTestBarrier { surface_token, entered: AtomicBool::new(false), release: AtomicBool::new(false) });
    *runtime.0.native_asset_page_barrier.lock().expect("install local page test barrier") = Some(barrier.clone());
    assert!(runtime.pump_native_asset(), "World A enters the real local-file fetch lane");
    let observation_deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !barrier.entered.load(Ordering::Acquire) && std::time::Instant::now() < observation_deadline {
        let _ = crate::pump_renderer_io_sessions(1);
        std::thread::yield_now();
    }
    let entered = barrier.entered.load(Ordering::Acquire);
    let mut target = crate::interpreter::fixture_scene_pointer_target("component-window", "document-world-a", "world-a");
    target.host_id = "component-world-a".into();
    target.kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let _ = runtime.close_component_asset_step(&target);
    barrier.release.store(true, Ordering::Release);
    let handback_deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while runtime.0.native_asset_fetching.load(Ordering::Acquire) && std::time::Instant::now() < handback_deadline {
        let _ = crate::pump_renderer_io_sessions(1);
        std::thread::yield_now();
    }
    let (published_cancelled_bytes, retained_page_bytes, seal) = runtime
        .0
        .native_asset_handoff
        .lock()
        .expect("local page handback owner")
        .as_ref()
        .map(|handoff| (handoff.fetch.as_ref().expect("local page request owner").owner().received_bytes(), handoff.payload.as_ref().map_or(0, semio_framework_job::RetainedJobPayload::len), handoff.seal))
        .unwrap_or((0, 0, false));
    *runtime.0.native_asset_page_barrier.lock().expect("clear local page test barrier") = None;
    let mut page_retirement_turns = 0usize;
    for _ in 0..16 {
        let payload_before = runtime.0.native_asset_handoff.lock().expect("local page payload before close").as_ref().is_some_and(|handoff| handoff.payload.is_some());
        if runtime.0.native_asset_handoff.lock().expect("local page handoff terminal probe").is_none() {
            break;
        }
        let _ = runtime.pump_native_asset_handoff_step(true);
        let payload_after = runtime.0.native_asset_handoff.lock().expect("local page payload after close").as_ref().is_some_and(|handoff| handoff.payload.is_some());
        page_retirement_turns += usize::from(payload_before && !payload_after);
    }
    let mut close_terminal = false;
    for _ in 0..4096 {
        close_terminal |= runtime.close_component_asset_step(&target);
        let _ = crate::pump_renderer_io_sessions(1);
        if close_terminal {
            break;
        }
    }
    let frame_faults = usize::from(runtime.0.frame_fault.lock().expect("local page frame fault").is_some());
    let terminal_native_owners = usize::from(runtime.0.native_asset_fetching.load(Ordering::Acquire)) + usize::from(runtime.0.native_asset_handoff.lock().expect("local page handoff owner").is_some()) + runtime.native_asset_transport_live_count();
    retire_component_world_fixture(&runtime);
    let _ = std::fs::remove_file(path);

    assert!(entered, "the real local ReadPage returns its retained page into the controlled cancellation boundary");
    assert_eq!(published_cancelled_bytes, local["publishedCancelledBytes"].as_u64().expect("published cancelled bytes") as usize);
    assert_eq!(retained_page_bytes, local["retainedPageBytesBeforeClose"].as_u64().expect("retained page bytes") as usize);
    assert!(!seal, "a cancelled local page cannot request response publication");
    assert_eq!(page_retirement_turns, local["pageRetirementTurns"].as_u64().expect("page retirement turns") as usize);
    assert!(close_terminal, "World A closes after its page and request owners return");
    assert_eq!(frame_faults, local["frameFaults"].as_u64().expect("frame faults") as usize);
    assert_eq!(terminal_native_owners, local["terminalNativeOwners"].as_u64().expect("terminal native owners") as usize);
}

/// 🌐️ Drives the actual asset and World retirement bridge for a retained Shell fixture.
#[cfg(test)]
pub(crate) fn finish_world_fixture_component_close(shell: crate::shell::ShellState) -> crate::shell::ShellState {
    let Some(request) = take_component_surface_close_request() else { return shell };
    assert_eq!(request.owner.kind, ui_wgpu::wgpu::SurfaceKind::World3d);
    assert!(request.engine_token.is_none(), "this World fixture has no paired GPU engine token");
    let runtime = runtime_with_shell(shell);
    let mut close = ComponentSurfaceCloseOwner::new(request);
    for _ in 0..262_144 {
        if close.terminal_is_empty() { break; }
        close.close_asset_world_step(&runtime);
    }
    assert!(close.terminal_is_empty(), "the production World close lane returns its exact owner");
    assert!(publish_component_surface_close_terminal(close.request.token));
    let mut owner = runtime.try_lock().expect("the fixture returns its Shell owner");
    std::mem::replace(&mut owner.interaction.as_mut().expect("the interaction is returned").shell, crate::shell::ShellState::new(Vec::new(), "closed-fixture".to_string()))
}
