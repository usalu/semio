use super::*;
use crate::geometry::Rect;
use crate::kernel_3d_scene::ScenePass3d;
use crate::theme::Rgba;
fn raster_identity(seed: u64) -> crate::prepared::RasterContentIdentity {
    crate::prepared::RasterContentIdentity::revision(1, 1, [seed; 6]).expect("bounded raster identity")
}

#[test]
fn scene_raster_profiles_keep_the_verified_srgb_storage_contract() {
    use crate::raster_ownership::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterMeshSeal, SceneRasterPool, SceneRasterProfile, SceneRasterWriteMode};

    for (revision, profile, mesh) in
        [(1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None), (2, SceneRasterProfile::ReferenceCanvasSrgb, None), (3, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 1 }))]
    {
        let pool = SceneRasterPool::new();
        let descriptor = SceneRasterDescriptor { width: 1, height: 1, source_digest: [revision, revision], source_revision: revision, profile, mesh };
        let SceneRasterBegin::Writer(writer) = pool.begin(descriptor, revision, SceneRasterWriteMode::Streamed) else { panic!("profile writer") };
        let writer = pool.push(writer, &[127, 189, 214, 255]).expect("profile pixels");
        let lease = pool.seal(writer).expect("profile seal");
        let pixels = RasterUploadPixels::Scene(&lease);
        assert_eq!(pixels.texture_format(), gpu_device::TextureFormat::Rgba8UnormSrgb);
    }
}

fn raster_key_for_start(start: usize, ordinal: usize) -> RasterTextureKey {
    (0usize..1_000_000).map(|candidate| RasterTextureKey::new(&format!("raster-{start}-{ordinal}-{candidate}")).expect("bounded raster key")).find(|key| key.start() == start).expect("deterministic raster key for slot")
}

#[test]
fn fixed_raster_registry_rejects_capacity_plus_one_with_exact_handback() {
    let mut registry = FixedRasterTextureRegistry::default();
    for index in 0..RASTER_TEXTURE_TABLE_CAPACITY {
        let key = raster_key_for_start(index, 0);
        registry
            .insert(RasterTextureEntry {
                key,
                witness: RasterTextureWitness { scene_revision: 1, preview_generation: 1, operation: index as u64 },
                content_identity: raster_identity(index as u64),
                bytes: 1,
                cpu_release: None,
                gpu_resident: None,
                value: Box::new(index),
            })
            .ok()
            .expect("fixed raster slot");
    }
    let rejected = Box::new(RASTER_TEXTURE_TABLE_CAPACITY);
    let rejected_pointer = (&*rejected) as *const usize;
    let rejected = match registry.insert(RasterTextureEntry {
        key: RasterTextureKey::new("overflow").expect("bounded raster key"),
        witness: RasterTextureWitness { scene_revision: 2, preview_generation: 2, operation: 1 },
        content_identity: raster_identity(257),
        bytes: 1,
        cpu_release: None,
        gpu_resident: None,
        value: rejected,
    }) {
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
        registry
            .insert(RasterTextureEntry {
                key,
                witness: RasterTextureWitness { scene_revision: 7, preview_generation: ordinal as u64, operation: ordinal as u64 + 1 },
                content_identity: raster_identity(ordinal as u64),
                bytes: 1,
                cpu_release: None,
                gpu_resident: None,
                value: Box::new(ordinal),
            })
            .ok()
            .expect("probe slot");
    }
    let rejected_owner = Box::new(99usize);
    let rejected_pointer = (&*rejected_owner) as *const usize;
    let rejected = match registry.insert(RasterTextureEntry {
        key: raster_key_for_start(start, RASTER_TEXTURE_PROBE_CAPACITY),
        witness: RasterTextureWitness { scene_revision: 8, preview_generation: 0, operation: 1 },
        content_identity: raster_identity(99),
        bytes: 1,
        cpu_release: None,
        gpu_resident: None,
        value: rejected_owner,
    }) {
        Err(rejected) => rejected,
        Ok(_) => panic!("probe capacity plus one was accepted"),
    };
    assert_eq!((&*rejected.value) as *const usize, rejected_pointer);
    let replacement_key = raster_key_for_start(start, 0);
    let previous = registry
        .insert(RasterTextureEntry {
            key: replacement_key,
            witness: RasterTextureWitness { scene_revision: 9, preview_generation: 3, operation: 77 },
            content_identity: raster_identity(777),
            bytes: 1,
            cpu_release: None,
            gpu_resident: None,
            value: Box::new(777usize),
        })
        .ok()
        .flatten()
        .expect("exact replaced owner");
    assert_eq!(*previous.value, 0);
    let current = registry.get(replacement_key.as_str()).expect("replacement generation");
    assert_eq!((current.witness.scene_revision, current.witness.preview_generation, current.witness.operation, *current.value), (9, 3, 77, 777));
}

