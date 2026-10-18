// 🌲️ Trunk, history shape and command decode laws: the new alternative trunk, the viewer head round trip, the config store undo and redo, the witness operations and the refused command decode retirement.

//#region 🧪️TrunkLaws
/// 🚉️ The alternatives a store lists as `(id, name, chain length)` and the alternative it is on.
fn alternative_view(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> (Vec<(String, String, usize)>, String) {
    (store.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.len())).collect(), store.active_line_id())
}

/// 🌳️ A new alternative preserves the trunk it branched from: after finalizing a history edit as a new alternative the
/// trunk is listed first (empty name, localized by the UI), switching to it restores the original positions with only
/// unscoped and trunk-scoped supersessions, switching back restores the edited ones, the log never names the trunk,
/// replicas holding the same events agree, and `.spr` and `.ops` reloads keep all of it.
#[semio_framework_async_macros::async_test]
async fn a_new_alternative_preserves_the_trunk_it_branched_from() {
    let mut store = demo_store("trunk", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    let ids = operation_ids(&store);
    let trunk = store.trunk_alternative_id();
    assert_eq!(store.active_line_id(), trunk, "every document starts on its trunk");
    assert!(store.envelope().vcs.alternatives.is_empty(), "a trunk without a checkpoint is not listed yet");
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("finalize as a new alternative");
    assert_eq!(store.snapshot_ref().n, Some(12));
    let (alternatives, current) = alternative_view(&store);
    assert_eq!(alternatives.iter().map(|(id, name, _)| (id.as_str(), name.as_str())).collect::<Vec<_>>()[0], (trunk.as_str(), ""), "the trunk is listed first");
    let edited = alternatives.iter().find(|(_, name, _)| name == "edited").map(|(id, _, _)| id.clone()).expect("the new alternative");
    assert_eq!(current, edited);
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk.clone() }).await.expect("switch back to the trunk");
    assert_eq!(store.snapshot_ref().n, Some(3), "the trunk keeps the original positions");
    assert_eq!((store.active_line_id(), store.envelope().active_alternative_id.clone()), (trunk.clone(), None));
    assert!(store.supersessions().is_empty(), "the edited alternative's scoped supersession stays on it");
    assert!(
        store.envelope().transitions.iter().all(|envelope| {
            match semio_framework_os_spr::history_transition_from_envelope(envelope).expect("transition").expect("history") {
                semio_framework_os_spr::HistoryTransition::Checkout { .. } => false,
                semio_framework_os_spr::HistoryTransition::Branch { alternative_id, .. } => alternative_id != trunk,
                semio_framework_os_spr::HistoryTransition::Commit(checkpoint) => checkpoint.line_id.as_deref() != Some(trunk.as_str()),
                _ => true,
            }
        }),
        "a head switch is local, and the log never names the trunk"
    );
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: Some(trunk.clone()), inputs: vec![input(&ids[1], Some(add(5)))] }).await.expect("a supersession scoped to the trunk");
    assert_eq!(store.snapshot_ref().n, Some(6));
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: edited.clone() }).await.expect("switch to the edited alternative");
    assert_eq!(store.snapshot_ref().n, Some(12), "the edited alternative keeps its positions and ignores the trunk's edit");
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk.clone() }).await.expect("and back again");
    assert_eq!(store.snapshot_ref().n, Some(6));
    let expected = alternative_view(&store);
    let log = store.event_log().expect("log");
    let mut peer = demo_store("trunk", Some(0)).await;
    for event in log.iter().rev().cloned() {
        peer.ingest_remote(event).await.expect("the peer ingests every event");
    }
    let mut other = demo_store("trunk", Some(0)).await;
    for event in log {
        other.ingest_remote(event).await.expect("another peer ingests them in authoring order");
    }
    for replica in [&peer, &other] {
        assert_eq!(alternative_view(replica), expected, "replicas list the same alternatives and stand on the same one");
        assert_eq!(replica.snapshot_ref().n, Some(6));
        assert_eq!(replica.supersessions(), store.supersessions());
    }
    assert_eq!(peer.0.content_revision(), other.0.content_revision(), "arrival order never changes a replica's identity");
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let mut reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("pair parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(alternative_view(&reloaded), expected, ".spr keeps the trunk");
    assert_eq!(reloaded.snapshot_ref().n, Some(6));
    let text = print_document_text(store.envelope()).await.expect("text prints");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("text parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(alternative_view(&mirrored), expected, ".ops keeps the trunk");
    assert_eq!(mirrored.snapshot_ref().n, Some(6));
    fixture_author(&mut reloaded, ArtifactCommand::SwitchAlternative { alternative_id: edited }).await.expect("a reloaded store switches to the edited alternative");
    assert_eq!(reloaded.snapshot_ref().n, Some(12));
}

/// 👁️ A replica standing on another alternative at an explicit checkpoint keeps that head through its `.spr` pack
/// (`REC_VIEWER`, audit W1G-9): the persisted log names the head, the reloaded store stands on the same alternative and
/// checkpoint with the same projection and supersessions, and the `.ops` text — the shared log, which carries no viewer head
/// — hydrates to the trunk tip.
#[semio_framework_async_macros::async_test]
async fn a_viewer_head_round_trips_through_the_spr_pack() {
    let mut store = demo_store("viewer", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("finalize as a new alternative");
    apply(&mut store, vec![add(3)]).await;
    fixture_author(&mut store, ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a checkpoint on the edited alternative");
    let edited = store.envelope().active_alternative_id.clone().expect("the edited alternative is active");
    let checkpoint = store.envelope().vcs.alternatives.iter().find(|alternative| alternative.id == edited).and_then(|alternative| alternative.checkpoint_ids.last().cloned()).expect("the edited alternative's newest checkpoint");
    apply(&mut store, vec![add(4)]).await;
    assert_eq!(store.snapshot_ref().n, Some(19));
    fixture_author(&mut store, ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint.clone() }).await.expect("look at the committed checkpoint");
    let head = (store.envelope().active_alternative_id.clone(), store.envelope().viewer_checkpoint_id.clone());
    assert_eq!(head, (Some(edited.clone()), Some(checkpoint.clone())), "the replica stands on the edited alternative at its checkpoint");
    assert_eq!(store.snapshot_ref().n, Some(15), "an explicit checkpoint hides the uncommitted edit after it");
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let log = semio_framework_os_spr::decode_history(&files.spr, &semio_framework_os_spr::DecodeOptions::default()).await.expect("spr decodes");
    assert_eq!((log.viewer_line.clone(), log.viewer_checkpoint.clone()), head, "the persisted log names the viewer head");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("pair parses").into_envelope(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!((reloaded.envelope().active_alternative_id.clone(), reloaded.envelope().viewer_checkpoint_id.clone()), head, ".spr keeps the viewer head");
    assert_eq!((reloaded.snapshot_ref().n, reloaded.supersessions()), (Some(15), store.supersessions()), ".spr reloads the viewer's projection");
    test_support::assert_document_pack_round_trip(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
}
//#endregion 🧪️TrunkLaws

//#region 🧪️HistoryShapeLaws
/// 🗂️ A config store's history holds undo and redo only: every command authoring another transition is refused at
/// dispatch with the typed `HistoryShape` refusal before anything runs — the store untouched, every replacement
/// retired — and so is a remote transition of another kind; undo and redo keep working. The store refuses exactly the
/// kinds the language-agnostic shape table excludes (`🧫️history-transition`, twinned in TypeScript and Python), and a
/// document store admits all of them.
#[semio_framework_async_macros::async_test]
async fn a_config_store_holds_undo_and_redo_only() {
    use semio_framework_os_spr::HistoryTransitionKind::Checkout; use semio_framework_os_spr::HistoryTransitionKind::Commit; use semio_framework_os_spr::HistoryTransitionKind::Supersede;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/🔗️causal/🧪️tests/🧫️fixtures/🧫️history-transition/🔣️.json")).expect("history transition fixture parses");
    let admitted = |shape: &str| -> Vec<String> {
        let row = fixture["shapes"].as_array().expect("shape rows").iter().find(|row| row["shape"] == shape).expect("a shape row");
        row["admits"].as_array().expect("admits").iter().map(|kind| kind.as_str().expect("kind").to_string()).collect()
    };
    let mut config = ArtifactStore::new(create_config_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "config", DemoSnapshot { n: Some(0) }, None).await, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    apply(&mut config, vec![set(1)]).await;
    apply(&mut config, vec![add(2)]).await;
    let ids = operation_ids(&config);
    let trunk = config.trunk_alternative_id();
    let untouched = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| (store.generation(), store.0.content_revision(), store.envelope().transitions.len(), store.snapshot_ref().n, store.supersessions().len());
    let before = untouched(&config);
    let refused = [
        (ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, Commit),
        (ArtifactCommand::CreateAlternative { name: "b".into() }, Commit),
        (ArtifactCommand::SwitchAlternative { alternative_id: trunk.clone() }, Checkout),
        (ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "c".into() }, Checkout),
        (ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(9)))] }, Supersede),
        (ArtifactCommand::Supersede { scope: Some(trunk), inputs: vec![input(&ids[1], None)] }, Supersede),
        (ArtifactCommand::CreateAlternativeWithSupersede { name: "b".into(), inputs: vec![input(&ids[0], Some(set(9)))] }, Commit),
    ];
    for (command, kind) in refused {
        let kinds = command.history_transition_kinds();
        assert!(kinds.iter().any(|kind| !admitted("config").contains(&kind.name().to_string())), "{kinds:?}: the shape table excludes a kind");
        assert!(kinds.iter().all(|kind| admitted("document").contains(&kind.name().to_string())), "{kinds:?}: a document holds every kind");
        assert_eq!(fixture_author(&mut config, command).await.err(), Some(VcsError::HistoryShape { shape: semio_framework_os_spr::HistoryShape::Config, kind }));
        assert_eq!(untouched(&config), before, "a refused command leaves the store untouched");
    }
    let remote = remote_supersession("config", &ids[0], draft(Some(set(9))), 0);
    assert_eq!(config.ingest_remote(remote).await.err(), Some(VcsError::HistoryShape { shape: semio_framework_os_spr::HistoryShape::Config, kind: Supersede }));
    assert_eq!(untouched(&config), before, "a refused remote transition is never recorded");
    for command in [ArtifactCommand::<DemoMutation>::Undo, ArtifactCommand::Redo] {
        assert!(command.history_transition_kinds().iter().all(|kind| admitted("config").contains(&kind.name().to_string())));
    }
    fixture_author(&mut config, ArtifactCommand::Undo).await.expect("a config store undoes");
    assert_eq!(config.snapshot_ref().n, Some(1));
    fixture_author(&mut config, ArtifactCommand::Redo).await.expect("and redoes");
    assert_eq!(config.snapshot_ref().n, Some(3));
    let mut document = demo_store("config", Some(0)).await;
    apply(&mut document, vec![set(1)]).await;
    let target = operation_ids(&document)[0].clone();
    fixture_author(&mut document, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&target, Some(set(9)))] }).await.expect("a document store supersedes");
    assert_eq!(document.snapshot_ref().n, Some(9));
}
//#endregion 🧪️HistoryShapeLaws

