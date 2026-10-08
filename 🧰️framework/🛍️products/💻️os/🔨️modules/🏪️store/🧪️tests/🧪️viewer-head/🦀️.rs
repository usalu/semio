//! 👁️ Store laws of the per-replica viewed alternative (design §22.3, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the
//! event log — edits, commits, alternative registrations, supersessions — is shared, the head a replica shows is its own.
//! Every case of the language-agnostic corpus (`🧫️fixtures/🧫️viewer-head`, whose independent fast-json-patch model is `🟦️.ts`
//! beside this file) runs over two real stores; a history edit finalized as a new alternative moves only its author; `.spr`
//! restores each replica's own head while the `.ops` text hydrates to the trunk tip; and a replica holding the whole log shows
//! every alternative alike, whatever order the events arrived in.
use super::*;

//#region 🧰️Harness
type DemoStore = ArtifactStore<DemoSnapshot, DemoMutation>;

/// 🪞️ One replica of a corpus case: its store and the operations of every edit it authored, in authoring order.
struct Replica {
    store: DemoStore,
    authored: Vec<Vec<MutationId>>,
}

async fn fresh(document: &str, initial: Option<i32>) -> DemoStore {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", document, DemoSnapshot { n: initial }, None)).await
}

async fn apply(store: &mut DemoStore, mutations: Vec<DemoMutation>) {
    store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }).await.expect("a clean edit applies");
}

fn integer(value: &serde_json::Value) -> i32 {
    i32::try_from(value.as_i64().expect("an integer")).expect("an i32")
}

fn operation(value: &serde_json::Value) -> DemoMutation {
    match value["operation"].as_str().expect("an operation kind") {
        "setN" => DemoMutation::SetN(SetN { n: integer(&value["n"]) }),
        "addN" => DemoMutation::AddN(AddN { delta: integer(&value["delta"]) }),
        kind => panic!("unknown corpus operation {kind}"),
    }
}

fn replacement(value: &serde_json::Value) -> Option<DemoMutation> {
    (value.as_str() != Some("withdrawn")).then(|| operation(value))
}

fn replica_index(name: &serde_json::Value) -> usize {
    match name.as_str().expect("a replica name") {
        "a" => 0,
        "b" => 1,
        other => panic!("unknown replica {other}"),
    }
}

/// 🎯️ The identity of the corpus target `{ by, edit, op }`: operation `op` of the `edit`-th edit replica `by` authored.
fn target(replicas: &[Replica; 2], value: &serde_json::Value) -> MutationId {
    let index = |key: &str| usize::try_from(value[key].as_u64().expect("an index")).expect("a usize");
    replicas[replica_index(&value["by"])].authored[index("edit")][index("op")].clone()
}

/// 🚉️ The id of the alternative the corpus calls `name`: `trunk`, or a registered alternative's name.
fn line_id(store: &DemoStore, name: &str) -> String {
    if name == "trunk" {
        return store.trunk_alternative_id();
    }
    store.envelope().vcs.alternatives.iter().find(|alternative| alternative.name == name).map(|alternative| alternative.id.clone()).unwrap_or_else(|| panic!("no alternative named {name}"))
}

/// 🗂️ The names of the alternatives a replica lists, the trunk's empty name first once it has a checkpoint.
fn listed(store: &DemoStore) -> Vec<String> {
    store.envelope().vcs.alternatives.iter().map(|alternative| alternative.name.clone()).collect()
}

/// 📦️ What a replica's head projects, in the corpus's words.
fn projection(store: &DemoStore) -> serde_json::Value {
    serde_json::json!({ "state": { "n": store.snapshot_ref().n }, "applied": store.applied_edit_ids().len(), "supersessions": store.supersessions().len() })
}

/// 🪟️ What a replica shows, in the corpus's words: its head, the projection there and the alternatives it lists.
fn view(store: &DemoStore) -> serde_json::Value {
    let line = store.active_line_id();
    let line = if line == store.trunk_alternative_id() { "trunk".to_string() } else { store.envelope().vcs.alternatives.iter().find(|alternative| alternative.id == line).map(|alternative| alternative.name.clone()).expect("the shown alternative is registered") };
    let checkpoint = store.envelope().viewer_checkpoint_id.as_ref().map(|id| store.envelope().vcs.checkpoints.iter().position(|checkpoint| checkpoint.id == *id).expect("the shown checkpoint is listed"));
    let mut view = projection(store);
    view["line"] = serde_json::json!(line);
    view["checkpoint"] = serde_json::json!(checkpoint);
    view["alternatives"] = serde_json::json!(listed(store));
    view
}

/// 📸️ Everything another replica's history step must leave alone: a replica's head, projection, applied edits and effective
/// supersessions.
fn shown(store: &DemoStore) -> (Option<String>, Option<String>, Option<i32>, Vec<String>, EffectiveSupersessions) {
    (store.envelope().active_alternative_id.clone(), store.envelope().viewer_checkpoint_id.clone(), store.snapshot_ref().n, store.applied_edit_ids().iter().cloned().collect(), store.supersessions().clone())
}

