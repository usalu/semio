use super::*;

const LIBRARY_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
const BINARY_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs");
const MANIFEST_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml");
const WINT_APP_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
const OS_HOST_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs");
const GPU_SOURCE: &str = include_str!("../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs");
const DRAW_SOURCE: &str = include_str!("../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs");
const PREPARED_SOURCE: &str = include_str!("../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs");
const ENGINE_CANVAS_SOURCE: &str = include_str!("../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");
const DRAW_LAWS_SOURCE: &str = include_str!("../../../../../../../🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs");

/// 🩺️ LAW: a frame build that ends because its PREPARATION refused must name the refusal. Nothing
/// else in the chain can: `AppFramePreparation` answers an empty `JobFault`, `ActiveFrameBuild`
/// answers it by cancelling its own token, and the job layer then reports a bare `Cancelled`. One
/// unnamed refusal per build is how the shell sat at one presented frame per boot with a silent
/// console for a whole packet (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn a_refused_frame_preparation_names_its_fault_before_the_build_cancels() {
    let frame_job = include_str!("../../🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
    let prepare_at = frame_job.find("ActiveFramePhase::Prepare(preparation) => {").expect("the prepare phase");
    let refused_at = prepare_at + frame_job[prepare_at..].find("StepOutcome::Cancelled | StepOutcome::Fault(_) =>").expect("the refused arm");
    let refused = &frame_job[refused_at..][..frame_job[refused_at..].find("ActiveFrameStep::Pending").expect("the refused arm ends on its own step")];
    assert!(refused.contains("preparation.fault()"), "the refused arm reads the preparation's own fault");

    for arm in [
        "self.fault = Some(\"prepared render job admission was refused\")",
        "self.fault = Some(\"prepared render job session admission was refused\")",
        "self.fault = Some(\"prepared render job lost its session\")",
        "self.fault = Some(\"prepared render job was cancelled\")",
    ] {
        assert!(LIBRARY_SOURCE.contains(arm), "every preparation refusal names itself: {arm}");
    }
    assert!(LIBRARY_SOURCE.contains("session.checked_out_job_mut().and_then(|job| job.fault())"), "and a refusal from inside the prepared job carries that job's own fault string");
}

#[test]
fn a_presented_input_seal_refusal_keeps_its_exact_fault_name() {
    assert!(LIBRARY_SOURCE.contains("Err(fault) => return FrameBuildBoundaryStep::Fault(fault)"));
    assert!(!LIBRARY_SOURCE.contains("Err(_) => return FrameBuildBoundaryStep::Fault(\"presented input candidate generation exhausted\")"));
}

#[test]
fn a_pre_submit_input_epoch_change_retires_and_reschedules_without_faulting_the_surface() {
    let render = LIBRARY_SOURCE.split("AppPresentPhase::Render => {").nth(1).expect("the render phase");
    let render = &render[..render.find("AppPresentPhase::CloseGpu =>").expect("the close-GPU phase")];
    assert!(render.contains("PresentedInputCandidateProgress::Stale"), "the pre-submit phase names an input epoch supersession");
    assert!(!render.contains("prepared frame input authority was stale before submit"), "normal pre-submit input supersession is not a surface fault");
    let stale_progress = render.split("PresentedInputCandidateProgress::Stale => {").nth(1).expect("the stale progress arm");
    let stale_progress = &stale_progress[..stale_progress.find("}\n                }").expect("the stale progress arm end")];
    let stale_match = render.split("if !shell.presented_input_candidate_matches(input_candidate) {").nth(1).expect("the final exact witness check");
    let stale_match = &stale_match[..stale_match.find("}\n                drop(runtime);").expect("the exact witness check end")];
    for stale in [stale_progress, stale_match] {
        assert!(stale.contains("cursor.frame.packet = self.gate.abort_pending();"), "the stale observation returns the unsubmitted packet");
        assert!(stale.contains("cursor.witness = None;"), "the stale observation retires the presenter witness");
        assert!(stale.contains("cursor.phase = AppPresentPhase::Aborted;"), "the stale observation enters bounded retirement");
        assert!(stale.contains("return Ok(AppPresentStep::Pending);"), "the stale observation keeps the presenter live without a surface fault");
    }

    let aborted = LIBRARY_SOURCE.split("AppPresentPhase::Aborted => {").nth(1).expect("the abort phase");
    let aborted = &aborted[..aborted.find("AppPresentPhase::Fullscreen =>").expect("the next presenter phase")];
    assert!(aborted.contains("discard_presented_input_candidate(input_candidate)"), "the next bounded abort turn returns the exact Shell/UI witness before rescheduling");
}

#[test]
fn product_library_has_no_executor_bridge() {
    assert!(!LIBRARY_SOURCE.contains(concat!("poll", "ster")));
    assert!(!LIBRARY_SOURCE.contains(concat!("block", "_on")));
    assert!(LIBRARY_SOURCE.contains("KernelPoolFuture::spawn"));
    assert!(LIBRARY_SOURCE.contains("spawn_app_task"));
    assert!(!LIBRARY_SOURCE.contains(concat!("TASK", "_POOL")));
    assert!(!LIBRARY_SOURCE.contains(concat!("poll", "_tasks")));
    assert!(!LIBRARY_SOURCE.contains("thread_local! {\n        static REAL_WAKER"));
    assert!(!WINT_APP_SOURCE.contains(concat!("poll", "_tasks")));
    assert!(!WINT_APP_SOURCE.contains(concat!("TASK", "_POOL")));
    assert!(!LIBRARY_SOURCE.contains("type RuntimeApply = Box"));
    assert!(!LIBRARY_SOURCE.contains(concat!("mem::", "forget")));
    assert!(!WINT_APP_SOURCE.contains("dispatch_drained_events"));
    assert!(LIBRARY_SOURCE.contains("struct RuntimeDispatchCursor"));
    assert!(LIBRARY_SOURCE.contains("ResumeDispatch"));
    assert!(!LIBRARY_SOURCE.contains("response.array_buffer()"));
    assert!(!LIBRARY_SOURCE.contains("collect_pending_ui_image_fetches"));
    assert!(!LIBRARY_SOURCE.contains("collect_pending_map_tile_fetches"));
    assert!(LIBRARY_SOURCE.contains("struct AppPresentCursor"));
    assert!(!LIBRARY_SOURCE.contains(".submit_prepared("));
    assert!(!GPU_SOURCE.contains("fn apply_prepared_uploads"));
    assert!(!DRAW_SOURCE.contains("for index in 0..schema.vertices"));
    assert!(!DRAW_SOURCE.contains("for index in 0..schema.indices"));
    assert!(DRAW_SOURCE.contains("pub fn ensure_mesh_step"));
    assert!(DRAW_SOURCE.contains("pub fn close_upload_step"));
    assert!(GPU_SOURCE.contains("pub fn close_mesh_upload_step"));
    let upload_close = OS_HOST_SOURCE.find("presenter.close_world_owners_step()").expect("active upload and packet close phase");
    let world_close = OS_HOST_SOURCE.find("runtime.close_world3d_dynamic_step()").expect("world mesh close phase");
    assert!(upload_close < world_close);
}

fn retained_raster_contract(draw: &str, gpu: &str, glue: &str, engine: &str) -> bool {
    fn guarded_allocations(source: &str, validation: &str, allocation: &str, expected: usize) -> bool {
        let validations = source.match_indices(validation).map(|(index, _)| index).collect::<Vec<_>>();
        if validations.len() != expected {
            return false;
        }
        let mut allocation_floor = 0;
        for validation_index in validations {
            let Some(relative_allocation) = source[validation_index + validation.len()..].find(allocation) else {
                return false;
            };
            let allocation_index = validation_index + validation.len() + relative_allocation;
            if allocation_index < allocation_floor {
                return false;
            }
            let gap = &source[validation_index + validation.len()..allocation_index];
            if ["device.create_texture", ".create_view", "device.create_bind_group", "create_target_texture", "Renderer::new"].iter().any(|marker| gap.contains(marker)) {
                return false;
            }
            allocation_floor = allocation_index + allocation.len();
        }
        true
    }

    let begin = gpu.find("self.raster_store.begin_presenting").unwrap_or(usize::MAX);
    let present_step = gpu.find("pub fn prepared_present_step").unwrap_or(0);
    let presenter_close = glue.find("self.gpu.close_raster_table_step()").unwrap_or(usize::MAX);
    let world_terminal = glue.find("self.gpu.raster_table_terminal_is_empty()").unwrap_or(0);
    let engine_reservation = "gpu.reserve_engine_texture(key, build.width, build.height, identity, candidate_generation, expected)";
    let engine_reservation_index = engine.find(engine_reservation).unwrap_or(usize::MAX);
    let first_engine_allocation = ["create_target_texture", ".create_view", "Renderer::new"].iter().filter_map(|marker| engine.find(marker)).min().unwrap_or(0);
    let upload_stage = &draw[draw.find("pub(crate) fn ensure_raster_step").unwrap_or(draw.len())..draw.find("pub fn get(&self, key: &str) -> Option<&RasterTexture>").unwrap_or(draw.len())];
    let gpu_stage = &draw[draw.find("pub fn stage_gpu_bind_group").unwrap_or(draw.len())..draw.find("pub fn begin_presenting").unwrap_or(draw.len())];
    let cancellation = &draw[draw.find("pub fn cancel_engine_texture_admission").unwrap_or(draw.len())..draw.find("fn claim_stage_before_gpu_allocation").unwrap_or(draw.len())];
    let upload_close = &draw[draw.find("pub fn close_upload_step(&mut self) -> RasterTextureCleanupStep").unwrap_or(draw.len())..];
    let upload_close = &upload_close[..upload_close.find("pub fn close_step(&mut self)").unwrap_or(upload_close.len())];
    draw.contains("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256")
        && draw.contains("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256")
        && draw.contains("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 64 * 1024 * 1024")
        && draw.contains("pub const RASTER_TEXTURE_TABLE_BYTE_CAPACITY: usize = 256 * 1024 * 1024")
        && draw.contains("const RASTER_TEXTURE_PROBE_CAPACITY: usize = 8")
        && draw.contains("Self::Scene(lease) => lease.transfer_bytes()")
        && upload_stage.contains("let transfer_bytes = pixels.transfer_bytes();")
        && upload_stage.contains("row_bytes > transfer_bytes")
        && upload_stage.contains("transfer_bytes / row_bytes")
        && draw.contains("struct FixedRasterTextureRegistry<T>")
        && draw.contains("pub struct RasterTextureAdmission")
        && draw.contains("candidate: RasterTextureWitnessSlot")
        && draw.contains("current.operation >= candidate.operation")
        && draw.contains("admission.witness != expected")
        && draw.contains("view: Option<wgpu::TextureView>")
        && draw.contains("scene_revision: Option<u64>")
        && draw.contains("preview_generation: Option<u64>")
        && draw.contains("operation: Option<u64>")
        && draw.contains("width: Option<u32>")
        && draw.contains("height: Option<u32>")
        && draw.contains("fn insert_vacant")
        && !draw.contains("fn insert_at")
        && draw.contains("RasterTextureRetirementMode::Commit")
        && draw.contains("RasterTextureRetirementMode::Abort")
        && draw.contains("pub fn commit_presented_step")
        && draw.contains("pub fn abort_presented_step")
        && draw.contains("pub fn terminal_is_empty(&self) -> bool")
        && draw.contains("struct RasterTextureUploadCloseCursor")
        && draw.contains("struct RasterTextureReservationCloseCursor")
        && draw.contains("enum RasterTextureCleanupStep")
        && cancellation.contains("self.reservation.take().expect(\"matching raster reservation\")")
        && cancellation.contains("RasterTextureReservationCloseCursor::cancelled(reservation, admission)")
        && !cancellation.contains(concat!("self.reservation =", " None"))
        && upload_close.contains("RasterTextureUploadCloseCursor::new(upload)")
        && !upload_close.contains("self.presenting.set")
        && !upload_close.contains(concat!("self.reservation =", " None"))
        && draw.contains("self.key == admission.key")
        && draw.contains("self.witness == admission.witness")
        && draw.contains("self.content_identity == admission.content_identity")
        && draw.contains("self.width == admission.width")
        && draw.contains("self.height == admission.height")
        && draw.contains("self.bytes == admission.bytes")
        && draw.contains("self.staged_index == admission.staged_index")
        && draw.contains("self.nonce == admission.nonce")
        && draw.contains("if admission.witness != expected")
        && draw.contains("if !reservation.matches(admission)")
        && draw.contains("if candidate != Some(expected)")
        && draw.contains("if staged_occupied")
        && draw.contains("staged_index: admission.staged_index, staged_nonce: admission.nonce")
        && draw.contains("fn claim_stage_before_gpu_allocation")
        && upload_stage.contains("self.upload = Some(RasterTextureUploadCursor")
        && guarded_allocations(upload_stage, "self.claim_texture_allocation(admission, expected)", "device.create_texture", 1)
        && guarded_allocations(upload_stage, "self.claim_view_allocation(admission, expected)", ".create_view", 1)
        && guarded_allocations(upload_stage, "self.claim_bind_group_allocation(admission, expected)", "device.create_bind_group", 1)
        && guarded_allocations(gpu_stage, "self.claim_bind_group_allocation(&admission, expected)", "device.create_bind_group", 1)
        && upload_stage.contains("if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, cpu_release)")
        && upload_stage.contains("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor")
        && upload_stage.contains("texture: Some(value.texture)")
        && upload_stage.contains("view: Some(value.view)")
        && upload_stage.contains("bind_group: Some(value.bind_group)")
        && upload_stage.contains("allocation_claim: Some(allocation_claim)")
        && !upload_stage.contains("map_err(|(fault, _, _)| fault)")
        && gpu_stage.matches("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor").count() == 1
        && gpu_stage.contains("RasterTextureStageFault::Returned { fault, admission, texture, view: raster_view }")
        && gpu_stage.contains("texture: Some(value.texture)")
        && gpu_stage.contains("view: Some(value.view)")
        && gpu_stage.contains("bind_group: Some(value.bind_group)")
        && gpu_stage.contains("allocation_claim: Some(allocation_claim)")
        && !gpu_stage.contains("map_err(|(fault, _, _)| fault)")
        && draw.contains("bind_group: source.bind_group.take()")
        && draw.contains("view: source.view.take()")
        && draw.contains("texture: source.texture.take()")
        && !draw.contains("HashMap<String, RasterTexture>")
        && gpu.contains("self.ensure_raster_texture_step(key, RasterUploadPixels::Contiguous(pixels)")
        && gpu.contains("self.ensure_raster_texture_step(key, RasterUploadPixels::Pages(pixels)")
        && gpu.contains("pub fn stage_engine_texture")
        && gpu.contains("view: wgpu::TextureView")
        && gpu.contains(".stage_gpu_bind_group")
        && gpu.contains("Result<(), RasterTextureStageFault>")
        && gpu.contains("pixels.content_identity()")
        && begin < present_step
        && engine.matches(engine_reservation).count() == 1
        && engine_reservation_index < first_engine_allocation
        && guarded_allocations(engine, "gpu.validate_engine_target_texture_allocation(admission, expected)", "create_target_texture", 1)
        && guarded_allocations(engine, "gpu.validate_engine_target_view_allocation(admission, expected)", ".create_view", 1)
        && guarded_allocations(engine, "gpu.validate_engine_renderer_allocation(admission, expected)", "Renderer::new", 1)
        && engine.contains("gpu.raster_content_is_reusable(key, identity, candidate_generation, expected)")
        && engine.matches("create_target_texture(gpu.device(), build.width, build.height)").count() == 1
        && engine.contains("Err(RasterTextureStageFault::Returned { fault, admission, texture, view }) => {")
        && engine.contains("build.admission = Some(admission);")
        && engine.contains("build.texture = Some(texture);")
        && engine.contains("build.view = Some(view);")
        && engine.contains("Err(RasterTextureStageFault::Retained(fault)) => {")
        && engine.contains("gpu.cancel_engine_texture_admission(admission)?;")
        && engine.contains("EngineGpuBuildPhase::RetireRenderer")
        && engine.contains("if build.renderer.take().is_some()")
        && !engine.contains("EngineGpuSurface")
        && !engine.contains("replacement_texture")
        && !engine.contains("replacement_view")
        && !engine.contains("surface.view.clone()")
        && !engine.contains("surface.texture.clone()")
        && glue.contains("struct RuntimeRasterOperationAuthority")
        && glue.contains("exhausted: AtomicBool")
        && glue.contains("compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)")
        && !glue.contains(concat!("next_operation.", "fetch_add(1, Ordering::AcqRel)"))
        && glue.contains("raster_operation_authority: RuntimeRasterOperationAuthority")
        && glue.contains("raster_operation_authority.begin(expected.scene_revision, expected.input_generation)")
        && glue.contains("raster_operation_authority.matches(raster_witness)")
        && glue.contains("RasterCandidateRetirement::Commit")
        && glue.contains("RasterCandidateRetirement::Abort")
        && presenter_close < world_terminal
}

