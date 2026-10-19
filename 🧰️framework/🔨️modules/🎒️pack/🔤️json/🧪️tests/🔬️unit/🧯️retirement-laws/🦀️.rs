/// 🧯️ Retirement laws of the pack-json companion: the retained writer, projection and parser admission, resumption, cancellation and controlled retirement against the independent grammar.

fn fixture_read_policy(fixture:&serde_json::Value)->JsonReadLimits{
    let limits=&fixture["readLimits"];JsonReadLimits{maximum_bytes:limits["maximumBytes"].as_u64().unwrap(),maximum_allocation_bytes:limits["maximumAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:limits["maximumDepth"].as_u64().unwrap()as usize,maximum_items:limits["maximumItems"].as_u64().unwrap()}
}

const JSON_NORMAL_TEST_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096};

#[test]
fn completed_json_projection_frame_reuses_inline_authority_without_transient_admission(){
    let law:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/📦️completed-frame/🔣️.json")).unwrap();let source=law["source"].as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(source).unwrap();let mut projection=JsonValueProjection::new(parse(source,JsonMemberPolicy::Reject).unwrap());let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if projection.retirement.is_some(){break;}assert!(projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}assert!(projection.retirement.is_some());let admitted=control.owned_bytes();let mut released=0;
    let original={let owner=projection.retirement.as_mut().unwrap();use semio_framework_value::{retirement::{RetirementCursor,RetirementStep},retained_clone::RetainedCloneGrant};let phase=owner.phase;let demand=owner.release_demand();assert!(demand>3);for grant in[RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:3,maximum_capacity_bytes:0,maximum_release_bytes:demand,maximum_depth:1},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:0,maximum_release_bytes:demand-1,maximum_depth:1}]{let(step,born,physical)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(grant));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((born,physical),(0,0));assert_eq!(owner.phase,phase);assert_eq!(owner.release_demand(),demand);}owner.frame.values.allocation+owner.frame.entries.allocation+owner.frame.array.capacity()*std::mem::size_of::<DslValue>()+owner.frame.object.capacity()*std::mem::size_of::<(String,DslValue)>()};
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if projection.retirement.is_none(){break;}let(result,born,physical)=semio_framework_trace_memory_testing::observe_backing(||projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY));assert!(result.unwrap().is_none());assert_eq!(born,0,"completed input backing needs no fresh retirement scaffold");assert_eq!(control.owned_bytes(),admitted,"original source admission is conserved through physical input release");released+=physical;}
    assert!(projection.retirement.is_none());assert_eq!(released,original);let value=projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().expect("original output survives physical source close");assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&value)).unwrap(),expected);retire_original_json(projection,3,65536);retire_original_json(value,3,65536);
}

