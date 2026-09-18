
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
    let engine_reservation = "let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?;";
    let engine_reservation_index = engine.find(engine_reservation).unwrap_or(usize::MAX);
    let first_engine_allocation = ["create_target_texture", ".create_view", "Renderer::new"].iter().filter_map(|marker| engine.find(marker)).min().unwrap_or(0);
    let upload_stage = &draw[draw.find("pub(crate) fn ensure_raster_step").unwrap_or(draw.len())..draw.find("pub fn get(&self, key: &str) -> Option<&RasterTexture>").unwrap_or(draw.len())];
    let gpu_stage = &draw[draw.find("pub fn stage_gpu_bind_group").unwrap_or(draw.len())..draw.find("pub fn begin_presenting").unwrap_or(draw.len())];
    let cancellation = &draw[draw.find("pub fn cancel_engine_texture_admission").unwrap_or(draw.len())..draw.find("fn claim_stage_before_gpu_allocation").unwrap_or(draw.len())];
    let upload_close = &draw[draw.find("pub fn close_upload_step(&mut self) -> RasterTextureCleanupStep").unwrap_or(draw.len())..];
    let upload_close = &upload_close[..upload_close.find("pub fn close_step(&mut self)").unwrap_or(upload_close.len())];
    draw.contains("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256")
        && draw.contains("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256")
        && draw.contains("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 16 * 1024 * 1024")
        && draw.contains("pub const RASTER_TEXTURE_TABLE_BYTE_CAPACITY: usize = 256 * 1024 * 1024")
        && draw.contains("const RASTER_TEXTURE_PROBE_CAPACITY: usize = 8")
        && draw.contains("const PAGE_BYTES: usize = 16 * 1024")
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
        && upload_stage.contains("if let Err((fault, admission, value)) = self.stage_claimed_texture(admission, value, allocation_claim)")
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
        && gpu.contains("self.ensure_raster_texture_step(key, pixels")
        && gpu.contains("pub fn stage_engine_texture")
        && gpu.contains("view: wgpu::TextureView")
        && gpu.contains(".stage_gpu_bind_group")
        && gpu.contains("Result<(), RasterTextureStageFault>")
        && begin < present_step
        && engine.matches(engine_reservation).count() == 1
        && engine_reservation_index < first_engine_allocation
        && guarded_allocations(engine, "gpu.validate_engine_target_texture_allocation(&admission, expected)", "create_target_texture", 1)
        && guarded_allocations(engine, "gpu.validate_engine_target_view_allocation(&admission, expected)", ".create_view", 1)
        && guarded_allocations(engine, "gpu.validate_engine_renderer_allocation(&admission, expected)", "Renderer::new", 1)
        && guarded_allocations(engine, "gpu.validate_engine_replacement_texture_allocation(&admission, expected)", "create_target_texture", 2)
        && guarded_allocations(engine, "gpu.validate_engine_replacement_view_allocation(&admission, expected)", ".create_view", 2)
        && engine.matches("gpu.retain_engine_allocation_fault(admission, Some(texture), None)").count() == 2
        && engine.contains("gpu.retain_engine_allocation_fault(admission, Some(texture), Some(view))")
        && engine.contains("gpu.retain_engine_allocation_fault(admission, Some(replacement_texture), None)")
        && engine.contains("RasterTextureStageFault::Returned { fault, admission, texture, view }")
        && engine.contains("surface.texture = texture")
        && engine.contains("surface.view = view")
        && engine.contains("RasterTextureStageFault::Retained(fault)")
        && engine.contains("let published_view = std::mem::replace(&mut surface.view, replacement_view)")
        && !engine.contains("surface.view.clone()")
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

