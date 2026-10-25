/// 🧊️ GLB world mesh, swapchain and original decoder close: the fetched and catalogued GLB resident world mesh, the swapchain opportunity, the glass snapshot, the non resident mesh skip, the ink barrier, the component close, the watchdog signature, the original decoder close, the engine packet pages and the cancelled dispatch future.

/// 🥽️📥️ The url half of the World3d mesh lane, end to end inside one surface: a fetched GLB becomes
/// the RESIDENT mesh under the very id the wire names it by, carrying real positions and indices, in
/// the world's Z-up frame.
///
/// 🩸️ Nothing in the repo ever reserved a `WorldAssetRequestKind::Glb` in production — the bridge
/// dropped every url-declared mesh before anything could ask for it, so this whole path (probe →
/// GLB structure → schema → materializer → `publish_world3d_asset_mesh_lease`) had never run once.
/// Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3d-world3d-glb-url-lane.md`.
#[test]
fn a_fetched_glb_becomes_the_resident_world_mesh_its_url_names() {
    use infinite_world::world::{
        begin_world3d_dynamic_retirement, finish_world3d_asset, publish_world3d_asset_mesh, reserve_world3d_asset_request, reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response, step_world3d_dynamic_retirement,
        take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_dynamic_retirement_terminal_is_empty, World3dState,
    };

    let mut json = br#"{"scene":0,"scenes":[{"nodes":[0]}],"nodes":[{"mesh":0}],"accessors":[{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3"},{"bufferView":1,"componentType":5121,"count":3,"type":"SCALAR"}],"bufferViews":[{"buffer":0,"byteOffset":0,"byteLength":36},{"buffer":0,"byteOffset":36,"byteLength":3}],"meshes":[{"primitives":[{"attributes":{"POSITION":0},"indices":1,"mode":4}]}]}"#.to_vec();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let mut bin = Vec::new();
    for position in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        for component in position {
            bin.extend_from_slice(&component.to_le_bytes());
        }
    }
    bin.extend_from_slice(&[0, 1, 2, 0]);
    let total = 12 + 8 + json.len() + 8 + bin.len();
    let mut glb = Vec::with_capacity(total);
    glb.extend_from_slice(b"glTF");
    glb.extend_from_slice(&2u32.to_le_bytes());
    glb.extend_from_slice(&(total as u32).to_le_bytes());
    glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
    glb.extend_from_slice(b"JSON");
    glb.extend_from_slice(&json);
    glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
    glb.extend_from_slice(b"BIN\0");
    glb.extend_from_slice(&bin);

    let url = "/mesh/🧊️probe.glb";
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::Glb, url).expect("the surface admits its own GLB request");
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("the host drains exactly the admitted request");
    assert_eq!(owner.url(), url);
    reserve_world3d_asset_response(&mut state.assets, &mut owner, glb.len()).expect("response credits");
    for chunk in glb.chunks(64) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(chunk.to_vec()).expect("bounded response page")).expect("page admission");
    }
    seal_world3d_asset_response(&mut state.assets, &mut owner).expect("sealed response");
    return_world3d_asset(&mut state.assets, owner).expect("handback");

    let owner = take_next_completed_world3d_asset_step(&mut state.assets).expect("completed decode owner");
    let mut probe = RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner));
    let mut ready = false;
    for _ in 0..16_384 {
        match probe.step() {
            RendererAssetProbeStep::Pending => {}
            RendererAssetProbeStep::Ready => {
                ready = true;
                break;
            }
            RendererAssetProbeStep::Reject(detail) => panic!("a valid world GLB was refused: {detail}"),
            RendererAssetProbeStep::Fault(detail) => panic!("a valid world GLB faulted: {detail}"),
        }
    }
    assert!(ready, "the GLB decode must reach Ready");
    let asset = probe.take_ready_mesh_asset().expect("the decoded mesh and appearance asset");
    publish_world3d_asset_mesh(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, url, asset).expect("the decoded GLB publishes under the url's own mesh id");

    let resident = state.mesh_lease("mesh:🧊️probe").expect("the mesh is resident under `mesh_id_from_url`, the id the wire names it by");
    let schema = resident.schema().expect("resident mesh schema");
    assert_eq!((schema.vertices, schema.indices), (3, 3), "the resident mesh carries real positions and indices, not an empty placeholder");
    assert_eq!((schema.edges, schema.edge_ids), (3, 0), "derived render outlines carry no selectable semantic edge identities");
    let mut positions = Vec::new();
    let mut cursor = resident.cursor(Mesh3dField::Positions).unwrap();
    while let Some(Mesh3dItem::Vec3(value)) = cursor.read_next().unwrap() {
        positions.push(value);
    }
    assert_eq!(positions, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]], "glTF's Y-up geometry arrives in the world's Z-up frame, the way React's `<group rotation={{[π/2, 0, 0]}}>` delivers it");

    probe.begin_close();
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("terminal owner") else { panic!("shared probe") };
    finish_world3d_asset(&mut state.assets, owner).expect("terminal handback");
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    let mut caller=OriginalRendererFixture::new();
    for _ in 0..4_096 {
        let terminal=caller.turn(|context| {
        let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,terrain_grant,context);
        assert!(terrain.ownership.fits(terrain_grant));
        assert!(!matches!(terrain.step,infinite_world::world::WorldTerrainMeshPublicationStep::Fault(_)));
        Ok(terrain.step==infinite_world::world::WorldTerrainMeshPublicationStep::Idle && step_world3d_dynamic_retirement(&mut state, context))
        }).unwrap();if terminal { break; }
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

