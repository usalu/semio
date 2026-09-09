//! 🧪️ Live slots use real Store leases, full authority tokens, and bounded three-root retirement.

use super::*;
use crate::{app::InteractionConfigMutation, local_interaction::retirement::interaction_store_owners};
use store::{ArtifactStore, ErasedSnapshotRetirement, SpaceMember};

type TestStore = ArtifactStore<protocol::InteractionState, InteractionConfigMutation>;
type Query = LocalInteractionLiveQuery<protocol::InteractionState, protocol::InteractionState>;

async fn stores() -> [TestStore; 3] {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "semantic-unicode-over-page").unwrap();
    let mut state = row["expected"].clone();
    state["hover"] = serde_json::json!({});
    let state: protocol::InteractionState = serde_json::from_value(state).unwrap();
    let mut result = Vec::new();
    for id in ["document", "config", "interaction"] {
        let envelope = store::create_document_envelope::<protocol::InteractionState, InteractionConfigMutation>("framework.interaction", id, state.clone(), None);
        let mut store = TestStore::new(envelope).await.unwrap();
        store.install_member_store_owners_exact(interaction_store_owners());
        result.push(store);
    }
    result.try_into().ok().unwrap()
}

fn query(stores: &[TestStore; 3], generation: u64) -> Query {
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: stores[2].generation_now(), revision: stores[2].content_revision_now(), document_revision: stores[0].content_revision_now(), topology_revision: [8; 32] };
    Query::new(13, generation, identity, Some(stores[0].snapshot_read().unwrap()), Some(stores[1].snapshot_read().unwrap()), Some(stores[2].snapshot_read().unwrap()))
}

fn pump(stores: &mut [TestStore; 3], owners: &mut [Option<Box<dyn ErasedSnapshotRetirement>>; 3], bytes: usize) {
    for (store, owner) in stores.iter_mut().zip(owners.iter_mut()) {
        if owner.is_none() {
            *owner = store.take_returned_snapshot_read_retirement().unwrap();
        }
        if let Some(active) = owner.as_mut() {
            match active.close_step(1, bytes).unwrap() {
                SnapshotRetirementStep::Complete => {
                    assert!(active.terminal_is_empty());
                    *owner = None;
                }
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
                SnapshotRetirementStep::Blocked => {}
            }
        }
    }
}

/// 🧹️ Drains the Store-side reclamation of the leases a finished query already handed back. Each
/// registry hands back one returned slot per probe, so this is the caller's own cooperative
/// maintenance and deliberately runs AFTER the query's terminal reply, never before it.
fn reclaim_returned_leases(stores: &mut [TestStore; 3], bytes: usize) {
    let mut owners = [None, None, None];
    for _ in 0..1_000_000 {
        pump(stores, &mut owners, bytes);
        if owners.iter().all(Option::is_none) && stores.iter().all(TestStore::snapshot_read_leases_terminal_is_empty) {
            return;
        }
    }
    panic!("the capture Stores never reclaimed their exact returned read leases");
}

fn finish_close(stores: &mut [TestStore; 3], query: &mut Query, bytes: usize) -> LocalInteractionQueryReply {
    for _ in 0..1_000_000 {
        let step = query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
        if step == LocalInteractionLiveStep::Complete {
            assert!(query.owners_are_empty());
        }
        assert!(query.take_reply_admitted(|_| false).is_none());
        assert!(!query.terminal_is_empty());
        if let Some(reply) = query.take_reply() {
            assert!(query.terminal_is_empty());
            reclaim_returned_leases(stores, bytes);
            return reply;
        }
    }
    panic!("live query never returned all three exact roots");
}

