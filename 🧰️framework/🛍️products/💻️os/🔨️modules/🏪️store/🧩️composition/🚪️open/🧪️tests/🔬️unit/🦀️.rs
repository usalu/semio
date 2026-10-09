use super::*;
use crate::os_store::{OWNED_SCHEMA_DECODE_PAGE_BYTES, OwnedSchemaDecodeCredits};

pub(super) fn request_for(bytes: &[u8]) -> MemberOpenRequest {
    let _ = request_close_policy();
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len().max(1) }).unwrap();
    for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let expected = ArtifactRef { artifact_id: "member".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
    MemberOpenRequest::new(OperationId(1), Generation(1), 1000, expected, None, pages, crate::os_spr::ActorId("actor:member-opening-fixture".into())).admit(1).unwrap_or_else(|_| panic!("admissible test input"))
}


pub(super) fn request_close_policy() -> RetainedCloneGrant {
    static POLICY: std::sync::LazyLock<RetainedCloneGrant>=std::sync::LazyLock::new(||{
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📏️retirement/🧫️fixtures/🔣️.json")).unwrap();
        serde_json::from_value(fixture["physicalCloseGrant"].clone()).unwrap()
    });
    *POLICY
}

fn retire_request(request: &mut MemberOpenRequest) {
    let policy = request_close_policy();
    for _ in 0..100_000 {
        let grant = request_close_policy().maximum_release_bytes;
        assert!(grant <= retirement_admission());
        match request.close_step(policy).unwrap() {
            RetainedCloneStep::Progress(progress) => assert!(progress.fits(policy)),
            RetainedCloneStep::Complete(_) => {
                assert!(request.terminal_is_empty());
                return;
            }
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
    if request.pages.is_none() && request.actor.0.has_owner() { return semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<String>(); }
    0
}

pub(super) fn request_logical_retained_bytes(request: &MemberOpenRequest) -> usize {
    let expected = request.expected.as_ref().map_or(0, |value| value.artifact_id.len() + value.dialect.artifact_kind.len() + value.dialect.standard.len() + value.dialect.subset.len());
    let owner = request.owner.as_ref().map_or(0, |value| value.parent.artifact_id.len() + value.parent.dialect.artifact_kind.len() + value.parent.dialect.standard.len() + value.parent.dialect.subset.len() + value.slot.len() + value.child_id.len());
    request.retained_input_bytes() + request.actor.0.len() + request.closing_actor.as_ref().map_or(0, String::len) + expected + owner
}

fn request_identity_capacity(request: &MemberOpenRequest) -> usize {
    let expected = request.expected.as_ref().map_or(0, |value| value.artifact_id.capacity() + value.dialect.artifact_kind.capacity() + value.dialect.standard.capacity() + value.dialect.subset.capacity());
    let owner = request.owner.as_ref().map_or(0, |value| value.parent.artifact_id.capacity() + value.parent.dialect.artifact_kind.capacity() + value.parent.dialect.standard.capacity() + value.parent.dialect.subset.capacity() + value.slot.capacity() + value.child_id.capacity());
    request.actor.0.original_allocation_bytes() + request.closing_actor.as_ref().map_or(0, String::capacity) + expected + owner
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
            let mut actual_retained_progress=RetainedCloneProgress::default();
            let mut zero = StepContext::new(OperationId(1), Generation(1), StepBudget::new(0, 999, request_close_policy()), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
            assert!(matches!(request.step_input(&mut zero), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
            let mut outcome = None;
            for _ in 0..16 {
                let mut actual_retained_progress=RetainedCloneProgress::default();
                let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, 999, request_close_policy()), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
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
                    let mut actual_retained_progress=RetainedCloneProgress::default();
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999, request_close_policy()), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
                    assert_eq!(request.copy_snapshot_chunk(0, &mut output, &mut cx).unwrap(), 1);
                    assert_eq!(output[0], bytes[frame.snapshot_start]);
                    assert_eq!(&output[1..], &[0xcc; 7]);
                    let mut actual_retained_progress=RetainedCloneProgress::default();
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999, request_close_policy()), cancel.clone(), || Some(1), &mut sequence, &mut actual_retained_progress);
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
        let mut actual_retained_progress=RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(8, 2000, request_close_policy()), cancel, clock, &mut sequence, &mut actual_retained_progress);
        assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
        assert_eq!(request.input_offset, 0);
        let mut actual_retained_progress=RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999, request_close_policy()), root_cancel_token(), || Some(1), &mut sequence, &mut actual_retained_progress);
        assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
        assert_eq!(request.retained_input_bytes(), 3);
        retire_request(&mut request);
    }
    let mut request = request_for(&[1, 97, 83]);
    let mut sequence = 0;
    let mut actual_retained_progress=RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 1, request_close_policy()), root_cancel_token(), || Some(1), &mut sequence, &mut actual_retained_progress);
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
        assert_eq!(request.close_step(RetainedCloneGrant { maximum_items: 0, maximum_release_bytes: 0, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap(), RetainedCloneStep::Progress(Default::default()));
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
            let demand = request.next_release_byte_demand().unwrap();
            if demand > 1 {
                if frame != 0 { assert_eq!(demand, frame); }
                let identity_pointer = request.identity_string_mut().map(|field| field.as_ptr());
                let pages_pointer = request.pages.as_ref().map(|pages| pages as *const OwnedSchemaDecodePages);
                let before = request_logical_retained_bytes(&request);
                let (denied, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand - 1, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
                assert_eq!(denied, RetainedCloneStep::Progress(Default::default()));
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(request_logical_retained_bytes(&request), before);
                if let Some(pointer) = identity_pointer { assert_eq!(pointer, request.identity_string_mut().unwrap().as_ptr()); }
                if let Some(pointer) = pages_pointer { assert!(std::ptr::eq(pointer, request.pages.as_ref().unwrap() as *const OwnedSchemaDecodePages)); }
                assert_eq!(request.next_release_byte_demand().unwrap(), demand);
            }
            let grant = request_close_policy().maximum_release_bytes;
            assert!(grant <= retirement_admission());
            let before = request_logical_retained_bytes(&request);
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: grant, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
            logical += before - request_logical_retained_bytes(&request);
            assert_eq!(events.requested_bytes, 0);
            match step {
                RetainedCloneStep::Progress(progress) => { let released_items = progress.copied_items; let released_bytes = progress.released_bytes;
                    assert!(released_items <= 1 && released_bytes <= grant);
                    assert_eq!(events.released_bytes, released_bytes);
                    if frame != 0 {
                        assert_eq!((released_items, released_bytes), (1, frame));
                        frames += frame;
                    }
                    released += released_bytes;
                }
                RetainedCloneStep::Complete(_) => { assert_eq!(events.released_bytes, 0); break; }
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
            let grant = request_close_policy().maximum_release_bytes;
            assert!(grant <= retirement_admission());
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: grant, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
            births += events.requested_bytes;
            physical += events.released_bytes;
            let released = match step {
                RetainedCloneStep::Progress(progress) => { let released_items = progress.copied_items; let released_bytes = progress.released_bytes; assert!(released_items <= 1 && released_bytes <= grant); released_bytes },
                RetainedCloneStep::Complete(_) => { assert!(request.terminal_is_empty()); 0 },
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
    let policy=request_close_policy();
    let setup=crate::os_store::component::tests::physical_test_close_grant();
    for row in fixture["cases"].as_array().unwrap() {
        let capacity=row["capacityBytes"].as_u64().unwrap() as usize;
        let mut request=request_for(&[1,97,83]);
        let mut previous=semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(&mut request.actor.0)).unwrap_or_else(|(error,_)|panic!("original actor: {error}"));
        for _ in 0..128 { if previous.terminal_is_empty(){break;} previous.step(setup).unwrap(); }
        assert!(previous.terminal_is_empty());
        let mut actor=String::with_capacity(capacity);actor.push_str(row["text"].as_str().unwrap());
        let pointer=actor.as_ptr();let text_bytes=actor.len();
        let (admitted,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::SharedUtf8::admit(actor,setup));
        let (actor,birth)=admitted.unwrap_or_else(|(error,_)|panic!("funded original actor: {error}"));
        assert!(birth.fits(setup));assert_eq!((events.requested_bytes,events.released_bytes),(birth.retained_capacity_bytes,0));
        assert_eq!(actor.as_ptr(),pointer);
        let lease=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<String>();
        assert_eq!(actor.original_allocation_bytes(),capacity+lease);request.actor.0=actor;
        while request.pages.is_some() {
            let (step,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(policy).unwrap());
            assert!(step.progress().fits(policy));assert_eq!((events.requested_bytes,events.released_bytes),(0,step.progress().released_bytes));
        }
        assert_eq!(request.next_release_byte_demand().unwrap(),lease);
        for denied in [RetainedCloneGrant{maximum_items:0,..policy},RetainedCloneGrant{maximum_copy_bytes:0,..policy},RetainedCloneGrant{maximum_release_bytes:lease-1,..policy}] {
            let (result,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(denied));
            if let Ok(step)=result {assert_eq!(step.progress(),Default::default());}
            assert_eq!((events.requested_bytes,events.released_bytes),(0,0));assert_eq!(request.actor.0.as_ptr(),pointer);assert_eq!(request.actor.0.len(),text_bytes);
        }
        let (step,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(policy).unwrap());
        assert!(step.progress().fits(policy));assert_eq!(step.progress().copied_bytes,std::mem::size_of::<Option<String>>());
        assert_eq!((events.requested_bytes,events.released_bytes),(0,lease));assert_eq!(step.progress().released_bytes,lease);
        assert!(!request.actor.0.has_owner());assert_eq!(request.closing_actor.as_ref().unwrap().as_ptr(),pointer);assert_eq!(request.closing_actor.as_ref().unwrap().capacity(),capacity);
        let (demand,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.next_release_byte_demand().unwrap());
        assert_eq!(demand,capacity);assert_eq!((events.requested_bytes,events.released_bytes),(0,0));
        for denied in [RetainedCloneGrant{maximum_items:0,..policy},RetainedCloneGrant{maximum_release_bytes:capacity-1,..policy}] {
            let (step,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(denied).unwrap());
            assert_eq!(step,RetainedCloneStep::Progress(Default::default()));assert_eq!((events.requested_bytes,events.released_bytes),(0,0));
            assert_eq!(request.closing_actor.as_ref().unwrap().as_ptr(),pointer);assert_eq!(request.closing_actor.as_ref().unwrap().len(),text_bytes);assert_eq!(request.next_release_byte_demand().unwrap(),capacity);
        }
        let (step,events)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(policy).unwrap());
        assert_eq!(step,RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()}));
        assert_eq!((events.requested_bytes,events.released_bytes),(0,capacity));assert!(request.closing_actor.is_none());
        retire_request(&mut request);
        println!("[DEBUG] member-open original identity={} logical-bytes={text_bytes} physical-capacity={capacity} shared-frame={lease} whole-denial-retained=true exact-paid-release=true",row["id"]);
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
            let (demand, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.next_release_byte_demand().unwrap());
            assert_eq!(demand, 0);
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            let (denied, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: 0, maximum_release_bytes: backing, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
            assert_eq!(denied, RetainedCloneStep::Progress(Default::default()));
            assert_eq!(request.retained_input_bytes(), before);
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: 0, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: 0, ..Default::default() }));
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            let tail = before % OWNED_SCHEMA_DECODE_PAGE_BYTES;
            assert_eq!(before - request.retained_input_bytes(), if tail == 0 { OWNED_SCHEMA_DECODE_PAGE_BYTES } else { tail });
            assert_eq!(request.pages.as_ref().unwrap().allocation_byte_demand(), backing);
        }
        assert_eq!(request.retained_input_bytes(), 0);
        assert_eq!(request.next_release_byte_demand().unwrap(), backing);
        for (items, grant) in [(0, backing), (1, 0), (1, backing - 1)] {
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: items, maximum_release_bytes: grant, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(request.next_release_byte_demand().unwrap(), backing);
        }
        assert!(backing <= fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize);
        let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| request.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: backing, maximum_depth: request_close_policy().maximum_depth, ..Default::default() }).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: backing, ..Default::default() }));
        assert_eq!((events.requested_bytes, events.released_bytes), (0, backing));
        retire_request(&mut request);
        println!("[DEBUG] inline request={} input={length} page-items={page_count} payload-physical-free=0 whole-backing={backing} zero-item-and-one-below-retained=true", row["id"]);
    }
}