/// 🧊️ A REAL catalogued mesh — a whole-building export, not a synthetic four-vertex fixture —
/// rides the surface's own asset lane into its mesh table. Every fixed credit on the way is a
/// ceiling this asset could hit: `GLB_SCHEMA_ITEM_CAPACITY` (the metabolism capsule and the
/// puzzle3d concrete-forest halves each declare over 200 accessors and bufferViews, and the forest
/// packs 76 primitives into ONE mesh), `GLB_SCHEMA_OUTPUT_BYTES`, `WORLD_ASSET_RESPONSE_PAGE_BYTES`
/// (a 16 KiB BYOB page, so a real asset always spans several) and the mesh authority's own schema.
///
/// Measured on this asset: 1 472 vertices / 5 250 indices / 1 472 uvs in 26 174 bounded probe steps;
/// `/mesh/🧊️hexagonal-cut-concrete-forest-left.glb`, the puzzle3d playground's own, answers
/// 847 / 2 874 / 847 in 19 884. Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w3d-world3d-glb-url-lane.md`.
#[test]
fn a_real_catalogued_glb_streams_through_the_surfaces_own_asset_lane_into_its_mesh_table() {
    use infinite_world::world::{
        begin_world3d_dynamic_retirement, finish_world3d_asset, publish_world3d_asset_mesh_lease, reserve_world3d_asset_request, reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response, step_world3d_dynamic_retirement,
        take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_dynamic_retirement_terminal_is_empty, World3dState,
    };
    let glb: &[u8] = semio_framework_os_renderer_wgpu_test_support::repo_bytes("🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let url = "/mesh/🧊️capsule_J.glb";
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::Glb, url).unwrap();
    let mut owner = take_next_world3d_asset(&mut state.assets).unwrap();
    reserve_world3d_asset_response(&mut state.assets, &mut owner, glb.len()).unwrap();
    for chunk in glb.chunks(16 * 1024) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(chunk.to_vec()).unwrap()).unwrap();
    }
    seal_world3d_asset_response(&mut state.assets, &mut owner).unwrap();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    let owner = take_next_completed_world3d_asset_step(&mut state.assets).unwrap();
    let mut probe = RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner));
    let mut steps = 0u64;
    loop {
        steps += 1;
        match probe.step() {
            RendererAssetProbeStep::Pending => {}
            RendererAssetProbeStep::Ready => break,
            RendererAssetProbeStep::Reject(detail) => panic!("a catalogued mesh was refused after {steps} steps: {detail}"),
            RendererAssetProbeStep::Fault(detail) => panic!("a catalogued mesh faulted after {steps} steps: {detail}"),
        }
        assert!(steps < 1_000_000, "no Ready within the step ceiling");
    }
    let asset = probe.take_ready_mesh_asset().unwrap();
    publish_world3d_asset_mesh(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, url, asset).unwrap();
    let resident = state.mesh_lease("mesh:🧊️capsule_J").unwrap();
    let schema = resident.schema().unwrap();
    assert_eq!((schema.vertices, schema.indices, schema.uvs), (1_472, 5_250, 1_472), "the whole export lands, every primitive welded into one mesh");
    assert!(steps < 100_000, "the decode stays inside a bounded step count: {steps}");
    probe.begin_close();
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!() };
    finish_world3d_asset(&mut state.assets, owner).unwrap();
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    let mut caller=OriginalRendererFixture::new();
    for _ in 0..8_192 {
        let terminal=caller.turn(|context| {
        let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,terrain_grant,context);
        assert!(terrain.ownership.fits(terrain_grant));
        assert!(!matches!(terrain.step,infinite_world::world::WorldTerrainMeshPublicationStep::Fault(_)));
        Ok(terrain.step==infinite_world::world::WorldTerrainMeshPublicationStep::Idle && step_world3d_dynamic_retirement(&mut state, context))
        }).unwrap();if terminal { break; }
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

