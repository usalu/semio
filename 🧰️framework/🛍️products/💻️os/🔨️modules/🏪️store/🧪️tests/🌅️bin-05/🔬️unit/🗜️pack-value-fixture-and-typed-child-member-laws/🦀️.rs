// 🗜️ Pack value fixture and typed child member laws: the pack value fixture corpus and its hex dump, the wire integer variants at the boundaries, the child, owner and link dsl fields, the typed child store factory, the member factory catalog, the member link resolver and the closed dialect graph admission.

//#region 🔖️ReconcileAlternative
//#region 🔖️PackValueFixtures
async fn pack_value_fixture_corpus() -> Vec<(&'static str, DslValue)> {
    vec![
        ("null", DslValue::Null),
        ("bool_true", DslValue::Bool(true)),
        ("bool_false", DslValue::Bool(false)),
        ("int_zero", DslValue::uint(0)),
        ("int_negative_one", DslValue::int(-1)),
        ("float_pi", DslValue::float(3.14)),
        ("float_whole_number", DslValue::float(2.0)),
        ("string_empty", DslValue::String(String::new())),
        ("string_escapes", DslValue::String("hello\nworld with \"quotes\"".into())),
        ("array_empty", DslValue::Array(vec![])),
        ("array_ints", DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::uint(3)])),
        ("object_empty", DslValue::Object(vec![])),
        ("object_mixed", DslValue::object([("a".into(), DslValue::uint(1)), ("b".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null]))])),
        (
            "nested_deep",
            DslValue::object([("a".into(), DslValue::object([("b".into(), DslValue::object([("c".into(), DslValue::Array(vec![DslValue::uint(1), DslValue::uint(2), DslValue::object([("d".into(), DslValue::String("leaf".into()))])]))]))]))]),
        ),
    ]
}

/// 🎯️ Variant-exact `DslValue` equality. `Number`'s own `PartialEq` calls a fitting
/// `UInt`/`Int` pair equal, so it cannot witness that the wire preserved the WRITER's variant —
/// which is exactly what `TAG_UINT` versus `TAG_INT` decides, and what the canonical bytes and
/// every hash over them depend on. `Float` compares by bits so `-0.0` and `NaN` are exact too.
// 🚫️async: E1 pure recursive comparison consumed inside sync `.all()`/`.is_some_and()` closures
// that cannot themselves be async — see R9/R10 residue shape 1
fn dsl_value_variant_exact_eq(a: &DslValue, b: &DslValue) -> bool {
    match (a, b) {
        (DslValue::Number(semio_framework_value::Number::UInt(x)), DslValue::Number(semio_framework_value::Number::UInt(y))) => x == y,
        (DslValue::Number(semio_framework_value::Number::Int(x)), DslValue::Number(semio_framework_value::Number::Int(y))) => x == y,
        (DslValue::Number(semio_framework_value::Number::Float(x)), DslValue::Number(semio_framework_value::Number::Float(y))) => x.to_bits() == y.to_bits(),
        (DslValue::Number(_), DslValue::Number(_)) => false,
        (DslValue::Array(x), DslValue::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(a, b)| dsl_value_variant_exact_eq(a, b)),
        (DslValue::Object(x), DslValue::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.iter().find(|(ok, _)| ok == k).is_some_and(|(_, ov)| dsl_value_variant_exact_eq(v, ov))),
        _ => a == b,
    }
}

/// 🔣️ Reads one `semio.pack.dynamic-integer/v1` value node — integers arrive as exact
/// decimal STRINGS and floats as little-endian hex, so no fixture number ever passes through a
/// JSON `f64` on its way into the corpus.
// 🚫️async: E1 pure recursive fixture reader consumed inside sync iterator closures — see R9
fn dynamic_integer_fixture_value(node: &serde_json::Value) -> DslValue {
    if let Some(text) = node.get("uint").and_then(|v| v.as_str()) {
        return DslValue::uint(text.parse::<u64>().expect("corpus uint is an exact u64"));
    }
    if let Some(text) = node.get("int").and_then(|v| v.as_str()) {
        return DslValue::int(text.parse::<i64>().expect("corpus int is an exact i64"));
    }
    if let Some(text) = node.get("f64LeHex").and_then(|v| v.as_str()) {
        let mut bits = [0u8; 8];
        for (index, byte) in bits.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("corpus f64LeHex byte");
        }
        return DslValue::Number(semio_framework_value::Number::Float(f64::from_le_bytes(bits)));
    }
    if let Some(items) = node.get("list").and_then(|v| v.as_array()) {
        return DslValue::Array(items.iter().map(dynamic_integer_fixture_value).collect());
    }
    if let Some(entries) = node.get("map").and_then(|v| v.as_array()) {
        return DslValue::Object(entries.iter().map(|entry| (entry[0].as_str().expect("corpus map key").to_string(), dynamic_integer_fixture_value(&entry[1]))).collect());
    }
    panic!("unsupported corpus value node {node}");
}

