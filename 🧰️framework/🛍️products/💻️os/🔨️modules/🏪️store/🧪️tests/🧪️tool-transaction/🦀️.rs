//! 🎞️ Store laws of transaction-scoped amend (design §15, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): a tool
//! transaction's ticks grow ONE open edit stamped with its ref, its commit announces exactly the edit one `Apply` would have
//! recorded, its abort leaves no trace, an open transaction refuses every other local command (retiring its operations) and
//! stays the applied tail under remote edits, no persisted form holds it, and the batched publication path streams the same
//! way. The language-agnostic corpus is checked by the TS twin beside this file (`🟦️.ts`, Ajv + fast-json-patch + xstate).
use super::supersede_replay_tests::WitnessOp;
use super::*;
use semio_framework_diagnostic::FaultFrom;
use std::collections::HashSet;

//#region 🧰️Harness
const TOOL: &str = "demo#import";

fn transaction(id: &str) -> protocol::TransactionRef {
    protocol::TransactionRef { id: id.to_string(), tool: TOOL.to_string() }
}

fn set(n: i32) -> DemoMutation {
    DemoMutation::SetN(SetN { n })
}

fn add(delta: i32) -> DemoMutation {
    DemoMutation::AddN(AddN { delta })
}

async fn store_named(id: &str, n: Option<i32>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n }, None)).await
}

async fn attached_store(n: Option<i32>) -> (ArtifactStore<DemoSnapshot, DemoMutation>, ChannelBackboneRemote) {
    let mut store = store_named("demo", n).await;
    let (channel, remote) = ChannelBackbone::pair("tool-transaction").await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote).expect("drain the attach announcement");
    (store, remote)
}

/// 📨️ The operation envelopes the store announced since the last drain (history transitions left out).
fn announced_operations(remote: &ChannelBackboneRemote) -> Vec<crate::os_spr::MutationEnvelope> {
    drain_channel_for_test(remote)
        .expect("drain")
        .into_iter()
        .flat_map(|message| match message {
            BackboneMessage::Mutations { envelopes } => crate::os_spr::decode_envelopes(&envelopes).expect("announced envelopes decode"),
            _ => Vec::new(),
        })
        .filter(|envelope| !crate::os_spr::is_history_transition(envelope))
        .collect()
}

/// 💾️ The value both persisted forms (pack + spr, dsl + ops) reload to — they must agree.
async fn persisted_n(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> Option<i32> {
    let files = print_document_pack(store.envelope()).await.expect("pack prints");
    let pack = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pack parses");
    let text_files = print_document_text(store.envelope()).await.expect("text prints");
    let text = parse_document_text::<DemoSnapshot, DemoMutation>(&text_files.dsl, &text_files.ops).await.expect("text parses");
    let (from_pack, from_text) = (pack.snapshot.n, text.snapshot.n);
    test_support::retire_parsed_document(pack);
    test_support::retire_parsed_document(text);
    assert_eq!(from_pack, from_text, "both persisted forms reload to one document");
    from_pack
}

fn open_edit<'a>(store: &'a ArtifactStore<DemoSnapshot, DemoMutation>) -> Option<&'a Edit<DemoMutation>> {
    let open = store.open_transaction()?;
    store.envelope().vcs.edits.iter().find(|edit| edit.id == open.edit_id)
}

fn tail_edit(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> &Edit<DemoMutation> {
    let tail = store.applied_edit_ids().last().expect("an applied tail");
    store.envelope().vcs.edits.iter().find(|edit| edit.id == *tail).expect("the tail edit")
}

fn remote_edit(name: &str, index: usize, operation: DemoMutation, physical_ms: u64) -> crate::os_spr::MutationEnvelope {
    mutation_envelope_at("peer", &format!("{name}-remote-{index}"), operation, HybridLogicalTimestamp { actor: 7, physical_ms, logical: 0 }, Vec::new())
}

/// 📬️ Publishes one batched gesture as the plugin SDK's migrated route does, `open` keeping its tool transaction open.
async fn publish_batch(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, operation: u64, mutations: Vec<DemoMutation>, reference: Option<&protocol::TransactionRef>, open: bool) {
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mut publication = store
        .begin_outbound_apply_batch(semio_framework_job::OperationId(operation), store.generation_now(), store.content_revision_now(), store.local_actor_id().0.clone(), mutations, Some(&factory), reference.cloned())
        .unwrap_or_else(|rejected| panic!("batch admission: {}", rejected.reason));
    publication.set_transaction_open(open);
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 512, ..one_item_test_policy() };
    for _ in 0..4_096 {
        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut publication, grant).expect("bounded batch step") {
            break;
        }
    }
    store.flush_published_apply_batch(&mut publication).await.expect("flush the published gesture");
    assert!(publication.acknowledge(), "a published gesture acknowledges once announced (or once streamed)");
    close_durable_publication(&mut publication);
}
//#endregion 🧰️Harness

