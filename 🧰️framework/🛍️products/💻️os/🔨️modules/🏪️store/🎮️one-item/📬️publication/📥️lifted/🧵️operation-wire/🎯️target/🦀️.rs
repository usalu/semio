//! 🧪️ Explicit native target authority borrows actual authored segment storage.
use super::*;
#[test]
fn original_targets_preserve_authored_path_without_fallback(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){let segments=row["segments"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect::<Vec<_>>();let target:&dyn ArtifactOperationTargets=&segments;assert_eq!(target.len(),segments.len());for(index,text)in segments.iter().enumerate(){assert_eq!(target.segment_len(index).unwrap(),text.len());let mut buffer=[0;3];let mut offset=0;while offset<text.len(){let count=target.read_segment(index,offset,&mut buffer).unwrap();assert_eq!(&buffer[..count],&text.as_bytes()[offset..offset+count]);offset+=count;}}assert!(target.segment_len(segments.len()).is_err());assert_eq!(segments.is_empty(),row["whole"].as_bool().unwrap());}
    let absent=ArtifactOperationWire::<u8>::canonical();assert!(absent.targets(&0).is_err());let explicit=absent.with_original_target(whole_document_target::<u8>);assert!(explicit.targets(&0).unwrap().is_none());
}

#[test]
fn installed_original_history_digest_preserves_borrowed_staged_inverse(){
    use semio_framework_value::FromValue;
    use semio_framework_os_spr::Edit;
    use semio_framework_os_store_ledger::native_digest::NativeEditDigest;
    use semio_framework_value_persistent::list::PagedList;
    fn step<M>(wire:ArtifactOperationWire<M>,cursor:&mut NativeEditDigest,edit:&Edit<M>,inverse:&PagedList<M,{usize::MAX}>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{wire.advance_history_digest(cursor,edit,inverse,grant)}
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../📒️ledger/🔏️digest/🔣️.json" )).unwrap();
    for case in cases.as_array().unwrap(){
        let value=semio_framework_pack_json::from_json_str(&serde_json::to_string(&case["edit"]).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut edit=Edit::<bool>::from_value(value).unwrap();
        let inverse=std::mem::take(&mut *edit.inverse);edit.inverse.push(true);edit.inverse.push(false);
        let wire=ArtifactOperationWire::<bool>::canonical();let mut cursor=NativeEditDigest::default();
        let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
        for denied in [RetainedCloneGrant{maximum_items:0,..full},RetainedCloneGrant{maximum_copy_bytes:0,..full},RetainedCloneGrant{maximum_depth:0,..full}]{let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||step(wire,&mut cursor,&edit,&inverse,denied).unwrap());assert_eq!(p,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(cursor.digest().is_none());}
        for _ in 0..100000{let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||step(wire,&mut cursor,&edit,&inverse,full).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(p.copied_bytes<=4096);assert_eq!(edit.inverse.len(),2);if cursor.digest().is_some(){break}}
        let expected=case["digest"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();assert_eq!(cursor.digest().unwrap().as_slice(),expected);
    }
}
