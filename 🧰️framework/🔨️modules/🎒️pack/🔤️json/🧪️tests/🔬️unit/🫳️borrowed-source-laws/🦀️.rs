/// 🫳️ Borrowed source laws of the pack-json companion: the borrowed source and sink, the direct semantic source and the retirement and cancellation accounting.

#[test]
fn borrowed_json_source_sink_keeps_caller_prefix_policy_and_original_words(){
    let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🫳️source-sink.json")).unwrap();
    let text=fixture["word"].as_str().unwrap().repeat(fixture["repeats"].as_u64().unwrap()as usize);
    let source=DslValue::Object(vec![("first".into(),DslValue::String(text.clone())),("second".into(),DslValue::Array(vec![DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0),DslValue::float(2.5),DslValue::Bool(false),DslValue::Null,DslValue::Bytes(vec![0,127,255])]))]);
    let independent=serde_json::json!({"first":text,"second":[u64::MAX,i64::MIN,-0.0,2.5,false,null,[0,127,255]]}).to_string();
    assert_eq!(to_json_string(&source),independent);
    let pointer=match &source{DslValue::Object(entries)=>match &entries[0].1{DslValue::String(text)=>text.as_ptr(),_=>unreachable!()},_=>unreachable!()};
    let prefix=fixture["prefix"].as_str().unwrap().as_bytes();let maximum=fixture["maximumOutputBytes"].as_u64().unwrap();let depth=fixture["maximumDepth"].as_u64().unwrap()as usize;let items=fixture["maximumItems"].as_u64().unwrap();
    let mut output=Vec::with_capacity(prefix.len()+independent.len());output.extend_from_slice(prefix);
    let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accept);
    let length=write_json_source_into(&source,maximum,depth,items,&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{output.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap();
    assert_eq!(length,independent.len()as u64);assert_eq!(&output[..prefix.len()],prefix);assert_eq!(&output[prefix.len()..],independent.as_bytes());assert_eq!(control.owned_bytes(),0);
    for reason in ["policy","cancel","sink"]{
        output.truncate(prefix.len());
        let mut callback=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|reason!="cancel"||progress.completed<fixture["cancelAt"].as_u64().unwrap()as usize;
        let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut callback);
        let limit=if reason=="policy"{independent.len()as u64-1}else{maximum};
        let error=write_json_source_into(&source,limit,depth,items,&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{if reason=="sink"&&output.len()-prefix.len()+bytes.len()>fixture["sinkRefuseAt"].as_u64().unwrap()as usize{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"literal caller sink refusal"));}output.extend_from_slice(bytes);Ok(())},&mut control).unwrap_err();
        assert_eq!(error.kind,match reason{"policy"=>ValueRefusalKind::OwnershipLimit,"cancel"=>ValueRefusalKind::Canceled,_=>ValueRefusalKind::WorkLimit});
        assert_eq!(&output[..prefix.len()],prefix);assert!(output.len()-prefix.len()<independent.len());assert_eq!(&output[prefix.len()..],&independent.as_bytes()[..output.len()-prefix.len()]);assert_eq!(control.owned_bytes(),0);
    }
    let original=match &source{DslValue::Object(entries)=>match &entries[0].1{DslValue::String(text)=>text.as_ptr(),_=>unreachable!()},_=>unreachable!()};assert_eq!(original,pointer);
    let mut output=Vec::new();let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accept);
    assert_eq!(write_json_source_into(&source,maximum,1,items,&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{output.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap_err().kind,ValueRefusalKind::DepthLimit);
    let mut output=Vec::new();let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accept);
    assert_eq!(write_json_source_into(&source,maximum,depth,fixture["refusedCollectionItems"].as_u64().unwrap(),&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{output.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap_err().kind,ValueRefusalKind::WorkLimit);assert!(independent.as_bytes().starts_with(&output));
}


#[test]
fn retained_json_borrowed_source_uses_original_pages_utf8_and_refusal_owner(){
    use semio_framework_value::ValueRefusalKind;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_value_persistent::list::PagedList;
    let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🫳️read-source.json")).unwrap();
    let text=fixture["source"].as_str().unwrap();
    let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
    let pointer=source.get(0).unwrap()as *const u8;
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(fixture["decodeAllocationBytes"].as_u64().unwrap()as usize,&mut accepted);
    let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject,fixture_read_policy(&fixture)).unwrap();
    let parsed=loop{if let Some(value)=cursor.step(fixture["stepUnits"].as_u64().unwrap()as usize,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};
    assert_eq!(cursor.source().get(0).unwrap()as *const u8,pointer);
    assert_eq!(cursor.position(),source.len());
    let oracle:serde_json::Value=serde_json::from_str(text).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&parsed)).unwrap(),oracle);
    assert_eq!(parsed,parse(text,JsonMemberPolicy::Reject).unwrap());
    close_borrowed_json(cursor,131072);
    let large=format!("\"{}\"", "x".repeat(fixture["largeStringBytes"].as_u64().unwrap()as usize));
    let source=PagedList::<u8,16384>::try_from_iter(large.bytes()).unwrap();
    let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject,fixture_read_policy(&fixture)).unwrap();
    let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeDecodeControl::new(131072,&mut accepted);
    while cursor.phase()!="materialize-string"{assert!(cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}
    for _ in 0..64{assert!(cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}
    let bytes=control.owned_bytes();assert!(bytes>=fixture["largeStringBytes"].as_u64().unwrap()as usize);
    let position=cursor.position();let pointer=cursor.source().get(0).unwrap()as *const u8;
    live.set(false);assert_eq!(cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().into_value_error().kind,ValueRefusalKind::Canceled);
    assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),bytes);assert_eq!(cursor.source().get(0).unwrap()as *const u8,pointer);
    live.set(true);
    let actual=loop{if let Some(value)=cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};
    assert_eq!(actual,parse(&large,JsonMemberPolicy::Reject).unwrap());close_borrowed_json(cursor,131072);
    for bytes in[fixture["invalidUtf8"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>(),fixture["duplicate"].as_str().unwrap().as_bytes().to_vec()]{
        let source=PagedList::<u8,16384>::try_from_iter(bytes.iter().copied()).unwrap();let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject,fixture_read_policy(&fixture)).unwrap();
        let error=loop{match cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY){Err(error)=>break error,Ok(None)=>{},Ok(Some(_))=>panic!("invalid literal source accepted")}};
        if bytes[1]==237{assert!(matches!(error,JsonError::InvalidUtf8));}else{assert!(matches!(error,JsonError::DuplicateMember{..}));}
        close_borrowed_json(cursor,131072);
    }
}


