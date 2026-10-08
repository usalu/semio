pub(crate) struct MemberReceiptOperation(pub(crate) String);
impl store::ArtifactCanonicalJson for MemberReceiptOperation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<store::ArtifactCanonicalJsonNode<'_>, String> { if path.is_empty() { Ok(store::ArtifactCanonicalJsonNode::String(&self.0)) } else { Err("neutral receipt operation has only its original string root".into()) } }
}
pub(crate) struct MemberReceiptPublication { pub(crate) edit: String, pub(crate) forward: Vec<MemberReceiptOperation>, pub(crate) inverse: Vec<MemberReceiptOperation>, pub(crate) metadata: Vec<protocol::MutationMeta>, pub(crate) schemas: Vec<(String,String)> }
impl store::ErasedMemberStoreOneItemPublication for MemberReceiptPublication {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn phase(&self) -> store::ArtifactStoreOneItemPublicationPhase { store::ArtifactStoreOneItemPublicationPhase::Publishing }
    fn progress(&self) -> store::ArtifactStoreOneItemCheckpoint { Default::default() }
    fn prepared_edit_id(&self) -> Option<&str> { Some(&self.edit) }
    fn prepared_operation_count(&self, inverse: bool) -> Option<usize> { Some(if inverse { self.inverse.len() } else { self.forward.len() }) }
    fn prepared_operation(&self, inverse: bool, index: usize) -> Option<&dyn std::any::Any> { (if inverse { &self.inverse } else { &self.forward }).get(index).map(|operation| operation as &dyn std::any::Any) }
    fn prepared_operation_metadata(&self, index: usize) -> Option<&protocol::MutationMeta> { self.metadata.get(index) }
    fn prepared_operation_schema_parts(&self, index: usize) -> Option<(&str, &str)> { self.schemas.get(index).map(|(entity,kind)| (entity.as_str(),kind.as_str())) }
    fn prepared_operation_wire_source(&self, inverse: bool, index: usize) -> Option<store::ArtifactPreparedOperationSource<'_>> { (if inverse { &self.inverse } else { &self.forward }).get(index).map(|body| store::ArtifactPreparedOperationSource::CanonicalJson { header: &[1,7], body }) }
    fn next_group_byte_demand(&self) -> usize { 0 }
    fn fault(&self) -> Option<&str> { None }
    fn retry(&mut self) -> bool { false }
    fn acknowledge(&mut self) -> bool { false }
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, ValueError> { Ok(store::SnapshotRetirementStep::Blocked) }
    fn terminal_is_empty(&self) -> bool { false }
}
pub(crate) fn member_receipt_fixture(fixture: &serde_json::Value, count: usize, inverse_count: usize) -> MemberReceiptPublication {
    let operation = |text: &str| { let mut original = String::with_capacity(fixture["payloadCapacity"].as_u64().unwrap() as usize); original.push_str(text); MemberReceiptOperation(original) };
    MemberReceiptPublication {
        edit: fixture["editId"].as_str().unwrap().into(),
        forward: fixture["operations"].as_array().unwrap().iter().take(count).map(|row| operation(row["text"].as_str().unwrap())).collect(),
        inverse: fixture["inverseTexts"].as_array().unwrap().iter().take(inverse_count).map(|text| operation(text.as_str().unwrap())).collect(),
        schemas: fixture["operations"].as_array().unwrap().iter().take(count).map(|row| { let schema = format!("{}{}", row["schema"].as_str().unwrap(), row["suffix"].as_str().unwrap()); let (entity,kind) = schema.rsplit_once('.').unwrap(); (entity.into(),kind.into()) }).collect(),
        metadata: fixture["operations"].as_array().unwrap().iter().take(count).map(|row| protocol::MutationMeta { mutation_id: Some(MutationId(row["id"].as_str().unwrap().into())), dependencies: row["dependencies"].as_array().unwrap().iter().map(|id| MutationId(id.as_str().unwrap().into())).collect(), base_version: row["baseVersion"].as_u64().unwrap(), author_id: row["author"].as_str().map(|id| ActorId(id.into())), timestamp: HybridLogicalTimestamp { actor: row["clock"][0].as_u64().unwrap(), physical_ms: row["clock"][1].as_u64().unwrap(), logical: row["clock"][2].as_u64().unwrap() }, undo_policy: UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: Some(fixture["groupId"].as_str().unwrap().into()), origin: protocol::MutationOrigin::Owner, transaction: None }).collect(),
    }
}
pub(crate) fn member_receipt_source<'a>(fixture: &'a serde_json::Value, publication: &'a MemberReceiptPublication, _index: usize) -> MountedMemberReceiptSource<'a> {
    MountedMemberReceiptSource { publication, document: ArtifactHandle(fixture["document"].as_u64().unwrap() as u128), invocation_id: fixture["invocationId"].as_str().unwrap(), group_id: fixture["groupId"].as_str().unwrap(), fallback_author: fixture["fallbackAuthor"].as_str().unwrap() }
}
fn member_receipt_close(owner: &mut MountedMemberReceipt) {
    for _ in 0..1000 {
        if owner.terminal_is_empty() { return; }
        let bytes = owner.next_close_byte_demand().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: 64 };
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: bytes.saturating_sub(1), ..grant }] {
            if denied.maximum_items != 0 && bytes == 0 { continue; }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied).unwrap());
            assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());
        assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
    }
    panic!("member receipt must reach exact terminal emptiness after admitted original owner retirement");
}
fn member_receipt_drive(owner: &mut MountedMemberReceipt, fixture: &serde_json::Value, publication: &MemberReceiptPublication, turns: usize) {
    for _ in 0..turns {
        if owner.is_complete() { return; }
        let source = member_receipt_source(fixture, publication, owner.operation_index().unwrap_or(0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: owner.next_capacity_byte_demand(&source).unwrap(), maximum_release_bytes: owner.next_release_byte_demand(&source).unwrap(), maximum_depth: 64 };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0));
        if grant.maximum_capacity_bytes > 0 || grant.maximum_release_bytes > 0 { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(1), maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(1), ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0)); }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, grant).unwrap());
        assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
}
#[test]
fn mounted_member_receipt_preserves_ordered_multiple_operations_and_original_causal_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["maximumCopyBytes"],64);assert_eq!(fixture["maximumCapacityBytes"],MOUNTED_RECEIPT_MAXIMUM_BYTES);assert_eq!(fixture["operations"][0]["text"].as_str().unwrap().len(),8194);
    for (count,inverse_count) in [(0,0),(1,0),(2,2)] {
        let publication = member_receipt_fixture(&fixture,count,inverse_count); let original: Vec<_> = publication.forward.iter().map(|body| (body.0.as_ptr(),body.0.capacity())).collect();
        let source = member_receipt_source(&fixture,&publication,0);
        let (mut owner,heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| MountedMemberReceipt::new(&source).unwrap()); assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        member_receipt_drive(&mut owner,&fixture,&publication,12000); assert!(owner.is_complete()); assert_eq!(owner.operation_count(),count); assert_eq!(owner.ready_document(),Some(source.document)); assert_eq!(owner.ready_edit_id(),Some(publication.edit.as_str()));
        let inverses: Vec<_> = publication.inverse.iter().map(|body| { let mut encoded=vec![1,7];encoded.extend(serde_json::to_vec(&body.0).unwrap());encoded }).collect(); let inverse = protocol::encode_ops_vec(&inverses); assert_eq!(inverse.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), fixture[if inverse_count == 0 { "zeroInverseWireHex" } else { "inverseWireHex" }].as_str().unwrap());
        let grant = RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64 };
        let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take_next_triple(RetainedCloneGrant { maximum_items: 0, ..grant })); assert!(denied.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0,0)); assert_eq!(owner.triples.len(), count);
        for (index,row) in fixture["operations"].as_array().unwrap().iter().take(count).enumerate() {
            let (triple,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take_next_triple(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let (mutation,id,undo)=triple;let meta=&publication.metadata[index];
            assert_eq!(mutation.id.0,row["id"].as_str().unwrap());assert_eq!(id,mutation.id);assert_eq!(undo,mutation.inverse);assert_eq!(mutation.document,source.document);assert_eq!(mutation.invocation_id.0,source.invocation_id);assert_eq!(mutation.base_version,ArtifactVersion(meta.base_version));assert_eq!(mutation.timestamp,meta.timestamp);assert_eq!(mutation.author.0,row["author"].as_str().unwrap_or(source.fallback_author));assert_eq!(mutation.diff.schema.0,format!("{}{}",row["schema"].as_str().unwrap(),row["suffix"].as_str().unwrap()));
            let mut encoded=vec![1,7];encoded.extend(serde_json::to_vec(&publication.forward[index].0).unwrap());assert_eq!(mutation.diff.payload,encoded);assert_eq!(mutation.inverse.inverse_diff.payload,inverse);assert_eq!(undo.inverse_diff.payload,inverse);assert_eq!(mutation.dependencies,meta.dependencies);assert_eq!(mutation.inverse.dependencies,meta.dependencies);assert_eq!(undo.dependencies,meta.dependencies);assert_eq!(mutation.inverse.undo_policy,meta.undo_policy);
            assert_eq!(owner.ready_edit_id(),Some(publication.edit.as_str())); assert_eq!(publication.forward[index].0.as_ptr(),original[index].0);assert_eq!(publication.forward[index].0.capacity(),8194);
        }
        assert!(owner.take_next_triple(grant).is_none());member_receipt_close(&mut owner);let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        println!("[DEBUG] member receipt operations={count} inverse={inverse_count}: distinct exact schemas, framed serde inverses, original8194 pointers, ordered partial pop0heap and every causal physical owner admitted");
    }
}
#[test]
fn mounted_member_receipt_cancellation_preserves_every_original_owner_and_partial_transfer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for turns in fixture["cancelTurns"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize).chain([12000]) {
        let publication=member_receipt_fixture(&fixture,2,2);let source=member_receipt_source(&fixture,&publication,0);let mut owner=MountedMemberReceipt::new(&source).unwrap();member_receipt_drive(&mut owner,&fixture,&publication,turns);
        if owner.is_complete() { let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};let (triple,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.take_next_triple(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(triple.0.id.0,"mutation:one");drop(triple);assert_eq!(owner.triples.len(),1); }
        member_receipt_close(&mut owner);let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] member receipt cancel turns={turns}: exact whole allocation denials, retained inverse/triples/backing/edit, partial original first triple stays source ordered");
    }
}
#[test]
fn mounted_member_receipt_refuses_missing_original_metadata_before_acknowledgment() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for refusal in 0..4 { let mut publication=member_receipt_fixture(&fixture,2,2);match refusal {0=>publication.metadata[0].mutation_id=None,1=>publication.metadata[0].group_id=Some("foreign-group".into()),2=>publication.metadata.clear(),_=>publication.metadata[0].group_id=Some("x".repeat(fixture["groupId"].as_str().unwrap().len()))};let source=member_receipt_source(&fixture,&publication,0);let mut owner=MountedMemberReceipt::new(&source).unwrap();if refusal == 3 { let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};assert_eq!(owner.next_capacity_byte_demand(&source).unwrap(),0);assert!(owner.advance(&source,grant).is_err()); } else { assert!(owner.next_capacity_byte_demand(&source).is_err()); } assert!(!owner.is_complete());assert_eq!(publication.forward.len(),2);member_receipt_close(&mut owner);println!("[DEBUG] member receipt refused original metadata case={refusal} before codec birth or acknowledgment; original publication owners unchanged"); }
}
