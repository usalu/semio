//! 📤️ LAW: a store attached to a backbone announces every locally authored operation exactly once
//! (`🧬️schema/📤️outbound-announcement`, fixture `🧫️fixtures/📤️outbound-announcement`, TS twin
//! `💻️os/🧪️tests/📤️outbound-announcement`). A coalesced batched gesture amends the tail edit the
//! previous gesture already announced and must announce only its own operations: re-announcing the
//! earlier ones made the hub refuse a writer's second keystroke as a replay (ticket 26/09/23 C10 09:4x). Every
//! announced operation — a batched gesture's like any other — names the newest foreign operation its author had
//! applied as `observed`, and a remote operation observing something this replica never saw applies at once
//! (ticket 26/09/23 LD item 2).

use super::*;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Fixture {
    schema: String,
    vectors: Vec<Vector>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Vector {
    id: String,
    steps: Vec<Step>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Step {
    gesture: Option<Gesture>,
    undo: Option<bool>,
    remote: Option<Remote>,
    expect: Expect,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Remote {
    id: String,
    observed: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Gesture {
    coalesce_key: Option<String>,
    items: usize,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Expect {
    announced_operations: usize,
    announced_transitions: usize,
    edits: usize,
    tail_operations: usize,
    observed: Option<String>,
}

/// 📨️ Every envelope the store sent since the last drain, split into edit operations and history transitions.
fn drain_announced(remote: &ChannelBackboneRemote) -> (Vec<crate::os_spr::MutationEnvelope>, Vec<crate::os_spr::MutationEnvelope>) {
    let (mut operations, mut transitions) = (Vec::new(), Vec::new());
    for message in drain_channel_for_test(remote).expect("drain outbound") {
        if let BackboneMessage::Mutations { envelopes } = message {
            for envelope in crate::os_spr::decode_envelopes(&envelopes).expect("announced envelopes decode") {
                let target = if crate::os_spr::is_history_transition(&envelope) { &mut transitions } else { &mut operations };
                target.push(envelope);
            }
        }
    }
    (operations, transitions)
}

/// 🧺️ Publishes one gesture exactly as the plugin SDK does on a store with a backbone: an outbound batch, its
/// coalescing key, advanced to its receipt, announced once, then acknowledged and closed.
async fn publish_outbound_gesture(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, operation: u64, gesture: &Gesture, next_value: &mut i32) {
    let factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>> = Arc::new(DemoOneItemPreparationFactory::admissible());
    let mutations = (0..gesture.items)
        .map(|_| {
            *next_value += 1;
            DemoMutation::SetN(SetN { n: *next_value })
        })
        .collect();
    let mut publication = store
        .begin_outbound_apply_batch(semio_framework_job::OperationId(operation), store.generation_now(), store.content_revision_now(), "retained-test".into(), mutations, None, Some(&factory))
        .unwrap_or_else(|rejected| panic!("outbound batch admission: {}", rejected.reason));
    publication.set_coalesce_key(gesture.coalesce_key.clone());
    let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 512 };
    for _ in 0..4_096 {
        if let ArtifactStoreOneItemAdvance::Published(_) = store.advance_apply_batch(&mut publication, grant).expect("bounded batch step") {
            break;
        }
    }
    assert!(!publication.acknowledge(), "an outbound publication refuses its ACK before it is announced");
    assert!(store.flush_published_apply_batch(&mut publication).await.expect("announce the published gesture"));
    assert!(!store.flush_published_apply_batch(&mut publication).await.expect("a second flush is refused"), "a publication is announced once");
    assert!(publication.acknowledge());
    close_durable_publication(&mut publication);
}

#[semio_framework_async_macros::async_test]
async fn every_locally_authored_operation_is_announced_exactly_once() {
    let fixture: Fixture = serde_json::from_str(include_str!("../../🧫️fixtures/📤️outbound-announcement/🔣️.json")).expect("outbound announcement fixture");
    assert_eq!(fixture.schema, "semio.store.outbound-announcement/v1");
    let mut steps = 0usize;
    for vector in &fixture.vectors {
        let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None)).await;
        store.install_document_store_owners_exact(demo_closable_store_owners());
        store.closes_on_drop();
        let (channel, remote) = ChannelBackbone::pair("outbound").await;
        store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
        drain_announced(&remote);
        let (mut announced, mut foreign, mut next_value) = (Vec::<String>::new(), Vec::<String>::new(), 0i32);
        for (index, step) in vector.steps.iter().enumerate() {
            match (&step.gesture, step.undo, &step.remote) {
                (Some(gesture), None, None) => publish_outbound_gesture(&mut store, index as u64 + 1, gesture, &mut next_value).await,
                (None, Some(true), None) => {
                    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
                }
                (None, None, Some(arriving)) => {
                    next_value += 1;
                    let mut envelope = mutation_envelope_at("peer", &arriving.id, DemoMutation::SetN(SetN { n: next_value }), HybridLogicalTimestamp { actor: 7, physical_ms: 4_000_000_000_000 + index as u64, logical: 0 }, Vec::new());
                    envelope.observed = arriving.observed.clone().map(MutationId);
                    remote.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&[envelope]) }).await.expect("push the remote operation");
                    store.tick().await.expect("ingest the remote operation");
                    assert!(store.envelope().vcs.edits.iter().any(|edit| edit.id == arriving.id), "{} step {index}: a remote operation is applied at once, whatever it observed", vector.id);
                    foreign.push(arriving.id.clone());
                }
                _ => panic!("{} step {index} names neither one gesture, an undo nor one remote operation", vector.id),
            }
            let (operations, transitions) = drain_announced(&remote);
            let observations: HashSet<Option<String>> = operations.iter().map(|operation| operation.observed.as_ref().map(|observed| observed.0.clone())).collect();
            assert!(observations.len() <= 1, "{} step {index}: one authoring moment observes one foreign operation: {observations:?}", vector.id);
            let tail_operations = store.applied_edit_ids().last().map_or(0, |tail| store.envelope().vcs.edits.iter().find(|edit| edit.id == *tail).expect("tail edit").forwards.len());
            let measured = Expect { announced_operations: operations.len(), announced_transitions: transitions.len(), edits: store.envelope().vcs.edits.len(), tail_operations, observed: observations.into_iter().next().flatten() };
            assert_eq!(measured, step.expect, "{} step {index}: announced {operations:?} + {transitions:?}", vector.id);
            announced.extend(operations.into_iter().map(|operation| operation.mutation_id.0));
            steps += 1;
        }
        let unique: HashSet<&String> = announced.iter().collect();
        assert_eq!(unique.len(), announced.len(), "{}: an operation was announced twice: {announced:?}", vector.id);
        let ledger: HashSet<String> = store.envelope().vcs.edits.iter().flat_map(|edit| crate::os_spr::mutation_ids_for_edit::<DemoSnapshot, DemoMutation>(edit)).map(|id| id.0).filter(|id| !foreign.contains(id)).collect();
        assert_eq!(unique.into_iter().cloned().collect::<HashSet<_>>(), ledger, "{}: the announced operations are exactly the ledger's", vector.id);
    }
    assert!(steps >= 22, "the fixture walks every declared step");
}
