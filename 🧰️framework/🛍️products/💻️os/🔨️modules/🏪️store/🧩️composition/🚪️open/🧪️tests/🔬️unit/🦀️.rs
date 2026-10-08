use super::*;
use crate::os_store::{OWNED_SCHEMA_DECODE_PAGE_BYTES, OwnedSchemaDecodeCredits};

fn request_for(bytes: &[u8]) -> MemberOpenRequest {
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len().max(1) }).unwrap();
    for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let expected = ArtifactRef { artifact_id: "member".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
    MemberOpenRequest::new(OperationId(1), Generation(1), 1000, expected, None, pages, crate::os_spr::ActorId("actor:member-opening-fixture".into())).admit(1).unwrap_or_else(|_| panic!("admissible test input"))
}

fn retire_request(request: &mut MemberOpenRequest) {
    for _ in 0..100_000 {
        let grant = request.next_close_byte_demand().max(7);
        assert!(grant <= retirement_admission());
        match request.close_step(1, grant).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= grant),
            SnapshotRetirementStep::Complete => {
                assert!(request.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Blocked => panic!("inline pages have no shared owner"),
        }
    }
    panic!("request retirement did not converge");
}

pub(super) fn retirement_admission() -> usize {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    fixture["admissionBytes"].as_u64().unwrap() as usize
}

pub(super) fn next_request_frame_bytes(request: &MemberOpenRequest) -> usize {
    if let Some(pages) = request.pages.as_ref().filter(|pages| pages.terminal_is_empty()) { return pages.allocation_byte_demand(); }
    0
}

pub(super) fn request_logical_retained_bytes(request: &MemberOpenRequest) -> usize {
    let expected = request.expected.as_ref().map_or(0, |value| value.artifact_id.len() + value.dialect.artifact_kind.len() + value.dialect.standard.len() + value.dialect.subset.len());
    let owner = request.owner.as_ref().map_or(0, |value| value.parent.artifact_id.len() + value.parent.dialect.artifact_kind.len() + value.parent.dialect.standard.len() + value.parent.dialect.subset.len() + value.slot.len() + value.child_id.len());
    request.retained_input_bytes() + request.actor.0.len() + expected + owner
}

fn request_identity_capacity(request: &MemberOpenRequest) -> usize {
    let expected = request.expected.as_ref().map_or(0, |value| value.artifact_id.capacity() + value.dialect.artifact_kind.capacity() + value.dialect.standard.capacity() + value.dialect.subset.capacity());
    let owner = request.owner.as_ref().map_or(0, |value| value.parent.artifact_id.capacity() + value.parent.dialect.artifact_kind.capacity() + value.parent.dialect.standard.capacity() + value.parent.dialect.subset.capacity() + value.slot.capacity() + value.child_id.capacity());
    request.actor.0.capacity() + expected + owner
}

