//! ⚡️ Store laws of the O(change) mutation path (design §20.14, audits W1G-3 and D22, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING):
//! past thousands of edits an `Apply`, a tool transaction's tick and its commit, and an undo or redo of the applied tail never
//! fold the event log, hash a prefix of the history only to seed the prefix ring once, and mirror only what they changed into the
//! persisted cursor and the revision records. The language-agnostic
//! corpus is `🧫️fixtures/⚡️hot-path/🔣️.json` (schema `🧬️schema/⚡️hot-path/🔣️.json`); every short-history bump re-derives the
//! cursor, the revision and the fold from scratch (`assert_mirrors_are_live`), so the incremental path is checked against the
//! full fold wherever the corpus and every other store law run.
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Corpus {
    schema: String,
    vectors: Vec<Vector>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Vector {
    id: String,
    warmup: usize,
    steps: Vec<Step>,
    expect: Census,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Step {
    kind: StepKind,
    count: usize,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
enum StepKind {
    Apply,
    Append,
    Commit,
    Undo,
    Redo,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Census {
    folds: usize,
    digested: usize,
    mirrored: usize,
    rebuilt: usize,
}

const TRANSACTION: &str = "tx-hot-path";

/// 🧹️ Retires every displaced owner a step left behind, as the runtime's maintenance turns do between steps.
fn settle(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) {
    while store.maintenance_retirements_under_pressure() {
        let grant=physical_test_close_grant();
        let step=store.maintenance_retirements_step(grant).expect("original displaced owners retire");
        assert!(step.progress().fits(grant));
    }
}

async fn apply(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, n: i32, identity:&mut crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: None }, identity).await.expect("a plain edit applies");
    settle(store);
}

#[semio_framework_async_macros::async_test]
async fn local_steps_cost_o_change_however_long_the_history() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<crate::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let corpus: Corpus = serde_json::from_str(include_str!("../../🧫️fixtures/⚡️hot-path/🔣️.json")).expect("hot path corpus");
    assert_eq!(corpus.schema, "semio.store.hot-path/v1");
    for vector in &corpus.vectors {
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", &vector.id, DemoSnapshot { n: Some(0) }, None)).await;
        for n in 0..vector.warmup {
            apply(&mut store, n as i32, &mut identity).await;
        }
        let mut next = vector.warmup as i32;
        let transaction = protocol::TransactionRef { id: TRANSACTION.into(), tool: "demo#drag".into() };
        hot_path_census::take();
        for step in &vector.steps {
            for _ in 0..step.count {
                next += 1;
                match step.kind {
                    StepKind::Apply => apply(&mut store, next, &mut identity).await,
                    StepKind::Append => drop(store.dispatch(ArtifactCommand::AppendTransaction { mutations: vec![DemoMutation::AddN(AddN { delta: 1 })], transaction: transaction.clone() }, &mut identity).await.expect("a tick appends")),
                    StepKind::Commit => drop(store.dispatch(ArtifactCommand::CommitTransaction { transaction_id: TRANSACTION.into() }, &mut identity).await.expect("the transaction commits")),
                    StepKind::Undo => drop(store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("the applied tail undoes")),
                    StepKind::Redo => drop(store.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("the redo top redoes")),
                }
                settle(&mut store);
            }
        }
        let census = hot_path_census::take();
        let measured = Census { folds: census.folds, digested: census.digested, mirrored: census.mirrored, rebuilt: census.rebuilt };
        assert_eq!(measured, vector.expect, "{}: whole-history work past {} edits", vector.id, vector.warmup);
        test_support::assert_live_equals_replay(&store).await;
        let fold = fold_envelope_history(store.envelope()).expect("the log folds");
        assert_eq!(fold.applied, store.applied_edit_ids().to_vec(), "{}: the unfolded tail is the fold's applied order", vector.id);
        assert_eq!(fold.redo, store.redo_edit_ids().to_vec(), "{}: and its redo stack", vector.id);
        let cursor = store.envelope().cursor.as_ref().expect("cursor");
        assert!(cursor.applied_edit_ids == *store.applied_edit_ids() && cursor.redo_edit_ids == *store.redo_edit_ids(), "{}: the persisted cursor mirrors the live stacks", vector.id);
    }
}
