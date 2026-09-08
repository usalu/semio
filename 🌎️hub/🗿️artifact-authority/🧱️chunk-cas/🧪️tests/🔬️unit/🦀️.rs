
use super::*;
use crate::artifact_authority::{AuthorityLimits, AuthorityOperationControl, AuthorityProgress};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct TestControl {
    now: AtomicU64,
    cancelled: AtomicBool,
}

impl AuthorityOperationControl for TestControl {
    fn now_ms(&self) -> u64 {
        self.now.load(Ordering::SeqCst)
    }
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
    fn report(&self, _: AuthorityProgress) {}
}

fn control() -> TestControl {
    TestControl { now: AtomicU64::new(1), cancelled: AtomicBool::new(false) }
}

fn bytes(length: usize) -> Vec<u8> {
    (0..length).map(|index| ((index * 31 + index / 251) % 256) as u8).collect()
}

#[test]
fn artifact_chunk_cas_manifest_boundaries_are_canonical_and_space_scoped() {
    for length in [0, 1, 262_143, 262_144, 262_145] {
        let raw = bytes(length);
        let plan = prepare_artifact_cas_manifest_v1("space-ü", &raw).expect("plan");
        assert_eq!(plan.manifest.chunks.len(), length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES));
        assert_eq!(decode_artifact_cas_manifest_v1(&plan.manifest_bytes, "space-ü", plan.manifest_id).expect("decode"), plan.manifest);
        assert_ne!(prepare_artifact_cas_manifest_v1("space-b", &raw).expect("other space").manifest_id, plan.manifest_id);
        let mut trailing = plan.manifest_bytes.clone();
        trailing.push(0);
        assert!(decode_artifact_cas_manifest_v1(&trailing, "space-ü", ArtifactHash(Sha256::digest(&trailing))).is_err());
        let mut mutated = plan.manifest_bytes.clone();
        if let Some(last) = mutated.last_mut() {
            *last ^= 1;
            assert!(decode_artifact_cas_manifest_v1(&mutated, "space-ü", plan.manifest_id).is_err());
        }
    }
    assert!(decode_artifact_cas_manifest_locator_v1("semio.artifact-cas.manifest/v1/AA").is_err());
}

#[test]
fn artifact_chunk_cas_neutral_fixture_matches_repository_sha256() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json")).expect("fixture JSON");
    let space_id = fixture["spaceId"].as_str().expect("space id");
    let assert_vector = |vector: &serde_json::Value| {
        let length = usize::try_from(vector["length"].as_u64().expect("length")).expect("bounded length");
        let plan = prepare_artifact_cas_manifest_v1(space_id, &bytes(length)).expect("plan vector");
        assert_eq!(hex_lower(&plan.manifest.raw_sha256.0), vector["rawSha256"].as_str().expect("raw hash"));
        assert_eq!(plan.manifest.chunks.len() as u64, vector["chunkCount"].as_u64().expect("chunk count"));
        assert_eq!(plan.manifest_bytes.len() as u64, vector["manifestBytes"].as_u64().expect("manifest bytes"));
        assert_eq!(hex_lower(&plan.manifest_id.0), vector["manifestId"].as_str().expect("manifest id"));
        assert_eq!(plan.manifest.chunks.first().map(|chunk| hex_lower(&chunk.chunk_id.0)), vector["firstChunkId"].as_str().map(str::to_string));
        assert_eq!(plan.manifest.chunks.last().map(|chunk| hex_lower(&chunk.chunk_id.0)), vector["lastChunkId"].as_str().map(str::to_string));
    };
    for vector in fixture["vectors"].as_array().expect("vectors") {
        assert_vector(vector);
    }
    assert_vector(&fixture["largePair"]["pack"]);
    let ledger = &fixture["retentionLedger"];
    assert_eq!(ledger["reservationMaximumTtlMs"].as_u64(), Some(crate::directory::ARTIFACT_CAS_RESERVATION_MAX_TTL_MS));
    assert_eq!(ledger["reservationGraceMs"].as_u64(), Some(ARTIFACT_CAS_RESERVATION_GRACE_MS));
    assert_eq!(ledger["sweepPageMaximum"].as_u64(), Some(crate::directory::ARTIFACT_CAS_SWEEP_PAGE_MAX as u64));
    assert_eq!(ledger["sweepObjectMaximum"].as_u64(), Some(crate::directory::ARTIFACT_CAS_SWEEP_OBJECT_MAX as u64));
}

