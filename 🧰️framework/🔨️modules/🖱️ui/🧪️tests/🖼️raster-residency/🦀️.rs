//! 🖼️ LAW: prepared frame ownership bounds the shared raster table across commit, abort, and close.

use super::*;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🖼️raster-residency/🔣️.json")).expect("raster residency fixture")
}

fn capacity_law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🖼️raster-capacity-followup/🔣️.json")).expect("raster capacity fixture")
}

fn scene_raster_law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖼️scene-raster-ownership/🔣️.json")).expect("scene raster ownership fixture")
}

fn witness(operation: u64) -> RasterTextureWitness {
    RasterTextureWitness { scene_revision: operation, preview_generation: operation, operation }
}

fn entry(key: &str, operation: u64, value: u16) -> RasterTextureEntry<u16> {
    RasterTextureEntry {
        key: RasterTextureKey::new(key).expect("fixture key"),
        witness: witness(operation),
        content_identity: crate::wgpu::prepared::RasterContentIdentity::revision(1, 1, [u64::from(value); 6]).expect("fixture identity"),
        bytes: 4,
        cpu_release: None,
        gpu_resident: None,
        value,
    }
}

fn publish(ledger: &mut RasterResidencyLedger, operation: u64, keys: &[&str]) {
    let owner = witness(operation);
    ledger.begin_candidate(owner).expect("candidate begins");
    for key in keys {
        ledger.publish_candidate_key(owner, key).expect("key publishes");
    }
    ledger.seal_candidate(owner).expect("candidate seals");
}

fn prune(live: &mut FixedRasterTextureRegistry<u16>, staged: &FixedRasterTextureRegistry<u16>, ledger: &RasterResidencyLedger) -> usize {
    let mut retired = 0;
    for index in 0..RASTER_TEXTURE_TABLE_CAPACITY {
        retired += usize::from(take_unowned_live_entry(live, staged, ledger, index).is_some());
    }
    retired
}

fn insert(live: &mut FixedRasterTextureRegistry<u16>, key: &str, operation: u64) {
    let previous = live.insert(entry(key, operation, operation as u16)).unwrap_or_else(|_| panic!("fixture registry admission"));
    assert!(previous.is_none());
}

fn key_for_start(start: usize) -> String {
    (0usize..1_000_000).map(|candidate| format!("capacity-{start:03}-{candidate:06}")).find(|key| RasterTextureKey::new(key).is_ok_and(|key| key.start() == start)).expect("deterministic key for full table slot")
}

#[cfg(not(target_arch = "wasm32"))]
static RASTER_TABLE_GPU_LAW_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(not(target_arch = "wasm32"))]
struct RasterTableGpuHarness {
    device: wgpu::Device,
    queue: wgpu::Queue,
    validation_scope: Option<wgpu::ErrorScopeGuard>,
    globals: wgpu::Buffer,
    _glyph_texture: wgpu::Texture,
    glyph_view: wgpu::TextureView,
    glyph_sampler: wgpu::Sampler,
    table: RasterTextureTable,
}

