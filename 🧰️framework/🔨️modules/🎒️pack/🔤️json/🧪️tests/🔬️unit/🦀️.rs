
use super::*;

#[test]
fn completed_json_projection_frame_reuses_inline_authority_without_transient_admission(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📦️completed-frame/🔣️.json")).unwrap();let source=law["source"].as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(source).unwrap();let mut projection=JsonValueProjection::new(parse(source,JsonMemberPolicy::Reject).unwrap());let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if projection.retirement.is_some(){break;}assert!(projection.step(1,3,&mut control).unwrap().is_none());}assert!(projection.retirement.is_some());let admitted=control.owned_bytes();let mut released=0;
    let original={let owner=projection.retirement.as_mut().unwrap();use semio_framework_value::{retirement::{RetirementCursor,RetirementStep},retained_clone::RetainedCloneGrant};let phase=owner.phase;let demand=owner.release_demand();assert!(demand>3);for grant in[RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:3,maximum_capacity_bytes:0,maximum_release_bytes:demand,maximum_depth:1},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:0,maximum_release_bytes:demand-1,maximum_depth:1}]{let(step,born,physical)=test_allocation::observe_backing(||owner.close_step(grant));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((born,physical),(0,0));assert_eq!(owner.phase,phase);assert_eq!(owner.release_demand(),demand);}owner.frame.values.allocation+owner.frame.entries.allocation+owner.frame.array.capacity()*std::mem::size_of::<DslValue>()+owner.frame.object.capacity()*std::mem::size_of::<(String,DslValue)>()};
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if projection.retirement.is_none(){break;}let(result,born,physical)=test_allocation::observe_backing(||projection.step(1,3,&mut control));assert!(result.unwrap().is_none());assert_eq!(born,0,"completed input backing needs no fresh retirement scaffold");assert_eq!(control.owned_bytes(),admitted,"original source admission is conserved through physical input release");released+=physical;}
    assert!(projection.retirement.is_none());assert_eq!(released,original);let value=projection.step(1,3,&mut control).unwrap().expect("original output survives physical source close");assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&value)).unwrap(),expected);retire_original_json(projection,3,65536);retire_original_json(value,3,65536);
    eprintln!("[DEBUG] completed JSON frame owns original backing, born=0 physical={released} original-admission={admitted}; Serde output remains exact after reusable inline close");
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
        if owner.terminal_is_empty() { eprintln!("[DEBUG] JSON controlled original owner closed turns={}", turn + 1); return; }
    }
    panic!("original JSON controlled retirement did not terminate");
}

#[test]
fn original_json_parser_writer_and_value_retire_with_independent_physical_grants() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️retirement/🔣️.json")).unwrap();
    let source = law["source"].as_str().unwrap();
    let expected: serde_json::Value = serde_json::from_str(source).unwrap();
    let value = parse(source, JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&value)).unwrap(), expected);
    retire_json_with_independent_grants(value, &law);
    for stop in law["cancelUnits"].as_array().unwrap() {
        let mut parser = JsonParseCursor::new(JsonMemberPolicy::Reject);
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(1_000_000, &mut accepted);
        for _ in 0..stop.as_u64().unwrap() { if let Some(value) = parser.step(source, 1, &mut control).unwrap() { retire_json_with_independent_grants(value, &law); break; } }
        retire_json_with_independent_grants(parser, &law);
        let mut writer = JsonWriteCursor::new(from_json_str::<semio_framework_value::DslValue>(source, JsonMemberPolicy::Reject).unwrap());
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(1_000_000, &mut accepted);
        for _ in 0..stop.as_u64().unwrap() { if writer.step(1, &mut control).unwrap().is_some() { break; } }
        retire_json_with_independent_grants(writer, &law);
        let mut projection = JsonValueProjection::new(parse(source, JsonMemberPolicy::Reject).unwrap());
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(1_000_000, &mut accepted);
        for _ in 0..stop.as_u64().unwrap() { if let Some(value) = projection.step(1, 3, &mut control).unwrap() { retire_json_with_independent_grants(value, &law); break; } }
        retire_json_with_independent_grants(projection, &law);
    }
}

fn owned_json_fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🚦️owned-controls.json")).unwrap()}
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
    for budget in law["budgets"].as_array().unwrap(){let original=source();let pointer=original.text.as_ptr();let mut writer=JsonWriteCursor::new(original);assert!(writer.take_source().is_none());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let mut turns=0;let output=loop{let before=writer.progress();assert!(writer.step(0,&mut control).unwrap().is_none());assert_eq!(writer.progress(),before);turns+=1;assert!(turns<100000);if let Some(output)=writer.step(budget.as_u64().unwrap()as usize,&mut control).unwrap(){break output}assert!(writer.take_source().is_none());};assert!(turns>1);assert_eq!(output,expected);let original=writer.take_source().unwrap();assert!(writer.take_source().is_none());assert_eq!(original.text.as_ptr(),pointer);retire(writer);retire_original_json(original,3,100000);}
    for stop in [0,1,32,512,4096]{let mut writer=JsonWriteCursor::new(source());let live=std::cell::Cell::new(true);let mut callback=|_|live.get();let mut control=semio_framework_value::NativeEncodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut callback);for _ in 0..stop{assert!(writer.step(1,&mut control).unwrap().is_none())}let before=writer.progress();live.set(false);assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(writer.progress(),before);retire(writer);}
    eprintln!("[DEBUG] Original owned JSON source pointer retained without projection; budgets1/8/256 exactSerde bytes zero/cancel3byte retirement passed");
}

#[test]
fn retained_json_projection_admits_exact_vectors_and_drains_consumed_input() {
    let fixture=owned_json_fixture();let law=&fixture["retainedProjectionAdmission"];let source=serde_json::json!({"values":(0..law["collectionItems"].as_u64().unwrap()).map(|index|serde_json::json!({"index":index,"label":format!("Label 😀 {index}")})).collect::<Vec<_>>()}).to_string();let oracle:serde_json::Value=serde_json::from_str(&source).unwrap();
    let input=||parse(&source,JsonMemberPolicy::Reject).unwrap();let mut projection=JsonValueProjection::new_ordered(input());let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let mut phases=std::collections::BTreeSet::new();let mut turns=0;
    let value=loop{assert!(projection.step(0,0,&mut control).unwrap().is_none());phases.insert(projection.phase());turns+=1;assert!(turns<2000000);if let Some(value)=projection.step(law["maximumUnits"].as_u64().unwrap()as usize,law["retirementBytes"].as_u64().unwrap()as usize,&mut control).unwrap(){break value;}};assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&value)).unwrap(),oracle);assert!(projection.retirement.is_none());
    for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()),"{phase}");}
    let retire=|projection:JsonValueProjection|retire_original_json(projection,3,4000000);retire(projection);retire_original_json(value,3,4000000);
    let mut projection=JsonValueProjection::new(input());let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(1,&mut accepted);assert_eq!(projection.step(1,3,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);retire(projection);
    for stop in law["cancelUnits"].as_array().unwrap(){let mut projection=JsonValueProjection::new(input());let canceled=std::cell::Cell::new(false);let mut accepted=|_|!canceled.get();let mut control=semio_framework_value::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);for _ in 0..stop.as_u64().unwrap(){assert!(projection.step(1,3,&mut control).unwrap().is_none());}canceled.set(true);assert_eq!(projection.step(1,3,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);retire(projection);}
    eprintln!("[DEBUG] Same JSON typed projection admitted exact vectors, retained consumed input scaffold, oneunit/3byte frontier and independentSerde parity");
}