// 🚫️async: E1 pure hex reader consumed inside sync iterator closures — see R9
fn dynamic_integer_fixture_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len() / 2).map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("corpus wireHex byte")).collect()
}

/// 🧾️ Hex-dumps `pack_rt::encode_pack_value` over a representative `DslValue`
/// corpus — ground truth for `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s TS
/// `PackValueCodec` mirror (`framework/product/os/ts/index.ts`). Run with `--nocapture` to
/// capture the printed `name -> hex` lines; also asserts `decode_pack_value(encode_pack_value(v))
/// == v` for every entry so the corpus is never accidentally out of date with the real codec.
#[semio_framework_async_macros::async_test]
async fn pack_value_fixture_corpus_hex_dump() {
    for (name, value) in pack_value_fixture_corpus().await {
        let bytes = pack_rt::encode_pack_value(&value);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[pack_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
        let decoded = pack_rt::decode_pack_value(&bytes).expect("decode_pack_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn interaction_state_pack_matches_first_party_value_and_json_oracle() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../../🎒️interaction-state-pack.json")).unwrap();
    let value = DslValue::from(&oracle);
    let state = <protocol::InteractionState as FromValue>::from_value(value.clone()).unwrap();
    let encoded = <protocol::InteractionState as ArtifactPack>::encode_pack(&state);
    assert_eq!(encoded, <DslValue as ArtifactPack>::encode_pack(&value));
    assert_eq!(encoded, pack_rt::encode_json_value(&oracle));
    let decoded = <protocol::InteractionState as ArtifactPack>::decode_pack(&encoded).unwrap();
    assert_eq!(decoded, state);
    assert_eq!(serde_json::Value::from(&<protocol::InteractionState as ToValue>::to_value(&decoded)), oracle);
}

/// 🪶️ Hex-dumps `pack_rt::encode_wire_value` over the SAME fixture corpus — ground
/// truth for the container-less wire codec mirror in TS.
#[semio_framework_async_macros::async_test]
async fn pack_wire_value_fixture_corpus_hex_dump() {
    for (name, value) in pack_value_fixture_corpus().await {
        let bytes = pack_rt::encode_wire_value(&value);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[pack_wire_value_fixture] {name} ({} bytes) -> {hex}", bytes.len());
        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "round-trip mismatch for fixture {name}: {decoded:?} != {value:?}");
    }
}

