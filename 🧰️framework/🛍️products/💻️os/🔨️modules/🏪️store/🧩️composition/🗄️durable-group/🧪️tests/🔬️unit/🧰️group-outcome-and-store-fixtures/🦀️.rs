/// 🧰️ Group outcome and store fixtures: the durable group fixture, the hex, revision and reference helpers, the member, prepared outcome and decision builders, the owned three stores and the erased snapshot authority assertion.

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("durable group fixture")
}

fn copy_grant(bytes: usize) -> semio_framework_os_store_one_item::ArtifactStoreOneItemGrant {
    semio_framework_os_store_one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_capacity_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_release_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_depth: 64 }
}

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn hex_string(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[usize::from(byte >> 4)] as char);
        encoded.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

fn revision(value: &str) -> [u8; 32] {
    hex(value).try_into().expect("fixed revision")
}

fn reference(value: &serde_json::Value) -> semio_framework_artifact_reference::ArtifactRef {
    semio_framework_pack_json::from_json_str(&serde_json::to_string(value).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture reference")
}

fn member(value: &serde_json::Value) -> DurableOwnedGroupMemberV1 {
    DurableOwnedGroupMemberV1 {
        role: value["role"].as_str().unwrap().into(),
        reference: reference(&value["reference"]),
        owner: if value["owner"].is_null() { None } else { Some(semio_framework_pack_json::from_json_str(&serde_json::to_string(&value["owner"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture owner")) },
        expected_generation: value["expectedGeneration"].as_u64().unwrap(),
        expected_revision: revision(value["expectedRevisionHex"].as_str().unwrap()),
        recovery_schema: value["recoverySchema"].as_str().unwrap().into(),
        recovery_pack: hex(value["callerRecoveryPackHex"].as_str().unwrap()),
        recovery_pack_sha256: value["callerRecoveryPackSha256"].as_str().unwrap().into(),
        unbound_outcome_sha256: value["unboundOutcomeSha256"].as_str().unwrap().into(),
        post_generation: value["postGeneration"].as_u64().unwrap(),
        post_revision: revision(value["postRevisionHex"].as_str().unwrap()),
    }
}

fn prepared_outcome(role: &str, recovery_schema: &str, generation: u64, base_revision: [u8; 32], ordinal: u64) -> (ArtifactStoreOneItemPrepared<DslValue, String>, DurableStorePreparedOutcomeV1) {
    let actor = semio_framework_value::SharedUtf8::admit(format!("map-owner-{role}"), semio_framework_value::testing::physical_test_close_grant()).unwrap_or_else(|(error,_)| panic!("funded original actor: {error}")).0;
    let next_clock = HybridLogicalTimestamp { actor: ordinal, physical_ms: 1_000 + ordinal, logical: ordinal + 1 };
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
        semio_framework_job::OperationId(100 + ordinal),
        semio_framework_job::Generation(generation),
        base_revision,
        ordinal as usize,
        ordinal as i32 + 10,
        next_clock,
        actor.clone(),
        None,
        None,
        None,
    ));
    let edit_id = format!("map-{role}-edit-{ordinal}");
    let edit = Edit {
        line: None,
        id: edit_id.clone(),
        actor: Some(actor.clone()),
        forwards: vec![format!("{role}:forward")],
        inverse: vec![format!("{role}:inverse")].into(),
        mutation_meta: vec![semio_framework_os_spr::MutationMeta {
            mutation_id: Some(semio_framework_os_spr::MutationId(format!("{edit_id}#0"))),
            dependencies: Vec::new(),
            base_version: generation,
            author_id: Some(semio_framework_os_spr::ActorId(actor)),
            timestamp: next_clock,
            undo_policy: semio_framework_os_spr::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: Some(semio_framework_os_spr::SchemaId(recovery_schema.into())),
            label: Some(format!("prepare {role}")),
            group_id: None,
            origin: Default::default(),
            transaction: None,
        }],
        verb: None,
        sequence_number: ordinal as i32 + 10,
        started_at: format!("2026-09-05T00:00:0{ordinal}Z"),
        finished_at: Some(format!("2026-09-05T00:00:1{ordinal}Z")),
    };
    let post_snapshot = Arc::new(DslValue::Object(vec![("role".into(), DslValue::String(role.into())), ("ordinal".into(), DslValue::uint(ordinal))]));
    let prepared = authority.prepare_one_item(edit, post_snapshot).expect("Store seals one exact unbound prepared owner");
    let outcome = DurableStorePreparedOutcomeV1::from_prepared(recovery_schema, &prepared).expect("Store derives and verifies the unbound outcome pack");
    (prepared, outcome)
}

fn outcomes() -> DurableStorePreparedOutcomesV1 {
    let fixture = fixture();
    let parent = &fixture["members"]["parent"];
    let drawing = &fixture["members"]["drawing"];
    let value = &fixture["members"]["value"];
    DurableStorePreparedOutcomesV1 {
        parent: prepared_outcome(PARENT_ROLE, parent["recoverySchema"].as_str().unwrap(), parent["expectedGeneration"].as_u64().unwrap(), revision(parent["expectedRevisionHex"].as_str().unwrap()), 1).1,
        drawing: prepared_outcome(DRAWING_ROLE, drawing["recoverySchema"].as_str().unwrap(), drawing["expectedGeneration"].as_u64().unwrap(), revision(drawing["expectedRevisionHex"].as_str().unwrap()), 2).1,
        value: prepared_outcome(VALUE_ROLE, value["recoverySchema"].as_str().unwrap(), value["expectedGeneration"].as_u64().unwrap(), revision(value["expectedRevisionHex"].as_str().unwrap()), 3).1,
    }
}

fn decision() -> DurableOwnedThreeMemberDecisionV1 {
    let fixture = fixture();
    let outcomes = outcomes();
    DurableOwnedThreeMemberDecisionV1::seal_fixture(
        semio_framework_pack_json::from_json_str(&serde_json::to_string(&fixture["anchor"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture anchor"),
        member(&fixture["members"]["parent"]),
        member(&fixture["members"]["drawing"]),
        member(&fixture["members"]["value"]),
        &outcomes,
    )
    .expect("fixture decision")
}

async fn owned_store(id: &str, dialect: semio_framework_artifact_reference::ArtifactDialect, owner: Option<OwnerRef>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    let mut envelope = semio_framework_os_store_document::create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.dialect = Some(dialect);
    envelope.owner = owner;
    let mut store = ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, semio_framework_os_spr::ActorId(fixture()["openedActor"].as_str().expect("declared opened group actor").into())).await.expect("owned group fixture Store");
    store.install_document_store_owners_exact(demo_closable_store_owners());
    store
}

fn store_prepared(store: &ArtifactStore<DemoSnapshot, DemoMutation>, ordinal: u64, next: i32) -> ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation> {
    let actor = semio_framework_value::SharedUtf8::admit(format!("map-owner-{ordinal}"), semio_framework_value::testing::physical_test_close_grant()).unwrap_or_else(|(error,_)| panic!("funded original actor: {error}")).0;
    let next_clock = HybridLogicalTimestamp { actor: ordinal, physical_ms: 2_000 + ordinal, logical: ordinal + 1 };
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
        semio_framework_job::OperationId(200 + ordinal),
        semio_framework_job::Generation(store.cursor.generation),
        store.cursor.content_revision,
        store.log.applied_edit_ids.len(),
        store.log.edit_sequence + 1,
        next_clock,
        actor.clone(),
        None,
        None,
        None,
    ));
    let edit_id = format!("map-store-edit-{ordinal}");
    let edit = Edit {
        line: None,
        id: edit_id.clone(),
        actor: Some(actor.clone()),
        forwards: vec![DemoMutation::SetN(SetN { n: next })],
        inverse: vec![DemoMutation::SetN(SetN { n: store.snapshot_ref().n.unwrap_or_default() })].into(),
        mutation_meta: vec![semio_framework_os_spr::MutationMeta {
            mutation_id: Some(semio_framework_os_spr::MutationId(format!("{edit_id}#0"))),
            dependencies: Vec::new(),
            base_version: store.cursor.generation,
            author_id: Some(semio_framework_os_spr::ActorId(actor)),
            timestamp: next_clock,
            undo_policy: semio_framework_os_spr::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: Some(semio_framework_os_spr::SchemaId(format!("map-role-{ordinal}"))),
            label: Some(format!("durable map member {ordinal}")),
            group_id: None,
            origin: Default::default(),
            transaction: None,
        }],
        verb: None,
        sequence_number: store.log.edit_sequence + 1,
        started_at: format!("2026-09-05T00:01:0{ordinal}Z"),
        finished_at: Some(format!("2026-09-05T00:01:1{ordinal}Z")),
    };
    authority.prepare_one_item(edit, Arc::new(DemoSnapshot { n: Some(next) })).expect("Store seals one exact group candidate")
}

async fn owned_three_stores() -> (ArtifactStore<DemoSnapshot, DemoMutation>, ArtifactStore<DemoSnapshot, DemoMutation>, ArtifactStore<DemoSnapshot, DemoMutation>) {
    let parent_dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() };
    let child_dialect = |subset: &str| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let parent_reference = semio_framework_artifact_reference::ArtifactRef { artifact_id: "map-a".into(), dialect: parent_dialect.clone() };
    let parent = owned_store("map-a", parent_dialect, None).await;
    let drawing = owned_store("gismap-drawing", child_dialect(DRAWING_ROLE), Some(OwnerRef { parent: parent_reference.clone(), slot: DRAWING_ROLE.into(), child_id: "gismap-drawing".into() })).await;
    let value = owned_store("gismap-value", child_dialect(VALUE_ROLE), Some(OwnerRef { parent: parent_reference, slot: VALUE_ROLE.into(), child_id: "gismap-value".into() })).await;
    (parent, drawing, value)
}

fn close_demo_artifact_store(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) {
    for _ in 0..4_096 {
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: 512, ..semio_framework_value::testing::physical_test_close_grant() };
        let step = semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_step(store, grant).expect("durable group fixture Store closes under its bounded owner grant");
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            assert!(semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_terminal_is_empty(store));
            return;
        }
    }
    panic!("durable group fixture Store did not reach its exact terminal-empty witness");
}

async fn assert_erased_snapshot_authority(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, expected: i32) {
    let generation = store.generation();
    let revision = store.content_revision();
    let read = semio_framework_os_store_space_member::lifted_store::SpaceMember::snapshot_read_erased(store).await.expect("erased group read publishes its selected authority");
    assert_eq!(read.typed::<DemoSnapshot>().expect("erased group read retains the exact snapshot type").n, Some(expected));
    assert!(store.owners.snapshot_read_leases.authority_matches(generation, revision));
    let mut read = Some(read);
    let birth = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_release_bytes: 4096, maximum_depth: 64 };
    let (retirement, _) = semio_framework_os_store_space_member::lifted_store::SpaceMember::retire_snapshot_read_erased(store, &mut read, birth).unwrap_or_else(|_| panic!("erased group read returns to its exact Store"));
    let mut retirement = retirement.expect("erased group read retirement is born under its funded grant");
    for _ in 0..64 {
        if matches!(retirement.close_step(semio_framework_value::testing::physical_test_close_grant()).expect("erased group read retirement remains infallible"), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            assert!(retirement.terminal_is_empty());
            return;
        }
    }
    panic!("erased group read retirement must terminate within its bounded fixture");
}

fn bound_three(
    parent: &ArtifactStore<DemoSnapshot, DemoMutation>,
    drawing: &ArtifactStore<DemoSnapshot, DemoMutation>,
    value: &ArtifactStore<DemoSnapshot, DemoMutation>,
) -> DurableOwnedThreeStoreBoundV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation> {
    DurableOwnedThreeStorePreparedV1::from_store_prepared(store_prepared(parent, 1, 7), store_prepared(drawing, 2, 11), store_prepared(value, 3, 13))
        .expect("Store owns exactly three unbound candidates")
        .bind_store_owned(parent, drawing, value)
        .expect("Store binds exactly three candidate owners")
}