#[test]
fn member_open_input_framing_is_canonical_scoped_and_budgeted() {
    use semio_framework_job::{StepBudget, root_cancel_token};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["framing"].as_array().unwrap() {
        let bytes: Vec<u8> = serde_json::from_value(row["bytes"].clone()).unwrap();
        for fuel in [1, 2, 13] {
            let mut request = request_for(&bytes);
            let mut sequence = 0;
            let cancel = root_cancel_token();
            let mut zero = StepContext::new(OperationId(1), Generation(1), StepBudget::new(0, 999), cancel.clone(), || Some(1), &mut sequence);
            assert!(matches!(request.step_input(&mut zero), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
            let mut outcome = None;
            for _ in 0..16 {
                let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, 999), cancel.clone(), || Some(1), &mut sequence);
                let before = request.input_offset;
                let step = request.step_input(&mut cx);
                assert!(request.input_offset - before <= fuel as usize);
                if !matches!(step, MemberOpenInputStep::Pending(_)) {
                    outcome = Some(step);
                    break;
                }
            }
            match outcome.expect("bounded varint is terminal") {
                MemberOpenInputStep::Framed(frame) => {
                    assert!(row["reason"].is_null(), "{}", row["id"]);
                    assert_eq!(serde_json::json!(frame.snapshot_range()), row["snapshot"]);
                    assert_eq!(serde_json::json!(frame.history_range()), row["history"]);
                    let mut output = [0xcc; 8];
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), cancel.clone(), || Some(1), &mut sequence);
                    assert_eq!(request.copy_snapshot_chunk(0, &mut output, &mut cx).unwrap(), 1);
                    assert_eq!(output[0], bytes[frame.snapshot_start]);
                    assert_eq!(&output[1..], &[0xcc; 7]);
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999), cancel.clone(), || Some(1), &mut sequence);
                    let count = request.copy_history_chunk(0, &mut output, &mut cx).unwrap();
                    assert_eq!(&output[..count], &bytes[frame.history_start..frame.history_start + count]);
                }
                MemberOpenInputStep::Rejected(reason) => {
                    assert_eq!(reason, MemberOpenDiagnostic::Malformed);
                    assert_eq!(row["reason"], "malformed");
                }
                MemberOpenInputStep::Pending(_) => unreachable!(),
            }
            assert_eq!(request.retained_input_bytes(), bytes.len());
            retire_request(&mut request);
        }
    }
    for (operation, generation, now, cancelled, expected) in
        [(2, 1, 1, false, MemberOpenDiagnostic::Stale), (1, 2, 1, false, MemberOpenDiagnostic::Stale), (1, 1, 1000, false, MemberOpenDiagnostic::Expired), (1, 1, 1, true, MemberOpenDiagnostic::Cancelled)]
    {
        let mut request = request_for(&[1, 97, 83]);
        let mut sequence = 0;
        let cancel = root_cancel_token();
        if cancelled {
            cancel.cancel_now();
        }
        let clock: fn() -> Option<u64> = if now == 1000 { || Some(1000) } else { || Some(1) };
        let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(8, 2000), cancel, clock, &mut sequence);
        assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
        assert_eq!(request.input_offset, 0);
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999), root_cancel_token(), || Some(1), &mut sequence);
        assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
        assert_eq!(request.retained_input_bytes(), 3);
        retire_request(&mut request);
    }
    let mut request = request_for(&[1, 97, 83]);
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 1), root_cancel_token(), || Some(1), &mut sequence);
    assert!(matches!(request.step_input(&mut cx), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
    retire_request(&mut request);
}

#[test]
fn member_open_request_rejection_retains_exact_pages_and_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let bytes = row["bytes"].as_u64().unwrap() as usize;
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.max(1) }).unwrap();
        for chunk in vec![0x63; bytes].chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
            pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
        }
        if row["sealed"].as_bool().unwrap() {
            pages.seal().unwrap();
        }
        let expected = ArtifactRef { artifact_id: row["artifactId"].as_str().unwrap().into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
        let owner = row["ownerChildId"].as_str().map(|child_id| OwnerRef { parent: ArtifactRef { artifact_id: "parent".into(), dialect: expected.dialect.clone() }, slot: "content".into(), child_id: child_id.into() });
        let request = MemberOpenRequest::new(OperationId(1), Generation(1), row["expiresAtUs"].as_u64().unwrap(), expected.clone(), owner.clone(), pages, crate::os_spr::ActorId(row["openedActor"].as_str().unwrap().into()));
        let result = request.admit(row["nowUs"].as_u64().unwrap());
        assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
        let mut request = match result {
            Ok(request) => request,
            Err(rejected) => {
                let reason = match rejected.diagnostic {
                    MemberOpenDiagnostic::Unsealed => "unsealed",
                    MemberOpenDiagnostic::Empty => "empty",
                    MemberOpenDiagnostic::Expired => "expired",
                    MemberOpenDiagnostic::Identity => "identity",
                    MemberOpenDiagnostic::Owner => "owner",
                    _ => panic!("unexpected admission reason"),
                };
                assert_eq!(reason, row["reason"].as_str().unwrap());
                rejected.request
            }
        };
        assert_eq!(request.expected(), &expected);
        assert_eq!(serde_json::to_value(request.actor()).unwrap(), row["openedActor"]);
        assert_eq!(request.owner(), owner.as_ref());
        assert_eq!(request.retained_input_bytes(), bytes);
        assert!(matches!(request.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert_eq!(request.retained_input_bytes(), bytes);
        let identity_bytes = row["openedActor"].as_str().unwrap().len() + expected.artifact_id.len()
            + expected.dialect.artifact_kind.len()
            + expected.dialect.standard.len()
            + expected.dialect.subset.len()
            + owner.as_ref().map_or(0, |owner| owner.parent.artifact_id.len() + owner.parent.dialect.artifact_kind.len() + owner.parent.dialect.standard.len() + owner.parent.dialect.subset.len() + owner.slot.len() + owner.child_id.len());
        let mut released = 0;
        let mut frames = 0;
        let mut logical = 0;
        let identity_capacity = request_identity_capacity(&request);
        for _ in 0..100_000 {
            let frame = next_request_frame_bytes(&request);
            let demand = request.next_close_byte_demand();
            if demand > 1 {
                if frame != 0 { assert_eq!(demand, frame); }
                let identity_pointer = request.identity_string_mut().map(|field| field.as_ptr());
                let pages_pointer = request.pages.as_ref().map(|pages| pages as *const OwnedSchemaDecodePages);
                let before = request_logical_retained_bytes(&request);
                let (denied, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, demand - 1).unwrap());
                assert_eq!(denied, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(request_logical_retained_bytes(&request), before);
                if let Some(pointer) = identity_pointer { assert_eq!(pointer, request.identity_string_mut().unwrap().as_ptr()); }
                if let Some(pointer) = pages_pointer { assert!(std::ptr::eq(pointer, request.pages.as_ref().unwrap() as *const OwnedSchemaDecodePages)); }
                assert_eq!(request.next_close_byte_demand(), demand);
            }
            let grant = demand.max(7);
            assert!(grant <= retirement_admission());
            let before = request_logical_retained_bytes(&request);
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, grant).unwrap());
            logical += before - request_logical_retained_bytes(&request);
            assert_eq!(events.requested_bytes, 0);
            match step {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= grant);
                    assert_eq!(events.released_bytes, released_bytes);
                    if frame != 0 {
                        assert_eq!((released_items, released_bytes), (1, frame));
                        frames += frame;
                    }
                    released += released_bytes;
                }
                SnapshotRetirementStep::Complete => { assert_eq!(events.released_bytes, 0); break; }
                SnapshotRetirementStep::Blocked => panic!("request owns no shared root"),
            }
        }
        assert!(request.terminal_is_empty());
        assert_eq!(logical, bytes + identity_bytes, "{}: exact original logical input and identity ownership", row["id"]);
        assert_eq!(released - frames, identity_capacity, "{}: exact original physical identity capacity", row["id"]);
        println!("[DEBUG] member-open request={} logical-input-plus-identity={logical} whole-page-backing={frames} physical-identity={identity_capacity} paid-physical-release={released} whole-allocation-denial-retained=true", row["id"]);
    }
}