/// 🔢️ Exact-variant law for the dynamic `Shape::Value` grammar over the shared,
/// language-neutral `semio.pack.dynamic-integer/v1` corpus
/// (`💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1`, whose `wireHex` is independently generated
/// by a third-party LEB128 oracle). Asserts the emitted TAG, the exact `Number` variant, and
/// byte equality — `DslValue` equality alone cannot witness any of the three, because
/// `Number::PartialEq` calls `UInt(7)` and `Int(7)` equal while their wire tags differ.
#[semio_framework_async_macros::async_test]
async fn pack_wire_value_preserves_integer_variants_at_u64_i64_boundaries() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json")).expect("dynamic integer corpus parses");
    assert_eq!(corpus["schema"].as_str(), Some("semio.pack.dynamic-integer/v1"));
    assert_eq!(corpus["version"].as_u64(), Some(1));
    let accept = corpus["accept"].as_array().expect("accept rows");
    assert_eq!(accept.len(), 6, "the corpus is bounded at six accept rows");

    for row in accept {
        let id = row["id"].as_str().expect("row id");
        let value = dynamic_integer_fixture_value(&row["value"]);
        let expected = dynamic_integer_fixture_bytes(row["wireHex"].as_str().expect("row wireHex"));
        let bytes = pack_rt::encode_wire_value(&value);
        assert_eq!(bytes, expected, "{id}: encode_wire_value must be byte-exact against the neutral corpus");
        let tag = bytes[4];
        let expected_tag = match row["variant"].as_str() {
            Some("uint") => 0x04u8,
            Some("int") => 0x03,
            Some("float") => 0x05,
            Some("map") => 0x10,
            other => panic!("{id}: unsupported corpus variant {other:?}"),
        };
        assert_eq!(tag, expected_tag, "{id}: the dynamic value's leading tag carries the writer's variant");
        let decoded = pack_rt::decode_wire_value(&bytes).expect("decode_wire_value");
        assert!(dsl_value_variant_exact_eq(&decoded, &value), "{id}: {decoded:?} is not variant-exact against {value:?}");
        assert_eq!(pack_rt::encode_wire_value(&decoded), expected, "{id}: re-encoding the decoded value is canonical and idempotent");
    }

    assert!(matches!(pack_rt::decode_wire_value(&dynamic_integer_fixture_bytes("000101110403")).expect("uint 3"), DslValue::Number(semio_framework_value::Number::UInt(3))));
    assert!(matches!(pack_rt::decode_wire_value(&pack_rt::encode_wire_value(&DslValue::int(7))).expect("int 7"), DslValue::Number(semio_framework_value::Number::Int(7))), "a positive Int keeps TAG_INT rather than collapsing onto TAG_UINT");
    assert_ne!(pack_rt::encode_wire_value(&DslValue::int(7)), pack_rt::encode_wire_value(&DslValue::uint(7)), "Int(7) and UInt(7) are equal under Number::PartialEq but are distinct on the wire");
    let beyond_exact_f64 = 9_007_199_254_740_993u64;
    assert_ne!(beyond_exact_f64 as f64 as u64, beyond_exact_f64, "the f64 widening this law replaced is not injective past 2^53");

    for row in corpus["reject"].as_array().expect("reject rows") {
        let id = row["id"].as_str().expect("row id");
        let bytes = dynamic_integer_fixture_bytes(row["wireHex"].as_str().expect("row wireHex"));
        match row["outcome"].as_str() {
            Some("decode-error") => {
                assert!(pack_rt::decode_wire_value(&bytes).is_err(), "{id}: a truncated or overlong varint must fail the reader");
            }
            Some("noncanonical") => {
                let decoded = pack_rt::decode_wire_value(&bytes).expect("the shared permissive varint reader admits redundant encodings");
                assert!(dsl_value_variant_exact_eq(&decoded, &dynamic_integer_fixture_value(&row["decodes"])), "{id}: {decoded:?} is not the documented decode");
                assert_ne!(pack_rt::encode_wire_value(&decoded), bytes, "{id}: a canonical-bytes caller rejects it because encode(decode(bytes)) != bytes");
            }
            other => panic!("{id}: unsupported corpus outcome {other:?}"),
        }
    }
}


#[semio_framework_async_macros::async_test]
async fn artifact_child_dsl_field_round_trips_via_pack_and_value() {
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "87a".into(), subset: "mesh".into() } };
    let child: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target);

    let spec = artifact_child_spec();
    let record = artifact_child_to_record(&child);
    let bytes = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
    let (decoded_record, _report) = semio_framework_os_pack::record::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
    let decoded: ArtifactChild<DemoSnapshot> = artifact_child_from_record(&decoded_record).expect("from_record");
    assert_eq!(decoded, child);

    let value = <ArtifactChild<DemoSnapshot> as semio_framework_dsl_record::DslField>::to_value(&child);
    let via_field = <ArtifactChild<DemoSnapshot> as semio_framework_dsl_record::DslField>::from_value(&value).expect("from_value");
    assert_eq!(via_field, child);

    assert_eq!(child.to_child_ref("mesh-slot").await, ChildRef { slot: "mesh-slot".into(), child_id: "child-1".into(), target: child.target.clone() });
}

#[semio_framework_async_macros::async_test]
async fn artifact_child_required_owner_distinguishes_absence_and_wrong_type() {
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-1".into(), dialect: demo_child_dialect() };
    let absent: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target.clone());
    assert_eq!(absent.require_local_owner::<String>(), Err(ArtifactChildMaterializationError::Absent));

    let typed: ArtifactChild<DemoSnapshot> = ArtifactChild::new("child-1".into(), target).with_local_owner(Arc::new(String::from("content")));
    assert_eq!(typed.require_local_owner::<u64>(), Err(ArtifactChildMaterializationError::WrongType));
    assert_eq!(typed.require_local_owner::<String>().unwrap().as_str(), "content");
}