//#region 🧪️Corpus
/// 🧫️ The language-agnostic corpus: every step's refusal, live value, ledger, open transaction, announcement and persisted
/// value; while a transaction is open its edit is the applied tail and neither the shared log nor the persisted log holds it,
/// and every operation a commit announces carries the transaction's ref.
#[semio_framework_async_macros::async_test]
async fn tool_transaction_corpus_matches_the_store() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️tool-transaction/🔣️.json")).expect("corpus parses");
    assert_eq!(corpus["tool"], TOOL);
    let operations = |value: &serde_json::Value| value.as_array().expect("operations").iter().map(|operation| DemoMutation::from_value(operation.clone().into()).expect("corpus operations decode")).collect::<Vec<_>>();
    let mut steps = 0;
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let initial: Option<i32> = serde_json::from_value(case["initial"]["n"].clone()).expect("initial n");
        let (mut store, remote) = attached_store(initial).await;
        for (index, step) in case["steps"].as_array().expect("steps").iter().enumerate() {
            let command = if let Some(applied) = step.get("apply") {
                ArtifactCommand::Apply { mutations: operations(applied), transaction: None }
            } else if let Some(append) = step.get("append") {
                ArtifactCommand::AppendTransaction { mutations: operations(&append["operations"]), transaction: transaction(append["transaction"].as_str().expect("transaction id")) }
            } else if let Some(id) = step.get("commit") {
                ArtifactCommand::CommitTransaction { transaction_id: id.as_str().expect("transaction id").into() }
            } else if let Some(id) = step.get("abort") {
                ArtifactCommand::AbortTransaction { transaction_id: id.as_str().expect("transaction id").into() }
            } else if step.get("undo").is_some() {
                ArtifactCommand::Undo
            } else {
                let arriving = &step["remote"];
                let physical_ms = if arriving["clock"] == "after" { 4_000_000_000_000 + index as u64 } else { 1 + index as u64 };
                ArtifactCommand::IngestRemote { envelope: remote_edit(name, index, DemoMutation::from_value(arriving["operation"].clone().into()).expect("remote operation decodes"), physical_ms) }
            };
            let committing = step.get("commit").and_then(serde_json::Value::as_str).map(transaction);
            let generation = store.generation();
            let refused = match store.dispatch(command, &mut identity).await {
                Ok(_) => None,
                Err(VcsError::TransactionOpen { .. }) => Some("transactionOpen"),
                Err(VcsError::UnknownTransaction(_)) => Some("unknownTransaction"),
                Err(VcsError::EmptyApply) => Some("emptyApply"),
                Err(error) => panic!("{name} step {index}: unexpected refusal {error}"),
            };
            if refused.is_some() {
                assert_eq!(store.generation(), generation, "{name} step {index}: a refusal changes nothing");
            }
            let announced = announced_operations(&remote);
            let observed = serde_json::json!({
                "refused": refused,
                "n": store.snapshot_ref().n,
                "edits": store.envelope().vcs.edits.len(),
                "open": store.open_transaction().map(|open| open.transaction.id.clone()),
                "openOperations": open_edit(&store).map_or(0, |edit| edit.forwards.len()),
                "announced": announced.len(),
                "persisted": persisted_n(&store).await,
            });
            assert_eq!(observed, step["expect"], "{name} step {index}");
            if let Some(open) = store.open_transaction() {
                assert_eq!(store.applied_edit_ids().last(), Some(&open.edit_id), "{name} step {index}: the open edit is the applied tail");
                let held: HashSet<MutationId> = crate::os_spr::mutation_ids_for_edit::<DemoSnapshot, DemoMutation>(open_edit(&store).expect("the open edit")).into_iter().collect();
                assert!(store.event_log().expect("log").iter().all(|event| !held.contains(&event.mutation_id)), "{name} step {index}: the shared log leaves the open edit out");
                let spr = print_document_spr(store.envelope()).await.expect("spr prints");
                let log = crate::os_spr::decode_history(&spr, &crate::os_spr::DecodeOptions::default()).await.expect("spr decodes");
                assert!(log.edits.iter().all(|edit| edit.id != open.edit_id), "{name} step {index}: the persisted log leaves the open edit out");
                assert!(open_edit(&store).expect("the open edit").mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&open.transaction)), "{name} step {index}: every appended operation carries the ref");
            }
            if let Some(reference) = committing.filter(|_| refused.is_none()) {
                assert!(announced.iter().all(|envelope| envelope.transaction.as_ref() == Some(&reference)), "{name} step {index}: a commit announces its ref on every operation");
            }
            test_support::assert_live_equals_replay(&store).await;
            steps += 1;
        }
    }
    assert!(steps >= 30, "the corpus walks every declared step");
}