/// 🖼️ LAW: the swapchain texture is acquired, written and presented inside ONE prepared opportunity,
/// and nothing before it may touch the surface.
///
/// 🩸️ A browser expires the canvas's current texture at the end of the task that obtained it and
/// presents whatever it held at that moment. The ladder used to acquire in `AcquireSurface`, then
/// yield at the host pump's own deadline, and only later blit the scene onto it — so on every boot
/// where those two landed in different pumps the canvas presented the untouched (opaque black)
/// texture and the composite was thrown away. Measured 6/6 on the live serve: every black boot had
/// `gpu ladder 2->3` and `gpu ladder 6->7` in DIFFERENT `pump=` values, every painted boot had them
/// in the same one (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `🗑️generated/w3c-ladder-2`). The ladder now composites into `PreparedCompositeTarget` and the
/// terminal `Present` phase is the only code in the file that names `get_current_texture`.
#[test]
fn the_swapchain_is_acquired_written_and_presented_in_one_prepared_opportunity() {
    let ladder = GPU_SOURCE().split("pub fn prepared_present_step").nth(1).expect("the prepared present ladder");
    let ladder = &ladder[..ladder.find("fn encode_prepared_draw_scalar").unwrap_or(ladder.len())];
    assert_eq!(ladder.matches("get_current_texture").count(), 1, "exactly one phase may acquire the surface");
    assert_eq!(ladder.matches("frame.present()").count(), 1, "and exactly one presents it");
    assert!(!GPU_SOURCE().contains("PreparedGpuPresentPhase::AcquireSurface"), "acquiring in its own phase is the defect this law exists for");
    assert!(!GPU_SOURCE().contains("PreparedGpuPresentPhase::CreateView"), "and so is viewing it in its own phase");

    let present = ladder.split("PreparedGpuPresentPhase::Present =>").nth(1).expect("the terminal present phase");
    let present = &present[..present.find("PreparedGpuPresentPhase::Complete =>").unwrap_or(present.len())];
    let acquire = present.find("get_current_texture").expect("the acquire");
    let blit = present.find("blit_prepared_composite").expect("the composite blit onto the acquired texture");
    let submit = present.find("self.queue.submit").expect("the submit");
    let present_call = present.find("frame.present()").expect("the present");
    assert!(acquire < blit && blit < submit && submit < present_call, "acquire, write, submit and present are one straight line inside one opportunity");

    let composite = ladder.split("PreparedGpuPresentPhase::InitializeComposite =>").nth(1).expect("the composite phase");
    assert!(composite[..composite.find("PreparedGpuPresentPhase::Commands =>").unwrap_or(composite.len())].contains("composite.view()"), "the scene blit lands offscreen");
    let glass = ladder.split("PreparedGpuPresentPhase::CompositeGlass =>").nth(1).expect("the glass phase");
    assert!(glass[..glass.find("PreparedGpuPresentPhase::Present =>").unwrap_or(glass.len())].contains("composite.view()"), "and so does every glass region");
}

