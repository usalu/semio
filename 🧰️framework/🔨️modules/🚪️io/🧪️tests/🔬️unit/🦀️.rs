//! 🧪️ `io_compose_via`'s own unit test (this file had no prior `#[cfg(test)]` region — this
//! module has no stdio dependency to borrow a real chain from, so this registers a minimal
//! synthetic 2-hop chain through the SAME `register_composer_entries`/`io_dispatch` machinery
//! every real chain (e.g. stdio's png↔deflate↔binary) goes through, proving the mechanism
//! against the real registry rather than a hand-simulated call graph.
use super::*;

const HOP1_FROM: Dialect = Dialect { artifact_kind: "test.io-compose-via.hop1.from", standard: StandardId("1"), subset: SubsetId("*") };
const HOP1_INTO: Dialect = Dialect { artifact_kind: "test.io-compose-via.hop1.into", standard: StandardId("1"), subset: SubsetId("*") };
const HOP2_INTO: Dialect = Dialect { artifact_kind: "test.io-compose-via.hop2.into", standard: StandardId("1"), subset: SubsetId("*") };

async fn hop_text(sources: &[ErasedComposeSource]) -> Result<String, ComposeError> {
    match sources {
        [one] => Ok(match &one.payload {
            IoPayload::Text(t) => t.clone(),
            IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
        }),
        other => Err(ComposeError { message: format!("expected exactly 1 source, got {}", other.len()), diagnostics: Vec::new() }),
    }
}

// 🐛️ terra-io-thunks: these were `async fn .. -> ComposeFuture<'_>` — an async fn ITSELF
// returning the boxed-future type is the "double future" shape R1 bans (calling it produces a
// `Future<Output = ComposeFuture>`, not a `ComposeFuture`); it also could never satisfy
// `AsyncComposeFn` even if it did typecheck, since an async fn's pointer type is unnameable.
// Real `async fn -> Result<ComposedArtifact, ComposeError>` plus `compose_thunk!` at the
// `ComposerEntry` construction sites below is the fix (see that macro's own doc comment).
async fn compose_hop1(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let text = hop_text(sources).await?;
    Ok(ComposedArtifact { dialect: HOP1_INTO, payload: IoPayload::Text(format!("hop1({text})")), diagnostics: Vec::new(), confidence: Confidence::High })
}

async fn compose_hop2(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let text = hop_text(sources).await?;
    Ok(ComposedArtifact { dialect: HOP2_INTO, payload: IoPayload::Text(format!("hop2({text})")), diagnostics: Vec::new(), confidence: Confidence::High })
}

static HOP1_READS: [Dialect; 1] = [HOP1_FROM];
static HOP2_READS: [Dialect; 1] = [HOP1_INTO];

static ENTRIES: [ComposerEntry; 2] = [ComposerEntry { writes: HOP1_INTO, reads: &HOP1_READS, compose: compose_thunk!(compose_hop1) }, ComposerEntry { writes: HOP2_INTO, reads: &HOP2_READS, compose: compose_thunk!(compose_hop2) }];

const CONFLICT_FROM: Dialect = Dialect { artifact_kind: "test.io-registry-conflict.from", standard: StandardId("1"), subset: SubsetId("*") };
const CONFLICT_INTO: Dialect = Dialect { artifact_kind: "test.io-registry-conflict.into", standard: StandardId("1"), subset: SubsetId("*") };
static CONFLICT_READS: [Dialect; 1] = [CONFLICT_FROM];
static CONFLICT_FIRST: ComposerEntry = ComposerEntry { writes: CONFLICT_INTO, reads: &CONFLICT_READS, compose: compose_thunk!(compose_hop1) };
static CONFLICT_SECOND: ComposerEntry = ComposerEntry { writes: CONFLICT_INTO, reads: &CONFLICT_READS, compose: compose_thunk!(compose_hop2) };

/// 🌉️🌉️ hub = HOP1_INTO (resolved directly from the seed source), target = HOP2_INTO
/// (resolved from hub's own composed output alone) — the exact 2-hop shape `io_compose_via`'s
/// doc comment describes, registered and resolved through the real `IO_REGISTRY`.
#[semio_framework_async_macros::async_test]
async fn io_compose_via_chains_two_registered_hops() {
    register_composer_entries(&ENTRIES).expect("register two-hop test entries");
    let hub_key = IoKey::from_owner_counterpart(HOP1_INTO, HOP1_FROM, IoDirection::Import);
    let target_key = IoKey::from_owner_counterpart(HOP2_INTO, HOP1_INTO, IoDirection::Import);
    let sources = [ErasedComposeSource { dialect: HOP1_FROM, payload: IoPayload::Text("seed".to_string()) }];

    let result = resolve_ready(io_compose_via(&hub_key, &target_key, &sources)).expect("2-hop compose over real registered entries should succeed");
    assert_eq!(result.dialect, HOP2_INTO);
    match result.payload {
        IoPayload::Text(t) => assert_eq!(t, "hop2(hop1(seed))"),
        IoPayload::Binary(_) => panic!("expected Text payload"),
    }
}

