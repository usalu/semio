//! 🧪️ Live slots use real Store leases, full authority tokens, and bounded three-root retirement.

use super::*;
use crate::{app::InteractionConfigMutation, local_interaction::query::tests::install_original_interaction_owners};
use store::{ArtifactStore, ErasedSnapshotRetirement, SpaceMember};

type TestStore = ArtifactStore<protocol::InteractionState, InteractionConfigMutation>;
type Query = LocalInteractionLiveQuery<protocol::InteractionState, protocol::InteractionState>;
use crate::local_interaction::query::tests::original_receiving_recipient;

async fn stores() -> ([TestStore; 3],store::NativeSnapshotBodyWallet) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "semantic-unicode-over-page").unwrap();
    let mut state = row["expected"].clone();
    state["hover"] = serde_json::json!({});
    let state: protocol::InteractionState = serde_json::from_value(state).unwrap();
    let mut result = Vec::new();let mut recipient=original_receiving_recipient();
    for id in ["document", "config", "interaction"] {
        let envelope = store::create_document_envelope::<protocol::InteractionState, InteractionConfigMutation>("framework.interaction", id, state.clone(), None);
        let mut store = TestStore::new(envelope, protocol::ActorId(store::os_spr::LOCAL_ACTOR_ID.into())).await.unwrap();
        install_original_interaction_owners(&mut store,&mut recipient);
        result.push(store);
    }
    (result.try_into().ok().unwrap(),recipient)
}

fn query(stores: &[TestStore; 3], generation: u64) -> Query {
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: stores[2].generation_now(), revision: stores[2].content_revision_now(), document_revision: stores[0].content_revision_now(), topology_revision: [8; 32] };
    Query::new(13, generation, identity, Some(stores[0].snapshot_read().unwrap()), Some(stores[1].snapshot_read().unwrap()), Some(stores[2].snapshot_read().unwrap()))
}

