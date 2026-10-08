use super::*;

#[semio_framework_async_macros::async_test]
async fn child_emission_private_genesis_writer_streams_canonical_pages_and_retains_every_cancelled_owner(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🏪️store/🧩️composition/🚪️open/🌱️genesis/🧫️fixtures/🔣️.json")).unwrap();
    let reference=|value:&serde_json::Value|semio_framework_artifact_reference::ArtifactRef{artifact_id:value["artifact_id"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["dialect"]["artifact_kind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:0,maximum_depth:64};
    for row in fixture["cases"].as_array().unwrap(){
        let expected=reference(&row["expected"]);let owner=crate::store::OwnerRef{parent:reference(&row["owner"]["parent"]),slot:row["owner"]["slot"].as_str().unwrap().into(),child_id:row["owner"]["child_id"].as_str().unwrap().into()};
        let segment=row["initialPackHex"].as_str().unwrap().as_bytes().as_chunks::<2>().0.iter().map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect::<Vec<_>>();let pack=segment.repeat(row["packRepeats"].as_u64().unwrap()as usize);let pointer=pack.as_ptr();
        let source=crate::store::MemberGenesisEnvelopeSource{schema:row["schema"].as_str().unwrap(),expected:&expected,owner:&owner,initial_pack:&pack};
        let canonical=crate::store::genesis_member_envelope_pack(source.schema,&expected,&owner,&pack).await.unwrap();
        for stop in [0,1,3,17,65,257]{
            let mut cursor=PrivateChildGenesisInput::default();
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source,RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            for _ in 0..stop{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap.released_bytes,0);assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(pack.as_ptr(),pointer);if matches!(step,RetainedCloneStep::Complete(_)){break;}}
            cursor.begin_close();
            for _ in 0..100000{if cursor.terminal_is_empty(){break;}let demand=cursor.next_close_byte_demand();if demand>0{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_granted(RetainedCloneGrant{maximum_release_bytes:demand-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let exact=RetainedCloneGrant{maximum_release_bytes:demand,..grant};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_granted(exact).unwrap());assert!(step.progress().fits(exact));assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);}
            assert!(cursor.terminal_is_empty());
        }
        let mut cursor=PrivateChildGenesisInput::default();let mut ready=false;
        for _ in 0..100000{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap.released_bytes,0);assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);if matches!(step,RetainedCloneStep::Complete(_)){ready=true;break;}}
        assert!(ready);let mut pages=cursor.take_ready().unwrap();let mut joined=Vec::new();for index in 0..pages.page_count(){joined.extend_from_slice(pages.admitted_page(index).unwrap().as_slice());}assert_eq!(joined,canonical);let(decoded,_)=crate::store::decode_document_pack_bytes(&joined).await.unwrap();assert_eq!(decoded,pack);assert_eq!(pack.as_ptr(),pointer);
        while pages.close_take_page().is_some(){}let backing=pages.allocation_byte_demand();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages));assert_eq!(heap.released_bytes,backing);cursor.begin_close();while !cursor.terminal_is_empty(){let demand=cursor.next_close_byte_demand();cursor.close_granted(RetainedCloneGrant{maximum_release_bytes:demand,..grant}).unwrap();}
        println!("[DEBUG] private genesis assembled canonical case={} original={} encoded={} fixed-page-backing={} copy64 and six cancellation phases returned every exact allocation",row["id"],pack.len(),joined.len(),backing);
    }
}

#[test]
fn child_emission_private_input_writer_fills_only_admitted_inline_prefix() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let source=serde_json::to_vec(&fixture["source"].as_str().unwrap().repeat(257)).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:0,maximum_depth:64};
    let mut cursor=PrivateChildInputPages::default();let mut position=0;let mut ready=false;
    assert_eq!(cursor.prepare_output(source.len(),RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());
    for _ in 0..100000{
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.prepare_output(source.len(),grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap.released_bytes,0);assert!(heap.requested_bytes<=progress.retained_capacity_bytes);
        if matches!(step,RetainedCloneStep::Complete(_)){ready=true;break;}
        if progress!=Default::default(){continue;}
        let lent=cursor.writable_output(grant.maximum_copy_bytes).unwrap().len();assert!(cursor.commit_output(lent+1).is_err());
        let output=cursor.writable_output(grant.maximum_copy_bytes).unwrap();let copied=output.len().min(source.len()-position);assert!(copied<=64);output[..copied].copy_from_slice(&source[position..position+copied]);cursor.commit_output(copied).unwrap();position+=copied;
        if position==source.len(){cursor.finish_output().unwrap();}
    }
    assert!(ready);let mut pages=cursor.take_ready().unwrap();let mut joined=Vec::new();for index in 0..pages.page_count(){joined.extend_from_slice(pages.admitted_page(index).unwrap().as_slice());}
    assert_eq!(joined,source);assert_eq!(serde_json::from_slice::<String>(&joined).unwrap(),fixture["source"].as_str().unwrap().repeat(257));
    while pages.close_take_page().is_some(){}let bytes=pages.allocation_byte_demand();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages));assert_eq!(heap.released_bytes,bytes);assert!(cursor.terminal_is_empty());
    println!("[DEBUG] private genesis writer sink admitted exact{} UTF8/JSON bytes directly into original pages under64-byte output prefixes and whole{} slot release",position,bytes);
}

#[test]
fn child_emission_private_input_pages_preserve_exact_source_and_whole_backing() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:0,maximum_depth:64};
    for count in fixture["repeatCounts"].as_array().unwrap(){
        let original=fixture["source"].as_str().unwrap().repeat(count.as_u64().unwrap()as usize);
        let source=serde_json::to_vec(&original).unwrap();let pointer=source.as_ptr();
        for stop in fixture["cancelAt"].as_array().unwrap(){
            let(mut cursor,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(PrivateChildInputPages::default);assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));
            let paused=RetainedCloneGrant{maximum_items:0,..grant};
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_source(&source,paused).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            for _ in 0..stop.as_u64().unwrap(){
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_source(&source,grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert!(heap.requested_bytes<=progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,0);assert_eq!(source.as_ptr(),pointer);
                if matches!(step,RetainedCloneStep::Complete(_)){break;}
            }
            cursor.begin_close();
            for _ in 0..100000{
                if cursor.terminal_is_empty(){break;}
                let demand=cursor.next_close_byte_demand();
                if demand>0{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_granted(RetainedCloneGrant{maximum_release_bytes:demand-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
                let exact=RetainedCloneGrant{maximum_release_bytes:demand,..grant};
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_granted(exact).unwrap());assert!(step.progress().fits(exact));assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);
            }
            assert!(cursor.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(serde_json::from_slice::<String>(&source).unwrap(),original);
        }
        let mut cursor=PrivateChildInputPages::default();
        let mut ready=false;
        for _ in 0..100000{let step=cursor.advance_source(&source,grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){ready=true;break;}}
        assert!(ready);let mut pages=cursor.take_ready().unwrap();let mut joined=Vec::new();
        for index in 0..pages.page_count(){joined.extend_from_slice(pages.admitted_page(index).unwrap().as_slice());}
        assert_eq!(joined,source);assert_eq!(serde_json::from_slice::<String>(&joined).unwrap(),original);
        let page_count=pages.page_count();while pages.close_take_page().is_some(){}
        let backing=pages.allocation_byte_demand();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages));assert_eq!(heap.released_bytes,backing);
        assert!(cursor.terminal_is_empty());
        println!("[DEBUG] private genesis input serde bytes={} pages={} backing={} copy64 preserved original source and all6 cancellation stops",source.len(),page_count,backing);
    }
}