/// ⚠️ The hub hop itself failing (no registered entry) must surface as the hub's own
/// `ComposeError`, never silently attempt the target hop with stale/absent data.
#[semio_framework_async_macros::async_test]
async fn io_compose_via_surfaces_hub_resolve_failure() {
    let unregistered_hub = IoKey::from_owner_counterpart(Dialect { artifact_kind: "test.io-compose-via.unregistered", standard: StandardId("1"), subset: SubsetId("*") }, HOP1_FROM, IoDirection::Import);
    let target_key = IoKey::from_owner_counterpart(HOP2_INTO, HOP1_INTO, IoDirection::Import);
    let sources = [ErasedComposeSource { dialect: HOP1_FROM, payload: IoPayload::Text("seed".to_string()) }];
    let err = match resolve_ready(io_compose_via(&unregistered_hub, &target_key, &sources)) {
        Err(err) => err,
        Ok(_) => panic!("unregistered hub key must fail hop 1"),
    };
    assert!(err.message.contains("no composer registered"), "{}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry() {
    register_composer_entries(std::slice::from_ref(&CONFLICT_FIRST)).expect("first owner registers");
    assert!(matches!(preflight_composer_entry_refs(&[&CONFLICT_SECOND]).await, Err(IoRegistryRegistrationError::Conflict(_))), "preflight must expose the same conflict before any later assembly mutation");
    let conflict = match register_composer_entries(std::slice::from_ref(&CONFLICT_SECOND)).expect_err("a second owner for the same IO key must fail") {
        IoRegistryRegistrationError::Conflict(conflict) => conflict,
        IoRegistryRegistrationError::Unavailable(error) => panic!("registry unavailable: {error:?}"),
    };
    assert_eq!(conflict.key, IoKey::from_owner_counterpart(CONFLICT_FROM, CONFLICT_INTO, IoDirection::Export));
    let resolved = resolve(&conflict.key).await.expect("first owner remains resolvable");
    assert!(std::ptr::eq(resolved, &CONFLICT_FIRST));
}

async fn format_descriptor_fixture(kind_id: &str, short_id: &str, mimes: &[&str], extensions: &[&str]) -> FormatDescriptor {
    FormatDescriptor {
        kind_id: kind_id.to_string(),
        short_id: short_id.to_string(),
        aliases: Vec::new(),
        mimes: mimes.iter().map(|mime| (*mime).to_string()).collect(),
        extensions: extensions.iter().map(|extension| (*extension).to_string()).collect(),
        name: short_id.to_string(),
        full_name: kind_id.to_string(),
        neutral: true,
        dir_name: short_id.to_string(),
        is_binary: false,
    }
}

#[semio_framework_async_macros::async_test]
async fn format_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims() {
    let txt = format_descriptor_fixture("test.format.txt", "txt", &["text/plain"], &[".txt"]).await;
    let epw = format_descriptor_fixture("test.format.epw", "epw", &[], &[".epw"]).await;
    preflight_format_descriptors(&[txt.clone(), epw.clone()]).await.expect("preflight accepts unclaimed distinct format metadata without mutation");
    assert!(format_descriptor("test.format.epw").expect("catalog availability").is_none(), "preflight must not publish a descriptor");
    register_format_descriptors(vec![txt.clone(), epw.clone()]).await.expect("txt MIME and EPW's absent MIME are unambiguous");
    assert!(format_mimes(&epw).next().is_none());
    assert!(format_mimes(&format_descriptor("test.format.epw").expect("catalog availability").expect("EPW descriptor")).next().is_none());

    let duplicate_step = format_descriptor_fixture("test.format.step-duplicate", "step-duplicate", &["application/step", "APPLICATION/STEP"], &[".step", ".stp", ".STEP"]).await;
    assert!(
        matches!(register_format_descriptors(vec![duplicate_step]).await, Err(FormatRegistryError::Conflict(FormatRegistryConflict::Invalid { .. }))),
        "claims that normalize to the same MIME or extension must reject instead of being silently deduplicated"
    );
    let step = format_descriptor_fixture("test.format.step", "step", &["application/step"], &[".step", ".stp"]).await;
    register_format_descriptors(vec![step]).await.expect("plural representation claims register when each identity is distinct");
    let step = format_descriptor("test.format.step").expect("catalog availability").expect("STEP descriptor");
    assert_eq!(step.mimes, ["application/step"]);
    assert_eq!(step.extensions, [".step", ".stp"]);
    assert_eq!(format_accept_filter(&["test.format.step"]).expect("catalog availability"), ".step,.stp");
    assert!(matches!(format_accept_filter(&["test.format.unknown"]), Err(FormatRegistryError::Unknown { input }) if input == "test.format.unknown"));
    assert!(formats_csv().await.expect("catalog availability").contains("application/step,.step"));

    let first = format_descriptor_fixture("test.format.mime-first", "mime-first", &["application/x-wave0-conflict"], &[".first"]).await;
    let second = format_descriptor_fixture("test.format.mime-second", "mime-second", &["application/x-wave0-conflict"], &[".second"]).await;
    register_format_descriptors(vec![first]).await.expect("first MIME owner registers");
    assert!(matches!(register_format_descriptors(vec![second]).await, Err(FormatRegistryError::Conflict(FormatRegistryConflict::Mime { mime, .. })) if mime == "application/x-wave0-conflict"));
}

#[semio_framework_async_macros::async_test]
async fn codec_budget_enforces_limits_and_shared_cancellation() {
    let cancellation = CancellationToken::new();
    let policy = DecodePolicy { representation: CodecRepresentation::Lossless, limits: CodecLimits { max_read_bytes: 4, max_written_bytes: 4, max_work_units: 2, max_allocations: 1, max_recursion_depth: 1 }, cancellation: cancellation.clone() };
    let mut context = DecodeContext::<TestResolver>::new(policy).await;
    assert_eq!(context.policy.representation, CodecRepresentation::Lossless);
    context.budget.charge_read(4).await.expect("limit edge is allowed");
    assert!(context.budget.charge_work(3).await.is_err(), "work overrun must fail");
    context.budget.charge_allocation(1).await.expect("allocation limit edge is allowed");
    cancellation.cancel().await;
    assert!(context.budget.charge_read(1).await.is_err(), "shared cancellation must stop later work");
}

struct TestPayload {
    bytes: Vec<u8>,
    cursor: usize,
    span: SourceSpan,
}

impl TestPayload {
    async fn new(bytes: &[u8], resource: &str) -> Self {
        Self { bytes: bytes.to_vec(), cursor: 0, span: SourceSpan { resource: resource.to_string(), byte_start: 0, byte_end: bytes.len() as u64, line: Some(1), column: Some(1) } }
    }
}

impl RandomAccessPayload for TestPayload {
    async fn len(&self) -> CodecResult<u64> {
        Ok(CodecOutput { value: self.bytes.len() as u64, diagnostics: Vec::new() })
    }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> CodecResult<usize> {
        let start = match usize::try_from(offset) {
            Ok(start) => start,
            Err(_) => return Err(CodecFailure::error("test.offset", "offset does not fit usize").await),
        };
        let available = match self.bytes.get(start..) {
            Some(available) => available,
            None => &[],
        };
        let count = available.len().min(output.len());
        output[..count].copy_from_slice(&available[..count]);
        Ok(CodecOutput { value: count, diagnostics: Vec::new() })
    }
}

impl PayloadSource for TestPayload {
    type RandomAccess = TestPayload;

    async fn span(&self) -> SourceSpan {
        self.span.clone()
    }

    async fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let available = &self.bytes[self.cursor..];
        let count = available.len().min(output.len());
        output[..count].copy_from_slice(&available[..count]);
        self.cursor += count;
        Ok(CodecOutput { value: count, diagnostics: Vec::new() })
    }

    async fn random_access(&self) -> Option<&Self::RandomAccess> {
        Some(self)
    }
}

