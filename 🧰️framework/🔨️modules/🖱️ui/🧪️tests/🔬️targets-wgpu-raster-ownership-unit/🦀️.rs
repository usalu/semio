use super::*;

/// 🧾️ Preserves actual raster and pool backing through every independently refused full grant.
#[cfg(all(feature = "wgpu-engine", not(target_arch = "wasm32")))]
#[test]
fn original_raster_lease_full_grant_closes_final_pool_with_physical_receipts() {
    use crate::wgpu::host::physical_job_close_tests::measured;
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../♻️raster-lease-close/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["rasters"].as_array().unwrap(){for copy in fixture["copyGrants"].as_array().unwrap(){
        let bytes=row["byteLength"].as_u64().unwrap() as usize;
        let capacity=row["capacityBytes"].as_u64().unwrap() as usize;
        let (pool,births,frees)=measured(||SceneRasterPool::try_new(SceneRasterPoolLimits{item_bytes:capacity,pool_bytes:capacity*2,slot_capacity:2,lease_capacity_per_slot:2,transfer_bytes:capacity,retire_bytes:8,gpu_resident_bytes:96}).unwrap());
        assert_eq!(frees,0);
        let mut source_births=births;
        let (pixels,births,frees)=measured(||{let mut pixels=Vec::with_capacity(capacity);pixels.resize(bytes,row["fill"].as_u64().unwrap() as u8);pixels});
        assert_eq!((births,frees),(capacity,0));source_births+=births;
        let pointer=pixels.as_ptr();
        let raster=descriptor_sized(871,row["width"].as_u64().unwrap() as u32,row["height"].as_u64().unwrap() as u32);
        for admitted_capacity in [bytes - 1, capacity + 1] {
            let (denied,births,frees)=measured(||pool.begin(raster,872,SceneRasterWriteMode::Moved { capacity_bytes: admitted_capacity }));
            assert!(matches!(denied,SceneRasterBegin::Refused(_)));assert_eq!((births,frees),(0,0));
            assert_eq!(pixels.as_ptr(),pointer);assert_eq!(pixels.len(),bytes);assert_eq!(pixels.capacity(),capacity);assert_eq!(pool.reserved_bytes(),0);
        }
        let (begin,births,frees)=measured(||pool.begin(raster,872,SceneRasterWriteMode::Moved { capacity_bytes: capacity }));assert_eq!((births,frees),(0,0));
        assert_eq!(pool.reserved_bytes(),capacity);
        let SceneRasterBegin::Writer(writer)=begin else{panic!("original raster source admission")};
        let (published,births,frees)=measured(||pool.seal_moved(writer,pixels));assert_eq!((births,frees),(0,0));let mut lease=published.unwrap();
        assert_eq!(lease.with_rows(0,bytes,|pixels,_|pixels.as_ptr()).unwrap(),pointer);
        let (_,births,frees)=measured(||drop(pool));assert_eq!((births,frees),(0,0));
        let copy=copy.as_u64().unwrap() as usize;
        let zero=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
        for denied in [zero,RetainedCloneGrant{maximum_items:1,maximum_depth:0,..zero}]{
            let (result,births,frees)=measured(||lease.close_step(denied));
            assert!(matches!(result.unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));assert_eq!((births,frees),(0,0));
            assert_eq!(lease.with_rows(0,bytes,|pixels,_|pixels.as_ptr()).unwrap(),pointer);
        }
        let mut retained_births=0;let mut released=0;let mut turns=0;let mut refusals=0;
        loop{
            turns+=1;assert!(turns<32768);
            lease.next_close_copy_byte_demand().unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:lease.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:lease.next_close_release_byte_demand().unwrap(),maximum_depth:lease.next_close_depth_demand().unwrap()};
            for denied in [grant.maximum_capacity_bytes.checked_sub(1).map(|bytes|RetainedCloneGrant{maximum_capacity_bytes:bytes,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|bytes|RetainedCloneGrant{maximum_release_bytes:bytes,..grant})].into_iter().flatten(){
                let (result,births,frees)=measured(||lease.close_step(denied));
                assert!(matches!(result.unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));assert_eq!((births,frees),(0,0));refusals+=1;
            }
            let (result,births,frees)=measured(||lease.close_step(grant));let result=result.unwrap();let progress=result.progress();
            assert!(progress.fits(grant));assert_eq!((births,frees),(progress.retained_capacity_bytes,progress.released_bytes));retained_births+=births;released+=frees;
            if matches!(result,RetainedCloneStep::Complete(_)){break}
        }
        assert!(lease.terminal_is_empty());assert_eq!(released,source_births+retained_births);assert!(refusals>0);
        let (_,births,frees)=measured(||drop(lease));assert_eq!((births,frees),(0,0));
        eprintln!("[DEBUG] original raster final lease pixels={bytes} capacity={capacity} copy={copy} births={retained_births} released={released} source={source_births} refused={refusals} turns={turns} terminalDropFree=0");
    }}
}