#[test]
fn retained_json_parser_admits_storage_before_copying_and_resumes_one_owner() {
    let fixture=owned_json_fixture();let law=&fixture["retainedAdmission"];let label=owned_json_text(law);let length=label.len();let text=serde_json::to_string(&label).unwrap();
    let retire=|parser:JsonParseCursor|retire_original_json(parser,law["retirementBytes"].as_u64().unwrap()as usize,4000000);
    let mut phases=std::collections::BTreeSet::new();
    for (source,maximum) in [(text.clone(),length),(serde_json::to_string(&vec![label.clone();law["collectionItems"].as_u64().unwrap()as usize/128]).unwrap(),law["maximumBytes"].as_u64().unwrap()as usize),(serde_json::json!({"labels":(0..law["collectionItems"].as_u64().unwrap()).collect::<Vec<_>>(),"object":(0..128).map(|index|(format!("key{index}"),serde_json::Value::from(index))).collect::<serde_json::Map<_,_>>(),"label":label}).to_string(),law["maximumBytes"].as_u64().unwrap()as usize)] {
        let oracle:serde_json::Value=serde_json::from_str(&source).unwrap();let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(maximum,&mut accepted);let mut turns=0;
        let value=loop{let before=(parser.position(),control.owned_bytes());assert!(parser.step(&source,0,&mut control).unwrap().is_none());assert_eq!((parser.position(),control.owned_bytes()),before);phases.insert(parser.phase());turns+=1;assert!(turns<4000000);if let Some(value)=parser.step(&source,law["maximumUnits"].as_u64().unwrap()as usize,&mut control).unwrap(){break value;}};
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&value)).unwrap(),oracle);assert!(control.owned_bytes()<=maximum);if source==text {assert_eq!(control.owned_bytes(),length);}retire(parser);retire_original_json(value,3,4000000);
    }
    for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()),"{phase}");}
    let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(length-1,&mut accepted);let error=loop{match parser.step(&text,1,&mut control){Ok(None)=>{},Ok(Some(_))=>panic!("ownership limit admitted a string"),Err(error)=>break error}};assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);retire(parser);
    for stop in law["cancelUnits"].as_array().unwrap(){let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let canceled=std::cell::Cell::new(false);let mut accepted=|_|!canceled.get();let mut control=semio_framework_value::NativeDecodeControl::new(length,&mut accepted);for _ in 0..stop.as_u64().unwrap(){assert!(parser.step(&text,1,&mut control).unwrap().is_none());}canceled.set(true);let before=(parser.position(),control.owned_bytes());assert_eq!(parser.step(&text,1,&mut control).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!((parser.position(),control.owned_bytes()),before);retire(parser);}
    let source=serde_json::json!({"label":law["textUnit"].as_str().unwrap().repeat(4),"array":[null,true,0],"object":{"z":true,"a":"日本"}}).to_string();
    for phase in law["phases"].as_array().unwrap(){let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let canceled=std::cell::Cell::new(false);let mut accepted=|_|!canceled.get();let mut control=semio_framework_value::NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let mut turns=0;while parser.phase()!=phase.as_str().unwrap(){turns+=1;assert!(turns<100000);assert!(parser.step(&source,1,&mut control).unwrap().is_none());}canceled.set(true);let before=(parser.position(),control.owned_bytes());assert_eq!(parser.step(&source,1,&mut control).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!((parser.position(),control.owned_bytes()),before);retire(parser);}
    eprintln!("[DEBUG] Same JSON parser measured/admitted strings, paged collection candidates, exact final storage, all6phases and3byte retirement");
}

#[test]
fn retained_json_projection_canonicalizes_nested_member_order_under_work_grants() {
    let fixture=owned_json_fixture();let law=&fixture["retainedCanonical"];let key=law["keyUnit"].as_str().unwrap().repeat(law["keyRepeats"].as_u64().unwrap()as usize);let label=owned_json_text(law);let text=to_string(&object([(format!("{key}z"),Value::String(label.clone())),("z".into(),parse("{\"z\":3,\"a\":1}",JsonMemberPolicy::Reject).unwrap()),(format!("{key}a"),Value::String(label)),("a".into(),parse("[{\"z\":false,\"a\":true}]",JsonMemberPolicy::Reject).unwrap())]));let oracle:serde_json::Value=serde_json::from_str(&text).unwrap();let canonical=serde_json::to_string(&oracle).unwrap();let input=||parse(&text,JsonMemberPolicy::Reject).unwrap();let mut projection=JsonValueProjection::new_ordered(input());let mut turns=0;let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4000000,&mut accepted);
    let actual=loop {assert!(projection.step(0,3,&mut control).unwrap().is_none());turns+=1;assert!(turns<100000);if let Some(value)=projection.step(law["maximumUnits"].as_u64().unwrap()as usize,3,&mut control).unwrap() {break value;}};assert_eq!(to_json_string(&actual),canonical);
    for cutoff in law["cancelUnits"].as_array().unwrap() {let mut projection=JsonValueProjection::new_ordered(input());let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4000000,&mut accepted);for _ in 0..cutoff.as_u64().unwrap() {assert!(projection.step(1,3,&mut control).unwrap().is_none());}retire_original_json(projection,law["retirementBytes"].as_u64().unwrap()as usize,100000);}
    eprintln!("[DEBUG] Same JSON projection retained canonical nested key order, transitions={turns}");
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
        let mut turns=0; let output=loop { let before=(writer.progress(),control.owned_bytes()); assert!(writer.step(0,&mut control).unwrap().is_none()); assert_eq!((writer.progress(),control.owned_bytes()),before); turns+=1; assert!(turns<100000); if let Some(output)=writer.step(budget.as_u64().unwrap() as usize,&mut control).unwrap() {break output;} };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(),expected); assert_eq!(output,source); final_turns=turns; retire(writer);
    }
    for stop in [0,1,32,final_turns*128] {
        let value=to_dsl_value(&parse(&source,JsonMemberPolicy::Reject).unwrap()); let mut writer=JsonWriteCursor::new(value); let canceled=std::cell::Cell::new(false); let mut callback=|_|!canceled.get(); let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut callback); control.begin_stage(0).unwrap();
        for _ in 0..stop {assert!(writer.step(1,&mut control).unwrap().is_none());}
        canceled.set(true); let before=writer.progress(); let error=writer.step(1,&mut control).unwrap_err(); assert_eq!(error.kind,ValueRefusalKind::Canceled); assert_eq!(writer.progress(),before); retire(writer);
    }
    let value=to_dsl_value(&parse(&source,JsonMemberPolicy::Reject).unwrap()); let mut writer=JsonWriteCursor::new(value); let mut callback=|_|true; let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut callback); assert_eq!(writer.step(1,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit); retire(writer);
    let owned=DslValue::String(owned_json_text(law));let pointer=match &owned {DslValue::String(text)=>text.as_ptr(),_=>unreachable!()};let mut writer=JsonWriteCursor::new(owned);assert!(writer.take_source().is_none());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut accepted);let mut turns=0;while writer.step(8,&mut control).unwrap().is_none() {turns+=1;assert!(turns<100000);assert!(writer.take_source().is_none());}let owned=writer.take_source().unwrap();assert!(writer.take_source().is_none());match &owned {DslValue::String(text)=>assert_eq!(text.as_ptr(),pointer),_=>unreachable!()};retire(writer);retire_original_json(owned,3,100000);
    eprintln!("[DEBUG] retained JSON physical writer budgets=1/8/256 independentSerde=true cancellation=true boundedSourceRetirement=true");
}

