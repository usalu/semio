/// 🖼️ Raster contract, upload cache and asset probe: the retained raster contract, the fixed generation witnessed raster upload cache and the renderer asset probe over chunk boundaries.

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
    let engine_reservation = "self.reserve_engine_texture(key,width,height,1,identity,candidate,expected)";
    let engine_reservation_index = engine.find(engine_reservation).unwrap_or(usize::MAX);
    let first_engine_allocation = ["device.create_texture", ".create_view", "CanvasRasterSession::new"].iter().filter_map(|marker| engine.find(marker)).min().unwrap_or(0);
    let upload_stage = &draw[draw.find("pub fn ensure_raster_step").unwrap_or(draw.len())..draw.find("pub fn get(&self, key: &str) -> Option<&RasterTexture>").unwrap_or(draw.len())];
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
        && draw.contains("view: Option<gpu_device::TextureView>")
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
        && gpu.contains("pub(crate) fn stage_engine_texture")
        && gpu.contains("view: gpu_device::TextureView")
        && gpu.contains(".stage_gpu_bind_group")
        && gpu.contains("Result<(), RasterTextureStageFault>")
        && gpu.contains("pixels.content_identity()")
        && begin < present_step
        && engine.matches(engine_reservation).count() == 1
        && engine_reservation_index < first_engine_allocation
        && guarded_allocations(engine, "self.validate_engine_target_texture_allocation(admission,expected)", "device.create_texture", 1)
        && guarded_allocations(engine, "self.validate_engine_target_view_allocation(admission,expected)", ".create_view", 1)
        && guarded_allocations(&engine[engine.find("EngineRasterBuildPhase::Renderer=>").unwrap_or(engine.len())..engine.find("EngineRasterBuildPhase::Render=>").unwrap_or(engine.len())], "self.validate_engine_renderer_allocation(admission,expected)", "CanvasRasterSession::new", 1)
        && engine.contains("gpu.raster_content_is_reusable(key, identity, candidate_generation, expected)")
        && engine.matches("self.roots.device.create_texture(&wgpu::TextureDescriptor").count() == 1
        && engine.contains("Err(RasterTextureStageFault::Returned{fault,admission,texture,view})=>{")
        && engine.contains("build.admission=Some(admission);")
        && engine.contains("build.texture=Some(texture);")
        && engine.contains("build.view=Some(view);")
        && engine.contains("Err(RasterTextureStageFault::Retained(fault))=>")
        && engine.contains("Err(RasterTextureCancelFault::Returned{fault,admission})=>{build.admission=Some(admission);return Err(fault.into())}")
        && engine.contains("Err(RasterTextureCancelFault::Retained(fault))=>return Err(fault.into())")
        && engine.contains("EngineRasterBuildPhase::RetireRenderer")
        && engine.contains("if build.renderer.take().is_none()")
        && engine.contains("Arc::ptr_eq(roots,&self.roots)")
        && engine.contains("roots:Option<Arc<GpuSessionRoots>>")
        && engine.contains("build.gpu=Some(owner);build.begin_close();return Err(fault);")
        && !engine.contains("gpu.device()")
        && !engine.contains("gpu.queue()")
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
    assert!(retained_raster_contract(DRAW_SOURCE(), GPU_SOURCE(), LIBRARY_SOURCE(), ENGINE_CANVAS_SOURCE()));
    let mutations = [
            (DRAW_SOURCE().replace("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256", "pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 257"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256", "pub const RASTER_TEXTURE_KEY_BYTES: usize = 255"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (
                DRAW_SOURCE().replace("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 64 * 1024 * 1024", "pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = usize::MAX"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (DRAW_SOURCE().replace("pub struct RasterTextureAdmission", "pub struct ErasedRasterTextureAdmission"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("current.operation >= candidate.operation", "current.operation > candidate.operation"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("view: Option<gpu_device::TextureView>", "view_erased: bool"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().replace("view: gpu_device::TextureView", "view: &gpu_device::TextureView"), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().replace("self.reserve_engine_texture", "self.realize_without_reservation")),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().to_string(), LIBRARY_SOURCE().replace("raster_operation_authority: RuntimeRasterOperationAuthority", "raster_operation_authority_erased: bool"), ENGINE_CANVAS_SOURCE().to_string()),
            (
                DRAW_SOURCE().to_string(),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().replace(
                    "let operation = loop {",
                    concat!("let operation = self.0.next_operation.", "fetch_add(1, Ordering::AcqRel); if operation == 0 { return Err(\"raster operation generation exhausted\"); } loop {"),
                ),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace(
                    "let reservation = self.reservation.take().expect(\"matching raster reservation\");",
                    concat!("self.reservation =", " None; let reservation = RasterTextureReservation { key: admission.key, witness: admission.witness, width: admission.width, height: admission.height, bytes: admission.bytes, staged_index: admission.staged_index, nonce: admission.nonce };"),
                ),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("self.upload_close = Some(RasterTextureUploadCloseCursor::new(upload));", "self.upload = None;"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("fn claim_stage_before_gpu_allocation", "fn claim_stage_after_gpu_allocation"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().to_string(),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().replace("raster_operation_authority.begin(expected.scene_revision, expected.input_generation)", "raster_operation_authority.begin(packet.scene_revision(), packet.preview_generation())"),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().to_string(), LIBRARY_SOURCE().replace("raster_operation_authority.matches(raster_witness)", "true"), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().to_string(), LIBRARY_SOURCE().replace("self.gpu.close_raster_table_step()", "true"), ENGINE_CANVAS_SOURCE().to_string()),
            (
                DRAW_SOURCE().replace("self.claim_texture_allocation(admission, expected)", "self.allocate_texture_without_full_claim(admission, expected)"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("self.claim_view_allocation(admission, expected)", "self.allocate_view_without_full_claim(admission, expected)"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("self.claim_bind_group_allocation(admission, expected)", "self.allocate_bind_group_without_full_claim(admission, expected)"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("self.claim_bind_group_allocation(&admission, expected)", "self.allocate_bind_group_without_full_claim(&admission, expected)"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().to_string(),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().replace("self.validate_engine_target_texture_allocation(admission,expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE().to_string(),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().replace("self.validate_engine_target_view_allocation(admission,expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE().to_string(),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().replace("self.validate_engine_renderer_allocation(admission,expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE().replace(
                    "if let Err((fault, admission, value, cpu_release)) = self.stage_claimed_texture(admission, value, allocation_claim, cpu_release)",
                    "if self.stage_claimed_texture(admission, value, allocation_claim, cpu_release).map_err(|(fault, _, _, _)| fault).is_err()",
                ),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor", "self.upload = None; Some(RasterTextureUploadCursor"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (DRAW_SOURCE().replace("texture: Some(value.texture)", "texture: None"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("view: Some(value.view)", "view: None"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("bind_group: Some(value.bind_group)", "bind_group: None"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("if !reservation.matches(admission)", "if false"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("if candidate != Some(expected)", "if false"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("if staged_occupied", "if false"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (
                DRAW_SOURCE().replace("staged_index: admission.staged_index, staged_nonce: admission.nonce", "staged_index: admission.staged_index, staged_nonce: 0"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (
                DRAW_SOURCE().replace("Returned {", "Erased {"),
                GPU_SOURCE().to_string(),
                LIBRARY_SOURCE().to_string(),
                ENGINE_CANVAS_SOURCE().to_string(),
            ),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().replace("build.texture=Some(texture);", "drop(texture);")),
            (DRAW_SOURCE().to_string(), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().replace("build.view=Some(view);", "drop(view);")),
            (DRAW_SOURCE().replace("Self::Scene(lease) => lease.transfer_bytes()", "Self::Scene(_) => usize::MAX"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("let transfer_bytes = pixels.transfer_bytes();", "let transfer_bytes = usize::MAX;"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("row_bytes > transfer_bytes", "false"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
            (DRAW_SOURCE().replace("transfer_bytes / row_bytes", "usize::MAX"), GPU_SOURCE().to_string(), LIBRARY_SOURCE().to_string(), ENGINE_CANVAS_SOURCE().to_string()),
        ];
    assert_eq!(mutations.len(), 39);
    for (draw, gpu, glue, engine) in mutations {
        assert!(!retained_raster_contract(&draw, &gpu, &glue, &engine));
    }
    let reservation = "let admission=self.reserve_engine_texture(key,width,height,1,identity,candidate,expected)?;";
    let first_allocation = "build.texture=Some(self.roots.device.create_texture";
    let reservation_after_first_allocation = ENGINE_CANVAS_SOURCE().replacen(reservation, "", 1).replacen(first_allocation, &format!("{first_allocation}\n            {reservation}"), 1);
    assert!(!retained_raster_contract(DRAW_SOURCE(), GPU_SOURCE(), LIBRARY_SOURCE(), &reservation_after_first_allocation));
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
    let schema = lease.schema().expect("the outline and vertex streams seal together");
    assert_eq!((schema.vertices, schema.indices, schema.edges, schema.edge_ids), (4, 6, 4, 0));
    let outline_points = (0..schema.edges).flat_map(|index| lease.edge(index).expect("sealed outline edge")).collect::<Vec<_>>();
    for point in [[1.0, -3.0, 2.0], [2.001, -3.0, 2.0], [1.0, -3.0, 3.001], [2.001, -3.0, 3.001]] {
        assert_eq!(outline_points.iter().filter(|actual| actual.iter().zip(point).all(|(actual, expected)| (*actual - expected).abs() < 0.00001)).count(), 2, "the primitive-local outline scale precedes node and world transforms");
    }
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
    let legacy = semio_framework::mesh_io::binary::mesh_from_glb(&valid).expect("legacy glTF oracle");
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