/// 🌍️ The retained decoder mounts every GLB scene root under the world frame (`glb_world_frame`,
/// glTF's Y-up → the world's Z-up), which is React's `<group rotation={[π/2, 0, 0]}>` around
/// `GlbInstanceMesh`. The legacy oracle decodes raw glTF space, so it is compared through the
/// same rotation: `(x, y, z)` → `(x, -z, y)`.
#[test]
fn raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete() {
    assert!(retained_raster_contract(DRAW_SOURCE, GPU_SOURCE, LIBRARY_SOURCE, ENGINE_CANVAS_SOURCE));
    let mutations = [
            (DRAW_SOURCE.replace("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256", "pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 257"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256", "pub const RASTER_TEXTURE_KEY_BYTES: usize = 255"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 64 * 1024 * 1024", "pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = usize::MAX"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.replace("pub struct RasterTextureAdmission", "pub struct ErasedRasterTextureAdmission"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("current.operation >= candidate.operation", "current.operation > candidate.operation"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("view: Option<wgpu::TextureView>", "view_erased: bool"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.replace("view: wgpu::TextureView", "view: &wgpu::TextureView"), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("gpu.reserve_engine_texture", "gpu.realize_without_reservation")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.replace("raster_operation_authority: RuntimeRasterOperationAuthority", "raster_operation_authority_erased: bool"), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.replace(
                    "let operation = loop {",
                    concat!("let operation = self.0.next_operation.", "fetch_add(1, Ordering::AcqRel); if operation == 0 { return Err(\"raster operation generation exhausted\"); } loop {"),
                ),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace(
                    "let reservation = self.reservation.take().expect(\"matching raster reservation\");",
                    concat!("self.reservation =", " None; let reservation = RasterTextureReservation { key: admission.key, witness: admission.witness, width: admission.width, height: admission.height, bytes: admission.bytes, staged_index: admission.staged_index, nonce: admission.nonce };"),
                ),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.upload_close = Some(RasterTextureUploadCloseCursor::new(upload));", "self.upload = None;"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("fn claim_stage_before_gpu_allocation", "fn claim_stage_after_gpu_allocation"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.replace("raster_operation_authority.begin(expected.scene_revision, expected.input_generation)", "raster_operation_authority.begin(packet.scene_revision(), packet.preview_generation())"),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.replace("raster_operation_authority.matches(raster_witness)", "true"), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.replace("self.gpu.close_raster_table_step()", "true"), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("self.claim_texture_allocation(admission, expected)", "self.allocate_texture_without_full_claim(admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.claim_view_allocation(admission, expected)", "self.allocate_view_without_full_claim(admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.claim_bind_group_allocation(admission, expected)", "self.allocate_bind_group_without_full_claim(admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.claim_bind_group_allocation(&admission, expected)", "self.allocate_bind_group_without_full_claim(&admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_target_texture_allocation(admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_target_view_allocation(admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_renderer_allocation(admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.replace(
                    "if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, cpu_release)",
                    "if self.stage_claimed_texture(admission, value, allocation_claim, cpu_release).map_err(|(fault, _, _, _)| fault).is_err()",
                ),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor", "self.upload = None; Some(RasterTextureUploadCursor"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.replace("texture: Some(value.texture)", "texture: None"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("view: Some(value.view)", "view: None"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("bind_group: Some(value.bind_group)", "bind_group: None"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("if !reservation.matches(admission)", "if false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("if candidate != Some(expected)", "if false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("if staged_occupied", "if false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("staged_index: admission.staged_index, staged_nonce: admission.nonce", "staged_index: admission.staged_index, staged_nonce: 0"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("Returned {", "Erased {"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("build.texture = Some(texture);", "drop(texture);")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("build.view = Some(view);", "drop(view);")),
            (DRAW_SOURCE.replace("Self::Scene(lease) => lease.transfer_bytes()", "Self::Scene(_) => usize::MAX"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("let transfer_bytes = pixels.transfer_bytes();", "let transfer_bytes = usize::MAX;"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("row_bytes > transfer_bytes", "false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("transfer_bytes / row_bytes", "usize::MAX"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
        ];
    assert_eq!(mutations.len(), 39);
    for (draw, gpu, glue, engine) in mutations {
        assert!(!retained_raster_contract(&draw, &gpu, &glue, &engine));
    }
    let reservation = "build.admission = Some(gpu.reserve_engine_texture(key, build.width, build.height, identity, candidate_generation, expected)?);";
    let first_allocation = "build.texture = Some(create_target_texture(gpu.device(), build.width, build.height));";
    let reservation_after_first_allocation = ENGINE_CANVAS_SOURCE.replacen(reservation, "", 1).replacen(first_allocation, &format!("{first_allocation}\n            {reservation}"), 1);
    assert!(!retained_raster_contract(DRAW_SOURCE, GPU_SOURCE, LIBRARY_SOURCE, &reservation_after_first_allocation));
}

#[test]
fn renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length() {
    fn semantic_glb() -> Vec<u8> {
        let mut json = br#"{"scene":0,"scenes":[{"nodes":[0]}],"nodes":[{"mesh":0,"translation":[1,2,3]}],"buffers":[{"byteLength":100}],"accessors":[{"bufferView":0,"componentType":5126,"count":4,"type":"VEC3"},{"bufferView":1,"componentType":5120,"count":4,"type":"VEC3","normalized":true},{"bufferView":2,"componentType":5123,"count":4,"type":"VEC2","normalized":true},{"bufferView":3,"componentType":5121,"count":4,"type":"SCALAR"}],"bufferViews":[{"buffer":0,"byteOffset":0,"byteLength":64,"byteStride":16},{"buffer":0,"byteOffset":64,"byteLength":16,"byteStride":4},{"buffer":0,"byteOffset":80,"byteLength":16,"byteStride":4},{"buffer":0,"byteOffset":96,"byteLength":4}],"meshes":[{"primitives":[{"attributes":{"POSITION":0,"NORMAL":1,"TEXCOORD_0":2},"indices":3,"mode":5}]}]}"#.to_vec();
        while !json.len().is_multiple_of(4) {
            json.push(b' ');
        }
        let mut bin = Vec::with_capacity(100);
        for position in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
            for component in position {
                bin.extend_from_slice(&component.to_le_bytes());
            }
            bin.extend_from_slice(&[0; 4]);
        }
        for _ in 0..4 {
            bin.extend_from_slice(&[0, 0, 127, 0]);
        }
        for uv in [[0u16, 0], [u16::MAX, 0], [0, u16::MAX], [u16::MAX, u16::MAX]] {
            for component in uv {
                bin.extend_from_slice(&component.to_le_bytes());
            }
        }
        bin.extend_from_slice(&[0, 1, 2, 3]);
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
        glb
    }

    fn probe(glb: &[u8], split: usize) -> (WorldAssetIoAuthority, RendererAssetProbe) {
        let mut authority = WorldAssetIoAuthority::default();
        authority.reserve(1, 1, WorldAssetRequestKind::Glb, "probe.glb", glb.len()).unwrap();
        let mut owner = authority.take_next().unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(glb[..split].to_vec()).unwrap()).unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(glb[split..].to_vec()).unwrap()).unwrap();
        owner.seal().unwrap();
        authority.return_owner(owner).unwrap();
        let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).expect("completed probe owner");
        (authority, RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)))
    }

    let valid = semantic_glb();
    let (mut authority, mut cursor) = probe(&valid, 5);
    let mut ready = false;
    for _ in 0..4_096 {
        match cursor.step() {
            RendererAssetProbeStep::Pending => {}
            RendererAssetProbeStep::Ready => {
                ready = true;
                break;
            }
            RendererAssetProbeStep::Reject(detail) => panic!("valid retained GLB structure was refused: {detail}"),
            RendererAssetProbeStep::Fault(detail) => panic!("valid retained GLB structure faulted: {detail}"),
        }
    }
    assert!(ready);
    let materialize = cursor.glb_materialize.as_ref().expect("retained GLB materializer");
    let lease = materialize.lease.expect("generation-witnessed paged GLB mesh");
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut position_cursor = lease.cursor(Mesh3dField::Positions).unwrap();
    while let Some(Mesh3dItem::Vec3(value)) = position_cursor.read_next().unwrap() {
        positions.extend_from_slice(&value);
    }
    let mut normal_cursor = lease.cursor(Mesh3dField::Normals).unwrap();
    while let Some(Mesh3dItem::Vec3(value)) = normal_cursor.read_next().unwrap() {
        normals.extend_from_slice(&value);
    }
    let mut index_cursor = lease.cursor(Mesh3dField::Indices).unwrap();
    while let Some(Mesh3dItem::U32(value)) = index_cursor.read_next().unwrap() {
        indices.push(value);
    }
    let world_frame = |values: &[f32]| -> Vec<f32> { values.chunks_exact(3).flat_map(|axis| [axis[0], -axis[2], axis[1]]).collect() };
    let legacy = semio_framework::mesh_from_glb(&valid).expect("legacy glTF oracle");
    assert_eq!(positions, world_frame(&legacy.positions));
    assert_eq!(normals, world_frame(&legacy.normals));
    assert_eq!(indices, legacy.indices);
    cursor.begin_close();
    while !cursor.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = cursor.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();
    assert!(authority.terminal_is_empty());

    let malformed = b"glTF\x02\0\0\0\xff\0\0\0";
    let (mut authority, mut cursor) = probe(malformed, 7);
    assert!(matches!(cursor.step(), RendererAssetProbeStep::Pending));
    assert!(matches!(cursor.step(), RendererAssetProbeStep::Pending));
    assert!(matches!(cursor.step(), RendererAssetProbeStep::Reject("asset response format probe rejected malformed input")));
    while !cursor.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = cursor.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();
    assert!(authority.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
fn owned_decoder_fixture() -> (WorldAssetIoAuthority, RendererAssetProbe) {
    let bytes = include_bytes!("../../../../../../../🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::Glb, "catalogue-decode-owner.glb", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    for page in bytes.chunks(16 * 1024) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(page.to_vec()).unwrap()).unwrap();
    }
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).unwrap();
    (authority, RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)))
}

#[cfg(not(target_arch = "wasm32"))]
fn close_owned_decoder_fixture(mut authority: WorldAssetIoAuthority, mut probe: RendererAssetProbe) {
    probe.begin_close();
    for _ in 0..262_144 {
        if probe.close_step() {
            break;
        }
    }
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("the exact decoder response closes") else {
        panic!("fixture owner changed");
    };
    authority.finish(owner).unwrap();
    authority.begin_close();
    for _ in 0..4096 {
        if authority.close_step() {
            break;
        }
    }
    assert!(authority.terminal_is_empty());
}

/// 🛑️ One worker grant decodes one page, and cancellation returns that exact owner before disposal.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_preserves_its_exact_response_through_cancellation() {
    use semio_framework_job::{InteractiveJob, StepOutcome};
    fn assert_send<T: Send>() {}
    assert_send::<RendererAssetDecodeJob>();
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let (authority, probe) = owned_decoder_fixture();
    let token = probe.owner().owner().token();
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut job = RendererAssetDecodeJob::new(probe, cancelled.clone());
    let mut sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::OperationId(72), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(job.step(&mut context), StepOutcome::Yield));
    let observed = job.probe.borrow().as_ref().unwrap().observed_bytes;
    assert_eq!(observed, 16 * 1024);
    assert_eq!(context.fuel_remaining(), 0);
    cancelled.store(true, Ordering::Release);
    let mut outcome = job.step(&mut context);
    assert!(matches!(outcome, StepOutcome::Complete(_)));
    assert_eq!(job.result, Some(RendererAssetDecodeResult::Cancelled));
    assert_eq!(law["cancelledDecode"]["afterClose"], "cancelled");
    assert_eq!(job.steps, 1);
    let recovered = job.take_probe().expect("host takes the response from its exact terminal job");
    assert_eq!(recovered.owner().owner().token(), token);
    assert_eq!(recovered.observed_bytes, observed);
    job.begin_close();
    assert_eq!(job.close_step(1, 1), semio_framework_job::InteractiveJobCloseStep::Complete);
    assert!(job.terminal_is_empty());
    let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    assert!(outcome.terminal_is_empty());
    close_owned_decoder_fixture(authority, recovered);
    println!("[DEBUG] independent decoder cancelled after one page and returned token {token:?}");
}

/// 🎟️ Session saturation returns the unchanged decoder response while rejected metadata closes separately.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_recovers_its_response_from_an_exact_rejected_session() {
    use semio_framework_job::*;
    struct Occupied;
    impl InteractiveJob for Occupied {
        fn step(&mut self, _: &mut StepContext<'_>) -> StepOutcome {
            StepOutcome::Yield
        }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, _: usize, _: usize) -> InteractiveJobCloseStep {
            InteractiveJobCloseStep::Complete
        }
        fn terminal_is_empty(&self) -> bool {
            true
        }
    }
    fn params() -> BatchJobParams {
        BatchJobParams {
            operation: allocate_operation_id(),
            generation: Generation(1),
            cancel: root_cancel_token(),
            config: BatchDriveConfig { site: "decoder_admission_law", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: INTERACTIVE_LANE_WALL_US },
            now_us: default_now_us,
        }
    }
    let mut occupied = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for _ in 0..WORKER_JOB_SESSION_SLOTS {
        occupied.push(WorkerJobSession::try_new(Occupied, params()).unwrap_or_else(|_| panic!("fixed session admits its owner")));
    }
    let (authority, probe) = owned_decoder_fixture();
    let token = probe.owner().owner().token();
    let bytes = probe.owner().owner().received_bytes();
    let job = RendererAssetDecodeJob::new(probe, Arc::new(AtomicBool::new(false)));
    let mut rejected = match WorkerJobSession::try_new(job, params()) {
        Ok(_) => panic!("maximum plus one must retain rejection"),
        Err(rejected) => rejected,
    };
    let recovered = rejected.job().take_probe().expect("the sole rejected guard returns its exact response");
    assert_eq!(recovered.owner().owner().token(), token);
    assert_eq!(recovered.owner().owner().received_bytes(), bytes);
    assert_eq!(recovered.observed_bytes, 0);
    assert!(rejected.job().take_probe().is_none());
    rejected.begin_close();
    for _ in 0..64 {
        let _ = rejected.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        if rejected.terminal_is_empty() {
            break;
        }
    }
    assert!(rejected.terminal_is_empty());
    for session in occupied {
        let _ = session.begin_close();
        for _ in 0..64 {
            let _ = session.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                break;
            }
        }
        assert!(session.terminal_is_empty());
    }
    close_owned_decoder_fixture(authority, recovered);
    println!("[DEBUG] saturated decoder session returned the same {bytes}-byte response token {token:?}");
}

#[cfg(not(target_arch = "wasm32"))]
fn decoder_worker_boundary(cancel_after_ready: bool) {
    use std::time::{Duration, Instant};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let bytes = b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"></svg>";
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "ready-cancel.svg", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    owner.push_page(WorldAssetResponsePage::try_from_owned(bytes.to_vec()).unwrap()).unwrap();
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).unwrap();
    let token = owner.token();
    let wakes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed_wakes = wakes.clone();
    let wake: RuntimeHostWaker = Arc::new(move || {
        observed_wakes.fetch_add(1, Ordering::AcqRel);
    });
    let mut decoder = RendererAssetDecodeSession::new(RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)), Some(wake));
    let deadline = Instant::now() + Duration::from_secs(10);
    while decoder.boundary.is_none() && Instant::now() < deadline {
        decoder.pump_one();
        std::thread::yield_now();
    }
    let completed = decoder.boundary.as_ref().map(|(_, result)| *result);
    if cancel_after_ready {
        decoder.cancel();
    }
    while (decoder.session.is_some() || decoder.rejected.is_some()) && Instant::now() < deadline {
        decoder.pump_one();
        std::thread::yield_now();
    }
    let (probe, outcome) = decoder.take_boundary().expect("the exact completed worker returns its response");
    let observed_token = probe.owner().owner().token();
    let observed_bytes = probe.observed_bytes;
    assert!(decoder.terminal_is_empty());
    close_owned_decoder_fixture(authority, probe);
    println!("[DEBUG] real decoder returned {outcome:?} with ready cancellation {cancel_after_ready}, token {observed_token:?}, bytes {observed_bytes}, wakes {}", wakes.load(Ordering::Acquire));
    assert_eq!(completed, Some(Some(RendererAssetDecodeResult::Ready)));
    assert_eq!(observed_token, token);
    assert_eq!(observed_bytes, bytes.len());
    assert!(wakes.load(Ordering::Acquire) > 0);
    assert_eq!(law["cancelledDecode"]["afterClose"], "cancelled");
    assert_eq!(outcome, Some(if cancel_after_ready { RendererAssetDecodeResult::Cancelled } else { RendererAssetDecodeResult::Ready }));
}

/// 🛑️ A real worker completion remains cancellable until its response crosses the host boundary.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_cancels_a_ready_response_before_host_handback() {
    decoder_worker_boundary(true);
}

/// 📬️ Session metadata retirement preserves a successful worker result when no cancellation occurred.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_independent_decoder_job_returns_a_ready_response_after_session_retirement() {
    decoder_worker_boundary(false);
}

/// 📄️ Native transport transfers a complete credited response page into its exact request.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_asset_response_transfers_each_credited_page_without_losing_its_bytes() {
    use semio_framework_job::{JobPayloadStream, RetainedJobPayloadWriter};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    assert_eq!(law["pageBytes"].as_u64().unwrap() as usize, WORLD_ASSET_RESPONSE_PAGE_BYTES);
    for case in law["cases"].as_array().unwrap() {
        let length = case["bytes"].as_u64().unwrap() as usize;
        let bytes: Vec<u8> = (0..length).map(|index| (index % law["patternPeriod"].as_u64().unwrap() as usize) as u8).collect();
        let mut authority = WorldAssetIoAuthority::default();
        authority.reserve(1, 1, WorldAssetRequestKind::Glb, "native-page.glb", length).unwrap();
        let mut fetch = RendererAssetFetchOwner::Shared(authority.take_next().unwrap());
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput);
        let mut sequence = 0;
        for source in bytes.chunks(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            let mut context =
                semio_framework_job::StepContext::new(semio_framework_job::OperationId(74), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            let mut page = writer.admit_page(&mut context).unwrap();
            page.write(source).unwrap();
            page.commit();
        }
        let mut payload = writer.finish().unwrap_or_else(|_| panic!("the fixture owns complete native pages"));
        let result = push_renderer_asset_page(&mut fetch, &mut payload);
        let source_released = payload.terminal_is_empty();
        let retained_source = payload.len();
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        let observed = fetch.owner().received_bytes();
        let decoded = if result.is_ok() {
            let RendererAssetFetchOwner::Shared(owner) = &mut fetch else { unreachable!() };
            owner.seal().unwrap();
            owner.decode_page().unwrap().unwrap().bytes().to_vec()
        } else {
            Vec::new()
        };
        fetch.begin_close();
        let RendererAssetFetchOwner::Shared(owner) = fetch else { unreachable!() };
        assert!(authority.return_owner(owner).is_ok());
        authority.begin_close();
        for _ in 0..4096 {
            if authority.close_step() {
                break;
            }
        }
        assert!(authority.terminal_is_empty());
        println!("[DEBUG] native response transferred {observed}/{length} bytes with result {result:?}");
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap());
        assert_eq!(observed as u64, case["transferredBytes"].as_u64().unwrap());
        if result.is_ok() {
            assert_eq!(decoded, bytes);
        }
        assert_eq!(source_released, case["accepted"].as_bool().unwrap());
        assert_eq!(retained_source, if result.is_ok() { 0 } else { length });
    }
}