fn close_stores(stores: &mut [TestStore; 3], bytes: usize) {
    for store in stores {
        for _ in 0..1_000_000 {
            if store.close_owned_step(1, bytes).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(store.close_owned_terminal_is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_live_pages_wait_exact_ack_and_all_three_roots() {
    for bytes in [1, 64, 4096] {
        let mut stores = stores().await;
        let mut query = query(&stores, 41);
        assert!(query.take_reply_admitted(|_| false).is_none());
        let LocalInteractionQueryReply::Started { token } = query.take_reply().unwrap() else { panic!("start token missing") };
        assert!(query.take_reply().is_none());
        let mut output = Vec::new();
        let mut emitted = 0;
        for _ in 0..200_000 {
            if let LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, released_items } = query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
                assert!(emitted_bytes + retired_bytes <= bytes && released_items <= 1);
                emitted += emitted_bytes;
            }
            assert!(query.take_reply_admitted(|_| false).is_none());
            let Some(reply) = query.take_reply() else { continue };
            let LocalInteractionQueryReply::Page { page } = reply else { panic!("page missing") };
            assert!(!query.has_pending_work());
            assert!(query.take_reply().is_none());
            let ack = LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            let mut stale = ack.clone();
            stale.query_generation -= 1;
            assert!(!query.acknowledge(&stale));
            assert!(!query.cancel_authorized(&stale));
            output.extend_from_slice(&page.bytes);
            assert!(query.acknowledge(&ack));
            if page.terminal {
                break;
            }
        }
        let captured: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&output).unwrap()).unwrap();
        assert_eq!(captured.identity, token.identity);
        assert!(output.len() > 4096);
        assert_eq!(emitted, output.len());
        assert!(!query.owners_are_empty());
        assert!(matches!(finish_close(&mut stores, &mut query, bytes), LocalInteractionQueryReply::Closed { cancelled: false, .. }));
        close_stores(&mut stores, bytes);
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_live_reopened_request_rejects_old_started_cancel() {
    let generations = LocalInteractionQueryGeneration::default();
    let mut stores = stores().await;
    let mut first = query(&stores, generations.next().unwrap());
    let LocalInteractionQueryReply::Started { token: old } = first.take_reply().unwrap() else { panic!("start") };
    assert!(first.cancel_authorized(&old));
    assert!(matches!(finish_close(&mut stores, &mut first, 1), LocalInteractionQueryReply::Closed { cancelled: true, .. }));
    drop(first);
    let mut second = query(&stores, generations.next().unwrap());
    assert!(!second.cancel_authorized(&old));
    let LocalInteractionQueryReply::Started { token: fresh } = second.take_reply().unwrap() else { panic!("fresh start") };
    assert_ne!(old.query_generation, fresh.query_generation);
    assert!(second.cancel_authorized(&fresh));
    finish_close(&mut stores, &mut second, 4096);
    close_stores(&mut stores, 4096);
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_live_partial_admission_retains_successful_roots() {
    let mut stores = stores().await;
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: 0, revision: [1; 32], document_revision: stores[0].content_revision_now(), topology_revision: [3; 32] };
    let mut query = Query::new(13, 41, identity, Some(stores[0].snapshot_read().unwrap()), None, Some(stores[2].snapshot_read().unwrap()));
    assert!(query.take_reply().is_none());
    assert_eq!(query.advance(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 }).unwrap(), LocalInteractionLiveStep::Blocked);
    assert!(matches!(finish_close(&mut stores, &mut query, 1), LocalInteractionQueryReply::Rejected { code: LocalInteractionQueryRejection::SourceFailed, .. }));
    close_stores(&mut stores, 4096);
}

//#region 🎛️CoordinatorRoundRobin
/// 🚧️ Runaway guard for the modelled host loop, far above the host's own 4096-continuation budget
/// so a failure here is the state machine's, never the guard's.
const COORDINATOR_TURNS: usize = 1_000_000;

/// 🎯️ Runnable turns the terminal acknowledgement may still cost before `Closed` is published.
/// Deliberately tiny and independent of both the byte grant and the Store registry's fixed
/// capacity: closing hands three captured roots back and nothing else. The host's own settle drain
/// gives up after 4096 more-work continuations, which is what a capacity-bounded reclamation
/// cursor inside this path spent on an unchanging closing state during the browser boot.
const CLOSE_TURN_BUDGET: usize = 32;

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: the whole read, driven exactly as the
/// coordinator drives it and with NO Store reclamation whatsoever, because the Store's own
/// one-slot-per-step cleanup cursor is bounded by the registry's fixed capacity and no host
/// continuation budget can contain it. Every page acknowledgement is delivered ONLY at the
/// quiescence point the host's settle loop can actually reach — `has_pending_work() == false` —
/// since a query that stays runnable while the sole remaining step belongs to the host never sees
/// its own ACK. This is the browser boot's stall, which spent 4096 continuations reporting
/// more-work over an unchanging closing state.
#[semio_framework_async_macros::async_test]
async fn local_interaction_live_terminates_without_any_store_reclamation() {
    for bytes in [1, 64, 4096] {
        let mut stores = stores().await;
        let mut query = query(&stores, 41);
        let mut awaiting_ack: Option<(LocalInteractionQueryToken, bool)> = None;
        let (mut capture, mut closed, mut close_turns, mut closing) = (Vec::new(), false, 0_usize, false);
        for _ in 0..COORDINATOR_TURNS {
            if !query.has_pending_work() {
                let Some((token, terminal)) = awaiting_ack.take() else { break };
                assert!(query.acknowledge(&token), "the page this query itself published accepts its own exact token");
                closing |= terminal;
                continue;
            }
            close_turns += usize::from(closing);
            match query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
                LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, released_items } => assert!(emitted_bytes + retired_bytes <= bytes && released_items <= 1),
                LocalInteractionLiveStep::Complete => assert!(query.owners_are_empty()),
                LocalInteractionLiveStep::Blocked => {}
            }
            if !query.reply_ready() {
                continue;
            }
            match query.take_reply() {
                Some(LocalInteractionQueryReply::Started { .. }) | None => {}
                Some(LocalInteractionQueryReply::Page { page }) => {
                    assert!(awaiting_ack.is_none(), "a second page was published while the first still awaited its acknowledgement");
                    capture.extend_from_slice(&page.bytes);
                    awaiting_ack = Some((LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal }, page.terminal));
                }
                Some(LocalInteractionQueryReply::Closed { cancelled, .. }) => {
                    assert!(!cancelled);
                    closed = true;
                    break;
                }
                Some(other) => panic!("unexpected local interaction reply: {other:?}"),
            }
        }
        assert!(closed, "the read never reached Closed without Store reclamation at bytes={bytes}");
        assert!(query.terminal_is_empty(), "a published Closed leaves the slot terminal-empty at bytes={bytes}");
        assert!(close_turns <= CLOSE_TURN_BUDGET, "closing spent {close_turns} runnable turns at bytes={bytes}, over the {CLOSE_TURN_BUDGET}-turn budget its three exact root handbacks cost");
        let decoded: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&capture).unwrap()).unwrap();
        assert!(!decoded.state.selection.is_empty(), "this fixture's capture carries a live selection");
        reclaim_returned_leases(&mut stores, bytes);
        close_stores(&mut stores, bytes);
    }
}
//#endregion 🎛️CoordinatorRoundRobin

