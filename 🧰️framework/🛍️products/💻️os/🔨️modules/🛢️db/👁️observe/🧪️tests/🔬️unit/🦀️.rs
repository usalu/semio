use super::*;

async fn hash(byte: u8) -> ContentHash {
    ContentHash([byte; 32])
}

//#region 🔖️Json
#[semio_framework_async_macros::async_test]
async fn encode_emit_event_json_escapes_and_shapes_fields() {
    let event = EmitEvent::new("wal.append").with_document(ArtifactId::from("doc\"1")).field("bytes", EmitField::U64(42)).field("ok", EmitField::Bool(true)).field("note", EmitField::Text("line\nbreak".to_string()));

    let json = encode_emit_event_json(&event);
    assert!(json.starts_with("{\"name\":\"wal.append\""));
    assert!(json.contains("\"document\":\"doc\\\"1\""));
    assert!(json.contains("\"bytes\":42"));
    assert!(json.contains("\"ok\":true"));
    assert!(json.contains("\"note\":\"line\\nbreak\""));
    assert!(json.ends_with("}}"));
}

#[semio_framework_async_macros::async_test]
async fn encode_emit_event_json_never_emits_nan_or_infinity_literals() {
    let event = EmitEvent::new("x").field("v", EmitField::F64(f64::NAN));
    let json = encode_emit_event_json(&event);
    assert!(json.contains("\"v\":null"));
    assert!(!json.contains("NaN"));
}
//#endregion 🔖️Json

//#region 🔖️Sink
#[semio_framework_async_macros::async_test]
async fn writer_sink_appends_newline_terminated_lines() {
    let sink = WriterSink::new(Vec::<u8>::new());
    sink.write_line("a").await.unwrap();
    sink.write_line("b").await.unwrap();
    let bytes = lock(&sink.writer).clone();
    assert_eq!(String::from_utf8(bytes).unwrap(), "a\nb\n");
}

struct FailingSink;
impl EventSink for FailingSink {
    async fn write_line(&self, _line: &str) -> Result<(), DbError> {
        Err(DbError::Io("disk full".to_string()))
    }
}
//#endregion 🔖️Sink

//#region 🔖️Structured
#[semio_framework_async_macros::async_test]
async fn structured_sink_writes_one_json_line_per_event() {
    let memory = MemorySink::new();
    let structured = StructuredSink::new(memory);
    structured.emit(EmitEvent::new("doc.commit").field("seq", EmitField::U64(7))).await;
    let lines = structured.sink.lines().await;
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("\"name\":\"doc.commit\""));
    assert_eq!(structured.failed_writes().await, 0);
}

#[semio_framework_async_macros::async_test]
async fn structured_sink_counts_failed_writes_instead_of_dropping_silently_or_panicking() {
    let structured = StructuredSink::new(FailingSink);
    structured.emit(EmitEvent::new("x")).await;
    structured.emit(EmitEvent::new("y")).await;
    assert_eq!(structured.failed_writes().await, 2);
}
//#endregion 🔖️Structured