#[test]
fn member_open_request_every_report_is_same_turn_physical_release_without_new_close_allocations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut mismatches = 0;
    let mut births = 0;
    for row in fixture["framing"].as_array().unwrap() {
        let bytes: Vec<u8> = serde_json::from_value(row["bytes"].clone()).unwrap();
        let mut request = request_for(&bytes);
        let mut reported = 0;
        let mut physical = 0;
        for _ in 0..100_000 {
            let grant = request.next_close_byte_demand().max(7);
            assert!(grant <= retirement_admission());
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, grant).unwrap());
            births += events.requested_bytes;
            physical += events.released_bytes;
            let released = match step {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= grant); released_bytes },
                SnapshotRetirementStep::Complete => { assert!(request.terminal_is_empty()); 0 },
                SnapshotRetirementStep::Blocked => panic!("unshared exact request cannot block"),
            };
            reported += released;
            if events.requested_bytes != 0 || events.released_bytes != released {
                mismatches += 1;
                if mismatches <= 8 { println!("[DEBUG] request allocator mismatch case={} grant={grant} born={} physical-release={} reported-release={released}", row["id"], events.requested_bytes, events.released_bytes); }
            }
            if request.terminal_is_empty() { break; }
        }
        assert!(request.terminal_is_empty());
        println!("[DEBUG] request complete physical census case={} inline-bytes={} actual-release={physical} reported-release={reported}", row["id"], bytes.len());
    }
    assert_eq!(births, 0, "closing retained original identities must not birth unadmitted retirement boxes");
    assert_eq!(mismatches, 0, "inline payload work cannot masquerade as same-turn physical allocation release");
}