fn retire_json_with_independent_grants<T: semio_framework_value::retirement::RetireOwned>(value: T, law: &serde_json::Value) {
    use semio_framework_value::{retirement::controlled::ControlledRetirement, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
    let mut owner = match ControlledRetirement::new(value) { Ok(owner) => owner, Err((error, value)) => { std::mem::forget(value); panic!("original JSON owner lacks controlled authority: {error}"); } };
    for turn in 0..law["maximumTurns"].as_u64().unwrap() {
        let grant = RetainedCloneGrant { maximum_items: law["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: law["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: owner.next_capacity_byte_demand(3).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
        assert!(grant.maximum_depth <= law["maximumDepth"].as_u64().unwrap() as usize);
        assert_eq!(owner.step(RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 }).unwrap().progress(), Default::default());
        assert_eq!(owner.next_capacity_byte_demand(3).unwrap(), grant.maximum_capacity_bytes);
        assert_eq!(owner.next_release_byte_demand().unwrap(), grant.maximum_release_bytes);
        assert_eq!(owner.next_depth_demand().unwrap(), grant.maximum_depth);
        match owner.step(grant).unwrap() {
            RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => {
                assert!(progress.copied_items <= grant.maximum_items && progress.copied_bytes <= grant.maximum_copy_bytes);
                assert!(progress.retained_capacity_bytes <= grant.maximum_capacity_bytes && progress.released_bytes <= grant.maximum_release_bytes);
            }
        }
        if owner.terminal_is_empty() {  return; }
    }
    panic!("original JSON controlled retirement did not terminate: owner={} copyDemand={} grantedCopy={} capacityDemand={} releaseDemand={} depthDemand={}",std::any::type_name::<T>(),owner.next_copy_byte_demand().unwrap(),law["maximumCopyBytes"],owner.next_capacity_byte_demand(3).unwrap(),owner.next_release_byte_demand().unwrap(),owner.next_depth_demand().unwrap());
}

#[test]
fn original_json_parser_writer_and_value_retire_with_independent_physical_grants() {
    let law: serde_json::Value = serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🎮️retirement/🔣️.json")).unwrap();
    let source = law["source"].as_str().unwrap();
    let expected: serde_json::Value = serde_json::from_str(source).unwrap();
    let value = parse(source, JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&value)).unwrap(), expected);
    retire_json_with_independent_grants(value, &law);
    for stop in law["cancelUnits"].as_array().unwrap() {
        let mut parser = JsonParseCursor::new(JsonMemberPolicy::Reject);
        let mut accepted = |_| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1_000_000, &mut accepted);
        for _ in 0..stop.as_u64().unwrap() { if let Some(value) = parser.step(source, 1, &mut control,JSON_NORMAL_TEST_POLICY).unwrap() { retire_json_with_independent_grants(value, &law); break; } }
        retire_json_with_independent_grants(parser, &law);
        let mut writer = JsonWriteCursor::new(from_json_str::<semio_framework_value::DslValue>(source, JsonMemberPolicy::Reject).unwrap());
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(1_000_000, &mut accepted);
        for _ in 0..stop.as_u64().unwrap() { if writer.step(1, &mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_some() { break; } }
        retire_json_with_independent_grants(writer, &law);
        let mut projection = JsonValueProjection::new(parse(source, JsonMemberPolicy::Reject).unwrap());
        let mut accepted = |_| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1_000_000, &mut accepted);
        for _ in 0..stop.as_u64().unwrap() { if let Some(value) = projection.step(1, 3, &mut control,JSON_NORMAL_TEST_POLICY).unwrap() { retire_json_with_independent_grants(value, &law); break; } }
        retire_json_with_independent_grants(projection, &law);
    }
}