struct TestSink;

impl PayloadSink for TestSink {
    async fn write_chunk(&mut self, _input: &[u8]) -> CodecResult<()> {
        Ok(CodecOutput { value: (), diagnostics: Vec::new() })
    }
}

struct TestResolver;

impl ResourceResolver for TestResolver {
    type Source = TestPayload;
    type Sink = TestSink;

    async fn resolve_decode(&self, _request: &ResourceRequest) -> CodecResult<TestPayload> {
        Ok(CodecOutput { value: TestPayload::new(b"resolved", "resolver://decode").await, diagnostics: Vec::new() })
    }

    async fn resolve_encode(&self, _request: &ResourceRequest) -> CodecResult<TestSink> {
        Ok(CodecOutput { value: TestSink, diagnostics: Vec::new() })
    }
}

#[semio_framework_async_macros::async_test]
async fn codec_context_bounds_streaming_random_access_recursion_and_resolved_resources() {
    let limits = CodecLimits { max_read_bytes: 4, max_written_bytes: 4, max_work_units: 16, max_allocations: 4, max_recursion_depth: 1 };
    let resolver = std::sync::Arc::new(TestResolver);
    let mut decode = DecodeContext::with_resolver(DecodePolicy { representation: CodecRepresentation::Lossless, limits: limits.clone(), cancellation: CancellationToken::new() }, resolver.clone()).await;
    let request = ResourceRequest { locator: "resolver://document".to_string(), expected_media_type: None };
    {
        let mut source = decode.resolve(&request).await.expect("decode resource resolves").value;
        let mut output = [0u8; 3];
        assert_eq!(source.read_chunk(&mut output).await.expect("bounded stream read").value, 3);
        let mut random = source.random_access().await.expect("random access is available");
        assert_eq!(random.read_at(0, &mut [0u8; 2]).await.expect("remaining bounded random read").value, 1);
        assert!(random.read_at(0, &mut [0u8; 1]).await.is_err(), "random access cannot bypass the shared read budget");
    }
    decode.budget.enter_recursion().await.expect("first recursion frame");
    assert!(decode.budget.enter_recursion().await.is_err(), "recursion limit is finite");
    decode.budget.leave_recursion().await.expect("leave recursion frame");

    let mut encode = EncodeContext::with_resolver(EncodePolicy { representation: CodecRepresentation::Canonical, limits, cancellation: CancellationToken::new() }, resolver).await;
    let mut sink = encode.resolve(&request).await.expect("encode resource resolves").value;
    sink.write_chunk(b"four").await.expect("bounded write");
    assert!(sink.write_chunk(b"x").await.is_err(), "writes cannot bypass the output budget");
}