fn limits(lease_capacity_per_slot: usize) -> SceneRasterPoolLimits {
    SceneRasterPoolLimits { item_bytes: 32, pool_bytes: 64, slot_capacity: 2, lease_capacity_per_slot, transfer_bytes: 8, retire_bytes: 8, gpu_resident_bytes: 96 }
}

fn descriptor(source_revision: u64) -> SceneRasterDescriptor {
    descriptor_sized(source_revision, 2, 2)
}

fn descriptor_sized(source_revision: u64, width: u32, height: u32) -> SceneRasterDescriptor {
    SceneRasterDescriptor { width, height, source_digest: [source_revision, source_revision.rotate_left(17)], source_revision, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None }
}

fn publish(pool: &SceneRasterPool, raster: SceneRasterDescriptor, owner: u64, pixels: &[u8]) -> SceneRasterLease {
    let SceneRasterBegin::Writer(mut writer) = pool.begin(raster, owner, SceneRasterWriteMode::Streamed) else { panic!("writer") };
    for chunk in pixels.chunks(8) {
        writer = pool.push(writer, chunk).expect("push");
    }
    pool.seal(writer).expect("seal")
}

fn publish_after_bounded_retirement(pool: &SceneRasterPool, raster: SceneRasterDescriptor, owner: u64, pixels: &[u8]) -> SceneRasterLease {
    for _ in 0..=2 {
        match pool.begin(raster, owner, SceneRasterWriteMode::Streamed) {
            SceneRasterBegin::Writer(mut writer) => {
                for chunk in pixels.chunks(8) {
                    writer = pool.push(writer, chunk).expect("push");
                }
                return pool.seal(writer).expect("seal");
            }
            SceneRasterBegin::Backpressure("scene raster retirement is pending") => assert!(!pool.maintenance_step()),
            _ => panic!("scene raster must make bounded progress"),
        }
    }
    panic!("scene raster did not progress after one bounded retirement");
}

#[test]
fn natural_scene_raster_is_pooled_once_and_read_through_an_immutable_scope() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let pixels: Vec<_> = (0..16).collect();
    let first = publish(&pool, descriptor(1), 7, &pixels);
    let SceneRasterBegin::Reused(second) = pool.begin(descriptor(1), 8, SceneRasterWriteMode::Streamed) else { panic!("reuse") };
    assert_eq!(pool.live_lease_count(descriptor(1)), 2);
    let mut observed = Vec::new();
    second
        .with_rows(0, 8, |bytes, rows| {
            assert_eq!(rows, 1);
            observed.extend_from_slice(bytes);
        })
        .expect("scoped read");
    assert_eq!(observed, pixels[..8]);
    assert!(first.release());
    assert!(second.release());
    assert_eq!(pool.live_lease_count(descriptor(1)), 0);
}

#[test]
fn active_lease_credit_refuses_without_minting_or_retiring_a_live_owner() {
    let pool = SceneRasterPool::try_new(limits(2)).expect("pool");
    let first = publish(&pool, descriptor(1), 7, &[1; 16]);
    let SceneRasterBegin::Reused(second) = pool.begin(descriptor(1), 8, SceneRasterWriteMode::Streamed) else { panic!("second") };
    let SceneRasterBegin::Backpressure(fault) = pool.begin(descriptor(1), 9, SceneRasterWriteMode::Streamed) else { panic!("capacity") };
    assert_eq!(fault, "scene raster lease credits are exhausted");
    assert_eq!(pool.live_lease_count(descriptor(1)), 2);
    assert!(first.release());
    let SceneRasterBegin::Reused(third) = pool.begin(descriptor(1), 10, SceneRasterWriteMode::Streamed) else { panic!("third") };
    assert_eq!(pool.live_lease_count(descriptor(1)), 2);
    assert!(second.release());
    assert!(third.release());
}

