//! ⏪️ The language-agnostic time-travel corpus of the puzzle 5d selection leaves (board and world) (`🧫️fixtures/🧫️selection-time-travel/🔣️.json`,
//! schema `🧬️schema/🔣️selection-time-travel/🔣️.json`), driven through the store's time-travel reads: every recorded
//! gesture is ONE edit carrying its `TransactionRef`; editing one gesture's inputs — its offset, angle, factors or
//! targets — previews as the state right before it plus the draft with nothing downstream applied, its Report replay
//! re-applies the whole downstream suffix with the committed per-mutation outcomes, a cancelled replay leaves zero
//! trace, and an overwrite folds to exactly the corpus' fresh fold of the edited log while the tool transactions stay.
//! The same corpus is folded by the independent Python oracle and validated with `jsonschema`
//! (`🧪️w3-t-puzzle-oracle-selfcheck.py` in ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`).

use super::*;
use protocol::OpBinary;

const CORPUS: &str = include_str!("../../../../🧫️fixtures/🧫️selection-time-travel/🔣️.json");

/// 🗂️ One decoded corpus case.
struct TimeTravelCase {
    id: String,
    base: Puzzle5dSnapshot,
    log: Vec<Puzzle5dMutation>,
    at: usize,
    replacement: Puzzle5dMutation,
    preview: Puzzle5dSnapshot,
    replayed: Puzzle5dSnapshot,
    outcomes: Vec<(usize, Option<protocol::Severity>, Vec<String>)>,
    blocks: bool,
}

/// 🎯️ Decodes the corpus with the framework JSON reader, whose float parse is correctly rounded: `serde_json` without
/// `float_roundtrip` lands some 17-digit decimals one ulp off (`1.9999999999999993` → `…91`), which made an exact
/// cross-language state compare fail on the harness, not on the leaves.
fn cases() -> Vec<TimeTravelCase> {
    let corpus = dsl::json::parse(CORPUS).expect("the corpus parses");
    let snapshot = |value: &dsl::json::Value| -> Puzzle5dSnapshot { dsl::json::from_json_str(&dsl::json::to_string(value)).expect("a corpus snapshot decodes") };
    let mutation = |value: &dsl::json::Value| -> Puzzle5dMutation { dsl::json::from_json_str(&dsl::json::to_string(value)).expect("a corpus payload decodes") };
    let level = |value: &dsl::json::Value| match value.as_str() {
        None => None,
        Some("info") => Some(protocol::Severity::Info),
        Some("warning") => Some(protocol::Severity::Warning),
        Some("error") => Some(protocol::Severity::Error),
        Some("fatal") => Some(protocol::Severity::Fatal),
        Some(other) => panic!("unknown corpus level {other:?}"),
    };
    corpus["cases"]
        .as_array()
        .expect("the corpus lists its cases")
        .iter()
        .map(|case| TimeTravelCase {
            id: case["id"].as_str().expect("a case id").to_string(),
            base: snapshot(&case["base"]),
            log: case["log"].as_array().expect("a case log").iter().map(mutation).collect(),
            at: case["edit"]["at"].as_u64().expect("the edited position") as usize,
            replacement: mutation(&case["edit"]["replacement"]),
            preview: snapshot(&case["preview"]),
            replayed: snapshot(&case["replayed"]),
            outcomes: case["outcomes"]
                .as_array()
                .expect("the case outcomes")
                .iter()
                .map(|outcome| (outcome["at"].as_u64().expect("an outcome position") as usize, level(&outcome["worst"]), outcome["codes"].as_array().expect("outcome codes").iter().map(|code| code.as_str().expect("a code").to_string()).collect()))
                .collect(),
            blocks: case["blocksFinalize"].as_bool().expect("the finalize verdict"),
        })
        .collect()
}

/// 🪪️ The transaction a corpus gesture is recorded under — what the transform tool stamps on its one edit.
fn gesture_transaction(index: usize) -> protocol::TransactionRef {
    protocol::TransactionRef { id: format!("tx-{index:016x}"), tool: format!("s.puzzle.puzzle5d@1/*#editor#gesture{index}") }
}