#[test]
fn artifact_chunk_cas_ownership_codec_is_canonical_scoped_and_locator_exact() {
    let pair = ArtifactPair { pack: bytes(ARTIFACT_CAS_CHUNK_BYTES + 1), spr: bytes(1) };
    let mut checkpoint = ArtifactCheckpoint {
        scope: DocumentScope::new("space-a", "document-a"),
        checkpoint_id: ArtifactHash([3; 32]),
        parent_checkpoint_id: None,
        descriptor_digest_v1: ArtifactHash([4; 32]),
        baseline_frontier: directory::os_directory::ArtifactFrontier { document_id: "document-a".into(), head_edit_ordinal: 1, head_edit_id: "edit-1".into(), last_commit_seq: 1, chain_hash: ArtifactHash([5; 32]) },
        pack: directory::os_directory::ArtifactBlobRef { sha256: ArtifactHash(Sha256::digest(&pair.pack)), byte_length: pair.pack.len() as u64, storage_key: String::new() },
        spr: directory::os_directory::ArtifactBlobRef { sha256: ArtifactHash(Sha256::digest(&pair.spr)), byte_length: pair.spr.len() as u64, storage_key: String::new() },
        aggregate_sha256: ArtifactHash([6; 32]),
        published_at_ms: 1,
    };
    let plan = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("ownership plan");
    let encoded = encode_artifact_cas_ownership_v1(&plan).expect("ownership encoding");
    assert_eq!(decode_artifact_cas_ownership_v1(&encoded).expect("ownership decode"), plan);
    checkpoint.pack.storage_key = artifact_cas_manifest_locator_v1(plan.pack_manifest_id);
    checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(plan.spr_manifest_id);
    validate_artifact_cas_publication_v1(&plan, &checkpoint).expect("exact reserved locators");
    checkpoint.spr.storage_key = artifact_cas_manifest_locator_v1(plan.pack_manifest_id);
    assert!(validate_artifact_cas_publication_v1(&plan, &checkpoint).is_err());
    let mut mutated = encoded;
    *mutated.last_mut().expect("ownership byte") ^= 1;
    assert!(decode_artifact_cas_ownership_v1(&mutated).is_err());
}

async fn storage_roundtrip_law<S: ArtifactChunkCasStorage>(storage: Arc<S>) {
    let raw = bytes(496 * 1024 + 1);
    let expected = ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(&raw)), byte_length: raw.len() as u64 };
    let control = control();
    let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
    let adapter = ArtifactChunkBlobStore::new(storage.clone());
    let staged = adapter.stage("space-a", expected, &raw, &context).await.expect("stage > legacy max");
    assert!(staged.storage_key.starts_with(ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX));
    assert_eq!(adapter.read("space-a", &staged, &context).await.expect("read"), raw);
    assert!(adapter.read("space-b", &staged, &context).await.is_err());
    assert_eq!(adapter.stage("space-a", expected, &raw, &context).await.expect("idempotent"), staged);
}

async fn fenced_delete_law<S: ArtifactChunkCasStorage>(storage: Arc<S>, space_id: &str) {
    let raw = bytes(ARTIFACT_CAS_CHUNK_BYTES + 1);
    let expected = ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(&raw)), byte_length: raw.len() as u64 };
    let control = control();
    let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
    let adapter = ArtifactChunkBlobStore::new(storage.clone());
    storage.configure_coordinator([1; 32], &context).await.expect("configure deletion coordinator");
    storage.advance_physical_epoch([1; 32], space_id, 1, &context).await.expect("activate deletion epoch");
    let staged = adapter.stage(space_id, expected, &raw, &context).await.expect("stage deletion fixture");
    let plan = prepare_artifact_cas_manifest_v1(space_id, &raw).expect("deletion plan");
    let key = ArtifactCasObjectKey { space_id: space_id.into(), kind: ArtifactCasObjectKind::Chunk, digest: plan.manifest.chunks[0].chunk_id };
    let wrong = ArtifactCasDeleteFence::new(ArtifactCasObjectKey { space_id: "wrong-space".into(), kind: key.kind, digest: key.digest }, 1, [1; 32], 1, [2; 32]);
    assert!(storage.delete_if_unreferenced(&key, &wrong, &context).await.is_err());
    let stale_fence = ArtifactCasDeleteFence::new(key.clone(), 1, [1; 32], 1, [2; 32]);
    storage.advance_physical_epoch([1; 32], space_id, 2, &context).await.expect("activate later reservation epoch");
    assert!(storage.delete_if_unreferenced(&key, &stale_fence, &context).await.is_err());
    assert!(storage.get(&key, &context).await.is_ok());
    let fence = ArtifactCasDeleteFence::new(key.clone(), 1, [1; 32], 2, [2; 32]);
    assert_eq!(storage.delete_if_unreferenced(&key, &fence, &context).await.expect("fenced delete"), ArtifactCasDeleteOutcome::Deleted);
    assert_eq!(storage.delete_if_unreferenced(&key, &fence, &context).await.expect("idempotent fenced delete"), ArtifactCasDeleteOutcome::Missing);
    assert!(adapter.read(space_id, &staged, &context).await.is_err());
}