fn advance_original<D,C,Q:LocalInteractionQueryCapture>(query:&mut LocalInteractionLiveQuery<D,C,Q>,recipient:&mut store::NativeSnapshotBodyWallet,width:usize)->LocalInteractionLiveStep{
    let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.advance(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),width).unwrap());
    let progress=match step{LocalInteractionLiveStep::Advanced{ownership,..}=>ownership,LocalInteractionLiveStep::Blocked=>RetainedCloneProgress::default()};
    recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));step
}
fn close_original<D,C,Q:LocalInteractionQueryCapture>(query:&mut LocalInteractionLiveQuery<D,C,Q>,recipient:&mut store::NativeSnapshotBodyWallet)->RetainedCloneStep{
    let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));step
}
fn pump(stores:&mut[TestStore;3],owners:&mut[Option<Box<dyn ErasedSnapshotRetirement>>;3],recipient:&mut store::NativeSnapshotBodyWallet){
    for(store,owner)in stores.iter_mut().zip(owners.iter_mut()){
        if owner.is_none(){let((received,progress),physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store.take_returned_snapshot_read_retirement(recipient.remaining_grant()).unwrap());recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));*owner=received;}
        if let Some(active)=owner.as_mut(){let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||active.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(active.terminal_is_empty());let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner.take()));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));}}
    }
}
fn reclaim_returned_leases(stores:&mut[TestStore;3],recipient:&mut store::NativeSnapshotBodyWallet){
    let mut owners=[None,None,None];for _ in 0..1_000_000{pump(stores,&mut owners,recipient);if owners.iter().all(Option::is_none)&&stores.iter().all(TestStore::snapshot_read_leases_terminal_is_empty){return}}
    panic!("the capture Stores never reclaimed their exact returned read leases");
}
fn finish_close(stores:&mut[TestStore;3],query:&mut Query,recipient:&mut store::NativeSnapshotBodyWallet)->LocalInteractionQueryReply{
    for _ in 0..1_000_000{let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(query.owners_are_empty());}assert!(query.take_reply_admitted(|_|false).is_none());assert!(!query.terminal_is_empty());if let Some(reply)=query.take_reply(){assert!(query.terminal_is_empty());reclaim_returned_leases(stores,recipient);return reply}}
    panic!("live query never returned all three exact roots");
}
fn close_stores(stores:&mut[TestStore;3],recipient:&mut store::NativeSnapshotBodyWallet){
    for store in stores{for _ in 0..1_000_000{let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store.close_owned_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break}}assert!(store.close_owned_terminal_is_empty())}
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_live_pages_wait_exact_ack_and_all_three_roots() {
    for bytes in [1, 64, 4096] {
        let(mut stores,mut recipient)=stores().await;
        let mut query = query(&stores, 41);
        assert!(query.take_reply_admitted(|_| false).is_none());
        let LocalInteractionQueryReply::Started { token } = query.take_reply().unwrap() else { panic!("start token missing") };
        assert!(query.take_reply().is_none());
        let mut output = Vec::new();
        let mut emitted = 0;
        for _ in 0..200_000 {
            if let LocalInteractionLiveStep::Advanced { emitted_bytes, retired_bytes, ownership } = advance_original(&mut query,&mut recipient,bytes) {
                assert!(emitted_bytes + retired_bytes <= bytes && ownership.copied_items <= 1);
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
        let captured: protocol::LocalInteractionCapture = semio_framework_pack_json::from_json_str(std::str::from_utf8(&output).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(captured.identity, token.identity);
        assert!(output.len() > 4096);
        assert_eq!(emitted, output.len());
        assert!(!query.owners_are_empty());
        assert!(matches!(finish_close(&mut stores,&mut query,&mut recipient), LocalInteractionQueryReply::Closed { cancelled: false, .. }));
        close_stores(&mut stores,&mut recipient);
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_live_reopened_request_rejects_old_started_cancel() {
    let generations = LocalInteractionQueryGeneration::default();
    let(mut stores,mut recipient)=stores().await;
    let mut first = query(&stores, generations.next().unwrap());
    let LocalInteractionQueryReply::Started { token: old } = first.take_reply().unwrap() else { panic!("start") };
    assert!(first.cancel_authorized(&old));
    assert!(matches!(finish_close(&mut stores,&mut first,&mut recipient), LocalInteractionQueryReply::Closed { cancelled: true, .. }));
    drop(first);
    let mut second = query(&stores, generations.next().unwrap());
    assert!(!second.cancel_authorized(&old));
    let LocalInteractionQueryReply::Started { token: fresh } = second.take_reply().unwrap() else { panic!("fresh start") };
    assert_ne!(old.query_generation, fresh.query_generation);
    assert!(second.cancel_authorized(&fresh));
    finish_close(&mut stores,&mut second,&mut recipient);
    close_stores(&mut stores,&mut recipient);
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_live_partial_admission_retains_successful_roots() {
    let(mut stores,mut recipient)=stores().await;
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: 0, revision: [1; 32], document_revision: stores[0].content_revision_now(), topology_revision: [3; 32] };
    let mut query = Query::new(13, 41, identity, Some(stores[0].snapshot_read().unwrap()), None, Some(stores[2].snapshot_read().unwrap()));
    assert!(query.take_reply().is_none());
    assert_eq!(query.advance(ArtifactStoreOneItemGrant::from_retained(RetainedCloneGrant{maximum_items:0,..recipient.remaining_grant()}),4096).unwrap(), LocalInteractionLiveStep::Blocked);
    assert!(matches!(finish_close(&mut stores,&mut query,&mut recipient), LocalInteractionQueryReply::Rejected { code: LocalInteractionQueryRejection::SourceFailed, .. }));
    close_stores(&mut stores,&mut recipient);
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
        let(mut stores,mut recipient)=stores().await;
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
            if query.is_closing(){close_original(&mut query,&mut recipient);}else{
                match advance_original(&mut query,&mut recipient,bytes){
                    LocalInteractionLiveStep::Advanced{emitted_bytes,retired_bytes,ownership}=>assert!(emitted_bytes+retired_bytes<=bytes&&ownership.copied_items<=1),
                    LocalInteractionLiveStep::Blocked=>{}
                }
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
        let decoded: protocol::LocalInteractionCapture = semio_framework_pack_json::from_json_str(std::str::from_utf8(&capture).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(!decoded.state.selection.is_empty(), "this fixture's capture carries a live selection");
        reclaim_returned_leases(&mut stores,&mut recipient);
        close_stores(&mut stores,&mut recipient);
    }
}
//#endregion 🎛️CoordinatorRoundRobin

/// 🫙️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: a capture with nothing to say still completes
/// the whole protocol — Started, one terminal page of length 0, its exact ACK, retirement and
/// `Closed` — and never leaves the shell draining an actor that reports more-work forever.
#[test]
fn local_interaction_live_empty_capture_reaches_its_terminal_closed_reply() {
    let inputs = LocalInteractionInputReads::<(), ()>::from_optional(None, None);let mut recipient=original_receiving_recipient();
    let mut query = LocalInteractionLiveQuery {
        owned: ManuallyDrop::new(LiveState { query: Some(LocalInteractionQuery::new(crate::local_interaction::query::tests::empty_capture_for_live_law(), 3, 8)), inputs, error: None, error_close:None }),
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
        if query.is_closing(){close_original(&mut query,&mut recipient);}else{advance_original(&mut query,&mut recipient,4096);}
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
        let inputs = LocalInteractionInputReads::<(), ()>::from_optional(None, None);let mut recipient=original_receiving_recipient();
        let mut query = LocalInteractionLiveQuery {
            owned: ManuallyDrop::new(LiveState { query: Some(LocalInteractionQuery::new(source, 13, 41)), inputs, error: None, error_close:None }),
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
        let mut closed = 0;
        for _ in 0..20_000 {
            if query.is_closing(){
                let before=query.owned.query.as_ref().unwrap().retired_bytes();
                close_original(&mut query,&mut recipient);
                retired+=(query.owned.query.as_ref().unwrap().retired_bytes()-before)as usize;
            }else{
                match advance_original(&mut query,&mut recipient,bytes){
                    LocalInteractionLiveStep::Advanced{emitted_bytes,retired_bytes,ownership}=>{
                        assert!(emitted_bytes+retired_bytes<=bytes&&ownership.copied_items<=1);emitted+=emitted_bytes;retired+=retired_bytes;
                    }
                    LocalInteractionLiveStep::Blocked=>{}
                }
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
        let payload = expected + fixture["partialError"]["first"].as_str().unwrap().len() + fixture["partialError"]["error"].as_str().unwrap().len();
        assert_eq!(retired + closed, payload, "[DEBUG] retired={retired} closed={closed} bytes={bytes}");
    }
}

#[semio_framework_async_macros::async_test]
async fn original_live_page_refusal_preserves_borrowed_page_before_any_heap_copy(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../📃️query/🧫️fixtures/📨️authority.json")).unwrap();
    let(mut stores,mut recipient)=stores().await;let mut query=query(&stores,41);assert!(matches!(query.take_reply(),Some(LocalInteractionQueryReply::Started{..})));
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){advance_original(&mut query,&mut recipient,64);if query.reply_ready(){break}}
    let page=query.owned.query.as_ref().unwrap().page().unwrap();assert!(!page.bytes.is_empty());let original=(page.bytes.as_ptr(),page.bytes.len(),page.token.clone());
    let(reply,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.take_reply_admitted(|_|false));let admitted=reply.is_some();
    let page=query.owned.query.as_ref().unwrap().page().unwrap();let preserved=(page.bytes.as_ptr(),page.bytes.len(),page.token.clone())==original;
    query.begin_close();finish_close(&mut stores,&mut query,&mut recipient);close_stores(&mut stores,&mut recipient);
    assert_eq!(serde_json::to_value([physical.requested_bytes,physical.released_bytes]).unwrap(),law["pagePublicationRefusal"]["physicalBytes"]);assert_eq!(serde_json::to_value(preserved).unwrap(),law["pagePublicationRefusal"]["originalPagePreserved"]);assert_eq!(serde_json::to_value(admitted).unwrap(),law["pagePublicationRefusal"]["replyAdmitted"]);
    eprintln!("[DEBUG] Original live page refused beforeheapcopy same borrowed page/token/pointer original capture/Store owners closed under same caller wallet independentSerde=true");
}