//#region 🧪️CommandDecodeLaws
std::thread_local! {
    static WITNESS_RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WITNESS_DROPPED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 🧿️ A demo operation with a fail-closed owner's discipline: its cold disposal is counted, and a bare drop — what a
/// partial decode used to do — is counted too, so a law can prove none happened.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct WitnessOp(DemoMutation, bool);

impl WitnessOp {
    pub(super) fn live(operation: DemoMutation) -> Self {
        Self(operation, true)
    }

    /// 📊️ `(retired, dropped)` so far on this thread.
    pub(super) fn tally() -> (usize, usize) {
        (WITNESS_RETIRED.with(std::cell::Cell::get), WITNESS_DROPPED.with(std::cell::Cell::get))
    }
}

impl Drop for WitnessOp {
    fn drop(&mut self) {
        if self.1 {
            WITNESS_DROPPED.with(|dropped| dropped.set(dropped.get() + 1));
        }
    }
}

impl ToValue for WitnessOp {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

impl FromValue for WitnessOp {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoMutation::from_value(value).map(Self::live)
    }
}

impl OpBinary for WitnessOp {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        self.0.encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        DemoMutation::decode_op(bytes).map(Self::live)
    }
}

impl OpText for WitnessOp {
    fn print_op(&self) -> String {
        self.0.print_op()
    }