/// 📥️ `to` takes every event of `log` it lacks, in the order given; afterwards every delivered event found its dependencies.
async fn deliver(to: &mut DemoStore, log: Vec<crate::os_spr::MutationEnvelope>) {
    let known: std::collections::HashSet<MutationId> = to.event_log().expect("log").into_iter().map(|event| event.mutation_id).collect();
    for event in log.into_iter().filter(|event| !known.contains(&event.mutation_id)) {
        to.ingest_remote(event).await.expect("a replica ingests a shared event");
    }
    assert!(to.dag.pending_is_empty(), "every delivered event found its dependencies");
}

/// 💾️ The store a replica restores from its own persisted pair (`spr`, head included) or from the shared text log (`ops`).
async fn reloaded(store: &DemoStore, through: &str) -> DemoStore {
    let envelope = match through {
        "spr" => {
            let files = print_document_pack(store.envelope()).await.expect("the pair prints");
            parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("the pair parses").into_envelope()
        }
        "ops" => {
            let text = print_document_text(store.envelope()).await.expect("the text prints");
            parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops).await.expect("the text parses").into_envelope()
        }
        other => panic!("unknown persisted form {other}"),
    };
    ArtifactStore::new(envelope).await
}

/// 🎬️ One corpus action at replica `at`.
async fn act(replicas: &mut [Replica; 2], at: usize, action: &serde_json::Value) {
    match action["kind"].as_str().expect("an action kind") {
        "edit" => {
            let store = &mut replicas[at].store;
            apply(store, action["edit"].as_array().expect("operations").iter().map(operation).collect()).await;
            let edit_id = store.applied_edit_ids().last().cloned().expect("the edit is applied");
            let operations = store.mutation_ops().expect("applied operations").into_iter().filter(|operation| operation.edit_id == edit_id.as_str()).map(|operation| operation.mutation_id).collect();
            replicas[at].authored.push(operations);
        }
        "commit" => {
            replicas[at].store.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a checkpoint");
        }
        "alternative" => {
            let inputs = vec![SupersedeInput { target: target(replicas, &action["target"]), replacement: replacement(&action["replacement"]) }];
            let name = action["name"].as_str().expect("an alternative name").to_string();
            replicas[at].store.dispatch(ArtifactCommand::CreateAlternativeWithSupersede { name, inputs }).await.expect("a finalize as a new alternative");
        }
        "supersede" => {
            let inputs = vec![SupersedeInput { target: target(replicas, &action["target"]), replacement: replacement(&action["replacement"]) }];
            let scope = (action["scope"].as_str() == Some("line")).then(|| replicas[at].store.active_line_id());
            replicas[at].store.dispatch(ArtifactCommand::Supersede { scope, inputs }).await.expect("a finalize");
        }
        "switch" => {
            let alternative_id = line_id(&replicas[at].store, action["to"].as_str().expect("an alternative"));
            replicas[at].store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id }).await.expect("a local switch");
        }
        "checkout" => {
            let ordinal = usize::try_from(action["checkpoint"].as_u64().expect("a checkpoint index")).expect("a usize");
            let checkpoint_id = replicas[at].store.envelope().vcs.checkpoints.iter().nth(ordinal).map(|checkpoint| checkpoint.id.clone()).expect("a listed checkpoint");
            replicas[at].store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).await.expect("a local checkout");
        }
        "receive" => {
            let mut log = replicas[replica_index(&action["from"])].store.event_log().expect("log");
            if action["order"].as_str() == Some("reversed") {
                log.reverse();
            }
            deliver(&mut replicas[at].store, log).await;
        }
        "reload" => {
            let restored = reloaded(&replicas[at].store, action["through"].as_str().expect("a persisted form")).await;
            replicas[at].store = restored;
        }
        kind => panic!("unknown corpus action {kind}"),
    }
}
//#endregion 🧰️Harness

//#region 🧪️Corpus
/// 🧫️ The language-agnostic corpus over two real stores: after every step both replicas show exactly what the corpus states
/// (head, projection, listed alternatives) and each live projection equals a fresh replay; a replica holding the whole log
/// lists the same alternatives and projects every alternative's tip as `converged` says for every arrival order.
#[semio_framework_async_macros::async_test]
async fn the_viewer_head_corpus_matches_two_stores() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️viewer-head/🔣️.json")).expect("the corpus parses");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("a case name");
        let initial = case["initial"]["n"].as_i64().map(|n| i32::try_from(n).expect("an i32"));
        let mut replicas = [Replica { store: fresh(name, initial).await, authored: Vec::new() }, Replica { store: fresh(name, initial).await, authored: Vec::new() }];
        for (index, step) in case["steps"].as_array().expect("steps").iter().enumerate() {
            act(&mut replicas, replica_index(&step["at"]), &step["do"]).await;
            for (replica, key) in replicas.iter().zip(["a", "b"]) {
                assert_eq!(view(&replica.store), step["expect"][key], "{name} step {index}: replica {key}");
                test_support::assert_live_equals_replay(&replica.store).await;
            }
        }
        let [a, b] = &mut replicas;
        deliver(&mut a.store, b.store.event_log().expect("log")).await;
        let log = a.store.event_log().expect("the whole log");
        let mut arrivals = vec![log.clone(), log.iter().rev().cloned().collect::<Vec<_>>()];
        arrivals.extend((1..log.len()).map(|rotation| {
            let mut rotated = log.clone();
            rotated.rotate_left(rotation);
            rotated
        }));
        for (arrival, events) in arrivals.into_iter().enumerate() {
            let mut replica = fresh(name, initial).await;
            deliver(&mut replica, events).await;
            assert_eq!(serde_json::json!(listed(&replica)), case["converged"]["alternatives"], "{name} arrival {arrival}: listed alternatives");
            for (line, expected) in case["converged"]["lines"].as_object().expect("lines") {
                let alternative_id = line_id(&replica, line);
                replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id }).await.expect("a local switch");
                assert_eq!(&projection(&replica), expected, "{name} arrival {arrival}: {line}");
            }
        }
    }
}
//#endregion 🧪️Corpus

