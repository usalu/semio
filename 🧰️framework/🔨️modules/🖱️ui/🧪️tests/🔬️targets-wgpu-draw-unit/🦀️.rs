
use super::{
    ClipRegion, DrawList, FixedMeshGpuRegistry, FixedRasterTextureRegistry, MESH_GPU_KEEP_VERSION_CAPACITY, MESH_GPU_TABLE_CAPACITY, MeshGpuEntry, MeshGpuKey, RASTER_TEXTURE_ITEM_BYTE_CAPACITY, RASTER_TEXTURE_KEY_BYTES,
    RASTER_TEXTURE_PROBE_CAPACITY, RASTER_TEXTURE_TABLE_CAPACITY, RasterTextureAdmission, RasterTextureCleanupStep, RasterTextureEntry, RasterTextureKey, RasterTextureReservation, RasterTextureReservationCloseCursor,
    RasterTextureReservationRetirement, RasterTextureStageClaim, RasterTextureUploadCloseCursor, RasterTextureUploadCursor, RasterTextureWitness, RasterTextureWitnessSlot, ScissorRect, WORLD_GLOBALS_SLOT_SIZE, claim_raster_stage_tuple,
    content_stencil_state, ear_clip_polygon, mask_instances, mask_stencil_state, mesh_content_version, raster_texture_bytes, raster_witness_is_stale,
};
use crate::wgpu::geometry::Rect;
use crate::wgpu::kernel_3d_scene::ScenePass3d;
use crate::wgpu::theme::Rgba;

#[test]
fn fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner() {
    let mut registry = FixedMeshGpuRegistry::default();
    for index in 0..MESH_GPU_TABLE_CAPACITY {
        let key = MeshGpuKey::new(&format!("mesh-{index}")).expect("bounded key");
        registry.insert(MeshGpuEntry { key, version: index as u64, value: Box::new(index) }).ok().expect("fixed mesh slot");
    }
    let rejected = Box::new(MESH_GPU_TABLE_CAPACITY);
    let rejected_pointer = (&*rejected) as *const usize;
    let rejected = registry.insert(MeshGpuEntry { key: MeshGpuKey::new("overflow").expect("bounded key"), version: 1, value: rejected }).expect_err("capacity plus one");
    assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
    assert_eq!(*rejected.value, MESH_GPU_TABLE_CAPACITY);
    let first = registry.take(0).expect("first exact owner");
    assert_eq!(*first.value, 0);
    registry.insert(rejected).ok().expect("returned owner retries after one slot retires");
    assert_eq!(registry.len, MESH_GPU_TABLE_CAPACITY);
}

#[test]
fn fixed_mesh_gpu_registry_version_identity_rejects_stale_aba_lookup() {
    let mut registry = FixedMeshGpuRegistry::default();
    registry.insert(MeshGpuEntry { key: MeshGpuKey::new("terrain").expect("bounded key"), version: 7, value: 70u64 }).ok().expect("first generation");
    assert_eq!(registry.get("terrain", 7), Some(&70));
    let old = registry.take(0).expect("old generation owner");
    assert_eq!(old.version, 7);
    registry.insert(MeshGpuEntry { key: MeshGpuKey::new("terrain").expect("bounded key"), version: 8, value: 80u64 }).ok().expect("replacement generation");
    assert!(registry.get("terrain", 7).is_none());
    assert_eq!(registry.get("terrain", 8), Some(&80));
}

fn raster_key_for_start(start: usize, ordinal: usize) -> RasterTextureKey {
    (0usize..1_000_000).map(|candidate| RasterTextureKey::new(&format!("raster-{start}-{ordinal}-{candidate}")).expect("bounded raster key")).find(|key| key.start() == start).expect("deterministic raster key for slot")
}

#[test]
fn fixed_raster_registry_rejects_capacity_plus_one_with_exact_handback() {
    let mut registry = FixedRasterTextureRegistry::default();
    for index in 0..RASTER_TEXTURE_TABLE_CAPACITY {
        let key = raster_key_for_start(index, 0);
        registry.insert(RasterTextureEntry { key, witness: RasterTextureWitness { scene_revision: 1, preview_generation: 1, operation: index as u64 }, bytes: 1, value: Box::new(index) }).ok().expect("fixed raster slot");
    }
    let rejected = Box::new(RASTER_TEXTURE_TABLE_CAPACITY);
    let rejected_pointer = (&*rejected) as *const usize;
    let rejected = match registry.insert(RasterTextureEntry { key: RasterTextureKey::new("overflow").expect("bounded raster key"), witness: RasterTextureWitness { scene_revision: 2, preview_generation: 2, operation: 1 }, bytes: 1, value: rejected })
    {
        Err(rejected) => rejected,
        Ok(_) => panic!("raster capacity plus one was accepted"),
    };
    assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
    assert_eq!(registry.len, RASTER_TEXTURE_TABLE_CAPACITY);
}

