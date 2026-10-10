use super::*;
use std::cell::Cell;

#[test]
fn json_failed_original_frontier_retains_only_actual_dsl_effects(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️frontier.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let preparation=grant(&policy["preparationGrant"]);let closing=grant(&policy["closeGrant"]);let maximum_turns=law["maximumTurns"].as_u64().unwrap();
    for row in law["refusals"].as_array().unwrap(){
        let allocation=row["kind"]=="allocationPort";let source=law["sources"][if allocation{0}else{3}].as_str().unwrap();let mut cursor=JsonGrammarCursor::<DslValue>::new(JsonMemberPolicy::Reject);let refused=std::sync::atomic::AtomicBool::new(false);let mut accepted=|_|true;
        let mut allocation_port=|_:semio_framework_value::native_decoding::NativeDecodeAllocation|if refused.load(std::sync::atomic::Ordering::Relaxed){Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original JSON allocation port refused"))}else{Ok(())};let mut control=semio_framework_value::NativeDecodeControl::new_forwarded(1<<20,&mut accepted,&mut allocation_port);let mut prepared=false;
        for _ in 0..maximum_turns{let demand=cursor.normal_step_demands(source).unwrap();prepared=if allocation{cursor.phase()=="materialize-string"&&demand.capacity_bytes==4}else{cursor.position()==source.len()-1};if prepared{break;}assert!(cursor.step(source,1,&mut control,preparation).unwrap().is_none());}
        assert!(prepared);refused.store(allocation,std::sync::atomic::Ordering::Relaxed);let position=cursor.position();let owned=control.owned_bytes();let incoming=if allocation{preparation}else{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}};
        let(result,born,released)=test_allocation::observe_backing(||cursor.step(source,1,&mut control,incoming));let error=result.unwrap_err();assert_eq!((born,released),(0,0));assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),owned);
        let expected=RetainedCloneProgress{copied_items:row["copiedItems"].as_u64().unwrap()as usize,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0};assert_eq!(cursor.normal_step_progress(),expected);assert!(expected.fits(incoming));
        if allocation{let JsonError::Native(error)=error else{unreachable!()};assert_eq!(error.kind,ValueRefusalKind::AllocationFailed);assert_eq!(error.retained_progress(),expected);let Some(JsonLexeme::String(scan))=&cursor.lexeme else{unreachable!()};assert_eq!((scan.output.len(),scan.output.capacity()),(0,0));}else{assert!(matches!(error,JsonError::TrailingData(_)));let Some(DslValue::String(value))=&cursor.result else{unreachable!()};assert_eq!(value.as_bytes(),serde_json::from_str::<String>(law["sources"][0].as_str().unwrap()).unwrap().as_bytes());}
        close(JsonSourceCursor::from_grammar(source,cursor),closing,maximum_turns);
    }
    eprintln!("[DEBUG] Original DslValue JSON allocation and syntax refusals preserve actual frontier work with no forecast backing or copy receipt");
}