//#region 🧪️RevisionLaws
/// 🧲️ LAW (design §22.28): a content revision names what a head holds, never where its edits stand in one replica's ledger.
/// For every corpus case a replica takes the whole log in every arrival order while it stands on each line of the case — it
/// steps onto a line the moment that line is listed, so the other lines' edits reach it while it shows something else.
/// Whatever line it stood on and whatever order the events came in, it names ONE content revision per line, and its own
/// persisted pair restores exactly the revision it showed.
#[semio_framework_async_macros::async_test]
async fn a_content_revision_names_a_head_whatever_line_its_replica_stood_on() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️viewer-head/🔣️.json")).expect("the corpus parses");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("a case name");
        let initial = case["initial"]["n"].as_i64().map(|n| i32::try_from(n).expect("an i32"));
        let mut replicas = [Replica { store: fresh(name, initial).await, authored: Vec::new() }, Replica { store: fresh(name, initial).await, authored: Vec::new() }];
        for step in case["steps"].as_array().expect("steps") {
            act(&mut replicas, replica_index(&step["at"]), &step["do"]).await;
        }
        let [a, b] = &mut replicas;
        deliver(&mut a.store, b.store.event_log().expect("log")).await;
        let log = a.store.event_log().expect("the whole log");
        let lines: Vec<&str> = case["converged"]["lines"].as_object().expect("lines").keys().map(String::as_str).collect();
        let mut arrivals = vec![log.clone(), log.iter().rev().cloned().collect::<Vec<_>>()];
        arrivals.extend((1..log.len()).map(|rotation| {
            let mut rotated = log.clone();
            rotated.rotate_left(rotation);
            rotated
        }));
        let mut named: BTreeMap<&str, BTreeMap<[u8; 32], String>> = BTreeMap::new();
        for (arrival, events) in arrivals.iter().enumerate() {
            for stand in &lines {
                let mut replica = fresh(name, initial).await;
                let mut standing = *stand == "trunk";
                for event in events.iter().cloned() {
                    replica.ingest_remote(event).await.expect("a replica ingests a shared event");
                    if !standing && replica.envelope().vcs.alternatives.iter().any(|alternative| alternative.name == *stand) {
                        let alternative_id = line_id(&replica, stand);
                        replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id }).await.expect("a replica steps onto a listed line");
                        standing = true;
                    }
                }
                assert!(standing && replica.dag.pending_is_empty(), "{name} arrival {arrival}: the replica stood on {stand} and every event found its dependencies");
                for line in &lines {
                    let alternative_id = line_id(&replica, line);
                    replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id }).await.expect("a local switch");
                    let revision = replica.content_revision_now();
                    assert_eq!(reloaded(&replica, "spr").await.content_revision_now(), revision, "{name} arrival {arrival}, stood on {stand}: the persisted pair restores the revision {line} showed");
                    let positions = replica.envelope().vcs.edits.iter().map(|edit| edit.sequence_number.to_string()).collect::<Vec<_>>().join(",");
                    named.entry(*line).or_default().entry(revision).or_insert_with(|| format!("arrival {arrival} stood on {stand}, ledger positions {positions}"));
                }
            }
        }
        for (line, revisions) in &named {
            assert_eq!(revisions.len(), 1, "{name}: every replica names one content revision for {line}: {:?}", revisions.values().collect::<Vec<_>>());
        }
    }
}
//#endregion 🧪️RevisionLaws

