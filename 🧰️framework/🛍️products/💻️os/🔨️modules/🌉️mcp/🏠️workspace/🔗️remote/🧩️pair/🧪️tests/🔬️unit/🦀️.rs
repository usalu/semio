
use super::*;
use crate::workspace::remote::{AuthorizedDescriptorSnapshot, AuthorizedDocumentView, HubStreamObservation};
use semio_framework_async::TraceId;
use semio_framework_os_kernel::os_directory::{DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentOwner, DocumentView, MemberSpaceViewV1, MemberView, RebootstrapRequired};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::{Arc, Mutex};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("canonical pair cache fixture")
}

fn decode_hex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("fixture hex"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn context(deadline_ms: u64) -> OperationContext {
    OperationContext { actor: 7, generation: 1, trace: TraceId(9), lane: 1, deadline_ms: Some(deadline_ms), cancel: CancelToken::root_now(), capability: None }
}

fn ready_binding(expires_at_ms: i64) -> Arc<HubRemoteBinding> {
    let contract = fixture();
    let scope = DocumentScope::new(contract["binding"]["spaceId"].as_str().unwrap(), contract["binding"]["documentId"].as_str().unwrap());
    let digest = contract["binding"]["descriptorDigest"].as_str().unwrap().to_string();
    let view = DocumentView {
        descriptor: DocumentDescriptor {
            space_id: scope.space_id.clone(),
            document_id: scope.document_id.clone(),
            artifact_kind: "note.document".into(),
            artifact_schema: "note.document@1".into(),
            owner: DocumentOwner { plugin_id: "note".into(), package_id: "note".into(), version: "1".into(), package_hash: "44".repeat(32) },
            pack_schema_hash: "55".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 1 },
            bootstrap_snapshot_hash: "66".repeat(32),
        },
        head_seq: 0,
        commit_seq: 0,
        epoch: 1,
    };
    let authorized = AuthorizedDocumentView { scope: scope.clone(), descriptor_digest_v1: digest, view };
    let membership = MemberView { user_id: "user-a".into(), email: "a@example.invalid".into(), display_name: "A".into(), role: DirectorySpaceRole::Author };
    let snapshot = AuthorizedDescriptorSnapshot {
        authenticated_user_id: membership.user_id.clone(),
        session_expires_at_ms: expires_at_ms,
        space: MemberSpaceViewV1 {
            id: scope.space_id.clone(),
            name: "A".into(),
            kind: DirectorySpaceKind::Studio,
            visibility: DirectorySpaceVisibility::Private,
            owner_user_id: membership.user_id.clone(),
            role: membership.role,
            member_count: 1,
            document_count: 1,
            active_connections: 0,
            created_at_ms: 1,
            updated_at_ms: 1,
        },
        membership,
        observed_event_seq: 0,
        documents: HashMap::from([(scope, authorized)]),
    };
    let binding = Arc::new(HubRemoteBinding::new("https://HUB.invalid/", contract["binding"]["spaceId"].as_str().unwrap()).unwrap());
    binding.install_snapshot_for_test(snapshot);
    binding
}