#[test]
fn fixed_raster_registry_probe_saturation_and_replacement_preserve_owners() {
    let start = 17;
    let mut registry = FixedRasterTextureRegistry::default();
    for ordinal in 0..RASTER_TEXTURE_PROBE_CAPACITY {
        let key = raster_key_for_start(start, ordinal);
        registry.insert(RasterTextureEntry { key, witness: RasterTextureWitness { scene_revision: 7, preview_generation: ordinal as u64, operation: ordinal as u64 + 1 }, bytes: 1, value: Box::new(ordinal) }).ok().expect("probe slot");
    }
    let rejected_owner = Box::new(99usize);
    let rejected_pointer = (&*rejected_owner) as *const usize;
    let rejected =
        match registry.insert(RasterTextureEntry { key: raster_key_for_start(start, RASTER_TEXTURE_PROBE_CAPACITY), witness: RasterTextureWitness { scene_revision: 8, preview_generation: 0, operation: 1 }, bytes: 1, value: rejected_owner }) {
            Err(rejected) => rejected,
            Ok(_) => panic!("probe capacity plus one was accepted"),
        };
    assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
    let replacement_key = raster_key_for_start(start, 0);
    let previous =
        registry.insert(RasterTextureEntry { key: replacement_key, witness: RasterTextureWitness { scene_revision: 9, preview_generation: 3, operation: 77 }, bytes: 1, value: Box::new(777usize) }).ok().flatten().expect("exact replaced owner");
    assert_eq!(*previous.value, 0);
    let current = registry.get(replacement_key.as_str()).expect("replacement generation");
    assert_eq!((current.witness.scene_revision, current.witness.preview_generation, current.witness.operation, *current.value), (9, 3, 77, 777));
}

#[test]
fn raster_key_and_byte_credits_reject_exact_plus_one() {
    let key = "k".repeat(RASTER_TEXTURE_KEY_BYTES);
    assert!(RasterTextureKey::new(&key).is_ok());
    assert!(RasterTextureKey::new(&(key + "x")).is_err());
    assert_eq!(raster_texture_bytes(2048, 2048), Some(RASTER_TEXTURE_ITEM_BYTE_CAPACITY));
    assert!(raster_texture_bytes(2048, 2049).is_some_and(|bytes| bytes > RASTER_TEXTURE_ITEM_BYTE_CAPACITY));
}

#[test]
fn raster_operation_freshness_is_independent_and_aba_safe() {
    let current = RasterTextureWitness { scene_revision: 9, preview_generation: 4, operation: 12 };
    assert!(raster_witness_is_stale(current, RasterTextureWitness { operation: 11, ..current }));
    assert!(raster_witness_is_stale(current, current));
    assert!(!raster_witness_is_stale(current, RasterTextureWitness { operation: 13, ..current }));
}

#[test]
fn raster_witness_close_retires_exactly_one_scalar_per_grant() {
    let mut slot = RasterTextureWitnessSlot::default();
    slot.set(RasterTextureWitness { scene_revision: 3, preview_generation: 5, operation: 7 });
    assert!(!slot.retire_one());
    assert!(slot.scene_revision.is_none());
    assert!(slot.preview_generation.is_some());
    assert!(!slot.retire_one());
    assert!(slot.preview_generation.is_none());
    assert!(slot.operation.is_some());
    assert!(!slot.retire_one());
    assert!(slot.operation.is_none());
    assert!(slot.retire_one());
}

