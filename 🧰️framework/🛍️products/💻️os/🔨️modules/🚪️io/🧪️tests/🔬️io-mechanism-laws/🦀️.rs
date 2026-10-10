mod laws {
    //! 🧪️ Task 4's seven laws. Every routing/registration law runs against a LOCAL `EntryMap`
    //! (never the process-global `IO_MECHANISM_REGISTRY`) so tests never interfere with each
    //! other under the test harness's default parallel execution — only
    //! `conformance_runs_after_deserialize` exercises the public constructor + `IoEntry.run`
    //! directly (no registry needed for that one at all).
    use super::*;
    use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const A: Dialect = Dialect { artifact_kind: "test.io-mechanism.a", standard: StandardId("1"), subset: SubsetId("*") };
    const B: Dialect = Dialect { artifact_kind: "test.io-mechanism.b", standard: StandardId("1"), subset: SubsetId("*") };
    const C: Dialect = Dialect { artifact_kind: "test.io-mechanism.c", standard: StandardId("1"), subset: SubsetId("*") };
    const D: Dialect = Dialect { artifact_kind: "test.io-mechanism.d", standard: StandardId("1"), subset: SubsetId("*") };
    const E: Dialect = Dialect { artifact_kind: "test.io-mechanism.e", standard: StandardId("1"), subset: SubsetId("*") };

    // 🚫️async: E4 fn-pointer slot — `IoEntry.run` is a bare `fn` pointer and this test double
    // never suspends, so it needs no `resolve_ready` wrapping either.
    #[allow(clippy::unnecessary_wraps, reason = "IoEntry test doubles must implement its fallible function-pointer contract")]
    fn passthrough(payload: &IoPayload, control: &mut IoRunControl<'_ , '_>) -> IoResult<IoPayload> {
        control.checkpoint().map_err(IoError::from_value_error)?;
        Ok(IoOutcome::clean(match payload { IoPayload::Binary(bytes) => IoPayload::Binary(control.encode().map_err(IoError::from_value_error)?.copy_bytes(bytes).map_err(IoError::from_value_error)?), IoPayload::Text(text) => IoPayload::Text(control.encode().map_err(IoError::from_value_error)?.copy_text(text).map_err(IoError::from_value_error)?) }))
    }

    fn with_control<T>(operation:impl FnOnce(&mut IoRunControl<'_, '_>)->T)->T {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️control/🔣️.json")).unwrap();
        let maximum=fixture["nativeBytes"].as_u64().unwrap() as usize;
        let mut receive=|_|true;let mut publish=|_|true;
        let mut decode=semio_framework_value::NativeDecodeControl::new(maximum,&mut receive);
        let mut encode=semio_framework_value::NativeEncodeControl::new(maximum,&mut publish);
        let mut control=IoRunControl::new(&mut decode,&mut encode,serde_json::from_value(fixture["snapshotGrant"].clone()).unwrap());
        operation(&mut control)
    }

    async fn key(from: Dialect, into: Dialect) -> EntryKey {
        (ArtifactDialect::from(from), ArtifactDialect::from(into))
    }

    #[semio_framework_async_macros::async_test]
    async fn route_is_deterministic() {
        static AB: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { owned_serializer: None, from: B, into: C, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };

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
        static AB: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { owned_serializer: None, from: B, into: C, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static CD: IoEntry = IoEntry { owned_serializer: None, from: C, into: D, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static DE: IoEntry = IoEntry { owned_serializer: None, from: D, into: E, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
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
        static AB: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static BA: IoEntry = IoEntry { owned_serializer: None, from: B, into: A, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { owned_serializer: None, from: B, into: C, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        registry.insert(key(A, B).await, &AB);
        registry.insert(key(B, A).await, &BA);
        registry.insert(key(B, C).await, &BC);

        let route = resolve_route(&registry, &ArtifactDialect::from(A), &ArtifactDialect::from(C), 3).await.expect("route exists despite a cycle in the graph");
        assert_eq!(route.value.hops.len(), 2, "the cycle edge B->A must never appear in the winning route");
    }

    #[semio_framework_async_macros::async_test]
    async fn route_prefers_higher_minimum_fidelity() {
        static DIRECT: IoEntry = IoEntry { owned_serializer: None, from: A, into: C, fidelity: IoFidelity::Lossy, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static AB: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static BC: IoEntry = IoEntry { owned_serializer: None, from: B, into: C, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
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
        static CARRIER_ENTRY: IoEntry = IoEntry { owned_serializer: None, from: CARRIER_TEXT, into: A, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: Some(always_high), run: passthrough };
        static NON_CARRIER_ENTRY: IoEntry = IoEntry { owned_serializer: None, from: B, into: C, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: Some(always_high), run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        registry.insert(key(CARRIER_TEXT, A).await, &CARRIER_ENTRY);
        registry.insert(key(B, C).await, &NON_CARRIER_ENTRY);

        let found = resolve_identify(&registry, &IoPayload::Text("hello".to_string())).await;
        assert_eq!(found, vec![(ArtifactDialect::from(A), Confidence::High)], "only the carrier-origin entry may be sniffed");
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_entry_is_a_typed_error() {
        static ENTRY_A: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static ENTRY_B: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Lossy, direction: IoEntryDirection::Export, sniff: None, run: passthrough };

        let mut existing: EntryMap = BTreeMap::new();
        existing.insert(key(A, B).await, &ENTRY_A);

        let same = build_proposed(&[&ENTRY_A]).unwrap();
        assert!(validate_against(&existing, &same).is_ok(), "re-registering the identical entry is idempotent");

        let different = build_proposed(&[&ENTRY_B]).unwrap();
        assert!(matches!(validate_against(&existing, &different), Err(IoRegistryError::Duplicate { .. })), "a different entry for the same (from, into) key is a typed conflict");
    }

    #[semio_framework_async_macros::async_test]
    async fn registration_is_all_or_nothing() {
        static ORIGINAL: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static CONFLICTING: IoEntry = IoEntry { owned_serializer: None, from: A, into: B, fidelity: IoFidelity::Lossy, direction: IoEntryDirection::Export, sniff: None, run: passthrough };
        static FRESH: IoEntry = IoEntry { owned_serializer: None, from: D, into: E, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run: passthrough };

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
    impl Deserializer<semio_framework_value::DslValue> for JsonDeserializer {
        const FROM: Dialect = B;
        const FIDELITY: IoFidelity = IoFidelity::Exact;
        const CONFORMANCE: Option<fn(&semio_framework_value::DslValue) -> Vec<Diagnostic>> = Some(flag_non_object);

        async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<semio_framework_value::DslValue> {
            let IoPayload::Text(text) = payload else {
                return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected text payload")));
            };
            let value: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str_controlled(text, semio_framework_pack_json::JsonMemberPolicy::Reject, control.decode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?;
            Ok(IoOutcome::clean(value))
        }
    }

    // 🚫️async: E4 fn-pointer slot — assigned directly into `Deserializer::CONFORMANCE`, a plain
    // `Option<fn(&S) -> Vec<Diagnostic>>` const (see that trait's own doc comment for why it
    // cannot be a closure or an `async fn` item).
    fn flag_non_object(value: &semio_framework_value::DslValue) -> Vec<Diagnostic> {
        if matches!(value, semio_framework_value::DslValue::Object(_)) { Vec::new() } else { vec![Diagnostic::error("test.io-mechanism.not-object", semio_framework_diagnostic::TextSpan::at(0, 0), "value is not a JSON object")] }
    }

    /// 🔁️ Echoes the parent value beside every owned child it received (`slot/childId`, archive order).
    struct ChildrenEcho;
    impl Serializer<semio_framework_value::DslValue> for ChildrenEcho {
        const INTO: Dialect = C;
        const FIDELITY: IoFidelity = IoFidelity::Exact;

        async fn serialize(from: &semio_framework_value::DslValue, children: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
            let slots: Vec<String> = children.slots().into_iter().map(|(slot, child_id)| format!("{slot}/{child_id}")).collect();
            Ok(IoOutcome::clean(IoPayload::Text(format!("{from:?}|{}", slots.join(",")))))
        }
    }

    /// 🗃️ A carrier of `parent` owning one child at `content`/`child-1`, with `parent_spr` as its parent history.
    fn composed_carrier(parent: &semio_framework_value::DslValue, parent_spr: Vec<u8>) -> Vec<u8> {
        let artifact = |id: &str, dialect: Dialect| store::channel::DocumentArchiveArtifactRef { artifact_id: id.into(), artifact_kind: dialect.artifact_kind.into(), standard: dialect.standard.0.into(), subset: dialect.subset.0.into() };
        let owner = store::channel::DocumentArchiveOwnerRef { parent: artifact("parent", A), slot: "content".into(), child_id: "child-1".into() };
        let member = store::channel::OwnedDocumentMemberPackEntry { ordinal: 0, reference: artifact("child-1", B), owner, envelope_pack: vec![7] };
        store::channel::encode_document_archive_bytes(&store::channel::DocumentArchivePack { parent_pack: store::ArtifactPack::encode_pack(parent), parent_spr, members: vec![member] }).expect("the carrier encodes")
    }

    /// 🪆️ LAW (design §20.15, readers compose on read): a serializer entry run over a composed head carrier receives the parent and
    /// its owned children; a plain pack reaches it with the empty view; a carrier with parent history or malformed bytes is refused.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_head_carrier_hands_its_owned_children_to_the_serializer() {
        let entry = serializer_entry::<semio_framework_value::DslValue, ChildrenEcho>(A);
        let parent = semio_framework_value::DslValue::String("parent".into());
        let text = |outcome: IoResult<IoPayload>| match outcome.expect("the entry runs").value {
            IoPayload::Text(text) => text,
            IoPayload::Binary(_) => panic!("the echo serializes text"),
        };
        assert_eq!(text(with_control(|control|(entry.run)(&IoPayload::Binary(composed_carrier(&parent, Vec::new())),control))), format!("{parent:?}|content/child-1"));
        assert_eq!(text(with_control(|control|(entry.run)(&IoPayload::Binary(store::ArtifactPack::encode_pack(&parent)),control))), format!("{parent:?}|"));
        assert!(with_control(|control|(entry.run)(&IoPayload::Binary(composed_carrier(&parent, vec![1])),control)).is_err(), "a carrier with parent history is not a head carrier");
        assert!(with_control(|control|(entry.run)(&IoPayload::Binary(vec![store::channel::DOCUMENT_ARCHIVE_VERSION, 0xff]),control)).is_err(), "a malformed carrier is refused");
    }

    /// 🧲️ LAW: the constructors declare the native side — a serializer entry exports out of its own dialect, a deserializer entry
    /// imports into it.
    #[semio_framework_async_macros::async_test]
    async fn entry_constructors_declare_their_native_side() {
        let exporting = serializer_entry::<semio_framework_value::DslValue, ChildrenEcho>(A);
        assert_eq!((exporting.direction, ArtifactDialect::from(exporting.from), ArtifactDialect::from(exporting.into)), (IoEntryDirection::Export, ArtifactDialect::from(A), ArtifactDialect::from(C)));
        let importing = deserializer_entry::<semio_framework_value::DslValue, JsonDeserializer>(A);
        assert_eq!((importing.direction, ArtifactDialect::from(importing.from), ArtifactDialect::from(importing.into)), (IoEntryDirection::Import, ArtifactDialect::from(B), ArtifactDialect::from(A)));
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_runs_after_deserialize() {
        let entry = deserializer_entry::<semio_framework_value::DslValue, JsonDeserializer>(A);

        let conforming = with_control(|control|(entry.run)(&IoPayload::Text("{}".to_string()),control)).expect("an empty object deserializes cleanly");
        assert!(conforming.diagnostics.is_empty(), "an object payload has no conformance diagnostics");

        let non_conforming = with_control(|control|(entry.run)(&IoPayload::Text("42".to_string()),control)).expect("deserialize still succeeds when conformance is unhappy");
        assert_eq!(non_conforming.diagnostics.len(), 1, "CONFORMANCE's diagnostics must reach the caller after a successful deserialize");
    }

    #[test]
    fn owned_serializer_factory_identity_is_part_of_registration(){
        use semio_framework_value::retained_clone::{RetainedCloneBirthDemand,RetainedCloneGrant};
        fn demand_a(_: &OwnedSerializerRequest)->Result<RetainedCloneBirthDemand,ValueError>{Ok(RetainedCloneBirthDemand{capacity_bytes:128,depth:1})}
        fn demand_b(_: &OwnedSerializerRequest)->Result<RetainedCloneBirthDemand,ValueError>{Ok(RetainedCloneBirthDemand{capacity_bytes:129,depth:1})}
        fn admit_a(request:OwnedSerializerRequest,_:RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>{Err(OwnedSerializerRefusal{error:ValueError::literal(ValueRefusalKind::UnsupportedOwner,"identity A preserves original request"),request,progress:Default::default()})}
        fn admit_b(request:OwnedSerializerRequest,_:RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>{Err(OwnedSerializerRefusal{error:ValueError::literal(ValueRefusalKind::UnsupportedOwner,"identity B preserves original request"),request,progress:Default::default()})}
        static ORIGINAL:IoEntry=IoEntry{from:A,into:B,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:passthrough,owned_serializer:Some(OwnedSerializerFactory{demand:demand_a,admit:admit_a})};
        static IDENTICAL:IoEntry=IoEntry{from:A,into:B,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:passthrough,owned_serializer:Some(OwnedSerializerFactory{demand:demand_a,admit:admit_a})};
        static DIFFERENT_DEMAND:IoEntry=IoEntry{from:A,into:B,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:passthrough,owned_serializer:Some(OwnedSerializerFactory{demand:demand_b,admit:admit_a})};
        static DIFFERENT_ADMISSION:IoEntry=IoEntry{from:A,into:B,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:passthrough,owned_serializer:Some(OwnedSerializerFactory{demand:demand_a,admit:admit_b})};
        static MISSING:IoEntry=IoEntry{from:A,into:B,fidelity:IoFidelity::Exact,direction:IoEntryDirection::Export,sniff:None,run:passthrough,owned_serializer:None};
        let expected:serde_json::Value=serde_json::from_str(include_str!("../../📤️serialization/📦️owned/🧫️fixtures/🔣️.json")).unwrap();assert_eq!(expected["cases"].as_array().unwrap().len(),21);
        assert!(same_io_entry(&ORIGINAL,&IDENTICAL));assert_eq!(build_proposed(&[&ORIGINAL,&IDENTICAL]).unwrap().len(),1);
        for changed in [&DIFFERENT_DEMAND,&DIFFERENT_ADMISSION,&MISSING]{assert!(!same_io_entry(&ORIGINAL,changed));assert!(matches!(build_proposed(&[&ORIGINAL,changed]),Err(IoRegistryError::Duplicate{..})));}
        eprintln!("[DEBUG] Registered owned serializer identity retains demand/admission function custody; differing factories refused atomically");
    }
}