/// 🧵️ A retained catalogue response belongs to the decoder even while a frame candidate advances.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_frame_candidate_never_advances_the_independently_owned_asset_decoder() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json")).unwrap();
    let bytes = include_bytes!("../../../../../../../🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let mut authority = WorldAssetIoAuthority::default();
    authority.reserve(1, 1, WorldAssetRequestKind::Glb, "catalogue-frame-isolation.glb", bytes.len()).unwrap();
    let mut owner = authority.take_next().unwrap();
    for page in bytes.chunks(16 * 1024) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(page.to_vec()).unwrap()).unwrap();
    }
    owner.seal().unwrap();
    authority.return_owner(owner).unwrap();
    let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).unwrap();
    let runtime = RuntimeMailbox::new(AppRuntime {
        atlas: FontAtlas::builtin(),
        icons: IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: None,
        checkout: Default::default(),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_plugin_mtimes: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_modified: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    });
    *runtime.0.asset_probe.lock().unwrap() = Some(RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)));
    let operation = semio_framework_job::OperationId(71);
    let generation = semio_framework_job::Generation(0);
    let mut candidate = FrameTransaction::new(Default::default(), operation, generation);
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    let result = candidate.step(&runtime, &Arc::downgrade(&runtime.0), &mut context);
    let mut probe = runtime.0.asset_probe.lock().unwrap().take().expect("the decoder retains its exact response");
    let observed = probe.observed_bytes;
    for _ in 0..4096 {
        if candidate.close_step() {
            break;
        }
    }
    assert!(candidate.terminal_is_empty());
    probe.begin_close();
    for _ in 0..262_144 {
        if probe.close_step() {
            break;
        }
    }
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("the response retires before observation assertions") else {
        panic!("fixture response owner changed");
    };
    authority.finish(owner).unwrap();
    authority.begin_close();
    for _ in 0..4096 {
        if authority.close_step() {
            break;
        }
    }
    assert!(authority.terminal_is_empty());
    for _ in 0..4096 {
        if runtime.close_renderer_asset_step() {
            break;
        }
    }
    println!("[DEBUG] frame candidate decoded {observed} asset bytes before returning without a mounted interaction");
    assert!(matches!(result, AppFrameTransactionStep::Superseded));
    assert_eq!(observed as u64, law["expected"]["frameDecodeUnits"].as_u64().unwrap());
}