/// 🔡️ The language-agnostic command vectors: the three transaction commands encode to exactly these bytes (an independent TS
/// encoder reproduces them beside this file) and decode back; with operations (the text grammar refuses an empty append, as
/// it refuses every empty operation block) they print and parse as text to the same command.
#[semio_framework_async_macros::async_test]
async fn tool_transaction_command_vectors_match_both_codecs() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️tool-transaction/🔣️.json")).expect("corpus parses");
    let vectors = corpus["codec"].as_array().expect("codec vectors");
    assert_eq!(vectors.len(), 3);
    for vector in vectors {
        let command: ArtifactCommand<DemoMutation> = match (&vector["command"]["append"], &vector["command"]["commit"], &vector["command"]["abort"]) {
            (serde_json::Value::String(id), _, _) => ArtifactCommand::AppendTransaction { mutations: Vec::new(), transaction: transaction(id) },
            (_, serde_json::Value::String(id), _) => ArtifactCommand::CommitTransaction { transaction_id: id.clone() },
            (_, _, serde_json::Value::String(id)) => ArtifactCommand::AbortTransaction { transaction_id: id.clone() },
            _ => panic!("a codec vector names one transaction command"),
        };
        let bytes = command.encode_command().expect("encodes");
        assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), vector["hex"].as_str().expect("hex"), "{vector}");
        assert_eq!(ArtifactCommand::<DemoMutation>::decode_command::<DemoSnapshot>(&bytes).expect("decodes"), command);
        if !matches!(&command, ArtifactCommand::AppendTransaction { mutations, .. } if mutations.is_empty()) {
            test_support::assert_command_text_binary_equivalence::<DemoSnapshot, DemoMutation>(&command).await;
        }
        assert!(command.history_transition_kinds().is_empty(), "a tool transaction authors no history transition");
    }
    test_support::assert_command_text_binary_equivalence::<DemoSnapshot, DemoMutation>(&ArtifactCommand::AppendTransaction { mutations: vec![set(1), add(2)], transaction: transaction("tx-a") }).await;
}
//#endregion 🧪️Corpus

