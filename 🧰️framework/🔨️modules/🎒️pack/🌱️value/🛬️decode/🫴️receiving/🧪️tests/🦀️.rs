//! 🧫️ Independent serde fields survive one-item original Record parsing and paid cancellation.
use super::*;
use semio_framework_dsl_record::{BorrowedFieldSpec,RecordLayout,FieldSpec,RecordSpec};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
#[test]
fn original_dynamic_value_moves_without_cloning_or_abandoning_record_backing(){let mut bank=bank();
 use semio_framework_dsl_record::Shape;
 let expected=DslValue::from(&serde_json::json!({"token":{"generation":"9007199254740993"},"args":{"id":"18446744073709551615","items":[{"bits":"8000000000000000"}]}}));
 let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Value(expected.clone()));
 let declared=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);
 let source=semio_framework_pack_record::encode_record_body(&declared,&record,&Default::default()).unwrap();
 static FIELDS:[BorrowedFieldSpec;1]=[BorrowedFieldSpec::new(1,"value",BorrowedShape::Value)];
 let grant=grant();let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),BorrowedRecordSpec{keyword:None,layout:RecordLayout::Lines,fields:&FIELDS},PackLimits::default(),grant).result.unwrap().unwrap();
 let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut callback);
 with_record_native_work(&mut native,|mut native|{
 for _ in 0..100000{if receiving.is_ready(){break}lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result.unwrap();}
 assert!(receiving.is_ready());
 let payload=match &receiving.owned.output.as_ref().unwrap().get(1).unwrap(){FieldValue::Value(DslValue::Object(fields))=>fields.as_ptr(),_=>unreachable!()};
 let mut output=None;
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  assert_eq!(receiving.take_value_into(1,&mut output,denied).result.unwrap(),Default::default());assert!(output.is_none());
 }
 assert_eq!(receiving.take_value_into(1,&mut output,RetainedCloneGrant{maximum_copy_bytes:0,..grant}).result.unwrap().copied_items,1);
 assert_eq!(output.as_ref(),Some(&expected));assert_eq!(match output.as_ref().unwrap(){DslValue::Object(fields)=>fields.as_ptr(),_=>unreachable!()},payload);
 let mut substitute=None;assert!(receiving.take_value_into(1,&mut substitute,grant).result.is_err());assert!(substitute.is_none());
 while !receiving.terminal_is_empty(){closed(&mut bank,&mut receiving,grant).result.unwrap();}
 let mut output=ControlledRetirement::new(output).ok().unwrap();while !output.terminal_is_empty(){retire(&mut bank,&mut output,grant).unwrap();}

 });
}
static FIELDS:[BorrowedFieldSpec;2]=[BorrowedFieldSpec::new(0,"text",BorrowedShape::Text),BorrowedFieldSpec::new(1,"items",BorrowedShape::List(||BorrowedShape::Text))];
fn spec()->BorrowedRecordSpec{BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS}}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn grant()->RetainedCloneGrant{let f=fixture();let f=&f["turn"];RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize}}
fn source()->(Vec<u8>,RecordValue){let f=fixture();let text:String=serde_json::from_value(f["text"].clone()).unwrap();let items:Vec<String>=serde_json::from_value(f["items"].clone()).unwrap();let mut expected=RecordValue::default();expected.fields.insert(0,FieldValue::Text(text));expected.fields.insert(1,FieldValue::List(items.into_iter().map(FieldValue::Text).collect()));let owned=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",semio_framework_dsl_record::Shape::Text),FieldSpec::new(1,"items",semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Text)))]);let source=semio_framework_pack_record::encode_record_body(&owned,&expected,&Default::default()).unwrap();(source,expected)}
#[test]
fn original_record_receiving_limits_follow_addressable_allocation() {let mut bank=bank();
    let (source, expected) = source();
    let grant = grant();
    for policy in fixture()["allocationPolicies"].as_array().unwrap() {
        let max_total_alloc = policy["maximumBytes"].as_str().unwrap().parse().unwrap();
        let limits = PackLimits { max_total_alloc, ..PackLimits::default() };
        let maximum: usize = policy[format!("limit{}", usize::BITS)].as_str().unwrap().parse().unwrap();
        let (mut receiving, _) = RetainedRecordReceiving::admit(&source, (0, source.len()), spec(), limits, grant).result.unwrap().unwrap();
        assert_eq!(receiving.maximum, maximum);
        let mut callback = |_| true;
        let mut native = NativeDecodeControl::new(1 << 24, &mut callback);
        with_record_native_work(&mut native, |mut native| {
            for _ in 0..10_000 {
                if receiving.is_ready() { break }
                assert!(lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result.unwrap().fits(grant));
            }
            assert!(receiving.is_ready());
            let mut output = None;
            receiving.take_into(&mut output, grant).result.unwrap();
            assert_eq!(output.as_ref(), Some(&expected));
            let mut output = ControlledRetirement::new(output).ok().unwrap();
            while !output.terminal_is_empty() { retire(&mut bank,&mut output,grant).unwrap(); }
            while !receiving.terminal_is_empty() { closed(&mut bank,&mut receiving,grant).result.unwrap(); }
        });
    }
}
#[test]
fn original_record_receiving_one_item_retains_source_native_and_serde_output(){let mut bank=bank();
 let(source,expected)=source();let grant=grant();let mut native_callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut native_callback);
 with_record_native_work(&mut native,|mut native|{let(mut receiving,admitted)=RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),grant).result.unwrap().unwrap();assert_eq!(admitted.copied_items,1);let mut births=0;let mut releases=0;let mut turns=0;
 while !receiving.is_ready(){assert!(turns<10000);let demand=receiving.demands(&source).unwrap();let mut denied=grant;denied.maximum_items=0;assert_eq!(lend(&mut bank,denied,|e|receiving.step(&source,&mut native,e)).result.unwrap(),Default::default());let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result);let progress=result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.retained_capacity_bytes<=demand.capacity_bytes);births+=born;releases+=released;turns+=1;}
 let mut output=None;let progress=receiving.take_into(&mut output,grant).result.unwrap();assert_eq!(progress.copied_items,1);assert_eq!(output.as_ref(),Some(&expected));let output=output.take().unwrap();let mut output=ControlledRetirement::new(output).ok().unwrap();while !output.terminal_is_empty(){let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||retire(&mut bank,&mut output,grant));let progress=step.unwrap().progress();assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;}
 while !receiving.terminal_is_empty(){let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||closed(&mut bank,&mut receiving,grant).result);let progress=step.unwrap().progress();assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;}assert_eq!(births,releases);

 });
}
#[test]
fn original_record_receiving_partial_original_owner_closes_under_independent_release(){let mut bank=bank();
 let(source,_)=source();let grant=grant();let mut observer=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut observer);
 with_record_native_work(&mut native,|mut native|{let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),grant).result.unwrap().unwrap();for _ in 0..50{lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result.unwrap();}let mut original=ControlledRetirement::new(receiving).ok().unwrap();let mut denied=grant;denied.maximum_release_bytes=0;for _ in 0..1000{let progress=retire(&mut bank,&mut original,denied).unwrap().progress();assert_eq!(progress.released_bytes,0);if progress==Default::default(){break}}assert!(!original.terminal_is_empty());for _ in 0..10000{if original.terminal_is_empty(){break}let r=retire(&mut bank,&mut original,grant).unwrap();}assert!(original.terminal_is_empty());

 });
}
fn nested_borrowed()->BorrowedRecordSpec{static FIELDS:[BorrowedFieldSpec;1]=[BorrowedFieldSpec::new(0,"text",BorrowedShape::Text)];BorrowedRecordSpec{keyword:Some("node"),layout:RecordLayout::Inline,fields:&FIELDS}}
fn nested_owned()->RecordSpec{RecordSpec::new(Some("node"),RecordLayout::Inline,vec![FieldSpec::new(0,"text",semio_framework_dsl_record::Shape::Text)])}
fn producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:nested_owned,decoding:|_|Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"fixture only invokes ordinary producer")),encoding:|_|Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"fixture only invokes ordinary producer"))}}
fn nested(text:&str)->RecordValue{let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(text.to_owned()));record}
#[test]
fn original_record_receiving_recursive_containers_and_wire_preserve_independent_json_fields(){let mut bank=bank();
 use semio_framework_dsl_record::{Shape,WireValue,WireNode,WireEdgeLabel};
 static FIELDS:[BorrowedFieldSpec;6]=[BorrowedFieldSpec::new(0,"block",BorrowedShape::Block(||BorrowedShape::Record(nested_borrowed))),BorrowedFieldSpec::new(1,"statements",BorrowedShape::Statements(&[("node",nested_borrowed)])),BorrowedFieldSpec::new(2,"dynamic",BorrowedShape::Value),BorrowedFieldSpec::new(3,"wire",BorrowedShape::Wire),BorrowedFieldSpec::new(4,"unsigned",BorrowedShape::List(||BorrowedShape::UInt)),BorrowedFieldSpec::new(5,"map",BorrowedShape::Map(||BorrowedShape::Text))];
 let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🪆️containers.json")).unwrap();let mut expected=RecordValue::default();expected.fields.insert(0,FieldValue::Block(Box::new(FieldValue::Record(nested(f["nested"].as_str().unwrap())))));expected.fields.insert(1,FieldValue::Statements(f["statements"].as_array().unwrap().iter().map(|text|("node".to_owned(),nested(text.as_str().unwrap()))).collect()));expected.fields.insert(2,FieldValue::Value(DslValue::from(&f["dynamic"])));let w=&f["wire"];expected.fields.insert(3,FieldValue::Wire(WireValue{from:WireNode{id:w["from"].as_str().unwrap().into(),kind:Some(w["kind"].as_str().unwrap().into()),port:Some(w["fromPort"].as_str().unwrap().into())},edge:Some((w["directed"].as_bool().unwrap(),WireNode{id:w["to"].as_str().unwrap().into(),kind:Some(w["kind"].as_str().unwrap().into()),port:Some(w["toPort"].as_str().unwrap().into())})),edge_label:WireEdgeLabel{id:Some(w["label"].as_str().unwrap().into()),kind:Some(w["labelKind"].as_str().unwrap().into())},properties:DslValue::from(&f["dynamic"])}));expected.fields.insert(4,FieldValue::List(serde_json::from_value::<Vec<u64>>(f["unsigned"].clone()).unwrap().into_iter().map(FieldValue::UInt).collect()));expected.fields.insert(5,FieldValue::Map(serde_json::from_value::<Vec<(String,String)>>(f["map"].clone()).unwrap().into_iter().map(|(key,value)|(key,FieldValue::Text(value))).collect()));
 let owned=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"block",Shape::Block(Box::new(Shape::Record(producer())))),FieldSpec::new(1,"statements",Shape::Statements(vec![("node".into(),producer())])),FieldSpec::new(2,"dynamic",Shape::Value),FieldSpec::new(3,"wire",Shape::Wire),FieldSpec::new(4,"unsigned",Shape::List(Box::new(Shape::UInt))),FieldSpec::new(5,"map",Shape::Map(Box::new(Shape::Text)))]);let source=semio_framework_pack_record::encode_record_body(&owned,&expected,&Default::default()).unwrap();let grant=grant();let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS},PackLimits::default(),grant).result.unwrap().unwrap();let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut callback);
 with_record_native_work(&mut native,|mut native|{let mut turns=0;while !receiving.is_ready(){assert!(turns<100000);let progress=lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result.unwrap();assert_eq!(progress.copied_items,1);assert!(progress.fits(grant));turns+=1;}let mut output=None;receiving.take_into(&mut output,grant).result.unwrap();assert_eq!(output.as_ref(),Some(&expected));while !receiving.terminal_is_empty(){closed(&mut bank,&mut receiving,grant).result.unwrap();}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){retire(&mut bank,&mut output,grant).unwrap();}

 });
}