/// 🫧 Every glass samples the accumulated authored stream before later content is encoded.
#[test]
fn glass_snapshots_the_accumulated_composite_at_its_authored_command() {
    let ladder = GPU_SOURCE().split("pub fn prepared_present_step").nth(1).expect("the prepared present ladder");
    let commands = ladder.split("PreparedGpuPresentPhase::Commands =>").nth(1).expect("the command phase");
    let commands = &commands[..commands.find("PreparedGpuPresentPhase::SnapshotBackdrop =>").expect("the snapshot phase")];
    assert!(commands.contains("DrawMeasureCursor::Glass(_)"));
    assert!(commands.contains("prepared_command_clip_piece(draw, draw_cursor, cursor.clip_piece"), "every authored color command resolves one silhouette piece at a time");
    assert!(commands.contains("self.encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay(), None)?"), "an unclipped scalar is encoded once");
    assert!(commands.contains("self.encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay(), Some(scissor))?"), "a clipped scalar is encoded once per nonempty piece");
    assert!(commands.contains("PreparedCommandClipPiece::Scissor(_scissor) if matches!(draw_cursor, DrawMeasureCursor::Glass(_))"), "glass snapshots before drawing its first nonempty piece");
    assert!(commands.contains("cursor.phase = PreparedGpuPresentPhase::SnapshotBackdrop"));
    let backdrop = ladder.split("PreparedGpuPresentPhase::SnapshotBackdrop =>").nth(1).unwrap();
    let backdrop = &backdrop[..backdrop.find("PreparedGpuPresentPhase::BlurScene =>").unwrap()];
    assert!(backdrop.contains("prepared_glass_command(packet, cursor.command)?"), "the snapshot validates its exact command owner");
    assert!(backdrop.contains("blit_prepared_composite(&self.device, &mut encoder, scene.mip_view(0), composite)"), "earlier content forms the backdrop");
    assert!(!backdrop.contains("cursor.command ="), "snapshotting retains the current glass command");
    let glass = ladder.split("PreparedGpuPresentPhase::CompositeGlass =>").nth(1).unwrap();
    let glass = &glass[..glass.find("PreparedGpuPresentPhase::Present =>").unwrap()];
    assert!(glass.contains("encode_prepared_glass_scalar"));
    assert!(glass.contains("cursor.command.checked_add(1)"), "one completed glass advances exactly one command");
    assert!(glass.contains("cursor.phase = PreparedGpuPresentPhase::Commands"), "later content resumes after its glass");
    assert!(!GPU_SOURCE().contains("ForegroundCommands"), "no foreground replay can overprint a later popup");
}