/// 🫙️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: a capture with nothing to say still completes
/// the whole protocol — Started, one terminal page of length 0, its exact ACK, retirement and
/// `Closed` — and never leaves the shell draining an actor that reports more-work forever.
#[test]
fn local_interaction_live_empty_capture_reaches_its_terminal_closed_reply() {
    let inputs = LocalInteractionInputReads::<(), ()>::from_optional(None, None);
    let mut query = LocalInteractionLiveQuery {
        owned: ManuallyDrop::new(LiveState { query: Some(LocalInteractionQuery::new(crate::local_interaction::query::tests::empty_capture_for_live_law(), 3, 8)), inputs, error_bytes: None }),
        request_id: 3,
        started: false,
        page_sent: false,
        closing: false,
        cancelled: false,
        failed: false,
        terminal_sent: false,
    };
    let LocalInteractionQueryReply::Started { .. } = query.take_reply().expect("an empty capture still starts") else { panic!("start") };
    let mut awaiting_ack = None;
    let mut closed = false;
    for _ in 0..64 {
        if !query.has_pending_work() {
            let Some(token) = awaiting_ack.take() else { break };
            assert!(query.acknowledge(&token));
            continue;
        }
        query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap();
        match query.take_reply() {
            Some(LocalInteractionQueryReply::Page { page }) => {
                assert!(page.terminal && page.bytes.is_empty(), "an empty capture publishes exactly one empty terminal page");
                awaiting_ack = Some(LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity, ordinal: page.ordinal });
            }
            Some(LocalInteractionQueryReply::Closed { cancelled: false, .. }) => {
                closed = true;
                break;
            }
            Some(other) => panic!("unexpected empty-capture reply: {other:?}"),
            None => {}
        }
    }
    assert!(closed, "an empty capture reaches its terminal Closed reply");
    assert!(query.terminal_is_empty());
}

#[test]
fn local_interaction_runtime_query_generation_exhausts_before_slot_admission() {
    let generations = LocalInteractionQueryGeneration(std::cell::Cell::new(u64::MAX - 1));
    assert_eq!(generations.next(), Some(u64::MAX));
    assert_eq!(generations.next(), None);
    assert_eq!(generations.0.get(), u64::MAX);
}

#[test]
fn local_interaction_live_partial_error_preserves_wrapper_emission_and_retirement_counts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🔣️.json")).unwrap();
    for bytes in [1, 64, 4096] {
        let source = crate::local_interaction::query::tests::hostile_capture_for_live_law();
        let inputs = LocalInteractionInputReads::<(), ()>::from_optional(None, None);
        let mut query = LocalInteractionLiveQuery {
            owned: ManuallyDrop::new(LiveState { query: Some(LocalInteractionQuery::new(source, 13, 41)), inputs, error_bytes: None }),
            request_id: 13,
            started: false,
            page_sent: false,
            closing: false,
            cancelled: false,
            failed: false,
            terminal_sent: false,
        };
        assert!(matches!(query.take_reply(), Some(LocalInteractionQueryReply::Started { .. })));
        let mut emitted = 0;
        let mut retired = 0;
        for _ in 0..20_000 {
            match query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
                LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, released_items } => {
                    assert!(emitted_bytes + retired_bytes <= bytes && released_items <= 1);
                    emitted += emitted_bytes;
                    retired += retired_bytes;
                }
                LocalInteractionLiveStep::Complete => assert!(query.owners_are_empty()),
                LocalInteractionLiveStep::Blocked => {}
            }
            if let Some(reply) = query.take_reply() {
                match reply {
                    LocalInteractionQueryReply::Page { page } => {
                        assert!(query.acknowledge(&LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity, ordinal: page.ordinal }));
                    }
                    LocalInteractionQueryReply::Rejected { code: LocalInteractionQueryRejection::SourceFailed, .. } => break,
                    _ => panic!("partial error must never publish a successful terminal page"),
                }
            }
        }
        assert!(query.terminal_is_empty());
        let expected = fixture["partialError"]["expectedPrefix"].as_str().unwrap().len();
        assert_eq!(emitted, expected);
        assert_eq!(retired, expected + fixture["partialError"]["first"].as_str().unwrap().len() + fixture["partialError"]["error"].as_str().unwrap().len());
    }
}