#[test]
fn raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete() {
    assert!(retained_raster_contract(DRAW_SOURCE, GPU_SOURCE, LIBRARY_SOURCE, ENGINE_CANVAS_SOURCE));
    let mutations = [
            (DRAW_SOURCE.replace("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256", "pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 257"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256", "pub const RASTER_TEXTURE_KEY_BYTES: usize = 255"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 16 * 1024 * 1024", "pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = usize::MAX"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.replace("pub struct RasterTextureAdmission", "pub struct ErasedRasterTextureAdmission"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("current.operation >= candidate.operation", "current.operation > candidate.operation"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("view: Option<wgpu::TextureView>", "view_erased: bool"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.replace("view: wgpu::TextureView", "view: &wgpu::TextureView"), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("gpu.reserve_engine_texture", "gpu.realize_without_reservation")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("let published_view = std::mem::replace(&mut surface.view, replacement_view)", "let published_view = surface.view.clone()")),
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
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_target_texture_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_target_view_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_renderer_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_replacement_texture_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_replacement_view_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.replace(
                    "if let Err((fault, admission, value)) = self.stage_claimed_texture(admission, value, allocation_claim)",
                    "if self.stage_claimed_texture(admission, value, allocation_claim).map_err(|(fault, _, _)| fault).is_err()",
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
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("surface.texture = texture", "drop(texture)")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("surface.view = view", "drop(view)")),
        ];
    assert_eq!(mutations.len(), 38);
    for (draw, gpu, glue, engine) in mutations {
        assert!(!retained_raster_contract(&draw, &gpu, &glue, &engine));
    }
    let reservation = "let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?;";
    let first_allocation = "let texture = create_target_texture(gpu.device(), width, height);";
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
    // 🌍️ The retained decoder mounts every GLB scene root under the world frame (`glb_world_frame`,
    // glTF's Y-up → the world's Z-up), which is React's `<group rotation={[π/2, 0, 0]}>` around
    // `GlbInstanceMesh`. The legacy oracle decodes raw glTF space, so it is compared through the
    // same rotation: `(x, y, z)` → `(x, -z, y)`.
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
    assert!(
        matches!(step, RendererAssetProbeStep::Reject("JPEG dimensions exceeded fixed pixel credits")),
        "a refused image is ONE missing asset — a Fault here quarantines the whole surface"
    );
    let (detail, kind, url) = probe.take_rejection().expect("a rejection carries the lane its miss belongs to");
    assert_eq!(detail, "JPEG dimensions exceeded fixed pixel credits");
    assert_eq!(kind, WorldAssetRequestKind::ReferenceImage);
    assert_eq!(url, "reference.jpg", "the url is captured BEFORE begin_close clears it");
    while !probe.close_step() {}
    let RendererAssetFetchOwner::Shared(owner) = probe.take_terminal_owner().unwrap() else { panic!("shared probe") };
    authority.finish(owner).unwrap();
    assert!(authority.terminal_is_empty());
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
fn runtime_dispatch_cursor_preserves_coalesced_then_discrete_order() {
    let pointer = ui_host::PointerMoveSample { pointer: ui_render::PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None }, x: 1.0, y: 2.0, generation: ui_host::InputGeneration(1) };
    let scroll = ui_host::ScrollSample { x: 3.0, y: 4.0, delta_x: 5.0, delta_y: 6.0, generation: ui_host::InputGeneration(2) };
    let discrete = ui_host::DiscreteEvent { event: ui_render::DispatchEvent::KeyDown { key: "A".to_string(), modifiers: ui_render::EventModifiers::default() }, generation: ui_host::InputGeneration(3) };
    let mut events = ui_host::DrainedEvents { pointer_move: Some(pointer), scroll: Some(scroll), ..Default::default() };
    events.discrete[0] = Some(discrete);
    let mut cursor = RuntimeDispatchCursor::new(events);

    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::PointerMove { .. })));
    assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::Scroll { .. })));
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
        modifiers: PointerModifiers::default(),
        wheel: crate::AppWheel::default(),
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