fn table_borrowed()->BorrowedRecordSpec{static FIELDS:[BorrowedFieldSpec;7]=[BorrowedFieldSpec::new(0,"label",BorrowedShape::Text),BorrowedFieldSpec::new(1,"enabled",BorrowedShape::Bool),BorrowedFieldSpec::new(2,"amount",BorrowedShape::Int),BorrowedFieldSpec::new(3,"count",BorrowedShape::UInt),BorrowedFieldSpec::new(4,"ratio",BorrowedShape::Float),BorrowedFieldSpec::new(5,"choice",BorrowedShape::Enum(&[("zero",0),("one",1),("two",2)])),BorrowedFieldSpec::new(6,"child",BorrowedShape::Record(nested_borrowed))];BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS}}
fn table_owned()->RecordSpec{use semio_framework_dsl_record::Shape;RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"label",Shape::Text),FieldSpec::new(1,"enabled",Shape::Bool),FieldSpec::new(2,"amount",Shape::Int),FieldSpec::new(3,"count",Shape::UInt),FieldSpec::new(4,"ratio",Shape::Float),FieldSpec::new(5,"choice",Shape::Enum(vec![("zero".into(),0),("one".into(),1),("two".into(),2)])),FieldSpec::new(6,"child",Shape::Record(producer()))])}
#[test]
fn original_record_receiving_columnar_sparse_table_preserves_independent_rows_and_receipts(){let mut bank=bank();
 use semio_framework_dsl_record::{Shape,RecordSpecProducer};let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📊️table.json")).unwrap();let rows=f["rows"].as_array().unwrap().iter().map(|row|{let mut record=RecordValue::default();for(id,value)in row.as_array().unwrap().iter().enumerate(){let value=if value.is_null(){FieldValue::Absent}else{match id{0=>FieldValue::Text(value.as_str().unwrap().into()),1=>FieldValue::Bool(value.as_bool().unwrap()),2=>FieldValue::Int(value.as_i64().unwrap()),3=>FieldValue::UInt(value.as_u64().unwrap()),4=>FieldValue::Float(value.as_f64().unwrap()),5=>FieldValue::Enum(value.as_u64().unwrap()as u32),6=>FieldValue::Record(nested(value.as_str().unwrap())),_=>unreachable!()}};record.fields.insert(id as u16,value);}FieldValue::Record(record)}).collect();let mut expected=RecordValue::default();expected.fields.insert(0,FieldValue::List(rows));let producer=RecordSpecProducer{ordinary:table_owned,decoding:|_|Err(invalid("fixture uses ordinary metadata")),encoding:|_|Err(invalid("fixture uses ordinary metadata"))};let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"rows",Shape::Table(producer))]);let source=semio_framework_pack_record::encode_record_body(&spec,&expected,&Default::default()).unwrap();static FIELDS:[BorrowedFieldSpec;1]=[BorrowedFieldSpec::new(0,"rows",BorrowedShape::Table(table_borrowed))];let grant=grant();let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS},PackLimits::default(),grant).result.unwrap().unwrap();let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut callback);
 with_record_native_work(&mut native,|mut native|{let mut turns=0;let mut births=0;let mut releases=0;while !receiving.is_ready(){assert!(turns<100000);let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result);let progress=result.unwrap();assert_eq!(progress.copied_items,1);assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(grant));births+=born;releases+=released;turns+=1;}let mut output=None;receiving.take_into(&mut output,grant).result.unwrap();assert_eq!(output.as_ref(),Some(&expected));while !receiving.terminal_is_empty(){let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||closed(&mut bank,&mut receiving,grant).result);let progress=step.unwrap().progress();assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;}let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||retire(&mut bank,&mut output,grant));let progress=step.unwrap().progress();assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;}assert_eq!(births,releases);

 });
}