//#region 🧪️FinalizeLaws
/// 🎏️ The history ledgers list their facts in log order, never in arrival order: two replicas each commit a checkpoint and
/// register an alternative, the second after it took the first's events. A replica that takes the whole log reversed or in
/// any rotation lists the changes, checkpoints and alternatives exactly as its own persisted pair restores them — the
/// trunk, then `red`, then `blue` — and so do both authors.
#[semio_framework_async_macros::async_test]
async fn the_ledgers_list_their_facts_in_log_order_whatever_order_the_events_arrived_in() {
    let mut a = fresh("viewer-ledger-order", Some(0)).await;
    apply(&mut a, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    a.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a's checkpoint");
    a.dispatch(ArtifactCommand::CreateAlternative { name: "red".into() }).await.expect("a's alternative");
    let mut b = fresh("viewer-ledger-order", Some(0)).await;
    deliver(&mut b, a.event_log().expect("log")).await;
    apply(&mut b, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    b.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("b's checkpoint");
    b.dispatch(ArtifactCommand::CreateAlternative { name: "blue".into() }).await.expect("b's alternative");
    deliver(&mut a, b.event_log().expect("log")).await;
    let facts = |store: &DemoStore| {
        let vcs = &store.envelope().vcs;
        (vcs.changes.iter().map(|change| change.id.clone()).collect::<Vec<_>>(), vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.clone()).collect::<Vec<_>>(), vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone())).collect::<Vec<_>>())
    };
    let expected = facts(&reloaded(&a, "spr").await);
    assert_eq!(expected.2.iter().map(|(_, name)| name.as_str()).collect::<Vec<_>>(), ["", "red", "blue"], "the log lists the trunk, then the alternatives as they were registered");
    assert_eq!((expected.0.len(), expected.1.len()), (2, 2));
    assert_eq!((facts(&a), facts(&b)), (expected.clone(), facts(&reloaded(&b, "spr").await)), "each author lists what its own pair restores");
    let log = a.event_log().expect("the whole log");
    let mut arrivals = vec![log.iter().rev().cloned().collect::<Vec<_>>()];
    arrivals.extend((1..log.len()).map(|rotation| {
        let mut rotated = log.clone();
        rotated.rotate_left(rotation);
        rotated
    }));
    for (arrival, events) in arrivals.into_iter().enumerate() {
        let mut replica = fresh("viewer-ledger-order", Some(0)).await;
        deliver(&mut replica, events).await;
        assert_eq!(facts(&replica), expected, "arrival {arrival}: the ledgers follow the log");
        assert_eq!(facts(&reloaded(&replica, "spr").await), expected, "arrival {arrival}: and so does the replica's own pair");
    }
}

/// 🌿️ A history edit finalized as a new alternative through the session path (`commit_finished_replay`) moves only its
/// author: a peer holding its own uncommitted edit receives the commit, the registration and the scoped supersession in
/// reversed order and keeps its head, projection, applied edits and effective supersessions; it lists the alternative and
/// checks it out locally, which authors nothing and leaves the author where it is; `.spr` restores each replica's own head,
/// the `.ops` text hydrates to the trunk tip; and every arrival order of the whole log converges on the same registrations,
/// projections and content revisions.
#[semio_framework_async_macros::async_test]
async fn a_finalize_as_a_new_alternative_moves_only_its_author() {
    let mut author = fresh("viewer-finalize", Some(0)).await;
    apply(&mut author, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    apply(&mut author, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    let mut peer = fresh("viewer-finalize", Some(0)).await;
    deliver(&mut peer, author.event_log().expect("log")).await;
    apply(&mut peer, vec![DemoMutation::AddN(AddN { delta: 100 })]).await;
    let before = shown(&peer);
    assert_eq!(before.2, Some(103));
    let edited_operation = author.mutation_ops().expect("applied operations")[0].mutation_id.clone();
    let drafts: BTreeMap<MutationId, protocol::InputReplacement> =
        [(edited_operation, protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: DemoMutation::SetN(SetN { n: 10 }).encode_op().expect("demo operations encode") })].into_iter().collect();
    let mut replay = author.begin_report_replay(&drafts, None).expect("the session replay");
    drive_test_report_replay(&mut replay, author.replay_edits());
    let finished = replay.finish().expect("a finished replay yields its result");
    author.commit_finished_replay(finished, HistoryFinalization::Alternative { name: "edited".into() }).await.expect("finalize as a new alternative");
    let edited = author.envelope().active_alternative_id.clone().expect("the author stands on the new alternative");
    assert_eq!((author.envelope().viewer_checkpoint_id.clone(), author.snapshot_ref().n), (None, Some(12)));

    let mut batch = author.event_log().expect("log");
    batch.reverse();
    deliver(&mut peer, batch).await;
    assert_eq!(shown(&peer), before, "the author's new alternative leaves the peer's head, projection, applied edits and supersessions alone");
    assert!(peer.envelope().vcs.alternatives.iter().any(|alternative| alternative.id == edited && alternative.name == "edited"), "the peer lists the new alternative");
    test_support::assert_live_equals_replay(&peer).await;

    let author_shown = shown(&author);
    let events = peer.event_log().expect("log").len();
    peer.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: edited.clone() }).await.expect("the peer checks the alternative out locally");
    assert_eq!((peer.envelope().active_alternative_id.clone(), peer.snapshot_ref().n, peer.supersessions().len()), (Some(edited.clone()), Some(12), 1));
    assert_eq!(peer.event_log().expect("log").len(), events, "a local checkout authors nothing");
    deliver(&mut author, peer.event_log().expect("log")).await;
    assert_eq!(shown(&author), author_shown, "the peer's checkout and its trunk edit leave the author's alternative alone");
    let trunk = peer.trunk_alternative_id();
    peer.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: trunk }).await.expect("the peer returns to the trunk");
    assert_eq!(shown(&peer), before);

    let (author_reloaded, peer_reloaded) = (reloaded(&author, "spr").await, reloaded(&peer, "spr").await);
    assert_eq!((shown(&author_reloaded), shown(&peer_reloaded)), (shown(&author), shown(&peer)), ".spr restores each replica's own head and projection");
    let shared = reloaded(&author, "ops").await;
    assert_eq!((shared.envelope().active_alternative_id.clone(), shared.envelope().viewer_checkpoint_id.clone(), shared.snapshot_ref().n), (None, None, Some(103)), "the .ops text is the shared log: it hydrates to the trunk tip");

    let log = author.event_log().expect("the whole log");
    let registrations = |store: &DemoStore| store.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.clone())).collect::<Vec<_>>();
    let mut revisions = std::collections::HashSet::new();
    for rotation in 0..log.len() {
        let mut replica = fresh("viewer-finalize", Some(0)).await;
        let mut arrival = log.clone();
        arrival.rotate_left(rotation);
        deliver(&mut replica, arrival).await;
        assert_eq!((replica.envelope().active_alternative_id.clone(), replica.snapshot_ref().n, replica.supersessions().len()), (None, Some(103), 0), "rotation {rotation}: a replica that only received the log stands on the trunk");
        assert_eq!(registrations(&replica), registrations(&author), "rotation {rotation}: the registrations are shared");
        replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: edited.clone() }).await.expect("a local switch");
        assert_eq!((replica.snapshot_ref().n, replica.supersessions()), (Some(12), author.supersessions()), "rotation {rotation}: equal heads project alike");
        revisions.insert(replica.content_revision_now());
    }
    assert_eq!(revisions.len(), 1, "arrival order never changes what a head's content revision names");
}