fn frame_ranges(wire: &[u8]) -> Result<Vec<(usize, usize, usize)>, CanonicalPairMountError> {
    let mut cursor = 0usize;
    let mut ranges = Vec::new();
    while cursor < wire.len() {
        let length_bytes: [u8; 4] = wire.get(cursor..cursor + 4).ok_or(CanonicalPairMountError::InvalidResponse("frame length"))?.try_into().unwrap();
        let length = u32::from_be_bytes(length_bytes) as usize;
        let end = cursor.checked_add(4).and_then(|value| value.checked_add(length)).ok_or(CanonicalPairMountError::ResourceLimit)?;
        if length == 0 || end > wire.len() {
            return Err(CanonicalPairMountError::InvalidResponse("frame payload"));
        }
        ranges.push((cursor, cursor + 4, end));
        cursor = end;
    }
    Ok(ranges)
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> usize {
    haystack.windows(needle.len()).position(|window| window == needle).expect("fixture mutation target")
}

fn mutated(name: &str, valid: &[u8], contract: &serde_json::Value) -> (Vec<u8>, String) {
    let ranges = frame_ranges(valid).unwrap();
    let mut wire = valid.to_vec();
    match name {
        "truncated" => {
            wire.pop();
        }
        "reordered" => wire = [valid[ranges[0].0..ranges[0].2].to_vec(), valid[ranges[2].0..ranges[2].2].to_vec(), valid[ranges[1].0..ranges[1].2].to_vec(), valid[ranges[3].0..].to_vec()].concat(),
        "duplicate" => wire.splice(ranges[2].0..ranges[2].0, valid[ranges[1].0..ranges[1].2].iter().copied()).for_each(drop),
        "oversize-record" => wire[ranges[1].1 + 14..ranges[1].1 + 18].copy_from_slice(&4_097u32.to_be_bytes()),
        "wrong-scope" | "same-document-other-space" => {
            let index = find_bytes(&wire, b"space:alpha") + 6;
            wire[index] ^= if name == "wrong-scope" { 1 } else { 2 };
        }
        "wrong-digest" => {
            let digest = decode_hex(contract["binding"]["descriptorDigest"].as_str().unwrap());
            let index = find_bytes(&wire, &digest);
            wire[index] ^= 1;
        }
        "wrong-checkpoint" => {
            let checkpoint = decode_hex(contract["binding"]["checkpointId"].as_str().unwrap());
            let index = find_bytes(&wire, &checkpoint);
            wire[index..index + 32].fill(0);
        }
        "malformed-utf8" => {
            let index = find_bytes(&wire, b"edit:7");
            wire[index] = 0xff;
        }
        "control-character" => {
            let index = find_bytes(&wire, b"edit:7");
            wire[index] = 0x01;
        }
        "bad-pack-hash" => {
            let hash = decode_hex(contract["valid"]["packSha256"].as_str().unwrap());
            let index = find_bytes(&wire, &hash);
            wire[index] ^= 1;
        }
        "bad-aggregate" => {
            let hash = decode_hex(contract["valid"]["aggregateSha256"].as_str().unwrap());
            let index = find_bytes(&wire, &hash);
            wire[index] ^= 1;
        }
        "bad-etag" => {}
        "missing-terminal" => wire.truncate(ranges.last().unwrap().0),
        "trailing-data" => wire.push(0),
        _ => panic!("unknown fixture mutation {name}"),
    }
    let changed_ranges = frame_ranges(&wire).ok();
    let etag = if name == "bad-etag" {
        format!("\"{}\"", "0".repeat(64))
    } else if let Some(changed) = changed_ranges.as_ref().and_then(|frames| frames.first()) {
        canonical_etag(&wire[changed.1..changed.2])
    } else {
        contract["valid"]["etag"].as_str().unwrap().to_string()
    };
    (wire, etag)
}

struct TestBody {
    bytes: Vec<u8>,
    offset: usize,
    delay_ms: u64,
    cancel_on_read: bool,
    reads: Arc<AtomicUsize>,
    wiped: Arc<AtomicU64>,
}

impl Drop for TestBody {
    fn drop(&mut self) {
        let length = self.bytes.len();
        self.bytes.fill(0);
        self.wiped.fetch_add(length as u64, Ordering::SeqCst);
    }
}

impl CanonicalPairBody for TestBody {
    async fn read(&mut self, context: &OperationContext, output: &mut [u8]) -> Result<usize, CanonicalPairMountError> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        if self.delay_ms != 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;
        }
        if self.cancel_on_read {
            context.cancel.cancel_now();
        }
        if context.cancel.is_cancelled_now() {
            return Err(CanonicalPairMountError::Cancelled);
        }
        let length = output.len().min(self.bytes.len().saturating_sub(self.offset));
        output[..length].copy_from_slice(&self.bytes[self.offset..self.offset + length]);
        self.offset += length;
        Ok(length)
    }
}