//#region 🧪️TransactionLaws
/// 🟰️ Appending ticks and committing records exactly the edit ONE `Apply` of the same operations under the same ref records:
/// the same state, forwards, inverses, per-operation kinds, labels and outcomes, every operation stamped, no description,
/// finished, and ONE undo step taking the whole gesture back — for a single-operation gesture too, whose
/// operation is named by its edit as an applied one is.
#[semio_framework_async_macros::async_test]
async fn appends_and_a_commit_record_exactly_the_edit_one_apply_records() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let reference = transaction("tx-one");
    for ticks in [vec![vec![set(1)], vec![add(2), add(3)], vec![add(4)]], vec![vec![set(7)]]] {
        let mut streamed = store_named("streamed", Some(0)).await;
        for tick in ticks.clone() {
            streamed.dispatch(ArtifactCommand::AppendTransaction { mutations: tick, transaction: reference.clone() }, &mut identity).await.expect("a tick appends");
        }
        streamed.dispatch(ArtifactCommand::CommitTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the transaction commits");
        let mut applied = store_named("applied", Some(0)).await;
        applied.dispatch(ArtifactCommand::Apply { mutations: ticks.concat(), transaction: Some(reference.clone()) }, &mut identity).await.expect("one apply");
        assert_eq!(streamed.snapshot().expect("streamed"), applied.snapshot().expect("applied"));
        assert_eq!((streamed.envelope().vcs.edits.len(), applied.envelope().vcs.edits.len()), (1, 1), "one gesture is one edit");
        let (edit, one) = (tail_edit(&streamed), tail_edit(&applied));
        assert_eq!(edit.forwards, one.forwards);
        assert_eq!(edit.inverse, one.inverse);
        let facts = |edit: &Edit<DemoMutation>| edit.mutation_meta.iter().map(|meta| (meta.semantic_kind.clone(), meta.label.clone(), meta.transaction.clone())).collect::<Vec<_>>();
        assert_eq!(facts(edit), facts(one));
        assert!(edit.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&reference)), "every operation carries the ref");
        assert!(edit.finished_at.is_some());
        if edit.forwards.len() == 1 {
            assert_eq!(edit.mutation_meta[0].mutation_id, Some(MutationId(edit.id.clone())), "a one-operation edit names its operation");
        }
        let outcomes = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| {
            store.mutation_outcomes().expect("outcomes").into_iter().map(|outcome| (outcome.op_index, outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>())).collect::<Vec<_>>()
        };
        assert_eq!(outcomes(&streamed), outcomes(&applied));
        assert!(streamed.open_transaction().is_none());
        streamed.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("one undo");
        assert_eq!(streamed.snapshot_ref().n, Some(0), "one undo step takes the whole gesture back");
        test_support::assert_document_pack_round_trip(&applied).await;
    }
}

/// 🪦️ An aborted transaction leaves no trace: the projection, content revision, applied order, ledger, message ledger,
/// outcomes, shared log and both persisted forms equal those of before its first append, nothing was ever announced, and the
/// next edit takes the edit sequence the transaction had taken.
#[semio_framework_async_macros::async_test]
async fn an_aborted_transaction_leaves_no_trace_anywhere() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let (mut store, remote) = attached_store(Some(0)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![set(1)], transaction: None }, &mut identity).await.expect("a plain edit");
    let _ = announced_operations(&remote);
    let pack = print_document_pack(store.envelope()).await.expect("pack");
    let text = print_document_text(store.envelope()).await.expect("text");
    let before = (
        store.snapshot().expect("state"),
        store.content_revision_now(),
        store.applied_edit_ids().to_vec(),
        store.envelope().vcs.edits.len(),
        store.envelope().edit_messages.len(),
        store.mutation_outcomes().expect("outcomes").len(),
        store.event_log().expect("log"),
    );
    let sequence = tail_edit(&store).sequence_number;
    let reference = transaction("tx-gone");
    for tick in [vec![add(1)], vec![add(2), set(9)], vec![add(3)]] {
        store.dispatch(ArtifactCommand::AppendTransaction { mutations: tick, transaction: reference.clone() }, &mut identity).await.expect("a tick appends");
    }
    assert_eq!(store.snapshot_ref().n, Some(12));
    assert!(store.envelope().edit_messages.iter().any(|entry| Some(&entry.edit_id) == store.open_transaction().map(|open| &open.edit_id)), "the open edit records its own outcomes");
    store.dispatch(ArtifactCommand::AbortTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the transaction aborts");
    let after = (
        store.snapshot().expect("state"),
        store.content_revision_now(),
        store.applied_edit_ids().to_vec(),
        store.envelope().vcs.edits.len(),
        store.envelope().edit_messages.len(),
        store.mutation_outcomes().expect("outcomes").len(),
        store.event_log().expect("log"),
    );
    assert_eq!(after, before, "an abort leaves no trace");
    assert_eq!(print_document_pack(store.envelope()).await.expect("pack"), pack);
    assert_eq!(print_document_text(store.envelope()).await.expect("text"), text);
    assert!(announced_operations(&remote).is_empty(), "nothing of an aborted transaction was ever announced");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![add(5)], transaction: None }, &mut identity).await.expect("the next edit");
    assert_eq!(tail_edit(&store).sequence_number, sequence + 1, "the next edit takes the sequence the transaction had taken");
    test_support::assert_live_equals_replay(&store).await;
}