/// 🪴️ A checkpoint committed on an alternative depends causally on the alternative's registration: its `Commit` names the
/// `Branch` that listed the line among its dependencies. A replica that takes the log in any rotation or in reverse
/// therefore holds the commit back until it lists the line — it never refuses the fold — and shows the line's
/// registration, checkpoints and projection exactly as the author does.
#[semio_framework_async_macros::async_test]
async fn a_commit_on_an_alternative_waits_for_the_alternative() {
    let mut author = fresh("viewer-commit-line", Some(0)).await;
    apply(&mut author, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    author.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a trunk checkpoint");
    author.dispatch(ArtifactCommand::CreateAlternative { name: "side".into() }).await.expect("an alternative");
    apply(&mut author, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    author.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a checkpoint on the alternative");
    let side = author.envelope().active_alternative_id.clone().expect("the author stands on the alternative");
    let log = author.event_log().expect("log");
    let transitions: Vec<(MutationId, Vec<MutationId>, crate::os_spr::HistoryTransition)> =
        log.iter().filter_map(|event| crate::os_spr::history_transition_from_envelope(event).ok().flatten().map(|transition| (event.mutation_id.clone(), event.dependencies.clone(), transition))).collect();
    let registration = transitions.iter().find(|(_, _, transition)| matches!(transition, crate::os_spr::HistoryTransition::Branch { alternative_id, .. } if *alternative_id == side)).map(|(id, _, _)| id.clone()).expect("the registration is in the log");
    let (_, dependencies, _) = transitions.iter().find(|(_, _, transition)| matches!(transition, crate::os_spr::HistoryTransition::Commit(checkpoint) if checkpoint.line_id.as_deref() == Some(side.as_str()))).expect("the commit on the alternative is in the log");
    assert!(dependencies.contains(&registration), "the commit names its line's registration among its dependencies");
    let registrations = |store: &DemoStore| store.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.clone())).collect::<Vec<_>>();
    let mut arrivals = vec![log.iter().rev().cloned().collect::<Vec<_>>()];
    arrivals.extend((1..log.len()).map(|rotation| {
        let mut rotated = log.clone();
        rotated.rotate_left(rotation);
        rotated
    }));
    for (arrival, events) in arrivals.into_iter().enumerate() {
        let mut replica = fresh("viewer-commit-line", Some(0)).await;
        deliver(&mut replica, events).await;
        assert_eq!(registrations(&replica), registrations(&author), "arrival {arrival}: the line and its checkpoints are shared");
        replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: side.clone() }).await.expect("a local switch");
        assert_eq!(replica.snapshot_ref().n, Some(3), "arrival {arrival}: the line projects as its author's");
    }
}
//#endregion 🧪️FinalizeLaws

//#region 🧪️ReloadLaws
type SeverityStore = ArtifactStore<DemoSnapshot, SeverityMutation>;

/// ✏️ One history edit of `target` through the session path: the Report replay of its draft, finalized as `finalization`.
async fn finalized(store: &mut SeverityStore, target: &MutationId, operation: SeverityMutation, finalization: HistoryFinalization) {
    let drafts: BTreeMap<MutationId, protocol::InputReplacement> = [(target.clone(), protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().expect("severity operations encode") })].into_iter().collect();
    let mut replay = store.begin_report_replay(&drafts, None).expect("the session replay");
    drive_test_report_replay(&mut replay, store.replay_edits());
    let finished = replay.finish().expect("a finished replay yields its result");
    store.commit_finished_replay(finished, finalization).await.expect("the history edit finalizes");
}

