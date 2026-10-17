//! 🧪️ Original finite codec streaming, cancellation and lossless ownership laws.
use super::*;

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

#[test]
fn neutral_contract_matches_independent_syn_and_json() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).expect("complete neutral codec contract");
    let parsed=syn::parse_file(include_str!("../🦀️.rs")).expect("original owned Rust codec syntax");
    let names=parsed.items.into_iter().filter_map(|item|match item{syn::Item::Struct(item)=>Some(item.ident.to_string()),syn::Item::Enum(item)=>Some(item.ident.to_string()),syn::Item::Trait(item)=>Some(item.ident.to_string()),syn::Item::Type(item)=>Some(item.ident.to_string()),_=>None}).collect::<std::collections::BTreeSet<_>>();
    let expected=corpus["definitions"].as_array().expect("owned definitions").iter().map(|value|value.as_str().expect("definition name").to_string()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(names,expected);
    semio_framework_async::poll::resolve_ready(async{
        for row in corpus["budgets"].as_array().expect("finite budget cases"){
            let limit=row["limit"].as_u64().unwrap();let mut budget=CodecBudget::new(CodecLimits{max_read_bytes:limit,..CodecLimits::default()},CancellationToken::new()).await;
            budget.charge_read(row["used"].as_u64().unwrap()).await.expect("original issued usage");
            assert_eq!(budget.charge_read(row["increment"].as_u64().unwrap()).await.is_ok(),row["admitted"].as_bool().unwrap());
            assert_eq!(budget.consumption().await.read_bytes,row["result"].as_u64().unwrap());
        }
    });
}