#[tokio::test]
async fn artifact_chunk_cas_memory_roundtrip_crosses_legacy_payload_ceiling() {
    let storage = Arc::new(MemoryArtifactChunkCasStorage::default());
    storage_roundtrip_law(storage.clone()).await;
    fenced_delete_law(storage, "memory-delete-space").await;
}

#[tokio::test]
async fn artifact_chunk_cas_filesystem_roundtrip_restart_and_collision_checks() {
    let root = std::env::temp_dir().join(format!("semio-artifact-cas-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("clean stale fixture")
    }
    let storage = Arc::new(FsArtifactChunkCasStorage::open(&root).await.expect("open filesystem CAS"));
    storage_roundtrip_law(storage).await;
    let reopened = Arc::new(FsArtifactChunkCasStorage::open(&root).await.expect("reopen filesystem CAS"));
    let raw = bytes(496 * 1024 + 1);
    let expected = ArtifactBlobIntegrity { sha256: ArtifactHash(Sha256::digest(&raw)), byte_length: raw.len() as u64 };
    let control = control();
    let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
    let plan = prepare_artifact_cas_manifest_v1("space-a", &raw).expect("restart plan");
    let staged = StagedArtifactBlob { storage_key: artifact_cas_manifest_locator_v1(plan.manifest_id), integrity: expected };
    let adapter = ArtifactChunkBlobStore::new(reopened.clone());
    assert_eq!(adapter.read("space-a", &staged, &context).await.expect("read after restart"), raw);
    let first = &plan.manifest.chunks[0];
    let first_key = ArtifactCasObjectKey { space_id: "space-a".into(), kind: ArtifactCasObjectKind::Chunk, digest: first.chunk_id };
    let mut corrupted = bytes(first.byte_length as usize);
    corrupted[0] ^= 1;
    tokio::fs::write(reopened.object_path(&first_key).expect("chunk path"), corrupted).await.expect("inject one-bit corruption");
    assert!(adapter.read("space-a", &staged, &context).await.is_err());
    fenced_delete_law(reopened, "filesystem-delete-space").await;
    std::fs::remove_dir_all(root).expect("remove fixture");
}

#[tokio::test]
async fn artifact_chunk_cas_filesystem_process_epoch_fences_stale_delete() {
    const ROOT_ENV: &str = "SEMIO_ARTIFACT_CAS_PROCESS_FENCE_ROOT";
    const EPOCH_ENV: &str = "SEMIO_ARTIFACT_CAS_PROCESS_FENCE_EPOCH";
    if let (Ok(root), Ok(epoch)) = (std::env::var(ROOT_ENV), std::env::var(EPOCH_ENV)) {
        let storage = FsArtifactChunkCasStorage::open(Path::new(&root)).await.expect("child opens shared filesystem CAS");
        let control = control();
        let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
        storage.configure_coordinator([7; 32], &context).await.expect("child verifies coordinator");
        storage.advance_physical_epoch([7; 32], "process-race-space", epoch.parse().expect("child epoch"), &context).await.expect("child advances shared epoch");
        return;
    }
    let root = std::env::temp_dir().join(format!("semio-artifact-cas-process-fence-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("clean process fence fixture")
    }
    let storage = FsArtifactChunkCasStorage::open(&root).await.expect("open parent filesystem CAS");
    let control = control();
    let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
    storage.configure_coordinator([7; 32], &context).await.expect("configure process coordinator");
    storage.advance_physical_epoch([7; 32], "process-race-space", 1, &context).await.expect("activate parent epoch");
    let raw = b"cross-process-fence";
    let key = ArtifactCasObjectKey { space_id: "process-race-space".into(), kind: ArtifactCasObjectKind::Chunk, digest: artifact_cas_chunk_id_v1("process-race-space", raw).expect("process object digest") };
    storage.put_if_absent(&key, raw, &context).await.expect("store process object");
    let executable = std::env::current_exe().expect("test executable");
    let spawn = |epoch: u64| {
        std::process::Command::new(&executable).arg("artifact_chunk_cas_filesystem_process_epoch_fences_stale_delete").arg("--test-threads=1").env(ROOT_ENV, &root).env(EPOCH_ENV, epoch.to_string()).spawn().expect("spawn filesystem fence child")
    };
    let mut second = spawn(2);
    let mut third = spawn(3);
    assert!(second.wait().expect("wait epoch two child").success());
    assert!(third.wait().expect("wait epoch three child").success());
    let stale = ArtifactCasDeleteFence::new(key.clone(), 1, [7; 32], 2, [8; 32]);
    assert!(storage.delete_if_unreferenced(&key, &stale, &context).await.is_err());
    assert_eq!(storage.get(&key, &context).await.expect("stale delete preserves bytes"), raw);
    let current = ArtifactCasDeleteFence::new(key.clone(), 1, [7; 32], 3, [8; 32]);
    assert_eq!(storage.delete_if_unreferenced(&key, &current, &context).await.expect("current process fence deletes"), ArtifactCasDeleteOutcome::Deleted);
    std::fs::remove_dir_all(root).expect("remove process fence fixture");
}

#[cfg(unix)]
#[tokio::test]
async fn artifact_chunk_cas_filesystem_rejects_symlinked_space_lock_and_metadata() {
    use std::os::unix::fs::symlink;
    let root = std::env::temp_dir().join(format!("semio-artifact-cas-symlink-{}", std::process::id()));
    let outside = std::env::temp_dir().join(format!("semio-artifact-cas-symlink-outside-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("clean symlink root")
    }
    if outside.exists() {
        std::fs::remove_dir_all(&outside).expect("clean symlink outside")
    }
    std::fs::create_dir_all(&outside).expect("create symlink outside");
    let storage = FsArtifactChunkCasStorage::open(&root).await.expect("open symlink fixture CAS");
    let descriptor_test = open_artifact_cas_leaf(&root.join("descriptor-flags"), true, true).expect("open descriptor flag fixture");
    use std::os::fd::AsRawFd as _;
    unsafe extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    assert_ne!(unsafe { fcntl(descriptor_test.as_raw_fd(), 1) } & 1, 0, "artifact CAS leaf descriptor is close-on-exec");
    drop(descriptor_test);
    let control = control();
    let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
    storage.configure_coordinator([9; 32], &context).await.expect("configure symlink fixture");
    let space_root = root.join(hex_lower(&space_digest("symlink-space").expect("space digest").0));
    symlink(&outside, &space_root).expect("install space symlink");
    assert!(storage.advance_physical_epoch([9; 32], "symlink-space", 1, &context).await.is_err());
    std::fs::remove_file(&space_root).expect("remove space symlink");
    storage.advance_physical_epoch([9; 32], "symlink-space", 1, &context).await.expect("create safe space fence");
    let fence_path = space_root.join("fence-v1");
    std::fs::remove_file(&fence_path).expect("remove safe fence metadata");
    let outside_fence = outside.join("fence-v1");
    std::fs::write(&outside_fence, b"redirected").expect("write outside fence");
    symlink(&outside_fence, &fence_path).expect("install fence symlink");
    assert!(storage.advance_physical_epoch([9; 32], "symlink-space", 2, &context).await.is_err());
    std::fs::remove_file(&fence_path).expect("remove fence symlink");
    let lock_path = space_root.join("fence.lock");
    std::fs::remove_file(&lock_path).expect("remove safe fence lock");
    symlink(&outside_fence, &lock_path).expect("install lock symlink");
    assert!(storage.advance_physical_epoch([9; 32], "symlink-space", 2, &context).await.is_err());
    std::fs::remove_dir_all(root).expect("remove symlink fixture");
    std::fs::remove_dir_all(outside).expect("remove symlink outside");
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn artifact_chunk_cas_sqlite_roundtrip_crosses_legacy_payload_ceiling() {
    let storage = Arc::new(SqliteArtifactChunkCasStorage::memory().await.expect("SQLite CAS"));
    storage_roundtrip_law(storage.clone()).await;
    fenced_delete_law(storage, "sqlite-delete-space").await;
}

#[tokio::test]
async fn artifact_chunk_cas_cancellation_and_max_plus_one_fail_before_storage() {
    let storage = MemoryArtifactChunkCasStorage::default();
    let control = control();
    control.cancelled.store(true, Ordering::SeqCst);
    let context = OperationContext::new(10, AuthorityLimits::maximum(), &control);
    let key = ArtifactCasObjectKey { space_id: "space".into(), kind: ArtifactCasObjectKind::Chunk, digest: ArtifactHash([1; 32]) };
    assert!(matches!(storage.put_if_absent(&key, &[1], &context).await, Err(AuthorityError::Cancelled)));
    assert!(prepare_artifact_cas_manifest_v1("space", &vec![0; AUTHORITY_MAX_PAIR_BYTES as usize + 1]).is_err());
    assert!(artifact_cas_chunk_id_v1("space", &vec![0; ARTIFACT_CAS_CHUNK_BYTES + 1]).is_err());
}