#[test]
fn json_dsl_original_receipt_conserves_prefix_backing_release_and_depth(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️frontier.json")).unwrap();let closing=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:256};let maximum_turns=law["maximumTurns"].as_u64().unwrap();
    let source=law["sources"][3].as_str().unwrap();let mut cursor=JsonGrammarCursor::<DslValue>::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new_retained(&mut accepted);let incoming=RetainedCloneGrant{maximum_items:32,..closing};
    let(result,born,released)=test_allocation::observe_backing(||cursor.step(source,32,&mut control,incoming));assert!(matches!(result,Err(JsonError::TrailingData(_))));let receipt=cursor.normal_step_progress();assert_eq!(receipt,RetainedCloneProgress{copied_items:law["cumulativeCopiedItems"].as_u64().unwrap()as usize,copied_bytes:4,retained_capacity_bytes:4,released_bytes:0});assert_eq!((born,released),(receipt.retained_capacity_bytes,receipt.released_bytes));let Some(DslValue::String(prefix))=&cursor.result else{unreachable!()};assert_eq!(prefix,"😀");close(JsonSourceCursor::from_grammar(source,cursor),closing,maximum_turns);
    for source in law["sources"].as_array().unwrap().iter().take(3){
        let source=source.as_str().unwrap();let original=source.as_ptr();let expected:serde_json::Value=serde_json::from_str(source).unwrap();let mut cursor=JsonGrammarCursor::<DslValue>::new(JsonMemberPolicy::Replace);let armed=Cell::new(false);let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|!(armed.get()&&event.completed==1&&event.total==1);let mut control=semio_framework_value::NativeDecodeControl::new_retained(&mut callback);let mut canceled=false;let mut page_depth=false;let mut cumulative=RetainedCloneProgress::default();let mut output=None;
        for _ in 0..maximum_turns{
            let demand=cursor.normal_step_demands(source).unwrap();let before=(cursor.position(),cursor.phase(),control.owned_bytes());if demand.depth>=law["physicalDepth"].as_u64().unwrap()as usize{page_depth=true;let short=RetainedCloneGrant{maximum_depth:demand.depth-1,..closing};let(result,born,released)=test_allocation::observe_backing(||cursor.step(source,1,&mut control,short));assert!(matches!(result,Err(JsonError::Native(error))if error.kind==ValueRefusalKind::DepthLimit));assert_eq!((born,released),(0,0));assert_eq!(cursor.normal_step_progress(),Default::default());assert_eq!((cursor.position(),cursor.phase(),control.owned_bytes()),before);}
            let cancel=demand.release_bytes>0&&!canceled;if cancel{control.begin_stage(1).unwrap();armed.set(true);}
            let(result,born,released)=test_allocation::observe_backing(||cursor.step(source,1,&mut control,closing));let receipt=cursor.normal_step_progress();assert!(receipt.fits(closing));assert_eq!(source.as_ptr(),original);assert_eq!(receipt.released_bytes,released);cumulative=cumulative.checked_add(receipt).unwrap();
            if cancel{let Err(JsonError::Native(error))=result else{unreachable!()};assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(error.retained_progress(),receipt);let diagnostic=match error.message{std::borrow::Cow::Owned(text)=>text.capacity(),std::borrow::Cow::Borrowed(_)=>0};assert_eq!(born,receipt.retained_capacity_bytes+diagnostic);assert_eq!(receipt.copied_bytes,0);assert_eq!(receipt.released_bytes,demand.release_bytes);armed.set(false);control.begin_stage(0).unwrap();canceled=true;}else{assert_eq!(born,receipt.retained_capacity_bytes);if let Some(value)=result.unwrap(){output=Some(value);break;}}
        }
        let output=output.expect("original DslValue parser finishes under authored turns");assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&output)).unwrap(),expected);if source.starts_with('[')||source.starts_with('{'){assert!(page_depth);assert!(canceled);assert!(cumulative.released_bytes>0);}assert!(cumulative.copied_bytes>=4);close(JsonSourceCursor::from_grammar(source,cursor),closing,maximum_turns);
    }
    eprintln!("[DEBUG] Original DslValue JSON prefixes and physical candidate births match actual allocator receipts; exact page depth refuses before effects and page release cancellation settles once");
}

#[test]
fn json_original_error_retirement_keeps_actual_duplicate_name_and_native_diagnostic(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️frontier.json")).unwrap();let source=law["sources"][2].as_str().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:256};let maximum_turns=law["maximumTurns"].as_u64().unwrap();let mut parser=JsonGrammarCursor::<DslValue>::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new_retained(&mut accepted);let mut pointer=None;
    for _ in 0..maximum_turns{if let Some(frame)=parser.frames.last().filter(|frame|frame.state==6&&frame.entries.len()==1&&frame.compare==frame.key.as_ref().unwrap().len()){pointer=frame.key.as_ref().map(|name|name.as_ptr());break;}assert!(parser.step(source,1,&mut native,grant).unwrap().is_none());}
    let pointer=pointer.unwrap();let(result,born,released)=test_allocation::observe_backing(||parser.step(source,1,&mut native,grant));assert_eq!((born,released),(0,0));let error=result.unwrap_err();let JsonError::DuplicateMember{name,..}=&error else{unreachable!()};assert_eq!(name.as_ptr(),pointer);assert_eq!(name,serde_json::from_str::<serde_json::Value>(source).unwrap().as_object().unwrap().keys().next().unwrap());let held=name.capacity();close(JsonSourceCursor::from_grammar(source,parser),grant,maximum_turns);
    let diagnostics=[(error,held),(JsonError::Native(ValueError::new(ValueRefusalKind::InvalidValue,source)),source.len()),(JsonError::UnexpectedEof,0),(JsonError::InvalidUtf8,0),(JsonError::UnexpectedByte{found:b'x',offset:7},0),(JsonError::InvalidEscape(3),0),(JsonError::MaxDepthExceeded(1),0)];
    for(error,held)in diagnostics{let mut owner=Some(semio_framework_value::admit_owned_retirement(error,grant).map_err(|(error,_)|error).unwrap().0);let mut allocated=0;let mut released=0;for _ in 0..maximum_turns{if owner.is_none(){break;}let(result,born,freed)=test_allocation::observe_backing(||semio_framework_value::close_factory_ticket(&mut owner,grant));let step=result.unwrap();assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));allocated+=born;released+=freed;}assert!(owner.is_none());assert_eq!(released,held+allocated+semio_framework_value::owned_retirement_birth_bytes::<JsonError>());}
    eprintln!("[DEBUG] Original duplicate name pointer and Native diagnostic retire without formatting or clones; actual scalar diagnostic fields use their genuine primitive leaf owners");
}