#[test]
fn raster_reservation_cancel_retires_one_exact_root_or_scalar_per_grant() {
    let reservation = RasterTextureReservation {
        key: RasterTextureKey::new("cancelled").expect("bounded key"),
        witness: RasterTextureWitness { scene_revision: 3, preview_generation: 5, operation: 7 },
        width: 16,
        height: 8,
        bytes: 512,
        staged_index: 11,
        nonce: 13,
    };
    let mut retirement = RasterTextureReservationRetirement::new(reservation);
    assert_eq!(retirement.step(), RasterTextureCleanupStep::Pending { released_roots: 1, released_scalars: 0 });
    for _ in 0..8 {
        assert_eq!(retirement.step(), RasterTextureCleanupStep::Pending { released_roots: 0, released_scalars: 1 });
    }
    assert_eq!(retirement.step(), RasterTextureCleanupStep::Complete);
}

#[test]
fn raster_matching_cancel_retains_both_reservation_and_admission_to_terminal() {
    let witness = RasterTextureWitness { scene_revision: 61, preview_generation: 67, operation: 71 };
    let key = RasterTextureKey::new("matching-cancel").expect("bounded key");
    let reservation = RasterTextureReservation { key, witness, width: 32, height: 8, bytes: 1024, staged_index: 73, nonce: 79 };
    let admission = RasterTextureAdmission { key, witness, width: 32, height: 8, bytes: 1024, staged_index: 73, nonce: 79 };
    let mut close = RasterTextureReservationCloseCursor::cancelled(reservation, admission);
    let mut released_roots = 0;
    let mut released_scalars = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        assert!(steps < 32, "matching cancellation must terminate");
        match close.step() {
            RasterTextureCleanupStep::Pending { released_roots: roots, released_scalars: scalars } => {
                assert!(roots + scalars <= 1);
                released_roots += roots;
                released_scalars += scalars;
            }
            RasterTextureCleanupStep::Blocked(fault) => panic!("unexpected matching cancellation block: {fault}"),
            RasterTextureCleanupStep::Complete => break,
        }
    }
    assert_eq!((released_roots, released_scalars), (2, 16));
    assert!(close.terminal_is_empty());
}

#[test]
fn raster_gpu_allocation_claim_rejects_missing_aba_candidate_and_occupied_slot() {
    fn authorities(nonce: u64) -> (RasterTextureReservation, RasterTextureAdmission, RasterTextureWitness) {
        let witness = RasterTextureWitness { scene_revision: 17, preview_generation: 19, operation: 23 };
        let key = RasterTextureKey::new("claimed").expect("bounded key");
        let reservation = RasterTextureReservation { key, witness, width: 32, height: 16, bytes: 2048, staged_index: 29, nonce };
        let admission = RasterTextureAdmission { key, witness, width: 32, height: 16, bytes: 2048, staged_index: 29, nonce };
        (reservation, admission, witness)
    }

    let (reservation, admission, witness) = authorities(31);
    let claim = claim_raster_stage_tuple(Some(reservation), Some(witness), false, &admission, witness).expect("full tuple");
    assert_eq!((claim.staged_index, claim.staged_nonce), (29, 31));
    assert!(claim_raster_stage_tuple(None, Some(witness), false, &admission, witness).is_err());

    let (reservation, stale_admission, witness) = authorities(37);
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &stale_admission, witness).is_ok());
    let (_, aba_admission, _) = authorities(38);
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &aba_admission, witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.key = RasterTextureKey::new("other-key").expect("bounded key");
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.witness.scene_revision += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, mismatched.witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.witness.preview_generation += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, mismatched.witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.witness.operation += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, mismatched.witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.width += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.height += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.bytes += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
    let (_, mut mismatched, _) = authorities(37);
    mismatched.staged_index += 1;
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), false, &mismatched, witness).is_err());
    assert!(claim_raster_stage_tuple(Some(reservation), Some(RasterTextureWitness { operation: 24, ..witness }), false, &stale_admission, witness).is_err());
    assert!(claim_raster_stage_tuple(Some(reservation), Some(witness), true, &stale_admission, witness).is_err());
}