#[test]
fn member_input_buffer_birth_and_retirement_preserve_actual_system_and_original_pointer(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    let plain:serde_json::Value=serde_json::from_str(include_str!("../../🏭️operation/📏️birth/🧫️fixtures/🔣️.json")).unwrap();
    let row=&plain["inputBuffer"];let grant=&row["policy"];
    let policy=RetainedCloneGrant{maximum_items:grant["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:grant["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:grant["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:grant["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:grant["maximumDepth"].as_u64().unwrap() as usize};
    let capacity=row["capacityBytes"].as_u64().unwrap() as usize;
    for blocked in [
        RetainedCloneGrant{maximum_items:0,..policy},
        RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..policy},
        RetainedCloneGrant{maximum_depth:0,..policy},
    ]{
        let mut original=None;
        let (step,heap)=observe_heap_allocations_on_this_thread(||super::operation::admit_member_input_buffer(&mut original,capacity,blocked).unwrap());
        assert!(step.is_none());assert!(original.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    let mut original=None;
    let (birth,heap)=observe_heap_allocations_on_this_thread(||super::operation::admit_member_input_buffer(&mut original,capacity,policy).unwrap().unwrap());
    assert!(birth.fits(policy));assert_eq!(birth.retained_capacity_bytes,capacity);assert_eq!((heap.requested_bytes,heap.released_bytes),(capacity,0));
    let pointer=original.as_ref().unwrap().as_ptr();
    let (refused,heap)=observe_heap_allocations_on_this_thread(||super::operation::admit_member_input_buffer(&mut original,capacity,policy).unwrap_err());
    assert_eq!(refused.kind,semio_framework_value::ValueRefusalKind::InvariantViolated);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.as_ref().unwrap().as_ptr(),pointer);
    let mut active=None;let mut allocated=capacity;let mut released=0;
    for turn in 0..32{
        let (step,heap)=observe_heap_allocations_on_this_thread(||if active.is_some(){crate::os_store::artifact_retirement_box_close_step(&mut active,policy)}else{crate::os_store::artifact_retirement_admit_owned(&mut original,&mut active,policy)});
        let progress=step.unwrap().progress();assert!(progress.fits(policy));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);
        allocated+=heap.requested_bytes;released+=heap.released_bytes;
        if original.is_none()&&active.is_none(){break;}
        assert!(progress.copied_items>0||progress.copied_bytes>0||progress.retained_capacity_bytes>0||progress.released_bytes>0);assert!(turn+1<32);
    }
    assert!(original.is_none()&&active.is_none());assert_eq!(released,allocated);
    let (_,heap)=observe_heap_allocations_on_this_thread(||drop((original,active)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    println!("[DEBUG] member input buffer explicit policy refuses before allocation; original pointer survives occupied refusal; every funded birth/close matches actual System and terminal Drop releases nothing");
}
