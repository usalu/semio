use super::super::fixture_mutations::demo::{DemoMutation, SetN};
use super::super::tests::{demo_closable_store_owners, DemoOneItemPreparationFactory, DemoSnapshot};
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("durable group fixture")
}

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
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

fn reference(value: &serde_json::Value) -> crate::os_io::ArtifactRef {
    crate::os_pack::json::from_json_str(&serde_json::to_string(value).unwrap()).expect("fixture reference")
}

fn member(value: &serde_json::Value) -> DurableOwnedGroupMemberV1 {
    DurableOwnedGroupMemberV1 {
        role: value["role"].as_str().unwrap().into(),
        reference: reference(&value["reference"]),
        owner: if value["owner"].is_null() { None } else { Some(crate::os_pack::json::from_json_str(&serde_json::to_string(&value["owner"]).unwrap()).expect("fixture owner")) },
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
    let actor = format!("map-owner-{role}");
    let next_clock = HybridLogicalTimestamp { actor: ordinal, physical_ms: 1_000 + ordinal, logical: ordinal + 1 };
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
        operation: semio_framework_job::OperationId(100 + ordinal),
        generation: semio_framework_job::Generation(generation),
        base_revision,
        base_applied_edit_count: ordinal as usize,
        next_sequence_number: ordinal as i32 + 10,
        next_clock,
        actor: actor.clone(),
        group_id: None,
    });
    let edit_id = format!("map-{role}-edit-{ordinal}");
    let edit = Edit {
        id: edit_id.clone(),
        actor: Some(actor.clone()),
        forwards: vec![format!("{role}:forward")],
        inverse: vec![format!("{role}:inverse")],
        mutation_meta: vec![crate::os_spr::MutationMeta {
            mutation_id: Some(crate::os_spr::MutationId(format!("{edit_id}#0"))),
            dependencies: Vec::new(),
            base_version: generation,
            author_id: Some(crate::os_spr::ActorId(actor)),
            timestamp: next_clock,
            undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: Some(crate::os_spr::SchemaId(recovery_schema.into())),
            label: Some(format!("prepare {role}")),
            group_id: None,
            origin: Default::default(),
        }],
        description: Some(format!("prepared {role} outcome")),
        coalesce_key: None,
        sequence_number: ordinal as i32 + 10,
        started_at: format!("2026-09-05T00:00:0{ordinal}Z"),
        finished_at: Some(format!("2026-09-05T00:00:1{ordinal}Z")),
    };
    let post_snapshot = Arc::new(DslValue::Object(vec![("role".into(), DslValue::String(role.into())), ("ordinal".into(), DslValue::uint(ordinal))]));
    let prepared = authority.prepare_one_item(edit, post_snapshot).expect("Store seals one exact unbound prepared owner");
    let outcome = prepared.durable_unbound_outcome(recovery_schema).expect("Store derives and verifies the unbound outcome pack");
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
        crate::os_pack::json::from_json_str(&serde_json::to_string(&fixture["anchor"]).unwrap()).expect("fixture anchor"),
        member(&fixture["members"]["parent"]),
        member(&fixture["members"]["drawing"]),
        member(&fixture["members"]["value"]),
        &outcomes,
    )
    .expect("fixture decision")
}

async fn owned_store(id: &str, dialect: crate::os_io::ArtifactDialect, owner: Option<OwnerRef>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    let mut envelope = crate::os_store::create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n: Some(0) }, None);
    envelope.dialect = Some(dialect);
    envelope.owner = owner;
    let mut store = ArtifactStore::new(envelope).await.expect("owned group fixture Store");
    store.install_member_store_owners_exact(demo_closable_store_owners());
    store
}

fn store_prepared(store: &ArtifactStore<DemoSnapshot, DemoMutation>, ordinal: u64, next: i32) -> ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation> {
    let actor = format!("map-owner-{ordinal}");
    let next_clock = HybridLogicalTimestamp { actor: ordinal, physical_ms: 2_000 + ordinal, logical: ordinal + 1 };
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority {
        operation: semio_framework_job::OperationId(200 + ordinal),
        generation: semio_framework_job::Generation(store.generation),
        base_revision: store.content_revision,
        base_applied_edit_count: store.applied_edit_ids.len(),
        next_sequence_number: store.edit_sequence + 1,
        next_clock,
        actor: actor.clone(),
        group_id: None,
    });
    let edit_id = format!("map-store-edit-{ordinal}");
    let edit = Edit {
        id: edit_id.clone(),
        actor: Some(actor.clone()),
        forwards: vec![DemoMutation::SetN(SetN { n: next })],
        inverse: vec![DemoMutation::SetN(SetN { n: store.snapshot_ref().n.unwrap_or_default() })],
        mutation_meta: vec![crate::os_spr::MutationMeta {
            mutation_id: Some(crate::os_spr::MutationId(format!("{edit_id}#0"))),
            dependencies: Vec::new(),
            base_version: store.generation,
            author_id: Some(crate::os_spr::ActorId(actor)),
            timestamp: next_clock,
            undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: Some(crate::os_spr::SchemaId(format!("map-role-{ordinal}"))),
            label: Some(format!("durable map member {ordinal}")),
            group_id: None,
            origin: Default::default(),
        }],
        description: Some(format!("durable map member {ordinal}")),
        coalesce_key: None,
        sequence_number: store.edit_sequence + 1,
        started_at: format!("2026-09-05T00:01:0{ordinal}Z"),
        finished_at: Some(format!("2026-09-05T00:01:1{ordinal}Z")),
    };
    authority.prepare_one_item(edit, Arc::new(DemoSnapshot { n: Some(next) })).expect("Store seals one exact group candidate")
}

async fn owned_three_stores() -> (ArtifactStore<DemoSnapshot, DemoMutation>, ArtifactStore<DemoSnapshot, DemoMutation>, ArtifactStore<DemoSnapshot, DemoMutation>) {
    let parent_dialect = crate::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() };
    let child_dialect = |subset: &str| crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let parent_reference = crate::os_io::ArtifactRef { artifact_id: "map-a".into(), dialect: parent_dialect.clone() };
    let parent = owned_store("map-a", parent_dialect, None).await;
    let drawing = owned_store("gismap-drawing", child_dialect(DRAWING_ROLE), Some(OwnerRef { parent: parent_reference.clone(), slot: DRAWING_ROLE.into(), child_id: "gismap-drawing".into() })).await;
    let value = owned_store("gismap-value", child_dialect(VALUE_ROLE), Some(OwnerRef { parent: parent_reference, slot: VALUE_ROLE.into(), child_id: "gismap-value".into() })).await;
    (parent, drawing, value)
}