#[test]
fn raster_interrupted_upload_close_is_truthful_before_first_and_mid_page() {
    for row in [0, 7] {
        let witness = RasterTextureWitness { scene_revision: 41, preview_generation: 43, operation: 47 };
        let key = RasterTextureKey::new("interrupted").expect("bounded key");
        let reservation = RasterTextureReservation { key, witness, width: 64, height: 64, bytes: 16 * 1024, staged_index: 53, nonce: 59 };
        let admission = RasterTextureAdmission { key, witness, width: 64, height: 64, bytes: 16 * 1024, staged_index: 53, nonce: 59 };
        let claim = RasterTextureStageClaim { reservation, candidate: witness, staged_index: 53, staged_nonce: 59 };
        let mut close = RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor { admission: Some(admission), row, texture: None, view: None, bind_group: None, allocation_claim: Some(claim) });
        let mut steps = 0;
        loop {
            steps += 1;
            assert!(steps < 64, "retained upload close must terminate");
            match close.step() {
                RasterTextureCleanupStep::Pending { released_roots, released_scalars } => assert!(released_roots + released_scalars <= 1),
                RasterTextureCleanupStep::Blocked(fault) => panic!("unexpected retained close block: {fault}"),
                RasterTextureCleanupStep::Complete => break,
            }
        }
        assert!(close.terminal_is_empty());
    }
}

#[test]
fn mesh_gpu_retirement_preserves_acknowledged_versions() {
    let key = MeshGpuKey::new("terrain").expect("bounded key");
    let other = MeshGpuKey::new("other").expect("bounded key");
    let mut versions = [0; MESH_GPU_KEEP_VERSION_CAPACITY];
    versions[..2].copy_from_slice(&[8, 9]);
    let selector = super::MeshGpuRetirementSelector::KeyExcept { key, versions, len: 2 };
    assert!(selector.selects(&MeshGpuEntry { key, version: 7, value: () }));
    assert!(!selector.selects(&MeshGpuEntry { key, version: 8, value: () }));
    assert!(!selector.selects(&MeshGpuEntry { key, version: 9, value: () }));
    assert!(!selector.selects(&MeshGpuEntry { key: other, version: 7, value: () }));
}

#[test]
fn scissor_intersects_child() {
    let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
    let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
    let c = a.intersect(&b);
    assert_eq!(c.w, 50);
    assert_eq!(c.h, 50);
}

#[test]
fn scissor_covers_fractional_rect_edges_without_pixel_seams() {
    assert_eq!(ScissorRect::from_rect(Rect::new(10.25, 20.75, 30.5, 40.5), 100.0), ScissorRect { x: 10, y: 20, w: 31, h: 42 });
}

//#region SilhouetteClipTests

#[test]
fn clip_region_preserves_disjoint_cutouts_and_intersects_scissor() {
    let clip = ClipRegion::from_rects(&[Rect::new(0.0, 0.0, 40.0, 20.0), Rect::new(80.0, 0.0, 20.0, 20.0), Rect::new(0.0, 20.0, 100.0, 80.0)], 100.0);
    let scissors = clip.effective_scissors(Some(ScissorRect { x: 10, y: 0, w: 80, h: 100 }), 100.0, 100.0);
    assert_eq!(scissors, vec![ScissorRect { x: 10, y: 0, w: 30, h: 20 }, ScissorRect { x: 80, y: 0, w: 10, h: 20 }, ScissorRect { x: 10, y: 20, w: 80, h: 80 }]);
}

#[test]
fn draw_list_nests_and_restores_clip_regions() {
    let mut draw = DrawList::default();
    draw.begin_silhouette_clip(&[Rect::new(0.0, 0.0, 100.0, 100.0)]);
    draw.begin_silhouette_clip(&[Rect::new(25.0, 25.0, 100.0, 100.0)]);
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 25, y: 25, w: 75, h: 75 }].as_slice()));
    draw.end_silhouette_clip();
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 0, y: 0, w: 100, h: 100 }].as_slice()));
    draw.end_silhouette_clip();
    assert!(draw.layers.last().is_some_and(|layer| layer.clip.is_none()));
}

#[test]
fn glass_foreground_inherits_active_silhouette_clip() {
    let mut draw = DrawList::default();
    draw.begin_silhouette_clip(&[Rect::new(0.0, 0.0, 40.0, 20.0), Rect::new(0.0, 20.0, 100.0, 80.0)]);
    let glass = draw.push_glass([0.0, 0.0, 40.0, 20.0], 0.0, crate::wgpu::theme::Theme::default().glass(crate::wgpu::theme::Level::Window));
    draw.begin_glass_content(glass);
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.len()), Some(2));
}

#[test]
fn silhouette_stencil_states_write_masks_then_require_equality() {
    let mask = mask_stencil_state();
    assert_eq!(mask.front.compare, wgpu::CompareFunction::Always);
    assert_eq!(mask.front.pass_op, wgpu::StencilOperation::Replace);
    assert_eq!(mask.write_mask, 0xff);
    let content = content_stencil_state();
    assert_eq!(content.front.compare, wgpu::CompareFunction::Equal);
    assert_eq!(content.front.pass_op, wgpu::StencilOperation::Keep);
    assert_eq!(content.write_mask, 0x00);
}

