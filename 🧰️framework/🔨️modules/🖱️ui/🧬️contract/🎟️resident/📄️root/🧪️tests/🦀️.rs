use super::*;

//#region 🧪️RootPermit
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }
fn reserve(bytes: usize) -> Option<UiResidentPermit> {
    let mut permit = None;
    UiResidentPermit::try_reserve(UiResidentLimits { items: 32, bytes }, &mut permit, 32768).unwrap();
    permit
}
fn assembled(generation: u64, bytes: usize) -> UiDocumentAssembly {
    let mut permit = reserve(bytes);
    let mut owner = UiDocumentAssembly::default();
    let mut surface = Some(SurfaceId::try_from(fixture()["surface"].as_str().unwrap()).unwrap());
    assert!(!owner.open_with_permit(&mut permit, &mut surface, generation, UiRevision(1), Some(UiNodeId(41)), 0, 1, 0).unwrap().progressed);
    assert!(permit.is_some() && surface.is_some());
    assert!(owner.open_with_permit(&mut permit, &mut surface, generation, UiRevision(1), Some(UiNodeId(41)), 0, 1, 32768).unwrap().progressed);
    assert!(permit.is_none() && surface.is_none());
    let mut record = super::tests::leaf_record(41, "root");
    record.component = crate::Component::Extension(crate::ExtensionProps { extension: crate::UiText::try_from_str("typed").unwrap(), props: serde_json::from_value(serde_json::json!({"payload":"Grüße"})).unwrap() });
    let mut record = Some(record);
    for _ in 0..100 { owner.place_one(&mut record, 1, 32768).unwrap(); if record.is_none() { return owner; } }
    panic!("root placement did not finish");
}
fn finish(owner: &mut UiDocumentAssembly) -> UiDocumentLease {
    let mut result = None;
    assert!(owner.finish_into(&mut result, UiRevision(1), 1, 32768).unwrap().complete);
    result.unwrap()
}
fn close(owner: &mut UiDocumentLease, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..100000 {
        let step = owner.close_read_step_with_grant(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { return bytes; }
    }
    panic!("root did not retire");
}

#[test]
fn retained_document_root_permit_nine_surfaces_share_one_aggregate() {
    let data = fixture();
    let before = UiResidentPermit::snapshot().unwrap();
    let count = data["surfaces"].as_u64().unwrap() as usize;
    let bytes = data["reservationBytes"].as_u64().unwrap() as usize;
    let mut roots = Vec::new();
    for index in 0..count { roots.push(finish(&mut assembled(index as u64 + 1, bytes))); }
    let during = UiResidentPermit::snapshot().unwrap();
    for root in &roots {
        let read = root.try_read().unwrap();
        assert_eq!(read.node_at(0).unwrap().id.0.to_le_bytes(), [41, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(serde_json::to_value(&read.node_at(0).unwrap().component).unwrap()["props"]["payload"], data["payload"]);
    }
    for root in &mut roots { close(root, 64); }
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
    assert_eq!(during.used_slots - before.used_slots, count);
    assert_eq!(during.bytes - before.bytes, count * bytes);
    eprintln!("[DEBUG] document-root-permit actual-surfaces={count} exact-bytes={} final-credit=0", count * bytes);
}

#[test]
fn retained_document_root_permit_last_reader_keeps_credit_and_typed_payload() {
    for grant in [1, 64, 4096] {
        let before = UiResidentPermit::snapshot().unwrap();
        let mut owner = finish(&mut assembled(100 + grant as u64, 65536));
        let mut reader = None;
        assert!(owner.try_alias_into(&mut reader, 32768).unwrap());
        assert_eq!(close(&mut owner, grant), 0);
        assert_eq!(UiResidentPermit::snapshot().unwrap().bytes - before.bytes, 65536);
        assert_eq!(reader.as_ref().unwrap().try_read().unwrap().len(), 1);
        let released = close(reader.as_mut().unwrap(), grant);
        assert!(released >= fixture()["payloadUtf8Bytes"].as_u64().unwrap() as usize);
        assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
    }
    eprintln!("[DEBUG] document-root-reader grants=1,64,4096 credit-until-final=true typed-descendants-before-credit=true");
}

#[test]
fn retained_document_root_permit_cancel_and_contended_final_return_keep_exact_owner() {
    let before = UiResidentPermit::snapshot().unwrap();
    let mut candidate = assembled(77, 65536);
    let observation = UiResidentPermit::try_observe().unwrap();
    let mut blocked = false;
    for _ in 0..100000 {
        let step = candidate.close_step(1, 1).unwrap();
        assert!(!step.complete);
        if !step.progressed { blocked = true; break; }
    }
    assert_eq!(observation.snapshot().bytes - before.bytes, 65536);
    drop(observation);
    for _ in 0..10000 { if candidate.close_step(1, 64).unwrap().complete { break; } }
    assert!(blocked && candidate.terminal_is_empty());
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
    let mut reused = finish(&mut assembled(78, 65536));
    drop(candidate);
    for _ in 0..128 { close_ui_document_page_with_grant(1, 64).unwrap(); }
    assert_eq!(reused.try_read().unwrap().len(), 1);
    close(&mut reused, 64);
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}
#[test]
fn retained_document_root_permit_reader_pressure_refuses_then_retries_exact_slot() {
    let data = fixture();
    assert_eq!(data["pressureAggregateBytes"].as_u64().unwrap() as usize, UI_RESIDENT_AGGREGATE_BYTES);
    let before = UiResidentPermit::snapshot().unwrap();
    let mut available = UI_RESIDENT_AGGREGATE_BYTES - before.bytes;
    let mut roots: Vec<_> = (0..data["pressureRoots"].as_u64().unwrap()).map(|index| {
        let bytes = available.min(data["pressureReservationBytes"].as_u64().unwrap() as usize);
        available -= bytes;
        finish(&mut assembled(200 + index, bytes))
    }).collect();
    assert_eq!(available, 0);
    let mut reader = None;
    assert!(roots[0].try_alias_into(&mut reader, 32768).unwrap());
    close(&mut roots[0], 64);
    let mut denied = None;
    assert_eq!(UiResidentPermit::try_reserve(UiResidentLimits { items: 1, bytes: 1 }, &mut denied, 32768), Err(UiResidentFault::Capacity));
    assert!(denied.is_none());
    assert_eq!(UiResidentPermit::snapshot().unwrap().bytes, UI_RESIDENT_AGGREGATE_BYTES);
    let old_key = reader.as_ref().unwrap().handle.unwrap();
    for _ in 0..100000 {
        reader.as_mut().unwrap().close_read_step_with_grant(1, 64).unwrap();
        if UiResidentPermit::snapshot().unwrap().bytes < UI_RESIDENT_AGGREGATE_BYTES { break; }
    }
    assert!(!reader.as_ref().unwrap().terminal_is_empty());
    let mut permit = reserve(65536);
    assert_eq!(permit.as_ref().unwrap().root_key().unwrap().slot, old_key.slot);
    assert!(permit.as_ref().unwrap().root_key().unwrap().epoch > old_key.epoch);
    let mut candidate = UiDocumentAssembly::default();
    let mut surface = Some(SurfaceId::try_from("retry").unwrap());
    let error = candidate.open_with_permit(&mut permit, &mut surface, 300, UiRevision(1), Some(UiNodeId(41)), 0, 1, 32768).unwrap_err();
    assert_eq!(error.kind, UiDocumentAssemblyErrorKind::Stale);
    assert!(data["slotReuseRequiresTypedTerminal"].as_bool().unwrap());
    assert!(permit.is_some() && surface.is_some());
    close(reader.as_mut().unwrap(), 64);
    assert!(candidate.open_with_permit(&mut permit, &mut surface, 300, UiRevision(1), Some(UiNodeId(41)), 0, 1, 32768).unwrap().progressed);
    for _ in 0..10000 { if candidate.close_step(1, 64).unwrap().complete { break; } }
    assert!(candidate.terminal_is_empty());
    for root in &mut roots { close(root, 64); }
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
    eprintln!("[DEBUG] document-root-pressure aggregate=33554432 captured-reader-keeps-credit=true exact-slot-epoch-retry=true");
}
#[test]
fn retained_document_root_permit_seal_transfers_output_without_detaching_root_credit() {
    let data = fixture();
    let sealed = data["sealedBytes"].as_u64().unwrap() as usize;
    for order in data["outputOrders"].as_array().unwrap() {
        let output_first = order[0].as_u64().unwrap() == 2;
        let before = UiResidentPermit::snapshot().unwrap();
        let mut candidate = assembled(400, 65536);
        let mut output = None;
        let limits = UiResidentLimits { items: 16, bytes: sealed };
        assert!(!candidate.shrink_resident(limits, 1, 0).unwrap().progressed);
        assert!(candidate.shrink_resident(limits, 1, 32768).unwrap().progressed);
        assert!(!candidate.split_resident_output(&mut output, 1, 0).unwrap().progressed);
        assert!(output.is_none());
        assert!(candidate.split_resident_output(&mut output, 1, 32768).unwrap().progressed);
        assert!(candidate.shrink_resident(limits, 1, 32768).is_err());
        assert!(!data["shrinkAfterSplit"].as_bool().unwrap());
        let mut lease = finish(&mut candidate);
        assert_eq!(lease.try_read().unwrap().resident_limits(), limits);
        if output_first { assert!(output.as_mut().unwrap().close_step(1).unwrap().complete); }
        else { close(&mut lease, 64); }
        assert_eq!(UiResidentPermit::snapshot().unwrap().bytes - before.bytes, sealed);
        if output_first { close(&mut lease, 64); }
        else { assert!(output.as_mut().unwrap().close_step(1).unwrap().complete); }
        assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
    }
    eprintln!("[DEBUG] document-root-output shrink-before-split=true output-first-and-root-first=true exact-final-return=32768");
}
//#endregion 🧪️RootPermit