fn close_demo_artifact_store(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) {
    for _ in 0..4_096 {
        let step = crate::os_store::SpaceMember::close_owned_step(store, 1, 512).expect("durable group fixture Store closes under its bounded owner grant");
        if step == crate::os_store::SnapshotRetirementStep::Complete {
            assert!(crate::os_store::SpaceMember::close_owned_terminal_is_empty(store));
            return;
        }
    }
    panic!("durable group fixture Store did not reach its exact terminal-empty witness");
}

async fn assert_erased_snapshot_authority(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, expected: i32) {
    let generation = store.generation();
    let revision = store.content_revision();
    let read = crate::os_store::SpaceMember::snapshot_read_erased(store).await.expect("erased group read publishes its selected authority");
    assert_eq!(read.typed::<DemoSnapshot>().expect("erased group read retains the exact snapshot type").n, Some(expected));
    assert!(store.snapshot_read_leases.authority_matches(generation, revision));
    let mut retirement = crate::os_store::SpaceMember::retire_snapshot_read_erased(store, read).unwrap_or_else(|_| panic!("erased group read returns to its exact Store"));
    for _ in 0..64 {
        match retirement.close_step(1, 4096).expect("erased group read retirement remains infallible") {
            crate::os_store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return;
            }
            crate::os_store::SnapshotRetirementStep::Pending { .. } => {}
            crate::os_store::SnapshotRetirementStep::Blocked => panic!("erased group read retirement has no external wait"),
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

#[derive(Clone, Copy)]
enum FakeJournalResolution {
    Commit,
    AbsentAfterCancel,
    ErrorThenCommit,
    ErrorThenAbsentAfterCancel,
    WrongAnchorThenAbsentAfterCancel,
}

#[derive(Default)]
struct FakeJournalState {
    begins: usize,
    advances: usize,
    cancelled: bool,
    close_count: usize,
    decision_pack: Vec<u8>,
}

struct FakeJournalSink {
    resolution: FakeJournalResolution,
    state: Arc<std::sync::Mutex<FakeJournalState>>,
}

struct FakeJournalCommit {
    resolution: FakeJournalResolution,
    state: Arc<std::sync::Mutex<FakeJournalState>>,
    decision_sha256: String,
    anchor_sha256: String,
    close_started: bool,
}

impl DurableOwnedGroupJournalSinkV1 for FakeJournalSink {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn DurableOwnedGroupJournalCommitV1> {
        let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&decision_pack).expect("fake journal independently admits the canonical decision");
        assert_eq!(decision.decision_sha256, decision_sha256);
        let mut state = self.state.lock().unwrap();
        state.begins += 1;
        state.decision_pack = decision_pack;
        drop(state);
        Box::new(FakeJournalCommit { resolution: self.resolution, state: Arc::clone(&self.state), decision_sha256, anchor_sha256: decision.anchor_sha256, close_started: false })
    }
}

impl DurableOwnedGroupJournalCommitV1 for FakeJournalCommit {
    fn advance(&mut self, grant: crate::os_store::ArtifactStoreOneItemGrant) -> Result<DurableOwnedGroupJournalAdvanceV1, String> {
        if !grant.permits_one() {
            return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
        }
        let mut state = self.state.lock().unwrap();
        state.advances += 1;
        if state.advances == 1 {
            return Ok(DurableOwnedGroupJournalAdvanceV1::Pending);
        }
        match self.resolution {
            FakeJournalResolution::Commit => {
                Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: self.anchor_sha256.clone(), decision_sha256: self.decision_sha256.clone(), transaction_id: 41, segment_index: 7 }))
            }
            FakeJournalResolution::AbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            FakeJournalResolution::AbsentAfterCancel => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
            FakeJournalResolution::ErrorThenCommit if state.advances == 2 => Err("uncertain sync result".into()),
            FakeJournalResolution::ErrorThenCommit => {
                Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: self.anchor_sha256.clone(), decision_sha256: self.decision_sha256.clone(), transaction_id: 43, segment_index: 9 }))
            }
            FakeJournalResolution::ErrorThenAbsentAfterCancel if state.advances == 2 => Err("uncertain sync result".into()),
            FakeJournalResolution::ErrorThenAbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            FakeJournalResolution::ErrorThenAbsentAfterCancel => Ok(DurableOwnedGroupJournalAdvanceV1::Pending),
            FakeJournalResolution::WrongAnchorThenAbsentAfterCancel if state.cancelled => Ok(DurableOwnedGroupJournalAdvanceV1::Absent),
            FakeJournalResolution::WrongAnchorThenAbsentAfterCancel => {
                Ok(DurableOwnedGroupJournalAdvanceV1::Committed(DurableOwnedGroupJournalReceiptV1 { anchor_sha256: "0".repeat(64), decision_sha256: self.decision_sha256.clone(), transaction_id: 47, segment_index: 11 }))
            }
        }
    }

    fn cancel(&mut self) {
        self.state.lock().unwrap().cancelled = true;
    }

    fn begin_close(&mut self) {
        self.state.lock().unwrap().close_count += 1;
        self.close_started = true;
    }

    fn close_step(&mut self, grant: crate::os_store::ArtifactStoreOneItemGrant) -> Result<crate::os_store::SnapshotRetirementStep, String> {
        if !self.close_started || !grant.permits_one() {
            return Ok(crate::os_store::SnapshotRetirementStep::Blocked);
        }
        Ok(crate::os_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_started
    }
}

type DemoMapAssembly = DurableOwnedThreeStoreMapAssemblyV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation>;