fn owned_json_fixture()->serde_json::Value{serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🚦️owned-controls.json")).unwrap()}
fn retire_original_json<T:semio_framework_value::retirement::RetireOwned>(value:T,maximum_copy_bytes:usize,maximum_turns:u64){let law=serde_json::json!({"maximumItems":1,"maximumCopyBytes":maximum_copy_bytes,"maximumDepth":4096,"maximumTurns":maximum_turns});retire_json_with_independent_grants(value,&law);}
fn owned_json_text(f:&serde_json::Value)->String{f["textUnit"].as_str().unwrap().repeat(f["textRepeats"].as_u64().unwrap() as usize)}

struct DirectJsonSource{member:String,text:String}
impl JsonWriteSource for DirectJsonSource{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{match path{[]=>Ok(JsonWriteNode::Object(1)),[0]=>Ok(JsonWriteNode::String(&self.text)),_=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"direct authored source path"))}}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty()&&index==0{Ok(&self.member)}else{Err(ValueError::new(ValueRefusalKind::InvariantViolated,"direct authored source key"))}}
}
impl semio_framework_value::retirement::RetireOwned for DirectJsonSource{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(self.member),deferred(self.text)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.member),deferred_birth_bytes_for(&self.text)])}
    fn controlled_retirement_supported()->bool{true}
}
#[test]
fn retained_json_writer_reads_original_owned_source_without_projected_copy(){
    let fixture=owned_json_fixture();let law=&fixture["retainedWriting"];let source=||DirectJsonSource{member:law["directSourceMember"].as_str().unwrap().into(),text:owned_json_text(law)};let expected=serde_json::json!({law["directSourceMember"].as_str().unwrap():owned_json_text(law)}).to_string();
    let retire=|writer:JsonWriteCursor<DirectJsonSource>|retire_original_json(writer,3,100000);
    for budget in law["budgets"].as_array().unwrap(){let original=source();let pointer=original.text.as_ptr();let mut writer=JsonWriteCursor::new(original);assert!(writer.take_source().is_none());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let mut turns=0;let output=loop{let before=writer.progress();assert!(writer.step(0,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());assert_eq!(writer.progress(),before);turns+=1;assert!(turns<100000);if let Some(output)=writer.step(budget.as_u64().unwrap()as usize,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break output}assert!(writer.take_source().is_none());};assert!(turns>1);assert_eq!(output,expected);let original=writer.take_source().unwrap();assert!(writer.take_source().is_none());assert_eq!(original.text.as_ptr(),pointer);retire(writer);retire_original_json(original,3,100000);}
    for stop in [0,1,32,512,4096]{let mut writer=JsonWriteCursor::new(source());let live=std::cell::Cell::new(true);let mut callback=|_|live.get();let mut control=semio_framework_value::NativeEncodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut callback);for _ in 0..stop{assert!(writer.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none())}let before=writer.progress();live.set(false);assert_eq!(writer.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(writer.progress(),before);retire(writer);}
}

#[test]
fn retained_json_projection_admits_exact_vectors_and_drains_consumed_input() {
    let fixture=owned_json_fixture();let law=&fixture["retainedProjectionAdmission"];let source=serde_json::json!({"values":(0..law["collectionItems"].as_u64().unwrap()).map(|index|serde_json::json!({"index":index,"label":format!("Label 😀 {index}")})).collect::<Vec<_>>()}).to_string();let oracle:serde_json::Value=serde_json::from_str(&source).unwrap();
    let input=||parse(&source,JsonMemberPolicy::Reject).unwrap();let mut projection=JsonValueProjection::new_ordered(input());let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let mut phases=std::collections::BTreeSet::new();let mut turns=0;
    let value=loop{assert!(projection.step(0,0,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());phases.insert(projection.phase());turns+=1;assert!(turns<2000000);if let Some(value)=projection.step(law["maximumUnits"].as_u64().unwrap()as usize,law["retirementBytes"].as_u64().unwrap()as usize,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&value)).unwrap(),oracle);assert!(projection.retirement.is_none());
    for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()),"{phase}");}
    let retire=|projection:JsonValueProjection|retire_original_json(projection,3,4000000);retire(projection);retire_original_json(value,3,4000000);
    let mut projection=JsonValueProjection::new(input());let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1,&mut accepted);assert_eq!(projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);retire(projection);
    for stop in law["cancelUnits"].as_array().unwrap(){let mut projection=JsonValueProjection::new(input());let canceled=std::cell::Cell::new(false);let mut accepted=|_|!canceled.get();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);for _ in 0..stop.as_u64().unwrap(){assert!(projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}canceled.set(true);assert_eq!(projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind,ValueRefusalKind::Canceled);retire(projection);}
}