#[test]
fn member_open_request_original_identity_capacity_is_indivisible_even_when_empty() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📏️retirement/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let capacity = row["capacityBytes"].as_u64().unwrap() as usize;
        let mut request = request_for(&[1, 97, 83]);
        let mut actor = String::with_capacity(capacity);
        actor.push_str(row["text"].as_str().unwrap());
        request.actor.0 = actor;
        assert_eq!(request.actor.0.capacity(), capacity);
        while request.pages.is_some() {
            let grant = request.next_close_byte_demand().max(7);
            assert!(grant <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, grant).unwrap());
            let SnapshotRetirementStep::Pending { released_bytes, .. } = step else { panic!("page retirement cannot complete the original request") };
            assert_eq!((events.requested_bytes, events.released_bytes), (0, released_bytes));
        }
        let pointer = request.actor.0.as_ptr();
        let text_bytes = request.actor.0.len();
        let (demand, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.next_close_byte_demand());
        assert_eq!(demand, capacity);
        assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
        for (items, grant) in [(0, capacity), (1, capacity - 1)] {
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(items, grant).unwrap());
            assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(request.actor.0.as_ptr(), pointer);
            assert_eq!(request.actor.0.len(), text_bytes);
            assert_eq!(request.next_close_byte_demand(), capacity);
        }
        let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, capacity).unwrap());
        assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: capacity });
        assert_eq!((events.requested_bytes, events.released_bytes), (0, capacity));
        assert_eq!(request.actor.0.capacity(), 0);
        retire_request(&mut request);
        println!("[DEBUG] member-open original identity={} logical-bytes={text_bytes} physical-capacity={capacity} whole-denial-retained=true exact-paid-release=true", row["id"]);
    }
}

#[test]
fn member_open_request_inline_pages_retire_one_item_without_physical_payload_credit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📏️retirement/🧫️fixtures/📄️inline.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let length = row["inputBytes"].as_u64().unwrap() as usize;
        let bytes = vec![0x63; length];
        let mut request = request_for(&bytes);
        let page_count = length.div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES);
        let backing = request.pages.as_ref().unwrap().allocation_byte_demand();
        assert_eq!(backing, page_count * fixture["slotBytes"].as_u64().unwrap() as usize);
        for _ in 0..page_count {
            let before = request.retained_input_bytes();
            let (demand, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.next_close_byte_demand());
            assert_eq!(demand, 0);
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            let (denied, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(0, backing).unwrap());
            assert_eq!(denied, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(request.retained_input_bytes(), before);
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, 0).unwrap());
            assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            let tail = before % OWNED_SCHEMA_DECODE_PAGE_BYTES;
            assert_eq!(before - request.retained_input_bytes(), if tail == 0 { OWNED_SCHEMA_DECODE_PAGE_BYTES } else { tail });
            assert_eq!(request.pages.as_ref().unwrap().allocation_byte_demand(), backing);
        }
        assert_eq!(request.retained_input_bytes(), 0);
        assert_eq!(request.next_close_byte_demand(), backing);
        for (items, grant) in [(0, backing), (1, 0), (1, backing - 1)] {
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(items, grant).unwrap());
            assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(request.next_close_byte_demand(), backing);
        }
        assert!(backing <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(1, backing).unwrap());
        assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: backing });
        assert_eq!((events.requested_bytes, events.released_bytes), (0, backing));
        retire_request(&mut request);
        println!("[DEBUG] inline request={} input={length} page-items={page_count} payload-physical-free=0 whole-backing={backing} zero-item-and-one-below-retained=true", row["id"]);
    }
}