/// 🧷️ LAW: a prepared world draw whose mesh is not resident at SUBMIT is skipped and reported, never
/// a fault — the pinning law's last line of defence.
///
/// 🩸️ `mesh_store.get_versioned` answers a prepared world draw hundreds of host steps after the build
/// that authored it, and a miss used to be `Err("prepared world mesh was missing")`, which
/// `AppPresentPhase::Render` turns into `worker-present-failed` and the browser shell turns into a
/// dead page. One unbacked ghost draw — a placeholder mesh lease still being admitted — therefore
/// killed the whole puzzle3d journey at t≈42.7 s (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w11a-prepared-world-mesh-missing.md`). React keeps rendering: its loader simply returns nothing
/// until the mesh resolves, which is exactly what a skipped draw is.
#[test]
fn a_non_resident_prepared_world_mesh_is_skipped_and_reported_not_faulted() {
    assert!(DRAW_SOURCE().contains("let Some(mesh) = mesh_store.get_versioned(mesh_key, mesh_version) else { return Ok(false) };"), "the encoder answers a non-resident mesh with a skip");
    assert!(!DRAW_SOURCE().contains("prepared world mesh was missing"), "and no fault string survives anywhere in the encoder");
    assert!(GPU_SOURCE().contains("self.missing_world_mesh = Some((draw_owner.mesh_key.clone(), draw_owner.mesh_version));"), "the GPU context records exactly which mesh was skipped");
    assert!(GPU_SOURCE().contains("pub fn take_missing_world_mesh"), "and hands it over once");
    assert!(LIBRARY_SOURCE().contains("os_host prepared world mesh was not resident, draw skipped"), "which the host reports on the transition");
    let render = LIBRARY_SOURCE().split("AppPresentPhase::Render => {").nth(1).expect("the render phase");
    assert!(render[..render.find("AppPresentPhase::CloseGpu =>").unwrap_or(render.len())].contains("take_missing_world_mesh()"), "from the phase that submits the frame");
}

#[test]
fn presented_ink_intent_barrier_progresses_before_gpu_submit_and_holds_runtime_ingress_until_acknowledgement() {
    let render = LIBRARY_SOURCE().split("AppPresentPhase::Render => {").nth(1).expect("the render phase");
    let render = &render[..render.find("AppPresentPhase::CloseGpu =>").expect("the close-GPU phase")];
    let progress = render.find("progress_presented_input_candidate").expect("the presenter-owned input barrier progress seam");
    let submit = render.find("begin_prepared_present").expect("the first GPU submit operation");
    assert!(progress < submit, "the old presented interaction reaches a terminal state before candidate pixels can submit");

    let acknowledge = LIBRARY_SOURCE().split("AppPresentPhase::Acknowledge => {").nth(1).expect("the acknowledgement phase");
    let acknowledge = &acknowledge[..acknowledge.find("AppPresentPhase::ProgressAcknowledge =>").expect("the next presenter phase")];
    let input_ack = acknowledge.find("shell.acknowledge_presented_input").expect("the exact input witness acknowledgement");
    let packet_ack = acknowledge.find("gate.acknowledge_presented").expect("the prepared packet acknowledgement");
    assert!(input_ack < packet_ack, "a refused input witness keeps the packet witness live for a Pending retry");

    let host_build = WINT_APP_SOURCE().split("fn build_and_publish_snapshot(").nth(1).expect("the host build pump");
    let host_build = &host_build[..host_build.find("fn present_snapshot").expect("the host presentation half")];
    let ingress_gate = host_build.find("holds_presented_input_publication").expect("the input-publication critical section");
    let runtime_pump = host_build.find("pump_pending_applies").expect("the runtime apply pump");
    assert!(ingress_gate < runtime_pump, "runtime input cannot enter between the last preflight and acknowledgement");
}

