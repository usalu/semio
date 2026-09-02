use super::*;

//#region 🧪️Assembly
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }
fn open() -> UiDocumentAssembly {
    let data = fixture();
    let mut owner = UiDocumentAssembly::default();
    let mut surface = Some(SurfaceId::try_from(data["surface"].as_str().unwrap()).unwrap());
    assert!(!owner.open_into(&mut surface, 117, UiRevision(4), Some(UiNodeId(41)), 0, 1, 0).unwrap().progressed);
    assert!(surface.is_some() && owner.terminal_is_empty());
    let step = owner.open_into(&mut surface, 117, UiRevision(4), Some(UiNodeId(41)), 0, 1, 32768).unwrap();
    assert!(step.progressed && surface.is_none());
    assert_eq!(step.allocated_bytes, 0);
    assert!(step.initialized_bytes + step.moved_bytes <= 32768);
    owner
}
fn place(owner: &mut UiDocumentAssembly, source: &mut Option<UiNodeRecord>) -> usize {
    let mut allocated = 0;
    for _ in 0..1000 {
        let before = owner.allocated_bytes().unwrap();
        let step = owner.place_one(source, 1, 32768).unwrap();
        assert!(step.metadata_items <= 1);
        assert!(step.allocated_bytes <= 32768 && step.moved_bytes <= 32768);
        assert_eq!(owner.allocated_bytes().unwrap() - before, step.allocated_bytes);
        allocated += step.allocated_bytes;
        if source.is_none() { return allocated; }
    }
    panic!("admitted record placement did not finish");
}
fn finish(owner: &mut UiDocumentAssembly) -> UiDocumentLease {
    let mut target = None;
    let step = owner.finish_into(&mut target, UiRevision(4), 1, 32768).unwrap();
    assert!(step.complete && owner.terminal_is_empty());
    target.unwrap()
}
fn close(lease: &mut UiDocumentLease, bytes: usize) {
    for _ in 0..500000 {
        let step = lease.close_read_step_with_grant(1, bytes).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= bytes);
        if step.complete { assert!(lease.terminal_is_empty()); return; }
    }
    panic!("exact document read did not retire");
}

#[test]
fn retained_document_assembly_places_exact_pages_and_preserves_wire_and_payload_pointer() {
    let data = fixture();
    let mut owner = open();
    let mut first = super::tests::leaf_record(41, "first");
    first.component = serde_json::from_value(serde_json::json!({"type":"surface","kind":"canvas-2d","docSchema":"wire","doc":{"bytes":[1,2,3,4]},"bindings":[]})).unwrap();
    let pointer = match &first.component { crate::Component::Surface(props) => props.doc.bytes.as_slice().as_ptr(), _ => unreachable!() };
    let mut source = Some(first);
    assert!(!owner.place_one(&mut source, 1, 0).unwrap().progressed);
    assert_eq!(owner.allocated_bytes().unwrap(), 0);
    let allocated = place(&mut owner, &mut source) + place(&mut owner, &mut Some(super::tests::leaf_record(9, "second")));
    assert!(allocated > 0 && allocated < 32768);
    let mut lease = finish(&mut owner);
    let read = lease.try_read().unwrap();
    assert_eq!(read.allocated_bytes(), allocated);
    let ids = (0..read.len()).map(|index| read.node_at(index).unwrap().id.0).collect::<Vec<_>>();
    assert_eq!(serde_json::to_value(ids).unwrap(), data["nodeIds"]);
    assert_eq!(match &read.node_at(0).unwrap().component { crate::Component::Surface(props) => props.doc.bytes.as_slice().as_ptr(), _ => unreachable!() }, pointer);
    assert_eq!(read.exact_node(1, UiNodeId(41)).map(|_| ()), Err(UiDocumentLeaseError::NodeIdentity));
    drop(read);
    close(&mut lease, 1);
    eprintln!("[DEBUG] document-assembly allocated={allocated} payload-pointer-preserved=true wire-order=41,9 root-copy=false");
}