#[test]
fn original_record_receiving_first_close_defers_partial_owner_cursor_birth(){let mut bank=bank();
 let(source,_)=source();let grant=grant();let(mut owner,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),grant).result.unwrap().unwrap();let mut observe=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut observe);
 with_record_native_work(&mut native,|mut native|{for _ in 0..50{lend(&mut bank,grant,|e|owner.step(&source,&mut native,e)).result.unwrap();}let demand=owner.close_demands(4096).unwrap();assert_eq!(demand.capacity_bytes,0);let no_birth=RetainedCloneGrant{maximum_capacity_bytes:0,..grant};let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||closed(&mut bank,&mut owner,no_birth).result);assert_eq!((born,released),(0,0));assert!(result.unwrap().progress().fits(no_birth));assert!(!owner.terminal_is_empty());let next=owner.close_demands(4096).unwrap();assert!(next.capacity_bytes>0);let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||closed(&mut bank,&mut owner,RetainedCloneGrant{maximum_capacity_bytes:next.capacity_bytes-1,..grant}).result);assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));while !owner.terminal_is_empty(){closed(&mut bank,&mut owner,grant).result.unwrap();}

 });
}

#[test]
fn original_record_receiving_complete_pack_preserves_catalog_body_and_actual_receipts(){let mut bank=bank();
 let(_,expected)=source();let declared=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",semio_framework_dsl_record::Shape::Text),FieldSpec::new(1,"items",semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Text)))]);let source=semio_framework_pack_record::encode_document(&declared,&expected,&Default::default()).unwrap();let source_pointer=source.as_ptr();let grant=grant();
 let max_total_alloc = fixture()["documentReceiving"]["totalAllocationCeiling"].as_str().unwrap().parse().unwrap();
 let limits = PackLimits { max_total_alloc, ..Default::default() };
 let(result,admit_born,admit_released)=semio_framework_trace_memory_testing::observe_backing(||RetainedRecordReceiving::admit_document(&source,(0,source.len()),spec(),limits,grant).result);let(mut receiving,admitted)=result.unwrap().unwrap();assert_eq!((admit_born,admit_released),(admitted.retained_capacity_bytes,admitted.released_bytes));let mut observe=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut observe);
 with_record_native_work(&mut native,|mut native|{let mut turns=0;let(mut born,mut released)=(admit_born,admit_released);
 while !receiving.is_ready(){assert!(turns<100000);let demand=receiving.demands(&source).unwrap();for axis in 0..5{let mut denied=grant;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,3 if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,4=>denied.maximum_depth=demand.depth-1,_=>continue}let(result,physical_born,physical_released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,denied,|e|receiving.step(&source,&mut native,e)).result);assert_eq!(result.unwrap(),Default::default());assert_eq!((physical_born,physical_released),(0,0));}
 let(result,physical_born,physical_released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result);let progress=result.unwrap();assert!(progress.fits(grant));assert_eq!((physical_born,physical_released),(progress.retained_capacity_bytes,progress.released_bytes));born+=physical_born;released+=physical_released;turns+=1;assert_eq!(source.as_ptr(),source_pointer);}
 let mut output=None;receiving.take_into(&mut output,grant).result.unwrap();assert_eq!(output.as_ref(),Some(&expected));while !receiving.terminal_is_empty(){let(result,physical_born,physical_released)=semio_framework_trace_memory_testing::observe_backing(||closed(&mut bank,&mut receiving,grant).result);let progress=result.unwrap().progress();assert_eq!((physical_born,physical_released),(progress.retained_capacity_bytes,progress.released_bytes));born+=physical_born;released+=physical_released;}
 let mut output=ControlledRetirement::retain(output.take().unwrap());while !output.terminal_is_empty(){let(result,physical_born,physical_released)=semio_framework_trace_memory_testing::observe_backing(||retire(&mut bank,&mut output,grant));let progress=result.unwrap().progress();assert_eq!((physical_born,physical_released),(progress.retained_capacity_bytes,progress.released_bytes));born+=physical_born;released+=physical_released;}assert_eq!(born,released);

 });
}