#[test]
fn silhouette_mask_reset_is_bounded_to_previous_and_current_unions() {
    let previous = Some(ScissorRect { x: 10, y: 10, w: 30, h: 20 });
    let clip = ClipRegion { scissors: vec![ScissorRect { x: 80, y: 15, w: 20, h: 25 }] };
    let (instances, current) = mask_instances(None, Some(&clip), previous, 500.0, 400.0);
    assert_eq!(instances[0].rect, [10.0, 10.0, 90.0, 30.0]);
    assert_eq!(instances[1].rect, [80.0, 15.0, 20.0, 25.0]);
    assert_eq!(current, Some(ScissorRect { x: 80, y: 15, w: 20, h: 25 }));
}

#[test]
fn empty_silhouette_clip_writes_no_visible_stencil_region() {
    let empty = ClipRegion { scissors: Vec::new() };
    let (instances, current) = mask_instances(None, Some(&empty), None, 500.0, 400.0);
    assert!(instances.is_empty(), "a cleared pass needs neither a reset nor a reference-one mask draw");
    assert_eq!(current, None);
}

//#endregion SilhouetteClipTests

#[test]
fn scissor_from_rect_uses_top_left_origin() {
    let scissor = ScissorRect::from_rect(Rect::new(10.0, 20.0, 80.0, 60.0), 720.0);
    assert_eq!(scissor.x, 10);
    assert_eq!(scissor.y, 20);
    assert_eq!(scissor.w, 80);
    assert_eq!(scissor.h, 60);
}

#[test]
fn draw_list_push_scissor_splits_layers() {
    let mut draw = DrawList::default();
    draw.set_screen_height(200.0);
    draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
    draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
    draw.pop_scissor();
    assert!(draw.layers.len() >= 3);
}

#[test]
fn ear_clip_produces_triangles() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let tris = ear_clip_polygon(&square);
    assert!(tris.len() >= 3);
}

#[test]
fn world_globals_slot_size_is_aligned() {
    const { assert!(WORLD_GLOBALS_SLOT_SIZE >= 80) };
    assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
}

#[test]
fn scene_pass_records_layer_watermarks() {
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 10.0, 10.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.push_solid([1.0, 1.0, 8.0, 8.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
    draw.push_scene_pass(ScenePass3d { viewport: [0.0, 0.0, 100.0, 100.0], view_proj: [0.0; 16], light_dir: [0.0, 0.0, 1.0], ..Default::default() });
    draw.push_line(0.0, 0.0, 1.0, 1.0, Rgba::new(0.0, 0.0, 1.0, 1.0), 1.0);
    let pass = &draw.scene_passes[0];
    assert_eq!(pass.layer_index, 0);
    assert_eq!(pass.ui_watermark, 2);
    assert_eq!(pass.vector_watermark, 0);
    assert_eq!(draw.layers[0].ui_instances.len(), 2);
    assert_eq!(draw.layers[0].vector_vertices.len(), 6);
}

#[test]
fn mesh_instances_without_lines_are_valid_world_pass() {
    use crate::wgpu::kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};

    let pass = ScenePass3d {
        viewport: [0.0, 0.0, 320.0, 240.0],
        view_proj: [0.0; 16],
        light_dir: [0.4, 0.6, 0.8],
        draws: vec![SceneDraw3d {
            mesh_key: "box".into(),
            mesh_version: 1,
            instances: vec![Instance3d { id: "preview".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [0.7, 0.7, 0.75, 1.0], selected: false, hovered: false }],
        }],
        ..Default::default()
    };
    assert!(!pass.draws[0].instances.is_empty());
    assert!(pass.line_draws.is_empty());
}

#[test]
fn mesh_content_version_changes_with_indices() {
    let v0 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
    let v1 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
    assert_ne!(v0, v1);
}

#[test]
fn overlay_layers_collected_separately_from_backdrop_ui() {
    use super::{LayerBatchFilter, build_layer_batches, build_overlay_layer_batches};
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
    draw.push_glyph_overlay([10.0, 10.0, 20.0, 12.0], Rgba::new(1.0, 1.0, 1.0, 1.0), [0.0, 0.0, 0.1, 0.1]);
    draw.push_line_overlay(0.0, 0.0, 50.0, 50.0, Rgba::new(1.0, 0.0, 0.0, 1.0), 1.0);
    let (backdrop_ui, _, _) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
    let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(&draw, LayerBatchFilter::Backdrop);
    assert_eq!(backdrop_ui.len(), 1);
    assert_eq!(overlay_ui.len(), 1);
    assert_eq!(overlay_vec.len(), 6);
    assert_eq!(overlay_batches.len(), 1);
    assert_eq!(draw.layers[overlay_batches[0].layer_index].overlay_ui_instances.len(), 1);
}

#[test]
fn glass_content_layers_tagged_with_foreground_of() {
    use super::Theme;
    use crate::wgpu::theme::Level;
    let theme = Theme::default();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.2, 0.2, 0.2, 1.0));
    let glass = draw.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, theme.glass(Level::Panel));
    assert_eq!(glass, 0);
    draw.begin_glass_content(glass);
    draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.end_glass_content();
    let backdrop = draw.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
    let foreground = draw.layers.iter().filter(|layer| layer.foreground_of == Some(glass)).count();
    assert_eq!(backdrop, 2);
    assert_eq!(foreground, 1);
    assert_eq!(draw.layers[1].ui_instances.len(), 1);
}