//#region 🔖️Audit
#[semio_framework_async_macros::async_test]
async fn audit_sink_verify_chain_accepts_an_untampered_log() {
    let memory = MemorySink::new();
    let audit = AuditSink::new(memory, 100).await;
    for i in 0..5u64 {
        audit.emit(EmitEvent::new("audit.write").field("seq", EmitField::U64(i))).await;
    }
    let lines = audit.sink.lines().await;
    assert_eq!(lines.len(), 5);
    assert_eq!(audit.chain().await.len(), 5);
    assert!(audit.verify_chain(&lines).await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn audit_sink_verify_chain_detects_a_single_tampered_line() {
    let memory = MemorySink::new();
    let audit = AuditSink::new(memory, 100).await;
    for i in 0..5u64 {
        audit.emit(EmitEvent::new("audit.write").field("seq", EmitField::U64(i))).await;
    }
    let mut lines = audit.sink.lines().await;
    lines[2] = lines[2].replace("\"seq\":2", "\"seq\":999");
    let err = audit.verify_chain(&lines).await.unwrap_err();
    match err {
        DbError::Corrupt(message) => assert!(message.contains("seq 2")),
        other => panic!("expected Corrupt, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn audit_sink_bounded_retention_still_verifies_the_surviving_window() {
    let memory = MemorySink::new();
    let audit = AuditSink::new(memory, 2).await;
    for i in 0..5u64 {
        audit.emit(EmitEvent::new("audit.write").field("seq", EmitField::U64(i))).await;
    }
    let all_lines = audit.sink.lines().await;
    assert_eq!(all_lines.len(), 5);
    let chain = audit.chain().await;
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].seq, 3);
    assert_eq!(chain[1].seq, 4);
    let retained_lines = all_lines[3..].to_vec();
    assert!(audit.verify_chain(&retained_lines).await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn audit_sink_does_not_chain_a_failed_write() {
    let audit = AuditSink::new(FailingSink, 10).await;
    audit.emit(EmitEvent::new("x")).await;
    assert_eq!(audit.failed_writes().await, 1);
    assert!(audit.chain().await.is_empty());
}
//#endregion 🔖️Audit

//#region 🔖️Cardinality
#[semio_framework_async_macros::async_test]
async fn cardinality_limiter_admits_up_to_the_limit_then_collapses_to_overflow() {
    let limiter = CardinalityLimiter::new(2).await;
    let a = limiter.admit("m", Labels::new([("k", "a".to_string())])).await;
    let b = limiter.admit("m", Labels::new([("k", "b".to_string())])).await;
    let c = limiter.admit("m", Labels::new([("k", "c".to_string())])).await;
    assert_eq!(a, Labels::new([("k", "a".to_string())]));
    assert_eq!(b, Labels::new([("k", "b".to_string())]));
    assert_eq!(c, Labels::new([("cardinality", "overflow".to_string())]));
    assert_eq!(limiter.series_count("m").await, 2);

    // 🔒️ a previously-admitted series stays itself, never gets swept into overflow later.
    let a_again = limiter.admit("m", Labels::new([("k", "a".to_string())])).await;
    assert_eq!(a_again, Labels::new([("k", "a".to_string())]));
}

#[semio_framework_async_macros::async_test]
async fn labels_canonicalize_insertion_order() {
    let l1 = Labels::new([("b", "2".to_string()), ("a", "1".to_string())]);
    let l2 = Labels::new([("a", "1".to_string()), ("b", "2".to_string())]);
    assert_eq!(l1, l2);
}
//#endregion 🔖️Cardinality

//#region 🔖️Metrics
#[semio_framework_async_macros::async_test]
async fn metric_registry_counter_accumulates_per_series() {
    let metrics = MetricRegistry::new(16).await;
    metrics.incr_counter("cmd.count", Labels::none().await, 1).await;
    metrics.incr_counter("cmd.count", Labels::none().await, 4).await;
    assert_eq!(metrics.counter_value("cmd.count", &Labels::none().await).await, 5);
}

#[semio_framework_async_macros::async_test]
async fn metric_registry_gauge_overwrites() {
    let metrics = MetricRegistry::new(16).await;
    metrics.set_gauge("mailbox.depth", Labels::none().await, 3.0).await;
    metrics.set_gauge("mailbox.depth", Labels::none().await, 7.0).await;
    assert_eq!(metrics.gauge_value("mailbox.depth", &Labels::none().await).await, Some(7.0));
}

#[semio_framework_async_macros::async_test]
async fn metric_registry_histogram_buckets_are_upper_inclusive_with_overflow_bucket() {
    let metrics = MetricRegistry::new(16).await;
    let bounds = [1.0, 5.0, 10.0];
    for v in [0.5, 1.0, 3.0, 5.0, 8.0, 20.0] {
        metrics.observe_histogram("latency_ms", Labels::none().await, &bounds, v).await.unwrap();
    }
    let snap = metrics.histogram_snapshot("latency_ms", &Labels::none().await).await.unwrap();
    assert_eq!(snap.bucket_counts, vec![2, 2, 1, 1]);
    assert_eq!(snap.count, 6);
}

#[semio_framework_async_macros::async_test]
async fn metric_registry_histogram_rejects_a_bounds_length_change() {
    let metrics = MetricRegistry::new(16).await;
    metrics.observe_histogram("h", Labels::none().await, &[1.0, 2.0], 1.0).await.unwrap();
    let err = metrics.observe_histogram("h", Labels::none().await, &[1.0], 1.0).await.unwrap_err();
    assert!(matches!(err, DbError::InvalidArgument(_)));
}
//#endregion 🔖️Metrics

//#region 🔖️Span
struct FakeClock(AtomicU64);
impl Clock for FakeClock {
    async fn now_ms(&self) -> u64 {
        self.0.fetch_add(10, Ordering::Relaxed)
    }
}

#[semio_framework_async_macros::async_test]
async fn span_registry_records_duration_and_moves_active_to_completed() {
    let registry = SpanRegistry::with_clock(FakeClock(AtomicU64::new(0)), 16).await;
    let id = registry.start("doc.commit", None, None).await;
    assert_eq!(registry.active_count().await, 1);
    let completed = registry.end(id).await.unwrap();
    assert_eq!(completed.duration_ms, 10);
    assert_eq!(registry.active_count().await, 0);
    assert_eq!(registry.completed().await.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn span_registry_double_end_returns_none_instead_of_panicking() {
    let registry = SpanRegistry::with_clock(FakeClock(AtomicU64::new(0)), 16).await;
    let id = registry.start("s", None, None).await;
    assert!(registry.end(id).await.is_some());
    assert!(registry.end(id).await.is_none());
}

#[semio_framework_async_macros::async_test]
async fn span_registry_retention_is_bounded() {
    let registry = SpanRegistry::with_clock(FakeClock(AtomicU64::new(0)), 2).await;
    for _ in 0..5 {
        let id = registry.start("s", None, None).await;
        registry.end(id).await;
    }
    assert_eq!(registry.completed().await.len(), 2);
}
//#endregion 🔖️Span

//#region 🔖️Health
#[semio_framework_async_macros::async_test]
async fn health_registry_reports_worst_of_all_components() {
    let health = HealthRegistry::new();
    health.set("wal", HealthState::Healthy);
    health.set("storage", HealthState::Degraded("slow fsync".to_string()));
    let report = health.report();
    assert_eq!(report.overall, HealthState::Degraded("slow fsync".to_string()));
    assert_eq!(report.components.len(), 2);

    health.set("storage", HealthState::Unhealthy("disk full".to_string()));
    assert_eq!(health.report().overall, HealthState::Unhealthy("disk full".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn health_registry_defaults_to_healthy_with_no_components() {
    let health = HealthRegistry::new();
    assert_eq!(health.report().overall, HealthState::Healthy);
}
//#endregion 🔖️Health

//#region 🔖️Determinism
#[semio_framework_async_macros::async_test]
async fn determinism_verifier_reports_no_divergence_on_matching_digests() {
    let verifier = DeterminismVerifier::new(["primary", "replay"], 16).await;
    assert!(verifier.record(0, "primary", hash(1).await).await.unwrap().is_none());
    assert!(verifier.record(0, "replay", hash(1).await).await.unwrap().is_none());
    assert_eq!(verifier.pending_count().await, 0);
}

#[semio_framework_async_macros::async_test]
async fn determinism_verifier_reports_divergence_on_mismatched_digests() {
    let verifier = DeterminismVerifier::new(["primary", "replay"], 16).await;
    verifier.record(3, "primary", hash(1).await).await.unwrap();
    let report = verifier.record(3, "replay", hash(2).await).await.unwrap().unwrap();
    assert_eq!(report.seq, 3);
    assert_eq!(report.digests.len(), 2);
    assert_eq!(verifier.pending_count().await, 0);
}

#[semio_framework_async_macros::async_test]
async fn determinism_verifier_bounds_pending_window() {
    let verifier = DeterminismVerifier::new(["primary", "replay"], 2).await;
    verifier.record(0, "primary", hash(1).await).await.unwrap();
    verifier.record(1, "primary", hash(1).await).await.unwrap();
    let err = verifier.record(2, "primary", hash(1).await).await.unwrap_err();
    assert!(matches!(err, DbError::LimitExceeded(_)));
}
//#endregion 🔖️Determinism

//#region 🔖️Otel
#[cfg(feature = "otel")]
#[semio_framework_async_macros::async_test]
async fn unwired_otel_exporter_reports_unimplemented_rather_than_panicking() {
    let span = CompletedSpan { id: SpanId(0), name: "s", parent: None, document: None, start_ms: 0, end_ms: 1, duration_ms: 1 };
    let err = UnwiredOtelExporter.export(&span).await.unwrap_err();
    assert!(matches!(err, DbError::Unimplemented(_)));
}
//#endregion 🔖️Otel