#[test]
fn retained_json_direct_semantic_source_moves_admitted_original_cells_without_tree_mirror(){
    use semio_framework_value::{DslValue, Number};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_value_persistent::list::PagedList;
    let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🫳️read-source.json")).unwrap();
    let text=fixture["source"].as_str().unwrap();let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accepted);
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject,fixture_read_policy(&fixture)).unwrap();
    let value=loop{if let Some(value)=cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};
    assert_eq!(cursor.source().get(0).unwrap()as *const u8,source.get(0).unwrap()as *const u8);
    assert_eq!(serde_json::Value::from(&value),serde_json::from_str::<serde_json::Value>(text).unwrap());
    let DslValue::Object(fields)=&value else{panic!("original object")};
    let DslValue::Array(numbers)=&fields.iter().find(|(key,_)|key=="numbers").unwrap().1 else{panic!("original number array")};
    assert!(matches!(numbers[0],DslValue::Number(Number::UInt(u64::MAX))));
    assert!(matches!(numbers[1],DslValue::Number(Number::Int(i64::MIN))));
    assert!(matches!(numbers[2],DslValue::Number(Number::Float(value))if value.to_bits()==(-0.0f64).to_bits()));
    assert!(matches!(numbers[3],DslValue::Number(Number::Float(value))if value.to_bits()==1));
    close_borrowed_json(cursor,131072);
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'"').chain(std::iter::repeat_n(b'x',fixture["largeStringBytes"].as_u64().unwrap()as usize)).chain(std::iter::once(b'"'))).unwrap();
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject,fixture_read_policy(&fixture)).unwrap();let mut control=NativeDecodeControl::new(131072,&mut accepted);
    let value=loop{if let Some(value)=cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};
    assert_eq!(control.owned_bytes(),fixture["largeStringBytes"].as_u64().unwrap()as usize);
    let DslValue::String(value)=value else{panic!("direct owned semantic string")};assert_eq!(value.len(),fixture["largeStringBytes"].as_u64().unwrap()as usize);assert!(value.bytes().all(|byte|byte==b'x'));
    close_borrowed_json(cursor,131072);
}