#[test]
fn component_close_retires_its_creating_frame_before_external_progress_and_readmits_after_terminal() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️component-close-frame-turn/🔣️.json")).expect("component-close frame-turn fixture");
    assert_ne!(law["closeHost"], law["liveHost"]);
    assert_eq!(law["maxCloseUnitsPerTurn"], 1);
    assert_eq!(law["expected"]["closeTerminal"], true);
    assert_eq!(law["expected"]["sameGenerationReadmitted"], true);
    assert_eq!(law["expected"]["closingHostPublications"].as_array().map(Vec::len), Some(0));

    let host_build = WINT_APP_SOURCE().split("fn build_and_publish_snapshot(").nth(1).expect("the host build pump");
    let host_build = &host_build[..host_build.find("fn present_snapshot").expect("the host presentation half")];
    let close_step = host_build.find("self.advance_component_surface_close(cx)?").expect("one bounded component-close step");
    let event_drain = host_build.find("if !self.events.is_empty()").expect("the unrelated input drain");
    let close_gate = host_build.find("if !component_close_pending").expect("the component-close frame-admission gate");
    let frame_admission = host_build.find("self.presenter.admit_next_frame").expect("the unrelated frame admission");
    let snapshot_publication = host_build.find("self.snapshot_sink.publish").expect("the immutable snapshot publication");
    assert!(close_step < event_drain && event_drain < close_gate && close_gate < frame_admission && frame_admission < snapshot_publication, "input remains queued while the bridge gates frame admission until terminal");
    assert!(host_build[close_step..event_drain].contains("InvalidationReason::RESOURCE_READY"), "the nonterminal close retains its bounded progress wake while the sibling advances");

    let close_owner = OS_HOST_SOURCE().split("pub fn advance_component_surface_close(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {").nth(1).expect("the component-close owner");
    let admission = &close_owner[..close_owner.find("let Some(owner)").expect("the active exact close owner")];
    assert!(admission.contains("self.presenter.has_pending_presentation()"), "an already prepared packet remains a hard close-admission fence");
    assert!(
        admission.contains("component_surface_close_pending() && self.frame_build.has_live_session()") && admission.contains("retire_for_component_surface_close_step"),
        "the frame that created the bridge is retired one bounded unit before external ownership detaches the surface"
    );

    let frame_job = semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
    let transient = frame_job.split("pub fn retire_for_component_surface_close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{").nth(1).expect("the reusable frame handoff");
    let transient = &transient[..transient.find("fn retire_original_frame_step").expect("the shared original retirement ladder")];
    assert!(transient.contains("self.retire_original_frame_step(cx)?&&!self.has_live_session()"), "the same generation may be readmitted once the original ladder drained");
    assert!(!transient.contains("self.closing = true") && !transient.contains("completion_waker.take()") && !transient.contains("cancel_now()"), "transient retirement preserves reusable-host state");
}