/// ⏪️ Every corpus edit of a selection leaf's offset, angle, factors or targets previews without downstream, replays
/// the downstream suffix with the committed outcomes and overwrites to the corpus' fresh fold; a cancelled replay
/// leaves zero trace; one gesture stays one edit with its `TransactionRef` throughout.
#[semio_framework_async_macros::async_test]
async fn every_corpus_edit_previews_replays_and_overwrites_like_the_fresh_fold() {
    let cases = cases();
    assert!(cases.len() >= 7, "the corpus edits every selection leaf's parameters and targets");
    for case in cases {
        let id = case.id.as_str();
        let mut store = crate::standards::v1::subsets::any::schema::mutations::binary::puzzle5d_store(store::create_document_envelope::<Puzzle5dSnapshot, Puzzle5dMutation>(crate::PUZZLE_5D_SCHEMA, &format!("time-travel-{id}"), case.base.clone(), None)).await.expect("the store opens");
        for (index, mutation) in case.log.iter().enumerate() {
            store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: Some(gesture_transaction(index)) }).await.expect("a recorded gesture applies");
        }
        let ids: Vec<protocol::MutationId> = {
            let operations = store.mutation_ops().expect("applied operations");
            assert_eq!(operations.iter().map(|operation| operation.position).collect::<Vec<_>>(), (0..case.log.len()).collect::<Vec<_>>(), "{id}: one gesture is one edit");
            assert!(operations.iter().enumerate().all(|(index, operation)| operation.transaction == Some(&gesture_transaction(index))), "{id}: every gesture's operation carries its TransactionRef");
            operations.iter().map(|operation| operation.mutation_id.clone()).collect()
        };
        let head = store.snapshot_ref().clone();
        let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[case.at].clone(), protocol::InputReplacement::Input { schema: crate::PUZZLE_5D_SCHEMA.into(), payload: case.replacement.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
        let mut preview = store.state_before(&ids[case.at], &drafts).expect("the preview base folds").as_ref().clone();
        apply_puzzle5d_mutation(&mut preview, &case.replacement).expect("the draft applies to its base");
        assert_eq!(preview, case.preview, "{id}: the preview is the state before the edited gesture plus the draft, nothing downstream");
        store.begin_report_replay(&drafts, Some(&ids[case.at])).expect("a replay begins").cancel();
        assert_eq!(store.snapshot_ref(), &head, "{id}: a cancelled replay leaves the head untouched");
        assert!(store.mutation_ops().expect("applied operations").iter().all(|operation| operation.supersession.is_none()), "{id}: a cancelled replay supersedes nothing");
        let mut replay = store.begin_report_replay(&drafts, Some(&ids[case.at])).expect("the replay begins at the edited gesture");
        while !matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)) {}
        let result = replay.finish().expect("a finished replay yields its result");
        let report = store.replay_report(&result).expect("report");
        assert_eq!(report.blocks_finalize(), case.blocks, "{id}: the finalize verdict");
        for (at, worst, codes) in &case.outcomes {
            let outcome = report.outcomes.iter().find(|outcome| outcome.mutation_id == ids[*at]).unwrap_or_else(|| panic!("{id}: gesture {at} is reported"));
            assert_eq!((outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect::<Vec<_>>()), (*worst, codes.clone()), "{id}: the outcome of gesture {at}");
        }
        assert_eq!(result.state().expect("the replay reached a state").as_ref(), &case.replayed, "{id}: the replay is the fresh fold of the edited log");
        if case.blocks {
            drop(result);
            assert_eq!(store.snapshot_ref(), &head, "{id}: a blocked replay is never committed");
        } else {
            store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
            assert_eq!(store.snapshot_ref(), &case.replayed, "{id}: the overwritten history folds to the edited state");
            let operations = store.mutation_ops().expect("applied operations");
            assert!(operations[case.at].supersession.is_some(), "{id}: the edited gesture is superseded, never rewritten");
            assert!(operations.iter().enumerate().all(|(index, operation)| operation.transaction == Some(&gesture_transaction(index))), "{id}: time travel edits the mutation, never the tool transaction");
        }
        crate::standards::v1::subsets::any::schema::mutations::binary::close_puzzle5d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
    }
}