#[test]
fn original_record_receiving_generic_retirement_selects_one_close_event_and_retains_original_source(){let mut bank=bank();
 use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
 let law=fixture();for body in law["correlatedRetirement"]["bodies"].as_array().unwrap(){for cut in law["correlatedRetirement"]["cancelCuts"].as_array().unwrap(){let(source,_)=source();let original=source.clone();let serialized=serde_json::to_value(&source).unwrap();let pointer=source.as_ptr();let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),grant()).result.unwrap().unwrap();let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut callback);
 with_record_native_work(&mut native,|mut native|{for _ in 0..cut.as_u64().unwrap(){lend(&mut bank,grant(),|e|receiving.step(&source,&mut native,e)).result.unwrap();}let mut cursor=receiving.retirement_seed();
 for _ in 0..law["correlatedRetirement"]["maximumTurns"].as_u64().unwrap(){if cursor.terminal_is_empty(){break;}let d=cursor.next_demand(body.as_u64().unwrap()as usize).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.next_demand(g.maximum_copy_bytes).unwrap(),d);
 for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,denied,|e|cursor.close_step(denied.maximum_copy_bytes,e)));assert!((matches!(&step,RetirementStep::Progress(progress,_)if *progress==RetainedCloneProgress::default())||matches!(&step,RetirementStep::Failure(error,_)if axis==4&&error.primary().is_some_and(|p|p.kind==semio_framework_value::ValueRefusalKind::DepthLimit))),"denied Record retirement body={body} cut={cut} axis={axis} demand={d:?} outcome={}",match &step{RetirementStep::Progress(p,_)=>format!("Progress({p:?})"),RetirementStep::Failure(e,_)=>format!("Failure({e})"),RetirementStep::BudgetExhausted=>"BudgetExhausted".into(),RetirementStep::Complete(_)=>"Complete".into(),_=>"child or byte transition".into()});assert_eq!((born,released),(0,0));assert_eq!(cursor.next_demand(g.maximum_copy_bytes).unwrap(),d);assert_eq!(source,original);assert_eq!(source.as_ptr(),pointer);}
 let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,g,|e|cursor.close_step(g.maximum_copy_bytes,e)));let progress=match step{RetirementStep::Progress(progress,_)=>progress,RetirementStep::Complete(_)=>Default::default(),_=>panic!("record retirement refused selected event")};assert!(progress.fits(g));assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));}assert!(cursor.terminal_is_empty());assert_eq!(source,original);assert_eq!(serde_json::to_value(&source).unwrap(),serialized);assert_eq!(source.as_ptr(),pointer);drop(cursor);
 
 });
}}
}

