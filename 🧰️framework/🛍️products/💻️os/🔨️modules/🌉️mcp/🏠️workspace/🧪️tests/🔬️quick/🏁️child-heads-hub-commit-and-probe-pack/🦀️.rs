/// 🏁️ Child heads, hub commit and probe pack: composed child heads as inference dependencies, the guest JSON decoder round trip, the isolated compile worker, the hub commit status and the cancelled owners of the original probe pack.

/// 🪆️ LAW (design §20.15, W-b): the `ChildHeads` a session guest answers become the inference request's dependencies one per owned
/// child, keyed `child:<slot>/<childId>` — the exact key the guest's request validation parses back — with the child's head pack.
#[test]
fn a_composed_documents_child_heads_become_child_keyed_inference_dependencies() {
    let entries = vec![
        store::ChildHeadPackEntry { slot: "content".into(), child_id: "flow-1".into(), dialect: "s.flow@1/*".into(), head_pack: vec![1, 2, 3], owner: String::new() },
        store::ChildHeadPackEntry { slot: "content".into(), child_id: "flow-2".into(), dialect: "s.flow@1/*".into(), head_pack: vec![4], owner: String::new() },
    ];
    let dependencies = child_head_dependencies(entries);
    assert_eq!(dependencies, vec![("child:content/flow-1".to_string(), vec![1, 2, 3]), ("child:content/flow-2".to_string(), vec![4])]);
    assert!(dependencies.iter().all(|(key, _)| semio_framework_plugin::inference_child_dependency_parts(key).is_some()), "the guest parses every key back");
    assert!(child_head_dependencies(Vec::new()).is_empty(), "a document without children sends no dependencies");
}

/// 🔡️ The bound document survives the GUEST's own JSON decoder, byte for byte. The gateway writes
/// the pair with `serde_json`; the guest reads it with the kernel's DSL JSON parser
/// (`protocol::json::from_json_str`) and then base64-decodes it. Two different parsers on one
/// string is exactly the seam where a document silently becomes garbage — and a garbage pack does
/// not fail cleanly, it traps the guest inside the pack inflater (measured 2026-09-22 21:1x).
#[test]
fn a_bound_document_round_trips_through_the_guest_json_decoder() {
    let pack: Vec<u8> = (0..=255u8).cycle().take(405).collect();
    let spr: Vec<u8> = (0..=255u8).rev().cycle().take(211).collect();
    let declared = bound_inference_row(true, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64);
    let command = bound_inference_command(Some(ArtifactDocumentBinding { pack: pack.clone(), spr: spr.clone() }), b"{}");
    let bound = bind_inference_document(&declared, &command).expect("a named artifact binds");
    let text = std::str::from_utf8(&bound).expect("the bound body is UTF-8");
    let value: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the guest's own JSON decoder reads what serde_json wrote");
    let document = value.get("document").expect("the declared field survives the decoder");
    let pack_text = document.get("pack").and_then(semio_framework_value::DslValue::as_str).expect("pack is a string");
    let spr_text = document.get("spr").and_then(semio_framework_value::DslValue::as_str).expect("spr is a string");
    assert_eq!(decode_base64(pack_text), Some(pack), "the guest decodes the EXACT pack bytes the host bound");
    assert_eq!(decode_base64(spr_text), Some(spr), "…and the exact spr bytes");
}

/// 🧊️ The isolated compile worker of a unit-test process (see [`ISOLATED_COMPILE_TEST_ENTRY`]): a
/// no-op in an ordinary test run, the whole worker when its two inputs are set.
#[test]
fn isolated_compile_worker_entry() {
    use std::io::Read as _;
    let (Some(engine), Some(out)) = (std::env::var_os(ISOLATED_COMPILE_TEST_ENGINE), std::env::var_os(ISOLATED_COMPILE_TEST_OUT)) else { return };
    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes).expect("component on stdin");
    let result = semio_framework_plugin_host::compile_component_isolated(&engine.to_string_lossy(), &bytes, Path::new(&out));
    if let Err(error) = &result {
        eprintln!("{error}");
    }
    std::process::exit(i32::from(result.is_err()));
}

