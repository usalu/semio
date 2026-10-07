//! 🛂️ Store laws of the supersede law and of cross-artifact units (design §3, §22.1, ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every row of the language-agnostic decision tables
//! (`🧫️fixtures/🧫️supersede-law`, whose independent twin is `🟦️.ts` beside this file) holds for `admit_replacement`, for what a
//! replica folds and persists, and for what authoring a supersession answers. A withdrawn operation that plans foreign steps
//! folds as a no-op at every fold site and on every replica, and its undo — its own recorded input — restores it.
use super::*;

//#region 🧰️Harness
/// 🧪️ A demo operation with the capabilities these laws need, told by its value so every codec round-trips them: an `addN`
/// of 1000 or more plans foreign steps, and from 2000 on its plan has one; a `setN` of 13 refuses its inverse on a base of
/// 100 or more.
#[derive(Clone, Debug, PartialEq)]
struct LawOp(DemoMutation);

impl LawOp {
    /// 🔢️ The delta of an `addN`.
    fn delta(&self) -> Option<i32> {
        match &self.0 {
            DemoMutation::AddN(AddN { delta }) => Some(*delta),
            _ => None,
        }
    }
}

impl ToValue for LawOp {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

impl FromValue for LawOp {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoMutation::from_value(value).map(Self)
    }
}

impl OpBinary for LawOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        self.0.encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        DemoMutation::decode_op(bytes).map(Self)
    }
}

impl OpText for LawOp {
    fn print_op(&self) -> String {
        self.0.print_op()
    }

    fn parse_op(line: &str) -> Result<Self, TextError> {
        DemoMutation::parse_op(line).map(Self)
    }
}

impl Mutation<DemoSnapshot> for LawOp {
    type Diff = <DemoMutation as Mutation<DemoSnapshot>>::Diff;
    const DESCRIPTORS: &'static [crate::os_spr::MutationLeafDescriptor] = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static crate::os_spr::MutationLeafDescriptor {
        self.0.descriptor()
    }

    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<Self::Diff> {
        self.0.diff(base)
    }

    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if matches!(&self.0, DemoMutation::SetN(SetN { n: 13 })) && base.n.is_some_and(|n| n >= 100) {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "a setN of 13 has no inverse on a base of 100 or more"));
        }
        Ok(self.0.inverse(base)?.into_iter().map(Self).collect())
    }

    fn conflict_target(&self) -> Vec<String> {
        self.0.conflict_target()
    }

    fn may_emit_foreign_steps(&self) -> bool {
        self.delta().is_some_and(|delta| delta >= 1000)
    }

    fn foreign_steps(&self, _base: &DemoSnapshot) -> Vec<crate::os_spr::ForeignStep> {
        if self.delta().is_some_and(|delta| delta >= 2000) {
            return vec![crate::os_spr::ForeignStep { target: elsewhere(), mutation_id: SchemaId("demo/v1#setN".into()), payload: vec![1], label: "notify".into() }];
        }
        Vec::new()
    }
}

impl MemberStoreOwner<LawOp> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> DocumentStoreOwners<Self, LawOp> {
        DocumentStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, LawOp>::new()))
    }
}

type LawStore = ArtifactStore<DemoSnapshot, LawOp>;

/// 🌍️ The other document a law operation's foreign step targets.
fn elsewhere() -> crate::os_spr::ForeignTarget {
    crate::os_spr::ForeignTarget { artifact_id: "elsewhere".into(), artifact_kind: "demo".into(), dialect: None }
}

fn corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️supersede-law/🔣️.json")).expect("the corpus parses")
}

async fn law_store(id: &str) -> LawStore {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, LawOp>("demo/v1", id, DemoSnapshot { n: Some(0) }, None)).await
}

async fn apply(store: &mut LawStore, operation: LawOp) {
    store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("a clean edit applies");
}

fn set(n: i32) -> LawOp {
    LawOp(DemoMutation::SetN(SetN { n }))
}