fn map_member_admission(store: &ArtifactStore<DemoSnapshot, DemoMutation>, ordinal: u64, next: i32, stale: bool) -> DurableOwnedMapMemberAdmissionV1<DemoMutation> {
    DurableOwnedMapMemberAdmissionV1::new(
        semio_framework_job::OperationId(300 + ordinal),
        store.generation_now() + u64::from(stale),
        store.content_revision_now(),
        format!("map-owner-{ordinal}"),
        DemoMutation::SetN(SetN { n: next }),
        Some(format!("durable Map member {ordinal}")),
    )
}

fn map_assembly(
    parent: ArtifactStore<DemoSnapshot, DemoMutation>,
    drawing: ArtifactStore<DemoSnapshot, DemoMutation>,
    value: ArtifactStore<DemoSnapshot, DemoMutation>,
    stale_role: Option<&str>,
    resolution: FakeJournalResolution,
    state: Arc<std::sync::Mutex<FakeJournalState>>,
) -> (DemoMapAssembly, [Arc<DemoOneItemPreparationFactory>; 3]) {
    let parent_admission = map_member_admission(&parent, 1, 7, stale_role == Some(PARENT_ROLE));
    let drawing_admission = map_member_admission(&drawing, 2, 11, stale_role == Some(DRAWING_ROLE));
    let value_admission = map_member_admission(&value, 3, 13, stale_role == Some(VALUE_ROLE));
    let factories = std::array::from_fn(|_| Arc::new(DemoOneItemPreparationFactory::admissible()));
    let parent_factory: Arc<dyn crate::os_store::ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factories[0].clone();
    let drawing_factory: Arc<dyn crate::os_store::ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factories[1].clone();
    let value_factory: Arc<dyn crate::os_store::ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = factories[2].clone();
    (DurableOwnedThreeStoreMapAssemblyV1::new(parent, drawing, value, parent_admission, drawing_admission, value_admission, parent_factory, drawing_factory, value_factory, Box::new(FakeJournalSink { resolution, state })), factories)
}

fn drive_assembly_to_mounted(assembly: &mut DemoMapAssembly) {
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    for _ in 0..512 {
        match assembly.advance(grant).expect("fixed-three assembly turn") {
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted => return,
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(_) | DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked => {}
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal => panic!("valid assembly reached rejection terminal"),
        }
    }
    panic!("fixed-three assembly did not mount within its bounded turns");
}

fn drain_assembly_rejection(assembly: &mut DemoMapAssembly) -> DurableOwnedThreeStoreMapAssemblyOwnersV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation> {
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    for _ in 0..512 {
        match assembly.advance(grant).expect("rejected fixed-three assembly cleanup turn") {
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal => return assembly.take_terminal_owners().expect("terminal assembly returns every owner"),
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Progress(_) | DurableOwnedThreeStoreMapAssemblyAdvanceV1::Blocked => {}
            DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted => panic!("rejected assembly mounted a journal host"),
        }
    }
    panic!("rejected fixed-three assembly did not terminate within its bounded turns");
}