#[semio_framework_async_macros::async_test]
async fn resolved_resources_cannot_outlive_their_cancellation_budget() {
    let cancellation = CancellationToken::new();
    let policy = DecodePolicy { representation: CodecRepresentation::Lossless, limits: CodecLimits::default(), cancellation: cancellation.clone() };
    let resolver = std::sync::Arc::new(TestResolver);
    let request = ResourceRequest { locator: "resolver://cancelled".to_string(), expected_media_type: None };
    let mut context = DecodeContext::with_resolver(policy, resolver).await;
    let mut source = context.resolve(&request).await.expect("resolve while active").value;
    cancellation.cancel().await;
    assert!(source.read_chunk(&mut [0u8; 1]).await.is_err(), "the resolved source must be cancellable after resolution");
}

#[semio_framework_async_macros::async_test]
async fn wire_rejects_oversized_and_unbounded_dialect_inputs_before_interning() {
    assert!(matches!(wire_decode_composed_artifact(&vec![b' '; MAX_IO_WIRE_BYTES + 1]).await, Err(IoWireError::Limit { operation: "composed-artifact", .. })));
    let wire = WireComposedArtifact {
        dialect: ArtifactDialect { artifact_kind: "x".repeat(MAX_IO_WIRE_DIALECT_COMPONENT_BYTES + 1), standard: "1".into(), subset: "*".into() },
        payload: IoPayload::Text("payload".into()),
        diagnostics: Vec::new(),
        confidence: Confidence::High,
    };
    let bytes = dsl::os_pack::json::to_json_string(&wire).into_bytes();
    assert!(matches!(wire_decode_composed_artifact(&bytes).await, Err(IoWireError::Limit { operation: "composed-artifact", .. })));
}