#[test]
fn retained_json_parser_admits_storage_before_copying_and_resumes_one_owner() {
    let fixture=owned_json_fixture();let law=&fixture["retainedAdmission"];let label=owned_json_text(law);let length=label.len();let text=serde_json::to_string(&label).unwrap();
    let retire=|parser:JsonParseCursor|retire_original_json(parser,law["retirementBytes"].as_u64().unwrap()as usize,4000000);
    let mut phases=std::collections::BTreeSet::new();
    for (source,maximum) in [(text.clone(),length),(serde_json::to_string(&vec![label.clone();law["collectionItems"].as_u64().unwrap()as usize/128]).unwrap(),law["maximumBytes"].as_u64().unwrap()as usize),(serde_json::json!({"labels":(0..law["collectionItems"].as_u64().unwrap()).collect::<Vec<_>>(),"object":(0..128).map(|index|(format!("key{index}"),serde_json::Value::from(index))).collect::<serde_json::Map<_,_>>(),"label":label}).to_string(),law["maximumBytes"].as_u64().unwrap()as usize)] {
        let oracle:serde_json::Value=serde_json::from_str(&source).unwrap();let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(maximum,&mut accepted);let mut turns=0;
        let value=loop{let before=(parser.position(),control.owned_bytes());assert!(parser.step(&source,0,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());assert_eq!((parser.position(),control.owned_bytes()),before);phases.insert(parser.phase());turns+=1;assert!(turns<4000000);if let Some(value)=parser.step(&source,law["maximumUnits"].as_u64().unwrap()as usize,&mut control,JSON_NORMAL_TEST_POLICY).unwrap(){break value;}};
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&value)).unwrap(),oracle);assert!(control.owned_bytes()<=maximum);if source==text {assert_eq!(control.owned_bytes(),length);}retire(parser);retire_original_json(value,3,4000000);
    }
    for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()),"{phase}");}
    let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(length-1,&mut accepted);let error=loop{match parser.step(&text,1,&mut control,JSON_NORMAL_TEST_POLICY){Ok(None)=>{},Ok(Some(_))=>panic!("ownership limit admitted a string"),Err(error)=>break error}};assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);retire(parser);
    for stop in law["cancelUnits"].as_array().unwrap(){let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let canceled=std::cell::Cell::new(false);let mut accepted=|_|!canceled.get();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(length,&mut accepted);for _ in 0..stop.as_u64().unwrap(){assert!(parser.step(&text,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}canceled.set(true);let before=(parser.position(),control.owned_bytes());assert_eq!(parser.step(&text,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!((parser.position(),control.owned_bytes()),before);retire(parser);}
    let source=serde_json::json!({"label":law["textUnit"].as_str().unwrap().repeat(4),"array":[null,true,0],"object":{"z":true,"a":"日本"}}).to_string();
    for phase in law["phases"].as_array().unwrap(){let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let canceled=std::cell::Cell::new(false);let mut accepted=|_|!canceled.get();let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let mut turns=0;while parser.phase()!=phase.as_str().unwrap(){turns+=1;assert!(turns<100000);assert!(parser.step(&source,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}canceled.set(true);let before=(parser.position(),control.owned_bytes());assert_eq!(parser.step(&source,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!((parser.position(),control.owned_bytes()),before);retire(parser);}
}

#[test]
fn retained_json_projection_canonicalizes_nested_member_order_under_work_grants() {
    let fixture=owned_json_fixture();let law=&fixture["retainedCanonical"];let key=law["keyUnit"].as_str().unwrap().repeat(law["keyRepeats"].as_u64().unwrap()as usize);let label=owned_json_text(law);let text=to_string(&object([(format!("{key}z"),Value::String(label.clone())),("z".into(),parse("{\"z\":3,\"a\":1}",JsonMemberPolicy::Reject).unwrap()),(format!("{key}a"),Value::String(label)),("a".into(),parse("[{\"z\":false,\"a\":true}]",JsonMemberPolicy::Reject).unwrap())]));let oracle:serde_json::Value=serde_json::from_str(&text).unwrap();let canonical=serde_json::to_string(&oracle).unwrap();let input=||parse(&text,JsonMemberPolicy::Reject).unwrap();let mut projection=JsonValueProjection::new_ordered(input());let mut turns=0;let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4000000,&mut accepted);
    let actual=loop {assert!(projection.step(0,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());turns+=1;assert!(turns<100000);if let Some(value)=projection.step(law["maximumUnits"].as_u64().unwrap()as usize,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap() {break value;}};assert_eq!(to_json_string(&actual),canonical);
    for cutoff in law["cancelUnits"].as_array().unwrap() {let mut projection=JsonValueProjection::new_ordered(input());let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4000000,&mut accepted);for _ in 0..cutoff.as_u64().unwrap() {assert!(projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}retire_original_json(projection,law["retirementBytes"].as_u64().unwrap()as usize,100000);}
}

#[test]
fn retained_json_writer_resumes_canonical_physical_bytes_and_retires_owned_source() {
    let fixture=owned_json_fixture(); let law=&fixture["retainedWriting"];
    let expected=serde_json::json!({"label":owned_json_text(law),"nested":[null,true,{"values":[1,2.5,-3]}]});
    let source=expected.to_string(); let maximum=law["maximumBytes"].as_u64().unwrap() as usize;
    let retire=|writer:JsonWriteCursor<DslValue>|retire_original_json(writer,law["retirementBytes"].as_u64().unwrap()as usize,100000);
    let mut final_turns=0;
    for budget in law["budgets"].as_array().unwrap() {
        let value=to_dsl_value(&parse(&source,JsonMemberPolicy::Reject).unwrap()); let mut writer=JsonWriteCursor::new(value); let mut callback=|_|true; let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut callback); control.begin_stage(0).unwrap();
        let mut turns=0; let output=loop { let before=(writer.progress(),control.owned_bytes()); assert!(writer.step(0,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none()); assert_eq!((writer.progress(),control.owned_bytes()),before); turns+=1; assert!(turns<100000); if let Some(output)=writer.step(budget.as_u64().unwrap() as usize,&mut control,JSON_NORMAL_TEST_POLICY).unwrap() {break output;} };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(),expected); assert_eq!(output,source); final_turns=turns; retire(writer);
    }
    for stop in [0,1,32,final_turns*128] {
        let value=to_dsl_value(&parse(&source,JsonMemberPolicy::Reject).unwrap()); let mut writer=JsonWriteCursor::new(value); let canceled=std::cell::Cell::new(false); let mut callback=|_|!canceled.get(); let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut callback); control.begin_stage(0).unwrap();
        for _ in 0..stop {assert!(writer.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}
        canceled.set(true); let before=writer.progress(); let error=writer.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err(); assert_eq!(error.kind,ValueRefusalKind::Canceled); assert_eq!(writer.progress(),before); retire(writer);
    }
    let value=to_dsl_value(&parse(&source,JsonMemberPolicy::Reject).unwrap()); let mut writer=JsonWriteCursor::new(value); let mut callback=|_|true; let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut callback); assert_eq!(writer.step(1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap_err().kind,ValueRefusalKind::OwnershipLimit); retire(writer);
    let owned=DslValue::String(owned_json_text(law));let pointer=match &owned {DslValue::String(text)=>text.as_ptr(),_=>unreachable!()};let mut writer=JsonWriteCursor::new(owned);assert!(writer.take_source().is_none());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut accepted);let mut turns=0;while writer.step(8,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none() {turns+=1;assert!(turns<100000);assert!(writer.take_source().is_none());}let owned=writer.take_source().unwrap();assert!(writer.take_source().is_none());match &owned {DslValue::String(text)=>assert_eq!(text.as_ptr(),pointer),_=>unreachable!()};retire(writer);retire_original_json(owned,3,100000);
}

#[test]
fn retained_json_parser_and_projection_resume_and_retire_bounded_candidates() {
    let fixture=owned_json_fixture();let law=&fixture["retainedParsing"];let expected=serde_json::json!({"label":owned_json_text(law),"nested":[null,true,{"values":[1,2.5,-3]}]});let text=expected.to_string();let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut turns=0;let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4000000,&mut accepted);
    let value=loop {let position=parser.position();assert!(parser.step(&text,0,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());assert_eq!(parser.position(),position);turns+=1;assert!(turns<100000);if let Some(value)=parser.step(&text,law["maximumUnits"].as_u64().unwrap()as usize,&mut control,JSON_NORMAL_TEST_POLICY).unwrap() {break value;}};
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&value)).unwrap(),expected);let mut projection=JsonValueProjection::new(value);let projected=loop {assert!(projection.step(0,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());if let Some(value)=projection.step(1,3,&mut control,JSON_NORMAL_TEST_POLICY).unwrap() {break value;}};assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&projected)).unwrap(),expected);
    for stop in [0,1,text.len()/2,text.len()-1] {let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4000000,&mut accepted);while parser.position()<stop {assert!(parser.step(&text,1,&mut control,JSON_NORMAL_TEST_POLICY).unwrap().is_none());}retire_original_json(parser,law["retirementBytes"].as_u64().unwrap()as usize,100000);}
}

#[test]
fn retained_number_grammar_preserves_long_lexemes_against_independent_serde() {
    let fixture=owned_json_fixture();for row in fixture["retainedNumberTexts"].as_array().unwrap() {let text=row["prefix"].as_str().unwrap().to_owned()+&"0".repeat(row["zeroes"].as_u64().unwrap()as usize)+row["suffix"].as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(&text).unwrap();let actual=parse(&text,JsonMemberPolicy::Reject).unwrap();assert_eq!(actual.as_f64().unwrap().to_bits(),expected.as_f64().unwrap().to_bits(),"{}",row["prefix"]);}
}

#[test]
fn controlled_json_matches_independent_grammar_and_ordered_values(){
 let fixture=owned_json_fixture();
 for text in fixture["validTexts"].as_array().unwrap(){let text=text.as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(text).unwrap();let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(2_000_000,&mut accepted);let actual:DslValue=from_json_str_controlled(text, crate::JsonMemberPolicy::Replace,&mut control).unwrap();assert_eq!(actual,from_json_str::<DslValue>(text, crate::JsonMemberPolicy::Replace).unwrap());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(2_000_000,&mut accepted);let written=to_json_string_controlled(&actual,&mut control).unwrap();assert_eq!(written,to_json_string(&actual));assert_eq!(serde_json::from_str::<serde_json::Value>(&written).unwrap(),expected);}
 for text in fixture["invalidTexts"].as_array().unwrap(){let text=text.as_str().unwrap();assert!(serde_json::from_str::<serde_json::Value>(text).is_err());let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(2_000_000,&mut accepted);assert!(from_json_str_controlled::<DslValue>(text, crate::JsonMemberPolicy::Replace,&mut control).is_err(),"{text:?}");}
}

#[test]
fn controlled_json_string_admission_covers_typed_and_physical_ownership(){
 let text=owned_json_text(&owned_json_fixture());let json=serde_json::to_string(&text).unwrap();let exact=text.len()*2;
 let mut accepted=|_|true;let mut decode=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(exact,&mut accepted);assert_eq!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut decode).unwrap(),text);assert_eq!(decode.owned_bytes(),exact);
 let mut accepted=|_|true;let mut decode=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(exact-1,&mut accepted);assert!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut decode).is_err());assert!(decode.owned_bytes()<=exact-1);
 let exact=text.len()+json.len();let mut accepted=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(exact,&mut accepted);assert_eq!(to_json_string_controlled(&text,&mut encode).unwrap(),json);assert_eq!(encode.owned_bytes(),exact);
 let mut accepted=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(exact-1,&mut accepted);assert!(to_json_string_controlled(&text,&mut encode).is_err());assert!(encode.owned_bytes()<=exact-1);
}

