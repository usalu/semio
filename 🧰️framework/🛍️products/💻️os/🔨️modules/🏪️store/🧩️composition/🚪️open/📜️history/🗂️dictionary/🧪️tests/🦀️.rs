//! 🧪️ Actual retained input and production SPR writer exercise the neutral owner contract.

use super::super::{MemberHistoryInputStep, MemberHistoryVerification};
use super::*;
use crate::os_io::{ArtifactDialect, ArtifactRef};
use crate::os_store::{OwnedSchemaDecodeCredits, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OwnerRef, OWNED_SCHEMA_DECODE_PAGE_BYTES};
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap()
}
fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}
fn delta(base: u64, entries: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = vec![1];
    protocol::codec::write_varint_u64(&mut bytes, base);
    protocol::codec::write_varint_u64(&mut bytes, entries.len() as u64);
    for entry in entries {
        protocol::codec::write_varint_u64(&mut bytes, entry.len() as u64);
        bytes.extend_from_slice(entry);
    }
    bytes
}

async fn history(fixture: &Value, row: &Value) -> Vec<u8> {
    let operation = row["operation"].as_str().unwrap_or("unchanged");
    let mut first: Vec<Vec<u8>> = fixture["dictionary"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().as_bytes().to_vec()).collect();
    let second: Vec<Vec<u8>> = fixture["secondDictionary"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().as_bytes().to_vec()).collect();
    match operation {
        "invalid-utf8" => first[0] = vec![195, 40],
        "empty-identity" => first[0].clear(),
        "bom-identity" => first[0].splice(..0, [239, 187, 191]).for_each(drop),
        "record-scratch" => first[row["index"].as_u64().unwrap() as usize] = hex(row["hex"].as_str().unwrap()),
        _ => {}
    }
    let mut first_delta = delta(if operation == "first-base" { row["value"].as_u64().unwrap() } else { 0 }, &first);
    if operation == "delta-tail" {
        first_delta.push(0);
    }
    let document = if operation == "raw-document" {
        let raw = hex(row["hex"].as_str().unwrap());
        let mut bytes = vec![1, 0];
        protocol::codec::write_varint_u64(&mut bytes, raw.len() as u64);
        bytes.extend_from_slice(&raw);
        bytes.extend_from_slice(&[1, 1]);
        bytes
    } else {
        hex(fixture["docHex"].as_str().unwrap())
    };
    let mut records = vec![(3, first_delta), (1, document)];
    if operation == "duplicate-document" {
        records.push((1, hex(fixture["docHex"].as_str().unwrap())));
    }
    records.push((3, delta(if operation == "second-base" { row["value"].as_u64().unwrap() } else { 7 }, &second)));
    if operation == "extra-entries" {
        records.push((3, delta(10, &(0..row["value"].as_u64().unwrap()).map(|index| format!("entry-{index}").into_bytes()).collect::<Vec<_>>())));
    }
    if operation == "unused-empty" {
        records.push((3, delta(10, &[Vec::new()])));
    }
    let mut composition = hex(fixture["compositionHex"].as_str().unwrap());
    match operation {
        "lookup-missing" => composition[3] = 127,
        "wrong-owner" => composition[5] = 6,
        "wrong-dialect" => composition[13] = 3,
        _ => {}
    }
    if operation == "earlier-foreign" {
        let mut foreign = composition.clone();
        foreign[5] = 6;
        records.push((65, foreign));
    }
    if operation == "earlier-malformed" {
        records.push((65, vec![1, 7]));
    }
    if matches!(operation, "aggregate-pin-limit" | "aggregate-group-limit") {
        records.push((65, composition.clone()));
    }
    if operation != "missing-composition" {
        records.push((65, composition));
    }
    let options = crate::os_spr::WriteOptions { required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: protocol::OPTIONAL_CANONICAL };
    let mut writer = crate::os_spr::SprWriter::begin(Vec::new(), &options).await.unwrap();
    for (kind, payload) in records {
        let critical = match (operation, kind) {
            ("noncritical-dictionary", 3) | ("noncritical-document", 1) => false,
            ("critical-composition", 65) => true,
            (_, 65) => false,
            _ => true,
        };
        writer.write_record(kind, critical, &payload, protocol::codec::ids::CodecId(0)).await.unwrap();
    }
    writer.commit().await.unwrap();
    if operation == "uncommitted-tail" {
        writer.write_record(3, true, &delta(10, &[b"uncommitted".to_vec()]), protocol::codec::ids::CodecId(0)).await.unwrap();
    }
    writer.into_sink().await
}