#[test]
fn retained_json_borrowed_source_honors_complete_caller_extent_depth_and_paid_allocation(){
    use semio_framework_value::{DslValue, ValueRefusalKind};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_value_persistent::list::PagedList;
    let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🫳️read-limits.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let text=row["source"].as_str().unwrap();let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
        let limits=JsonReadLimits{maximum_bytes:text.len()as u64,maximum_allocation_bytes:fixture["decodeAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize,maximum_items:row["maximumItems"].as_u64().unwrap()};
        let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject,limits).unwrap();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(262144,&mut accepted);
        let result=loop{match cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY){Ok(Some(value))=>break Ok(value),Err(error)=>break Err(error),Ok(None)=>{}}};
        assert_eq!(control.maximum_bytes(),262144);
        if row["accept"].as_bool().unwrap(){assert_eq!(serde_json::Value::from(&result.unwrap()),serde_json::from_str::<serde_json::Value>(text).unwrap());}
        else{assert_eq!(result.unwrap_err().into_value_error().kind,match row["kind"].as_str().unwrap(){"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,_=>panic!("closed literal refusal")});}
        close_borrowed_json(cursor,131072);
        let short=JsonReadLimits{maximum_bytes:text.len()as u64-1,..limits};assert_eq!(JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject,short).err().unwrap().kind,ValueRefusalKind::WorkLimit);
    }
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'"').chain(std::iter::repeat_n(b'x',fixture["largeStringBytes"].as_u64().unwrap()as usize)).chain(std::iter::once(b'"'))).unwrap();
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(262144,&mut accepted);
    let limits=JsonReadLimits{maximum_bytes:source.len()as u64,maximum_allocation_bytes:fixture["refusedAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:8,maximum_items:8};
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject,limits).unwrap();
    let error=loop{match cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY){Err(error)=>break error,Ok(Some(_))=>panic!("original paid string must refuse"),Ok(None)=>{}}};
    assert_eq!(error.into_value_error().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);assert_eq!(control.maximum_bytes(),262144);
    assert_eq!(cursor.source().len(),source.len());close_borrowed_json(cursor,131072);
    let _=std::mem::size_of::<DslValue>();
}