#[test]
fn same_source_revision_reuses_but_changed_bytes_revision_gets_a_distinct_slot() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let first = publish(&pool, descriptor(1), 7, &[1; 16]);
    let second = publish(&pool, descriptor(2), 8, &[2; 16]);
    assert_ne!(first.identity(), second.identity());
    assert_eq!(pool.reserved_bytes(), 32);
    assert!(first.release());
    assert!(second.release());
}

#[test]
fn moved_decode_transfers_its_exact_vec_without_exposing_writable_pool_bytes() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let SceneRasterBegin::Writer(writer) = pool.begin(descriptor(1), 7, SceneRasterWriteMode::Moved { capacity_bytes: 16 }) else { panic!("writer") };
    let pixels = vec![3; 16];
    let pointer = pixels.as_ptr();
    assert_eq!(pool.seal(writer), Err("moved scene raster must use seal_moved"));
    assert_eq!(pool.reserved_bytes(), 16);
    let lease = pool.seal_moved(writer, pixels).expect("seal moved");
    lease
        .with_rows(0, 8, |bytes, _| {
            assert_eq!(bytes.as_ptr(), pointer);
            assert_eq!(bytes, &[3; 8]);
        })
        .expect("read");
    assert!(lease.release());
}

#[test]
fn cancellation_and_zero_lease_eviction_hold_credits_until_bounded_retirement() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let SceneRasterBegin::Writer(writer) = pool.begin(descriptor(1), 7, SceneRasterWriteMode::Streamed) else { panic!("writer") };
    let writer = pool.push(writer, &[1; 8]).expect("push");
    assert!(pool.cancel(writer));
    assert_eq!(pool.reserved_bytes(), 16);
    assert!(!pool.maintenance_step());
    assert_eq!(pool.reserved_bytes(), 0);
    assert!(pool.maintenance_step());

    let first = publish(&pool, descriptor(1), 8, &[1; 16]);
    let second = publish(&pool, descriptor(2), 9, &[2; 16]);
    assert!(first.release());
    let SceneRasterBegin::Backpressure(fault) = pool.begin(descriptor(3), 10, SceneRasterWriteMode::Streamed) else { panic!("retire") };
    assert_eq!(fault, "scene raster retirement is pending");
    assert_eq!(pool.reserved_bytes(), 32);
    assert!(!pool.maintenance_step());
    assert_eq!(pool.reserved_bytes(), 16);
    let third = publish(&pool, descriptor(3), 10, &[3; 16]);
    assert!(second.release());
    assert!(third.release());
}

#[test]
fn stale_writer_and_duplicate_release_cannot_consume_another_owner() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let SceneRasterBegin::Writer(writer) = pool.begin(descriptor(1), 7, SceneRasterWriteMode::Streamed) else { panic!("writer") };
    let stale = writer;
    let writer = pool.push(writer, &[1; 8]).expect("push");
    assert!(pool.push(stale, &[1; 8]).is_err());
    assert!(pool.cancel(writer));
    assert!(pool.push(writer, &[1; 8]).is_err());
    while !pool.maintenance_step() {}
    let first = publish(&pool, descriptor(1), 8, &[1; 16]);
    let duplicate = SceneRasterLease { inner: first.inner.as_ref().map(Arc::clone), token: first.token, identity: first.identity,retirement:None };
    assert!(first.release());
    assert!(!duplicate.release());
    assert_eq!(pool.live_lease_count(descriptor(1)), 0);
}

#[test]
fn retiring_a_partial_small_writer_releases_only_its_exact_credit_beside_a_live_large_slot() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let large_descriptor = descriptor_sized(1, 2, 2);
    let large = publish(&pool, large_descriptor, 7, &[9; 16]);
    let small_descriptor = descriptor_sized(2, 1, 2);
    let SceneRasterBegin::Writer(small) = pool.begin(small_descriptor, 8, SceneRasterWriteMode::Streamed) else { panic!("small writer") };
    let small = pool.push(small, &[1, 2, 3, 4]).expect("partial small write");
    assert_eq!(pool.reserved_bytes(), 24);
    assert!(pool.cancel(small));
    assert!(!pool.maintenance_step());
    assert_eq!(pool.reserved_bytes(), 16);
    large
        .with_rows(0, 8, |bytes, rows| {
            assert_eq!(rows, 1);
            assert_eq!(bytes, &[9; 8]);
        })
        .expect("large slot remains readable");
    assert_eq!(pool.live_lease_count(large_descriptor), 1);
    assert!(large.release());
}