/// 📮️ A hub commit is acknowledged only by a status the document actor reports AFTER the relay: a live
/// link, nothing pending and an acknowledged head. A stale acknowledgement from before the relay, a
/// link in backoff and a link that expired all answer `acknowledged: false` with the link state and
/// the last coded fault, so `action_invoke` reverts the edit and answers `hub.relay-unacknowledged`, never an unqualified success.
/// Measured 2026-09-26 on hub 7800: 1 of 3 agent commits answered SUCCEEDED while the hub head never moved.
#[test]
fn a_hub_commit_is_acknowledged_only_by_a_live_status_reported_after_its_relay() {
    let head = semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1 { document_id: "doc".to_string(), head_edit_ordinal: 1, head_edit_id: "edit".to_string(), last_commit_seq: 1, chain_sha256: "0".repeat(64) };
    let live = |pending: usize, acknowledged: bool| store::sync::ArtifactSyncStatus { persisted: true, pending_mutations: pending, remote: store::sync::RemoteState::Live { peer_count: 1 }, acknowledged_head: acknowledged.then(|| head.clone()) };
    let relay = HubRelay::default();
    relay.record(Some(live(0, true)), None);
    let since = relay.version();
    let stale = relay.await_acknowledged(since, 20);
    assert!(!stale.acknowledged, "an acknowledgement from before the relay does not cover it: {stale:?}");
    relay.record(Some(live(1, false)), None);
    assert!(!relay.await_acknowledged(since, 20).acknowledged, "a pending mutation is not acknowledged");
    relay.record(Some(store::sync::ArtifactSyncStatus { persisted: true, pending_mutations: 1, remote: store::sync::RemoteState::Backoff { retry_in_ms: 4_000 }, acknowledged_head: None }), Some("document.link.expired: the link to the hub expired".to_string()));
    let backoff = relay.await_acknowledged(since, 20);
    assert!(!backoff.acknowledged && backoff.detail.contains("backoff") && backoff.detail.contains("document.link.expired"), "{backoff:?}");
    let waiter = std::thread::scope(|scope| {
        let handle = scope.spawn(|| relay.await_acknowledged(since, 5_000));
        std::thread::sleep(std::time::Duration::from_millis(50));
        relay.record(Some(live(0, true)), None);
        handle.join().expect("waiter")
    });
    assert!(waiter.acknowledged, "a later live, fully acknowledged status wakes the waiting commit: {waiter:?}");
    assert_eq!(relay.report()["remote"], "live");
    assert_eq!(relay.report()["lastFault"], "document.link.expired: the link to the hub expired");
}

#[test]
fn original_probe_pack_receiving_retains_every_cancelled_owner_and_exact_allocations(){
 use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,retirement::ControlledRetirement};
 use std::cell::Cell;
 let vectors:serde_json::Value=serde_json::from_str(fixture_text("../../../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🧫️fixtures/📦️receiving.json")).unwrap();
 let policy=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
 let mut successes=0;
 for row in vectors["cases"].as_array().unwrap(){for cutoff in (0..=16).chain(std::iter::once(usize::MAX)){
  let input=row["text"].as_str().unwrap().as_bytes();let pointer=input.as_ptr();let allowance=Cell::new(cutoff);let events=Cell::new(0);
  let mut original=|_|{let next=events.get()+1;events.set(next);next<=allowance.get()};
  let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut original);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,policy);
  let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||<ProbeSnapshot as store::ArtifactPackReceiving>::receive_pack(input,&mut owner));let accepted=owner.progress();assert!(accepted.fits(policy));assert_eq!(born,accepted.retained_capacity_bytes);assert_eq!(released,accepted.released_bytes);drop(owner);assert_eq!(input.as_ptr(),pointer);allowance.set(usize::MAX);
  let mut terminal_born=0;let mut terminal_released=0;
  if let Ok(snapshot)=result{successes+=1;assert_eq!(serde_json::to_value(&snapshot.0).unwrap(),row["expected"]);let mut retained=ControlledRetirement::new(snapshot).ok().unwrap();let(_,born,released)=semio_framework_trace_memory_testing::observe_backing(||retained.step(RetainedCloneGrant::default()).unwrap());assert_eq!((born,released),(0,0));for _ in 0..4096{if retained.terminal_is_empty(){break}let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||retained.step(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));terminal_born+=born;terminal_released+=released;}assert!(retained.terminal_is_empty());}
  for _ in 0..4096{if !native.has_retirement_owner(){break}let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||native.close_retirement_recipient(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));terminal_born+=born;terminal_released+=released;}assert!(!native.has_retirement_owner());assert_eq!(accepted.released_bytes+terminal_released,accepted.retained_capacity_bytes+terminal_born);
 }}
 assert!(successes>=vectors["cases"].as_array().unwrap().len());
 for axis in ["items","capacity","depth"]{let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"capacity"=>denied.maximum_capacity_bytes=0,_=>denied.maximum_depth=0};let mut yes=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,denied);let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||<ProbeSnapshot as store::ArtifactPackReceiving>::receive_pack(b"null",&mut owner));assert!(result.is_err());assert_eq!((born,released,owner.progress()),(0,0,RetainedCloneProgress::default()));drop(owner);assert!(!native.has_retirement_owner());}
}