#[cfg(not(target_arch = "wasm32"))]
impl RasterTableGpuHarness {
    async fn new() -> Option<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false }).await.ok()?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("raster_table_lifecycle_law"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            })
            .await
            .ok()?;
        let validation_scope = Some(device.push_error_scope(wgpu::ErrorFilter::Validation));
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("raster_table_lifecycle_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });
        let globals = device.create_buffer(&wgpu::BufferDescriptor { label: Some("raster_table_lifecycle_globals"), size: 16, usage: wgpu::BufferUsages::UNIFORM, mapped_at_creation: false });
        let glyph_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("raster_table_lifecycle_glyph"),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let glyph_view = glyph_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let glyph_sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("raster_table_lifecycle_glyph_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        let table = RasterTextureTable::new(&device, &layout);
        Some(Self { device, queue, validation_scope, globals, _glyph_texture: glyph_texture, glyph_view, glyph_sampler, table })
    }

    fn identity(width: u32, height: u32, revision: u64) -> crate::wgpu::prepared::RasterContentIdentity {
        crate::wgpu::prepared::RasterContentIdentity::revision(width, height, [revision; 6]).expect("bounded raster identity")
    }

    fn publish(&mut self, owner: RasterTextureWitness, keys: &[String]) {
        self.table.begin_candidate_ownership(owner).expect("table candidate begins");
        for key in keys {
            self.table.publish_candidate_ownership(owner, key).expect("table candidate key publishes");
        }
        self.table.seal_candidate_ownership(owner).expect("table candidate seals");
    }

    fn stage(&mut self, key: &str, width: u32, height: u32, identity: crate::wgpu::prepared::RasterContentIdentity, owner: RasterTextureWitness) {
        assert!(self.table.prepare_admission_step(key, width, height, identity, owner).expect("table admission prepares"));
        let admission = self.table.reserve_engine_texture(key, width, height, identity, owner, owner).expect("table texture reserves");
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("raster_table_lifecycle_value"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.table.stage_gpu_bind_group(&self.device, &self.globals, &self.glyph_view, &self.glyph_sampler, admission, view, texture, owner).unwrap_or_else(|_| panic!("table raster stages"));
    }

    fn upload_scene(&mut self, key: &str, lease: &crate::wgpu::raster_ownership::SceneRasterLease, owner: RasterTextureWitness) {
        let descriptor = lease.identity().descriptor();
        for turn in 0..64 {
            if self
                .table
                .ensure_raster_step(
                    &self.device,
                    &self.queue,
                    &self.globals,
                    &self.glyph_view,
                    &self.glyph_sampler,
                    &self.glyph_view,
                    &self.glyph_sampler,
                    key,
                    RasterUploadPixels::Scene(lease),
                    descriptor.width,
                    descriptor.height,
                    owner,
                    owner,
                )
                .expect("scene raster uploads through the production table")
            {
                return;
            }
            assert!(turn + 1 < 64, "scene raster upload reaches a bounded terminal state");
        }
    }

    fn commit(&mut self, owner: RasterTextureWitness) {
        for turn in 0..16_384 {
            if self.table.commit_presented_step(owner).expect("table presentation commits") {
                return;
            }
            assert!(turn + 1 < 16_384, "table commit reaches bounded terminal state");
        }
    }

    fn abort(&mut self, owner: RasterTextureWitness) {
        for turn in 0..16_384 {
            if self.table.abort_presented_step(owner).expect("table presentation aborts") {
                return;
            }
            assert!(turn + 1 < 16_384, "table abort reaches bounded terminal state");
        }
    }

    fn retire_unowned(&mut self) {
        for turn in 0..16_384 {
            if self.table.retire_unowned_step().expect("unowned raster retires") {
                return;
            }
            assert!(turn + 1 < 16_384, "unowned raster retirement reaches a bounded terminal state");
        }
    }

    fn close(&mut self) {
        for turn in 0..65_536 {
            if self.table.close_step().expect("table closes") {
                assert!(self.table.terminal_is_empty());
                return;
            }
            assert!(turn + 1 < 65_536, "table close reaches bounded terminal state");
        }
    }

    async fn validation_error(&mut self) -> Option<wgpu::Error> {
        self.validation_scope.take().expect("one owned validation scope").pop().await
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn integration_scene_descriptor(source_revision: u64) -> crate::wgpu::raster_ownership::SceneRasterDescriptor {
    crate::wgpu::raster_ownership::SceneRasterDescriptor {
        width: 2,
        height: 1,
        source_digest: [source_revision, source_revision.rotate_left(17)],
        source_revision,
        profile: crate::wgpu::raster_ownership::SceneRasterProfile::ReferenceImageMapNoColorSpace,
        mesh: None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_integration_scene(
    pool: &crate::wgpu::raster_ownership::SceneRasterPool,
    descriptor: crate::wgpu::raster_ownership::SceneRasterDescriptor,
    owner: u64,
    byte: u8,
) -> crate::wgpu::raster_ownership::SceneRasterLease {
    use crate::wgpu::raster_ownership::{SceneRasterBegin, SceneRasterWriteMode};
    for turn in 0..4 {
        match pool.begin(descriptor, owner, SceneRasterWriteMode::Streamed) {
            SceneRasterBegin::Writer(writer) => {
                let writer = pool.push(writer, &[byte; 8]).expect("scene raster row publishes");
                return pool.seal(writer).expect("scene raster pixels seal");
            }
            SceneRasterBegin::Backpressure("scene raster retirement is pending") => {
                assert!(!pool.maintenance_step(), "one CPU slot retirement grant remains bounded");
            }
            _ => panic!("scene raster CPU publication must progress or own one retirement grant"),
        }
        assert!(turn + 1 < 4, "scene raster CPU publication reaches a bounded terminal state");
    }
    unreachable!("bounded scene raster publication returned above")
}

#[test]
fn neutral_fixture_pins_sources_capacity_and_required_lifetimes() {
    let law = law();
    assert_eq!(law["schema"], "semio.ui.prepared-raster-residency.v1");
    assert_eq!(law["capacity"]["items"], RASTER_TEXTURE_TABLE_CAPACITY);
    assert_eq!(law["capacity"]["bytes"], RASTER_TEXTURE_TABLE_BYTE_CAPACITY);
    assert_eq!(law["capacity"]["retirementUnitsPerStep"], 1);
    assert_eq!(law["sources"], serde_json::json!(["worldReference", "worldPaint", "uiRaster", "overlayRaster"]));
    assert_eq!(law["scenarios"].as_array().expect("fixture scenarios").len(), 6);
}

#[test]
fn neutral_capacity_fixture_pins_identity_peak_policy_and_single_engine_target() {
    let law = capacity_law();
    assert_eq!(law["schema"], "semio.ui.raster-capacity-followup.v1");
    assert_eq!(law["identity"], serde_json::json!({ "pageBytes": crate::wgpu::prepared::PREPARED_RASTER_PAGE_BYTES, "digestLanes": 2, "dimensionsIncluded": true }));
    assert_eq!(law["capacity"]["items"], RASTER_TEXTURE_TABLE_CAPACITY);
    assert_eq!(law["capacity"]["bytes"], RASTER_TEXTURE_TABLE_BYTE_CAPACITY);
    assert_eq!(law["capacity"]["changedFullPolicy"], "backpressureUntilHeadroom");
    assert_eq!(law["scenarios"].as_array().expect("capacity scenarios").len(), 6);
    assert_eq!(law["scenarios"][5]["expected"]["presenterTargets"], 0);
    assert_eq!(law["scenarios"][5]["expected"]["tableTargets"], 1);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn real_gpu_table_commits_six_scene_leases_retires_the_first_and_recovers_its_natural_dimensions() {
    use crate::wgpu::raster_ownership::{SceneRasterPool, SceneRasterPoolLimits};

    let _guard = RASTER_TABLE_GPU_LAW_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(mut harness) = semio_framework_async::block_on(RasterTableGpuHarness::new()) else {
        eprintln!("[DEBUG] raster-table-scene-integration skipped: no headless WGPU adapter available");
        return;
    };
    let contract = scene_raster_law();
    assert_eq!(contract["creditLifetimes"]["gpuUploadLogical"], "committed-ack-or-partial-retirement-terminal");
    assert_eq!(contract["creditLifetimes"]["gpuResident"], "texture-retirement-terminal");
    assert_eq!(contract["upload"]["committedAckRequired"], true);
    assert_eq!(contract["profiles"][0]["id"], "reference-image-map-v1");

    let pool = SceneRasterPool::try_new(SceneRasterPoolLimits {
        item_bytes: 8,
        pool_bytes: 32,
        slot_capacity: 2,
        lease_capacity_per_slot: 4,
        transfer_bytes: 8,
        retire_bytes: 8,
        gpu_resident_bytes: 48,
    })
    .expect("private two-slot scene raster pool");
    let mut keys = Vec::new();
    let mut descriptors = Vec::new();
    let mut identities = Vec::new();

    for revision in 1..=6_u64 {
        let descriptor = integration_scene_descriptor(revision);
        let lease = publish_integration_scene(&pool, descriptor, revision, revision as u8);
        let identity = lease.identity();
        let key = format!("scene-raster-{revision}");
        keys.push(key.clone());
        descriptors.push(descriptor);
        identities.push(identity);
        let owner = witness(1_000 + revision);
        harness.publish(owner, &keys);
        harness.upload_scene(&key, &lease, owner);
        assert!(harness.table.begin_presenting(owner).expect("new scene texture begins presentation"));
        harness.commit(owner);
        harness.table.release_previous_ownership();
        assert_eq!(harness.table.residency_counts(), (revision as usize, 0, 0));
        assert_eq!(harness.table.get(&key).map(|texture| (texture.width, texture.height)), Some((2, 1)));
        assert!(pool.gpu_resident(&key, identity), "GPU ACK owns scene raster {revision}");
        assert_eq!(pool.live_lease_count(descriptor), 0, "GPU ACK releases CPU lease {revision}");
        assert_eq!(pool.gpu_resident_bytes(), revision as usize * 8);
        drop(lease);
    }

    assert_eq!(pool.reserved_bytes(), 16, "two 8-byte CPU slots remain under the private 32-byte pool limit");
    assert!(!pool.cpu_resident(identities[0]), "the first CPU raster was evicted while its real GPU texture stayed resident");
    for ((key, identity), descriptor) in keys.iter().zip(&identities).zip(&descriptors) {
        assert!(harness.table.get(key).is_some(), "all six real GPU textures remain table-owned");
        assert!(pool.gpu_resident(key, *identity), "all six exact GPU witnesses remain live");
        assert_eq!(identity.descriptor(), *descriptor);
    }

    let without_first = witness(2_000);
    harness.publish(without_first, &keys[1..]);
    assert!(!harness.table.begin_presenting(without_first).expect("ownership-only frame stages no texture"));
    harness.commit(without_first);
    harness.table.release_previous_ownership();
    harness.retire_unowned();
    assert!(harness.table.get(&keys[0]).is_none(), "the first real GPU texture retires with its absent keep-set key");
    assert!(!pool.gpu_resident(&keys[0], identities[0]), "actual table retirement drops the first GPU witness");
    assert_eq!(pool.gpu_resident_bytes(), 40);

    let recovered = publish_integration_scene(&pool, descriptors[0], 3_000, 1);
    assert_eq!(recovered.identity(), identities[0], "same descriptor and pixels recover the same content identity");
    let recovered_owner = witness(3_000);
    harness.publish(recovered_owner, &keys);
    harness.upload_scene(&keys[0], &recovered, recovered_owner);
    assert!(harness.table.begin_presenting(recovered_owner).expect("recovered texture begins presentation"));
    harness.commit(recovered_owner);
    harness.table.release_previous_ownership();
    assert_eq!(harness.table.get(&keys[0]).map(|texture| (texture.width, texture.height)), Some((2, 1)), "recovered reference keeps its natural 2:1 dimensions");
    assert!(pool.gpu_resident(&keys[0], identities[0]));
    assert_eq!(pool.live_lease_count(descriptors[0]), 0);
    assert_eq!(pool.gpu_resident_bytes(), 48);
    drop(recovered);

    harness.close();
    assert_eq!(pool.gpu_resident_bytes(), 0, "closing the real table retires every GPU witness byte");
    assert!(semio_framework_async::block_on(harness.validation_error()).is_none());
    eprintln!("[DEBUG] raster-table-scene-integration executed: textures=6 cpu_slots=2 resident_bytes=48 recovered=2x1 closed_bytes=0");
}

#[test]
fn repeated_full_capacity_frame_reuses_exact_content_without_staging_or_peak_credit() {
    let mut live = FixedRasterTextureRegistry::default();
    let staged = FixedRasterTextureRegistry::default();
    let mut keys = Vec::with_capacity(RASTER_TEXTURE_TABLE_CAPACITY);
    for index in 0..RASTER_TEXTURE_TABLE_CAPACITY {
        let key = key_for_start(index);
        insert(&mut live, &key, u64::try_from(index).expect("fixture operation"));
        keys.push(key);
    }
    assert_eq!(live.len, RASTER_TEXTURE_TABLE_CAPACITY);
    for (index, key) in keys.iter().enumerate() {
        let identity = crate::wgpu::prepared::RasterContentIdentity::revision(1, 1, [index as u64; 6]).expect("fixture identity");
        assert!(raster_content_is_reusable(&live, &staged, RasterTextureKey::new(key).expect("fixture key"), identity, witness(300)).expect("unchanged content decision"));
    }
    assert!(staged.is_empty());
    assert!(!raster_admission_fits(live.len, live.bytes, 4), "changed content at full retained capacity must backpressure");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn production_table_reoffers_a_full_committed_frame_and_backpressures_a_changed_protected_key() {
    let _guard = RASTER_TABLE_GPU_LAW_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(mut harness) = semio_framework_async::block_on(RasterTableGpuHarness::new()) else {
        eprintln!("skipping: no headless WGPU adapter available");
        return;
    };
    let keys = (0..RASTER_TEXTURE_TABLE_CAPACITY).map(key_for_start).collect::<Vec<_>>();
    let committed = witness(10);
    harness.publish(committed, &keys);
    for (index, key) in keys.iter().enumerate() {
        harness.stage(key, 1, 1, RasterTableGpuHarness::identity(1, 1, index as u64 + 1), committed);
    }
    assert!(harness.table.begin_presenting(committed).expect("full frame begins presentation"));
    harness.commit(committed);
    harness.table.release_previous_ownership();
    assert_eq!(harness.table.residency_counts(), (RASTER_TEXTURE_TABLE_CAPACITY, 0, 0));
    assert_eq!(harness.table.live.len, RASTER_TEXTURE_TABLE_CAPACITY);

    let repeated = witness(11);
    harness.publish(repeated, &keys);
    for (index, key) in keys.iter().enumerate() {
        assert!(harness.table.prepare_admission_step(key, 1, 1, RasterTableGpuHarness::identity(1, 1, index as u64 + 1), repeated).expect("unchanged full-table admission reuses"));
    }
    assert!(harness.table.staged.is_empty());
    assert!(!harness.table.begin_presenting(repeated).expect("unchanged frame needs no staged presentation"));
    harness.commit(repeated);
    harness.table.release_previous_ownership();

    let changed = witness(12);
    harness.publish(changed, &keys);
    let mut pending = 0usize;
    let fault = loop {
        match harness.table.prepare_admission_step(&keys[0], 1, 1, RasterTableGpuHarness::identity(1, 1, u64::MAX), changed) {
            Ok(false) => pending += 1,
            Ok(true) => panic!("changed protected content must not exceed full table capacity"),
            Err(fault) => break fault,
        }
        assert!(pending <= RASTER_TEXTURE_TABLE_CAPACITY + 1, "full-table admission scan stays bounded");
    };
    assert_eq!(fault, "raster texture credits are owned by retained frames");
    assert!(pending >= RASTER_TEXTURE_TABLE_CAPACITY);
    assert_eq!(harness.table.live.len, RASTER_TEXTURE_TABLE_CAPACITY);
    assert!(harness.table.staged.is_empty());
    harness.abort(changed);
    harness.close();
    assert!(semio_framework_async::block_on(harness.validation_error()).is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn production_table_paints_changed_same_key_only_while_presenting_and_abort_restores_committed_content() {
    let _guard = RASTER_TABLE_GPU_LAW_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(mut harness) = semio_framework_async::block_on(RasterTableGpuHarness::new()) else {
        eprintln!("skipping: no headless WGPU adapter available");
        return;
    };
    let keys = vec!["image".to_string()];
    let a = witness(20);
    harness.publish(a, &keys);
    harness.stage(&keys[0], 1, 1, RasterTableGpuHarness::identity(1, 1, 1), a);
    assert!(harness.table.get(&keys[0]).is_none());
    assert!(harness.table.begin_presenting(a).expect("A begins presentation"));
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(1));
    harness.commit(a);
    harness.table.release_previous_ownership();

    let aborted_b = witness(21);
    harness.publish(aborted_b, &keys);
    harness.stage(&keys[0], 2, 1, RasterTableGpuHarness::identity(2, 1, 2), aborted_b);
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(1));
    assert!(harness.table.begin_presenting(aborted_b).expect("B begins presentation"));
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(2));
    harness.abort(aborted_b);
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(1));
    assert!(harness.table.staged.is_empty());

    let committed_b = witness(22);
    harness.publish(committed_b, &keys);
    harness.stage(&keys[0], 2, 1, RasterTableGpuHarness::identity(2, 1, 2), committed_b);
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(1));
    assert!(harness.table.begin_presenting(committed_b).expect("replacement B begins presentation"));
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(2));
    harness.commit(committed_b);
    assert_eq!(harness.table.get(&keys[0]).map(|texture| texture.width), Some(2));
    assert_eq!(harness.table.residency_counts(), (1, 0, 1));
    harness.table.release_previous_ownership();
    assert_eq!(harness.table.residency_counts(), (1, 0, 0));
    harness.close();
    assert!(semio_framework_async::block_on(harness.validation_error()).is_none());
}

#[test]
fn changed_same_key_stages_distinct_content_and_rejects_conflicting_candidate_offer() {
    let mut live = FixedRasterTextureRegistry::default();
    let mut staged = FixedRasterTextureRegistry::default();
    insert(&mut live, "image", 1);
    let changed = entry("image", 2, 2);
    assert!(!raster_content_is_reusable(&live, &staged, changed.key, changed.content_identity, witness(2)).expect("changed content decision"));
    staged.insert(changed).ok().expect("changed content stages beside committed content");
    let identity_b = crate::wgpu::prepared::RasterContentIdentity::revision(1, 1, [2; 6]).expect("B identity");
    let identity_c = crate::wgpu::prepared::RasterContentIdentity::revision(1, 1, [3; 6]).expect("C identity");
    let key = RasterTextureKey::new("image").expect("fixture key");
    assert!(raster_content_is_reusable(&live, &staged, key, identity_b, witness(2)).expect("same staged content reuses"));
    assert!(raster_content_is_reusable(&live, &staged, key, identity_c, witness(2)).is_err());
    assert!(live.get("image").is_some_and(|entry| entry.content_identity != identity_b));
}

#[test]
fn previous_owner_prevents_retirement_until_successor_release() {
    let mut ledger = RasterResidencyLedger::default();
    let mut live = FixedRasterTextureRegistry::default();
    let staged = FixedRasterTextureRegistry::default();
    insert(&mut live, "image-A", 1);
    publish(&mut ledger, 1, &["image-A"]);
    ledger.commit(witness(1)).expect("A commits");
    ledger.release_previous();
    insert(&mut live, "image-B", 2);
    publish(&mut ledger, 2, &["image-B"]);
    ledger.commit(witness(2)).expect("B commits while A becomes previous");
    assert_eq!(prune(&mut live, &staged, &ledger), 0);
    assert!(live.get("image-A").is_some());
    ledger.release_previous();
    assert_eq!(prune(&mut live, &staged, &ledger), 1);
    assert!(live.get("image-B").is_some());
}

#[test]
fn three_hundred_distinct_committed_replacements_stay_bounded() {
    let mut ledger = RasterResidencyLedger::default();
    let mut live = FixedRasterTextureRegistry::default();
    let staged = FixedRasterTextureRegistry::default();
    let mut maximum = 0;
    for operation in 1..=300 {
        let key = format!("reference-{operation:03}");
        publish(&mut ledger, operation, &[key.as_str()]);
        insert(&mut live, key.as_str(), operation);
        maximum = maximum.max(live.len);
        ledger.commit(witness(operation)).expect("frame commits");
        ledger.release_previous();
        prune(&mut live, &staged, &ledger);
        assert!(live.len <= 1);
    }
    assert!(maximum <= 2);
    assert_eq!(live.len, 1);
    assert!(live.get("reference-300").is_some());
}

#[test]
fn shared_key_survives_one_owner_close_and_final_owner_release_retires_it() {
    let mut ledger = RasterResidencyLedger::default();
    let mut live = FixedRasterTextureRegistry::default();
    let staged = FixedRasterTextureRegistry::default();
    publish(&mut ledger, 1, &["image-A", "image-A"]);
    insert(&mut live, "image-A", 1);
    ledger.commit(witness(1)).expect("shared frame commits");
    ledger.release_previous();
    assert_eq!(ledger.counts(), (1, 0, 0));
    publish(&mut ledger, 2, &["image-A"]);
    ledger.commit(witness(2)).expect("remaining owner commits");
    ledger.release_previous();
    assert_eq!(prune(&mut live, &staged, &ledger), 0);
    assert!(live.get("image-A").is_some());
    publish(&mut ledger, 3, &[]);
    ledger.commit(witness(3)).expect("empty frame commits");
    ledger.release_previous();
    assert_eq!(prune(&mut live, &staged, &ledger), 1);
    assert!(live.is_empty());
}

#[test]
fn aborted_replacement_retires_transient_b_and_keeps_committed_a_paintable() {
    let mut ledger = RasterResidencyLedger::default();
    let mut live = FixedRasterTextureRegistry::default();
    let mut staged = FixedRasterTextureRegistry::default();
    publish(&mut ledger, 1, &["image-A"]);
    insert(&mut live, "image-A", 1);
    ledger.commit(witness(1)).expect("A commits");
    ledger.release_previous();
    publish(&mut ledger, 2, &["image-A"]);
    insert(&mut staged, "image-A", 2);
    ledger.abort(witness(2)).expect("B aborts");
    let index = staged.locate(RasterTextureKey::new("image-A").expect("replacement key")).expect("replacement lookup").expect("B staged");
    let _ = staged.take(index);
    assert_eq!(prune(&mut live, &staged, &ledger), 0);
    assert!(live.get("image-A").is_some());
    assert_eq!(live.get("image-A").map(|entry| entry.content_identity), Some(crate::wgpu::prepared::RasterContentIdentity::revision(1, 1, [1; 6]).expect("A identity")));
    assert!(staged.is_empty());
}

#[test]
fn all_world_ui_and_overlay_raster_sources_publish_into_one_exact_set() {
    let mut draw = DrawList::default();
    let mut pass = crate::wgpu::kernel_3d_scene::ScenePass3d::default();
    pass.textured_draws.push(crate::wgpu::kernel_3d_scene::TexturedDraw3d {
        instances: vec![crate::wgpu::kernel_3d_scene::TexturedInstance3d { texture_key: "world-reference".into(), model: crate::wgpu::kernel_3d_scene::Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), background: [0.0; 4], appearance: [1.0, 0.0, 0.0, 0.0] }],
        ..Default::default()
    });
    pass.material_draws.push(crate::wgpu::kernel_3d_scene::SceneMaterialDraw3d {
        mesh_key: "painted-mesh".into(),
        mesh_version: 1,
        instances: vec![crate::wgpu::kernel_3d_scene::Instance3d {
            id: "painted-instance".into(),
            model: crate::wgpu::kernel_3d_scene::Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]),
            color: [1.0; 4],
            selected: false,
            hovered: false,
            material: Default::default(),
        }],
        material: crate::wgpu::kernel_3d_scene::SceneMaterialKind3d::Painted { texture_key: "mesh-paint".into() },
        translucent: false,
    });
    draw.scene_passes.push(pass);
    draw.layers[0].raster_instances.push(("world-paint".into(), UiInstance::raster([0.0; 4], [0.0, 0.0, 1.0, 1.0], 1.0)));
    draw.layers[0].overlay_raster_instances.push(("overlay-image".into(), UiInstance::raster([0.0; 4], [0.0, 0.0, 1.0, 1.0], 1.0)));
    let mut cursor = RasterKeepCursorV1::default();
    let mut ledger = RasterResidencyLedger::default();
    let owner = witness(1);
    ledger.begin_candidate(owner).expect("candidate begins");
    loop {
        match draw.raster_keep_step(&mut cursor) {
            RasterKeepStepV1::Pending => {}
            RasterKeepStepV1::Key(key) => ledger.publish_candidate_key(owner, key).expect("source publishes"),
            RasterKeepStepV1::Complete => break,
        }
    }
    ledger.seal_candidate(owner).expect("candidate seals");
    assert_eq!(ledger.counts(), (0, 4, 0));
}

#[test]
fn full_budget_recovers_by_retiring_only_unowned_live_entries() {
    let mut ledger = RasterResidencyLedger::default();
    let mut live = FixedRasterTextureRegistry::default();
    let staged = FixedRasterTextureRegistry::default();
    for index in 0..RASTER_TEXTURE_TABLE_CAPACITY {
        live.insert_vacant(index, entry(format!("historical-{index:03}").as_str(), 1, index as u16)).unwrap_or_else(|_| panic!("fixed slot"));
    }
    publish(&mut ledger, 1, &["historical-255"]);
    ledger.commit(witness(1)).expect("current frame commits");
    ledger.release_previous();
    publish(&mut ledger, 2, &["image-B"]);
    assert_eq!(prune(&mut live, &staged, &ledger), 255);
    assert_eq!(live.len, 1);
    assert!(live.slots.iter().flatten().any(|entry| entry.key.as_str() == "historical-255"));
    insert(&mut live, "image-B", 2);
    assert_eq!(live.len, 2);
}