/// 🚫️ While a transaction is open every other local command is refused `TransactionOpen` (fault code
/// `toolTransaction.open`) with the store unchanged; its own appends, a merge-policy change and its commit still run. The
/// refusal retires every operation a refused command carried — operation lists, supersede replacements, nested commands —
/// through the technology's cold disposal (`retire_command`, which the dispatch gate calls), never dropping one bare.
#[semio_framework_async_macros::async_test]
async fn an_open_transaction_refuses_every_other_command_and_retires_its_operations() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let mut store = store_named("busy", Some(0)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![set(1)], transaction: None }, &mut identity).await.expect("a plain edit");
    let target = store.mutation_ops().expect("operations")[0].mutation_id.clone();
    let reference = transaction("tx-busy");
    store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("the transaction opens");
    fn refused<Op>(target: &MutationId, live: impl Fn(DemoMutation) -> Op) -> Vec<(ArtifactCommand<Op>, usize)> {
        let apply = |mutations: Vec<Op>| ArtifactCommand::Apply { mutations, transaction: None };
        vec![
            (apply(vec![live(set(2)), live(add(3))]), 2),
            (ArtifactCommand::ApplyInLane { mutations: vec![live(set(2))], lane: HistoryLane::Interaction, transaction: None }, 1),
            (ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: target.clone(), replacement: Some(live(set(4))) }] }, 1),
            (ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![SupersedeInput { target: target.clone(), replacement: Some(live(set(5))) }] }, 1),
            (ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: Some(Box::new(apply(vec![live(set(6))]))) }, 1),
            (ArtifactCommand::AppendTransaction { mutations: vec![live(add(1))], transaction: transaction("tx-other") }, 1),
            (ArtifactCommand::Undo, 0),
            (ArtifactCommand::Redo, 0),
            (ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, 0),
            (ArtifactCommand::CreateAlternative { name: "branch".into() }, 0),
        ]
    }
    let (generation, revision) = (store.generation(), store.content_revision_now());
    for (command, _) in refused(&target, |operation| operation) {
        let label = format!("{command:?}");
        let refusal = store.dispatch(command, &mut identity).await.expect_err("refused while the transaction is open");
        assert!(matches!(&refusal, VcsError::TransactionOpen { transaction_id } if *transaction_id == reference.id), "{label}: {refusal}");
        assert_eq!(refusal.into_fault().code.0, "toolTransaction.open", "{label}");
    }
    assert_eq!((store.generation(), store.content_revision_now()), (generation, revision), "refusals change nothing");
    let start = WitnessOp::tally();
    let mut retired = 0;
    for (command, carried) in refused(&target, WitnessOp::live) {
        retire_command::<DemoSnapshot, WitnessOp>(command);
        retired += carried;
        assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "every carried operation retired, none dropped");
    }
    store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(2)], transaction: reference.clone() }, &mut identity).await.expect("its own tick still appends");
    store.dispatch(ArtifactCommand::SetMergePolicy { policy: crate::os_spr::MergePolicy::default() }, &mut identity).await.expect("a merge-policy change still runs");
    assert!(matches!(store.dispatch(ArtifactCommand::CommitTransaction { transaction_id: "tx-other".into() }, &mut identity).await, Err(VcsError::UnknownTransaction(id)) if id == "tx-other"));
    assert_eq!(VcsError::UnknownTransaction("tx".into()).into_fault().code.0, "toolTransaction.unknown");
    store.dispatch(ArtifactCommand::CommitTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("its commit runs");
    assert_eq!(store.snapshot_ref().n, Some(4));
    store.dispatch(ArtifactCommand::Apply { mutations: vec![add(1)], transaction: None }, &mut identity).await.expect("after the commit every command runs again");
}