#[test]
fn retained_json_parser_and_projection_resume_and_retire_bounded_candidates() {
    let fixture=owned_json_fixture();let law=&fixture["retainedParsing"];let expected=serde_json::json!({"label":owned_json_text(law),"nested":[null,true,{"values":[1,2.5,-3]}]});let text=expected.to_string();let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut turns=0;let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4000000,&mut accepted);
    let value=loop {let position=parser.position();assert!(parser.step(&text,0,&mut control).unwrap().is_none());assert_eq!(parser.position(),position);turns+=1;assert!(turns<100000);if let Some(value)=parser.step(&text,law["maximumUnits"].as_u64().unwrap()as usize,&mut control).unwrap() {break value;}};
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&value)).unwrap(),expected);let mut projection=JsonValueProjection::new(value);let projected=loop {assert!(projection.step(0,3,&mut control).unwrap().is_none());if let Some(value)=projection.step(1,3,&mut control).unwrap() {break value;}};assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&projected)).unwrap(),expected);
    for stop in [0,1,text.len()/2,text.len()-1] {let mut parser=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4000000,&mut accepted);while parser.position()<stop {assert!(parser.step(&text,1,&mut control).unwrap().is_none());}retire_original_json(parser,law["retirementBytes"].as_u64().unwrap()as usize,100000);}
    eprintln!("[DEBUG] Existing JSON grammar retained borrowed source and moved typed projection, transitions={turns}");
}

#[test]
fn retained_number_grammar_preserves_long_lexemes_against_independent_serde() {
    let fixture=owned_json_fixture();for row in fixture["retainedNumberTexts"].as_array().unwrap() {let text=row["prefix"].as_str().unwrap().to_owned()+&"0".repeat(row["zeroes"].as_u64().unwrap()as usize)+row["suffix"].as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(&text).unwrap();let actual=parse(&text,JsonMemberPolicy::Reject).unwrap();assert_eq!(actual.as_f64().unwrap().to_bits(),expected.as_f64().unwrap().to_bits(),"{}",row["prefix"]);}
}

#[test]
fn controlled_json_matches_independent_grammar_and_ordered_values(){
 let fixture=owned_json_fixture();
 for text in fixture["validTexts"].as_array().unwrap(){let text=text.as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(text).unwrap();let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(2_000_000,&mut accepted);let actual:DslValue=from_json_str_controlled(text, crate::JsonMemberPolicy::Replace,&mut control).unwrap();assert_eq!(actual,from_json_str::<DslValue>(text, crate::JsonMemberPolicy::Replace).unwrap());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(2_000_000,&mut accepted);let written=to_json_string_controlled(&actual,&mut control).unwrap();assert_eq!(written,to_json_string(&actual));assert_eq!(serde_json::from_str::<serde_json::Value>(&written).unwrap(),expected);}
 for text in fixture["invalidTexts"].as_array().unwrap(){let text=text.as_str().unwrap();assert!(serde_json::from_str::<serde_json::Value>(text).is_err());let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(2_000_000,&mut accepted);assert!(from_json_str_controlled::<DslValue>(text, crate::JsonMemberPolicy::Replace,&mut control).is_err(),"{text:?}");}
}

#[test]
fn controlled_json_string_admission_covers_typed_and_physical_ownership(){
 let text=owned_json_text(&owned_json_fixture());let json=serde_json::to_string(&text).unwrap();let exact=text.len()*2;
 let mut accepted=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(exact,&mut accepted);assert_eq!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut decode).unwrap(),text);assert_eq!(decode.owned_bytes(),exact);
 let mut accepted=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(exact-1,&mut accepted);assert!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut decode).is_err());assert!(decode.owned_bytes()<=exact-1);
 let exact=text.len()+json.len();let mut accepted=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(exact,&mut accepted);assert_eq!(to_json_string_controlled(&text,&mut encode).unwrap(),json);assert_eq!(encode.owned_bytes(),exact);
 let mut accepted=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(exact-1,&mut accepted);assert!(to_json_string_controlled(&text,&mut encode).is_err());assert!(encode.owned_bytes()<=exact-1);
}

#[test]
fn controlled_json_cancels_inside_source_scan_materialization_and_typed_binding(){
 let fixture=owned_json_fixture();let text=owned_json_text(&fixture);let json=serde_json::to_string(&text).unwrap();
 for (index,stage)in fixture["decodeStages"].as_array().unwrap().iter().enumerate(){let total=json.len()-index;let mut stopped=false;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==total&&event.completed>0&&event.completed<total{stopped=true;false}else{true}};let mut control=semio_framework_value::NativeDecodeControl::new(2_000_000,&mut progress);assert!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut control).is_err(),"{stage}");assert_eq!(control.owned_bytes(),index*text.len(),"{stage}");assert!(stopped,"{stage}");}
 let mut stopped=|_|false;let mut control=semio_framework_value::NativeDecodeControl::new(2_000_000,&mut stopped);assert!(from_json_str_controlled::<String>(&json, crate::JsonMemberPolicy::Replace,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
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
 for depth in[fixture["acceptedDepth"].as_u64().unwrap() as usize,fixture["refusedDepth"].as_u64().unwrap() as usize]{let text="[".repeat(depth)+"null"+&"]".repeat(depth);let mut accepted=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(2_000_000,&mut accepted);let actual=from_json_str_controlled::<DslValue>(&text, crate::JsonMemberPolicy::Replace,&mut decode);if depth==fixture["acceptedDepth"].as_u64().unwrap() as usize{let actual=actual.unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),serde_json::from_str::<serde_json::Value>(&to_json_string(&actual)).unwrap());let mut accepted=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(2_000_000,&mut accepted);assert_eq!(to_json_string_controlled(&actual,&mut encode).unwrap(),text);}else{assert!(actual.is_err());assert!(serde_json::from_str::<serde_json::Value>(&text).is_err());}}
}


#[test]
fn macro_borrows_records_and_matches_json_vectors() {
    struct Record {
        name: String,
        count: u64,
    }
    impl ToValue for Record {
        fn to_value(&self) -> DslValue {
            DslValue::object([("name".into(), self.name.to_value()), ("count".into(), self.count.to_value())])
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️macro-values.json")).unwrap();
    for vector in fixture.as_array().unwrap() {
        let record = Record { name: vector["name"].as_str().unwrap().into(), count: vector["count"].as_u64().unwrap() };
        let borrowed = &record;
        let nested = parse(&vector["nested"].to_string(), crate::JsonMemberPolicy::Replace).unwrap();
        let actual = crate::json!({ "record": record, "name": borrowed.name, "again": &borrowed.name, "nested": nested });
        let oracle = serde_json::json!({ "record": { "name": record.name, "count": record.count }, "name": record.name, "again": record.name, "nested": vector["nested"] });
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&actual)).unwrap(), oracle);
        let typed: Value = from_json_str(&to_json_string(&nested), crate::JsonMemberPolicy::Replace).unwrap();
        assert_eq!(typed, nested);
        assert_eq!(borrowed.name, vector["name"].as_str().unwrap());
    }
}

//#region 🔖️Literals
#[test]
fn parses_literals() {
    assert_eq!(parse("null", crate::JsonMemberPolicy::Replace).unwrap(), Value::Null);
    assert_eq!(parse("true", crate::JsonMemberPolicy::Replace).unwrap(), Value::Bool(true));
    assert_eq!(parse("false", crate::JsonMemberPolicy::Replace).unwrap(), Value::Bool(false));
    assert_eq!(parse("  null  ", crate::JsonMemberPolicy::Replace).unwrap(), Value::Null);
}