#[test]
fn retained_document_assembly_rejects_duplicate_without_consuming_input_and_cancels_exact_backing() {
    for bytes in fixture()["closeGrants"].as_array().unwrap() {
        let mut owner = open();
        place(&mut owner, &mut Some(super::tests::leaf_record(41, "original")));
        let before = owner.allocated_bytes().unwrap();
        let mut duplicate = Some(super::tests::leaf_record(41, "duplicate"));
        let mut error = None;
        for _ in 0..10 { match owner.place_one(&mut duplicate, 1, 32768) { Ok(_) => {}, Err(found) => { error = Some(found); break; } } }
        let error = error.unwrap();
        assert_eq!(error.kind, UiDocumentAssemblyErrorKind::DuplicateNode);
        let compared_bytes = error.compared_bytes;
        assert_eq!(owner.allocated_bytes().unwrap(), before);
        assert_eq!(duplicate.as_ref().unwrap().key.as_ref(), "duplicate");
        drop(duplicate);
        for _ in 0..100000 {
            let step = owner.close_step(1, bytes.as_u64().unwrap() as usize).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= bytes.as_u64().unwrap() as usize);
            if step.complete { break; }
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(compared_bytes, fixture()["comparisonBytesPerIdentity"].as_u64().unwrap() as usize);
    }
}

#[test]
fn retained_document_assembly_and_read_alias_do_not_wait_on_contended_arena() {
    let mut owner = open();
    place(&mut owner, &mut Some(super::tests::leaf_record(41, "live")));
    let mut lease = finish(&mut owner);
    let mut alias = None;
    assert!(!lease.try_alias_into(&mut alias, 0).unwrap());
    assert!(lease.try_alias_into(&mut alias, 32768).unwrap());
    assert!(lease.same_root(alias.as_ref().unwrap()));
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let holder = std::thread::spawn(move || { let _guard = UI_DOCUMENT_ARENA.lock().unwrap(); ready_tx.send(()).unwrap(); release_rx.recv().unwrap(); });
    ready_rx.recv().unwrap();
    let read_blocked = matches!(lease.try_read(), Err(UiDocumentLeaseError::Contended));
    let mut second = None;
    let alias_blocked = matches!(lease.try_alias_into(&mut second, 32768), Err(UiDocumentLeaseError::Contended));
    let mut blocked = UiDocumentAssembly::default();
    let mut surface = Some(SurfaceId::try_from("blocked").unwrap());
    let open_blocked = matches!(blocked.open_into(&mut surface, 117, UiRevision(4), Some(UiNodeId(41)), 0, 1, 32768), Err(error) if error.kind == UiDocumentAssemblyErrorKind::Contended);
    release_tx.send(()).unwrap(); holder.join().unwrap();
    close(alias.as_mut().unwrap(), 64);
    assert!(lease.try_read().unwrap().node_at(0).is_some());
    close(&mut lease, 64);
    assert!(read_blocked && alias_blocked && open_blocked && second.is_none() && surface.is_some());
}
//#endregion 🧪️Assembly

//#region 🧪️MetadataAccounting
#[test]
fn retained_document_assembly_reports_metadata_initialization_separately_from_empty_payload_capacity() {
    let mut owner = open();
    let mut source = Some(super::tests::leaf_record(41, "first"));
    let mut allocated = 0;
    let mut initialized = 0;
    for _ in 0..1000 {
        let step = owner.place_one(&mut source, 1, 32768).unwrap();
        allocated += step.allocated_bytes; initialized += step.initialized_bytes;
        if source.is_none() { break; }
    }
    let expected = allocated - std::mem::size_of::<UiNodeRecord>();
    let mut lease = finish(&mut owner);
    close(&mut lease, 64);
    eprintln!("[DEBUG] document-metadata allocated={allocated} initialized={initialized} expected-initialized={expected}");
    assert!(expected > 0);
    assert_eq!(initialized, expected, "metadata pages are initialized; reserved payload capacity is not");
}
//#endregion 🧪️MetadataAccounting