#[test]
fn retained_json_cursor_owns_exact_borrowed_source_view(){
    use semio_framework_value::DslValue;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_value_persistent::list::PagedList;
    #[derive(Clone,Copy)]
    struct OriginalView<'source>{owner:&'source PagedList<u8,16384>,start:usize,length:usize}
    impl JsonReadSource for OriginalView<'_>{fn byte_len(&self)->usize{self.length}fn byte_at(&self,index:usize)->Option<u8>{(index<self.length).then(||self.owner.get(self.start+index).copied()).flatten()}}
    let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🫳️read-source.json")).unwrap();let text=fixture["source"].as_str().unwrap();
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'!').chain(text.bytes()).chain(std::iter::once(b'?'))).unwrap();
    let pointer=source.get(1).unwrap()as *const u8;
    let view=OriginalView{owner:&source,start:1,length:text.len()};
    let limits=JsonReadLimits{maximum_bytes:text.len()as u64,maximum_allocation_bytes:131072,maximum_depth:8,maximum_items:4};
    let mut cursor=JsonSourceCursor::<_,DslValue>::new(view,JsonMemberPolicy::Reject,limits).unwrap();
    let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeDecodeControl::new(262144,&mut accepted);
    for _ in 0..4{assert!(cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}
    let position=cursor.position();let owned=control.owned_bytes();live.set(false);assert_eq!(cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),owned);live.set(true);
    let value=loop{if let Some(value)=cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value}};
    assert_eq!(serde_json::Value::from(&value),serde_json::from_str::<serde_json::Value>(text).unwrap());
    assert_eq!(cursor.source_ref().owner.get(cursor.source_ref().start).unwrap()as *const u8,pointer);assert_eq!(cursor.source_ref().length,text.len());assert_eq!(control.maximum_bytes(),262144);
    close_borrowed_json(cursor,131072);
    assert_eq!(*source.get(0).unwrap(),b'!');assert_eq!(*source.get(source.len()-1).unwrap(),b'?');
}

fn close_borrowed_json<S:JsonReadSource+Copy,V:JsonParsedValue>(cursor:JsonSourceCursor<S,V>,maximum_bytes:usize){
    use semio_framework_value::{retained_clone::RetainedCloneGrant,close_factory_ticket};
    let grant=RetainedCloneGrant::one_capacity_turn(cursor.retirement_birth_bytes(),256);
    let(owner,birth)=cursor.into_retirement(grant).map_err(|(error,_)|error).unwrap();
    assert!(birth.fits(grant));
    let mut owner=Some(owner);
    for _ in 0..100000{
        let Some(cursor)=owner.as_ref()else{return;};
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:cursor.next_capacity_byte_demand(maximum_bytes).unwrap(),maximum_release_bytes:if cursor.terminal_is_empty(){std::mem::size_of_val(cursor.as_ref())}else{cursor.next_release_byte_demand().unwrap()},maximum_depth:if cursor.terminal_is_empty(){1}else{cursor.next_depth_demand().unwrap()}};
        assert!(close_factory_ticket(&mut owner,grant).unwrap().progress().fits(grant));
    }
    panic!("original borrowed JSON owner failed to retire");
}

#[test]
fn borrowed_json_retirement_admission_preserves_original_source_on_refusal(){
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    let law:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🎟️borrowed-retirement/🔣️.json")).unwrap();
    let source=law["source"].as_str().unwrap();
    for row in law["cases"].as_array().unwrap(){
        let mut cursor=JsonBorrowedParseCursor::new(source,JsonMemberPolicy::Reject,fixture_read_policy(&law)).unwrap();
        let mut accept=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accept);
        for _ in 0..4{assert!(cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}
        let position=cursor.position();let pointer=cursor.source().as_ptr();
        let birth=cursor.retirement_birth_bytes();
        let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:0,maximum_capacity_bytes:if row["capacity"].as_str().unwrap()=="exact"{birth}else{birth-1},maximum_release_bytes:0,maximum_depth:row["depth"].as_u64().unwrap()as usize};
        if row["accepted"].as_bool().unwrap(){
            let(owner,progress)=cursor.into_retirement(grant).map_err(|(error,_)|error).unwrap();assert!(progress.fits(grant));assert_eq!(progress.retained_capacity_bytes,birth);
            let mut owner=Some(owner);
            while let Some(cursor)=owner.as_ref(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:cursor.next_capacity_byte_demand(65536).unwrap(),maximum_release_bytes:if cursor.terminal_is_empty(){std::mem::size_of_val(cursor.as_ref())}else{cursor.next_release_byte_demand().unwrap()},maximum_depth:if cursor.terminal_is_empty(){1}else{cursor.next_depth_demand().unwrap()}};assert!(semio_framework_value::close_factory_ticket(&mut owner,grant).unwrap().progress().fits(grant));}
        }else{
            let(error,restored)=cursor.into_retirement(grant).err().expect("original handoff must refuse");cursor=restored;
            assert_eq!(error.kind.as_str(),row["kind"].as_str().unwrap());assert_eq!(cursor.position(),position);assert_eq!(cursor.source().as_ptr(),pointer);
            let parsed=loop{if let Some(value)=cursor.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};
            assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&parsed)).unwrap(),serde_json::from_str::<serde_json::Value>(source).unwrap());
            close_borrowed_json(cursor,65536);
        }
    }
}