struct ResponseSpec {
    status: u16,
    content_type: String,
    etag: String,
    declared_length: u64,
    body: Vec<u8>,
    delay_ms: u64,
    cancel_on_read: bool,
}

struct TestTransport {
    responses: Mutex<VecDeque<ResponseSpec>>,
    requests: Mutex<Vec<CanonicalPairFetchRequest>>,
    reads: Arc<AtomicUsize>,
    wiped: Arc<AtomicU64>,
}

impl TestTransport {
    fn new(responses: Vec<ResponseSpec>) -> Self {
        Self { responses: Mutex::new(responses.into()), requests: Mutex::new(Vec::new()), reads: Arc::new(AtomicUsize::new(0)), wiped: Arc::new(AtomicU64::new(0)) }
    }
}

impl CanonicalPairTransport for TestTransport {
    type Body = TestBody;

    async fn fetch(&self, _context: &OperationContext, request: &CanonicalPairFetchRequest) -> Result<CanonicalPairHttpResponse<Self::Body>, CanonicalPairMountError> {
        self.requests.lock().unwrap().push(request.clone());
        let response = self.responses.lock().unwrap().pop_front().ok_or(CanonicalPairMountError::Unavailable)?;
        Ok(CanonicalPairHttpResponse {
            status: response.status,
            content_type: response.content_type,
            etag: response.etag,
            content_length: response.declared_length,
            body: TestBody { bytes: response.body, offset: 0, delay_ms: response.delay_ms, cancel_on_read: response.cancel_on_read, reads: self.reads.clone(), wiped: self.wiped.clone() },
        })
    }
}

fn response(wire: Vec<u8>, etag: String) -> ResponseSpec {
    ResponseSpec { status: 200, content_type: HUB_PAIR_MEDIA_TYPE.into(), etag, declared_length: wire.len() as u64, body: wire, delay_ms: 0, cancel_on_read: false }
}