/// 🐕️ LAW: the presentation watchdog sees WITHIN-item upload progress, so a healthy mesh upload can
/// never look like a frozen cursor and a frozen one is still named.
///
/// 🩸️ `AppPresentCursor::upload` only moves when a whole upload item completes, and `ensure_mesh_step`
/// writes ONE vertex per step — so a 293-vertex mesh froze the watchdog signature for 293 steps and
/// the host's gate census could only say `phase=Some(Uploads) … stall-steps=293` with no reason
/// attached (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w11a-prepared-world-mesh-missing.md`).
#[test]
fn the_present_watchdog_signature_carries_within_item_upload_progress() {
    assert!(DRAW_SOURCE().contains("pub fn upload_progress(&self) -> (u32, u32)"), "the mesh table exposes its own cursor's walk");
    assert!(GPU_SOURCE().contains("pub fn prepared_upload_progress(&self) -> (u32, u32, usize)"), "the GPU context joins it with the atlas page cursor");
    assert!(LIBRARY_SOURCE().contains("let upload_progress = self.gpu.prepared_upload_progress();"), "and the presenter reads it every step");
    assert!(LIBRARY_SOURCE().contains("cursor.gpu_cursor.as_ref().map(ui_gpu::PreparedGpuPresentCursor::progress)"), "the signature retains exact GPU cursor progress");
    let stall = LIBRARY_SOURCE().split("fn note_present_stall(").nth(1).expect("the presenter's stall watch");
    let stall = &stall[..stall.find("\n    }\n").expect("the stall watch closes")];
    let signature = &stall[stall.find("(cursor.phase").expect("the stall signature tuple") + 1..];
    let terms: Vec<&str> = signature.split(", ").map(|term| term.trim_end_matches([')', ',', '\n', ' '])).collect();
    assert!(["upload_progress", "cursor.raster_keep_steps", "cursor.input_progress"].iter().all(|term| terms.contains(term)), "upload, raster-ownership, and bounded input progress are independent signature terms: {terms:?}");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_decoder_close_hands_back_same_response_under_exact_independent_grant() {
    use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,RetainedCloneGrant,RetainedCloneProgress};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();let row=&law["decoderClose"];
    let(authority,probe)=owned_decoder_fixture();let token=probe.owner().owner().token();let observed=probe.owner().owner().received_bytes();
    let mut job=RendererAssetDecodeJob::new(probe);let mut caller=OriginalRendererFixture::new();job.begin_close();
    let before=caller.receipt();let denied=RetainedCloneGrant{maximum_items:0,..renderer_law_stock()};
    assert_eq!(caller.limited(denied,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Blocked);assert_eq!(caller.receipt(),before);
    let borrowed=job.probe.borrow_mut();assert!(job.take_probe().is_none());drop(borrowed);
    assert_eq!(caller.turn(|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Blocked);
    let(recovered,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.take_probe().unwrap());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(recovered.owner().owner().token(),token);assert_eq!(recovered.owner().owner().received_bytes(),observed);
    assert_eq!(row["maximumReleaseBytes"].as_u64().unwrap(),0);
    assert_eq!(caller.turn(|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()});assert!(job.terminal_is_empty());
    close_owned_decoder_fixture(authority,recovered);
}

#[test]
fn original_engine_packet_pages_require_actual_birth_and_release_grants() {
    use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️frame-close/🔣️.json")).unwrap();
    assert_eq!(fixture["enginePacketBacking"]["slots"], FRAME_ENGINE_PACKET_CAPACITY);
    let caller = &fixture["caller"];
    let grant = RetainedCloneGrant { maximum_items: caller["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: caller["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: caller["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: caller["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: caller["maximumDepth"].as_u64().unwrap() as usize };
    let (mut owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(FrameEnginePackets::default);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(owner.terminal_is_empty());
    assert_eq!(fixture["enginePacketBacking"]["pageSlots"], FRAME_ENGINE_PACKET_PAGE_SLOTS);
    let layout = std::alloc::Layout::array::<Option<semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::EngineCanvasPacket>>(FRAME_ENGINE_PACKET_PAGE_SLOTS).unwrap();
    assert!(layout.size() <= grant.maximum_capacity_bytes);
    let mut caller = semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(grant);
    for refused in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: layout.size() - 1, ..grant }, RetainedCloneGrant { maximum_copy_bytes: layout.size() - 1, ..grant }] {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(|cx|cx.with_retained_loan(refused, |cx|owner.prepare_step(cx))));
        assert_eq!(step.unwrap(), false);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(owner.terminal_is_empty());
    }
    for index in 0..FRAME_ENGINE_PACKET_PAGES {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(|cx|owner.prepare_step(cx)));
        assert_eq!(step.unwrap(), index + 1 == FRAME_ENGINE_PACKET_PAGES);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (layout.size(), 0));
    }
    assert!(owner.is_empty());
    assert!(!owner.terminal_is_empty());
    let pointer = owner.slots[FRAME_ENGINE_PACKET_PAGES - 1].as_ref().unwrap().as_ptr();
    let demand = owner.backing_retirement_demands();
    assert_eq!(demand.release_bytes, layout.size());
    for refused in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }] {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_backing_step(refused));
        assert!(matches!(step, Ok(InteractiveJobCloseStep::Pending { progress }) if progress == Default::default()));
        assert_eq!((heap.requested_bytes,heap.released_bytes), (0, 0));
        assert_eq!(owner.slots[FRAME_ENGINE_PACKET_PAGES - 1].as_ref().unwrap().as_ptr(), pointer);
    }
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_backing_step(RetainedCloneGrant { maximum_depth: 0, ..grant }));
    let error=step.unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert_eq!(error.retained_progress(),Default::default());
    assert_eq!((heap.requested_bytes,heap.released_bytes), (0, 0));
    assert_eq!(owner.slots[FRAME_ENGINE_PACKET_PAGES - 1].as_ref().unwrap().as_ptr(), pointer);
    let tiny = RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant };
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_backing_step(tiny));
    assert!(matches!(step, Ok(InteractiveJobCloseStep::Pending { progress }) if progress == Default::default()));
    assert_eq!((heap.requested_bytes,heap.released_bytes), (0, 0));
    assert_eq!(owner.slots[FRAME_ENGINE_PACKET_PAGES - 1].as_ref().unwrap().as_ptr(), pointer);
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_backing_step(grant));
    assert!(matches!(step, Ok(InteractiveJobCloseStep::Pending { progress }) if progress.fits(grant) && progress.released_bytes == layout.size() && progress.copied_bytes == demand.copy_bytes && progress.retained_capacity_bytes == 0));
    assert_eq!((heap.requested_bytes,heap.released_bytes), (0, layout.size()));
    for _ in 1..FRAME_ENGINE_PACKET_PAGES {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_backing_step(grant));
        assert!(step.unwrap().progress().fits(grant));
        assert_eq!((heap.requested_bytes,heap.released_bytes), (0, layout.size()));
    }
    assert!(owner.terminal_is_empty());
    caller.finish();
    assert!(matches!(owner.close_backing_step(RetainedCloneGrant::default()).unwrap(), InteractiveJobCloseStep::Complete { progress } if progress == Default::default()));
}