/// 🤝️ A remote edit arriving while a transaction is open — later or earlier than it by clock — lands under it: the open edit
/// stays the applied tail, keeps its ids and ref, and its commit converges every replica, including the one whose edit landed
/// under it, onto one state and one applied order.
#[semio_framework_async_macros::async_test]
async fn a_committed_transaction_converges_with_the_replicas_that_edited_under_it() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let mut author = ArtifactStore::new_with_actor(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None), ActorId("author".into())).await;
    let mut peer = ArtifactStore::new_with_actor(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None), ActorId("peer".into())).await;
    author.dispatch(ArtifactCommand::Apply { mutations: vec![set(1)], transaction: None }, &mut identity).await.expect("a shared edit");
    for event in author.event_log().expect("log") {
        peer.ingest_remote(event).await.expect("the peer takes the shared edit");
    }
    let reference = transaction("tx-shared");
    author.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("the transaction opens");
    let open_id = author.open_transaction().expect("open").edit_id.clone();
    let open_operations: Vec<Option<MutationId>> = open_edit(&author).expect("the open edit").mutation_meta.iter().map(|meta| meta.mutation_id.clone()).collect();
    peer.dispatch(ArtifactCommand::Apply { mutations: vec![set(10)], transaction: None }, &mut identity).await.expect("the peer edits meanwhile");
    let known: HashSet<MutationId> = author.event_log().expect("log").into_iter().map(|event| event.mutation_id).collect();
    for event in peer.event_log().expect("log").into_iter().filter(|event| !known.contains(&event.mutation_id)) {
        author.dispatch(ArtifactCommand::IngestRemote { envelope: event }, &mut identity).await.expect("the author ingests under its open transaction");
    }
    assert_eq!(author.snapshot_ref().n, Some(11), "the remote edit lands under the open transaction");
    assert_eq!(author.applied_edit_ids().last(), Some(&open_id), "the open edit stays the applied tail");
    assert_eq!(open_edit(&author).expect("still open").mutation_meta.iter().map(|meta| meta.mutation_id.clone()).collect::<Vec<_>>(), open_operations, "the open edit keeps its ids");
    author.ingest_remote(remote_edit("earlier", 0, set(20), 1)).await.expect("an earlier-clocked remote edit");
    assert_eq!(author.applied_edit_ids().last(), Some(&open_id), "an earlier remote edit lands under it too");
    author.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(2)], transaction: reference.clone() }, &mut identity).await.expect("a later tick");
    author.dispatch(ArtifactCommand::CommitTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the transaction commits");
    peer.ingest_remote(remote_edit("earlier", 0, set(20), 1)).await.expect("the peer takes the earlier edit");
    let known: HashSet<MutationId> = peer.event_log().expect("log").into_iter().map(|event| event.mutation_id).collect();
    for event in author.event_log().expect("log").into_iter().filter(|event| !known.contains(&event.mutation_id)) {
        assert_eq!(event.transaction.as_ref(), Some(&reference), "only the committed transaction is new to the peer");
        peer.ingest_remote(event).await.expect("the peer takes the committed transaction");
    }
    assert_eq!(author.snapshot_ref().n, Some(13));
    assert_eq!(peer.snapshot().expect("peer"), author.snapshot().expect("author"), "both replicas converge");
    let order = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| store.mutation_ops().expect("operations").into_iter().map(|operation| operation.mutation_id).collect::<Vec<_>>();
    assert_eq!(order(&peer), order(&author), "one applied order");
    test_support::assert_live_equals_replay(&author).await;
    test_support::assert_live_equals_replay(&peer).await;
}

/// 📬️ The batched publication route streams the same way: streamed gestures carrying the ref grow one open, unannounced edit;
/// a gesture without it is refused admission; the closing gesture appends and commits, announcing every operation of the
/// edit once with the ref; a batch-opened transaction aborts with zero trace.
#[semio_framework_async_macros::async_test]
async fn batched_gestures_stream_into_one_open_edit_and_a_closing_gesture_commits_it() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners());
    store.closes_on_drop();
    let (channel, remote) = ChannelBackbone::pair("batched-transaction").await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote).expect("drain the attach announcement");
    let reference = transaction("tx-batch");
    publish_batch(&mut store, 1, vec![set(1)], Some(&reference), true).await;
    publish_batch(&mut store, 2, vec![add(1), add(2)], Some(&reference), true).await;
    assert_eq!((store.snapshot_ref().n, store.envelope().vcs.edits.len(), open_edit(&store).map(|edit| edit.forwards.len())), (Some(4), 1, Some(3)), "streamed gestures grow one open edit");
    assert!(announced_operations(&remote).is_empty(), "a streamed gesture announces nothing");
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let refused = store.begin_outbound_apply_batch(semio_framework_job::OperationId(3), store.generation_now(), store.content_revision_now(), "retained-test".into(), vec![set(9)], Some(&factory), None);
    assert!(refused.is_err_and(|rejected| rejected.reason.contains("open tool transaction tx-batch")), "a gesture outside the transaction is refused admission");
    publish_batch(&mut store, 4, vec![add(3)], Some(&reference), false).await;
    let announced = announced_operations(&remote);
    assert_eq!((store.snapshot_ref().n, store.envelope().vcs.edits.len(), store.open_transaction().is_none()), (Some(7), 1, true), "the closing gesture commits the one edit");
    assert_eq!(announced.len(), 4, "the commit announces every operation of the edit once");
    assert!(announced.iter().all(|envelope| envelope.transaction.as_ref() == Some(&reference)));
    assert!(tail_edit(&store).finished_at.is_some());
    let revision = store.content_revision_now();
    let gone = transaction("tx-batch-gone");
    publish_batch(&mut store, 5, vec![add(10)], Some(&gone), true).await;
    assert_eq!(store.snapshot_ref().n, Some(17));
    store.dispatch(ArtifactCommand::AbortTransaction { transaction_id: gone.id.clone() }, &mut identity).await.expect("the batch-opened transaction aborts");
    assert_eq!((store.snapshot_ref().n, store.envelope().vcs.edits.len(), store.content_revision_now()), (Some(7), 1, revision), "zero trace");
    assert!(announced_operations(&remote).is_empty());
}