fn close_assembly_owners(owners: DurableOwnedThreeStoreMapAssemblyOwnersV1<DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation, DemoSnapshot, DemoMutation>) {
    let DurableOwnedThreeStoreMapAssemblyOwnersV1 { mut parent, mut drawing, mut value, sink, .. } = owners;
    drop(sink);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_uses_exact_gis_factories_and_binds_one_decision() {
    let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, factories) = map_assembly(parent, drawing, value, None, FakeJournalResolution::Commit, state.clone());
    drive_assembly_to_mounted(&mut assembly);
    assert!(factories.iter().all(|factory| Arc::strong_count(factory) == 1), "each explicit role factory leaves the assembly immediately after admission");
    let mut host = assembly.take_mounted_host().expect("valid assembly transfers only the existing fixed host");
    assert!(assembly.terminal_is_empty());
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    for _ in 0..128 {
        match host.advance(grant).expect("mounted assembly commit turn") {
            DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_) => assert!(host.acknowledge()),
            DurableOwnedThreeStoreCommitAdvanceV1::Complete => break,
            DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked => {}
        }
    }
    let decision_pack = state.lock().unwrap().decision_pack.clone();
    let decision = DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&decision_pack).expect("journal receives one Store-sealed canonical decision");
    for member in [&decision.parent, &decision.drawing, &decision.value] {
        let outcome = DurableBoundOneItemOutcomeV1::decode_canonical_pack(&member.recovery_pack).expect("member retains one bound recovery owner");
        assert_eq!(outcome.group_id, decision.decision_sha256);
    }
    assert_eq!(state.lock().unwrap().begins, 1);
    let owners = host.take_terminal_owners().expect("acknowledged host returns all Stores and sink");
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_late_member_rejection_closes_prior_publications_before_owner_handoff() {
    for role in [DRAWING_ROLE, VALUE_ROLE] {
        let (parent, drawing, value) = owned_three_stores().await;
        let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
        let (mut assembly, _) = map_assembly(parent, drawing, value, Some(role), FakeJournalResolution::Commit, state.clone());
        let owners = drain_assembly_rejection(&mut assembly);
        assert!(assembly.terminal_is_empty());
        assert_eq!([owners.parent.generation(), owners.drawing.generation(), owners.value.generation()], [0, 0, 0]);
        assert_eq!(state.lock().unwrap().begins, 0);
        assert!(matches!(owners.failure, DurableOwnedThreeStoreMapAssemblyFailureV1::Admission { role: rejected, .. } if rejected == role));
        assert_eq!(owners.parent_mutation.is_some(), false);
        assert_eq!(owners.drawing_mutation.is_some(), role == DRAWING_ROLE);
        assert!(owners.value_mutation.is_some());
        close_assembly_owners(owners);
    }
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_cancellation_before_journal_restores_all_three_frontiers() {
    let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, _) = map_assembly(parent, drawing, value, None, FakeJournalResolution::Commit, state.clone());
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    while assembly.phase() != DurableOwnedThreeStoreMapAssemblyPhaseV1::PreparingDrawing {
        assembly.advance(grant).expect("pre-cancellation assembly turn");
    }
    assert!(assembly.cancel());
    let owners = drain_assembly_rejection(&mut assembly);
    assert!(matches!(owners.failure, DurableOwnedThreeStoreMapAssemblyFailureV1::Cancelled));
    assert_eq!([owners.parent.generation(), owners.drawing.generation(), owners.value.generation()], [0, 0, 0]);
    assert_eq!(state.lock().unwrap().begins, 0);
    close_assembly_owners(owners);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_three_store_assembly_uncertain_journal_retains_same_host_until_committed_or_proven_absent() {
    let (parent, drawing, value) = owned_three_stores().await;
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let (mut assembly, _) = map_assembly(parent, drawing, value, None, FakeJournalResolution::ErrorThenCommit, state.clone());
    drive_assembly_to_mounted(&mut assembly);
    let mut host = assembly.take_mounted_host().expect("prepared assembly transfers one fixed host");
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    let mut uncertain = false;
    for _ in 0..128 {
        match host.advance(grant) {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                uncertain = true;
                let snapshot = host.capture_snapshot().expect("uncertain journal retains one all-old snapshot");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
                assert_eq!(state.lock().unwrap().begins, 1);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => assert!(host.acknowledge()),
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
            Err(error) => panic!("unexpected retained assembly journal error: {error}"),
        }
    }
    assert!(uncertain);
    assert_eq!(state.lock().unwrap().begins, 1);
    let owners = host.take_terminal_owners().expect("same host terminates after one commit and ACK");
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[test]
fn durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds() {
    let fixture = fixture();
    let decision = decision();
    assert_eq!(decision.anchor_sha256, fixture["expected"]["anchorSha256"]);
    assert_eq!(decision.decision_sha256, fixture["expected"]["decisionSha256"]);
    assert_eq!(decision.canonical_unsigned_json(), fixture["expected"]["unsignedJson"]);
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 8);
    let json = decision.canonical_json();
    assert_eq!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&json).unwrap(), decision);
    let pack = decision.encode_pack();
    assert!(pack.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&pack).unwrap(), decision);
    let handles = DurableOwnedGroupMapHandlesV1 { drawing: decision.drawing.reference.clone(), value: decision.value.reference.clone() };
    let frontiers = DurableOwnedGroupMapFrontiersV1 {
        parent_generation: decision.parent.expected_generation,
        parent_revision: decision.parent.expected_revision,
        drawing_generation: decision.drawing.expected_generation,
        drawing_revision: decision.drawing.expected_revision,
        value_generation: decision.value.expected_generation,
        value_revision: decision.value.expected_revision,
    };
    decision.admit_map(&handles, &frontiers).unwrap();
    let mut bound_derivations = decision.clone();
    bound_derivations.parent.recovery_pack.push(0);
    bound_derivations.parent.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&bound_derivations.parent.recovery_pack);
    bound_derivations.parent.post_generation += 1;
    bound_derivations.parent.post_revision[0] ^= 1;
    assert_eq!(bound_derivations.canonical_unsigned_json(), decision.canonical_unsigned_json());
    assert_eq!(semio_framework_hash::sha256_hex(bound_derivations.canonical_unsigned_json().as_bytes()), decision.decision_sha256);
    assert_eq!(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES * 3 + DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, 490_096);
    assert!(490_096 <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    let varint = |value: usize| {
        if value < 1 << 7 {
            1
        } else if value < 1 << 14 {
            2
        } else if value < 1 << 21 {
            3
        } else if value < 1 << 28 {
            4
        } else {
            5
        }
    };
    let frame = |payload: usize| varint(payload + 2) + payload + 10;
    assert_eq!(129 + frame(8) + frame(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES) + frame(12) + 75, 491_779);
    assert!(491_779 <= 507_904);
}

#[test]
fn durable_group_journal_record_projects_edits_only_after_all_three_bound_outcomes_verify() {
    let decision = decision();
    let canonical_pack = decision.encode_pack();
    let record = DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack.clone()).expect("Store derives identity from the complete canonical decision");
    assert!(matches!(DurableOwnedGroupJournalRecordV1::admit(canonical_pack, &"0".repeat(64)), Err(DurableOwnedGroupDecisionError::InvalidHash)));
    let verified = record.verify_fixed_three_edits::<DslValue, String, DslValue, String, DslValue, String>().expect("all three typed outcomes verify before projection");
    assert_eq!(verified.decision_sha256(), decision.decision_sha256);
    assert_eq!(verified.anchor_sha256(), decision.anchor_sha256);
    assert_eq!(verified.document(), &decision.anchor.parent);
    assert_eq!(record.parent_edit_id(), verified.parent().id);
    assert_eq!(record.parent_post_revision(), decision.parent.post_revision);
    assert_eq!(verified.parent().forwards, ["parent:forward"]);
    assert_eq!(verified.drawing().forwards, ["drawing:forward"]);
    assert_eq!(verified.value().forwards, ["value:forward"]);
    for edit in [verified.parent(), verified.drawing(), verified.value()] {
        assert_eq!(edit.mutation_meta.len(), 1);
        assert_eq!(edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    }
}