/// 🖼️ The reference underlay the puzzle3d playground ships
/// (`/infinite-assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`, 2275×2560 = 23 296 000
/// straight-RGBA bytes) must DECODE — React's `WorldReferenceLayer` paints it — and a genuine pixel
/// bomb must be REFUSED as one missing asset, never as a frame fault that quarantines the surface.
#[test]
fn retained_image_decoder_admits_a_reference_plan_and_rejects_a_pixel_bomb_without_faulting() {
    fn jpeg(width: u16, height: u16) -> Vec<u8> {
        let [height_high, height_low] = height.to_be_bytes();
        let [width_high, width_low] = width.to_be_bytes();
        vec![0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, height_high, height_low, width_high, width_low, 1, 1, 0x11, 0, 0xff, 0xd9]
    }
    fn png(width: u32, height: u32) -> Vec<u8> {
        [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &width.to_be_bytes(), &height.to_be_bytes(), &[8, 6, 0, 0, 0], &[0; 4], &[0, 0, 0, 1], b"IDAT", &[0], &[0; 4], &[0, 0, 0, 0], b"IEND", &[0; 4]].concat()
    }
    fn feed(bytes: &[u8], cursor: &mut RendererAssetFormatCursor) -> Result<(), &'static str> {
        for block in bytes.chunks(RENDERER_ASSET_PARSE_BLOCK_BYTES) {
            cursor.feed(block)?;
        }
        cursor.finish()
    }
    fn probe_image(bytes: &[u8]) -> (WorldAssetIoAuthority, RendererAssetProbe) {
        let mut authority = WorldAssetIoAuthority::default();
        authority.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "reference.jpg", bytes.len()).unwrap();
        let mut owner = authority.take_next().unwrap();
        owner.push_page(WorldAssetResponsePage::try_from_owned(bytes.to_vec()).unwrap()).unwrap();
        owner.seal().unwrap();
        authority.return_owner(owner).unwrap();
        let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).expect("completed probe owner");
        (authority, RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)))
    }
    fn drive(bytes: &[u8]) -> (WorldAssetIoAuthority, RendererAssetProbe, RendererAssetProbeStep) {
        let (authority, mut probe) = probe_image(bytes);
        for _ in 0..4_096 {
            match probe.step() {
                RendererAssetProbeStep::Pending => {}
                step => return (authority, probe, step),
            }
        }
        panic!("bounded image probe never reached a terminal step");
    }

    let plan = jpeg(2275, 2560);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &plan, plan.len()).expect("JPEG format cursor");
    feed(&plan, &mut cursor).expect("a 2275x2560 reference plan is a legal image, not a bomb");
    let plan = png(2275, 2560);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &plan, plan.len()).expect("PNG format cursor");
    feed(&plan, &mut cursor).expect("the same dimensions decode through the PNG scanner");

    assert_eq!(RENDERER_ASSET_PIXEL_BYTES, 64 * 1024 * 1024, "the ceiling is 16 megapixels of straight RGBA");
    let bomb = jpeg(4097, 4097);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &bomb, bomb.len()).expect("JPEG format cursor");
    assert_eq!(feed(&bomb, &mut cursor), Err("JPEG dimensions exceeded fixed pixel credits"));
    let bomb = png(4097, 4097);
    let mut cursor = RendererAssetFormatCursor::new(WorldAssetRequestKind::ReferenceImage, &bomb, bomb.len()).expect("PNG format cursor");
    assert_eq!(feed(&bomb, &mut cursor), Err("PNG dimensions exceeded fixed pixel credits"));

    let (mut authority, mut probe, step) = drive(&jpeg(2275, 2560));
    assert!(matches!(step, RendererAssetProbeStep::Ready), "the playground's own reference plan reaches Ready");
    probe.begin_close();
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();

    let (mut authority, mut probe, step) = drive(&jpeg(4097, 4097));
    assert!(matches!(step, RendererAssetProbeStep::Reject("JPEG dimensions exceeded fixed pixel credits")), "a refused image is ONE missing asset — a Fault here quarantines the whole surface");
    let (detail, kind, url) = probe.take_rejection().expect("a rejection carries the lane its miss belongs to");
    assert_eq!(detail, "JPEG dimensions exceeded fixed pixel credits");
    assert_eq!(kind, WorldAssetRequestKind::ReferenceImage);
    assert_eq!(url, "reference.jpg", "the url is captured BEFORE begin_close clears it");
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();
    assert!(authority.terminal_is_empty());

    for source in [br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#.as_slice(), b"GIF89a\x01\0\x01\0".as_slice()] {
        let (mut authority, mut probe, step) = drive(source);
        assert!(matches!(step, RendererAssetProbeStep::Ready), "React-accepted reference media reaches the target decoder rather than being narrowed at the shared probe");
        probe.begin_close();
        while !probe.close_step() {}
        let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
        authority.finish(owner).unwrap();
    }
}

#[test]
fn reference_decode_staging_refuses_a_fifth_live_token_without_evicting_the_first() {
    let mut io = WorldAssetIoAuthority::default();
    let tokens = (0..5).map(|index| io.reserve(1, index + 1, WorldAssetRequestKind::ReferenceImage, &format!("reference-{index}.png"), 4).expect("bounded reference token")).collect::<Vec<_>>();
    let mut staged = StagedReferenceImageAuthority::new();
    let pool = SceneRasterPool::new();
    for (index, token) in tokens.iter().copied().take(4).enumerate() {
        let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [index as u64 + 1, index as u64 + 101], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        assert_eq!(staged.begin(&pool, token, format!("reference-{index}.png"), descriptor, index as u64 + 1).expect("begin"), StagedReferenceImageAuthority::WRITER);
        assert!(staged.push(&pool, token, 0, &[index as u8, 0, 0, 255]).expect("exact row"));
        assert!(staged.seal(&pool, token).expect("seal"));
    }
    let fifth = SceneRasterDescriptor { width: 1, height: 1, source_digest: [5, 105], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    assert_eq!(staged.begin(&pool, tokens[4], "reference-4.png".into(), fifth, 5).expect("bounded staging refusal"), StagedReferenceImageAuthority::BUSY, "the fifth live token waits instead of evicting");
    let first = staged.take(tokens[0], "reference-0.png").expect("first token remains staged");
    first.with_rows(0, 4, |bytes, rows| assert_eq!((bytes, rows), (&[0, 0, 0, 255][..], 1))).expect("immutable first row");
    assert!(first.release());
    for token in tokens.iter().copied().skip(1).take(3) {
        staged.discard(&pool, token);
    }
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty(), "the token fixture returns every request owner after proving staged residency");
}

#[test]
fn reference_decode_staging_grants_one_writer_and_reuses_only_after_seal() {
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "shared.png", 4).expect("source token");
    let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [7, 11], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    let pool = SceneRasterPool::new();
    let mut staged = StagedReferenceImageAuthority::new();

    assert_eq!(staged.begin(&pool, token, "shared.png".into(), descriptor, 1).expect("first begin"), StagedReferenceImageAuthority::WRITER);
    assert_eq!(staged.begin(&pool, token, "shared.png".into(), descriptor, 1).expect("reentrant begin"), StagedReferenceImageAuthority::BUSY);
    assert!(staged.push(&pool, token, 0, &[7, 11, 13, 255]).expect("only writer pixels"));
    assert!(staged.seal(&pool, token).expect("exact seal"));
    assert_eq!(staged.begin(&pool, token, "shared.png".into(), descriptor, 1).expect("sealed begin"), StagedReferenceImageAuthority::REUSED);

    let lease = staged.take(token, "shared.png").expect("sealed lease remains available");
    lease.with_rows(0, 4, |bytes, rows| assert_eq!((bytes, rows), (&[7, 11, 13, 255][..], 1))).expect("immutable source");
    assert!(lease.release());
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty());
}

#[test]
fn reference_decode_staging_reports_exact_pool_reuse_before_browser_decode() {
    let mut io = WorldAssetIoAuthority::default();
    let first_token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "shared.png", 4).expect("first source token");
    let second_token = io.reserve(2, 1, WorldAssetRequestKind::ReferenceImage, "shared.png", 4).expect("second source token");
    let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [7, 11], source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    let pool = SceneRasterPool::new();
    let mut staged = StagedReferenceImageAuthority::new();
    assert_eq!(staged.begin(&pool, first_token, "shared.png".into(), descriptor, 1).expect("first begin"), StagedReferenceImageAuthority::WRITER);
    assert!(staged.push(&pool, first_token, 0, &[7, 11, 13, 255]).expect("first pixels"));
    assert!(staged.seal(&pool, first_token).expect("first seal"));
    let first = staged.take(first_token, "shared.png").expect("first ready lease");
    assert!(first.release());

    assert_eq!(staged.begin(&pool, second_token, "shared.png".into(), descriptor, 2).expect("second begin"), StagedReferenceImageAuthority::REUSED);
    assert!(staged.seal(&pool, second_token).expect("reused seal is terminal without another pixel stream"));
    let second = staged.take(second_token, "shared.png").expect("reused ready lease");
    second.with_rows(0, 4, |bytes, rows| assert_eq!((bytes, rows), (&[7, 11, 13, 255][..], 1))).expect("reused immutable source");
    assert!(second.release());

    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