#[tokio::test]
async fn canonical_pair_neutral_receiver_rejects_all_malformed_vectors_and_wipes_candidates() {
    let contract = fixture();
    let wire = decode_hex(contract["valid"]["wireHex"].as_str().unwrap());
    let scope = DocumentScope::new(contract["binding"]["spaceId"].as_str().unwrap(), contract["binding"]["documentId"].as_str().unwrap());
    let descriptor = contract["binding"]["descriptorDigest"].as_str().unwrap();
    let ctx = context(10_000);
    let identity = decode_fixture_for_test(wire.clone(), &scope, descriptor, contract["valid"]["etag"].as_str().unwrap(), &ctx).unwrap();
    assert_eq!(identity.scope, scope);
    assert_eq!(identity.active_checkpoint_id, contract["binding"]["checkpointId"].as_str().unwrap());
    let before = test_wiped_bytes();
    for name in contract["negativeVectors"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()) {
        let (candidate, etag) = mutated(name, &wire, &contract);
        assert!(decode_fixture_for_test(candidate, &scope, descriptor, &etag, &ctx).is_err(), "negative vector {name} must fail closed");
    }
    assert!(test_wiped_bytes() > before, "malformed wire and partial part candidates must be wiped on every error path");

    let edit = find_bytes(&wire, b"edit:7");
    let length = edit - 4;
    let ordinal = length - 8;
    let mut initial = wire.clone();
    initial[ordinal..ordinal + 8].fill(0);
    initial[length..length + 4].copy_from_slice(&0u32.to_be_bytes());
    initial.drain(edit..edit + 6);
    initial[length + 4..length + 12].fill(0);
    initial[length + 12..length + 44].fill(0);
    let header_length = u32::from_be_bytes(initial[0..4].try_into().unwrap()) - 6;
    initial[0..4].copy_from_slice(&header_length.to_be_bytes());
    let ranges = frame_ranges(&initial).unwrap();
    let initial_etag = canonical_etag(&initial[ranges[0].1..ranges[0].2]);
    assert!(decode_fixture_for_test(initial.clone(), &scope, descriptor, &initial_etag, &ctx).is_ok(), "exact scope-bound genesis baseline must decode");
    for (name, offset, value) in [("genesis-ordinal", ordinal, 1u64), ("genesis-commit", length + 4, 1u64)] {
        let mut partial = initial.clone();
        partial[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
        let ranges = frame_ranges(&partial).unwrap();
        let partial_etag = canonical_etag(&partial[ranges[0].1..ranges[0].2]);
        assert!(decode_fixture_for_test(partial, &scope, descriptor, &partial_etag, &ctx).is_err(), "partial frontier {name} must fail before pair allocation");
    }
    let mut partial_chain = initial;
    partial_chain[length + 12] = 1;
    let ranges = frame_ranges(&partial_chain).unwrap();
    let partial_etag = canonical_etag(&partial_chain[ranges[0].1..ranges[0].2]);
    assert!(decode_fixture_for_test(partial_chain, &scope, descriptor, &partial_etag, &ctx).is_err(), "genesis with a nonzero chain must fail before pair allocation");
}

#[tokio::test]
async fn canonical_pair_actor_keys_cache_and_mount_to_one_binding_and_evicts_by_fixed_credits() {
    let contract = fixture();
    let valid = decode_hex(contract["valid"]["wireHex"].as_str().unwrap());
    let valid_etag = contract["valid"]["etag"].as_str().unwrap().to_string();
    let scope = DocumentScope::new(contract["binding"]["spaceId"].as_str().unwrap(), contract["binding"]["documentId"].as_str().unwrap());
    let binding = ready_binding(i64::MAX);
    let transport = TestTransport::new(vec![response(valid.clone(), valid_etag)]);
    let ctx = context(10_000);
    let first = binding.mount_canonical_pair(&transport, &scope, Some(9), None, &ctx, 1, 1).await.unwrap();
    assert_eq!(first.identity().hub_origin, "https://hub.invalid");
    assert_eq!(first.baseline().document_id, scope.document_id);
    assert_eq!(
        transport.requests.lock().unwrap().as_slice(),
        &[CanonicalPairFetchRequest { hub_origin: "https://hub.invalid".into(), path: "/spaces/space%3Aalpha/documents/doc%3Atokyo/active-checkpoint/pair".into(), accept: HUB_PAIR_MEDIA_TYPE, maximum_response_bytes: maximum_wire_bytes() }]
    );
    let cached = binding.mount_canonical_pair(&transport, &scope, Some(9), Some(first.identity()), &ctx, 1, 1).await.unwrap();
    assert_eq!(cached.opaque_id(), first.opaque_id());
    assert_eq!(transport.requests.lock().unwrap().len(), 1, "an exact full-identity hit must not reach HTTP");
    let (_, loadings, entries, bytes, hits, misses) = binding.canonical_pair_test_stats();
    assert_eq!((loadings, entries, bytes, hits, misses), (0, 1, 8, 1, 1));

    let mut responses = Vec::new();
    for value in 3u8..8 {
        let mut next = valid.clone();
        let checkpoint = decode_hex(contract["binding"]["checkpointId"].as_str().unwrap());
        let checkpoint_offset = find_bytes(&next, &checkpoint);
        next[checkpoint_offset] = value;
        let ranges = frame_ranges(&next).unwrap();
        let next_etag = canonical_etag(&next[ranges[0].1..ranges[0].2]);
        responses.push(response(next, next_etag));
    }
    let eviction_transport = TestTransport::new(responses);
    for _ in 0..5 {
        binding.mount_canonical_pair(&eviction_transport, &scope, Some(9), None, &ctx, 1, 1).await.unwrap();
    }
    let (_, _, entries, bytes, _, misses) = binding.canonical_pair_test_stats();
    assert_eq!((entries, bytes, misses), (HUB_PAIR_CACHE_MAX_ENTRIES, 32, 6));

    let other = ready_binding(i64::MAX);
    assert_ne!(binding.authority_generation.load(Ordering::SeqCst), other.authority_generation.load(Ordering::SeqCst));
    assert!(matches!(other.mount_canonical_pair(&TestTransport::new(Vec::new()), &scope, Some(9), Some(first.identity()), &ctx, 1, 1).await, Err(CanonicalPairMountError::InvalidResponse(_))));
    binding.revoke(HubBindingError::MembershipRequired);
    let (state, loadings, entries, bytes, _, _) = binding.canonical_pair_test_stats();
    assert_eq!(state, CanonicalPairActorState::Revoked);
    assert_eq!((loadings, entries, bytes), (0, 0, 0));
}

#[tokio::test]
async fn authenticated_hub_checkpoint_resource_projects_exact_verified_pair_and_never_crosses_scope() {
    let contract = fixture();
    let resource_contract: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔐️canonical-checkpoint-resource/🔣️.json")).unwrap();
    let wire = decode_hex(contract["valid"]["wireHex"].as_str().unwrap());
    let etag = contract["valid"]["etag"].as_str().unwrap().to_string();
    let scope = DocumentScope::new(contract["binding"]["spaceId"].as_str().unwrap(), contract["binding"]["documentId"].as_str().unwrap());
    assert_eq!(super::super::checkpoint_resource_uri(&scope), resource_contract["resource"]["uri"]);
    assert_eq!(super::super::parse_checkpoint_resource_uri(resource_contract["resource"]["uri"].as_str().unwrap()), Some(scope.clone()));

    let binding = ready_binding(i64::MAX);
    let transport = TestTransport::new(vec![response(wire.clone(), etag.clone())]);
    let mount = binding.mount_canonical_pair(&transport, &scope, Some(9), None, &context(10_000), 1, 1).await.unwrap();
    let text = binding.project_mounted_canonical_pair(&mount, 1, super::super::canonical_checkpoint_resource_text).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&text).unwrap();
    let mut expected = resource_contract["resource"]["value"].clone();
    expected["authorityGeneration"] = mount.identity.authority_generation.into();
    assert_eq!(actual, expected);
    let pack = decode_hex(contract["valid"]["packHex"].as_str().unwrap());
    let spr = decode_hex(contract["valid"]["sprHex"].as_str().unwrap());
    assert_eq!(actual["pack"]["base64"], super::super::base64_encode_exact(&pack, super::super::checked_base64_length(pack.len()).unwrap()).unwrap());
    assert_eq!(actual["spr"]["base64"], super::super::base64_encode_exact(&spr, super::super::checked_base64_length(spr.len()).unwrap()).unwrap());

    let other_scope = DocumentScope::new(contract["binding"]["sameDocumentOtherSpace"].as_str().unwrap(), scope.document_id.clone());
    let denied_transport = TestTransport::new(Vec::new());
    assert_eq!(binding.mount_canonical_pair(&denied_transport, &other_scope, Some(9), None, &context(10_000), 1, 1).await.unwrap_err(), CanonicalPairMountError::DescriptorUnavailable);
    assert!(denied_transport.requests.lock().unwrap().is_empty(), "cross-space scope must fail before transport or body allocation");

    for revoke in [false, true] {
        let binding = ready_binding(i64::MAX);
        let transport = TestTransport::new(vec![response(wire.clone(), etag.clone())]);
        let mount = binding.mount_canonical_pair(&transport, &scope, Some(9), None, &context(10_000), 1, 1).await.unwrap();
        let reached = Arc::new(std::sync::Barrier::new(2));
        let release = Arc::new(std::sync::Barrier::new(2));
        binding.pause_next_canonical_pair_mount_return(reached.clone(), release.clone());
        let contender = binding.clone();
        let projected = std::thread::spawn(move || contender.project_mounted_canonical_pair(&mount, 1, super::super::canonical_checkpoint_resource_text));
        reached.wait();
        let wiped_before = test_wiped_bytes();
        if revoke {
            binding.revoke(HubBindingError::MembershipRequired);
        } else {
            binding.invalidate("descriptor refresh began before checkpoint resource publication");
        }
        release.wait();
        assert_eq!(projected.join().unwrap().unwrap_err(), CanonicalPairMountError::StaleCompletion);
        assert!(test_wiped_bytes() >= wiped_before + pack.len() as u64 + spr.len() as u64, "invalidated retained pair bytes must be wiped before no resource content is published");
    }

    let identity = CanonicalPairMountIdentity {
        hub_origin: "https://hub.invalid".into(),
        authority_generation: 7,
        scope: scope.clone(),
        descriptor_digest_v1: contract["binding"]["descriptorDigest"].as_str().unwrap().into(),
        active_checkpoint_id: contract["binding"]["checkpointId"].as_str().unwrap().into(),
        etag,
        catalog_generation: Some(9),
    };
    let frontier = ArtifactFrontier { document_id: scope.document_id, head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: ArtifactHash::new([0; 32]) };
    assert_eq!(super::super::canonical_checkpoint_resource_text(&identity, &frontier, &vec![0; HUB_PAIR_MAX_VERIFIED_BYTES + 1], &[]).unwrap_err(), CanonicalPairMountError::ResourceLimit);
}