    fn parse_op(line: &str) -> Result<Self, TextError> {
        DemoMutation::parse_op(line).map(Self::live)
    }
}

impl Mutation<DemoSnapshot> for WitnessOp {
    type Diff = <DemoMutation as Mutation<DemoSnapshot>>::Diff;
    const DESCRIPTORS: &'static [semio_framework_os_spr::MutationLeafDescriptor] = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static semio_framework_os_spr::MutationLeafDescriptor {
        self.0.descriptor()
    }

    fn diff(&self, base: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<Self::Diff> {
        self.0.diff(base)
    }

    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok({
        self.0.inverse(base)?.into_iter().map(Self::live).collect()
    
    })
}

    fn retire_cold(mut self) {
        self.1 = false;
        WITNESS_RETIRED.with(|retired| retired.set(retired.get() + 1));
    }
}

/// 🧷️ Command decoding is all-or-nothing, binary and text alike: a command whose operation list, supersede inputs or
/// nested command fails midway — or that carries bytes past its end — is refused with the typed `CommandDecodeError`
/// naming the failing entry, and every operation decoded before the failure retires through the technology's cold
/// disposal; none is ever dropped bare (audit F-m11b). A hostile operation count never allocates beyond the input.
#[semio_framework_async_macros::async_test]
async fn a_refused_command_decode_retires_every_operation_it_decoded() {
    let binary = |command: ArtifactCommand<DemoMutation>| command.encode_command().expect("encodes");
    let garbled = |command: ArtifactCommand<DemoMutation>, last: DemoMutation| {
        let mut bytes = binary(command);
        let at = bytes.len() - last.encode_op().expect("encodes").len();
        bytes[at] = 0xff;
        bytes
    };
    let decode = |bytes: &[u8]| ArtifactCommand::<WitnessOp>::decode_command::<DemoSnapshot>(bytes);
    let apply = |mutations: Vec<DemoMutation>| ArtifactCommand::Apply { mutations, transaction: None };
    let start = WitnessOp::tally();
    let cases: Vec<(Vec<u8>, usize, Option<usize>)> = vec![
        (garbled(apply(vec![set(1), add(2), set(3)]), set(3)), 2, Some(2)),
        (garbled(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&MutationId("m-1".into()), Some(set(9))), input(&MutationId("m-2".into()), None), input(&MutationId("m-3".into()), Some(add(4)))] }, add(4)), 1, Some(2)),
        (garbled(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: Some(Box::new(apply(vec![set(5), add(6)]))) }, add(6)), 1, Some(1)),
        ([binary(apply(vec![set(7), add(8)])), vec![0]].concat(), 2, None),
    ];
    let mut retired = 0;
    for (bytes, decoded, failing) in cases {
        let refused = decode(&bytes).expect_err("a broken command is refused");
        retired += decoded;
        match failing {
            Some(index) => assert!(matches!(refused, CommandDecodeError::Operation { index: at, .. } if at == index), "{refused:?}"),
            None => assert!(matches!(refused, CommandDecodeError::Layout(_)), "{refused:?}"),
        }
        assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "every decoded operation retired, none dropped");
    }
    let mut hostile = vec![COMMAND_BINARY_FORMAT, 0, 0];
    semio_framework_os_pack::write_varint_u64(&mut hostile, u64::MAX >> 1);
    assert!(matches!(decode(&hostile), Err(CommandDecodeError::Operation { index: 0, .. })));
    let one = print_command(&apply(vec![set(1)])).await.expect("prints");
    let op_line = one.lines().find(|line| line.starts_with("  ")).expect("an op line").to_string();
    for (command, decoded) in [(format!("{}\n{op_line}\n  not an operation\n", one.lines().next().unwrap()), 1), (format!("supersede\n  replace m-1\n  {op_line}\n  replace m-2\n    not an operation\n"), 1)] {
        assert!(parse_command::<DemoSnapshot, WitnessOp>(&command).await.is_err(), "{command}");
        retired += decoded;
        assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "{command}");
    }
    let accepted = decode(&binary(apply(vec![set(1), add(2)]))).expect("a whole command decodes");
    assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "an accepted command's operations stay live");
    retire_command::<DemoSnapshot, WitnessOp>(accepted);
    assert_eq!(WitnessOp::tally(), (start.0 + retired + 2, start.1));
}
//#endregion 🧪️CommandDecodeLaws