#[test]
fn borrowed_json_writer_retains_cumulative_admission_across_real_observer_grants(){
    let fixture=owned_json_fixture();let law=&fixture["retainedWriting"];
    let source=DirectJsonSource{member:law["directSourceMember"].as_str().unwrap().into(),text:owned_json_text(law)};
    let pointer=source.text.as_ptr();let expected=serde_json::json!({source.member.clone():source.text.clone()}).to_string();
    let live=std::cell::Cell::new(true);let mut writer=JsonBorrowedWriteCursor::new();let mut receipt=None;let mut previous_owned=0;let mut turns=0;
    let output=loop{
        let mut observe=|_|live.get();let mut control=match receipt.take(){Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut observe).unwrap(),None=>semio_framework_value::NativeEncodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut observe)};
        let before=(writer.progress(),control.owned_bytes());assert!(writer.step(&source,0,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());assert_eq!((writer.progress(),control.owned_bytes()),before);
        let value=writer.step(&source,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap();assert!(control.owned_bytes()>=previous_owned);previous_owned=control.owned_bytes();receipt=Some(control.pause().unwrap());turns+=1;assert!(turns<100000);if let Some(value)=value{break value}
    };
    assert_eq!(output,expected);assert_eq!(source.text.as_ptr(),pointer);assert!(turns>1);
    retire_original_json(writer,3,100000);
    let mut writer=JsonBorrowedWriteCursor::new();let mut observe=|_|live.get();let mut control=semio_framework_value::NativeEncodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut observe);writer.step(&source,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap();let before=writer.progress();let receipt=control.pause().unwrap();live.set(false);let mut control=semio_framework_value::NativeEncodeControl::resume(receipt,&mut observe).unwrap();assert_eq!(writer.step(&source,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(writer.progress(),before);retire_original_json(writer,3,100000);retire_original_json(source,3,100000);
}

#[test]
fn original_json_normal_frontiers_preserve_actual_independent_receipts(){
    let law:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🎟️normal/🔣️.json")).unwrap();
    let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
    let zero=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
    for text in law["sources"].as_array().unwrap(){
        let text=text.as_str().unwrap();let pointer=text.as_ptr();let expected:serde_json::Value=serde_json::from_str(text).unwrap();
        let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted);let mut total=RetainedCloneProgress::default();let mut turns=0;
        let parsed=loop{
            turns+=1;assert!(turns<100000);let before=(parser.position(),parser.phase(),control.owned_bytes());
            let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||parser.step(text,1,&mut control,zero));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!((parser.position(),parser.phase(),control.owned_bytes()),before);assert_eq!(parser.normal_step_progress(),Default::default());
            let demand=parser.normal_step_demands(text).unwrap();
            for short in [if demand.capacity_bytes>0{Some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}else{None},if demand.copy_bytes>0{Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}else{None},if demand.release_bytes>0{Some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})}else{None}].into_iter().flatten(){
                let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||parser.step(text,1,&mut control,short));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!((parser.position(),parser.phase(),control.owned_bytes()),before);
            }
            let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||parser.step(text,1,&mut control,grant));let progress=parser.normal_step_progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,released));total=total.checked_add(progress).unwrap();assert_eq!(text.as_ptr(),pointer);
            if let Some(value)=result.unwrap(){break value;}
        };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&parsed)).unwrap(),expected);assert!(total.retained_capacity_bytes>0);retire_original_json(parser,3,100000);
        let mut projection=JsonValueProjection::new(parsed);let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted);let mut turns=0;
        let projected=loop{
            turns+=1;assert!(turns<100000);let before=(projection.phase(),control.owned_bytes());let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||projection.step(1,32,&mut control,zero));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!((projection.phase(),control.owned_bytes()),before);
            let demand=projection.normal_step_demands().unwrap();for short in [if demand.capacity_bytes>0{Some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}else{None},if demand.release_bytes>0{Some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})}else{None},if demand.copy_bytes>0{Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}else{None}].into_iter().flatten(){let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||projection.step(1,32,&mut control,short));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!((projection.phase(),control.owned_bytes()),before);}
            let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||projection.step(1,32,&mut control,grant));let progress=projection.normal_step_progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,released));if let Some(value)=result.unwrap(){break value;}
        };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&projected)).unwrap(),expected);retire_original_json(projection,3,100000);
        let mut writer=JsonWriteCursor::new(projected);let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new_retained(&mut accepted);let mut turns=0;
        let output=loop{
            turns+=1;assert!(turns<100000);let before=(writer.progress(),control.owned_bytes());let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||writer.step(1,&mut control,zero));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!((writer.progress(),control.owned_bytes()),before);
            let demand=writer.normal_step_demands().unwrap();for short in [if demand.capacity_bytes>0{Some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}else{None},if demand.copy_bytes>0{Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}else{None}].into_iter().flatten(){let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||writer.step(1,&mut control,short));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!((writer.progress(),control.owned_bytes()),before);}
            let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||writer.step(1,&mut control,grant));let progress=writer.normal_step_progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,released));if let Some(value)=result.unwrap(){break value;}
        };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(),expected);let original=writer.take_source().unwrap();retire_original_json(original,3,100000);retire_original_json(writer,3,100000);
    }
}