/// 🧾️ Everything a history panel reads of a store's log: every edit with its author and operations, every history
/// transition, the alternatives with their chains, and every mutation's durable outcome.
#[allow(clippy::type_complexity)]
fn ledger(store: &SeverityStore) -> (Vec<(String, Option<String>, Vec<SeverityMutation>)>, Vec<MutationId>, Vec<(String, String, Vec<String>)>, Vec<(MutationId, Option<semio_framework_diagnostic::Severity>, Vec<String>, bool, bool)>) {
    (
        store.envelope().vcs.edits.iter().map(|edit| (edit.id.clone(), edit.actor.clone(), edit.forwards.clone())).collect(),
        store.envelope().transitions.iter().map(|transition| transition.mutation_id.clone()).collect(),
        store.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.clone())).collect(),
        store.mutation_outcomes().expect("durable outcomes").into_iter().map(|outcome| (outcome.mutation_id, outcome.worst, outcome.messages.into_iter().map(|message| message.code.0).collect(), outcome.superseded, outcome.withdrawn)).collect(),
    )
}

/// 📁️ The live probe's folder sequence at the store (e2e R2-2, design §22.3): edits, a history edit finalized as an
/// overwrite whose replay leaves a warning, a second one finalized as a new alternative. The pair a detach persists
/// restores on re-attach every edit, every history transition (both supersessions and the registration), the alternatives,
/// this replica's head (`REC_VIEWER`), the effective supersessions, the durable warning, the projection and the content
/// revision; the `.ops` text — the shared log — restores all of it but the head, at the trunk tip.
#[semio_framework_async_macros::async_test]
async fn the_persisted_pair_restores_edits_history_edits_alternatives_the_head_and_warnings() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "folder-reload", DemoSnapshot { n: Some(0) }, None)).await;
    for n in [1, 2, 3] {
        store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n })], transaction: None }).await.expect("a clean edit applies");
    }
    let ids: Vec<MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    finalized(&mut store, &ids[0], SeverityMutation::SetWarningN(SetWarningN { n: 5 }), HistoryFinalization::Overwrite).await;
    finalized(&mut store, &ids[2], SeverityMutation::SetN(SeveritySetN { n: 30 }), HistoryFinalization::Alternative { name: "Edited history".into() }).await;
    let head = (store.envelope().active_alternative_id.clone(), store.envelope().viewer_checkpoint_id.clone());
    let expected = ledger(&store);
    assert!(head.0.is_some() && expected.2.iter().any(|(id, name, _)| Some(id) == head.0.as_ref() && name == "Edited history"), "the source stands on its new alternative");
    assert_eq!(expected.1.len(), 4, "an overwrite, then a commit, a registration and a scoped supersession");
    assert!(expected.3.iter().any(|(target, worst, _, superseded, _)| *target == ids[0] && *worst == Some(semio_framework_diagnostic::Severity::Warning) && *superseded), "the overwrite left a durable warning");
    assert_eq!((store.snapshot_ref().n, store.supersessions().len()), (Some(30), 2));

    let files = print_document_pack(store.envelope()).await.expect("the pair prints");
    let reattached = ArtifactStore::new(parse_document_pack::<DemoSnapshot, SeverityMutation>(&files.pack, &files.spr).await.expect("the pair parses").into_envelope()).await;
    assert_eq!(ledger(&reattached), expected, "edits, history transitions, alternatives and outcomes survive the re-attach");
    assert_eq!((reattached.envelope().active_alternative_id.clone(), reattached.envelope().viewer_checkpoint_id.clone()), head, "the replica stands where its own pair says");
    assert_eq!((reattached.snapshot_ref().n, reattached.supersessions(), reattached.applied_edit_ids().iter().collect::<Vec<_>>()), (Some(30), store.supersessions(), store.applied_edit_ids().iter().collect::<Vec<_>>()));
    assert_eq!(reattached.content_revision_now(), store.content_revision_now(), "a reload names the same content");
    test_support::assert_live_equals_replay(&reattached).await;

    let text = print_document_text(store.envelope()).await.expect("the text prints");
    let shared = ArtifactStore::new(parse_document_text::<DemoSnapshot, SeverityMutation>(&text.dsl, &text.ops).await.expect("the text parses").into_envelope()).await;
    let (edits, transitions, alternatives, _) = ledger(&shared);
    assert_eq!((edits, transitions, alternatives), (expected.0.clone(), expected.1.clone(), expected.2.clone()), "the shared log carries every edit, transition and alternative");
    assert_eq!((shared.envelope().active_alternative_id.clone(), shared.envelope().viewer_checkpoint_id.clone(), shared.snapshot_ref().n, shared.supersessions().len()), (None, None, Some(3), 1), "and no head: it hydrates to the trunk tip, where only the overwrite holds");
    test_support::assert_document_pack_round_trip(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
}
//#endregion 🧪️ReloadLaws