fn add(delta: i32) -> LawOp {
    LawOp(DemoMutation::AddN(AddN { delta }))
}

fn operation_ids(store: &LawStore) -> Vec<MutationId> {
    store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect()
}

fn input(schema: &str, operation: &LawOp) -> protocol::InputReplacement {
    protocol::InputReplacement::Input { schema: schema.into(), payload: operation.encode_op().expect("demo operations encode") }
}

fn supersede(target: &MutationId, replacement: Option<LawOp>) -> ArtifactCommand<LawOp> {
    ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: target.clone(), replacement }] }
}

/// ✉️ Another replica's unscoped supersession of `target`, authored far after every edit.
fn remote_supersession(document: &str, target: &MutationId, replacement: protocol::InputReplacement, logical: u64) -> crate::os_spr::MutationEnvelope {
    let transition = crate::os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs: vec![protocol::SupersededInput { target: target.clone(), replacement }] });
    crate::os_spr::history_transition_envelope(&transition, &ArtifactId(document.to_string()), &ActorId("editor".into()), vec![target.clone()], HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical })
}

/// 🧱️ The recorded operation and the replacement an input row of the corpus names.
fn staged(row: &serde_json::Value) -> (LawOp, protocol::InputReplacement) {
    let original = if row["original"].as_str() == Some("plain") { add(2) } else { add(1000) };
    let replacement = match row["replacement"].as_str().expect("a replacement kind") {
        "withdrawn" => protocol::InputReplacement::Withdrawn,
        "recorded" => input("demo/v1", &original),
        "plain" => input("demo/v1", &add(5)),
        "foreignStep" => input("demo/v1", &add(1001)),
        "otherSchema" => input("other/v1", &add(5)),
        "garbage" => protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: vec![0xff, 0x13, 0x37] },
        kind => panic!("unknown replacement kind {kind}"),
    };
    (original, replacement)
}

async fn reloaded(store: &LawStore) -> LawStore {
    let files = print_document_pack(store.envelope()).await.expect("the pair prints");
    ArtifactStore::new(parse_document_pack::<DemoSnapshot, LawOp>(&files.pack, &files.spr).await.expect("the pair parses").into_envelope()).await
}
//#endregion 🧰️Harness

//#region 🧪️InputLaws
/// 🧫️ Every input row of the corpus: `admit_replacement` admits exactly what the table admits, and a replica that receives
/// the supersession folds, reports and persists exactly what the table says — the replacement in the operation's place, a
/// no-op for a withdrawal, a no-op carrying one `Fatal` `mutation.invariant` for an input the law refuses.
#[semio_framework_async_macros::async_test]
async fn every_input_row_is_admitted_and_folded_as_the_table_says() {
    for row in corpus()["inputs"].as_array().expect("input rows") {
        let name = row["name"].as_str().expect("a row name");
        let (original, replacement) = staged(row);
        let admitted = admit_replacement::<DemoSnapshot, LawOp>(&original, &replacement, "demo/v1");
        assert_eq!(admitted.is_ok(), row["admitted"].as_bool().expect("admitted"), "{name}: {admitted:?}");
        let folds = row["folds"].as_str().expect("folds");
        let folded = match (&admitted, folds) {
            (Ok(Some(operation)), "replacement") => operation.delta().expect("an addN"),
            (Ok(None), "noOp") | (Err(_), "fatalNoOp") => 0,
            other => panic!("{name}: the law answered {other:?}"),
        };
        let mut store = law_store(name).await;
        apply(&mut store, set(1)).await;
        apply(&mut store, original).await;
        apply(&mut store, add(7)).await;
        let target = operation_ids(&store)[1].clone();
        let report = store.ingest_remote(remote_supersession(name, &target, replacement, 0)).await.expect("a replica never refuses a supersession it cannot take");
        assert!(report.accepted && report.conflict.is_none(), "{name}");
        assert_eq!(store.snapshot_ref().n, Some(8 + folded), "{name}: folded state");
        let outcome = store.mutation_outcomes().expect("durable outcomes").into_iter().find(|outcome| outcome.mutation_id == target).expect("the target's outcome");
        let fatal = outcome.messages.iter().filter(|message| message.level == semio_framework_diagnostic::Severity::Fatal).map(|message| message.code.0.as_str()).collect::<Vec<_>>();
        assert_eq!((fatal, outcome.superseded, outcome.withdrawn), (if folds == "fatalNoOp" { vec!["mutation.invariant"] } else { Vec::new() }, true, folds == "noOp"), "{name}: outcome");
        test_support::assert_live_equals_replay(&store).await;
        assert_eq!(reloaded(&store).await.snapshot_ref().n, Some(8 + folded), "{name}: reloaded state");
    }
}