#[test]
fn raster_key_and_byte_credits_reject_exact_plus_one() {
    let key = "k".repeat(RASTER_TEXTURE_KEY_BYTES);
    assert!(RasterTextureKey::new(&key).is_ok());
    assert!(RasterTextureKey::new(&(key + "x")).is_err());
    assert_eq!(raster_texture_bytes(4096, 4096), Some(RASTER_TEXTURE_ITEM_BYTE_CAPACITY));
    assert!(raster_texture_bytes(4096, 4097).is_some_and(|bytes| bytes > RASTER_TEXTURE_ITEM_BYTE_CAPACITY));
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
        content_identity: raster_identity(7),
        width: 16,
        height: 8,
        bytes: 512,
        staged_index: 11,
        nonce: 13,
    };
    let mut retirement = RasterTextureReservationRetirement::new(reservation);
    assert_eq!(retirement.step(), RasterTextureCleanupStep::Pending { released_roots: 1, released_scalars: 0 });
    for _ in 0..9 {
        assert_eq!(retirement.step(), RasterTextureCleanupStep::Pending { released_roots: 0, released_scalars: 1 });
    }
    assert_eq!(retirement.step(), RasterTextureCleanupStep::Complete);
}

#[test]
fn raster_matching_cancel_retains_both_reservation_and_admission_to_terminal() {
    let witness = RasterTextureWitness { scene_revision: 61, preview_generation: 67, operation: 71 };
    let key = RasterTextureKey::new("matching-cancel").expect("bounded key");
    let content_identity = raster_identity(71);
    let reservation = RasterTextureReservation { key, witness, content_identity, width: 32, height: 8, bytes: 1024, staged_index: 73, nonce: 79 };
    let admission = RasterTextureAdmission { key, witness, content_identity, width: 32, height: 8, bytes: 1024, staged_index: 73, nonce: 79 };
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
    assert_eq!((released_roots, released_scalars), (2, 18));
    assert!(close.terminal_is_empty());
}

#[test]
fn raster_gpu_allocation_claim_rejects_missing_aba_candidate_and_occupied_slot() {
    fn authorities(nonce: u64) -> (RasterTextureReservation, RasterTextureAdmission, RasterTextureWitness) {
        let witness = RasterTextureWitness { scene_revision: 17, preview_generation: 19, operation: 23 };
        let key = RasterTextureKey::new("claimed").expect("bounded key");
        let content_identity = raster_identity(nonce);
        let reservation = RasterTextureReservation { key, witness, content_identity, width: 32, height: 16, bytes: 2048, staged_index: 29, nonce };
        let admission = RasterTextureAdmission { key, witness, content_identity, width: 32, height: 16, bytes: 2048, staged_index: 29, nonce };
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
        let content_identity = raster_identity(59);
        let reservation = RasterTextureReservation { key, witness, content_identity, width: 64, height: 64, bytes: 16 * 1024, staged_index: 53, nonce: 59 };
        let admission = RasterTextureAdmission { key, witness, content_identity, width: 64, height: 64, bytes: 16 * 1024, staged_index: 53, nonce: 59 };
        let claim = RasterTextureStageClaim { reservation, candidate: witness, staged_index: 53, staged_nonce: 59 };
        let mut close = RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor {mip_levels:1,mip:1,mip_row:0, admission: Some(admission), row, texture: None, view: None, bind_group: None, allocation_claim: Some(claim), cpu_release: None });
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