fn presenter_retirement_contract(glue: &str, prepared: &str, gpu: &str, draw: &str, host: &str, winit: &str) -> bool {
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
        && glue.matches(concat!("runtime.presentation_", "witness_for(self.generation.0)")).count() == 1
        && glue.contains("presentation_witness.scene_revision")
        && glue.contains("presentation_witness.input_generation")
        && glue.matches(concat!("let expected = self.presentation_", "authority.current();")).count() == 2
        && glue.contains("begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
        && glue.contains("begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
        && glue.contains("packet.scene_revision() != expected.scene_revision || packet.preview_generation() != expected.input_generation")
        && glue.matches(concat!("self.presentation_authority.", "mark_scene_changed();")).count() == 2
        && glue.contains("runtime_presentation_authority_and_candidate_identity_change_independently")
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
        && draw.contains("mesh_gpu_retirement_preserves_acknowledged_versions")
        && draw.contains("pub fn close_step(&mut self) -> bool")
        && draw.contains("fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner")
        && !table.contains("HashMap")
        && !table.contains(".retain(")
        && gpu.contains("self.mesh_store.evict_mesh_except_step(key, keep_versions)")
        && gpu.contains("self.mesh_store.close_step()")
}

#[test]
fn presenter_ack_retirement_source_mutations_are_denied() {
    assert!(presenter_retirement_contract(LIBRARY_SOURCE, PREPARED_SOURCE, GPU_SOURCE, DRAW_SOURCE, OS_HOST_SOURCE, WINT_APP_SOURCE));
    let mutations = [
        (LIBRARY_SOURCE.replace("AppPresentPhase::Acknowledge", "AppPresentPhase::Fullscreen"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (
            LIBRARY_SOURCE.replace("self.gate.acknowledge_presented(witness)", "drop(witness); self.gate.acknowledge_presented_unchecked()"),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.to_string(),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (LIBRARY_SOURCE.to_string(), PREPARED_SOURCE.replace("pub fn abort_pending", "fn abandon_pending"), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (
            LIBRARY_SOURCE.to_string(),
            PREPARED_SOURCE.replace("last_valid: Option<PreparedRenderPacket>", "last_valid: Option<Arc<PreparedRenderPacket>>"),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.to_string(),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (LIBRARY_SOURCE.replace("previous.retire_step()", "drop(previous)"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (LIBRARY_SOURCE.replace("AppPresentPhase::Stage", "AppPresentPhase::Render"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (LIBRARY_SOURCE.replace("AppPresentPhase::Aborted", "AppPresentPhase::Render"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (
            LIBRARY_SOURCE.to_string(),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.replace("self.mesh_store.evict_mesh_except_step(key, keep_versions)", "self.mesh_store.evict_mesh_step(key)"),
            DRAW_SOURCE.to_string(),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (
            LIBRARY_SOURCE.to_string(),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.replace("MESH_GPU_TABLE_CAPACITY: usize = 256", "MESH_GPU_TABLE_CAPACITY: usize = usize::MAX"),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (
            LIBRARY_SOURCE.to_string(),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.replace("FixedMeshGpuRegistry<GpuMeshBuffers>", "std::collections::HashMap<String, GpuMeshBuffers>"),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (LIBRARY_SOURCE.to_string(), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.replace("pub fn close_step(&mut self) -> bool", "pub fn close_all(&mut self) -> bool"), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (LIBRARY_SOURCE.to_string(), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.replace("presenter.world_owners_terminal_is_empty()", "true"), WINT_APP_SOURCE.to_string()),
        (
            LIBRARY_SOURCE.to_string(),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.to_string(),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.replace(".admit_next_frame(|| frame_build.poll_runtime_and_resubmit", ".poll_runtime_and_resubmit"),
        ),
        (
            LIBRARY_SOURCE.replacen(
                concat!("let expected = self.presentation_", "authority.current();"),
                concat!("let expected = RuntimePresentationWitness { scene_revision: packet.", "scene_revision(), input_generation: packet.", "preview_generation() };"),
                1,
            ),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.to_string(),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (
            LIBRARY_SOURCE.replace(concat!("runtime.presentation_", "witness_for(self.generation.0)"), "Some(RuntimePresentationWitness { scene_revision: self.generation.0, input_generation: self.generation.0 })"),
            PREPARED_SOURCE.to_string(),
            GPU_SOURCE.to_string(),
            DRAW_SOURCE.to_string(),
            OS_HOST_SOURCE.to_string(),
            WINT_APP_SOURCE.to_string(),
        ),
        (LIBRARY_SOURCE.replace(concat!("self.presentation_authority.", "mark_scene_changed();"), ""), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
        (LIBRARY_SOURCE.to_string(), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.replace("self.runtime.observe_presentation_input_generation(build_generation.0);", "")),
    ];
    for (glue, prepared, gpu, draw, host, winit) in mutations {
        assert!(!presenter_retirement_contract(&glue, &prepared, &gpu, &draw, &host, &winit));
    }
}

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
        begin_world3d_dynamic_retirement, finish_world3d_asset, publish_world3d_asset_mesh_lease, reserve_world3d_asset_request, reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response,
        step_world3d_dynamic_retirement, take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_dynamic_retirement_terminal_is_empty, World3dState,
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
        begin_world3d_dynamic_retirement, finish_world3d_asset, publish_world3d_asset_mesh_lease, reserve_world3d_asset_request, reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response,
        step_world3d_dynamic_retirement, take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_dynamic_retirement_terminal_is_empty, World3dState,
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

    let composite = ladder.split("PreparedGpuPresentPhase::EncodeComposite =>").nth(1).expect("the composite phase");
    assert!(composite[..composite.find("PreparedGpuPresentPhase::GlassCommands =>").unwrap_or(composite.len())].contains("composite.view()"), "the scene blit lands offscreen");
    let glass = ladder.split("PreparedGpuPresentPhase::GlassCommands =>").nth(1).expect("the glass phase");
    assert!(glass[..glass.find("PreparedGpuPresentPhase::ForegroundCommands =>").unwrap_or(glass.len())].contains("composite.view()"), "and so does every glass region");
}

/// 🫧 LAW: the content a glass region carries on its face is encoded AFTER the glass pass, never
/// into the scene the glass pass samples.
///
/// 🩸️ The scalar ladder encoded every layer into the scene and then composited the glass regions
/// over it, so the window cap's own `Puzzle 3D` title and its Focus/Close controls were painted and
/// then blurred away by the very region that labels them — the batch renderer has always split the
/// two with `LayerBatchFilter` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
#[test]
fn glass_foreground_scalars_are_encoded_after_the_glass_pass_and_never_into_the_scene() {
    assert!(GPU_SOURCE.contains("fn prepared_draw_scalar_is_glass_foreground"), "the ladder classifies a scalar by its layer's glass ownership");
    assert!(GPU_SOURCE.contains("layer.foreground_of.is_some()"), "using the draw list's own glass-content marker");
    let ladder = GPU_SOURCE.split("pub fn prepared_present_step").nth(1).expect("the prepared present ladder");
    let commands = ladder.split("PreparedGpuPresentPhase::Commands =>").nth(1).expect("the scene command phase");
    let commands = &commands[..commands.find("PreparedGpuPresentPhase::BlurScene =>").unwrap_or(commands.len())];
    assert!(commands.contains("if !owner.is_some_and(|draw| prepared_draw_scalar_is_glass_foreground(draw, draw_cursor))"), "the scene phase skips glass-foreground scalars");
    assert!(commands.contains("PreparedDrawTarget::Scene"), "and everything else goes to the scene");
    let foreground = ladder.split("PreparedGpuPresentPhase::ForegroundCommands =>").nth(1).expect("the glass-foreground phase");
    let foreground = &foreground[..foreground.find("PreparedGpuPresentPhase::Present =>").unwrap_or(foreground.len())];
    assert!(foreground.contains("if owner.is_some_and(|draw| prepared_draw_scalar_is_glass_foreground(draw, draw_cursor))"), "and only they are re-encoded later");
    assert!(foreground.contains("PreparedDrawTarget::Composite"), "onto the composite the glass pass already wrote");
    assert!(GPU_SOURCE.contains("self.foreground_command"), "and its own index is part of the watchdog signature");
}