#[test]
fn rejects_trailing_data() {
    assert_eq!(parse("null null", crate::JsonMemberPolicy::Replace), Err(JsonError::TrailingData(5)));
}

#[test]
fn rejects_empty_input() {
    assert_eq!(parse("", crate::JsonMemberPolicy::Replace), Err(JsonError::UnexpectedEof));
    assert_eq!(parse("   ", crate::JsonMemberPolicy::Replace), Err(JsonError::UnexpectedEof));
}
//#endregion 🔖️Literals

//#region 🔖️Numbers
#[test]
fn integer_and_float_are_never_confused() {
    assert_eq!(to_string(&Value::Number(Number::UInt(42))), "42");
    assert_eq!(to_string(&Value::Number(Number::Float(42.0))), "42.0");
    assert_eq!(parse("42", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::UInt(42)));
    assert_eq!(parse("42.0", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(42.0)));
    assert_ne!(parse("42", crate::JsonMemberPolicy::Replace).unwrap(), parse("42.0", crate::JsonMemberPolicy::Replace).unwrap());
}

#[test]
fn parses_number_grammar() {
    assert_eq!(parse("0", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::UInt(0)));
    assert_eq!(parse("-0", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Int(0)));
    assert_eq!(parse("-17", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Int(-17)));
    assert_eq!(parse("3.125", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(3.125)));
    assert_eq!(parse("1e10", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(1e10)));
    assert_eq!(parse("1.5e-3", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(1.5e-3)));
    assert_eq!(parse("-2E+2", crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(-200.0)));
}

#[test]
fn parses_exact_fractional_zero_below_f64_mantissa_boundary() {
    let text = "8322951083873004.0";
    let expected = 8_322_951_083_873_004.0;
    assert_eq!(parse(text, crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(expected)));
    assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
}

#[test]
fn parses_exact_decimal_exponent_below_f64_mantissa_boundary() {
    let text = "83229510838730040e-1";
    let expected = 8_322_951_083_873_004.0;
    assert_eq!(parse(text, crate::JsonMemberPolicy::Replace).unwrap(), Value::Number(Number::Float(expected)));
    assert_eq!(serde_json::from_str::<serde_json::Value>(text).unwrap().as_f64(), Some(expected));
}

#[test]
fn rejects_leading_zeros() {
    assert!(parse("01", crate::JsonMemberPolicy::Replace).is_err());
    assert!(parse("[01]", crate::JsonMemberPolicy::Replace).is_err());
    assert!(parse("-01", crate::JsonMemberPolicy::Replace).is_err());
}

#[test]
fn huge_integer_falls_back_to_float() {
    let text = "99999999999999999999999999999999";
    match parse(text, crate::JsonMemberPolicy::Replace).unwrap() {
        Value::Number(Number::Float(_)) => {}
        other => panic!("expected float fallback, got {other:?}"),
    }
}

#[test]
fn rejects_numbers_outside_f64_range() {
    assert!(matches!(parse("1e999", crate::JsonMemberPolicy::Replace), Err(JsonError::InvalidNumber(0))));
    assert!(matches!(parse("-1e999", crate::JsonMemberPolicy::Replace), Err(JsonError::InvalidNumber(0))));
    assert!(serde_json::from_str::<serde_json::Value>("1e999").is_err());
    assert!(serde_json::from_str::<serde_json::Value>("-1e999").is_err());
}

#[test]
fn non_finite_floats_encode_as_null() {
    assert_eq!(to_string(&Value::Number(Number::Float(f64::NAN))), "null");
    assert_eq!(to_string(&Value::Number(Number::Float(f64::INFINITY))), "null");
    assert_eq!(to_string(&Value::Number(Number::Float(f64::NEG_INFINITY))), "null");
}

#[test]
fn large_and_small_magnitudes_use_exponential_notation() {
    let text = to_string(&Value::Number(Number::Float(1.5e300)));
    assert!(text.contains('e'), "expected exponential form, got {text}");
    assert_eq!(parse(&text, crate::JsonMemberPolicy::Replace).unwrap().as_f64().unwrap(), 1.5e300);

    let text = to_string(&Value::Number(Number::Float(5e-300)));
    assert!(text.contains('e'), "expected exponential form, got {text}");
    assert_eq!(parse(&text, crate::JsonMemberPolicy::Replace).unwrap().as_f64().unwrap(), 5e-300);
}
//#endregion 🔖️Numbers