/// 📄 One history page is 64 edits. The next edit opens another page, so a session past that bound keeps every edit.
#[semio_framework_async_macros::async_test]
async fn a_history_past_one_page_keeps_admitting_edits() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let mut store = store_named("paged", Some(0)).await;
    for index in 0..80 {
        store.dispatch(ArtifactCommand::Apply { mutations: vec![set(index)], transaction: None }, &mut identity).await.unwrap_or_else(|error| panic!("edit {index}: {error}"));
    }
    assert_eq!(store.envelope().vcs.edits.len(), 80);
    assert_eq!(store.applied_edit_ids().len(), 80);
    assert_eq!(store.snapshot_ref().n, Some(79));
}

/// ↩️ An abort gives back what the open edit took from the history, on the command route and on the batched route (audit
/// W1G-5): the redo entry this actor's open edit dropped, the persisted cursor, the applied order, the projection and the
/// revision equal those of before the first append, and redo then undo still step exactly as before.
#[semio_framework_async_macros::async_test]
async fn an_abort_restores_the_redo_stack_and_the_cursor_on_both_routes() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    for batched in [false, true] {
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "redo", DemoSnapshot { n: Some(0) }, None)).await;
        store.install_document_store_owners_exact(demo_closable_store_owners());
        store.closes_on_drop();
        for n in [1, 2] {
            store.dispatch(ArtifactCommand::Apply { mutations: vec![set(n)], transaction: None }, &mut identity).await.expect("a plain edit");
        }
        store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo the second edit");
        let observe = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| {
            let cursor = store.envelope().cursor.as_ref().expect("a live cursor");
            (store.snapshot_ref().n, store.content_revision_now(), store.applied_edit_ids().to_vec(), store.redo_edit_ids().to_vec(), cursor.applied_edit_ids.to_vec(), cursor.redo_edit_ids.to_vec(), cursor.checkpoint_id.clone())
        };
        let before = observe(&store);
        assert_eq!((before.0, before.3.len()), (Some(1), 1), "batched={batched}: one redo entry waits");
        let reference = transaction("tx-redo");
        if batched {
            publish_batch(&mut store, 1, vec![add(5)], Some(&reference), true).await;
        } else {
            store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(5)], transaction: reference.clone() }, &mut identity).await.expect("the transaction opens");
        }
        assert!(store.redo_edit_ids().is_empty(), "batched={batched}: the actor's open edit drops its redo entries, as every new edit of that actor does");
        store.dispatch(ArtifactCommand::AbortTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the transaction aborts");
        assert_eq!(observe(&store), before, "batched={batched}: an abort restores the redo stack, the cursor, the order, the projection and the revision");
        store.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("the restored redo entry redoes");
        assert_eq!(store.snapshot_ref().n, Some(2), "batched={batched}");
        store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo");
        store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo again");
        assert_eq!(store.snapshot_ref().n, Some(0), "batched={batched}");
        test_support::assert_live_equals_replay(&store).await;
    }
}