fn grant(value:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize}}

fn close<S:JsonReadSource+Copy,V:JsonParsedValue>(cursor:JsonSourceCursor<S,V>,grant:RetainedCloneGrant,maximum_turns:u64){
    let(owner,birth)=cursor.into_retirement(grant).map_err(|(error,_)|error).unwrap();assert!(birth.fits(grant));let mut owner=Some(owner);
    for _ in 0..maximum_turns{if owner.is_none(){return;}let step=semio_framework_value::close_factory_ticket(&mut owner,grant).unwrap();assert!(step.progress().fits(grant));}
    panic!("original JSON cancellation owner did not close under authored authority");
}

#[test]
fn json_refusal_receipt_preserves_actual_work_before_cancellation(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let text=fixture["source"].as_str().unwrap();let limits=&fixture["limits"];
    let limits=JsonReadLimits{maximum_bytes:limits["maximumBytes"].as_u64().unwrap(),maximum_allocation_bytes:limits["maximumAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:limits["maximumDepth"].as_u64().unwrap()as usize,maximum_items:limits["maximumItems"].as_u64().unwrap()};
    let preparation=grant(&fixture["preparationGrant"]);let closing=grant(&fixture["closeGrant"]);let maximum_turns=fixture["maximumTurns"].as_u64().unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let mut cursor=JsonSourceCursor::<_,Value>::new(text,JsonMemberPolicy::Reject,limits).unwrap();let armed=Cell::new(false);let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|!(armed.get()&&event.completed==1&&event.total==1);let mut control=semio_framework_value::NativeDecodeControl::new(limits.maximum_allocation_bytes,&mut callback);let frontier=row["frontier"].as_str().unwrap();let mut prepared=false;
        for _ in 0..maximum_turns{
            let demand=cursor.normal_step_demands().unwrap();prepared=match frontier{"utf8Scan"=>cursor.position()==0,"stringCapacity"=>cursor.phase()=="materialize-string"&&demand.capacity_bytes==4,"stringCopy"=>cursor.phase()=="materialize-string"&&demand.copy_bytes==1,_=>unreachable!()};
            if prepared{break;}assert!(cursor.step(1,&mut control,preparation).unwrap().is_none());
        }
        assert!(prepared);control.begin_stage(1).unwrap();armed.set(true);let original_source=cursor.source().as_ptr();let incoming=grant(&row["grant"]);let expected=&row["receipt"];
        let expected=RetainedCloneProgress{copied_items:expected["copiedItems"].as_u64().unwrap()as usize,copied_bytes:expected["copiedBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:expected["retainedCapacityBytes"].as_u64().unwrap()as usize,released_bytes:expected["releasedBytes"].as_u64().unwrap()as usize};
        let(result,born,released)=test_allocation::observe_backing(||cursor.step(1,&mut control,incoming));let error=result.unwrap_err();
        assert!(matches!(&error,JsonError::Native(error)if error.kind==ValueRefusalKind::Canceled));
        let diagnostic_bytes=match &error{JsonError::Native(error)=>match &error.message{std::borrow::Cow::Owned(text)=>text.capacity(),std::borrow::Cow::Borrowed(_)=>0},_=>unreachable!()};
        assert_eq!((born,released),(expected.retained_capacity_bytes+diagnostic_bytes,0));assert_eq!(cursor.source().as_ptr(),original_source);
        assert_eq!(cursor.normal_step_progress(),expected,"{frontier}");assert!(expected.fits(incoming));
        let JsonError::Native(refusal)=&error else{unreachable!()};assert_eq!(refusal.retained_progress(),expected,"{frontier}");
        let((source,grammar),born,released)=test_allocation::observe_backing(||cursor.into_grammar());assert_eq!((born,released),(0,0));assert_eq!(source.as_ptr(),original_source);
        if frontier=="stringCopy"{let Some(JsonLexeme::String(scan))=&grammar.lexeme else{unreachable!()};assert_eq!(scan.output.as_slice(),&serde_json::from_str::<String>(text).unwrap().as_bytes()[..expected.copied_bytes]);}
        if frontier=="stringCapacity"{let Some(JsonLexeme::String(scan))=&grammar.lexeme else{unreachable!()};assert_eq!(scan.output.capacity(),expected.retained_capacity_bytes);assert!(scan.output.is_empty());}
        close(JsonSourceCursor::from_grammar(source,grammar),closing,maximum_turns);
    }
    eprintln!("[DEBUG] JSON real scan, capacity birth and UTF8 copy remain in original caller receipt after post-effect cancellation; actual allocator diagnostic body is counted separately and remains an unpriced Native-control frontier");
}
