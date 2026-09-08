mod laws {
    //! 🧪️ Task 4's seven laws. Every routing/registration law runs against a LOCAL `EntryMap`
    //! (never the process-global `IO_MECHANISM_REGISTRY`) so tests never interfere with each
    //! other under the test harness's default parallel execution — only
    //! `conformance_runs_after_deserialize` exercises the public constructor + `IoEntry.run`
    //! directly (no registry needed for that one at all).
    use super::*;
    use crate::io_schema::{StandardId, SubsetId};

    const A: Dialect = Dialect { artifact_kind: "test.io-mechanism.a", standard: StandardId("1"), subset: SubsetId("*") };
    const B: Dialect = Dialect { artifact_kind: "test.io-mechanism.b", standard: StandardId("1"), subset: SubsetId("*") };
    const C: Dialect = Dialect { artifact_kind: "test.io-mechanism.c", standard: StandardId("1"), subset: SubsetId("*") };
    const D: Dialect = Dialect { artifact_kind: "test.io-mechanism.d", standard: StandardId("1"), subset: SubsetId("*") };
    const E: Dialect = Dialect { artifact_kind: "test.io-mechanism.e", standard: StandardId("1"), subset: SubsetId("*") };

    // 🚫️async: E4 fn-pointer slot — `IoEntry.run` is a bare `fn` pointer and this test double
    // never suspends, so it needs no `resolve_ready` wrapping either.
    #[allow(clippy::unnecessary_wraps, reason = "IoEntry test doubles must implement its fallible function-pointer contract")]
    fn passthrough(payload: &IoPayload) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(payload.clone()))
    }

    async fn key(from: Dialect, into: Dialect) -> EntryKey {
        (ArtifactDialect::from(from), ArtifactDialect::from(into))
    }

    #[semio_framework_async_macros::async_test]
    async fn route_is_deterministic() {
        static AB: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { from: B, into: C, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };

        let mut order1: EntryMap = BTreeMap::new();
        order1.insert(key(A, B).await, &AB);
        order1.insert(key(B, C).await, &BC);

        let mut order2: EntryMap = BTreeMap::new();
        order2.insert(key(B, C).await, &BC);
        order2.insert(key(A, B).await, &AB);

        let route1 = resolve_route(&order1, &ArtifactDialect::from(A), &ArtifactDialect::from(C), 3).await.expect("route exists");
        let route2 = resolve_route(&order2, &ArtifactDialect::from(A), &ArtifactDialect::from(C), 3).await.expect("route exists");
        assert_eq!(route1.value, route2.value, "route must not depend on registration order");
    }

    #[semio_framework_async_macros::async_test]
    async fn route_respects_max_hops() {
        static AB: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { from: B, into: C, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static CD: IoEntry = IoEntry { from: C, into: D, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static DE: IoEntry = IoEntry { from: D, into: E, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        registry.insert(key(A, B).await, &AB);
        registry.insert(key(B, C).await, &BC);
        registry.insert(key(C, D).await, &CD);
        registry.insert(key(D, E).await, &DE);

        assert!(resolve_route(&registry, &ArtifactDialect::from(A), &ArtifactDialect::from(D), 2).await.is_err(), "3 hops must fail a max_hops of 2");
        assert!(resolve_route(&registry, &ArtifactDialect::from(A), &ArtifactDialect::from(D), 3).await.is_ok(), "3 hops must succeed at the max_hops=3 ceiling");
        assert!(resolve_route(&registry, &ArtifactDialect::from(A), &ArtifactDialect::from(E), 10).await.is_err(), "4 hops must fail even when the caller asks for 10 — max_hops is hard-clamped to 3");
    }

    #[semio_framework_async_macros::async_test]
    async fn route_never_cycles() {
        static AB: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static BA: IoEntry = IoEntry { from: B, into: A, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { from: B, into: C, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        registry.insert(key(A, B).await, &AB);
        registry.insert(key(B, A).await, &BA);
        registry.insert(key(B, C).await, &BC);

        let route = resolve_route(&registry, &ArtifactDialect::from(A), &ArtifactDialect::from(C), 3).await.expect("route exists despite a cycle in the graph");
        assert_eq!(route.value.hops.len(), 2, "the cycle edge B->A must never appear in the winning route");
    }

    #[semio_framework_async_macros::async_test]
    async fn route_prefers_higher_minimum_fidelity() {
        static DIRECT: IoEntry = IoEntry { from: A, into: C, fidelity: IoFidelity::Lossy, sniff: None, run: passthrough };
        static AB: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { from: B, into: C, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        registry.insert(key(A, C).await, &DIRECT);
        registry.insert(key(A, B).await, &AB);
        registry.insert(key(B, C).await, &BC);

        let route = resolve_route(&registry, &ArtifactDialect::from(A), &ArtifactDialect::from(C), 3).await.expect("route exists").value;
        assert_eq!(route.fidelity, IoFidelity::Exact, "the 2-hop all-Exact route must beat the shorter 1-hop Lossy direct route");
        assert_eq!(route.hops.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn identify_only_sniffs_carriers() {
        // 🚫️async: E4 fn-pointer slot
        fn always_high(_: &IoPayload) -> Confidence {
            Confidence::High
        }
        static CARRIER_ENTRY: IoEntry = IoEntry { from: CARRIER_TEXT, into: A, fidelity: IoFidelity::Exact, sniff: Some(always_high), run: passthrough };
        static NON_CARRIER_ENTRY: IoEntry = IoEntry { from: B, into: C, fidelity: IoFidelity::Exact, sniff: Some(always_high), run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        registry.insert(key(CARRIER_TEXT, A).await, &CARRIER_ENTRY);
        registry.insert(key(B, C).await, &NON_CARRIER_ENTRY);

        let found = resolve_identify(&registry, &IoPayload::Text("hello".to_string())).await;
        assert_eq!(found, vec![(ArtifactDialect::from(A), Confidence::High)], "only the carrier-origin entry may be sniffed");
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_entry_is_a_typed_error() {
        static ENTRY_A: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static ENTRY_B: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Lossy, sniff: None, run: passthrough };

        let mut existing: EntryMap = BTreeMap::new();
        existing.insert(key(A, B).await, &ENTRY_A);

        let same = build_proposed(&[&ENTRY_A]).unwrap();
        assert!(validate_against(&existing, &same).is_ok(), "re-registering the identical entry is idempotent");

        let different = build_proposed(&[&ENTRY_B]).unwrap();
        assert!(matches!(validate_against(&existing, &different), Err(IoRegistryError::Duplicate { .. })), "a different entry for the same (from, into) key is a typed conflict");
    }

    #[semio_framework_async_macros::async_test]
    async fn registration_is_all_or_nothing() {
        static ORIGINAL: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };
        static CONFLICTING: IoEntry = IoEntry { from: A, into: B, fidelity: IoFidelity::Lossy, sniff: None, run: passthrough };
        static FRESH: IoEntry = IoEntry { from: D, into: E, fidelity: IoFidelity::Exact, sniff: None, run: passthrough };

        let mut existing: EntryMap = BTreeMap::new();
        existing.insert(key(A, B).await, &ORIGINAL);

        let proposed = build_proposed(&[&CONFLICTING, &FRESH]).expect("the batch is internally conflict-free");
        assert!(matches!(validate_against(&existing, &proposed), Err(IoRegistryError::Duplicate { .. })), "one conflicting key must fail the whole batch");
        assert!(!existing.contains_key(&key(D, E).await), "FRESH must not leak into the registry as a side effect of a failed validation");
        assert_eq!(existing.len(), 1, "existing registry state must be untouched by a failed validation");
    }

    // 🌉️ `serde_json::Value` retired in favor of `dsl::DslValue` — `impl store::ArtifactPack for
    // DslValue` (`🏪️store/🦀️.rs`) already gives it the same "schema-agnostic fixture type" role
    // `serde_json::Value`'s own `impl ArtifactPack` used to play, with no serde dependency.
    struct JsonDeserializer;
    impl Deserializer<dsl::DslValue> for JsonDeserializer {
        const FROM: Dialect = B;
        const FIDELITY: IoFidelity = IoFidelity::Exact;
        const CONFORMANCE: Option<fn(&dsl::DslValue) -> Vec<Diagnostic>> = Some(flag_non_object);

        async fn deserialize(payload: &IoPayload) -> IoResult<dsl::DslValue> {
            let IoPayload::Text(text) = payload else {
                return Err(IoError { message: "expected text payload".to_string(), diagnostics: Vec::new() });
            };
            let value: dsl::DslValue = dsl::os_pack::json::from_json_str(text).map_err(|error| IoError { message: error.to_string(), diagnostics: Vec::new() })?;
            Ok(IoOutcome::clean(value))
        }
    }

    // 🚫️async: E4 fn-pointer slot — assigned directly into `Deserializer::CONFORMANCE`, a plain
    // `Option<fn(&S) -> Vec<Diagnostic>>` const (see that trait's own doc comment for why it
    // cannot be a closure or an `async fn` item).
    fn flag_non_object(value: &dsl::DslValue) -> Vec<Diagnostic> {
        if matches!(value, dsl::DslValue::Object(_)) { Vec::new() } else { vec![Diagnostic::error("test.io-mechanism.not-object", dsl::TextSpan::at(0, 0), "value is not a JSON object")] }
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_runs_after_deserialize() {
        let entry = deserializer_entry::<dsl::DslValue, JsonDeserializer>(A);

        let conforming = (entry.run)(&IoPayload::Text("{}".to_string())).expect("an empty object deserializes cleanly");
        assert!(conforming.diagnostics.is_empty(), "an object payload has no conformance diagnostics");

        let non_conforming = (entry.run)(&IoPayload::Text("42".to_string())).expect("deserialize still succeeds when conformance is unhappy");
        assert_eq!(non_conforming.diagnostics.len(), 1, "CONFORMANCE's diagnostics must reach the caller after a successful deserialize");
    }
}