#[test]
fn durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes() {
    let fixture = fixture();
    let parent = &fixture["members"]["parent"];
    let (prepared, outcome) = prepared_outcome(PARENT_ROLE, parent["recoverySchema"].as_str().unwrap(), parent["expectedGeneration"].as_u64().unwrap(), revision(parent["expectedRevisionHex"].as_str().unwrap()), 1);
    let verified = outcome.verify_inverse::<DslValue, String>().expect("Store-owned bytes invert to the same typed owners");
    assert_eq!(ValueToValue::to_value(verified.edit.as_ref()), ValueToValue::to_value(prepared.edit.as_ref()),);
    assert_eq!(verified.post_snapshot.encode_pack(), prepared.post_snapshot.encode_pack(),);
    assert_eq!(outcome.sha256, semio_framework_hash::sha256_hex(&outcome.pack));
    assert_eq!(hex_string(&outcome.pack), parent["unboundOutcomePackHex"]);
    assert_eq!(outcome.sha256, parent["unboundOutcomeSha256"]);

    let mut modified_bytes = outcome.clone();
    *modified_bytes.pack.last_mut().expect("nonempty outcome pack") ^= 1;
    assert!(matches!(modified_bytes.verify_inverse::<DslValue, String>(), Err(DurableOwnedGroupDecisionError::InvalidHash)));

    let mut reordered = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack).expect("canonical outcome");
    let mut reordered_edit: DslValue = crate::os_pack::json::from_json_str(std::str::from_utf8(&reordered.edit_without_group_canonical_json).unwrap()).expect("edit value");
    let DslValue::Object(fields) = &mut reordered_edit else { panic!("edit projection is an object") };
    fields.rotate_left(1);
    reordered.edit_without_group_canonical_json = crate::os_pack::json::to_json_string(&reordered_edit).into_bytes();
    let reordered_pack = reordered.encode_pack();
    let reordered = DurableStorePreparedOutcomeV1 { recovery_schema: outcome.recovery_schema.clone(), sha256: semio_framework_hash::sha256_hex(&reordered_pack), pack: reordered_pack };
    assert!(matches!(reordered.verify_inverse::<DslValue, String>(), Err(DurableOwnedGroupDecisionError::NonCanonical)));

    let mut retagged = DurableUnboundOneItemOutcomeV1::decode_canonical_pack(&outcome.pack).expect("canonical outcome");
    let mut retagged_edit: DslValue = crate::os_pack::json::from_json_str(std::str::from_utf8(&retagged.edit_without_group_canonical_json).unwrap()).expect("edit value");
    let DslValue::Object(fields) = &mut retagged_edit else { panic!("edit projection is an object") };
    let sequence = fields.iter_mut().find(|(name, _)| name == "sequenceNumber").expect("sequence field");
    sequence.1 = DslValue::float(sequence.1.as_i64().expect("signed sequence") as f64);
    retagged.edit_without_group_canonical_json = crate::os_pack::json::to_json_string(&retagged_edit).into_bytes();
    let retagged_pack = retagged.encode_pack();
    let retagged = DurableStorePreparedOutcomeV1 { recovery_schema: outcome.recovery_schema.clone(), sha256: semio_framework_hash::sha256_hex(&retagged_pack), pack: retagged_pack };
    assert!(retagged.verify_inverse::<DslValue, String>().is_err());

    let all = outcomes();
    for (role, outcome) in [(PARENT_ROLE, &all.parent), (DRAWING_ROLE, &all.drawing), (VALUE_ROLE, &all.value)] {
        assert_eq!(hex_string(&outcome.pack), fixture["members"][role]["unboundOutcomePackHex"]);
        assert_eq!(outcome.sha256, fixture["members"][role]["unboundOutcomeSha256"]);
    }
}

#[test]
fn durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity() {
    let decision = decision();
    let mut forged = decision.clone();
    forged.drawing.owner.as_mut().unwrap().slot = "image".into();
    assert_eq!(forged.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
    let mut forged_child = decision.clone();
    forged_child.drawing.reference.artifact_id = "image".into();
    assert_eq!(forged_child.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
    let mut forged_parent = decision.clone();
    forged_parent.value.owner.as_mut().unwrap().parent.artifact_id = "map-b".into();
    assert_eq!(forged_parent.validate(), Err(DurableOwnedGroupDecisionError::InvalidOwner));
    let mut tampered = decision.clone();
    tampered.value.recovery_pack[0] ^= 1;
    assert_eq!(tampered.validate(), Err(DurableOwnedGroupDecisionError::InvalidHash));
    let mut oversized = decision.clone();
    oversized.parent.recovery_pack.resize(DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES + 1, 0);
    oversized.parent.recovery_pack_sha256 = semio_framework_hash::sha256_hex(&oversized.parent.recovery_pack);
    assert_eq!(oversized.validate(), Err(DurableOwnedGroupDecisionError::RecoveryPackTooLarge));
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&vec![0; DURABLE_OWNED_GROUP_EVENT_MAX_BYTES + 1]), Err(DurableOwnedGroupDecisionError::EventTooLarge));
    let canonical = decision.canonical_json();
    let unknown = format!("{},\"image\":{{}}}}", canonical.strip_suffix('}').unwrap());
    assert!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&unknown).is_err());
    let duplicate = canonical.replacen("{\"schema\":", "{\"schema\":\"semio.store.durable-owned-three-member-decision.v1\",\"schema\":", 1);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::parse_canonical_json(&duplicate), Err(DurableOwnedGroupDecisionError::NonCanonical));
    let mut stale = DurableOwnedGroupMapFrontiersV1 {
        parent_generation: decision.parent.expected_generation,
        parent_revision: decision.parent.expected_revision,
        drawing_generation: decision.drawing.expected_generation,
        drawing_revision: decision.drawing.expected_revision,
        value_generation: decision.value.expected_generation,
        value_revision: decision.value.expected_revision,
    };
    stale.value_generation += 1;
    let handles = DurableOwnedGroupMapHandlesV1 { drawing: decision.drawing.reference.clone(), value: decision.value.reference.clone() };
    assert_eq!(decision.admit_map(&handles, &stale), Err(DurableOwnedGroupDecisionError::InvalidFrontier));
    let mut noncanonical = decision.encode_pack();
    noncanonical.push(0);
    assert!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&noncanonical).is_err());
}