#[semio_framework_async_macros::async_test]
async fn owner_ref_dsl_field_round_trips_via_pack() {
    let owner = OwnerRef {
        parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.object".into(), standard: "1".into(), subset: "*".into() } },
        slot: "mesh-slot".into(),
        child_id: "child-1".into(),
    };
    let spec = owner_ref_spec();
    let record = owner_ref_to_record(&owner);
    let bytes = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
    let (decoded_record, _report) = semio_framework_os_pack::record::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
    let decoded = owner_ref_from_record(&decoded_record).expect("from_record");
    assert_eq!(decoded, owner);
}

#[semio_framework_async_macros::async_test]
async fn artifact_link_dsl_field_round_trips_every_link_pin_variant() {
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "linked-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
    let pins = vec![LinkPin::Head, LinkPin::Checkpoint { id: "ck-abc123".into() }, LinkPin::Snapshot { blob: BlobRef { hash: "deadbeef".into(), size: 42, media_type: "image/png".into() } }];
    for pin in pins {
        let link = ArtifactLink { target: target.clone(), pin: pin.clone(), role: "cover-image".into() };
        let semio_framework_dsl_record::Shape::Record(producer)=<ArtifactLink as semio_framework_dsl_record::DslField>::shape()else{unreachable!()};
        let spec=(producer.ordinary)();
        let semio_framework_dsl_record::FieldValue::Record(record)=<ArtifactLink as semio_framework_dsl_record::DslField>::to_value(&link)else{unreachable!()};
        let bytes = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).expect("encode");
        let (decoded_record, _report) = semio_framework_os_pack::record::decode_record_body(&bytes, &spec, &PackDecodeOptions::default()).expect("decode");
        let decoded=<ArtifactLink as semio_framework_dsl_record::DslField>::from_value(&semio_framework_dsl_record::FieldValue::Record(decoded_record)).expect("from_record");
        assert_eq!(decoded, link, "round trip diverged for pin variant {pin:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_refs_defaults_to_empty_for_a_leaf_snapshot() {
    struct LeafSnapshot;
    impl ArtifactRefs for LeafSnapshot {}
    let snapshot = LeafSnapshot;
    assert!(snapshot.child_refs().await.is_empty());
    assert!(snapshot.links().await.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn typed_child_store_factory_round_trips_a_child_through_create_persist_open() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let dialect = demo_child_dialect();

    let mut child = create_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", "child-round-trip", &dialect, &DemoSnapshot { n: Some(7) }.encode_pack(), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await.expect("create");
    assert!(child.owners.snapshot_retirement_factory.is_some(), "generated member creation must install the exact snapshot retirement factory before returning ownership");
    assert!(child.owned_disposer_installed(), "generated member creation must install the exact whole-store bounded disposer before returning ownership");
    child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }, &mut identity).await.expect("apply");
    child.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
    child.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 11 })], transaction: None }, &mut identity).await.expect("re-apply");

    let files = print_document_pack(child.envelope()).await.expect("print");
    let persisted = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-round-trip".into(), dialect };
    let mut reopened = open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, None, &persisted, child.local_actor_id().clone(), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await.expect("open");

    assert!(reopened.owners.snapshot_retirement_factory.is_some(), "generated member reopen must install the exact snapshot retirement factory before returning ownership");
    assert!(reopened.owned_disposer_installed(), "generated member reopen must install the exact whole-store bounded disposer before returning ownership");
    assert_eq!(reopened.envelope().id, "child-round-trip");
    assert_eq!(reopened.snapshot().expect("head snapshot"), child.snapshot().expect("head snapshot"), "reopened child's live content diverged from the persisted one");
    assert_eq!(reopened.snapshot().expect("head snapshot"), DemoSnapshot { n: Some(11) }, "reopen restored the wrong cursor position");
    close_member_dialect_fixture(&mut reopened);
    close_member_dialect_fixture(&mut child);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🌱️ A derivable child's genesis envelope is minted from bytes alone and opens exactly like a