/// 🛬️ Remote edits that landed under an open transaction — one later and one earlier by clock — stay when it aborts (audit
/// W1G-5): the store equals a replica that only ever saw the shared log, nothing of the transaction was announced, both
/// persisted forms reload to it and the live projection equals a fresh replay.
#[semio_framework_async_macros::async_test]
async fn remote_edits_that_landed_under_an_open_transaction_survive_its_abort() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let (mut store, remote) = attached_store(Some(0)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![set(1)], transaction: None }, &mut identity).await.expect("a plain edit");
    let _ = announced_operations(&remote);
    let reference = transaction("tx-under");
    store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("the transaction opens");
    store.ingest_remote(remote_edit("under", 0, add(10), 4_000_000_000_000)).await.expect("a later remote edit lands under it");
    store.ingest_remote(remote_edit("under", 1, add(100), 1)).await.expect("an earlier remote edit lands under it");
    let open_id = store.open_transaction().expect("still open").edit_id.clone();
    assert_eq!(store.applied_edit_ids().last(), Some(&open_id), "the open edit stays the applied tail");
    store.dispatch(ArtifactCommand::AbortTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the transaction aborts");
    assert!(store.open_transaction().is_none() && store.envelope().vcs.edits.iter().all(|edit| edit.id != open_id), "the open edit is gone");
    assert_eq!(store.envelope().vcs.edits.len(), 3, "the plain edit and both remote edits stay");
    let mut replica = store_named("demo", Some(0)).await;
    for event in store.event_log().expect("log") {
        replica.ingest_remote(event).await.expect("the replica takes the shared log");
    }
    assert_eq!(store.snapshot().expect("store"), replica.snapshot().expect("replica"), "the aborted store equals a replica of the shared log");
    assert_eq!(store.applied_edit_ids().len(), 3);
    assert!(announced_operations(&remote).is_empty(), "nothing of the aborted transaction was announced");
    assert_eq!(persisted_n(&store).await, store.snapshot_ref().n);
    test_support::assert_live_equals_replay(&store).await;
}

/// 🐎️ A tool transaction streaming one tick per turn never starves a history step that waits for later turns (audit W1G-5):
/// a tick appended to the open edit — the applied tail — keeps the waiting replay running instead of restarting it, so the
/// step adopts within its budgeted turns while the ticks keep landing, and the committed result equals a fresh replay.
#[semio_framework_async_macros::async_test]
async fn a_transaction_streaming_while_a_local_step_waits_never_starves_it() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let mut store = store_named("starve", Some(0)).await;
    for _ in 0..40 {
        store.dispatch(ArtifactCommand::Apply { mutations: vec![add(1)], transaction: None }, &mut identity).await.expect("a plain edit");
    }
    store.defer_local_replays(Some(ReplayTurnBudget::operations(4)));
    let target = store.mutation_ops().expect("operations")[0].mutation_id.clone();
    let reference = transaction("tx-drag");
    store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("the drag opens");
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target, replacement: Some(add(5)) }] }, &mut identity).await.expect_err("an open transaction refuses the finalize");
    store.dispatch(ArtifactCommand::CommitTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the drag commits");
    let target = store.mutation_ops().expect("operations")[0].mutation_id.clone();
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target, replacement: Some(add(5)) }] }, &mut identity).await.expect("the finalize waits for later turns");
    assert!(store.local_step_pending(), "a 41-edit replay outlasts one 4-operation turn");
    let reference = transaction("tx-drag-2");
    store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("a drag opens while the step waits");
    let mut turns = 0;
    while store.local_step_pending() {
        store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("one tick per turn");
        store.step_reprojection(None).await.expect("one budgeted turn");
        turns += 1;
        assert!(turns <= 32, "the waiting step adopts while the ticks keep streaming");
    }
    assert_eq!(store.supersessions().len(), 1, "the finalize was adopted");
    store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![add(1)], transaction: reference.clone() }, &mut identity).await.expect("the drag keeps streaming after the adoption");
    store.dispatch(ArtifactCommand::CommitTransaction { transaction_id: reference.id.clone() }, &mut identity).await.expect("the drag commits");
    assert_eq!(store.snapshot_ref().n, Some(5 + 40 + turns + 2), "the superseded first edit, the other 40 and every tick");
    test_support::assert_live_equals_replay(&store).await;
}
//#endregion 🧪️TransactionLaws