#[tokio::test]
async fn canonical_pair_cache_hit_never_returns_after_binding_revocation() {
    let contract = fixture();
    let wire = decode_hex(contract["valid"]["wireHex"].as_str().unwrap());
    let etag = contract["valid"]["etag"].as_str().unwrap().to_string();
    let scope = DocumentScope::new(contract["binding"]["spaceId"].as_str().unwrap(), contract["binding"]["documentId"].as_str().unwrap());
    let binding = ready_binding(i64::MAX);
    let transport = TestTransport::new(vec![response(wire, etag)]);
    let mounted = binding.mount_canonical_pair(&transport, &scope, Some(9), None, &context(10_000), 1, 1).await.unwrap();
    let expected = mounted.identity().clone();
    let reached = Arc::new(std::sync::Barrier::new(2));
    let release = Arc::new(std::sync::Barrier::new(2));
    binding.pause_next_canonical_pair_mount_return(reached.clone(), release.clone());
    let contender = binding.clone();
    let thread = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        runtime.block_on(contender.mount_canonical_pair(&TestTransport::new(Vec::new()), &scope, Some(9), Some(&expected), &context(10_000), 1, 1))
    });
    reached.wait();
    binding.revoke(HubBindingError::MembershipRequired);
    release.wait();
    assert_eq!(thread.join().unwrap().unwrap_err(), CanonicalPairMountError::StaleCompletion);
    assert_eq!(binding.canonical_pair_test_stats().0, CanonicalPairActorState::Revoked);
}