#[test]
fn original_record_receiving_document_driver_admits_every_close_currency_before_effect(){let mut bank=bank();
 let law=fixture();let(_,expected)=source();let declared=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",semio_framework_dsl_record::Shape::Text),FieldSpec::new(1,"items",semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Text)))]);let bytes=semio_framework_pack_record::encode_document(&declared,&expected,&Default::default()).unwrap();let serialized=serde_json::to_value(&bytes).unwrap();
 for cut in law["correlatedRetirement"]["cancelCuts"].as_array().unwrap(){let mut driver=lend(&mut bank,full(),|e|RetainedDocumentDriver::record(PackLimits::default(),bytes.len(),1<<24,e)).unwrap();for _ in 0..cut.as_u64().unwrap(){let(d,born,released)=semio_framework_trace_memory_testing::observe_backing(||driver.normal_demands().unwrap());assert_eq!((born,released),(0,0));lend(&mut bank,full(),|e|driver.one(&bytes,(0,bytes.len()),d.capacity_bytes,e)).unwrap();}
 for _ in 0..law["correlatedRetirement"]["maximumTurns"].as_u64().unwrap(){if driver.terminal_is_empty(){break;}let(d,born,released)=semio_framework_trace_memory_testing::observe_backing(||driver.close_demands(4096).unwrap());assert_eq!((born,released),(0,0));let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(driver.close_demands(g.maximum_copy_bytes).unwrap(),d);let allocated=driver.allocated_bytes();
 for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,denied,|e|driver.close_step(e)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,released),(0,0));assert_eq!(driver.allocated_bytes(),allocated);assert_eq!(driver.close_demands(g.maximum_copy_bytes).unwrap(),d);}
 let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,g,|e|driver.close_step(e)).unwrap());let progress=step.progress();assert!(progress.fits(g));assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));}assert!(driver.terminal_is_empty());drop(driver);assert_eq!(serde_json::to_value(&bytes).unwrap(),serialized);
 }
}