#[test]
fn mesh_paint_profile_requires_a_nonzero_revision_and_uv_seal() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let mut raster = descriptor(1);
    raster.profile = SceneRasterProfile::MeshPaintMapNoColorSpace;
    let SceneRasterBegin::Refused(fault) = pool.begin(raster, 7, SceneRasterWriteMode::Moved { capacity_bytes: raster.byte_len().unwrap() }) else { panic!("missing seal") };
    assert_eq!(fault, "scene raster mesh seal is missing or invalid for its profile");
    raster.mesh = Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 0, uv_count: 4 });
    assert!(matches!(pool.begin(raster, 7, SceneRasterWriteMode::Moved { capacity_bytes: raster.byte_len().unwrap() }), SceneRasterBegin::Refused(_)));
    raster.mesh = Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 2, uv_count: 4 });
    assert!(matches!(pool.begin(raster, 7, SceneRasterWriteMode::Moved { capacity_bytes: raster.byte_len().unwrap() }), SceneRasterBegin::Writer(_)));
}

#[test]
fn gpu_commit_releases_cpu_credit_and_six_distinct_residents_progress_through_two_slots() {
    let pool = SceneRasterPool::try_new(limits(4)).expect("pool");
    let mut gpu = Vec::new();
    let mut identities = Vec::new();
    for revision in 1..=6 {
        let raster = descriptor(revision);
        let lease = publish_after_bounded_retirement(&pool, raster, revision, &[revision as u8; 16]);
        let identity = lease.identity();
        let witness = lease.release_witness().expect("live CPU witness");
        gpu.push(witness.commit_gpu(&format!("surface-{revision}")).expect("GPU byte credit"));
        assert_eq!(pool.live_lease_count(raster), 0, "commit ACK releases CPU lease {revision}");
        assert_eq!(pool.gpu_resident_bytes(), revision as usize * 16);
        identities.push(identity);
    }
    assert_eq!(pool.reserved_bytes(), 32, "two CPU slots remain bounded after six commits");
    assert_eq!(pool.gpu_resident_bytes(), 96, "GPU residents retain their exact byte credit");
    for (index, identity) in identities.into_iter().enumerate() {
        assert!(pool.gpu_resident(&format!("surface-{}", index + 1), identity));
    }
    drop(gpu.pop());
    assert_eq!(pool.gpu_resident_bytes(), 80, "texture retirement releases only its exact GPU bytes");
}

#[test]
fn gpu_replacement_invalidates_the_old_witness_and_byte_budget_refuses_atomically() {
    let mut constrained = limits(4);
    constrained.gpu_resident_bytes = 32;
    let pool = SceneRasterPool::try_new(constrained).expect("pool");
    let first = publish(&pool, descriptor(1), 1, &[1; 16]);
    let first_identity = first.identity();
    let old = first.release_witness().expect("old CPU witness").commit_gpu("surface").expect("old GPU resident");
    let second = publish(&pool, descriptor(2), 2, &[2; 16]);
    let second_identity = second.identity();
    let replacement = second.release_witness().expect("replacement CPU witness").commit_gpu("surface").expect("replacement reuses the exact key credit");
    assert!(!pool.gpu_resident("surface", first_identity));
    assert!(pool.gpu_resident("surface", second_identity));
    assert_eq!(pool.gpu_resident_bytes(), 16);
    drop(old);
    assert!(pool.gpu_resident("surface", second_identity), "a stale replacement witness cannot retire the current texture");
    assert_eq!(pool.gpu_resident_bytes(), 16);

    let third = publish_after_bounded_retirement(&pool, descriptor(3), 3, &[3; 16]);
    let third_identity = third.identity();
    let third_gpu = third.release_witness().expect("third CPU witness").commit_gpu("other").expect("second resident fits");
    assert_eq!(pool.gpu_resident_bytes(), 32);
    let fourth = publish_after_bounded_retirement(&pool, descriptor(4), 4, &[4; 16]);
    let refused = fourth.release_witness().expect("fourth CPU witness").commit_gpu("overflow").expect_err("GPU byte capacity refuses before consuming CPU ownership");
    assert_eq!(pool.gpu_resident_bytes(), 32);
    assert_eq!(pool.live_lease_count(descriptor(4)), 1);
    assert!(refused.release());
    drop(third_gpu);
    assert_eq!(pool.gpu_resident_bytes(), 16);
    assert!(!pool.gpu_resident("other", third_identity));
    drop(replacement);
    assert_eq!(pool.gpu_resident_bytes(), 0);
}