#[tokio::test]
async fn canonical_pair_receipt_preflights_streams_cancels_expires_and_never_resurrects_after_invalidation() {
    let contract = fixture();
    let wire = decode_hex(contract["valid"]["wireHex"].as_str().unwrap());
    let etag = contract["valid"]["etag"].as_str().unwrap().to_string();
    let scope = DocumentScope::new(contract["binding"]["spaceId"].as_str().unwrap(), contract["binding"]["documentId"].as_str().unwrap());
    let ctx = context(10_000);

    let binding = ready_binding(i64::MAX);
    let mut oversized = response(Vec::new(), etag.clone());
    oversized.declared_length = maximum_wire_bytes() as u64 + 1;
    let transport = TestTransport::new(vec![oversized]);
    assert_eq!(binding.mount_canonical_pair(&transport, &scope, None, None, &ctx, 1, 1).await.unwrap_err(), CanonicalPairMountError::ResourceLimit);
    assert_eq!(transport.reads.load(Ordering::SeqCst), 0, "Content-Length must be refused before a body read or pair allocation");

    let binding = ready_binding(i64::MAX);
    let mut cancelled = response(wire.clone(), etag.clone());
    cancelled.cancel_on_read = true;
    let transport = TestTransport::new(vec![cancelled]);
    assert_eq!(binding.mount_canonical_pair(&transport, &scope, None, None, &ctx, 1, 1).await.unwrap_err(), CanonicalPairMountError::Cancelled);
    assert_eq!(binding.canonical_pair_test_stats().0, CanonicalPairActorState::DescriptorReady);
    assert!(transport.wiped.load(Ordering::SeqCst) >= wire.len() as u64);

    let binding = ready_binding(i64::MAX);
    let mut delayed = response(wire.clone(), etag.clone());
    delayed.delay_ms = 5;
    let transport = TestTransport::new(vec![delayed]);
    assert_eq!(binding.mount_canonical_pair(&transport, &scope, None, None, &context(2), 1, 1).await.unwrap_err(), CanonicalPairMountError::DeadlineExceeded);
    assert_eq!(binding.canonical_pair_test_stats().1, 0);

    let binding = ready_binding(2);
    let mut expiring = response(wire.clone(), etag.clone());
    expiring.delay_ms = 5;
    let transport = TestTransport::new(vec![expiring]);
    assert_eq!(binding.mount_canonical_pair(&transport, &scope, None, None, &context(10_000), 1, 1).await.unwrap_err(), CanonicalPairMountError::StaleCompletion);
    let (state, loadings, entries, bytes, _, _) = binding.canonical_pair_test_stats();
    assert_eq!(state, CanonicalPairActorState::Revoked);
    assert_eq!((loadings, entries, bytes), (0, 0, 0));

    let binding = ready_binding(i64::MAX);
    let authority = binding.authority_generation.load(Ordering::SeqCst);
    let expected = CanonicalPairMountIdentity {
        hub_origin: "https://hub.invalid".into(),
        authority_generation: authority,
        scope: scope.clone(),
        descriptor_digest_v1: contract["binding"]["descriptorDigest"].as_str().unwrap().into(),
        active_checkpoint_id: contract["binding"]["checkpointId"].as_str().unwrap().into(),
        etag: etag.clone(),
        catalog_generation: Some(9),
    };
    let owner = match binding.pair_actor.lock().unwrap().begin(authority, &scope, Some(&expected), &ctx.cancel).unwrap() {
        PairBegin::Owner(owner) => owner,
        PairBegin::Join(_) => panic!("first receipt owns"),
    };
    let joined = match binding.pair_actor.lock().unwrap().begin(authority, &scope, Some(&expected), &ctx.cancel).unwrap() {
        PairBegin::Join(joined) => joined,
        PairBegin::Owner(_) => panic!("equal receipt joins"),
    };
    binding.invalidate_stream();
    assert_eq!(wait_for_equal_receipt(joined, &ctx, 1, std::time::Instant::now()).await.unwrap_err(), CanonicalPairMountError::StaleCompletion);
    binding.pair_actor.lock().unwrap().cancel_receipt(&owner);
    assert_eq!(binding.canonical_pair_test_stats().0, CanonicalPairActorState::Refreshing, "stale cleanup must not resurrect DescriptorReady after invalidation");
    assert_eq!(binding.canonical_pair_test_stats().1, 0);

    let binding = ready_binding(i64::MAX);
    let control = RebootstrapRequired {
        scope: scope.clone(),
        checkpoint_id: ArtifactHash::new([2; 32]),
        descriptor_digest_v1: ArtifactHash::new([1; 32]),
        baseline_frontier: ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: ArtifactHash::new([3; 32]) },
    };
    assert_eq!(binding.observe_stream_message(&semio_framework_os_kernel::os_directory::DirectoryStreamMessage::RebootstrapRequired { control }), HubStreamObservation::RefreshRequired);
    assert_eq!(binding.canonical_pair_test_stats().0, CanonicalPairActorState::Refreshing);
}