//#region 🧪️PairMergeLaws
/// 🫱️ Two replicas on one folder (probe batch H): a read-back of the other writer's pair merges its log and never moves the
/// reader. A stands on its own alternative with an uncommitted edit there; B edits the trunk and writes the pair. A's merge
/// takes B's edit and leaves A's head, projection, applied edits and supersessions alone; A persists because it is ahead; B's
/// merge of that union takes A's four events and keeps B on the trunk although the pair names A's alternative; then neither
/// has anything left to take or to persist. A pair of another document is refused with nothing changed, and only a cold load
/// stands where a pair says.
#[semio_framework_async_macros::async_test]
async fn a_read_back_pair_merges_its_log_and_never_moves_the_reader() {
    let mut a = fresh("pair-merge", Some(0)).await;
    apply(&mut a, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    apply(&mut a, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    let mut b = fresh("pair-merge", Some(0)).await;
    deliver(&mut b, a.event_log().expect("log")).await;
    let first = a.mutation_ops().expect("applied operations")[0].mutation_id.clone();
    a.dispatch(ArtifactCommand::CreateAlternativeWithSupersede { name: "mine".into(), inputs: vec![SupersedeInput { target: first, replacement: Some(DemoMutation::SetN(SetN { n: 10 })) }] }).await.expect("a history edit as a new alternative");
    apply(&mut a, vec![DemoMutation::AddN(AddN { delta: 5 })]).await;
    apply(&mut b, vec![DemoMutation::AddN(AddN { delta: 100 })]).await;
    let (a_shown, b_shown) = (shown(&a), shown(&b));
    assert_eq!((a_shown.2, b_shown.2), (Some(17), Some(103)));

    let written = print_document_pack(b.envelope()).await.expect("b's pair prints");
    assert_eq!(a.merge_persisted_pair(&written.pack, &written.spr).await.expect("the read-back merges"), PairMerge { merged: 1, ahead: 4 });
    assert_eq!(shown(&a), a_shown, "the read-back leaves the reader's head, projection, applied edits and supersessions alone");
    test_support::assert_live_equals_replay(&a).await;

    let union = print_document_pack(a.envelope()).await.expect("a's pair prints");
    assert_eq!(b.merge_persisted_pair(&union.pack, &union.spr).await.expect("the union merges"), PairMerge { merged: 4, ahead: 0 });
    assert_eq!(shown(&b), b_shown, "the pair names its writer's alternative; the reader stays on the trunk");
    assert_eq!(listed(&b), listed(&a), "the registration is shared");
    test_support::assert_live_equals_replay(&b).await;

    let (a_pair, b_pair) = (print_document_pack(a.envelope()).await.expect("a's pair prints"), print_document_pack(b.envelope()).await.expect("b's pair prints"));
    assert_eq!(a.merge_persisted_pair(&b_pair.pack, &b_pair.spr).await.expect("nothing left to take"), PairMerge::default());
    assert_eq!(b.merge_persisted_pair(&a_pair.pack, &a_pair.spr).await.expect("nothing left to take"), PairMerge::default());
    let trunk = a.trunk_alternative_id();
    let mine = a.active_line_id();
    a.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: trunk }).await.expect("a looks at the trunk");
    assert_eq!(a.snapshot_ref().n, Some(103), "the merged edit shows where it was authored");
    a.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: mine }).await.expect("a returns");
    assert_eq!(shown(&a), a_shown);

    let mut other = fresh("another-document", Some(0)).await;
    apply(&mut other, vec![DemoMutation::SetN(SetN { n: 7 })]).await;
    let foreign = print_document_pack(other.envelope()).await.expect("the other pair prints");
    let generation = a.generation();
    let refused = a.merge_persisted_pair(&foreign.pack, &foreign.spr).await;
    assert!(matches!(refused, Err(VcsError::ValidationFailed(_))), "{refused:?}");
    assert_eq!((a.generation(), shown(&a)), (generation, a_shown.clone()), "a refused merge changes nothing");
    let cold = reloaded(&a, "spr").await;
    assert_eq!((cold.envelope().active_alternative_id.clone(), cold.snapshot_ref().n), (a_shown.0.clone(), Some(17)), "only a cold load stands where its pair says");
}