fn native_reference_runtime() -> RuntimeMailbox {
    RuntimeMailbox::new(AppRuntime {
        atlas: FontAtlas::builtin(),
        icons: IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: None,
        checkout: Default::default(),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_plugin_mtimes: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_modified: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    })
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_reference_decode_cancellation_keeps_encoded_and_decoded_owners_until_bounded_close() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    let retirement = &law["referenceDecodeRetirement"];
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "reference.png", 4).expect("reference token");
    let input_bytes = retirement["phaseZero"]["encodedBytes"].as_u64().unwrap() as usize;
    let pixel_bytes = retirement["phaseZero"]["decodedPixelBytes"].as_u64().unwrap() as usize;
    let job = NativeReferenceDecodeJob::new(token, "reference.png".into(), vec![7; input_bytes]);
    job.rearm(DecodedReferenceImage { width: u32::try_from(pixel_bytes / 4).unwrap(), height: 1, source_digest: [71, 73], pixels: vec![11; pixel_bytes] });
    job.cancel();
    let retained_input = job.input.lock().expect("decode input").as_ref().map(Vec::len);
    let retained_pixels = job.decoded.lock().expect("decoded reference image").as_ref().map(|decoded| decoded.pixels.len());
    let cancelled = job.cancelled.load(Ordering::Acquire);
    assert_eq!(job.close_step(1, 0), NativeReferenceDecodeCloseStep::Pending { released_items: 0, released_bytes: retirement["zeroGrantReleasedBytes"].as_u64().unwrap() as usize });
    let mut released = Vec::new();
    let mut terminal = false;
    for _ in 0..32 {
        match job.close_step(retirement["maximumItems"].as_u64().unwrap() as usize, law["pageBytes"].as_u64().unwrap() as usize) {
            NativeReferenceDecodeCloseStep::Pending { released_bytes, .. } if released_bytes > 0 => released.push(released_bytes),
            NativeReferenceDecodeCloseStep::Pending { .. } | NativeReferenceDecodeCloseStep::Busy => {}
            NativeReferenceDecodeCloseStep::Complete => {
                terminal = true;
                break;
            }
        }
    }
    assert!(LIBRARY_SOURCE.contains("renderer_worker_pool().try_submit(semio_framework_async::Lane::Maintenance, job)"), "native reference decode submits only to the bounded maintenance lane");
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty(), "the token fixture returns its request owner after proving job cancellation");
    assert!(cancelled, "cancellation must fence worker publication before retirement starts");
    assert_eq!(retained_input, Some(input_bytes), "cancellation cannot destroy the encoded owner before an explicit bounded close grant");
    assert_eq!(retained_pixels, Some(pixel_bytes), "cancellation cannot destroy the decoded pixel owner before an explicit bounded close grant");
    let expected = retirement["phaseZero"]["decodedPageReleases"].as_array().unwrap().iter().chain(retirement["phaseZero"]["encodedPageReleases"].as_array().unwrap()).map(|value| value.as_u64().unwrap() as usize).collect::<Vec<_>>();
    assert_eq!(released, expected, "every close turn releases at most the neutral one-page grant, decoded before encoded");
    assert_eq!(released.iter().sum::<usize>(), retirement["phaseZero"]["releasedBytes"].as_u64().unwrap() as usize);
    assert!(terminal);
    assert!(job.input.lock().expect("terminal decode input").is_none());
    assert!(job.decoded.lock().expect("terminal decoded reference image").is_none());
    assert!(job.output.lock().expect("terminal decode output").is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_reference_decode_phase_two_waiting_output_stays_in_the_mailbox_after_one_close_turn() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    let retirement = &law["referenceDecodeRetirement"]["phaseTwoWaiting"];
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "waiting-reference.png", 4).expect("reference token");
    let job = Arc::new(NativeReferenceDecodeJob::new(token, "waiting-reference.png".into(), vec![13; retirement["encodedBytes"].as_u64().unwrap() as usize]));
    let pixel_bytes = retirement["decodedPixelBytes"].as_u64().unwrap() as usize;
    *job.output.lock().expect("decode output") = Some(NativeReferenceDecodeOutput::Waiting(DecodedReferenceImage { width: u32::try_from(pixel_bytes / 4).unwrap(), height: 1, source_digest: [79, 83], pixels: vec![17; pixel_bytes] }));
    job.phase.store(2, Ordering::Release);
    let runtime = native_reference_runtime();
    *runtime.0.native_reference_decode.lock().expect("native reference decode slot") = Some(job);
    let _ = runtime.close_renderer_asset_step();
    let retained_after_one_turn = runtime.0.native_reference_decode.lock().expect("native reference decode slot after one close turn").is_some();
    let mut close_turns = 1;
    while runtime.0.native_reference_decode.lock().expect("native reference decode terminal probe").is_some() && close_turns < 32 {
        let _ = runtime.close_renderer_asset_step();
        close_turns += 1;
    }
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty(), "the token fixture returns its request owner after observing mailbox retention");
    assert_eq!(retained_after_one_turn, retirement["retainedAfterOneTurn"].as_bool().unwrap(), "one close turn cannot clear a phase-two waiting decode whose pixels exceed one page");
    assert_eq!(close_turns, retirement["closeTurns"].as_u64().unwrap() as usize, "the mailbox retains the exact owner through output handback, decoded pages, metadata, encoded pages, metadata, and terminal acknowledgement");
    assert!(runtime.0.native_reference_decode.lock().expect("native reference decode terminal").is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_reference_decode_running_worker_returns_its_encoded_and_decoded_owners_before_bounded_close() {
    use std::time::{Duration, Instant};

    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️native-asset-response/🔣️.json")).unwrap();
    let retirement = &law["referenceDecodeRetirement"]["phaseZero"];
    let input_bytes = retirement["encodedBytes"].as_u64().unwrap() as usize;
    let pixel_bytes = retirement["decodedPixelBytes"].as_u64().unwrap() as usize;
    let mut io = WorldAssetIoAuthority::default();
    let token = io.reserve(1, 1, WorldAssetRequestKind::ReferenceImage, "worker-reference.png", 4).expect("reference token");
    let job = Arc::new(NativeReferenceDecodeJob::new(token, "worker-reference.png".into(), vec![19; input_bytes]));
    job.rearm(DecodedReferenceImage { width: u32::try_from(pixel_bytes / 4).unwrap(), height: 1, source_digest: [89, 97], pixels: vec![23; pixel_bytes] });
    let entered = Arc::new(std::sync::Barrier::new(2));
    let release = Arc::new(std::sync::Barrier::new(2));
    *job.worker_barriers.lock().expect("native reference worker barrier") = Some((entered.clone(), release.clone()));
    assert!(job.try_schedule(native_reference_runtime()), "the real maintenance worker accepts the retained decode owner");
    entered.wait();
    job.cancel();
    assert_eq!(job.close_step(1, law["pageBytes"].as_u64().unwrap() as usize), NativeReferenceDecodeCloseStep::Busy, "a checked-out worker owner cannot be stolen by close");
    release.wait();
    let deadline = Instant::now() + Duration::from_secs(10);
    while job.phase.load(Ordering::Acquire) == 1 && Instant::now() < deadline {
        std::thread::yield_now();
    }
    let retained_input = job.input.lock().expect("worker-returned input").as_ref().map(Vec::len);
    let retained_pixels = job.decoded.lock().expect("worker-returned decoded image").as_ref().map(|decoded| decoded.pixels.len());
    for _ in 0..32 {
        if matches!(job.close_step(1, law["pageBytes"].as_u64().unwrap() as usize), NativeReferenceDecodeCloseStep::Complete) {
            break;
        }
    }
    io.begin_close();
    while !io.close_step() {}
    assert!(io.terminal_is_empty());
    assert_eq!(retained_input, Some(input_bytes), "the worker must hand the encoded owner back before bounded close");
    assert_eq!(retained_pixels, Some(pixel_bytes), "the worker must hand the decoded pixel owner back before bounded close");
}

#[test]
fn retained_asset_structure_scanners_resume_at_arbitrary_block_boundaries() {
    let png = [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0], &[0; 4], &[0, 0, 0, 1], b"IDAT", &[0], &[0; 4], &[0, 0, 0, 0], b"IEND", &[0; 4]].concat();
    let mut png_cursor = PngStructureCursor::new(png.len());
    for bytes in png.chunks(3) {
        png_cursor.feed(bytes).expect("bounded PNG block");
    }
    png_cursor.finish().expect("PNG terminal");

    let jpeg = [0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, 0, 1, 0, 1, 1, 1, 0x11, 0, 0xff, 0xd9];
    let mut jpeg_cursor = JpegStructureCursor::new(jpeg.len());
    for bytes in jpeg.chunks(2) {
        jpeg_cursor.feed(bytes).expect("bounded JPEG block");
    }
    jpeg_cursor.finish().expect("JPEG terminal");

    let protobuf = [0x1a, 0x02, 0x08, 0x01];
    let mut protobuf_cursor = ProtobufStructureCursor::new(protobuf.len());
    for byte in protobuf {
        protobuf_cursor.feed(&[byte]).expect("bounded protobuf byte");
    }
    protobuf_cursor.finish().expect("protobuf terminal");
}

#[test]
fn retained_asset_structure_scanners_reject_pixel_and_varint_capacity_plus_one() {
    let png = [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &[0, 0, 0x10, 1, 0, 0, 0x10, 0, 8, 6, 0, 0, 0]].concat();
    let mut png_cursor = PngStructureCursor::new(png.len() + 4);
    assert!(png_cursor.feed(&png).is_err());

    let mut protobuf_cursor = ProtobufStructureCursor::new(11);
    assert!(protobuf_cursor.feed(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02]).is_err());
}

#[test]
fn runtime_mailbox_reserves_completion_capacity_and_coalesces_only_matching_keys() {
    let completion = |key: Option<&'static str>, revision: u64| RuntimeCompletion { key, revision, requires_interaction: false, restores_interaction: false, apply: RuntimeApply::Resize { width: 1.0, height: 1.0, dpr: 1.0 } };
    let mut queue = RuntimeCompletionQueue::new();
    for revision in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
        assert!(queue.enqueue(completion(None, revision as u64)));
    }
    assert!(!queue.enqueue(completion(None, 10_000)));
    assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY - 1);

    let mut queue = RuntimeCompletionQueue::new();
    for revision in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
        assert!(queue.enqueue(completion(Some("refresh"), revision as u64)));
    }
    assert!(queue.enqueue(completion(Some("refresh"), 10_000)));
    assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY - 1);
    assert_eq!(queue.ready.back().expect("latest refresh").revision, 10_000);

    let mut queue = RuntimeCompletionQueue::new();
    for _ in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
        assert!(queue.reserve(None));
    }
    assert!(!queue.reserve(None));
    assert!(queue.reserve_interaction());
    assert!(!queue.reserve_interaction());
    queue.finish(completion(None, 20_000));
    assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn presenter_restores_only_the_checked_out_interaction_and_retains_its_deferred_cursor() {
    let runtime = native_reference_runtime();
    let interaction = frame_maintenance_test_owner(121, 0).interaction.take().expect("interaction fixture");
    {
        let mut owner = runtime.try_lock().expect("runtime owner");
        owner.interaction = Some(interaction);
        let checked_out = owner.check_out_interaction("presenter-return-law").expect("exact checkout owner");
        let cursor = FrameDeferredCursor::new(FrameActionOwners::default(), false, false, false, false, 121, semio_framework_job::root_cancel_token());
        let mut queue = runtime.0.completions.lock().expect("runtime completion mailbox lock");
        assert!(queue.reserve_interaction());
        queue.finish(returned_completion(121, RuntimeApply::ResumeFrameDeferred { interaction: Some(checked_out), cursor: Some(cursor) }));
    }

    assert_eq!(runtime.restore_presenter_interaction_step(), PresenterInteractionStep::Restored);
    assert!(runtime.try_lock().expect("restored runtime").interaction_available(), "the presenter-only step returns the exact checked-out state");
    let mut completion = runtime.0.completions.lock().expect("runtime completion mailbox lock").take_at(0).expect("deferred cursor remains queued");
    assert!(!completion.restores_interaction);
    let RuntimeApply::ResumeFrameDeferred { interaction, cursor } = &mut completion.apply else { panic!("exact deferred continuation") };
    assert!(interaction.is_none(), "the returned state cannot be applied twice");
    let mut cursor = cursor.take().expect("the deferred cursor remains owned for post-presentation apply");
    cursor.begin_close();
    while !cursor.close_step() {}
    assert!(cursor.terminal_is_empty());
}

#[test]
fn native_binary_owns_exactly_one_entrypoint_driver() {
    assert_eq!(BINARY_SOURCE.matches(concat!("block", "_on(")).count(), 1);
    assert_eq!(BINARY_SOURCE.matches("drive_entrypoint(").count(), 3);
}

#[test]
fn manifest_has_no_retired_direct_edges() {
    assert!(!MANIFEST_SOURCE.contains(concat!("poll", "ster =")));
    assert!(!MANIFEST_SOURCE.contains(concat!("wasm-bindgen", "-test")));
    assert!(!MANIFEST_SOURCE.lines().any(|line| line.trim_start().starts_with("naga =")));
    assert!(!MANIFEST_SOURCE.lines().any(|line| line.trim_start().starts_with("rfd =")));
    assert!(!MANIFEST_SOURCE.lines().any(|line| line.trim_start().starts_with("ureq =")));
}

#[test]
fn runtime_dispatch_cursor_merges_retained_pointer_by_input_generation() {
    let pointer = ui_host::PointerMoveSample {
        pointer: ui_render::PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None },
        x: 1.0,
        y: 2.0,
        modifiers: ui_render::EventModifiers { shift: true, ..Default::default() },
        generation: ui_host::InputGeneration(2),
    };
    let scroll = ui_host::DiscreteEvent { event: ui_render::DispatchEvent::Scroll { x: 3.0, y: 4.0, delta_x: 5.0, delta_y: 6.0, modifiers: ui_render::EventModifiers { ctrl: true, ..Default::default() } }, generation: ui_host::InputGeneration(1) };
    let key = ui_host::DiscreteEvent { event: ui_render::DispatchEvent::KeyDown { key: "A".to_string(), modifiers: ui_render::EventModifiers::default() }, generation: ui_host::InputGeneration(3) };
    let mut events = ui_host::DrainedEvents { pointer_move: Some(pointer), ..Default::default() };
    events.discrete[0] = Some(scroll);
    events.discrete[1] = Some(key);
    let mut cursor = RuntimeDispatchCursor::new(events);

    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::Scroll { .. })));
    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::PointerMove { .. })));
    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::KeyDown { .. })));
    assert!(cursor.take_next().is_none());
    assert!(cursor.terminal_is_empty());
}

#[test]
fn runtime_dispatch_cursor_retires_one_owned_event_per_step() {
    let mut events = ui_host::DrainedEvents::default();
    for (index, slot) in events.discrete.iter_mut().enumerate() {
        *slot = Some(ui_host::DiscreteEvent { event: ui_render::DispatchEvent::Paste { text: "x".repeat(4096) }, generation: ui_host::InputGeneration(index as u64) });
    }
    let mut cursor = RuntimeDispatchCursor::new(events);
    for _ in 0..cursor.events.discrete.len() {
        assert!(!cursor.close_step());
    }
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
}

#[test]
fn frame_deferred_cursor_advances_one_owned_operation_in_order() {
    let mut actions = FrameActionOwners::default();
    assert!(actions.try_push(ActionDescriptor { controller_id: "a".to_string(), action: "one".to_string(), args: None }).is_ok());
    assert!(actions.try_push(ActionDescriptor { controller_id: "b".to_string(), action: "two".to_string(), args: None }).is_ok());
    let mut cursor = FrameDeferredCursor::new(actions, true, true, true, false, 1, semio_framework_job::root_cancel_token());
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::ShellMaintenance)));
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::PumpSync)));
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Action(action)) if action.action == "one"));
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Action(action)) if action.action == "two"));
    assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::FlushTutorial)));
    assert!(cursor.take_next().is_none());
    assert!(cursor.terminal_is_empty());
}

#[test]
fn frame_deferred_cancel_retires_one_action_per_step() {
    let mut actions = FrameActionOwners::default();
    for index in 0..WORLD3D_DEADLINE_CAPACITY {
        assert!(actions.try_push(ActionDescriptor { controller_id: index.to_string(), action: "cancel".to_string(), args: None }).is_ok());
    }
    let mut cursor = FrameDeferredCursor::new(actions, false, true, false, false, 1, semio_framework_job::root_cancel_token());
    for _ in 0..WORLD3D_DEADLINE_CAPACITY {
        assert!(!cursor.close_step());
    }
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
}