struct PagedOriginal{pages:Vec<Vec<u8>>,extent:usize}
impl RetainedWireByteSource for PagedOriginal{fn original_source_identity(&self)->OriginalWireSourceIdentity{OriginalWireSourceIdentity{range_start:0,address:self as*const Self as usize,extent:self.extent,authority:Some((1,2))}}fn byte_at(&self,index:usize)->Option<u8>{if index>=self.extent{return None}self.pages.get(index/7)?.get(index%7).copied()}}
#[test]fn record_materializer_reads_the_same_original_paged_source_without_contiguous_copy(){let mut bank=bank();let(bytes,expected)=source();let input=PagedOriginal{pages:bytes.chunks(7).map(|page|page.to_vec()).collect(),extent:bytes.len()};let foreign=PagedOriginal{pages:bytes.chunks(7).map(|page|page.to_vec()).collect(),extent:bytes.len()};let g=grant();let(mut receiving,_)=RetainedRecordReceiving::admit(&input,(0,input.extent),spec(),PackLimits::default(),g).result.unwrap().unwrap();assert!(receiving.demands(&foreign).is_err());let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut callback);
 with_record_native_work(&mut native,|mut native|{for _ in 0..100000{if receiving.is_ready(){break}let denied=RetainedCloneGrant{maximum_items:0,..g};assert_eq!(lend(&mut bank,denied,|e|receiving.step(&input,&mut native,e)).result.unwrap(),Default::default());let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,g,|e|receiving.step(&input,&mut native,e)).result);let p=result.unwrap();assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));}assert!(receiving.is_ready());let mut output=None;receiving.take_into(&mut output,g).result.unwrap();assert_eq!(output.as_ref(),Some(&expected));let mut close=ControlledRetirement::new(output).ok().unwrap();while !close.terminal_is_empty(){retire(&mut bank,&mut close,g).unwrap();}while !receiving.terminal_is_empty(){closed(&mut bank,&mut receiving,g).result.unwrap();}
 });
}

#[test]fn record_receiver_header_birth_funds_items_depth_and_moves_with_zero_payload_copy(){let mut bank=bank();let(source,_)=source();let d=RetainedRecordReceiving::birth_demand();let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(grant().maximum_copy_bytes),maximum_depth:d.depth.max(grant().maximum_depth),..grant()};for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),denied).result);assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));}let((mut owner,p),born,released)={let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),g).result);(result.unwrap().unwrap(),born,released)};assert!(p.fits(g));assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!((born,released),(0,0));while !owner.terminal_is_empty(){closed(&mut bank,&mut owner,g).result.unwrap();}}