fn request(fixture: &Value, row: &Value, history: &[u8]) -> MemberOpenRequest {
    let mut fields: Vec<String> = fixture["requestIdentity"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect();
    if row["operation"] == "request-field" {
        let index = row["index"].as_u64().unwrap() as usize;
        fields[index] = row["text"].as_str().unwrap().into();
        if index == 0 || index == 9 {
            fields[0] = row["text"].as_str().unwrap().into();
            fields[9] = fields[0].clone();
        }
    }
    let expected = ArtifactRef { artifact_id: fields[0].clone(), dialect: ArtifactDialect { artifact_kind: fields[1].clone(), standard: fields[2].clone(), subset: fields[3].clone() } };
    let owner =
        OwnerRef { parent: ArtifactRef { artifact_id: fields[4].clone(), dialect: ArtifactDialect { artifact_kind: fields[5].clone(), standard: fields[6].clone(), subset: fields[7].clone() } }, slot: fields[8].clone(), child_id: fields[9].clone() };
    let mut bytes = vec![1, 170];
    bytes.extend_from_slice(history);
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len() }).unwrap();
    for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    MemberOpenRequest::new(OperationId(7), Generation(11), 1000, expected, Some(owner), pages).admit(1).unwrap_or_else(|_| panic!("neutral request admission"))
}

fn verified_input(fixture: &Value, row: &Value, history: &[u8]) -> VerifiedMemberHistoryInput {
    let mut owner = MemberHistoryVerification::new(request(fixture, row, history), RetainedSprLimits::default()).unwrap_or_else(|_| panic!("retained verifier admission"));
    let mut sequence = 0;
    for _ in 0..10000 {
        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(4096, 999), root_cancel_token(), || Some(1), &mut sequence);
        match owner.step(&mut cx) {
            MemberHistoryInputStep::Pending(_) => {}
            MemberHistoryInputStep::Ready => {
                let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
                let input = owner.take_ready(&mut cx).unwrap().unwrap();
                assert!(owner.terminal_is_empty());
                return input;
            }
            MemberHistoryInputStep::Rejected(error) => panic!("neutral writer framing rejected: {error:?}"),
        }
    }
    panic!("retained verification did not converge");
}

fn new_owner(fixture: &Value, row: &Value, history: &[u8]) -> MemberHistoryDictionaryOwner {
    let input = verified_input(fixture, row, history);
    let mut limits = MemberHistoryDictionaryLimits::default();
    match row["operation"].as_str().unwrap_or("unchanged") {
        "entry-limit" => limits.dictionary_entries = row["value"].as_u64().unwrap() as usize,
        "byte-limit" => limits.dictionary_bytes = row["value"].as_u64().unwrap(),
        "pin-limit" | "aggregate-pin-limit" => limits.pins = row["value"].as_u64().unwrap(),
        "aggregate-group-limit" => limits.pin_groups = row["value"].as_u64().unwrap(),
        _ => {}
    }
    let mut sequence = 0;
    let cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
    MemberHistoryDictionaryOwner::begin(input, "stdio.semio.flow", limits, &cx).unwrap_or_else(|_| panic!("dictionary input admission"))
}

fn retire(owner: &mut dyn ErasedSnapshotRetirement, grant: usize) -> usize {
    let mut retired = 0;
    assert!(matches!(owner.close_step(0, grant).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    for _ in 0..20000 {
        match owner.close_step(1, grant).unwrap() {
            SnapshotRetirementStep::Complete => {
                assert!(owner.terminal_is_empty());
                return retired;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= grant);
                retired += released_bytes;
            }
            SnapshotRetirementStep::Blocked => panic!("private owner cannot block its close"),
        }
    }
    panic!("bounded dictionary retirement did not converge");
}

fn expected_error(row: &Value) -> Option<MemberOpenDiagnostic> {
    row["error"].as_str().map(|error| match error {
        "malformed" => MemberOpenDiagnostic::Malformed,
        "capacity" => MemberOpenDiagnostic::Capacity,
        "identity" => MemberOpenDiagnostic::Identity,
        "cancelled" => MemberOpenDiagnostic::Cancelled,
        "stale" => MemberOpenDiagnostic::Stale,
        "expired" => MemberOpenDiagnostic::Expired,
        _ => panic!("unknown neutral error"),
    })
}

fn run_step(owner: &mut MemberHistoryDictionaryOwner, grant: u64) -> MemberHistoryDictionaryStep {
    let mut sequence = 0;
    let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(grant, 999), root_cancel_token(), || Some(1), &mut sequence);
    let before = owner.scanner.as_ref().map_or(owner.end, RetainedSprVerification::consumed);
    let result = owner.step(&mut cx);
    let after = owner.scanner.as_ref().map_or(owner.end, RetainedSprVerification::consumed);
    assert!(after - before <= grant - cx.fuel_remaining());
    result
}