#[test]
fn catalogue_terminal_pair_never_partially_enters_the_frame_action_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🛒️canvas-catalogue-terminal/🔣️.json"))).unwrap();
    let frame = &fixture["frameOwner"];
    assert_eq!(WORLD3D_DEADLINE_CAPACITY, frame["capacity"].as_u64().unwrap() as usize);
    let mut actions = FrameActionOwners::default();
    for index in 0..frame["occupiedBeforePair"].as_u64().unwrap() as usize {
        actions.try_push(ActionDescriptor { controller_id: "fixture".into(), action: format!("occupied-{index}"), args: None }).unwrap();
    }
    let mut input = InputState::<ActionDescriptor>::default();
    let mut batch = input.reserve_actions(2, 512).unwrap();
    batch
        .action("layout-play", "canvasDragLeave", 256, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), "canvas.catalogue.terminal")?;
            builder.end_container()
        })
        .unwrap();
    batch
        .action("layout-play", "canvasDrop", 256, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), "canvas.catalogue.terminal")?;
            builder.string(Some("dragData"), "{\"kind\":\"rect\"}")?;
            builder.end_container()
        })
        .unwrap();
    batch.publish().unwrap();
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Deferred), "expected capacity pressure defers the whole source batch without faulting the frame");
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(2)), "refusal retains the complete source batch at its original owner");
    assert_eq!(actions.pop_front().unwrap().action, "occupied-0", "the previously published FIFO remains independently drainable");
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Pending), "one opportunity materializes only the first member into the staged owner");
    assert_eq!(input.take_action_batch_len_step(), Ok(Some(1)), "the unpublished drop remains source-owned between opportunities");
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Transferred));
    assert_eq!(input.take_action_batch_len_step(), Ok(None));
    let mut published = Vec::new();
    while let Some(action) = actions.pop_front() {
        published.push(action.action);
    }
    assert_eq!(published.first().map(String::as_str), Some("occupied-1"));
    assert_eq!(published[published.len() - 2..].iter().map(String::as_str).collect::<Vec<_>>(), ["canvasDragLeave", "canvasDrop"]);
    assert_eq!(frame["expectedPairAdmission"], "refusedWhole");
}

#[test]
fn a_fault_after_staging_leave_retires_the_source_drop_before_a_later_single_dispatches() {
    let mut input = InputState::<ActionDescriptor>::default();
    let mut terminal = input.reserve_actions(2, 64).unwrap();
    terminal.action("fixture", "canvasDragLeave", 32, |_| Ok(())).unwrap();
    terminal.action("fixture", "canvasDrop", 32, |_| Ok(())).unwrap();
    terminal.publish().unwrap();
    input.reserve_action("fixture", "later", 16).unwrap().publish().unwrap();
    let mut actions = FrameActionOwners::default();
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Pending));
    input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    let mut faulted = false;
    for _ in 0..ui_wgpu::wgpu::action::ACTION_BATCH_ITEM_CAPACITY * 2 + 2 {
        match transfer_frame_input_action(&mut input, &mut actions) {
            Err("bounded frame input action batch source faulted") => {
                faulted = true;
                break;
            }
            Ok(FrameInputActionStep::Pending) => {}
            other => panic!("fault retirement published or deferred a terminal member: {other:?}"),
        }
    }
    assert!(faulted, "the batch fault becomes observable after bounded staged/source retirement");
    assert!(actions.is_empty(), "no terminal prefix entered the dispatch ledger");
    assert_eq!(transfer_frame_input_action(&mut input, &mut actions), Ok(FrameInputActionStep::Transferred));
    assert_eq!(actions.pop_front().unwrap().action, "later", "the independent successor remains the next dispatchable owner");
    assert_eq!(input.take_action_batch_len_step(), Ok(None));
}