#[test]
fn original_document_driver_reads_nonzero_paged_range_and_closes_every_cancelled_prefix(){let mut bank=bank();
 let law=fixture();let plan=&law["documentReceiving"]["sourceRange"];let(_,expected)=source();
 let declared=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",semio_framework_dsl_record::Shape::Text),FieldSpec::new(1,"items",semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Text)))]);
 let document=semio_framework_pack_record::encode_document(&declared,&expected,&Default::default()).unwrap();
 let mut bytes:Vec<u8>=serde_json::from_value(plan["prefix"].clone()).unwrap();let start=bytes.len();bytes.extend_from_slice(&document);let end=bytes.len();bytes.extend(serde_json::from_value::<Vec<u8>>(plan["suffix"].clone()).unwrap());
 assert_eq!(plan["pageBytes"].as_u64(),Some(7));
 let input=PagedOriginal{pages:bytes.chunks(7).map(|page|page.to_vec()).collect(),extent:bytes.len()};let identity=input.original_source_identity();let pointers=input.pages.iter().map(|page|page.as_ptr()).collect::<Vec<_>>();let source:&dyn RetainedWireByteSource=&input;
 for cut in plan["cancelCuts"].as_array().unwrap(){
  let birth=RetainedRecordReceiving::birth_demand();let g=RetainedCloneGrant{maximum_copy_bytes:grant().maximum_copy_bytes.max(birth.copy_bytes),maximum_depth:grant().maximum_depth.max(birth.depth),..grant()};for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:birth.depth-1,..g}]{let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||RetainedRecordReceiving::admit_document(source,(start,end),spec(),PackLimits::default(),denied).result);assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!(source.original_source_identity(),identity);}let((mut receiving,admitted),born,released)={let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||RetainedRecordReceiving::admit_document(source,(start,end),spec(),PackLimits::default(),g).result);(result.unwrap().unwrap(),born,released)};
  assert!(admitted.fits(g));assert_eq!((born,released),(admitted.retained_capacity_bytes,admitted.released_bytes));let(mut births,mut releases)=(born,released);let mut callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut callback);
 with_record_native_work(&mut native,|mut native|{
  for _ in 0..cut.as_u64().unwrap(){
   if receiving.is_ready(){break}
   let d=receiving.demands(source).unwrap();
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,denied,|e|receiving.step(source,&mut native,e)).result);assert_eq!(result.unwrap(),Default::default());assert_eq!((born,released),(0,0));}
   let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,g,|e|receiving.step(source,&mut native,e)).result);let progress=result.unwrap();assert!(progress.fits(g));assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;
  }
  let mut output=None;if cut.as_u64().unwrap()==100000{assert!(receiving.is_ready());receiving.take_into(&mut output,g).result.unwrap();assert_eq!(output.as_ref(),Some(&expected));}
  for _ in 0..100000{if receiving.terminal_is_empty(){break}let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||closed(&mut bank,&mut receiving,g).result);let progress=result.unwrap().progress();assert!(progress.fits(g));assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;}assert!(receiving.terminal_is_empty());
  let mut output=ControlledRetirement::new(output).ok().unwrap();for _ in 0..100000{if output.terminal_is_empty(){break}let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||retire(&mut bank,&mut output,g));let progress=result.unwrap().progress();assert!(progress.fits(g));assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=released;}assert!(output.terminal_is_empty());assert_eq!(births,releases);
  assert_eq!(source.original_source_identity(),identity);for(index,page)in input.pages.iter().enumerate(){assert_eq!(page.as_ptr(),pointers[index]);assert_eq!(page.as_slice(),&bytes[index*7..bytes.len().min((index+1)*7)]);}
 
 });
}
}
#[test]
fn original_record_receiving_every_close_step_after_all_rows_reports_funded_progress(){let mut bank=bank();
 let(source,_)=source();let grant=grant();let mut native_callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut native_callback);
 with_record_native_work(&mut native,|mut native|{let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),grant).result.unwrap().unwrap();let mut turns=0;
 while !receiving.is_ready(){assert!(turns<10000);lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result.unwrap();turns+=1;}
 let mut steps=0;while !receiving.terminal_is_empty(){assert!(steps<10000);let progress=closed(&mut bank,&mut receiving,grant).result.unwrap().progress();assert_ne!(progress,Default::default(),"a non-terminal close turn must report funded progress");steps+=1;}

 });
}
#[test]
fn original_record_receiving_birth_quote_fits_the_fixture_turn_grant(){
 assert!(RetainedRecordReceiving::birth_demand().copy_bytes<=grant().maximum_copy_bytes,"fixed header quote must stay inside the principled turn grant");
}
#[test]
fn original_record_receiving_close_never_reports_progress_without_an_item_grant(){let mut bank=bank();
 let(source,_)=source();let grant=grant();let starved=RetainedCloneGrant{maximum_items:0,..grant};let mut native_callback=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut native_callback);
 with_record_native_work(&mut native,|mut native|{let(mut receiving,_)=RetainedRecordReceiving::admit(&source,(0,source.len()),spec(),PackLimits::default(),grant).result.unwrap().unwrap();let mut turns=0;
 while !receiving.is_ready(){assert!(turns<10000);lend(&mut bank,grant,|e|receiving.step(&source,&mut native,e)).result.unwrap();turns+=1;}
 for _ in 0..64{assert_eq!(closed(&mut bank,&mut receiving,starved).result.unwrap().progress(),Default::default(),"a starved close turn must not fabricate item progress");assert!(!receiving.terminal_is_empty());}
 let mut steps=0;while !receiving.terminal_is_empty(){assert!(steps<10000);closed(&mut bank,&mut receiving,grant).result.unwrap();steps+=1;}

 });
}

#[test]
fn original_record_header_quotes_only_present_logical_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🎟️work/case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("🎟️work/case.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v);}
 let body=RetainedRecordReceiving::birth_demand();let document=RetainedRecordReceiving::document_birth_demand();assert_eq!(body.copy_bytes,law["headerCopyBytes"].as_u64().unwrap()as usize);assert_eq!(document.copy_bytes,body.copy_bytes);assert_eq!(body.capacity_bytes,law["headerCapacityBytes"].as_u64().unwrap()as usize);assert_eq!(document.capacity_bytes,body.capacity_bytes);
}