#[test]
fn borrowed_json_writer_preserves_actual_post_birth_cancellation_receipt(){
 let source=DslValue::String("雪".repeat(16));let expected=serde_json::to_string(&"雪".repeat(16)).unwrap();
 let live=std::cell::Cell::new(true);let cancel_at_turn=std::cell::Cell::new(true);
 let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|live.get()&&!(cancel_at_turn.get()&&event.completed==1);
 let mut control=semio_framework_value::NativeEncodeControl::new(1<<20,&mut callback);control.begin_stage(1).unwrap();let mut writer=JsonBorrowedWriteCursor::new();
 let (result,born,released)=semio_framework_trace_memory_testing::observe_backing(||writer.step(&source,1,&mut control,JSON_NORMAL_TEST_POLICY));
 let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);let progress=writer.normal_step_progress();assert_eq!(error.retained_progress(),progress);assert_eq!(progress.copied_items,1);assert_eq!(progress.copied_bytes,0);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);assert_eq!(progress.released_bytes,0);assert_eq!(writer.retained_capacity_bytes().unwrap(),born);
 let pointers=(writer.frames.as_ptr(),writer.path.as_ptr());let frontier=writer.progress();let mut denied=JSON_NORMAL_TEST_POLICY;denied.maximum_items=0;
 let (empty,born,released)=semio_framework_trace_memory_testing::observe_backing(||writer.step(&source,1,&mut control,denied));assert!(empty.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!(writer.progress(),frontier);assert_eq!((writer.frames.as_ptr(),writer.path.as_ptr()),pointers);
 cancel_at_turn.set(false);control.begin_stage(0).unwrap();let actual=loop{if let Some(output)=writer.step(&source,4,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break output}};
 assert_eq!(actual,expected);assert_eq!(serde_json::from_str::<serde_json::Value>(&actual).unwrap(),serde_json::Value::String("雪".repeat(16)));
 retire_original_json(writer,3,100000);retire_original_json(source,3,100000);retire_original_json(actual,3,100000);
}