/// 📬️ Cancelling the unpolled dispatch future returns every original owner before any physical event release.
#[cfg(not(target_arch="wasm32"))]
#[test]
fn cancelled_dispatch_future_preserves_original_interaction_event_and_paid_source(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/📋️clipboard-ingress/🧫️fixtures/🔣️.json")).unwrap();assert_eq!(law["dispatchCustody"][1]["event"],1);
    let runtime=native_reference_runtime();let interaction=frame_maintenance_test_owner(511,0).interaction.take().unwrap();assert!(runtime.reserve_interaction_future());
    let mut text=String::with_capacity(law["capacity"].as_u64().unwrap()as usize);text.push_str(law["text"].as_str().unwrap());let pointer=text.as_ptr();let capacity=text.capacity();
    let mut events=ui_host::DrainedEvents::default();events.discrete[0]=Some(ui_host::DiscreteEvent{event:ui_render::DispatchEvent::Paste{text},generation:ui_host::InputGeneration(1)});
    let grant=ui_contract::UI_WORKER_RETIREMENT_POLICY;let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(grant);let original=caller.turn(super::RuntimeDispatchSource::admit).unwrap().unwrap();let mut cursor=RuntimeDispatchCursor::new(events,original);
    caller.turn(|cx|{cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap();cx.consume_fuel(1);cursor.inflight=cursor.take_next();});
    let future=RuntimeDispatchReturn{mailbox:runtime.clone(),revision:511,cursor:Some(cursor),interaction:Some(interaction)}.run();drop(future);
    let mut completion=runtime.0.completions.lock().unwrap().take_at(0).expect("cancellation returns exact reserved custody");let RuntimeApply::ResumeDispatch{interaction,cursor}=&mut completion.apply else{panic!("original dispatch return")};assert!(interaction.is_some());let mut cursor=cursor.take().expect("original cursor remains owned");
    let ui_render::DispatchEvent::Paste{text}=cursor.inflight.as_ref().unwrap()else{panic!("same original event")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);
    assert!(!caller.turn(|cx|cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_release_bytes:capacity-1,..grant},|cx|cursor.close_inflight_step(cx))).unwrap());let ui_render::DispatchEvent::Paste{text}=cursor.inflight.as_ref().unwrap()else{panic!("denied original event remains owned")};assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);
    assert!(!caller.turn(|cx|cursor.close_inflight_step(cx)).unwrap());assert!(cursor.inflight.is_none());while !caller.turn(|cx|cursor.close_step(cx)).unwrap(){}assert!(cursor.terminal_is_empty());
    runtime.try_lock().unwrap().return_interaction(interaction.take().unwrap());let mut complete=false;for _ in 0..65536{complete=caller.turn(|cx|runtime.close_input_step(cx)).unwrap();if complete{break}}assert!(complete,"original interaction input reaches paid terminal closure");caller.finish();
}