/// store-printed one: same id, schema, dialect and owner, the initial snapshot as live content, no
/// history — and a mismatched owner, like an empty initial pack, fails closed.
#[semio_framework_async_macros::async_test]
async fn genesis_member_envelope_pack_opens_as_the_owned_child_without_a_live_store() {
    let dialect = demo_child_dialect();
    let parent = semio_framework_artifact_reference::ArtifactRef { artifact_id: "genesis-parent".into(), dialect: dialect.clone() };
    let owner = OwnerRef { parent: parent.clone(), slot: "slot".into(), child_id: "genesis-child".into() };
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "genesis-child".into(), dialect: dialect.clone() };
    let envelope_pack = genesis_member_envelope_pack("demo/v1", &expected, &owner, &DemoSnapshot { n: Some(5) }.encode_pack()).await.expect("genesis envelope");
    let mut opened = open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, Some(&owner), &envelope_pack, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await.expect("open genesis member");
    assert_eq!(opened.envelope().id, "genesis-child");
    assert_eq!(opened.envelope().owner.as_ref(), Some(&owner));
    assert_eq!(opened.envelope().dialect.as_ref(), Some(&dialect));
    assert_eq!(opened.snapshot().expect("head snapshot"), DemoSnapshot { n: Some(5) });
    assert!(opened.envelope().vcs.edits.is_empty(), "a genesis member carries no history");
    close_member_dialect_fixture(&mut opened);
    let other = OwnerRef { parent, slot: "other".into(), child_id: "genesis-child".into() };
    assert!(matches!(open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, Some(&other), &envelope_pack, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await, Err(VcsError::Deserialize(_))), "the stamped owner must match the requested one");
    assert!(matches!(genesis_member_envelope_pack("demo/v1", &expected, &owner, &[]).await, Err(VcsError::Deserialize(_))), "an empty initial pack is rejected, never substituted");
}

#[semio_framework_async_macros::async_test]
async fn member_factory_wrapper_cannot_bypass_exact_create_or_open_owner_catalog() {
    type Wrapped = ArtifactStore<DemoSnapshot, DemoMutation>;
    let dialect = demo_child_dialect();
    let mut created = <Wrapped as MemberFactory>::create("wrapped-member", &dialect, &DemoSnapshot { n: Some(3) }.encode_pack(), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("wrapper create");
    assert!(created.owners.snapshot_retirement_factory.is_some());
    assert!(created.owners.initial_snapshot_retirement_factory.is_some());
    assert!(created.owners.mutation_retirement_factory.is_some());
    assert!(created.owned_disposer_installed());
    let files = print_document_pack(created.envelope()).await.expect("print wrapped member");
    let persisted = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "wrapped-member".into(), dialect };
    let mut reopened = <Wrapped as MemberFactory>::open(&expected, None, &persisted, created.local_actor_id().clone()).await.expect("wrapper open");
    assert!(reopened.owners.snapshot_retirement_factory.is_some());
    assert!(reopened.owners.initial_snapshot_retirement_factory.is_some());
    assert!(reopened.owners.mutation_retirement_factory.is_some());
    assert!(reopened.owned_disposer_installed());
    close_member_dialect_fixture(&mut reopened);
    close_member_dialect_fixture(&mut created);
}

#[semio_framework_async_macros::async_test]
async fn member_close_rejects_missing_owner_and_preserves_the_installed_disposer_until_terminal() {
    let mut missing = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "missing-member-owner", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    let refused = SpaceMember::close_owned_step(&mut missing, physical_test_close_grant());
    assert!(matches!(&refused, Err(reason) if reason.kind == semio_framework_value::ValueRefusalKind::InvariantViolated && reason.message.contains("no installed owner disposer")), "{refused:?}");

    missing.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    missing.closes_on_drop();
    let first = SpaceMember::close_owned_step(&mut missing, physical_test_close_grant());
    assert!(matches!(&first, Ok(RetainedCloneStep::Progress(progress)) if progress.fits(physical_test_close_grant())), "{first:?}");
    assert!(missing.owned_disposer_installed(), "the admitted owner remains retained until terminal close");
    assert!(!SpaceMember::close_owned_terminal_is_empty(&missing));
    let replacement=complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()));
    let(error,mut returned)=missing.install_document_store_owners_exact(replacement).expect_err("the original catalog cannot be replaced");
    assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
    for _ in 0..10_000 { if returned.uninstalled_owners_terminal_is_empty(){break;}let step=returned.close_uninstalled_owners_step(physical_test_close_grant()).expect("returned original catalog closes under the supplied grant");assert!(step.progress().fits(physical_test_close_grant())); }
    assert!(returned.uninstalled_owners_terminal_is_empty());
    close_member_dialect_fixture(&mut missing);
}