#[semio_framework_async_macros::async_test]
async fn member_history_dictionary_is_atomic_and_bounded_by_neutral_records() {
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let bytes = history(&fixture, row).await;
        for grant in [1, 7, 4096] {
            let mut owner = new_owner(&fixture, row, &bytes);
            assert!(matches!(run_step(&mut owner, 0), MemberHistoryDictionaryStep::Pending(_)));
            assert!(owner.pending.is_none());
            let mut error = None;
            let mut terminal = false;
            let mut events = Vec::new();
            for _ in 0..20000 {
                let step = run_step(&mut owner, grant);
                if grant == 1 && row["operation"] == "unchanged" && matches!(owner.transition, "delta-begin" | "entry" | "delta") {
                    let index = owner.owners.as_ref().unwrap().index.as_ref().unwrap();
                    events.push(serde_json::json!([owner.transition, index.visible_entries(), index.allocated_pages()]));
                    if owner.transition != "delta" {
                        assert!(index.lookup(index.visible_entries()).is_err());
                    }
                }
                match step {
                    MemberHistoryDictionaryStep::Pending(_) => {}
                    MemberHistoryDictionaryStep::Ready => {
                        terminal = true;
                        break;
                    }
                    MemberHistoryDictionaryStep::Rejected(found) => {
                        error = Some(found);
                        terminal = true;
                        break;
                    }
                }
            }
            assert!(terminal, "{}", row["id"]);
            let index = owner.owners.as_ref().unwrap().index.as_ref().unwrap();
            let facts = (index.visible_entries(), index.allocated_pages(), owner.groups, owner.pins, owner.owners.as_ref().unwrap().input.as_ref().unwrap().retained_input_bytes());
            let ranges = if row["operation"] == "unchanged" {
                (0..index.visible_entries())
                    .map(|entry| {
                        let range = index.lookup(entry).unwrap();
                        serde_json::json!([range.offset, range.length])
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let mut sequence = 0;
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            let retired = if error.is_none() {
                let mut witness = owner.take_ready(&mut cx).unwrap().unwrap();
                assert!(owner.terminal_is_empty());
                assert!(matches!(owner.take_ready(&mut cx), Err(MemberOpenDiagnostic::Stale)));
                retire(&mut witness, grant as usize)
            } else {
                assert!(matches!(owner.take_ready(&mut cx), Err(found) if Some(found) == error));
                retire(&mut owner, grant as usize)
            };
            assert_eq!(error, expected_error(row), "{}", row["id"]);
            assert_eq!(facts, (row["entries"].as_u64().unwrap() as usize, row["pages"].as_u64().unwrap() as usize, row["groups"].as_u64().unwrap(), row["pins"].as_u64().unwrap(), row["inputBytes"].as_u64().unwrap() as usize), "{}", row["id"]);
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
            if row["operation"] == "unchanged" {
                assert_eq!(ranges, *fixture["dictionaryRanges"].as_array().unwrap());
                if grant == 1 {
                    assert_eq!(events, *fixture["ownerEvents"].as_array().unwrap());
                }
            }
        }
    }
    println!(
        "[DEBUG] retained dictionary owner:36 exact production-writer histories x3 grants; canonical critical flags, atomic deltas, full request identity, cumulative caps, one private handoff and literal complete retirement; no typed publication"
    );
}

#[semio_framework_async_macros::async_test]
async fn member_history_dictionary_retains_every_denied_owner_until_exact_close() {
    let fixture = fixture();
    let unchanged = serde_json::json!({ "operation": "unchanged" });
    let bytes = history(&fixture, &unchanged).await;
    for row in fixture["lifecycle"].as_array().unwrap() {
        for grant in [1, 7, 4096] {
            let mut owner = new_owner(&fixture, &unchanged, &bytes);
            let at = row["at"].as_str().unwrap();
            let mut found = at == "begin";
            for _ in 0..20000 {
                if found {
                    break;
                }
                assert!(!matches!(run_step(&mut owner, 1), MemberHistoryDictionaryStep::Rejected(_)));
                let index = owner.owners.as_ref().unwrap().index.as_ref().unwrap();
                found = match at {
                    "first-entry" => owner.transition == "entry" && index.visible_entries() == 0 && index.allocated_pages() == 1,
                    "first-delta" => owner.transition == "delta" && index.visible_entries() == 7,
                    "ready" | "witness" => owner.ready,
                    _ => false,
                };
            }
            assert!(found, "{}", row["id"]);
            let mut sequence = 0;
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), root_cancel_token(), || Some(1), &mut sequence);
            let mut witness = if at == "witness" { owner.take_ready(&mut cx).unwrap() } else { None };
            let cancel = root_cancel_token();
            let event = row["event"].as_str().unwrap();
            if event == "cancel" {
                cancel.cancel_now();
            }
            let operation = if event == "operation" { 8 } else { 7 };
            let generation = if event == "generation" { 12 } else { 11 };
            let clock: fn() -> Option<u64> = match event {
                "expired" => || Some(1000),
                "clock-absent" => || None,
                _ => || Some(1),
            };
            let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(7, 999), cancel, clock, &mut sequence);
            if let Some(witness) = witness.as_mut() {
                assert_eq!(witness.schema, fixture["expected"]["schema"].as_str().unwrap());
                assert_eq!(witness.check_step_authority(&cx).err(), expected_error(row));
                let mut retry_sequence = 0;
                let retry = StepContext::new(OperationId(7), Generation(11), StepBudget::new(7, 999), root_cancel_token(), || Some(1), &mut retry_sequence);
                assert_eq!(witness.check_step_authority(&retry).err(), expected_error(row));
                assert!(witness.owners.as_ref().unwrap().index.as_ref().unwrap().lookup(0).is_err());
            } else if event == "none" {
                witness = owner.take_ready(&mut cx).unwrap();
                assert!(witness.is_some());
            } else {
                assert!(matches!(owner.step(&mut cx), MemberHistoryDictionaryStep::Rejected(error) if Some(error) == expected_error(row)));
            }
            let owners = witness.as_ref().and_then(|witness| witness.owners.as_ref()).or_else(|| owner.owners.as_ref()).unwrap();
            let index = owners.index.as_ref().unwrap();
            let facts = (index.visible_entries(), index.allocated_pages(), owners.input.as_ref().unwrap().retained_input_bytes());
            assert_eq!(usize::from(witness.is_some()), row["handoffs"].as_u64().unwrap() as usize);
            let retired = if let Some(witness) = witness.as_mut() { retire(witness, grant) } else { retire(&mut owner, grant) };
            assert_eq!(facts, (row["entries"].as_u64().unwrap() as usize, row["pages"].as_u64().unwrap() as usize, row["inputBytes"].as_u64().unwrap() as usize));
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
    for row in fixture["recordRetirement"].as_array().unwrap() {
        let request_row = serde_json::json!({ "operation": "record-scratch", "index": row["index"], "hex": row["hex"] });
        let bytes = history(&fixture, &request_row).await;
        for grant in [1, 7, 4096] {
            let mut owner = new_owner(&fixture, &unchanged, &bytes);
            let mut error = None;
            for _ in 0..20000 {
                if let MemberHistoryDictionaryStep::Rejected(found) = run_step(&mut owner, 1) {
                    error = Some(found);
                    break;
                }
                if let Some(after) = row["cancelAfter"].as_u64() {
                    let pages = owner.owners.as_ref().unwrap().index.as_ref().unwrap().allocated_pages();
                    if owner.transition == "payload" && pages == row["pages"].as_u64().unwrap() as usize && owner.delta.as_ref().is_some_and(|delta| delta.retained_scratch_bytes() as u64 == after) {
                        let mut sequence = 0;
                        let cancel = root_cancel_token();
                        cancel.cancel_now();
                        let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(1, 999), cancel, || Some(1), &mut sequence);
                        if let MemberHistoryDictionaryStep::Rejected(found) = owner.step(&mut cx) {
                            error = Some(found);
                        }
                        break;
                    }
                }
            }
            let scratch = owner.delta.as_ref().map_or(0, RetainedDictionaryDelta::retained_scratch_bytes);
            let owners = owner.owners.as_ref().unwrap();
            let index = owners.index.as_ref().unwrap();
            let facts = (index.visible_entries(), index.allocated_pages(), owners.input.as_ref().unwrap().retained_input_bytes());
            let retired = retire(&mut owner, grant);
            assert_eq!(error, expected_error(row));
            assert_eq!(scratch as u64, row["scratchBytes"].as_u64().unwrap());
            assert_eq!(facts, (0, row["pages"].as_u64().unwrap() as usize, row["inputBytes"].as_u64().unwrap() as usize));
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
    for row in fixture["ownerRetirement"].as_array().unwrap() {
        let bytes = history(&fixture, row).await;
        for grant in [1, 7, 4096] {
            let mut owner = new_owner(&fixture, &unchanged, &bytes);
            let mut occurrence = 0;
            let mut found = false;
            for _ in 0..20000 {
                assert!(matches!(run_step(&mut owner, 1), MemberHistoryDictionaryStep::Pending(_)));
                let stage = row["stage"].as_str().unwrap();
                let matches = match stage {
                    "lookup-copy" => owner.transition == "id-copy",
                    "lookup-parse" => owner.transition == "id-feed",
                    "id-raw" => {
                        owner.transition == "payload" && owner.id.is_some() && owner.lookup.is_none() && owner.record.as_ref().is_some_and(|record| record.kind() == 1 && owner.scanner.as_ref().unwrap().consumed() > record.payload_start() + 3)
                    }
                    _ => owner.transition == stage,
                } && row["offset"].as_u64().is_none_or(|offset| owner.pending.as_ref().is_some_and(|pending| pending.offset == offset));
                if matches {
                    occurrence += 1;
                    if occurrence == row["occurrence"].as_u64().unwrap() {
                        found = true;
                        break;
                    }
                }
            }
            assert!(found, "{}", row["id"]);
            let pending = usize::from(owner.pending.is_some());
            let lookup = usize::from(owner.lookup_byte.is_some());
            let before = owner.scanner.as_ref().unwrap().consumed();
            let mut sequence = 0;
            let cancel = root_cancel_token();
            cancel.cancel_now();
            let mut cx = StepContext::new(OperationId(7), Generation(11), StepBudget::new(grant as u64, 999), cancel, || Some(1), &mut sequence);
            assert!(matches!(owner.step(&mut cx), MemberHistoryDictionaryStep::Rejected(MemberOpenDiagnostic::Cancelled)));
            assert_eq!(cx.fuel_remaining(), grant as u64);
            assert_eq!(owner.scanner.as_ref().unwrap().consumed(), before);
            assert!(matches!(run_step(&mut owner, grant as u64), MemberHistoryDictionaryStep::Rejected(MemberOpenDiagnostic::Cancelled)));
            assert!(matches!(owner.take_ready(&mut cx), Err(MemberOpenDiagnostic::Cancelled)));
            let owners = owner.owners.as_ref().unwrap();
            let index = owners.index.as_ref().unwrap();
            let facts = (index.visible_entries(), index.allocated_pages(), owners.input.as_ref().unwrap().retained_input_bytes());
            let retired = retire(&mut owner, grant);
            assert_eq!(pending as u64, row["pendingBytes"].as_u64().unwrap());
            assert_eq!(lookup as u64, row["lookupBytes"].as_u64().unwrap());
            assert_eq!(facts, (row["entries"].as_u64().unwrap() as usize, row["pages"].as_u64().unwrap() as usize, row["inputBytes"].as_u64().unwrap() as usize));
            assert_eq!(retired, row["retiredBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
            assert_eq!(retired - facts.2 - 72 - facts.1 * 1024 - pending - lookup, row["idBytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
    println!("[DEBUG] retained dictionary rejection owner:11 authority transitions +7 payload scratch +9 pending-copy/ID scratch traces x3 grants; original witness retained and exactly closed; no public member");
}