//#region 🔖️Strings
#[test]
fn parses_escapes_and_unicode() {
    assert_eq!(parse(r#""hi\nthere""#, crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "hi\nthere");
    assert_eq!(parse(r#""café""#, crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "café");
    assert_eq!(parse(r#""😀""#, crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "😀");
    assert_eq!(parse("\"café\"", crate::JsonMemberPolicy::Replace).unwrap().as_str().unwrap(), "café"); // raw UTF-8 passthrough
}

#[test]
fn rejects_lone_surrogate() {
    assert!(matches!(parse(r#""\ud83d""#, crate::JsonMemberPolicy::Replace), Err(JsonError::UnpairedSurrogate(_))));
    assert!(matches!(parse(r#""\ud83dX""#, crate::JsonMemberPolicy::Replace), Err(JsonError::UnpairedSurrogate(_))));
}

#[test]
fn rejects_raw_control_character_in_string() {
    let text = "\"a\u{0001}b\"";
    assert!(matches!(parse(text, crate::JsonMemberPolicy::Replace), Err(JsonError::ControlCharacterInString { .. })));
}

#[test]
fn writer_round_trips_supplementary_plane_and_control_chars() {
    let value = Value::String("😀\u{0001}\t\"\\".to_string());
    let text = to_string(&value);
    assert_eq!(parse(&text, crate::JsonMemberPolicy::Replace).unwrap(), value);
}
//#endregion 🔖️Strings

//#region 🔖️Containers
#[test]
fn parses_arrays_and_objects() {
    let value = parse(r#"{"a":1,"b":[1,2,3],"c":{"nested":true}}"#, crate::JsonMemberPolicy::Replace).unwrap();
    assert_eq!(value.get("a").unwrap().as_u64(), Some(1));
    assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 3);
    assert_eq!(value.get("c").unwrap().get("nested").unwrap().as_bool(), Some(true));
}

#[test]
fn duplicate_object_keys_keep_first_position_last_value() {
    let value = parse(r#"{"a":1,"b":2,"a":3}"#, crate::JsonMemberPolicy::Replace).unwrap();
    let object = value.as_object().unwrap();
    assert_eq!(object.len(), 2);
    assert_eq!(object.get("a").unwrap().as_u64(), Some(3));
    assert_eq!(object.iter().next().unwrap().0, "a"); // first occurrence's position kept
}

#[test]
fn empty_array_and_object() {
    assert_eq!(parse("[]", crate::JsonMemberPolicy::Replace).unwrap(), Value::Array(vec![]));
    assert_eq!(parse("{}", crate::JsonMemberPolicy::Replace).unwrap(), Value::Object(Object::new()));
    assert_eq!(to_string(&Value::Array(vec![])), "[]");
    assert_eq!(to_string(&Value::Object(Object::new())), "{}");
}

#[test]
fn max_depth_is_enforced() {
    let mut text = String::new();
    for _ in 0..(MAX_DEPTH + 10) {
        text.push('[');
    }
    assert!(matches!(parse(&text, crate::JsonMemberPolicy::Replace), Err(JsonError::MaxDepthExceeded(_))));
}
//#endregion 🔖️Containers

//#region 🔖️ToFromValueBridge
/// 🌉️ `from_dsl_value`/`to_dsl_value` (`//#region 🔖️DslValueBridge` above) had no direct test
/// yet — this exercises the structural walk this region's `to_json_string`/`from_json_str`
/// are built on.
#[test]
fn from_dsl_value_and_to_dsl_value_round_trip_every_shape() {
    let value = DslValue::object([("a".to_string(), DslValue::uint(1)), ("b".to_string(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null, DslValue::String("x".to_string())]))]);
    assert_eq!(to_dsl_value(&from_dsl_value(&value)), value);
}

#[test]
fn to_json_string_and_from_json_str_round_trip_a_dsl_value() {
    let value = DslValue::object([("count".to_string(), DslValue::uint(3)), ("label".to_string(), DslValue::String("ok".to_string()))]);
    let text = to_json_string(&value);
    let parsed: DslValue = from_json_str(&text, crate::JsonMemberPolicy::Replace).unwrap();
    assert_eq!(parsed, value);
}

#[test]
fn from_json_str_reports_a_value_error_on_malformed_text() {
    assert!(from_json_str::<DslValue>("not json", crate::JsonMemberPolicy::Replace).is_err());
}

/// 🎯️ The exact regression named in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
/// 📓️directory-spr-serde-removal.md`: `DirectoryCommand::CreateInvite.ttl_secs: u64` posted
/// through `to_json_string(&command.to_value())` must produce `"ttlSecs":3600` on the wire, not
/// `"ttlSecs":3600.0` — a real Rust/serde hub rejects a float literal for a `u64` field. A
/// genuine `f64` field must keep its `.0` so it never collapses onto its integer twin.
#[test]
fn u64_field_through_to_value_and_to_json_string_renders_as_a_bare_integer() {
    let ttl_secs: u64 = 3600;
    let text = to_json_string(&DslValue::object([("ttlSecs".to_string(), ttl_secs.to_value())]));
    assert_eq!(text, r#"{"ttlSecs":3600}"#);
    let parsed: DslValue = from_json_str(&text, crate::JsonMemberPolicy::Replace).unwrap();
    assert_eq!(parsed.get("ttlSecs").and_then(DslValue::as_u64), Some(3600));

    let ratio: f64 = 3600.0;
    let text = to_json_string(&DslValue::object([("ratio".to_string(), ratio.to_value())]));
    assert_eq!(text, r#"{"ratio":3600.0}"#);
}

//#region 🔖️JsonMacro
#[test]
fn json_macro_builds_scalars_and_null() {
    assert_eq!(crate::json!(null), Value::Null);
    assert_eq!(crate::json!(true), Value::Bool(true));
    assert_eq!(crate::json!(false), Value::Bool(false));
    assert_eq!(crate::json!(1), Value::Number(Number::Int(1)));
    assert_eq!(crate::json!(1.5), Value::Number(Number::Float(1.5)));
    assert_eq!(crate::json!("hi"), Value::String("hi".to_string()));
}

#[test]
fn json_macro_builds_arrays_incl_empty_and_nested() {
    assert_eq!(crate::json!([]), Value::Array(vec![]));
    assert_eq!(crate::json!([1, 2, 3]), Value::Array(vec![Value::Number(Number::Int(1)), Value::Number(Number::Int(2)), Value::Number(Number::Int(3))]));
    assert_eq!(crate::json!([[1], [2, 3]]), Value::Array(vec![Value::Array(vec![Value::Number(Number::Int(1))]), Value::Array(vec![Value::Number(Number::Int(2)), Value::Number(Number::Int(3))])]));
}

#[test]
fn json_macro_builds_objects_incl_empty_and_trailing_commas() {
    assert_eq!(crate::json!({}), Value::Object(Object::new()));
    let value = crate::json!({
        "a": 1,
        "b": [1, 2],
    });
    assert_eq!(value.get("a").unwrap().as_i64(), Some(1));
    assert_eq!(value.get("b").unwrap().as_array().unwrap().len(), 2);
}

#[test]
fn json_macro_evaluates_arbitrary_expressions_and_options() {
    let index = 3;
    let value = crate::json!({
        "id": format!("semio_text-{index}"),
        "meshId": "box",
        "position": [index as f64 * 2.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": format!("Semio Text {index}"),
        "smoothShading": false,
        "nested": { "deep": { "deeper": [1, 2, 3] } },
        "present": Some(5),
        "absent": Option::<i32>::None,
    });
    assert_eq!(value.get("id").unwrap().as_str(), Some("semio_text-3"));
    assert_eq!(value.get("position").unwrap().as_array().unwrap()[0].as_f64(), Some(6.0));
    assert_eq!(value.get("nested").unwrap().get("deep").unwrap().get("deeper").unwrap().as_array().unwrap().len(), 3);
    assert_eq!(value.get("present").unwrap().as_i64(), Some(5));
    assert!(value.get("absent").unwrap().is_null());
}

#[test]
fn json_macro_matches_to_string_of_equivalent_hand_built_value() {
    let via_macro = crate::json!({"a": 1, "b": [true, null, "x"]});
    let hand_built = Value::Object(Object::from_iter([("a".to_string(), Value::Number(Number::Int(1))), ("b".to_string(), Value::Array(vec![Value::Bool(true), Value::Null, Value::String("x".to_string())]))]));
    assert_eq!(to_string(&via_macro), to_string(&hand_built));
}
#[test]
fn value_eq_ignoring_object_order_is_order_insensitive_but_still_structural() {
    let a = crate::json!({"x": 1, "y": [1, 2, {"p": true, "q": "s"}]});
    let b = crate::json!({"y": [1, 2, {"q": "s", "p": true}], "x": 1});
    assert!(value_eq_ignoring_object_order(&a, &b));
    let c = crate::json!({"x": 1, "y": [1, 2, {"p": true, "q": "different"}]});
    assert!(!value_eq_ignoring_object_order(&a, &c));
    let d = crate::json!({"x": 1});
    assert!(!value_eq_ignoring_object_order(&a, &d));
}

#[test]
fn mutable_accessors_update_nested_members() {
    let mut value = crate::json!({ "items": [{ "state": "before" }] });
    value.get_mut("items").and_then(Value::as_array_mut).and_then(|items| items.first_mut()).and_then(Value::as_object_mut).and_then(|item| item.get_mut("state")).expect("nested state").clone_from(&Value::String("after".to_string()));
    assert_eq!(to_string(&value), r#"{"items":[{"state":"after"}]}"#);
}
//#endregion 🔖️JsonMacro

/// 🔬️ Differential (single-key object — see the module's own note above on key-order
/// ambiguity): our bridge's bytes agree with the framework's existing `DslValue ->
/// serde_json::Value` path (`🌱️value/🦀️.rs`'s `impl From<DslValue> for
/// serde_json::Value`), the oracle every framework-internal caller still speaks.
#[test]
fn to_json_string_bytes_match_the_serde_json_bridge() {
    let value = DslValue::object([("nested".to_string(), DslValue::Array(vec![DslValue::uint(1), DslValue::float(2.5)]))]);
    let mine = to_json_string(&value);
    let theirs = serde_json::to_string(&serde_json::Value::from(value)).unwrap();
    assert_eq!(mine, theirs);
}
//#endregion 🔖️ToFromValueBridge

//#region 🔖️PropertyTesting
/// 🎲️ A tiny deterministic PRNG (SplitMix64) — property/differential tests need arbitrary
/// `Value` trees, but adding a `rand`/`proptest`/`arbitrary` crate would itself be a NEW
/// third-party dependency the freeze ratchet forbids; this is small enough to own outright.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn range(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound.max(1)
    }

    fn unit_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn bool(&mut self) -> bool {
        self.next_u64() & 1 == 0
    }
}

fn arbitrary_finite_float(rng: &mut Rng) -> f64 {
    loop {
        let value = match rng.range(4) {
            0 => 0.0,
            1 => (rng.next_u64() as i64) as f64 / 1e3,
            2 => {
                let exponent = rng.range(600) as i32 - 300;
                let mantissa = 1.0 + rng.unit_f64();
                mantissa * 10f64.powi(exponent)
            }
            _ => f64::from_bits(rng.next_u64()),
        };
        if value.is_finite() {
            return value;
        }
    }
}

fn arbitrary_string(rng: &mut Rng) -> String {
    let len = rng.range(6);
    let mut out = String::new();
    for _ in 0..len {
        let ch = match rng.range(11) {
            0 => '"',
            1 => '\\',
            2 => '\n',
            3 => '\t',
            4 => '\u{0000}',
            5 => '\u{001F}',
            6 => '€',
            7 => '😀',
            8 => '\u{0008}',
            9 => '\u{000C}',
            _ => char::from_u32(0x20 + rng.range(0x5E) as u32).unwrap_or('x'),
        };
        out.push(ch);
    }
    out
}

fn arbitrary_value(rng: &mut Rng, depth: u32) -> Value {
    let kind_bound = if depth >= 4 { 5 } else { 7 };
    match rng.range(kind_bound) {
        0 => Value::Null,
        1 => Value::Bool(rng.bool()),
        2 => Value::Number(Number::UInt(rng.range(1_000_000))),
        3 => Value::Number(Number::Int(rng.range(2_000_000) as i64 - 1_000_000)),
        4 => Value::Number(Number::Float(arbitrary_finite_float(rng))),
        5 => Value::String(arbitrary_string(rng)),
        6 => {
            let len = rng.range(4);
            Value::Array((0..len).map(|_| arbitrary_value(rng, depth + 1)).collect())
        }
        _ => {
            let len = rng.range(4);
            let mut object = Object::new();
            for index in 0..len {
                object.insert(format!("k{index}_{}", rng.range(1000)), arbitrary_value(rng, depth + 1));
            }
            Value::Object(object)
        }
    }
}

const PROPERTY_TEST_ITERATIONS: u32 = 3000;

#[test]
fn round_trips_arbitrary_values() {
    let mut rng = Rng::new(0xC0FF_EE00_1234_5678);
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let text = to_string(&value);
        let parsed = parse(&text, crate::JsonMemberPolicy::Replace).unwrap_or_else(|error| panic!("case {case}: parse failed: {error}; text={text}"));
        assert_eq!(value, parsed, "case {case}: round-trip mismatch; text={text}");
    }
}
//#endregion 🔖️PropertyTesting

//#region 🔖️DifferentialTesting
/// 🔬️ Structural equality between our `Value` and `serde_json::Value` — deliberately NOT a
/// byte-for-byte text comparison: object key order is allowed to differ (this crate's `Object`
/// is insertion-ordered, `serde_json::Value`'s default `Map` is not) as long as the two trees
/// denote the same value. Float notation no longer needs an exception here — see the
/// `FloatParity` region below — but `values_match` stays a value comparison for the key-order
/// reason. Byte-for-byte agreement is checked separately, both for typical documents
/// (`canonical_bytes_match_serde_json_for_typical_documents`) and exhaustively for `f64` alone
/// (`FloatParity`).
fn values_match(mine: &Value, theirs: &serde_json::Value) -> bool {
    match (mine, theirs) {
        (Value::Null, serde_json::Value::Null) => true,
        (Value::Bool(a), serde_json::Value::Bool(b)) => a == b,
        (Value::String(a), serde_json::Value::String(b)) => a == b,
        (Value::Number(a), serde_json::Value::Number(b)) => number_matches(a, b),
        (Value::Array(a), serde_json::Value::Array(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_match(x, y)),
        (Value::Object(a), serde_json::Value::Object(b)) => a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|bv| values_match(v, bv))),
        _ => false,
    }
}

fn number_matches(mine: &Number, theirs: &serde_json::Number) -> bool {
    match *mine {
        Number::UInt(v) => theirs.as_u64() == Some(v) || theirs.as_f64() == Some(v as f64),
        Number::Int(v) => theirs.as_i64() == Some(v) || theirs.as_f64() == Some(v as f64),
        Number::Float(v) => theirs.as_f64().is_some_and(|t| t == v),
    }
}

#[test]
fn differential_parse_matches_serde_json_on_arbitrary_values() {
    let mut rng = Rng::new(0xD1FF_0000_BEEF_CAFE);
    let mut checked = 0usize;
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let text = to_string(&value);
        let mine = parse(&text, crate::JsonMemberPolicy::Replace).unwrap();
        let theirs: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|error| panic!("case {case}: serde_json rejected our own writer output: {error}; text={text}"));
        assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}\nmine={mine:?}\ntheirs={theirs:?}");
        checked += 1;
    }
}

#[test]
fn differential_cross_parse_serde_json_writer_output() {
    let mut rng = Rng::new(0x5EED_1357_2468_ACE0);
    let mut checked = 0usize;
    for case in 0..PROPERTY_TEST_ITERATIONS {
        let value = arbitrary_value(&mut rng, 0);
        let theirs = to_serde_json(&value);
        let text = serde_json::to_string(&theirs).unwrap();
        let mine = parse(&text, crate::JsonMemberPolicy::Replace).unwrap_or_else(|error| panic!("case {case}: our parser rejected serde_json's writer output: {error}; text={text}"));
        assert!(values_match(&mine, &theirs), "case {case}: structural mismatch; text={text}");
        checked += 1;
    }
}

fn to_serde_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(v) => serde_json::Value::Bool(*v),
        Value::String(v) => serde_json::Value::String(v.clone()),
        Value::Number(Number::UInt(v)) => serde_json::Value::Number((*v).into()),
        Value::Number(Number::Int(v)) => serde_json::Value::Number((*v).into()),
        Value::Number(Number::Float(v)) => serde_json::Number::from_f64(*v).map_or(serde_json::Value::Null, serde_json::Value::Number),
        Value::Array(items) => serde_json::Value::Array(items.iter().map(to_serde_json).collect()),
        Value::Object(object) => serde_json::Value::Object(object.iter().map(|(k, v)| (k.to_string(), to_serde_json(v))).collect()),
    }
}

/// 🔬️ On documents with no object-key-order ambiguity (scalars, arrays, single-key objects),
/// our writer's bytes agree with `serde_json`'s exactly — including large-magnitude floats,
/// now that `FloatParity` below proves the writers agree on every `f64`.
#[test]
fn canonical_bytes_match_serde_json_for_typical_documents() {
    let cases: &[&str] = &[r#"null"#, r#"true"#, r#"false"#, r#"0"#, r#"-17"#, r#"3.5"#, r#""hello""#, r#""café""#, r#"[]"#, r#"{}"#, r#"[1,2,3]"#, r#"{"a":1}"#, r#"{"only":{"one":"key"}}"#, r#"1.5e300"#, r#"5e-300"#, r#"1e21"#, r#"1e-7"#];
    for text in cases {
        let mine = parse(text, crate::JsonMemberPolicy::Replace).unwrap();
        let theirs: serde_json::Value = serde_json::from_str(text).unwrap();
        let mine_bytes = to_string(&mine);
        let their_bytes = serde_json::to_string(&theirs).unwrap();
        assert_eq!(mine_bytes, their_bytes, "byte mismatch for {text}");
    }
}
//#endregion 🔖️DifferentialTesting

//#region 🔖️FloatParity
/// 🔬️ Exhaustive-ish differential sweep proving `write_float`'s output is byte-identical to
/// `serde_json`'s (`zmij`'s) for every `f64` it is handed — a constant-seeded LCG over random
/// bit patterns (this crate's own `Rng`, reused rather than adding a `rand`/`proptest`
/// dependency) plus every historically-awkward case named in this ticket's own brief. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️float-format-parity.md` for the derivation and the full sweep this test's
/// smaller in-crate corpus is drawn from.
fn float_parity_edge_cases() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::MAX,
        f64::MIN,
        1e-7,
        -1e-7,
        1e21,
        1e22,
        5e-324,
        -5e-324,
        1.7976931348623157e308,
        -1.7976931348623157e308,
        0.1,
        0.3,
        1e16,
        1e15,
        9999999999999998.0,
        9007199254740993.0,
        123456789012345.0,
        1234567890123456.0,
        12345678901234567.0,
        100000.0,
        1000000.0,
        99999.0,
        999999999999999.9,
        1.0e-5,
        1.0e-6,
        1.0e-4,
        8322951083873004.0,
        f64::from_bits(0xc316b3096f9dcd35),
        f64::from_bits(0x431b807272ea6281),
        f64::from_bits(0xc9409f0951d8de1a),
        f64::from_bits(0x40f869f000000000),
        f64::from_bits(0x430c6bf52633ffff),
    ]
}

#[test]
fn write_float_matches_serde_json_byte_for_byte() {
    let mut checked = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    for &value in &float_parity_edge_cases() {
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        if mine != theirs {
            mismatches.push(format!("bits={:#018x} value={value:e} mine={mine} theirs={theirs}", value.to_bits()));
        }
        let reparsed: f64 = mine.parse().unwrap_or_else(|error| panic!("our own output {mine:?} failed to reparse: {error}"));
        assert_eq!(reparsed.to_bits(), value.to_bits(), "round-trip bit mismatch for {value:e}, wrote {mine}");
        checked += 1;
    }
    let mut rng = Rng::new(0xF10A_7000_0000_0001);
    for _ in 0..300_000u32 {
        let bits = rng.next_u64();
        let value = f64::from_bits(bits);
        if !value.is_finite() {
            continue;
        }
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        if mine != theirs {
            mismatches.push(format!("bits={bits:#018x} value={value:e} mine={mine} theirs={theirs}"));
        }
        let reparsed: f64 = mine.parse().unwrap_or_else(|error| panic!("our own output {mine:?} failed to reparse: {error}"));
        assert_eq!(reparsed.to_bits(), value.to_bits(), "round-trip bit mismatch for {value:e}, wrote {mine}");
        checked += 1;
    }
    assert!(mismatches.is_empty(), "{} of {checked} floats mismatched serde_json byte-for-byte:\n{}", mismatches.len(), mismatches.join("\n"));
}

/// 🔬️ The two real production call sites this parity result unblocks
/// (`🌿️vcs::content_addressed_checkpoint_id_core`'s `serde_json::to_vec(change)` and
/// `🧵️canonical-edit::ScalarBytes::from_node`'s `serde_json::to_writer`) both serialize a
/// single JSON document containing ordinary application floats, not adversarial bit patterns —
/// this proves byte-identity on a realistic corpus shaped like those payloads (nested objects
/// with float-valued fields, the kind a checkpoint or a canonical scalar actually carries).
#[test]
fn realistic_payloads_byte_match_serde_json() {
    let mut rng = Rng::new(0x0011_2233_4455_6677);
    let mut checked = 0usize;
    for _ in 0..20_000u32 {
        let value = arbitrary_finite_float(&mut rng);
        let mine = to_string(&Value::Number(Number::Float(value)));
        let theirs = serde_json::to_string(&value).unwrap();
        assert_eq!(mine, theirs, "realistic-payload float mismatch for {value:e}");
        checked += 1;
    }
}
//#endregion 🔖️FloatParity

#[test]
fn borrowed_json_source_sink_keeps_caller_prefix_policy_and_original_words(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️source-sink.json")).unwrap();
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
    println!("[DEBUG] Original JSON source pointer and exact Serde words survive caller prefix, full-byte/depth policy and typed sink/cancellation refusal without admitted source or payload mirror; caller output backing is independently preowned and refusal ownership is excluded");
}


#[test]
fn retained_json_borrowed_source_uses_original_pages_utf8_and_refusal_owner(){
    use semio_framework_value::{NativeDecodeControl,ValueRefusalKind,list::PagedList};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-source.json")).unwrap();
    let text=fixture["source"].as_str().unwrap();
    let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
    let pointer=source.get(0).unwrap()as *const u8;
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(fixture["decodeAllocationBytes"].as_u64().unwrap()as usize,&mut accepted);
    let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject);
    let parsed=loop{if let Some(value)=cursor.step(fixture["stepUnits"].as_u64().unwrap()as usize,&mut control).unwrap(){break value;}};
    assert_eq!(cursor.source().get(0).unwrap()as *const u8,pointer);
    assert_eq!(cursor.position(),source.len());
    let oracle:serde_json::Value=serde_json::from_str(text).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&parsed)).unwrap(),oracle);
    assert_eq!(parsed,parse(text,JsonMemberPolicy::Reject).unwrap());
    close_borrowed_json(cursor,131072);
    let large=format!("\"{}\"", "x".repeat(fixture["largeStringBytes"].as_u64().unwrap()as usize));
    let source=PagedList::<u8,16384>::try_from_iter(large.bytes()).unwrap();
    let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject);
    let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeDecodeControl::new(131072,&mut accepted);
    while cursor.phase()!="materialize-string"{assert!(cursor.step(1,&mut control).unwrap().is_none());}
    for _ in 0..64{assert!(cursor.step(1,&mut control).unwrap().is_none());}
    let bytes=control.owned_bytes();assert!(bytes>=fixture["largeStringBytes"].as_u64().unwrap()as usize);
    let position=cursor.position();let pointer=cursor.source().get(0).unwrap()as *const u8;
    live.set(false);assert_eq!(cursor.step(1,&mut control).unwrap_err().into_value_error().kind,ValueRefusalKind::Canceled);
    assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),bytes);assert_eq!(cursor.source().get(0).unwrap()as *const u8,pointer);
    live.set(true);
    let actual=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
    assert_eq!(actual,parse(&large,JsonMemberPolicy::Reject).unwrap());close_borrowed_json(cursor,131072);
    for bytes in[fixture["invalidUtf8"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>(),fixture["duplicate"].as_str().unwrap().as_bytes().to_vec()]{
        let source=PagedList::<u8,16384>::try_from_iter(bytes.iter().copied()).unwrap();let mut cursor=JsonBorrowedParseCursor::new(&source,JsonMemberPolicy::Reject);
        let error=loop{match cursor.step(1,&mut control){Err(error)=>break error,Ok(None)=>{},Ok(Some(_))=>panic!("invalid literal source accepted")}};
        if bytes[1]==237{assert!(matches!(error,JsonError::InvalidUtf8));}else{assert!(matches!(error,JsonError::DuplicateMember{..}));}
        close_borrowed_json(cursor,131072);
    }
    eprintln!("[DEBUG] retained JSON binds original paged source, matches Serde and keeps paid parser/source on interior refusal; candidate retirement uses its separately declared131072 grant");
}