/// 🏠️ owner ⇒ dialect: an envelope that is somebody's child but names no dialect cannot be
/// typed by its parent, so `open` must fail closed rather than hand back an untypable member.
#[semio_framework_async_macros::async_test]
async fn typed_child_store_factory_rejects_empty_genesis_and_dialect_less_owned_child() {
    assert!(matches!(create_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", "child-empty", &demo_child_dialect(), &[], ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await, Err(VcsError::Deserialize(_))), "an empty genesis pack must never silently default");

    let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "child-no-dialect", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.owner = Some(OwnerRef { parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: "parent".into(), dialect: demo_child_dialect() }, slot: "mesh".into(), child_id: "child-no-dialect".into() });
    let files = print_document_pack(&envelope).await.expect("print");
    let orphan = encode_document_pack_bytes(&files.pack, &files.spr).await;
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "child-no-dialect".into(), dialect: demo_child_dialect() };
    assert!(matches!(open_member_store::<DemoSnapshot, DemoMutation, _,_>("demo/v1", &expected, envelope.owner.as_ref(), &orphan, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).await, Err(VcsError::Deserialize(_))), "an owned child with no dialect must fail closed");
    retire_demo_envelope(envelope);
}

#[semio_framework_async_macros::async_test]
async fn pack_at_checkpoint_reads_history_without_moving_the_live_cursor() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "pinned-target", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply");
    let checkpoint = ReferenceSpaceMember::commit_checkpoint(&mut store, "v1".into(), Vec::new(), &mut identity).await.expect("checkpoint");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply after checkpoint");

    let historical = DemoSnapshot::decode_pack(&store.pack_at_checkpoint(&checkpoint).await.expect("pack at checkpoint")).expect("decode");
    let live = DemoSnapshot::decode_pack(&store.document_pack_bytes().await.expect("head pack")).expect("decode");
    assert_eq!(historical, DemoSnapshot { n: Some(2) }, "checkpoint read did not return the pinned content");
    assert_eq!(live, DemoSnapshot { n: Some(3) }, "reading a checkpoint moved the live cursor");
    assert!(matches!(store.pack_at_checkpoint("no-such-checkpoint").await, Err(VcsError::UnknownChange(_))));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn member_link_resolver_resolves_head_checkpoint_and_degrades_snapshot_pins() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    struct FixtureDirectory {
        member: ArtifactStore<DemoSnapshot, DemoMutation>,
    }
    impl MemberDirectory for FixtureDirectory {
        async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
            if artifact_id == "linked-doc" { Some(self.member.document_pack_bytes().await) } else { None }
        }
        async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
            if artifact_id == "linked-doc" { Some(self.member.pack_at_checkpoint(checkpoint_id).await) } else { None }
        }
    }

    let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "linked-doc", DemoSnapshot { n: Some(1) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 42 })], transaction: None }, &mut identity).await.expect("apply");
    let pinned = ReferenceSpaceMember::commit_checkpoint(&mut member, "pinned".into(), Vec::new(), &mut identity).await.expect("checkpoint");
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 99 })], transaction: None }, &mut identity).await.expect("apply after pin");

    let resolver = MemberLinkResolver::new(FixtureDirectory { member }).await;
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "linked-doc".into(), dialect: demo_child_dialect() };
    let link_of = |pin: LinkPin| ArtifactLink { target: target.clone(), pin, role: "representation".into() };
    let decode = async |state: LinkState| match state {
        LinkState::Resolved { pack_bytes, .. } => DemoSnapshot::decode_pack(&pack_bytes).expect("decode"),
        other => panic!("expected Resolved, found {other:?}"),
    };

    assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Head)).await).await, DemoSnapshot { n: Some(99) }, "a Head pin must follow the target's live tip");
    assert_eq!(decode(resolver.resolve(&link_of(LinkPin::Checkpoint { id: pinned })).await).await, DemoSnapshot { n: Some(42) }, "a Checkpoint pin must keep resolving to the pinned history, not the tip");

    let blob = BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() };
    assert!(matches!(resolver.resolve(&link_of(LinkPin::Snapshot { blob })).await, LinkState::PinnedOnly { .. }), "a snapshot pin with no blob store must degrade to PinnedOnly, never Missing");

    let absent = ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef { artifact_id: "gone".into(), dialect: demo_child_dialect() }, pin: LinkPin::Head, role: "representation".into() };
    assert_eq!(resolver.resolve(&absent).await, LinkState::Missing);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn link_resolver_reports_resolved_missing_and_pinned_only_states() {
    struct DemoResolver;
    impl LinkResolver for DemoResolver {
        async fn resolve(&self, link: &ArtifactLink) -> LinkState {
            match link.role.as_str() {
                "known" => LinkState::Resolved { pack_bytes: vec![1, 2, 3], dialect: link.target.dialect.clone() },
                "pinned" => LinkState::PinnedOnly { blob: BlobRef { hash: "deadbeef".into(), size: 3, media_type: "application/octet-stream".into() } },
                _ => LinkState::Missing,
            }
        }
    }
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "linked-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.image".into(), standard: "1".into(), subset: "*".into() } };
    let resolver = DemoResolver;
    assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "known".into() }).await, LinkState::Resolved { .. }));
    assert!(matches!(resolver.resolve(&ArtifactLink { target: target.clone(), pin: LinkPin::Head, role: "pinned".into() }).await, LinkState::PinnedOnly { .. }));
    assert!(matches!(resolver.resolve(&ArtifactLink { target, pin: LinkPin::Head, role: "gone".into() }).await, LinkState::Missing));
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_parent_projection_matches_neutral_corpus() {
    use semio_framework_schema_composition::{ArtifactCompositionFields, ChildRefFields, ChildRefVisitor, ChildSlotSpec};
    struct Parent<'a>(&'a [serde_json::Value]);
    fn fields(row: &serde_json::Value) -> ChildRefFields<'_> {
        ChildRefFields {
            child_id: row["childId"].as_str().unwrap(),
            artifact_id: row["artifactId"].as_str().unwrap(),
            artifact_kind: row["artifactKind"].as_str().unwrap(),
            standard: row["standard"].as_str().unwrap(),
            subset: row["subset"].as_str().unwrap(),
        }
    }
    impl ArtifactCompositionFields for Parent<'_> {
        fn child_slots() -> &'static [ChildSlotSpec] {
            &[ChildSlotSpec { name: "single", selection: semio_framework_schema_composition::ChildSlotSelection::ArtifactKind("s.test.member"), many: false }, ChildSlotSpec { name: "many", selection: semio_framework_schema_composition::ChildSlotSelection::ArtifactKind("s.test.member"), many: true }, ChildSlotSpec { name: "other", selection: semio_framework_schema_composition::ChildSlotSelection::ArtifactKind("s.test.member"), many: false }]
        }
        fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
            for row in self.0 {
                visitor.step()?;
                let slot = match row["slot"].as_str().unwrap() {
                    "single" => "single",
                    "many" => "many",
                    "other" => "other",
                    _ => "unknown",
                };
                visitor.child(slot, fields(row))?;
            }
            Ok(())
        }
    }
    let fixture = member_dialect_fixture();
    let slots = Parent::child_slots();
    for (actual, expected) in slots.iter().zip(fixture["projectionSlots"].as_array().unwrap()) {
        assert_eq!(actual.name, expected["name"].as_str().unwrap());
        let semio_framework_schema_composition::ChildSlotSelection::ArtifactKind(kind)=actual.selection else{panic!("original projection fixture requires artifact kind selection")};
        assert_eq!(kind,expected["kind"].as_str().unwrap());
        assert_eq!(actual.many, expected["many"].as_bool().unwrap());
    }
    let rows = fixture["projectionCases"].as_array().unwrap();
    assert_eq!(rows.len(), 16);
    for row in rows {
        let parent = Parent(row["parent"].as_array().unwrap());
        let projection = ChildRestoreProjection::from_snapshot(&parent);
        assert_eq!(projection.is_ok(), row["projected"].as_bool().unwrap(), "{}", row["id"]);
        let admitted = projection.as_ref().is_ok_and(|projection| projection.admit_complete(row["incoming"].as_array().unwrap().iter().map(|row| (row["slot"].as_str().unwrap(), fields(row)))).is_ok());
        assert_eq!(admitted, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
    }
}