/// 🚪️ A withdrawn operation that plans foreign steps folds as a no-op at every fold site and on every replica, without a
/// fault; another input is refused at authoring with nothing recorded; the undo of the withdrawal — the operation's own
/// recorded input — restores it.
#[semio_framework_async_macros::async_test]
async fn a_withdrawn_planner_folds_as_a_no_op_at_every_site_and_its_recorded_input_restores_it() {
    let mut store = law_store("planner-withdraw").await;
    apply(&mut store, set(1)).await;
    apply(&mut store, add(1000)).await;
    apply(&mut store, add(3)).await;
    let ids = operation_ids(&store);
    assert_eq!(store.snapshot_ref().n, Some(1004));
    store.dispatch(supersede(&ids[1], None)).await.expect("an operation that plans foreign steps is withdrawn");
    assert_eq!(store.snapshot_ref().n, Some(4));
    let outcome = store.mutation_outcomes().expect("durable outcomes").into_iter().find(|outcome| outcome.mutation_id == ids[1]).expect("the planner's outcome");
    assert_eq!((outcome.worst, outcome.superseded, outcome.withdrawn), (None, true, true), "a withdrawal carries no fault");
    assert_eq!(store.state_before(&ids[2], &BTreeMap::new()).expect("state before").n, Some(1));
    assert_eq!(materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("materialized").n, Some(4));
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(reloaded(&store).await.snapshot_ref().n, Some(4));
    let text = print_document_text(store.envelope()).await.expect("the text prints");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, LawOp>(&text.dsl, &text.ops).await.expect("the text parses").into_envelope()).await;
    assert_eq!(mirrored.snapshot_ref().n, Some(4));
    let log = store.event_log().expect("log");
    for rotation in 0..log.len() {
        let mut replica = law_store("planner-withdraw").await;
        let mut arrival = log.clone();
        arrival.rotate_left(rotation);
        for event in arrival {
            replica.ingest_remote(event).await.expect("a replica ingests the log");
        }
        assert_eq!((replica.snapshot_ref().n, replica.supersessions().contains_key(&ids[1])), (Some(4), true), "rotation {rotation}");
    }
    let (generation, transitions) = (store.generation(), store.envelope().transitions.len());
    let refused = store.dispatch(supersede(&ids[1], Some(add(1001)))).await;
    assert!(matches!(refused, Err(VcsError::ValidationFailed(_))), "{refused:?}");
    assert_eq!((store.generation(), store.envelope().transitions.len(), store.snapshot_ref().n), (generation, transitions, Some(4)), "a refused input records nothing");
    store.dispatch(supersede(&ids[1], Some(add(1000)))).await.expect("the recorded input restores the planner");
    assert_eq!(store.snapshot_ref().n, Some(1004));
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(reloaded(&store).await.snapshot_ref().n, Some(1004));
}
//#endregion 🧪️InputLaws