#[semio_framework_async_macros::async_test]
async fn durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners() {
    let (mut parent_store, mut drawing_store, mut value_store) = owned_three_stores().await;
    let bound = bound_three(&parent_store, &drawing_store, &value_store);
    let decision = bound.decision.clone();
    assert!(decision.validate().is_ok());
    assert_eq!(decision.parent.expected_revision, parent_store.content_revision_now());
    assert_eq!(decision.drawing.expected_revision, drawing_store.content_revision_now());
    assert_eq!(decision.value.expected_revision, value_store.content_revision_now());
    assert_eq!(bound.parent.prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    assert_eq!(bound.drawing.prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    assert_eq!(bound.value.prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    for member in [&decision.parent, &decision.drawing, &decision.value] {
        assert!(member.recovery_pack.starts_with(b"\x89SEM\r\n\x1a\n"));
        assert_eq!(member.recovery_pack_sha256, semio_framework_hash::sha256_hex(&member.recovery_pack));
    }
    let recovered = match decision.recover_store_owned(&parent_store, &drawing_store, &value_store).expect("all-base recovery verifies every bound outcome") {
        DurableOwnedThreeStoreRecoveryV1::Apply(recovered) => recovered,
        DurableOwnedThreeStoreRecoveryV1::AlreadyApplied => panic!("base stores cannot report already applied"),
    };
    for prepared in [&recovered.parent.prepared, &recovered.drawing.prepared, &recovered.value.prepared] {
        prepared.seal.authority.validate_prepared(prepared).expect("recovery reconstructs a valid private Store seal");
        assert_eq!(prepared.edit.mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    }
    let mut tampered = decision.clone();
    tampered.value.recovery_pack[0] ^= 1;
    assert!(tampered.recover_store_owned(&parent_store, &drawing_store, &value_store).is_err());
    drop(recovered);
    drop(bound);
    close_demo_artifact_store(&mut value_store);
    close_demo_artifact_store(&mut drawing_store);
    close_demo_artifact_store(&mut parent_store);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_private_committed_record_recovers_all_three_stores_without_reappending_journal() {
    let (parent, drawing, value) = owned_three_stores().await;
    let bound = bound_three(&parent, &drawing, &value);
    let decision = bound.decision.clone();
    let record = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("committed recovery record remains canonical");
    let receipt = DurableOwnedGroupJournalReceiptV1 { anchor_sha256: decision.anchor_sha256.clone(), decision_sha256: decision.decision_sha256.clone(), transaction_id: 71, segment_index: 5 };
    drop(bound);
    let admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let mut host = match admission.restore_untrusted_committed_record(&record, receipt.clone()).unwrap_or_else(|_| panic!("base frontiers return every owner through a retained recovery host")) {
        DurableOwnedMapRecoveryStartV1::Apply(host) => host,
        DurableOwnedMapRecoveryStartV1::AlreadyApplied(_) => panic!("base frontiers cannot report an applied decision"),
    };
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::StagingParent));
    assert!(matches!(host.advance(grant), DurableOwnedMapRecoveryAdvanceV1::Progress(_)), "committed parent stages before a recoverable drawing dependency fault");
    assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing));
    let drawing_retirement_factory = host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).and_then(|drawing| drawing.snapshot_retirement_factory.take());
    assert_eq!(host.advance(grant), DurableOwnedMapRecoveryAdvanceV1::Fault(DurableOwnedGroupDecisionError::InvalidFrontier));
    assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing));
    let all_old = host.capture_snapshot().expect("pre-visibility recovery fault keeps one complete old triple visible");
    assert_eq!([all_old.parent.n, all_old.drawing.n, all_old.value.n], [Some(0), Some(0), Some(0)]);
    *host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).expect("retained recovery keeps the exact drawing Store").snapshot_retirement_factory = drawing_retirement_factory;
    let mut observed_post_visibility_retry = false;
    for _ in 0..128 {
        if !observed_post_visibility_retry && host.phase() == Some(DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing) {
            let drawing_retirement_factory = host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).and_then(|drawing| drawing.snapshot_retirement_factory.take());
            assert_eq!(host.advance(grant), DurableOwnedMapRecoveryAdvanceV1::Fault(DurableOwnedGroupDecisionError::InvalidOutcome));
            assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing));
            let all_new = host.capture_snapshot().expect("post-visibility recovery fault keeps one complete new triple visible");
            assert_eq!([all_new.parent.n, all_new.drawing.n, all_new.value.n], [Some(7), Some(11), Some(13)]);
            *host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).expect("retained recovery keeps the exact drawing Store after visibility").snapshot_retirement_factory = drawing_retirement_factory;
            observed_post_visibility_retry = true;
        }
        match host.advance(grant) {
            DurableOwnedMapRecoveryAdvanceV1::AwaitingAcknowledgement => assert!(host.acknowledge_restoration(&receipt)),
            DurableOwnedMapRecoveryAdvanceV1::Complete => break,
            DurableOwnedMapRecoveryAdvanceV1::Progress(_) | DurableOwnedMapRecoveryAdvanceV1::Blocked => {}
            DurableOwnedMapRecoveryAdvanceV1::Fault(error) => panic!("committed recovery retained an unexpected fault: {error}"),
        }
    }
    assert!(observed_post_visibility_retry);
    let DurableOwnedMapRecoveryOwnersV1 { parent, drawing, value } = host.take_terminal_owners().expect("terminal recovery ACK returns all three exact Stores");
    assert!(host.terminal_is_empty());
    assert_eq!([parent.fold_current().await.unwrap().n, drawing.fold_current().await.unwrap().n, value.fold_current().await.unwrap().n], [Some(7), Some(11), Some(13)]);
    let replay = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("same committed decision re-admits canonically");
    let replay_admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let DurableOwnedMapRecoveryStartV1::AlreadyApplied(DurableOwnedMapRecoveryOwnersV1 { parent, mut drawing, mut value }) =
        replay_admission.restore_untrusted_committed_record(&replay, receipt.clone()).unwrap_or_else(|_| panic!("all-post replay remains idempotent"))
    else {
        panic!("all three post-frontiers must not publish twice")
    };
    let (mut unused_base_parent, base_drawing, base_value) = owned_three_stores().await;
    let mixed_record = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("mixed-frontier recovery record remains canonical");
    let mixed_admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, base_drawing, base_value);
    let DurableOwnedMapRecoveryRejectedV1 { error, mut parent, drawing: mut mixed_drawing, value: mut mixed_value } = match mixed_admission.restore_untrusted_committed_record(&mixed_record, receipt.clone()) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a committed decision cannot reconstruct over mixed base and post frontiers"),
    };
    assert_eq!(error, DurableOwnedGroupDecisionError::InvalidFrontier);
    close_demo_artifact_store(&mut mixed_value);
    close_demo_artifact_store(&mut mixed_drawing);
    close_demo_artifact_store(&mut parent);
    close_demo_artifact_store(&mut unused_base_parent);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    let (parent, drawing, value) = owned_three_stores().await;
    let mut foreign_receipt = receipt;
    foreign_receipt.decision_sha256 = "00".repeat(32);
    let foreign_record = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("foreign-receipt recovery record remains canonical");
    let foreign_admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let DurableOwnedMapRecoveryRejectedV1 { error, mut parent, mut drawing, mut value } = match foreign_admission.restore_untrusted_committed_record(&foreign_record, foreign_receipt) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a foreign committed receipt cannot enter Store recovery"),
    };
    assert_eq!(error, DurableOwnedGroupDecisionError::InvalidHash);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::Commit, state: Arc::clone(&state) };
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::StartingJournal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("fixed-three staging turn");
        let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("pending partial staging reads all-old");
        assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
    }
    let decision_bytes = coordinator.decision_pack.as_ref().expect("starting journal retains decision bytes").len();
    let one_byte = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, one_byte).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
    assert_eq!(state.lock().unwrap().begins, 0);
    let insufficient = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: decision_bytes - 1 };
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, insufficient).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
    assert_eq!(state.lock().unwrap().begins, 0);
    let exact = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: decision_bytes };
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, exact).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Progress(DurableOwnedThreeStoreCommitPhaseV1::Journal));
    assert_eq!(state.lock().unwrap().begins, 1);
    let mut observed_pending = false;
    let mut observed_committed = false;
    let mut rejected_partial_committed_root = false;
    let mut rejected_foreign_visibility = false;
    let mut rejected_structurally_false_adoption = false;
    for _ in 0..64 {
        let step = coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("retained group commit turn");
        let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("one captured group read");
        if coordinator.phase() == DurableOwnedThreeStoreCommitPhaseV1::Journal {
            assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
            assert!(parent.set_local_actor_id(None).is_err());
            assert!(drawing.invalidate_after_replay().is_err());
            observed_pending = true;
        }
        if matches!(
            coordinator.phase(),
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingParent
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingValue
                | DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck
        ) {
            assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(7), Some(11), Some(13)]);
            if !rejected_partial_committed_root {
                let retained_value_root = value.durable_group_root.take().expect("committed value root");
                assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                *value.durable_group_root = Some(retained_value_root);
                rejected_partial_committed_root = true;
            }
            if !rejected_foreign_visibility {
                let foreign_owner = crate::os_vcs::ArtifactGroupVisibilityOwner::new();
                let original_visibility = std::mem::replace(&mut value.durable_group_root.as_mut().expect("committed value root").visibility, foreign_owner.view());
                assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                value.durable_group_root.as_mut().expect("committed value root").visibility = original_visibility;
                drop(foreign_owner);
                rejected_foreign_visibility = true;
            }
            if !rejected_structurally_false_adoption && coordinator.phase() == DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease {
                parent.durable_group_root.as_mut().expect("committed parent root").adopted = true;
                assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                parent.durable_group_root.as_mut().expect("committed parent root").adopted = false;
                rejected_structurally_false_adoption = true;
            }
            assert_erased_snapshot_authority(&mut parent, 7).await;
            assert_erased_snapshot_authority(&mut drawing, 11).await;
            assert_erased_snapshot_authority(&mut value, 13).await;
            observed_committed = true;
        }
        if let DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt) = step {
            assert!(coordinator.acknowledge(&receipt));
        }
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(observed_pending && observed_committed && rejected_partial_committed_root && rejected_foreign_visibility && rejected_structurally_false_adoption && coordinator.terminal_is_empty());
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [1, 1, 1]);
    assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [1, 1, 1]);
    let journal = state.lock().unwrap();
    assert_eq!((journal.begins, journal.advances), (1, 2));
    assert_eq!(journal.close_count, 1);
    assert!(!journal.cancelled && !journal.decision_pack.is_empty());
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::AbsentAfterCancel, state: Arc::clone(&state) };
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
    }
    assert!(coordinator.cancel());
    for _ in 0..64 {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("cancel resolution turn");
        let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("cancelled captured group read");
        assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
    assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [0, 0, 0]);
    let journal = state.lock().unwrap();
    assert!(journal.cancelled);
    assert_eq!(journal.close_count, 1);
    assert_eq!(journal.advances, 2);
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::Commit, state: Arc::clone(&state) };
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("parent stages before injected drawing failure");
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing);
    let drawing_retirement_factory = drawing.snapshot_retirement_factory.take();
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant), Err(DurableOwnedGroupDecisionError::InvalidFrontier));
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::AbortingValue);
    *drawing.snapshot_retirement_factory = drawing_retirement_factory;
    for _ in 0..64 {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("retained stage-error owner continues abort cleanup");
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    assert!(parent.durable_group_root.is_none() && drawing.durable_group_root.is_none() && value.durable_group_root.is_none());
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
    let journal = state.lock().unwrap();
    assert_eq!((journal.begins, journal.close_count), (0, 0));
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::ErrorThenCommit, state: Arc::clone(&state) };
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
    }
    coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("journal first pending turn");
    assert!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).is_err());
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::Journal);
    let pending = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("uncertain journal read remains coherent");
    assert_eq!([pending.parent.n, pending.drawing.n, pending.value.n], [Some(0), Some(0), Some(0)]);
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances), (1, 2));
    }
    for _ in 0..64 {
        let step = coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("same journal owner retries");
        if let DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt) = step {
            assert!(coordinator.acknowledge(&receipt));
        }
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances), (1, 3));
    }
    let committed = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("retried journal commit read");
    assert_eq!([committed.parent.n, committed.drawing.n, committed.value.n], [Some(7), Some(11), Some(13)]);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff() {
    let (parent, drawing, value) = owned_three_stores().await;
    let coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained Map coordinator before mounting any request-facing turn");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let sink = Box::new(FakeJournalSink { resolution: FakeJournalResolution::ErrorThenCommit, state: Arc::clone(&state) });
    let mut host = coordinator.mount_map(parent, drawing, value, sink);
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    let mut observed_uncertain_error = false;
    let mut observed_committed = false;
    for _ in 0..128 {
        let request_observation = host.advance(grant);
        match request_observation {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                observed_uncertain_error = true;
                assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::Journal));
                let snapshot = host.capture_snapshot().expect("dropped request observation leaves the fixed host's coherent Map read retained");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt)) => {
                observed_committed = true;
                let snapshot = host.capture_snapshot().expect("mounted committed observation retains one coherent Map read");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(7), Some(11), Some(13)]);
                drop(receipt);
                assert!(host.acknowledge(), "only the receipt retained by the fixed host can unlock terminal handoff");
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
            Err(error) => panic!("mounted Map operation returned an unexpected terminal error: {error}"),
        }
    }
    assert!(observed_uncertain_error && observed_committed);
    let owners = host.take_terminal_owners().expect("terminal Map host returns every live Store and sink owner exactly once");
    assert!(host.terminal_is_empty());
    assert!(host.take_terminal_owners().is_none());
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
    }
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence() {
    let (parent, drawing, value) = owned_three_stores().await;
    let coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained Map coordinator before mounting any request-facing turn");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let sink = Box::new(FakeJournalSink { resolution: FakeJournalResolution::ErrorThenAbsentAfterCancel, state: Arc::clone(&state) });
    let mut host = coordinator.mount_map(parent, drawing, value, sink);
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    let mut dropped_uncertain_request = false;
    for _ in 0..128 {
        match host.advance(grant) {
            Err(DurableOwnedGroupDecisionError::Codec(error)) if error == "uncertain sync result" => {
                dropped_uncertain_request = true;
                assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::Journal));
                assert!(host.cancel());
                let snapshot = host.capture_snapshot().expect("uncertain cancellation retains one all-old Map snapshot");
                assert_eq!([snapshot.parent.n, snapshot.drawing.n, snapshot.value.n], [Some(0), Some(0), Some(0)]);
            }
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => break,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(_) | DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => {}
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => panic!("trusted absence cannot produce a commit acknowledgement"),
            Err(error) => panic!("fixed Map host returned an unexpected cancellation error: {error}"),
        }
    }
    assert!(dropped_uncertain_request);
    let owners = host.take_terminal_owners().expect("trusted absence terminates the fixed host before owner handoff");
    assert!(host.terminal_is_empty());
    let DurableOwnedMapCommitOwnersV1 { mut parent, mut drawing, mut value, sink } = owners;
    drop(sink);
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
    assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [0, 0, 0]);
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
        assert!(journal.cancelled);
    }
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::WrongAnchorThenAbsentAfterCancel, state: Arc::clone(&state) };
    let grant = crate::os_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: DURABLE_OWNED_GROUP_EVENT_MAX_BYTES };
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
    }
    coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("journal first pending turn");
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant), Err(DurableOwnedGroupDecisionError::InvalidHash));
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::Journal);
    let pending = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("foreign receipt cannot flip visibility");
    assert_eq!([pending.parent.n, pending.drawing.n, pending.value.n], [Some(0), Some(0), Some(0)]);
    assert!(coordinator.cancel());
    for _ in 0..64 {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("trusted absence cleanup turn");
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    let journal = state.lock().unwrap();
    assert_eq!((journal.begins, journal.advances, journal.close_count), (1, 3, 1));
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[test]
fn durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess() {
    let fixture = fixture();
    let numeric = DslValue::Object(vec![("u64".into(), DslValue::uint(u64::MAX)), ("i64".into(), DslValue::int(i64::MIN)), ("float".into(), DslValue::float(1.5))]);
    let encoded = crate::os_pack::json::to_json_string(&numeric);
    for row in fixture["carrier"]["numericCases"].as_array().unwrap() {
        assert!(encoded.contains(row["canonical"].as_str().unwrap()));
    }
    let decoded: DslValue = parse_canonical_json_value(encoded.as_bytes()).expect("typed canonical JSON preserves numeric tags and extrema");
    assert_eq!(decoded, numeric);
    let clock = HybridLogicalTimestamp { actor: u64::MAX, physical_ms: u64::MAX, logical: u64::MAX };
    let clock_json = crate::os_pack::json::to_json_string(&clock);
    assert!(clock_json.matches("18446744073709551615").count() == 3);
    assert_eq!(parse_canonical_json_value::<HybridLogicalTimestamp>(clock_json.as_bytes()).unwrap(), clock);

    let control = fixture["carrier"]["escapedControlIdentity"].as_str().unwrap();
    assert!(!valid_identity(control));
    let raw_control = b"\"map\0x\"";
    assert_eq!(validate_json_budget(raw_control), Err(DurableOwnedGroupDecisionError::NonCanonical));
    let over_depth = format!("{}0{}", "[".repeat(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH + 1), "]".repeat(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH + 1));
    assert_eq!(validate_json_budget(over_depth.as_bytes()), Err(DurableOwnedGroupDecisionError::InvalidOutcome));
    let over_items = format!("[{}]", std::iter::repeat_n("0", DURABLE_OWNED_GROUP_JSON_MAX_ITEMS + 1).collect::<Vec<_>>().join(","));
    assert!(over_items.len() <= DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES);
    assert_eq!(validate_json_budget(over_items.as_bytes()), Err(DurableOwnedGroupDecisionError::InvalidOutcome));
}

#[test]
fn durable_decision_rejects_deflate_expansion_before_document_body_allocation() {
    let decision = decision();
    let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<DurableOwnedThreeMemberDecisionV1 as ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).unwrap();
    let mut expanded_record = decision.__dsl_to_record();
    expanded_record.fields.insert(u16::MAX, FieldValue::Bytes64(vec![0; DURABLE_OWNED_GROUP_EVENT_MAX_BYTES + 1]));
    let mut expanded_options = PackEncodeOptions::default();
    expanded_options.chunk_threshold = u64::MAX;
    expanded_options.frame_size = u64::MAX;
    let expanded_inner = pack_rt::encode_document(&DurableOwnedThreeMemberDecisionV1::__dsl_spec(), &expanded_record, &expanded_options).unwrap();
    let expanded = crate::os_store::semio_format::wrap_binary(&envelope, &expanded_inner);
    assert!(expanded.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&expanded), Err(DurableOwnedGroupDecisionError::EventTooLarge));

    let mut framed_options = PackEncodeOptions::default();
    framed_options.frame_size = 64;
    let framed_inner = pack_rt::encode_document(&DurableOwnedThreeMemberDecisionV1::__dsl_spec(), &decision.__dsl_to_record(), &framed_options).unwrap();
    let framed = crate::os_store::semio_format::wrap_binary(&envelope, &framed_inner);
    assert!(framed.len() <= DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&framed), Err(DurableOwnedGroupDecisionError::NonCanonical));
}