#[semio_framework_async_macros::async_test]
async fn codec_result_requires_valid_owned_spans_and_deterministic_opaque_order() {
    let invalid = SourceSpan { resource: String::new(), byte_start: 3, byte_end: 2, line: Some(1), column: None };
    assert!(invalid.validate().await.is_err());
    let result = ArtifactCodecResult {
        semantic: (),
        anchors: vec![AnchoredSyntax { anchor: "root".to_string(), span: SourceSpan { resource: "memory://source".to_string(), byte_start: 0, byte_end: 2, line: Some(1), column: Some(1) }, bytes: b"ok".to_vec() }],
        opaque_extensions: vec![
            OpaqueExtension { kind: "z".to_string(), source: AnchoredSyntax { anchor: "z".to_string(), span: SourceSpan { resource: "memory://source".to_string(), byte_start: 2, byte_end: 3, line: Some(1), column: Some(3) }, bytes: b"z".to_vec() } },
            OpaqueExtension { kind: "a".to_string(), source: AnchoredSyntax { anchor: "a".to_string(), span: SourceSpan { resource: "memory://source".to_string(), byte_start: 3, byte_end: 4, line: Some(1), column: Some(4) }, bytes: b"a".to_vec() } },
        ],
    };
    result.validate_lossless().await.expect("owned anchored result is lossless-valid");
    assert!(result.validate_representation(CodecRepresentation::Canonical).await.is_err(), "a canonical result cannot merely declare a canonical policy while retaining insertion order");
    let mut context = EncodeContext::<TestResolver>::new(EncodePolicy::default()).await;
    let finalized = context.finalize_result(result).await.expect("host finalization canonicalizes an owned result").value;
    assert_eq!(finalized.canonical_opaque_extensions().await.iter().map(|extension| extension.kind.as_str()).collect::<Vec<_>>(), vec!["a", "z"]);
    finalized.validate_representation(CodecRepresentation::Canonical).await.expect("finalized canonical result is executable-policy-valid");
}

/// ✅️ Accept table for `is_canonical_artifact_kind`/`ArtifactKindId::parse`: exactly three
/// dot-separated ASCII segments, first literally `s`, the rest lowercase-kebab.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_id_accepts_canonical_grammar() {
    for kind in ["s.stdio.stl", "s.stdio.semio"] {
        assert!(is_canonical_artifact_kind(kind), "{kind:?} should be canonical");
        ArtifactKindId::parse(kind).unwrap_or_else(|e| panic!("{kind:?} should parse: {e}"));
    }
}

/// ⚠️ Reject table covering: missing `s.` prefix, non-canonical vocabulary, uppercase, emoji,
/// too few/too many segments, empty segment, leading hyphen.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_id_rejects_non_canonical_grammar() {
    for kind in ["stdio.stl", "3d.cad", "data.🧩widget", "s.Stdio.stl", "s.stdio", "s.stdio.stl.extra", "s..stl", "s.stdio.-stl"] {
        assert!(!is_canonical_artifact_kind(kind), "{kind:?} should be rejected");
        assert!(ArtifactKindId::parse(kind).is_err(), "{kind:?} should fail to parse");
    }
}

/// 🔁️ `ArtifactRef::to_uri`/`parse_uri` round-trip, including an artifact id containing dots
/// and dashes (must not be mistaken for dialect-coordinate delimiters since only the FIRST
/// `!` is significant).
#[semio_framework_async_macros::async_test]
async fn artifact_ref_uri_round_trips() {
    let cases = [
        ArtifactRef { artifact_id: "abc123".to_string(), dialect: ArtifactDialect { artifact_kind: "s.stdio.stl".to_string(), standard: "1".to_string(), subset: "*".to_string() } },
        ArtifactRef { artifact_id: "doc.v2-final.draft".to_string(), dialect: ArtifactDialect { artifact_kind: "s.norm.en-1994-1".to_string(), standard: "2024".to_string(), subset: "cc6".to_string() } },
    ];
    for artifact_ref in cases {
        let uri = artifact_ref.to_uri();
        let parsed = ArtifactRef::parse_uri(&uri).unwrap_or_else(|e| panic!("{uri:?} should round-trip: {e}"));
        assert_eq!(parsed, artifact_ref);
    }
}

/// 🔁️ Exact expected shape of `to_uri`, pinned so the format doesn't silently drift.
#[semio_framework_async_macros::async_test]
async fn artifact_ref_to_uri_matches_expected_shape() {
    let artifact_ref = ArtifactRef { artifact_id: "abc123".to_string(), dialect: ArtifactDialect { artifact_kind: "s.stdio.gif".to_string(), standard: "87a".to_string(), subset: "*".to_string() } };
    assert_eq!(artifact_ref.to_uri(), "abc123!s.stdio.gif@87a/*");
}

/// ⚠️ `parse_uri` rejects a missing `!` and an empty artifact id, mirroring
/// `parse_coordinate`'s own empty-component rejection.
#[semio_framework_async_macros::async_test]
async fn artifact_ref_parse_uri_rejects_malformed_input() {
    assert!(ArtifactRef::parse_uri("s.stdio.gif@87a/*").is_err(), "missing '!' should fail");
    assert!(ArtifactRef::parse_uri("!s.stdio.gif@87a/*").is_err(), "empty artifact id should fail");
}