#[test]
fn glass_foreground_layers_excluded_from_backdrop_batches() {
    use super::{LayerBatchFilter, Theme, build_layer_batches};
    use crate::wgpu::theme::Level;
    let theme = Theme::default();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
    let glass = draw.push_glass([20.0, 20.0, 160.0, 160.0], 8.0, theme.glass(Level::Panel));
    draw.begin_glass_content(glass);
    draw.push_solid([20.0, 20.0, 160.0, 160.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.end_glass_content();
    let (backdrop_ui, _, backdrop_batches) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
    let (foreground_ui, _, foreground_batches) = build_layer_batches(&draw, LayerBatchFilter::Foreground);
    assert_eq!(backdrop_ui.len(), 1);
    assert_eq!(foreground_ui.len(), 1);
    assert_eq!(backdrop_batches.len(), 1);
    assert_eq!(foreground_batches.len(), 1);
    assert!(draw.layers[backdrop_batches[0].layer_index].foreground_of.is_none());
    assert_eq!(draw.layers[foreground_batches[0].layer_index].foreground_of, Some(glass));
}

#[test]
fn glass_scissor_inherits_foreground_tag() {
    use super::Theme;
    use crate::wgpu::theme::Level;
    let theme = Theme::default();
    let mut draw = DrawList::default();
    let glass = draw.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, theme.glass(Level::Panel));
    draw.begin_glass_content(glass);
    draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
    draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
    draw.pop_scissor();
    draw.end_glass_content();
    let scissor_layer = draw.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
    assert_eq!(scissor_layer.foreground_of, Some(glass));
}

/// 🪜️ `Theme::glass` must be formula-derived off `Level::index` (never a per-tier lookup
/// table): alpha/blur both monotone across all 6 levels. There is deliberately no separate
/// "chrome" variant — a level's attached chrome (title caps, ribbons, tab bars, rails) always
/// renders the exact same `glass(level)` as its body, so one level never shows two appearances.
#[test]
fn glass_alpha_and_blur_are_formula_derived_per_level() {
    use super::Theme;
    use crate::wgpu::theme::Level;
    let theme = Theme::default();
    let ordered = [Level::Base, Level::Window, Level::Pane, Level::Panel, Level::Dialog, Level::Menu];
    for (k, level) in ordered.iter().enumerate() {
        assert_eq!(level.index(), k);
        let style = theme.glass(*level);
        assert!((style.alpha - (1.0 - k as f32 * ui_styling::levels::GLASS_ALPHA_STEP as f32)).abs() < 1e-6);
        assert!((style.blur_px - k as f32 * ui_styling::levels::GLASS_BLUR_STEP_PX as f32).abs() < 1e-6);
        assert_eq!(style.tint, theme.level_bg[k]);
    }
    assert!(theme.glass(Level::Base).alpha > theme.glass(Level::Menu).alpha);
    assert!(theme.glass(Level::Base).blur_px < theme.glass(Level::Menu).blur_px);
    assert_eq!(theme.surface(Level::Panel), theme.level_bg[Level::Panel.index()]);
}