#[test]
fn frame_deferred_cancel_token_closes_one_exact_owner_per_grant() {
    let cancel = semio_framework_job::root_cancel_token();
    let mut actions = FrameActionOwners::default();
    assert!(actions.try_push(ActionDescriptor { controller_id: "cancel-owner".to_string(), action: "one".to_string(), args: None }).is_ok());
    let mut cursor = FrameDeferredCursor::new(actions, true, true, true, false, 17, cancel.clone());
    cancel.cancel_now();
    assert!(cursor.cancel.is_cancelled_now());
    cursor.begin_close();
    assert!(!cursor.close_step());
    assert!(!cursor.close_step());
    assert!(!cursor.close_step());
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation() {
    let authority = FrameMaintenanceAuthority::new();
    assert!(authority.try_reserve(41));
    assert!(authority.is_live(41));
    assert!(!authority.try_reserve(42));
    assert!(!authority.release(42));
    assert!(authority.is_live(41));
    assert!(authority.release(41));
    assert!(!authority.is_live(41));
    assert!(authority.try_reserve(42));
    assert!(authority.release(42));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking() {
    let owner = Box::new(73u64);
    let identity = owner.as_ref() as *const u64;
    let cell = FrameMaintenanceOwnerCell::new(owner);
    let Some(owner) = cell.try_take() else { panic!("exact owner admission") };
    assert_eq!(owner.as_ref() as *const u64, identity);
    assert!(cell.try_take().is_none());
    assert!(cell.try_restore(owner).is_ok());
    let Some(owner) = cell.try_take() else { panic!("exact owner handback") };
    assert_eq!(owner.as_ref() as *const u64, identity);
    assert!(cell.try_take().is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant() {
    let cancel = semio_framework_job::root_cancel_token();
    let mut cancelled_actions = FrameActionOwners::default();
    assert!(cancelled_actions.try_push(ActionDescriptor { controller_id: "cancelled".to_string(), action: "one".to_string(), args: None }).is_ok());
    let mut cancelled = FrameDeferredCursor::new(cancelled_actions, false, false, false, false, 81, cancel.clone());
    cancel.cancel_now();
    assert_eq!(frame_maintenance_terminal_fault(cancelled.cancel.is_cancelled_now(), false, false), Some("frame maintenance cancelled"));
    cancelled.begin_close();
    let before = cancelled.actions.len;
    assert!(!cancelled.close_step());
    assert_eq!(before - cancelled.actions.len, 1);
    assert!(cancelled.close_step());
    assert!(cancelled.terminal_is_empty());

    let authority = FrameMaintenanceAuthority::new();
    assert!(authority.try_reserve(82));
    assert!(authority.release(82));
    let mut stale_actions = FrameActionOwners::default();
    assert!(stale_actions.try_push(ActionDescriptor { controller_id: "stale".to_string(), action: "one".to_string(), args: None }).is_ok());
    let mut stale = FrameDeferredCursor::new(stale_actions, false, false, false, false, 82, semio_framework_job::root_cancel_token());
    assert_eq!(frame_maintenance_terminal_fault(false, !authority.is_live(stale.generation), false), Some("frame maintenance generation became stale"));
    stale.begin_close();
    let before = stale.actions.len;
    assert!(!stale.close_step());
    assert_eq!(before - stale.actions.len, 1);
    assert!(stale.close_step());
    assert!(stale.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
fn frame_maintenance_test_owner(generation: u64, actions: usize) -> FrameMaintenanceOwner {
    let mut action_owners = FrameActionOwners::default();
    for index in 0..actions {
        assert!(action_owners.try_push(ActionDescriptor { controller_id: format!("owner-{index}"), action: "close".to_string(), args: None }).is_ok());
    }
    let interaction = AppInteractionState {
        shell: ShellState::new(Vec::new(), "maintenance-owner".to_string()),
        input: InputState::default(),
        theme: Theme::default(),
        theme_dark: false,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        pointer_capture: shell::PointerCapture::default(),
        modifiers: PointerModifiers::default(),
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        caret_blink_at_ms: 0.0,
        caret_blink_visible: true,
        text_streams: std::array::from_fn(|_| None),
        text_fault: None,
        frame_fault: None,
        text_cancel_pending: false,
        last_sync_pump_ms: 0.0,
    };
    FrameMaintenanceOwner { interaction: Some(interaction), cursor: Some(FrameDeferredCursor::new(action_owners, true, true, true, false, generation, semio_framework_job::root_cancel_token())), deadline_ms: Some(u64::MAX) }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_incomplete_text_stream_waits_for_external_ingress_without_a_worker_self_wake() {
    let mut interaction = frame_maintenance_test_owner(83, 0).interaction.take().expect("text interaction owner");
    assert!(!interaction.has_pending_text_work(), "an empty text authority has no runnable work");
    interaction.start_text_operation(71, 6).expect("text stream admission");
    assert!(!interaction.has_pending_text_work(), "a reserved stream awaiting its first page is externally blocked");
    interaction.push_text_operation(71, "abc".to_string()).expect("first text page");
    assert!(!interaction.has_pending_text_work(), "a partial stream awaits its next ingress page without self-waking");
    interaction.push_text_operation(71, "def".to_string()).expect("final text page");
    interaction.commit_text_operation(71).expect("text stream commit");
    assert!(interaction.has_pending_text_work(), "a committed stream owns runnable text-buffer work");

    for _ in 0..128 {
        if !interaction.has_pending_text_work() {
            break;
        }
        interaction.drive_text_operation();
    }
    assert!(!interaction.has_pending_text_work(), "the committed stream reaches an idle direct-continuation state");

    interaction.start_text_operation(72, 3).expect("cancelled text stream admission");
    interaction.cancel_text_operations();
    assert!(interaction.has_pending_text_work(), "explicit cancellation remains runnable while it retires the reserved owner");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accepted_frame_maintenance_queue_drop_publishes_exact_generation_owner() {
    let registry = FrameMaintenanceExecutionRegistry::new();
    let owner = frame_maintenance_test_owner(91, 1);
    let identity = owner.interaction.as_ref().map(|interaction| interaction.shell.plugin_filter.as_ptr());
    let cell = Arc::new(FrameMaintenanceOwnerCell::new(owner));
    assert!(registry.try_publish(91, cell.clone()).is_ok());
    assert!(!registry.abandon(92));
    assert!(registry.abandon(91));
    let Some((generation, recovered_cell)) = registry.try_take_abandoned() else { panic!("abandoned execution owner") };
    assert_eq!(generation, 91);
    assert!(Arc::ptr_eq(&cell, &recovered_cell));
    let Some(recovered) = recovered_cell.try_take() else { panic!("exact abandoned owner") };
    assert_eq!(recovered.interaction.as_ref().map(|interaction| interaction.shell.plugin_filter.as_ptr()), identity);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn interrupted_frame_maintenance_execution_restores_before_incremental_recovery() {
    let registry = FrameMaintenanceExecutionRegistry::new();
    let owner = frame_maintenance_test_owner(93, 2);
    let cell = Arc::new(FrameMaintenanceOwnerCell::new(owner));
    assert!(registry.try_publish(93, cell.clone()).is_ok());
    let Some(running_cell) = registry.try_begin(93) else { panic!("running execution cell") };
    let Some(mut running_owner) = running_cell.try_take() else { panic!("running exact owner") };
    let Some(cursor) = running_owner.cursor.as_mut() else { panic!("running cursor") };
    cursor.begin_close();
    running_cell.restore_taken(running_owner);
    assert!(registry.abandon(93));
    let Some((generation, recovered_cell)) = registry.try_take_abandoned() else { panic!("interrupted execution owner") };
    assert_eq!(generation, 93);
    let Some(mut recovered) = recovered_cell.try_take() else { panic!("interrupted exact owner") };
    let Some(cursor) = recovered.cursor.as_mut() else { panic!("interrupted cursor") };
    let before = cursor.actions.len;
    assert!(!cursor.close_step());
    assert_eq!(before - cursor.actions.len, 1);
}

#[test]
fn frame_action_max_plus_one_returns_the_exact_owner() {
    let mut actions = FrameActionOwners::default();
    for index in 0..WORLD3D_DEADLINE_CAPACITY {
        assert!(actions.try_push(ActionDescriptor { controller_id: index.to_string(), action: "admitted".to_string(), args: None }).is_ok());
    }
    let rejected = ActionDescriptor { controller_id: "exact-owner".to_string(), action: "commit".to_string(), args: None };
    let identity = rejected.controller_id.as_ptr();
    let Err(rejected) = actions.try_push(rejected) else { panic!("MAX + 1 must refuse") };
    assert_eq!(rejected.controller_id.as_ptr(), identity);
}

#[test]
fn asset_poll_does_not_accumulate_completed_response_vectors() {
    assert!(!LIBRARY_SOURCE.contains("let mut fetched_glb = Vec::new()"));
    assert!(!LIBRARY_SOURCE.contains("let mut fetched_map = Vec::new()"));
    assert!(!LIBRARY_SOURCE.contains("let mut fetched_ui_images = Vec::new()"));
}

/// 🔗️ The presenter contract's own cross-reference to the law below, checked by the COMPILER rather
/// than by a source-text match: the census in `presenter_retirement_contract` only states that
/// freshness is decided once because this law states that the authority and a candidate witness move
/// independently. Renaming or deleting the law fails the build instead of quietly narrowing the
/// contract, which is exactly how the old text-marker version went stale.
const _: fn() = runtime_presentation_authority_and_candidate_identity_change_independently;

#[test]
fn runtime_presentation_authority_and_candidate_identity_change_independently() {
    let authority = RuntimePresentationAuthority::new();
    authority.observe_input_generation(7);
    let unchanged_candidate = authority.current();
    assert!(authority.matches(unchanged_candidate.scene_revision, unchanged_candidate.input_generation));

    authority.mark_scene_changed();
    assert_eq!(unchanged_candidate, RuntimePresentationWitness { scene_revision: 1, input_generation: 7 });
    assert!(!authority.matches(unchanged_candidate.scene_revision, unchanged_candidate.input_generation));

    let unchanged_authority = authority.current();
    let changed_candidate = RuntimePresentationWitness { scene_revision: unchanged_authority.scene_revision + 1, input_generation: unchanged_authority.input_generation + 1 };
    assert_eq!(authority.current(), unchanged_authority);
    assert!(!authority.matches(changed_candidate.scene_revision, changed_candidate.input_generation));

    authority.observe_input_generation(8);
    assert!(authority.witness_for(7).is_none());
    assert_eq!(authority.witness_for(8), Some(authority.current()));
}

#[test]
fn runtime_raster_operation_authority_rejects_stale_duplicate_and_device_close_ack() {
    let authority = RuntimeRasterOperationAuthority::new();
    let first = authority.begin(3, 5).expect("independent raster operation");
    assert_eq!(first.scene_revision, 3);
    assert_eq!(first.preview_generation, 5);
    assert!(authority.matches(first));
    assert!(authority.begin(3, 5).is_err());
    let stale = ui_wgpu::wgpu::RasterTextureWitness { operation: first.operation.wrapping_add(1), ..first };
    assert!(!authority.matches(stale));
    assert!(authority.release(stale).is_err());
    assert!(authority.matches(first));
    authority.release(first).expect("device close returns exact operation owner");
    assert!(authority.current().is_none());
    assert!(authority.release(first).is_err());
    let second = authority.begin(3, 5).expect("next operation");
    assert!(second.operation > first.operation);
}

#[test]
fn runtime_raster_operation_authority_exhausts_permanently_without_aba() {
    let authority = RuntimeRasterOperationAuthority::new();
    authority.0.next_operation.store(u64::MAX - 1, Ordering::Release);

    let penultimate = authority.begin(8, 13).expect("MAX-1 operation");
    assert_eq!(penultimate.operation, u64::MAX - 1);
    authority.release(penultimate).expect("release MAX-1");

    let last = authority.begin(8, 13).expect("MAX operation");
    assert_eq!(last.operation, u64::MAX);
    assert!(authority.begin(8, 13).is_err(), "live MAX remains occupied");
    authority.release(last).expect("release MAX");
    assert!(authority.begin(8, 13).is_err(), "exhaustion remains permanent after release");
    assert!(authority.current().is_none());
    assert!(authority.0.exhausted.load(Ordering::Acquire));
    assert_eq!(authority.0.next_operation.load(Ordering::Acquire), u64::MAX);
}

/// 🎟️ LAW (source census): the presentation authority is read ONCE per frame, by the BUILD, and the
/// presenter is then bound to that frozen pair for the rest of the presentation.
///
/// The census walks the four hand-offs that make that one claim true:
///
/// 1. **The build asks for its own generation.** `AppFrameTransaction::step` refuses to advance when
///    `presentation_witness_for(self.generation.0)` answers nothing (a superseded input generation),
///    and the `AppFrameTransactionPhase::Build` arm asks again to mint the `FrameBuildCursor`. Two
///    reads, both keyed on the transaction's OWN generation, and the second one writes what it read
///    into the authority with `admit_build_presentation_witness` — the build declares the pair it
///    was admitted under instead of leaving the presenter to guess.
/// 2. **The presenter reads `admitted()`, never `current()`.** `current()` is a moving target:
///    `mark_scene_changed` fires from every runtime completion the host pumps, several per tick, and
///    `observe_presentation_input_generation` is republished on every `build_and_publish_snapshot`.
///    A presenter that re-read the live pair at admission refused packets that had been built
///    correctly against the authority as it stood when the build started and turned them into
///    surface faults (`prepared render revision is stale: live=35, packet=27` →
///    `worker-present-failed`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// 3. **Acknowledgement compares against the FROZEN witness.** `AppPresentPhase::BeginGpu` mints one
///    `RasterTextureWitness` from the admitted pair; the acknowledge phase compares the packet to
///    THAT witness, so a presentation that legitimately spans more than one host tick is never
///    faulted by an authority that moved underneath it.
/// 4. **Every owner is returned by a step, never dropped** — the prepared witness ladder, the fixed
///    mesh GPU table, and the host's `close_world_owners_step` drain.
///
/// **React ref:** R3F's demand frameloop is the same contract. `invalidate()`
/// (`🌐️World3dHost/🟦️.tsx:2358` and its twelve siblings) marks the scene changed and the next raf
/// renders the scene AS IT STANDS when the render begins; React never refuses a queued frame because
/// something invalidated again in between — the later `invalidate` supersedes it with a new frame.
/// Freshness is the producer's decision on both targets.
fn presenter_retirement_contract(glue: &str, prepared: &str, gpu: &str, draw: &str, host: &str, winit: &str, laws: &str) -> bool {
    let table = &draw[draw.find("pub struct MeshGpuTable").unwrap_or(0)..draw.find("pub const WORLD_GLOBALS_SLOT_SIZE").unwrap_or(draw.len())];
    prepared.contains("pub struct PreparedPresenterWitness")
        && prepared.contains("pub fn stage_presented")
        && prepared.contains("pub fn pending_presented")
        && prepared.contains("pub fn acknowledge_presented")
        && prepared.contains("pub fn abort_pending")
        && prepared.contains("pub fn take_last_valid")
        && prepared.contains("pub fn retire_step(&mut self) -> bool")
        && !prepared.contains("Option<Arc<PreparedRenderPacket>>")
        && glue.contains("AppPresentPhase::Acknowledge")
        && glue.contains("AppPresentPhase::Stage")
        && glue.contains("AppPresentPhase::Aborted")
        && glue.contains("self.gate.stage_presented(packet)")
        && glue.contains("self.gate.pending_presented(witness)")
        && glue.contains("self.gate.acknowledge_presented(witness)")
        && glue.contains("struct RuntimePresentationAuthority")
        && glue.contains("presentation_authority: RuntimePresentationAuthority")
        && glue.matches(concat!("runtime.presentation_", "witness_for(self.generation.0)")).count() == 2
        && glue.contains("if base_witness != current_witness {")
        && glue.contains("self.base_witness = Some(current_witness);")
        && glue.contains(concat!("runtime.admit_build_presentation_", "witness(presentation_witness);"))
        && glue.contains("self.build_cursor = Some(FrameBuildCursor::new(presentation_witness));")
        && glue.contains("presentation_witness.scene_revision")
        && glue.contains("presentation_witness.input_generation")
        && glue.matches(concat!("let expected = self.presentation_", "authority.admitted();")).count() == 1
        && !glue.contains(concat!("let expected = self.presentation_", "authority.current();"))
        && glue.contains("begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
        && glue.contains("begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
        && glue.contains("self.raster_operation_authority.begin(expected.scene_revision, expected.input_generation)")
        && glue.contains(concat!("raster_witness.scene_revision != packet.scene_revision() || raster_witness.", "preview_generation != packet.preview_generation()"))
        && glue.matches("mark_scene_changed();").count() == 4
        && !glue.contains(concat!("let revision = packet.", "scene_revision();"))
        && !glue.contains(concat!("let generation = packet.", "preview_generation();"))
        && !glue.contains(concat!("scene_revision: packet.", "scene_revision()"))
        && !glue.contains(concat!("input_generation: packet.", "preview_generation()"))
        && glue.contains("struct AppPresentedRetirement")
        && glue.contains("acknowledged_eviction")
        && glue.contains("acknowledged_upload_scan")
        && glue.contains("acknowledged_versions")
        && glue.contains("previous.retire_step()")
        && glue.contains("if !gpu.close_mesh_upload_step()")
        && glue.contains("self.gpu.close_mesh_table_step()")
        && glue.contains("pub(crate) fn admit_next_frame")
        && winit.contains(".admit_next_frame(|| frame_build.poll_runtime_and_resubmit")
        && winit.contains("observe_presentation_input_generation(build_generation.0)")
        && !winit.contains("begin_present(frame).is_err()")
        && host.contains("presenter.close_world_owners_step()")
        && host.contains("presenter.world_owners_terminal_is_empty()")
        && draw.contains("pub const MESH_GPU_TABLE_CAPACITY: usize = 256")
        && draw.contains("slots: [Option<MeshGpuEntry<T>>; MESH_GPU_TABLE_CAPACITY]")
        && draw.contains("pub fn evict_mesh_step")
        && draw.contains("pub fn evict_mesh_except_step")
        && laws.contains("fn mesh_gpu_retirement_preserves_acknowledged_versions()")
        && draw.contains("pub fn close_step(&mut self) -> bool")
        && laws.contains("fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner()")
        && !table.contains("HashMap")
        && !table.contains(".retain(")
        && gpu.contains("self.mesh_store.evict_mesh_except_step(key, keep_versions)")
        && gpu.contains("self.mesh_store.close_step()")
}

#[test]
fn presenter_ack_retirement_source_mutations_are_denied() {
    assert!(presenter_retirement_contract(LIBRARY_SOURCE, PREPARED_SOURCE, GPU_SOURCE, DRAW_SOURCE, OS_HOST_SOURCE, WINT_APP_SOURCE, DRAW_LAWS_SOURCE));
    for (source, from, to, once) in PRESENTER_RETIREMENT_MUTATIONS {
        let mutated = if *once { source.text().replacen(from, to, 1) } else { source.text().replace(from, to) };
        assert_ne!(mutated.as_str(), source.text(), "mutation '{from}' must actually rewrite {source:?} — a no-op edit proves nothing");
        let (glue, prepared, gpu, draw, host, laws, winit) = source.substitute(&mutated);
        assert!(!presenter_retirement_contract(glue, prepared, gpu, draw, host, winit, laws), "{source:?} mutation '{from}' -> '{to}' was accepted by the presenter retirement contract");
    }
}

/// 🧬️ Which source of the presenter ladder a mutation rewrites.
#[derive(Clone, Copy, Debug)]
enum PresenterContractSource {
    Glue,
    Prepared,
    Gpu,
    Draw,
    Host,
    Laws,
    Winit,
}

impl PresenterContractSource {
    fn text(self) -> &'static str {
        match self {
            PresenterContractSource::Glue => LIBRARY_SOURCE,
            PresenterContractSource::Prepared => PREPARED_SOURCE,
            PresenterContractSource::Gpu => GPU_SOURCE,
            PresenterContractSource::Draw => DRAW_SOURCE,
            PresenterContractSource::Host => OS_HOST_SOURCE,
            PresenterContractSource::Laws => DRAW_LAWS_SOURCE,
            PresenterContractSource::Winit => WINT_APP_SOURCE,
        }
    }

    fn substitute<'a>(self, mutated: &'a str) -> (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str) {
        let mut sources = [LIBRARY_SOURCE, PREPARED_SOURCE, GPU_SOURCE, DRAW_SOURCE, OS_HOST_SOURCE, DRAW_LAWS_SOURCE, WINT_APP_SOURCE];
        sources[self as usize] = mutated;
        (sources[0], sources[1], sources[2], sources[3], sources[4], sources[5], sources[6])
    }
}

/// 🧪️ Every rewrite `presenter_retirement_contract` must refuse — `(source, from, to, replace_once)`.
/// Each one is a real regression this ladder has already suffered or a shortcut it invites, so a
/// clause that stops discriminating shows up here as an accepted mutation rather than as silence.
const PRESENTER_RETIREMENT_MUTATIONS: &[(PresenterContractSource, &str, &str, bool)] = &[
    (PresenterContractSource::Glue, "AppPresentPhase::Acknowledge", "AppPresentPhase::Fullscreen", false),
    (PresenterContractSource::Glue, "self.gate.acknowledge_presented(witness)", "drop(witness); self.gate.acknowledge_presented_unchecked()", false),
    (PresenterContractSource::Glue, "previous.retire_step()", "drop(previous)", false),
    (PresenterContractSource::Glue, "AppPresentPhase::Stage", "AppPresentPhase::Render", false),
    (PresenterContractSource::Glue, "AppPresentPhase::Aborted", "AppPresentPhase::Render", false),
    (PresenterContractSource::Glue, "let expected = self.presentation_authority.admitted();", "let expected = self.presentation_authority.current();", false),
    (PresenterContractSource::Glue, "let expected = self.presentation_authority.admitted();", "let expected = RuntimePresentationWitness { scene_revision: packet.scene_revision(), input_generation: packet.preview_generation() };", true),
    (PresenterContractSource::Glue, "runtime.admit_build_presentation_witness(presentation_witness);", "", false),
    (PresenterContractSource::Glue, "runtime.presentation_witness_for(self.generation.0)", "Some(RuntimePresentationWitness { scene_revision: self.generation.0, input_generation: self.generation.0 })", false),
    (PresenterContractSource::Glue, "if base_witness != current_witness {", "if false {", false),
    (PresenterContractSource::Glue, "raster_witness.scene_revision != packet.scene_revision() || raster_witness.preview_generation != packet.preview_generation()", "false", false),
    (PresenterContractSource::Glue, "mark_scene_changed();", "", true),
    (PresenterContractSource::Glue, "if !gpu.close_mesh_upload_step()", "if false", false),
    (PresenterContractSource::Prepared, "pub fn abort_pending", "fn abandon_pending", false),
    (PresenterContractSource::Prepared, "last_valid: Option<PreparedRenderPacket>", "last_valid: Option<Arc<PreparedRenderPacket>>", false),
    (PresenterContractSource::Prepared, "pub fn retire_step(&mut self) -> bool", "pub fn retire_all(&mut self)", false),
    (PresenterContractSource::Gpu, "self.mesh_store.evict_mesh_except_step(key, keep_versions)", "self.mesh_store.evict_mesh_step(key)", false),
    (PresenterContractSource::Gpu, "self.mesh_store.close_step()", "drop(&mut self.mesh_store)", false),
    (PresenterContractSource::Draw, "MESH_GPU_TABLE_CAPACITY: usize = 256", "MESH_GPU_TABLE_CAPACITY: usize = usize::MAX", false),
    (PresenterContractSource::Draw, "FixedMeshGpuRegistry<GpuMeshBuffers>", "std::collections::HashMap<String, GpuMeshBuffers>", false),
    (PresenterContractSource::Draw, "pub fn close_step(&mut self) -> bool", "pub fn close_all(&mut self) -> bool", false),
    (PresenterContractSource::Draw, "pub fn evict_mesh_except_step", "pub fn evict_mesh_all", false),
    (PresenterContractSource::Laws, "fn mesh_gpu_retirement_preserves_acknowledged_versions()", "fn mesh_gpu_retirement_drops_everything()", false),
    (PresenterContractSource::Laws, "fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner()", "fn fixed_mesh_gpu_registry_grows()", false),
    (PresenterContractSource::Host, "presenter.world_owners_terminal_is_empty()", "true", false),
    (PresenterContractSource::Host, "presenter.close_world_owners_step()", "drop(presenter)", false),
    (PresenterContractSource::Winit, ".admit_next_frame(|| frame_build.poll_runtime_and_resubmit", ".poll_runtime_and_resubmit", false),
    (PresenterContractSource::Winit, "self.runtime.observe_presentation_input_generation(build_generation.0);", "", false),
];

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
        begin_world3d_dynamic_retirement, finish_world3d_asset, publish_world3d_asset_mesh_lease, reserve_world3d_asset_request, reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response, step_world3d_dynamic_retirement,
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
    reserve_world3d_asset_request(&mut state, WorldAssetRequestKind::Glb, url).expect("the surface admits its own GLB request");
    let mut owner = take_next_world3d_asset(&mut state).expect("the host drains exactly the admitted request");
    assert_eq!(owner.url(), url);
    reserve_world3d_asset_response(&mut state, &mut owner, glb.len()).expect("response credits");
    for chunk in glb.chunks(64) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(chunk.to_vec()).expect("bounded response page")).expect("page admission");
    }
    seal_world3d_asset_response(&mut state, &mut owner).expect("sealed response");
    return_world3d_asset(&mut state, owner).expect("handback");

    let owner = take_next_completed_world3d_asset_step(&mut state).expect("completed decode owner");
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
    let lease = probe.take_ready_mesh_lease().expect("the decoded mesh lease");
    publish_world3d_asset_mesh_lease(&mut state, url, lease).expect("the decoded GLB publishes under the url's own mesh id");

    let resident = state.mesh_lease("mesh:🧊️probe").expect("the mesh is resident under `mesh_id_from_url`, the id the wire names it by");
    let schema = resident.schema().expect("resident mesh schema");
    assert_eq!((schema.vertices, schema.indices), (3, 3), "the resident mesh carries real positions and indices, not an empty placeholder");
    let mut positions = Vec::new();
    let mut cursor = resident.cursor(Mesh3dField::Positions).unwrap();
    while let Some(Mesh3dItem::Vec3(value)) = cursor.read_next().unwrap() {
        positions.push(value);
    }
    assert_eq!(positions, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]], "glTF's Y-up geometry arrives in the world's Z-up frame, the way React's `<group rotation={{[π/2, 0, 0]}}>` delivers it");

    probe.begin_close();
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().expect("terminal owner") else { panic!("shared probe") };
    finish_world3d_asset(&mut state, owner).expect("terminal handback");
    assert!(begin_world3d_dynamic_retirement(&mut state));
    for _ in 0..4_096 {
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        if step_world3d_dynamic_retirement(&mut state, &mut context) {
            break;
        }
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
    let glb: &[u8] = include_bytes!("../../../../../../../🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let url = "/mesh/🧊️capsule_J.glb";
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    reserve_world3d_asset_request(&mut state, WorldAssetRequestKind::Glb, url).unwrap();
    let mut owner = take_next_world3d_asset(&mut state).unwrap();
    reserve_world3d_asset_response(&mut state, &mut owner, glb.len()).unwrap();
    for chunk in glb.chunks(16 * 1024) {
        owner.push_page(WorldAssetResponsePage::try_from_owned(chunk.to_vec()).unwrap()).unwrap();
    }
    seal_world3d_asset_response(&mut state, &mut owner).unwrap();
    return_world3d_asset(&mut state, owner).unwrap();
    let owner = take_next_completed_world3d_asset_step(&mut state).unwrap();
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
    let lease = probe.take_ready_mesh_lease().unwrap();
    publish_world3d_asset_mesh_lease(&mut state, url, lease).unwrap();
    let resident = state.mesh_lease("mesh:🧊️capsule_J").unwrap();
    let schema = resident.schema().unwrap();
    assert_eq!((schema.vertices, schema.indices, schema.uvs), (1_472, 5_250, 1_472), "the whole export lands, every primitive welded into one mesh");
    assert!(steps < 100_000, "the decode stays inside a bounded step count: {steps}");
    probe.begin_close();
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!() };
    finish_world3d_asset(&mut state, owner).unwrap();
    assert!(begin_world3d_dynamic_retirement(&mut state));
    for _ in 0..8_192 {
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        if step_world3d_dynamic_retirement(&mut state, &mut context) {
            break;
        }
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
    let ladder = GPU_SOURCE.split("pub fn prepared_present_step").nth(1).expect("the prepared present ladder");
    let ladder = &ladder[..ladder.find("fn encode_prepared_draw_scalar").unwrap_or(ladder.len())];
    assert_eq!(ladder.matches("get_current_texture").count(), 1, "exactly one phase may acquire the surface");
    assert_eq!(ladder.matches("frame.present()").count(), 1, "and exactly one presents it");
    assert!(!GPU_SOURCE.contains("PreparedGpuPresentPhase::AcquireSurface"), "acquiring in its own phase is the defect this law exists for");
    assert!(!GPU_SOURCE.contains("PreparedGpuPresentPhase::CreateView"), "and so is viewing it in its own phase");

    let present = ladder.split("PreparedGpuPresentPhase::Present =>").nth(1).expect("the terminal present phase");
    let present = &present[..present.find("PreparedGpuPresentPhase::Complete").unwrap_or(present.len())];
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
    let ladder = GPU_SOURCE.split("pub fn prepared_present_step").nth(1).expect("the prepared present ladder");
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
    assert!(!GPU_SOURCE.contains("ForegroundCommands"), "no foreground replay can overprint a later popup");
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
    assert!(DRAW_SOURCE.contains("let Some(mesh) = mesh_store.get_versioned(mesh_key, mesh_version) else { return Ok(false) };"), "the encoder answers a non-resident mesh with a skip");
    assert!(!DRAW_SOURCE.contains("prepared world mesh was missing"), "and no fault string survives anywhere in the encoder");
    assert!(GPU_SOURCE.contains("self.missing_world_mesh = Some((draw_owner.mesh_key.clone(), draw_owner.mesh_version));"), "the GPU context records exactly which mesh was skipped");
    assert!(GPU_SOURCE.contains("pub fn take_missing_world_mesh"), "and hands it over once");
    assert!(LIBRARY_SOURCE.contains("os_host prepared world mesh was not resident, draw skipped"), "which the host reports on the transition");
    let render = LIBRARY_SOURCE.split("AppPresentPhase::Render => {").nth(1).expect("the render phase");
    assert!(render[..render.find("AppPresentPhase::CloseGpu =>").unwrap_or(render.len())].contains("take_missing_world_mesh()"), "from the phase that submits the frame");
}

#[test]
fn presented_ink_intent_barrier_progresses_before_gpu_submit_and_holds_runtime_ingress_until_acknowledgement() {
    let render = LIBRARY_SOURCE.split("AppPresentPhase::Render => {").nth(1).expect("the render phase");
    let render = &render[..render.find("AppPresentPhase::CloseGpu =>").expect("the close-GPU phase")];
    let progress = render.find("progress_presented_input_candidate").expect("the presenter-owned input barrier progress seam");
    let submit = render.find("begin_prepared_present").expect("the first GPU submit operation");
    assert!(progress < submit, "the old presented interaction reaches a terminal state before candidate pixels can submit");

    let acknowledge = LIBRARY_SOURCE.split("AppPresentPhase::Acknowledge => {").nth(1).expect("the acknowledgement phase");
    let acknowledge = &acknowledge[..acknowledge.find("AppPresentPhase::ProgressAcknowledge =>").expect("the next presenter phase")];
    let input_ack = acknowledge.find("shell.acknowledge_presented_input").expect("the exact input witness acknowledgement");
    let packet_ack = acknowledge.find("gate.acknowledge_presented").expect("the prepared packet acknowledgement");
    assert!(input_ack < packet_ack, "a refused input witness keeps the packet witness live for a Pending retry");

    let host_build = WINT_APP_SOURCE.split("fn build_and_publish_snapshot(&mut self) {").nth(1).expect("the host build pump");
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

    let host_build = WINT_APP_SOURCE.split("fn build_and_publish_snapshot(&mut self) {").nth(1).expect("the host build pump");
    let host_build = &host_build[..host_build.find("fn present_snapshot").expect("the host presentation half")];
    let close_step = host_build.find("self.advance_component_surface_close()").expect("one bounded component-close step");
    let event_drain = host_build.find("if !self.events.is_empty()").expect("the unrelated input drain");
    let close_gate = host_build.find("if !component_close_pending").expect("the component-close frame-admission gate");
    let frame_admission = host_build.find("self.presenter.admit_next_frame").expect("the unrelated frame admission");
    let snapshot_publication = host_build.find("self.snapshot_sink.publish").expect("the immutable snapshot publication");
    assert!(close_step < event_drain && event_drain < close_gate && close_gate < frame_admission && frame_admission < snapshot_publication, "input remains queued while the bridge gates frame admission until terminal");
    assert!(host_build[close_step..event_drain].contains("InvalidationReason::RESOURCE_READY"), "the nonterminal close retains its bounded progress wake while the sibling advances");

    let close_owner = OS_HOST_SOURCE.split("pub(crate) fn advance_component_surface_close(&mut self) -> bool {").nth(1).expect("the component-close owner");
    let admission = &close_owner[..close_owner.find("let Some(owner)").expect("the active exact close owner")];
    assert!(admission.contains("self.presenter.has_pending_presentation()"), "an already prepared packet remains a hard close-admission fence");
    assert!(admission.contains("component_surface_close_pending() && self.frame_build.has_live_session()") && admission.contains("retire_for_component_surface_close_step"), "the frame that created the bridge is retired one bounded unit before external ownership detaches the surface");

    let frame_job = include_str!("../../🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
    let transient = frame_job.split("pub(crate) fn retire_for_component_surface_close_step(&mut self) -> bool {").nth(1).expect("the reusable frame handoff");
    let transient = &transient[..transient.find("pub(crate) fn close_step").expect("the permanent close path")];
    assert!(transient.contains("self.last_submitted_generation = None"), "native may readmit the same generation after handoff");
    assert!(!transient.contains("self.closing = true") && !transient.contains("completion_waker.take()"), "transient retirement preserves reusable-host state");
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
    assert!(DRAW_SOURCE.contains("pub fn upload_progress(&self) -> (u32, u32)"), "the mesh table exposes its own cursor's walk");
    assert!(GPU_SOURCE.contains("pub fn prepared_upload_progress(&self) -> (u32, u32, usize)"), "the GPU context joins it with the atlas page cursor");
    assert!(LIBRARY_SOURCE.contains("let upload_progress = self.gpu.prepared_upload_progress();"), "and the presenter reads it every step");
    assert!(LIBRARY_SOURCE.contains("cursor.gpu_cursor.as_ref().map(ui_wgpu::wgpu::PreparedGpuPresentCursor::progress)"), "the signature retains exact GPU cursor progress");
    assert!(LIBRARY_SOURCE.contains("upload_progress,") && LIBRARY_SOURCE.contains("cursor.raster_keep_steps,") && LIBRARY_SOURCE.contains("cursor.input_progress,"), "upload, raster-ownership, and bounded input progress are independent signature terms");
}
