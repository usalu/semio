use super::*;

//#region 🧪️DocumentComparison
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }
fn document(component: crate::Component) -> UiDocumentLease {
    let fixture = fixture();
    let mut builder = UiDocumentBuilder::try_new(fixture["documentGeneration"].as_u64().unwrap(), SurfaceId::try_from("comparison").unwrap(), UiRevision(fixture["revision"].as_u64().unwrap()), Some(UiNodeId(41)), 0).unwrap();
    let mut record = super::tests::leaf_record(41, "first");
    record.component = component;
    builder.try_push(record).unwrap();
    builder.try_push(super::tests::leaf_record(9, "second")).unwrap();
    builder.finish().unwrap()
}
fn data(lease: &UiDocumentLease) -> serde_json::Value {
    let arena = UI_DOCUMENT_ARENA.lock().unwrap();
    let slot = arena.slot(lease.handle.unwrap()).unwrap();
    serde_json::to_value(&slot.nodes.entries).unwrap()
}
fn close_document(lease: &mut UiDocumentLease) {
    for _ in 0..500_000 { if lease.close_step_with_grant(1, 64).unwrap().complete { assert!(lease.terminal_is_empty()); return; } }
    panic!("exact document did not close");
}
fn close(owner: &mut UiDocumentComponentCompare, grant: usize) {
    for _ in 0..500_000 {
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        if step.complete { assert!(owner.terminal_is_empty()); return; }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("comparison waited on the live document");
}
fn start(lease: UiDocumentLease, component: crate::Component) -> UiDocumentComponentCompare {
    match UiDocumentComponentCompare::try_new(lease, 0, UiNodeId(41), component, 32768) {
        Ok((owner, admission)) => {
            assert_eq!(admission.allocated_bytes, 0);
            assert_eq!(admission.owner_bytes, std::mem::size_of::<UiDocumentComponentCompare>());
            assert_eq!(admission.moved_bytes, std::mem::size_of::<UiDocumentLease>() + std::mem::size_of::<crate::Component>());
            assert_eq!(admission.owner_bytes + admission.moved_bytes, UiDocumentComponentCompare::required_admission_bytes());
            assert!(admission.owner_bytes > 4096 && admission.owner_bytes + admission.moved_bytes <= 32768);
            owner
        }
        Err((error, _, _)) => panic!("admitted exact comparison: {error:?}"),
    }
}

#[test]
fn retained_document_component_compare_reads_exact_lease_without_copy_and_preserves_wire_order() {
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../../♻️retirement/🌳️typed/🧩️components.json")).unwrap();
    for row in components["cases"].as_array().unwrap() {
        for grant in fixture["grants"].as_array().unwrap() {
            let mut keeper = document(serde_json::from_value(row["component"].clone()).unwrap());
            let before = data(&keeper);
            assert_eq!(before.as_array().unwrap().iter().map(|record| record["id"].as_u64().unwrap()).collect::<Vec<_>>(), vec![41,9]);
            let pointer = { let arena = UI_DOCUMENT_ARENA.lock().unwrap(); &arena.slot(keeper.handle.unwrap()).unwrap().nodes.get_index(0).unwrap().component as *const _ };
            let mut owner = start(keeper.try_alias().unwrap(), serde_json::from_value(row["component"].clone()).unwrap());
            assert!(!owner.advance(0, 4096).unwrap().progressed);
            assert!(!owner.advance(1, 0).unwrap().progressed);
            for _ in 0..500_000 {
                let step = owner.advance(1, grant.as_u64().unwrap() as usize).unwrap();
                assert!(step.compared_bytes <= grant.as_u64().unwrap() as usize);
                if step.complete { break; }
            }
            let equal = owner.result();
            close(&mut owner, 64);
            assert_eq!(equal, Some(true));
            assert_eq!(data(&keeper), before);
            let after = { let arena = UI_DOCUMENT_ARENA.lock().unwrap(); &arena.slot(keeper.handle.unwrap()).unwrap().nodes.get_index(0).unwrap().component as *const _ };
            assert_eq!(pointer, after);
            assert_eq!(keeper.header().unwrap().node_count, 2);
            close_document(&mut keeper);
        }
    }
    eprintln!("[DEBUG] document-component-read variants=18 grants=1,64,4096 old-root-copy=false wire-order=41,9 exact-close=true");
}

#[test]
fn retained_document_component_compare_rejects_exact_owners_before_admission_or_foreign_ordinal() {
    let required = UiDocumentComponentCompare::required_admission_bytes();
    let component: crate::Component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":[1,2,3]},"bindings":[]})).unwrap();
    let mut keeper = document(crate::Component::Separator(crate::SeparatorProps {}));
    let mut lease = keeper.try_alias().unwrap();
    let mut incoming = component;
    let original = match &incoming { crate::Component::Surface(props) => props.doc.bytes.as_slice().as_ptr(), _ => unreachable!() };
    for (ordinal, id, grant, expected) in [(0,41,required-1,UiDocumentCompareError::Admission), (1,41,32768,UiDocumentCompareError::NodeIdentity), (2,41,32768,UiDocumentCompareError::NodeIdentity)] {
        match UiDocumentComponentCompare::try_new(lease, ordinal, UiNodeId(id), incoming, grant) {
            Err((error, returned_lease, returned_incoming)) => { assert_eq!(error, expected); lease = returned_lease; incoming = returned_incoming; }
            Ok(_) => panic!("invalid admission must return exact owners"),
        }
        assert_eq!(lease.generation(), 93);
        assert_eq!(match &incoming { crate::Component::Surface(props) => props.doc.bytes.as_slice().as_ptr(), _ => unreachable!() }, original);
    }
    let mut owner = start(lease, incoming);
    for _ in 0..100 { if owner.advance(1, 64).unwrap().complete { break; } }
    let equal = owner.result();
    close(&mut owner, 64);
    close_document(&mut keeper);
    assert_eq!(equal, Some(false));
    eprintln!("[DEBUG] document-component-admission required={required} rejected-root-pointer-exact=true foreign-ordinal-denied=true");
}

#[test]
fn retained_document_component_compare_cancel_and_contention_keep_live_document_and_incoming_root() {
    let fixture = fixture();
    let text = fixture["text"].as_str().unwrap().repeat(fixture["textRepeats"].as_u64().unwrap() as usize);
    let value = serde_json::json!({"type":"extension","extension":"nested","props":[text.clone(),{"key":[text]}]});
    for frontier in fixture["cancelFrontiers"].as_array().unwrap() {
        for grant in fixture["grants"].as_array().unwrap() {
            let mut keeper = document(serde_json::from_value(value.clone()).unwrap());
            let before = data(&keeper);
            let mut owner = start(keeper.try_alias().unwrap(), serde_json::from_value(value.clone()).unwrap());
            for _ in 0..frontier.as_u64().unwrap() { owner.advance(1, 1).unwrap(); }
            close(&mut owner, grant.as_u64().unwrap() as usize);
            assert_eq!(data(&keeper), before);
            close_document(&mut keeper);
        }
    }
    let mut keeper = document(serde_json::from_value(value.clone()).unwrap());
    let lease = keeper.try_alias().unwrap();
    let incoming = serde_json::from_value(value).unwrap();
    let (entered_tx, entered_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let holder = std::thread::spawn(move || { let _arena = UI_DOCUMENT_ARENA.lock().unwrap(); entered_tx.send(()).unwrap(); release_rx.recv().unwrap(); });
    entered_rx.recv().unwrap();
    let rejected = UiDocumentComponentCompare::try_new(lease, 0, UiNodeId(41), incoming, 32768);
    release_tx.send(()).unwrap(); holder.join().unwrap();
    let (lease, incoming) = match rejected { Err((UiDocumentCompareError::Contended, lease, incoming)) => (lease, incoming), _ => panic!("busy document admission must not wait") };
    let mut owner = start(lease, incoming);
    let arena = UI_DOCUMENT_ARENA.lock().unwrap();
    let step = owner.advance(1, 4096).unwrap();
    let close_step = owner.close_step(1, 64).unwrap();
    drop(arena);
    assert!(!step.progressed && step.compared_bytes == 0);
    assert!(!close_step.complete);
    close(&mut owner, 64);
    assert!(keeper.header().is_ok());
    close_document(&mut keeper);
    eprintln!("[DEBUG] document-component-cancel frontiers=7 grants=1,64,4096 contention-retains=true live-document-wait=false");
}
#[test]
fn retained_document_component_compare_final_reads_transfer_exact_root_to_one_retirement_owner() {
    let fixture = fixture();
    let value = serde_json::json!({"type":"extension","extension":"final","props":{"é":["éé"]}});
    for mode in fixture["finalReadModes"].as_array().unwrap() {
        let lease = document(serde_json::from_value(value.clone()).unwrap());
        let handle = lease.handle.unwrap();
        match mode.as_str().unwrap() {
            "sole" => {
                let mut owner = start(lease, serde_json::from_value(value.clone()).unwrap());
                close(&mut owner, 1);
            }
            "queued-alias" => {
                let mut owner = start(lease.try_alias().unwrap(), serde_json::from_value(value.clone()).unwrap());
                drop(lease);
                close(&mut owner, 1);
            }
            "concurrent-pair" => {
                let other = lease.try_alias().unwrap();
                let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
                let workers: Vec<_> = [lease, other].into_iter().map(|lease| {
                    let mut owner = start(lease, serde_json::from_value(value.clone()).unwrap());
                    let barrier = barrier.clone();
                    std::thread::spawn(move || { barrier.wait(); close(&mut owner, 1); })
                }).collect();
                for worker in workers { worker.join().unwrap(); }
            }
            _ => unreachable!(),
        }
        let arena = UI_DOCUMENT_ARENA.lock().unwrap();
        assert!(arena.slot(handle).is_none());
    }
    eprintln!("[DEBUG] document-component-final-read modes=sole,concurrent-pair,queued-alias exact-final-owner=true");
}
//#endregion 🧪️DocumentComparison