#[test]
fn retained_json_direct_semantic_source_moves_admitted_original_cells_without_tree_mirror(){
    use semio_framework_value::{DslValue,Number,NativeDecodeControl,list::PagedList};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-source.json")).unwrap();
    let text=fixture["source"].as_str().unwrap();let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accepted);
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject);
    let value=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
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
    let mut cursor=JsonBorrowedDslCursor::new(&source,JsonMemberPolicy::Reject);let mut control=NativeDecodeControl::new(131072,&mut accepted);
    let value=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
    assert_eq!(control.owned_bytes(),fixture["largeStringBytes"].as_u64().unwrap()as usize);
    let DslValue::String(value)=value else{panic!("direct owned semantic string")};assert_eq!(value.len(),fixture["largeStringBytes"].as_u64().unwrap()as usize);assert!(value.bytes().all(|byte|byte==b'x'));
    close_borrowed_json(cursor,131072);
    eprintln!("[DEBUG] original retained JSON grammar moves native UInt/Int/Float words and one paid8194 String directly into DslValue; no JSON DOM mirror or source flatten");
}


#[test]
fn retained_json_borrowed_source_honors_complete_caller_extent_depth_and_paid_allocation(){
    use semio_framework_value::{DslValue,NativeDecodeControl,ValueRefusalKind,list::PagedList};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-limits.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let text=row["source"].as_str().unwrap();let source=PagedList::<u8,16384>::try_from_iter(text.bytes()).unwrap();
        let limits=JsonReadLimits{maximum_bytes:text.len()as u64,maximum_allocation_bytes:fixture["decodeAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize,maximum_items:row["maximumItems"].as_u64().unwrap()};
        let mut cursor=JsonBorrowedDslCursor::new_with_limits(&source,JsonMemberPolicy::Reject,limits).unwrap();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(262144,&mut accepted);
        let result=loop{match cursor.step(1,&mut control){Ok(Some(value))=>break Ok(value),Err(error)=>break Err(error),Ok(None)=>{}}};
        assert_eq!(control.maximum_bytes(),262144);
        if row["accept"].as_bool().unwrap(){assert_eq!(serde_json::Value::from(&result.unwrap()),serde_json::from_str::<serde_json::Value>(text).unwrap());}
        else{assert_eq!(result.unwrap_err().into_value_error().kind,match row["kind"].as_str().unwrap(){"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,_=>panic!("closed literal refusal")});}
        close_borrowed_json(cursor,131072);
        let short=JsonReadLimits{maximum_bytes:text.len()as u64-1,..limits};assert_eq!(JsonBorrowedDslCursor::new_with_limits(&source,JsonMemberPolicy::Reject,short).err().unwrap().kind,ValueRefusalKind::WorkLimit);
    }
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'"').chain(std::iter::repeat_n(b'x',fixture["largeStringBytes"].as_u64().unwrap()as usize)).chain(std::iter::once(b'"'))).unwrap();
    let mut accepted=|_|true;let mut control=NativeDecodeControl::new(262144,&mut accepted);
    let limits=JsonReadLimits{maximum_bytes:source.len()as u64,maximum_allocation_bytes:fixture["refusedAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:8,maximum_items:8};
    let mut cursor=JsonBorrowedDslCursor::new_with_limits(&source,JsonMemberPolicy::Reject,limits).unwrap();
    let error=loop{match cursor.step(1,&mut control){Err(error)=>break error,Ok(Some(_))=>panic!("original paid string must refuse"),Ok(None)=>{}}};
    assert_eq!(error.into_value_error().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);assert_eq!(control.maximum_bytes(),262144);
    assert_eq!(cursor.source().len(),source.len());close_borrowed_json(cursor,131072);
    let _=std::mem::size_of::<DslValue>();
    eprintln!("[DEBUG] original bound JSON source keeps complete byte limits, per-collection extents, depth and cumulative allocation with exact caller control restored");
}

#[test]
fn retained_json_cursor_owns_exact_borrowed_source_view(){
    use semio_framework_value::{DslValue,NativeDecodeControl,list::PagedList};
    #[derive(Clone,Copy)]
    struct OriginalView<'source>{owner:&'source PagedList<u8,16384>,start:usize,length:usize}
    impl JsonReadSource for OriginalView<'_>{fn byte_len(&self)->usize{self.length}fn byte_at(&self,index:usize)->Option<u8>{(index<self.length).then(||self.owner.get(self.start+index).copied()).flatten()}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️read-source.json")).unwrap();let text=fixture["source"].as_str().unwrap();
    let source=PagedList::<u8,16384>::try_from_iter(std::iter::once(b'!').chain(text.bytes()).chain(std::iter::once(b'?'))).unwrap();
    let pointer=source.get(1).unwrap()as *const u8;
    let view=OriginalView{owner:&source,start:1,length:text.len()};
    let limits=JsonReadLimits{maximum_bytes:text.len()as u64,maximum_allocation_bytes:131072,maximum_depth:8,maximum_items:4};
    let mut cursor=JsonSourceCursor::<_,DslValue>::new_with_limits(view,JsonMemberPolicy::Reject,limits).unwrap();
    let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeDecodeControl::new(262144,&mut accepted);
    for _ in 0..4{assert!(cursor.step(1,&mut control).unwrap().is_none());}
    let position=cursor.position();let owned=control.owned_bytes();live.set(false);assert_eq!(cursor.step(1,&mut control).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!(cursor.position(),position);assert_eq!(control.owned_bytes(),owned);live.set(true);
    let value=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value}};
    assert_eq!(serde_json::Value::from(&value),serde_json::from_str::<serde_json::Value>(text).unwrap());
    assert_eq!(cursor.source_ref().owner.get(cursor.source_ref().start).unwrap()as *const u8,pointer);assert_eq!(cursor.source_ref().length,text.len());assert_eq!(control.maximum_bytes(),262144);
    close_borrowed_json(cursor,131072);
    assert_eq!(*source.get(0).unwrap(),b'!');assert_eq!(*source.get(source.len()-1).unwrap(),b'?');
    eprintln!("[DEBUG] retained JSON cursor owns the exact immutable bounded view, preserving original pointer and source sentinels across cancellation without flattening");
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
    use semio_framework_value::{NativeDecodeControl,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️borrowed-retirement/🔣️.json")).unwrap();
    let source=law["source"].as_str().unwrap();
    for row in law["cases"].as_array().unwrap(){
        let mut cursor=JsonBorrowedParseCursor::new(source,JsonMemberPolicy::Reject);
        let mut accept=|_|true;let mut control=NativeDecodeControl::new(131072,&mut accept);
        for _ in 0..4{assert!(cursor.step(1,&mut control).unwrap().is_none());}
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
            let parsed=loop{if let Some(value)=cursor.step(1,&mut control).unwrap(){break value;}};
            assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&parsed)).unwrap(),serde_json::from_str::<serde_json::Value>(source).unwrap());
            close_borrowed_json(cursor,65536);
        }
    }
    eprintln!("[DEBUG] borrowed JSON retirement preserves original source and candidate on three independent admission refusals; Serde confirms resumed values");
}