#[test]
fn controlled_json_cancels_inside_source_scan_materialization_and_typed_binding(){
 let fixture=owned_json_fixture();let text=owned_json_text(&fixture);let json=serde_json::to_string(&text).unwrap();
 for (index,stage)in fixture["decodeStages"].as_array().unwrap().iter().enumerate(){let total=json.len()-index;let mut stopped=false;let mut progress=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|{if event.total==total&&event.completed>0&&event.completed<total{stopped=true;false}else{true}};let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(2_000_000,&mut progress);assert!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut control).is_err(),"{stage}");assert_eq!(control.owned_bytes(),index*text.len(),"{stage}");assert!(stopped,"{stage}");}
 let mut stopped=|_|false;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(2_000_000,&mut stopped);assert!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
}

#[test]
fn controlled_json_cancels_inside_typed_copy_measurement_and_physical_output(){
 let fixture=owned_json_fixture();let text=owned_json_text(&fixture);let json=serde_json::to_string(&text).unwrap();
 for (index,stage)in fixture["encodeStages"].as_array().unwrap().iter().enumerate(){let mut completed_stages=0;let mut stopped=false;let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if event.total==text.len(){if event.completed==event.total{completed_stages+=1;}else if event.completed>0&&completed_stages==index{stopped=true;return false;}}true};let mut control=semio_framework_value::NativeEncodeControl::new(2_000_000,&mut progress);assert!(to_json_string_controlled(&text,&mut control).is_err(),"{stage}");assert_eq!(control.owned_bytes(),text.len()+if index==2{json.len()}else{0},"{stage}");assert!(stopped,"{stage}");}
 let mut stopped=|_|false;let mut control=semio_framework_value::NativeEncodeControl::new(2_000_000,&mut stopped);assert!(to_json_string_controlled(&text,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
}

#[test]
fn controlled_json_preserves_nested_values_and_refuses_unbounded_depth(){
 let fixture=owned_json_fixture();
 for depth in[fixture["acceptedDepth"].as_u64().unwrap() as usize,fixture["refusedDepth"].as_u64().unwrap() as usize]{let text="[".repeat(depth)+"null"+&"]".repeat(depth);let mut accepted=|_|true;let mut decode=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(2_000_000,&mut accepted);let actual=from_json_str_controlled::<DslValue>(&text, crate::JsonMemberPolicy::Replace,&mut decode);if depth==fixture["acceptedDepth"].as_u64().unwrap() as usize{let actual=actual.unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),serde_json::from_str::<serde_json::Value>(&to_json_string(&actual)).unwrap());let mut accepted=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(2_000_000,&mut accepted);assert_eq!(to_json_string_controlled(&actual,&mut encode).unwrap(),text);}else{assert!(actual.is_err());assert!(serde_json::from_str::<serde_json::Value>(&text).is_err());}}
}