/// 👥️ The live two-peer fault F4 (probe batch H, two tabs on one folder) at the store, as the acceptance case of the merge:
/// A attaches and edits, B opens the folder's pair cold; A opens a history edit with a draft; B edits and writes its pair. A's
/// read-back MERGES: B's edit is ingested as a remote event, the store's generation moves (the session's base move: a replay
/// begun before it is refused `Stale`), the preview before the edited mutation is unchanged and B's edit is downstream of
/// it; A's Accept replays B's edit too; A's finalize puts A ahead of the folder, so A persists a pair holding both; B merges
/// it, and both replicas show the same document, supersession and applied edits — and neither has anything left to persist.
#[semio_framework_async_macros::async_test]
async fn two_peers_on_one_folder_converge_through_an_open_history_edit() {
    let mut a = fresh("two-peers", Some(0)).await;
    apply(&mut a, vec![DemoMutation::SetN(SetN { n: 100 })]).await;
    apply(&mut a, vec![DemoMutation::AddN(AddN { delta: 60 })]).await;
    let folder = print_document_pack(a.envelope()).await.expect("a writes the folder");
    let mut b = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&folder.pack, &folder.spr).await.expect("b opens the folder").into_envelope()).await;
    assert_eq!((b.snapshot_ref().n, b.applied_edit_ids().len()), (Some(160), 2), "b shows a's document and a's edit");

    let target = a.mutation_ops().expect("applied operations")[1].mutation_id.clone();
    let drafts: BTreeMap<MutationId, protocol::InputReplacement> = [(target.clone(), protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: DemoMutation::AddN(AddN { delta: 80 }).encode_op().expect("demo operations encode") })].into_iter().collect();
    assert_eq!(a.state_before(&target, &drafts).expect("the preview base").n, Some(100));
    let mut early = a.begin_report_replay(&drafts, None).expect("a replay begun before the peer's write");
    drive_test_report_replay(&mut early, a.replay_edits());
    let early = early.finish().expect("a finished replay yields its result");

    apply(&mut b, vec![DemoMutation::AddN(AddN { delta: 1000 })]).await;
    let folder = print_document_pack(b.envelope()).await.expect("b writes the folder");
    let generation = a.generation();
    assert_eq!(a.merge_persisted_pair(&folder.pack, &folder.spr).await.expect("a's read-back merges"), PairMerge { merged: 1, ahead: 0 });
    assert!(a.generation() > generation, "the merge is a base move the open session sees");
    assert_eq!((a.snapshot_ref().n, a.applied_edit_ids().len(), a.supersessions().len()), (Some(1160), 3, 0), "a's store took the peer's edit; the draft is no event yet");
    assert_eq!(a.state_before(&target, &drafts).expect("the preview base").n, Some(100), "the peer's edit is downstream of the edited mutation and not applied before it");
    let peer_edit = a.mutation_ops().expect("applied operations").into_iter().find(|operation| operation.position == 2).map(|operation| operation.mutation_id).expect("the peer's edit follows the edited mutation");
    assert!(matches!(a.commit_finished_replay(early, HistoryFinalization::Overwrite).await, Err(VcsError::Stale { .. })), "a replay begun before the base move is stale");

    let mut replay = a.begin_report_replay(&drafts, None).expect("accept: the replay over the moved base");
    drive_test_report_replay(&mut replay, a.replay_edits());
    let accepted = replay.finish().expect("a finished replay yields its result");
    let report = a.replay_report(&accepted).expect("the report");
    assert!(report.outcomes.iter().any(|outcome| outcome.mutation_id == peer_edit), "accept replays the peer's edit too");
    assert_eq!((accepted.state().expect("the reviewed head").n, report.blocks_finalize()), (Some(1180), false));
    a.commit_finished_replay(accepted, HistoryFinalization::Overwrite).await.expect("finalize: overwrite");
    assert_eq!(a.snapshot_ref().n, Some(1180));

    assert_eq!(a.merge_persisted_pair(&folder.pack, &folder.spr).await.expect("the folder still holds b's pair"), PairMerge { merged: 0, ahead: 1 }, "the finalize put a ahead of the folder: it persists");
    let folder = print_document_pack(a.envelope()).await.expect("a writes the folder");
    assert_eq!(b.merge_persisted_pair(&folder.pack, &folder.spr).await.expect("b's read-back merges"), PairMerge { merged: 1, ahead: 0 });
    assert_eq!((shown(&b).2, shown(&b).3.len(), b.supersessions()), (Some(1180), 3, a.supersessions()), "both peers show the same document");
    assert_eq!((a.snapshot_ref().n, a.applied_edit_ids().len(), a.supersessions().len()), (Some(1180), 3, 1));
    test_support::assert_live_equals_replay(&a).await;
    test_support::assert_live_equals_replay(&b).await;
    let folder = print_document_pack(b.envelope()).await.expect("b's pair prints");
    assert_eq!(a.merge_persisted_pair(&folder.pack, &folder.spr).await.expect("nothing left to take"), PairMerge::default(), "neither peer has anything left to take or to persist");
}
//#endregion 🧪️PairMergeLaws

//#region 🧪️PortLaws
/// 🔌️ A port rebinding changes no event (live fault F4, probe batch H): binding and unbinding the document port moves the
/// store's generation but not its content revision, and a history-edit replay finished before the rebinding still commits —
/// only a change of content makes it stale.
#[semio_framework_async_macros::async_test]
async fn a_port_rebinding_moves_no_content_and_keeps_a_finished_replay() {
    let mut store = fresh("rebind", Some(0)).await;
    apply(&mut store, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    apply(&mut store, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    let target = store.mutation_ops().expect("applied operations")[0].mutation_id.clone();
    let drafts: BTreeMap<MutationId, protocol::InputReplacement> = [(target, protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: DemoMutation::SetN(SetN { n: 10 }).encode_op().expect("demo operations encode") })].into_iter().collect();
    let finish = |store: &DemoStore| {
        let mut replay = store.begin_report_replay(&drafts, None).expect("the session replay");
        drive_test_report_replay(&mut replay, store.replay_edits());
        replay.finish().expect("a finished replay yields its result")
    };
    let kept = finish(&store);
    let (generation, revision) = (store.generation(), store.content_revision_now());
    let (port, far) = ChannelBackbone::pair("rebind").await;
    store.attach_hot_backbone(Backbones::Channel(port)).await.expect("the document port binds");
    drop(store.detach_backbone().expect("the document port unbinds"));
    drop(far);
    assert!(store.generation() > generation, "a rebinding is a change of the store");
    assert_eq!(store.content_revision_now(), revision, "and of no event");
    store.commit_finished_replay(kept, HistoryFinalization::Overwrite).await.expect("a replay finished before the rebinding still commits");
    assert_eq!(store.snapshot_ref().n, Some(12));
    test_support::assert_live_equals_replay(&store).await;
}
//#endregion 🧪️PortLaws