//#region 🧪️UnitLaws
/// 🧷️ Every unit row of the corpus: the store names the cross-artifact unit the table names (the gesture's group where it
/// holds one, else the operation), and authoring a supersession of an operation of a unit is refused with
/// `UnitSpansDocuments` — fault code `history.unit-spans-documents` — with nothing recorded, while every other operation,
/// a planner whose plan is empty included, withdraws.
#[semio_framework_async_macros::async_test]
async fn every_unit_row_names_its_unit_and_refuses_its_supersession_as_the_table_says() {
    for row in corpus()["units"].as_array().expect("unit rows") {
        let name = row["name"].as_str().expect("a row name");
        let mut store = law_store(name).await;
        apply(&mut store, set(1)).await;
        let operation = match row["operation"].as_str().expect("an operation kind") {
            "plain" => add(2),
            "planner" => add(1000),
            "foreignPlanner" => add(2000),
            kind => panic!("unknown operation kind {kind}"),
        };
        apply(&mut store, operation).await;
        if row["group"].as_bool().expect("group") {
            SpaceMember::stamp_tail_group_id(&mut store, "gesture-1").await.expect("a group stamp");
        }
        if row["origin"].as_str() == Some("transaction") {
            SpaceMember::stamp_tail_origin(&mut store, crate::os_spr::MutationOrigin::Transaction { initiator: elsewhere() }).await.expect("an origin stamp");
        }
        let target = operation_ids(&store)[1].clone();
        let unit = match row["unit"].as_str().expect("a unit") {
            "none" => None,
            "group" => Some("gesture-1".to_string()),
            "operation" => Some(target.0.clone()),
            unit => panic!("unknown unit {unit}"),
        };
        assert_eq!(store.unit_id(&target).expect("the unit"), unit, "{name}: unit");
        hot_path_census::take();
        assert_eq!(store.applied_mutation(&target).expect("the indexed opening row").unit, unit, "{name}: indexed opening unit");
        assert_eq!(hot_path_census::take(), hot_path_census::HotPathCensus::default(), "{name}: opening consumes captured unit facts without prefix folding or hashing");
        assert_eq!(store.unit_operations(&target).expect("the unit's operations"), vec![target.clone()], "{name}: this store's part");
        let (generation, transitions, state) = (store.generation(), store.envelope().transitions.len(), store.snapshot_ref().n);
        let authored = store.dispatch(supersede(&target, None)).await;
        match row["supersede"].as_str().expect("a supersede verdict") {
            "authored" => {
                authored.unwrap_or_else(|error| panic!("{name}: an operation that is its own unit withdraws, got {error}"));
                assert_eq!(store.snapshot_ref().n, Some(1), "{name}: the withdrawn operation folds as a no-op");
                test_support::assert_live_equals_replay(&store).await;
            }
            code => {
                let error = authored.expect_err("an operation of a cross-artifact unit takes no supersession in one document alone");
                assert!(matches!(&error, VcsError::UnitSpansDocuments { mutation_id, unit: named } if *mutation_id == target.0 && Some(named) == unit.as_ref()), "{name}: {error:?}");
                assert_eq!(semio_framework_diagnostic::FaultFrom::fault_code(&error).0, code, "{name}: fault code");
                assert_eq!((store.generation(), store.envelope().transitions.len(), store.snapshot_ref().n), (generation, transitions, state), "{name}: nothing recorded");
            }
        }
    }
}
//#endregion 🧪️UnitLaws