#[test]
fn original_record_receiving_preserves_null_attempt_work_without_phantom_native_bytes(){let mut bank=bank();
 let law:serde_json::Value=serde_json::from_str(include_str!("🎟️work/case.json")).unwrap();let(input,_)=source();let g=grant();
 for document in [false,true]{
  let turn=if document{RetainedRecordReceiving::admit_document(&input,(0,input.len()),spec(),PackLimits::default(),g)}else{RetainedRecordReceiving::admit(&input,(0,input.len()),spec(),PackLimits::default(),g)};
  assert_eq!(turn.work,u128::from(law["headerWork"].as_u64().unwrap()));let(mut owner,p)=turn.result.unwrap().unwrap();assert_eq!((p.copied_bytes,p.retained_capacity_bytes),(0,0));
  let mut observer=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut observer);
 with_record_native_work(&mut native,|mut native|{
  if document{let turn=lend(&mut bank,g,|e|owner.step(&input,&mut native,e));assert_eq!(turn.work,1);turn.result.unwrap();assert!(owner.owned.frames.capacity()>0);}
  let owned=native.owned_bytes();let((turn,attempts),born,released)=semio_framework_trace_memory_testing::observe_backing(||semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||lend(&mut bank,g,|e|owner.step(&input,&mut native,e))));
  assert_eq!(attempts,1);assert_eq!(turn.work,u128::from(law["nullAllocationWork"].as_u64().unwrap()));let error=turn.result.unwrap_err();assert_eq!(error.primary.as_ref().unwrap().kind,ValueRefusalKind::AllocationFailed);assert_eq!(error.progress.retained_capacity_bytes,law["nullAllocationCapacityBytes"].as_u64().unwrap()as usize);assert_eq!((born,released),(0,0));assert_eq!(native.owned_bytes(),owned);
  let(cached,born,released)=semio_framework_trace_memory_testing::observe_backing(||lend(&mut bank,g,|e|owner.step(&input,&mut native,e)));assert!(cached.result.is_err());assert_eq!(cached.work,u128::from(law["cachedWork"].as_u64().unwrap()));assert_eq!((born,released),(0,0));
  for _ in 0..10000{if owner.terminal_is_empty(){break}closed(&mut bank,&mut owner,g).result.unwrap();}assert!(owner.terminal_is_empty());
 
 });
}
}

struct RecordWorkStock{units:u64}
impl semio_framework_value_native_owned::native_decoding::OriginalWorkRecipient for RecordWorkStock{type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(Ok(()))}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:semio_framework_value_native_owned::native_decoding::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->semio_framework_value_native_owned::native_decoding::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=self.units)))}
}
fn with_record_native_work<T>(native:&mut NativeDecodeControl<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->T{
 let law:serde_json::Value=serde_json::from_str(include_str!("🎟️work/case.json")).unwrap();let mut original=semio_framework_value_native_owned::native_decoding::OriginalWorkAuthority::new(RecordWorkStock{units:law["workUnits"].as_u64().unwrap()});original.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|Ok::<_,ValueError>(operation(native)))).unwrap().unwrap()
}
const EFFECT_WORK:u64=1<<32;
/// 🏦️ Seats one fixture Source whose finite stock never binds, so each law lends real original effects at its own turn grants.
fn bank()->EffectBank{let all=RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX};EffectBank::new(Wallet::new(all,u64::MAX))}
/// 🧱️ Funds one Document driver action, whose own quote is not part of the record turn grant.
fn full()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:256}}
/// 🪢️ Runs one action inside a real original effect at the grant and returns what the action produced.
fn lend<R>(bank:&mut EffectBank,grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{let mut out=None;let(poll,_)=bank.run(grant,EFFECT_WORK,|effect|{let mut run=Some(run);semio_framework_value::retained_clone::with_original_invocation(None,effect,&mut |effect|out=Some(run.take().expect("fixture effect ran once")(effect))).unwrap();semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()})});assert!(matches!(poll,semio_framework_value::RetainedWorkPoll::Complete(_)));assert_ne!(bank.take_admitted(),Some(false));out.expect("fixture effect ran its action")}
/// ⌛️ Settles one synchronous close poll; record retirement never stays pending.
fn settled<T,E>(poll:semio_framework_value::RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{..}=>panic!("synchronous record close stayed pending")}}
/// 🧹️ Runs one record receiver close poll at the grant, its copy grant serving as the selected body.
fn closed(bank:&mut EffectBank,receiving:&mut RetainedRecordReceiving,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{settled(lend(bank,grant,|effect|receiving.close_step(grant.maximum_copy_bytes,effect)))}
/// 🗑️ Runs one controlled retirement poll at the grant, its copy grant serving as the selected body.
fn retire<T:semio_framework_value::retirement::RetireOwned>(bank:&mut EffectBank,owner:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{settled(lend(bank,grant,|effect|owner.step(grant.maximum_copy_bytes,effect))).result}