/// 🌲️ Original forest and linked owners require exact physical closure under independent common grants.
#[semio_framework_async_macros::async_test]
async fn composition_graph_retires_with_exact_independent_frontier_grants() {
    let (mut graph, original_bytes, released) = semio_framework_trace_memory_testing::observe_backing(|| {
        let mut graph = CompositionGraph::default();
        graph.owns.insert("s.flow.flow@1/child".into(), ("s.flow.flow@1/parent".into(), "slot".into()));
        graph.links.insert("s.flow.flow@1/parent".into(), ["s.flow.flow@1/linked".into()].into_iter().collect());
        graph
    });
    assert_eq!(released, 0);
    let mut released_bytes = 0;
    for _ in 0..10000 {
        let grant = physical_test_close_grant();
        assert!(graph.next_close_copy_byte_demand().expect("linked graph requires genuine controlled owner") <= grant.maximum_copy_bytes);
        assert!(graph.next_close_capacity_byte_demand(grant.maximum_copy_bytes).expect("original graph capacity") <= grant.maximum_capacity_bytes);
        assert!(graph.next_close_release_byte_demand().expect("original graph release") <= grant.maximum_release_bytes);
        assert!(graph.next_close_depth_demand().expect("original graph depth") <= grant.maximum_depth);
        assert!(grant.maximum_copy_bytes <= 4096);
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| graph.close_step(grant));
        let step = step.expect("every original graph owner closes without fallback");
        assert_eq!(allocated, 0); assert!(step.progress().fits(grant)); assert_eq!(released, step.progress().released_bytes);
        released_bytes += released;
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(graph.terminal_is_empty()); assert_eq!(released_bytes, original_bytes);
    let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(graph)); assert_eq!((allocated, released), (0, 0));
}