//#region 🧪️InverseRefusedLaws
/// 🧯️ One leaf whose inverse is refused on the replayed state is one mutation's `Fatal` `mutation.inverse-refused`, not a
/// faulted replay (design §22.5): the Report replay runs to its end and reports every other mutation, the operation folds
/// forward as every other fold site folds it, the report blocks finalizing, and the session is repaired through that row —
/// withdrawing the failing mutation makes the report clean and the finalize lands. A replica that ingests such a
/// supersession keeps running, shows the fatal row and reloads it; a new local command whose inverse is refused is still
/// refused whole.
#[semio_framework_async_macros::async_test]
async fn a_refused_inverse_is_one_fatal_mutation_and_the_session_stays_repairable() {
    let mut store = law_store("inverse-refused").await;
    apply(&mut store, set(1)).await;
    apply(&mut store, set(13)).await;
    apply(&mut store, add(3)).await;
    let ids = operation_ids(&store);
    let finished = |store: &LawStore, drafts: &BTreeMap<MutationId, protocol::InputReplacement>| {
        let mut replay = store.begin_report_replay(drafts, None).expect("the session replay");
        assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("a Report replay never aborts on a refused inverse"), ReplayStep::Finished(_)));
        replay.finish().expect("a finished replay yields its result")
    };
    let codes = |outcomes: &[protocol::MutationReplayOutcome], target: &MutationId| outcomes.iter().find(|outcome| outcome.mutation_id == *target).map(|outcome| (outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>())).expect("the mutation's outcome");
    let fatal = (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.inverse-refused".to_string()]);
    let mut drafts: BTreeMap<MutationId, protocol::InputReplacement> = [(ids[0].clone(), input("demo/v1", &set(500)))].into_iter().collect();
    let blocked = finished(&store, &drafts);
    let report = store.replay_report(&blocked).expect("the report");
    assert_eq!(codes(&report.outcomes, &ids[1]), fatal, "the refused inverse is that mutation's fatal outcome");
    assert_eq!(codes(&report.outcomes, &ids[2]).0, Some(semio_framework_diagnostic::Severity::Info), "the replay kept reporting the mutations after it");
    assert_eq!((blocked.state().expect("the reached state").n, report.blocks_finalize()), (Some(16), true));
    let generation = store.generation();
    let refused = store.commit_finished_replay(blocked, HistoryFinalization::Overwrite).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.inverse-refused")), "{refused:?}");
    assert_eq!(store.generation(), generation, "a blocked finalize changes nothing");
    drafts.insert(ids[1].clone(), protocol::InputReplacement::Withdrawn);
    let repaired = finished(&store, &drafts);
    assert!(!store.replay_report(&repaired).expect("the report").blocks_finalize(), "withdrawing the failing mutation repairs the session");
    store.commit_finished_replay(repaired, HistoryFinalization::Overwrite).await.expect("the repaired session finalizes");
    assert_eq!(store.snapshot_ref().n, Some(503));
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(reloaded(&store).await.snapshot_ref().n, Some(503));

    let mut replica = law_store("inverse-refused-replica").await;
    apply(&mut replica, set(1)).await;
    apply(&mut replica, set(13)).await;
    apply(&mut replica, add(3)).await;
    let remote = operation_ids(&replica);
    let report = replica.ingest_remote(remote_supersession("inverse-refused-replica", &remote[0], input("demo/v1", &set(500)), 0)).await.expect("a replica never refuses a supersession for a leaf's refused inverse");
    assert!(report.accepted && report.conflict.is_none());
    assert_eq!(replica.snapshot_ref().n, Some(16));
    assert_eq!(codes(&replica.mutation_outcomes().expect("durable outcomes"), &remote[1]), fatal, "the replica shows the fatal row");
    test_support::assert_live_equals_replay(&replica).await;
    let restored = reloaded(&replica).await;
    assert_eq!((restored.snapshot_ref().n, codes(&restored.mutation_outcomes().expect("durable outcomes"), &remote[1])), (Some(16), fatal), "a document holding the fatal row reloads");

    let mut local = law_store("inverse-refused-local").await;
    apply(&mut local, set(500)).await;
    let generation = local.generation();
    let refused = local.dispatch(ArtifactCommand::Apply { mutations: vec![set(13)], transaction: None }).await;
    assert!(matches!(refused, Err(VcsError::InverseRefused(_))), "a new command whose inverse is refused is refused whole");
    assert_eq!((local.generation(), local.snapshot_ref().n), (generation, Some(500)));
}
//#endregion 🧪️InverseRefusedLaws