#[semio_framework_async_macros::async_test]
async fn member_factory_closed_dialect_graph_admission_matches_neutral_corpus() {
    let fixture = member_dialect_fixture();
    let rows = fixture["graphCases"].as_array().expect("neutral graph rows");
    assert_eq!(rows.len(), 7);
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let mut graph = CompositionGraph::new().await;
        for edge in row["edges"].as_array().unwrap() {
            graph.insert_owns(edge["parent"].as_str().unwrap(), edge["slot"].as_str().unwrap(), edge["child"].as_str().unwrap()).await.expect("valid initial forest");
        }
        let previous = graph.owns.clone();
        let candidate = &row["candidate"];
        let parent = candidate["parent"].as_str().unwrap();
        let slot = candidate["slot"].as_str().unwrap();
        let child = candidate["child"].as_str().unwrap();
        let admission = graph.admit_owns(parent, slot, child).await;
        assert_eq!(admission.is_ok(), row["accepted"].as_bool().unwrap(), "{id}");
        assert_eq!(graph.owns, previous, "{id}: admission is read-only");
        if let Ok(admission) = admission {
            assert_eq!(admission.inserts(), row["inserted"].as_bool().unwrap(), "{id}");
            graph.commit_owns_admitted(admission);
            assert_eq!(graph.owner_of(child).await, Some(parent), "{id}");
            assert_eq!(graph.slot_of(child).await, Some(slot), "{id}");
            assert!(!graph.admit_owns(parent, slot, child).await.unwrap().inserts(), "{id}: exact repeat cannot erase or replace an existing edge");
        }
    }
    let graph = CompositionGraph::new().await;
    let mut foreign = CompositionGraph::new().await;
    let ticket = graph.admit_owns("parent", "slot", "child").await.unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| foreign.commit_owns_admitted(ticket))).is_err());
    assert!(foreign.owns.is_empty());
    let mut graph = CompositionGraph::new().await;
    let ticket = graph.admit_owns("parent", "slot", "child").await.unwrap();
    graph.insert_owns("child", "ancestor", "parent").await.unwrap();
    let previous = graph.owns.clone();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| graph.commit_owns_admitted(ticket))).is_err());
    assert_eq!(graph.owns, previous);
}
//#endregion 🔖️PackValueFixtures
//#endregion 🔖️ReconcileAlternative
