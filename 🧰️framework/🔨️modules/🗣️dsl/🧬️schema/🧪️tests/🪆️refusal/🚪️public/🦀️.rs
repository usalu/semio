//! 🚪️ External derived fields retain typed refusal identity and their authored field path.
use semio_framework_dsl_record::{DslField,FieldValue,RecordValue,Shape};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
#[derive(Debug)]
struct RefusingField(ValueError);
fn refusal(value:&FieldValue)->ValueError {
    let FieldValue::Text(text)=value else { panic!("authored refusal carrier") };
    let row:serde_json::Value=serde_json::from_str(text).unwrap();
    let kind=match row["kind"].as_str().unwrap() {
        "InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,
        "OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,
        "WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,
        "UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,
        _=>panic!("unknown authored refusal kind"),
    };
    ValueError::new(kind,row["message"].as_str().unwrap())
}
impl DslField for RefusingField {
    fn shape()->Shape {Shape::Text}
    fn to_value(&self)->FieldValue {panic!("ordinary projection must not run")}
    fn from_value(_:&FieldValue)->Result<Self,String> {panic!("ordinary construction must not run")}
    fn to_value_controlled(&self,_:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{Err(self.0.clone())}
    fn to_record_controlled(&self,_:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{Err(self.0.clone())}
    fn from_value_controlled(value:&FieldValue,_:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{Err(refusal(value))}
    fn from_record_controlled(record:&RecordValue,_:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{Err(refusal(record.fields.get(&0).expect("authored refusal field")))}
}
#[derive(Debug,semio_framework_dsl_record_derive::DslRecord)]
struct DerivedField {field:RefusingField}
#[test]
fn external_derived_fields_retain_every_refusal_kind_and_authored_path() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪆️refusal/🔣️.json")).unwrap();
    for kind in fixture["kinds"].as_array().unwrap() {
        for row in fixture["derivedPaths"].as_array().unwrap() {
            let mut record=RecordValue::default();
            record.fields.insert(0,FieldValue::Text(serde_json::json!({"kind":kind,"message":row["message"]}).to_string()));
            let cause=refusal(record.fields.get(&0).expect("authored refusal field"));
            let expected_kind=cause.kind;
            let source=DerivedField{field:RefusingField(cause)};
            let mut accept=|_|true;
            let mut encode=NativeEncodeControl::new(65536,&mut accept);
            let mut accept=|_|true;
            let mut decode=NativeDecodeControl::new(65536,&mut accept);
            let error=match row["method"].as_str().unwrap() {
                "projectValue"=>source.to_value_controlled(&mut encode).unwrap_err(),
                "projectRecord"=>source.to_record_controlled(&mut encode).unwrap_err(),
                "constructValue"=>DerivedField::from_value_controlled(&FieldValue::Record(record),&mut decode).unwrap_err(),
                "constructRecord"=>DerivedField::from_record_controlled(&record,&mut decode).unwrap_err(),
                _=>panic!("unknown authored derived path"),
            };
            assert_eq!(error.kind,expected_kind);
            assert_eq!(error.message,row["expectedMessage"].as_str().unwrap());
        }
    }
}

use semio_framework_dsl_record::{print,JoinMode};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ProjectionBlock{value:String}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ProjectionRow{label:String,cost:f64,optional:Option<String>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ProjectionDocument{label:String,rows:Vec<ProjectionRow>,#[dsl(block)]child:Option<ProjectionBlock>}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ProjectionIntrinsicDocument{payload:semio_framework_value::DslValue}

#[test]
fn retained_derived_projection_reads_original_intrinsic_payload_without_whole_clone(){
    use semio_framework_dsl_record::native_encoding::RetainedFieldProjection;
    use semio_framework_value::{DslValue,Number,SnapshotRetirementStep};
    use semio_framework_value::retirement::owned_retirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();assert_eq!(fixture["projection"]["intrinsic"],true);
    let prefix=fixture["intrinsic"]["keyPrefix"].as_str().unwrap().repeat(fixture["intrinsic"]["keyRepeats"].as_u64().unwrap()as usize);
    let text=fixture["textUnit"].as_str().unwrap().repeat(fixture["intrinsic"]["textRepeats"].as_u64().unwrap()as usize);
    let source=ProjectionIntrinsicDocument{payload:DslValue::Object(vec![(format!("{prefix}z"),DslValue::Array(vec![DslValue::Bool(false),DslValue::Null,DslValue::Number(Number::UInt(3))])),(format!("{prefix}a"),DslValue::Object(vec![("bytes".into(),DslValue::Bytes(fixture["intrinsic"]["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect())),("text".into(),DslValue::String(text.clone()))])),("signed".into(),DslValue::Number(Number::Int(1)))])};
    let expected=source.to_value();let spec=ProjectionIntrinsicDocument::__dsl_spec();let FieldValue::Record(record)=&expected else{panic!("record")};let ordinary=print(record,&spec,JoinMode::Inline);assert!(ordinary.contains(&serde_json::to_string(&text).unwrap()));assert!(ordinary.contains("int64(1)"));
    let retire=|cursor:RetainedFieldProjection<ProjectionIntrinsicDocument>|{let mut close=owned_retirement(cursor);let mut turns=0;while !close.terminal_is_empty(){turns+=1;assert!(turns<1000000);if let SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}};
    for budget in fixture["budgets"].as_array().unwrap(){let mut cursor=RetainedFieldProjection::new(&source);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(2000000,&mut accepted);assert!(cursor.step(&source,1,&mut control).unwrap().is_none());let mut turns=0;let actual=loop{let before=cursor.position();assert!(cursor.step(&source,0,&mut control).unwrap().is_none());assert_eq!(cursor.position(),before);turns+=1;assert!(turns<1000000);if let Some(value)=cursor.step(&source,budget.as_u64().unwrap()as usize,&mut control).unwrap(){break value}};assert!(turns>1);assert_eq!(actual,expected);let mut close=owned_retirement(actual);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}retire(cursor);}
    for stop in [0,1,8,64,256,1024]{let mut cursor=RetainedFieldProjection::new(&source);let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeEncodeControl::new(2000000,&mut accepted);for _ in 0..stop{assert!(cursor.step(&source,1,&mut control).unwrap().is_none())}let before=cursor.position();live.set(false);assert_eq!(cursor.step(&source,1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(cursor.position(),before);retire(cursor);}
    eprintln!("[DEBUG] Original intrinsic source projection budgets1/8/256, UTF8/bytes/signed variants, zero/cancel and3byte retirement passed");
}

#[test]
fn retained_derived_projection_yields_resumes_refuses_stale_source_and_retires(){
    use semio_framework_dsl_record::native_encoding::RetainedFieldProjection;
    use semio_framework_value::{ValueRefusalKind,SnapshotRetirementStep};
    use semio_framework_value::retirement::owned_retirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();
    let text=fixture["textUnit"].as_str().unwrap().repeat(fixture["projection"]["textRepeats"].as_u64().unwrap()as usize);
    let source=ProjectionDocument{label:text.clone(),rows:(0..fixture["projection"]["rows"].as_u64().unwrap()).map(|index|ProjectionRow{label:format!("{index}{text}"),cost:index as f64,optional:if index%2==0{Some(text.clone())}else{None}}).collect(),child:Some(ProjectionBlock{value:text.clone()})};
    let expected=source.to_value();let spec=ProjectionDocument::__dsl_spec();let FieldValue::Record(record)=&expected else{panic!("record")};let ordinary=print(record,&spec,JoinMode::Inline);assert!(ordinary.contains(&serde_json::to_string(&text).unwrap()));
    let retire=|cursor:RetainedFieldProjection<ProjectionDocument>|{let mut close=owned_retirement(cursor);assert!(matches!(close.close_step(0,0).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}));let mut turns=0;while !close.terminal_is_empty(){turns+=1;assert!(turns<1000000);if let SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}};
    for budget in fixture["budgets"].as_array().unwrap(){let mut cursor=RetainedFieldProjection::new(&source);let mut callback=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut callback);assert!(cursor.step(&source,1,&mut control).unwrap().is_none());let mut turns=0;let actual=loop{let before=cursor.position();assert!(cursor.step(&source,0,&mut control).unwrap().is_none());assert_eq!(cursor.position(),before);turns+=1;assert!(turns<1000000);if let Some(value)=cursor.step(&source,budget.as_u64().unwrap()as usize,&mut control).unwrap(){break value}};assert!(turns>1);assert_eq!(actual,expected);retire(cursor);}
    for stop in [0,1,32,4096]{let mut cursor=RetainedFieldProjection::new(&source);let accepted=std::cell::Cell::new(true);let mut callback=|_|accepted.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop{assert!(cursor.step(&source,1,&mut control).unwrap().is_none())}let before=cursor.position();accepted.set(false);assert_eq!(cursor.step(&source,1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(cursor.position(),before);retire(cursor);}
    let changed=ProjectionDocument{label:"stale".into(),rows:Vec::new(),child:None};let mut cursor=RetainedFieldProjection::new(&source);let mut callback=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut callback);assert_eq!(cursor.step(&changed,1,&mut control).unwrap_err().kind,ValueRefusalKind::InvariantViolated);retire(cursor);
    let mut cursor=RetainedFieldProjection::new(&source);let mut control=NativeEncodeControl::new(1,&mut callback);assert_eq!(cursor.step(&source,1,&mut control).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);retire(cursor);
    eprintln!("[DEBUG] Retained derived source projection budgets1/8/256 first-yield zero/cancel/stale/admission3byte retirement and independentSerde quoting passed");
}

struct RankedProjectionSource{values:semio_framework_value::ordered::OrderedMap<ProjectionRow>}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for RankedProjectionSource{
    fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{use semio_framework_dsl_record::native_encoding::{FieldProjectionView as V,projection_path_error};match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::Map(self.values.len())),[0,index,rest @ ..]=>DslField::projection_view(self.values.entry_at_rank(*index).ok_or_else(projection_path_error)?.1,rest),_=>Err(projection_path_error())}}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[0]{self.values.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error)}else{Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
}
#[test]
fn retained_projection_reads_ranked_primary_map_and_copies_keys_per_scalar(){
    use semio_framework_dsl_record::native_encoding::RetainedFieldProjection;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();let prefix=fixture["intrinsic"]["keyPrefix"].as_str().unwrap().repeat(fixture["intrinsic"]["keyRepeats"].as_u64().unwrap()as usize);let mut values=semio_framework_value::ordered::OrderedMap::new();for(index,key)in fixture["containers"]["keys"].as_array().unwrap().iter().enumerate(){values.insert(format!("{prefix}{}",key.as_str().unwrap()),ProjectionRow{label:format!("{index}Mesh 😀"),cost:index as f64,optional:None});}let source=RankedProjectionSource{values};let expected=FieldValue::Record(RecordValue{fields:[(0,FieldValue::Map(source.values.iter().map(|(key,value)|(key.clone(),value.to_value())).collect()))].into_iter().collect()});
    let retire=|cursor:RetainedFieldProjection<RankedProjectionSource>|{let mut close=semio_framework_value::retirement::owned_retirement(cursor);while !close.terminal_is_empty(){if let semio_framework_value::SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}};
    for budget in fixture["budgets"].as_array().unwrap(){let mut cursor=RetainedFieldProjection::new(&source);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(1000000,&mut accepted);let mut turns=0;let actual=loop{let before=cursor.position();assert!(cursor.step(&source,0,&mut control).unwrap().is_none());assert_eq!(cursor.position(),before);turns+=1;assert!(turns<100000);if let Some(value)=cursor.step(&source,budget.as_u64().unwrap()as usize,&mut control).unwrap(){break value}};assert_eq!(actual,expected);let FieldValue::Record(record)=&actual else{panic!("record")};let spec=semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,vec![semio_framework_dsl_record::FieldSpec::new(0,"values",Shape::Map(Box::new(Shape::Record(ProjectionRow::__dsl_spec_producer()))))]);let text=print(record,&spec,JoinMode::Inline);assert!(text.contains(&serde_json::to_string(&format!("{prefix}Mesh 😀")).unwrap()));retire(cursor);}
    for stop in [1,8,64,256,1024]{let mut cursor=RetainedFieldProjection::new(&source);let live=std::cell::Cell::new(true);let mut callback=|_|live.get();let mut control=NativeEncodeControl::new(1000000,&mut callback);for _ in 0..stop{assert!(cursor.step(&source,1,&mut control).unwrap().is_none())}live.set(false);let before=cursor.position();assert_eq!(cursor.step(&source,1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(cursor.position(),before);retire(cursor);}
    let mut source=source.values.retire();loop{match source.advance(semio_framework_value::ordered::Grant{maximum_items:1,maximum_bytes:65536}){semio_framework_value::ordered::RetirementStep::Complete=>break,semio_framework_value::ordered::RetirementStep::OwnedValue(value)=>drop(value),_=>{}}}
}

struct ProjectionTaggedWireSource{rows:Vec<ProjectionRow>,wire:semio_framework_dsl_record::Wire}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for ProjectionTaggedWireSource{
    fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{use semio_framework_dsl_record::native_encoding::{FieldProjectionView as V,projection_path_error};match path{[]=>Ok(V::Record(&[0,1])),[0]=>Ok(V::Statements(self.rows.len())),[0,index,rest @ ..]=>DslField::projection_view(self.rows.get(*index).ok_or_else(projection_path_error)?,rest),[1,rest @ ..]=>DslField::projection_view(&self.wire,rest),_=>Err(projection_path_error())}}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[0]&&index<self.rows.len(){Ok("row")}else if let [1,rest @ ..]=path{DslField::projection_key(&self.wire,rest,index)}else{Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
}
#[test]
fn retained_projection_reads_original_tagged_rows_and_wire_payload(){
    use semio_framework_dsl_record::{native_encoding::RetainedFieldProjection,Wire,WireValue,WireNode,WireEdgeLabel};
    use semio_framework_value::{DslValue,SnapshotRetirementStep};use semio_framework_value::retirement::owned_retirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();assert_eq!(fixture["projection"]["taggedAndWire"],true);
    let text=fixture["textUnit"].as_str().unwrap().repeat(fixture["projection"]["textRepeats"].as_u64().unwrap()as usize);
    let source=ProjectionTaggedWireSource{rows:vec![ProjectionRow{label:text.clone(),cost:1.5,optional:None},ProjectionRow{label:"second".into(),cost:2.,optional:Some(text.clone())}],wire:Wire(WireValue{from:WireNode{id:text.clone(),kind:Some("input".into()),port:Some(text.clone())},edge:Some((true,WireNode{id:"target".into(),kind:None,port:Some("value".into())})),edge_label:WireEdgeLabel{id:Some(text.clone()),kind:Some("link".into())},properties:DslValue::Object(vec![("payload".into(),DslValue::String(text.clone()))])})};
    let expected=FieldValue::Record(RecordValue{fields:[(0,FieldValue::Statements(source.rows.iter().map(|row|{let FieldValue::Record(record)=row.to_value()else{panic!("record")};("row".into(),record)}).collect())),(1,source.wire.to_value())].into_iter().collect()});
    let retire=|cursor:RetainedFieldProjection<ProjectionTaggedWireSource>|{let mut close=owned_retirement(cursor);assert!(matches!(close.close_step(0,0).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0}));let mut turns=0;while !close.terminal_is_empty(){turns+=1;assert!(turns<1000000);if let SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}};
    assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&text).unwrap()).unwrap(),text);
    for budget in fixture["budgets"].as_array().unwrap(){let mut cursor=RetainedFieldProjection::new(&source);let mut accept=|_|true;let mut control=NativeEncodeControl::new(2000000,&mut accept);assert!(cursor.step(&source,1,&mut control).unwrap().is_none());let actual=loop{let before=cursor.position();assert!(cursor.step(&source,0,&mut control).unwrap().is_none());assert_eq!(cursor.position(),before);if let Some(value)=cursor.step(&source,budget.as_u64().unwrap()as usize,&mut control).unwrap(){break value}};assert_eq!(actual,expected);let mut close=owned_retirement(actual);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}retire(cursor);}
    for stop in [0,1,8,64,1024,8192]{let mut cursor=RetainedFieldProjection::new(&source);let live=std::cell::Cell::new(true);let mut accept=|_|live.get();let mut control=NativeEncodeControl::new(2000000,&mut accept);for _ in 0..stop{assert!(cursor.step(&source,1,&mut control).unwrap().is_none())}let before=cursor.position();live.set(false);assert_eq!(cursor.step(&source,1,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(cursor.position(),before);retire(cursor);}
    eprintln!("[DEBUG] Original tagged rows and wire Unicode/property source projection budgets1/8/256 zero/cancel and3byte retirement passed");
}

#[test]
fn retained_projection_measures_original_source_before_candidate_allocation(){
    use semio_framework_dsl_record::native_encoding::RetainedFieldProjection;
    use semio_framework_value::{DslValue,SnapshotRetirementStep};
    use semio_framework_value::retirement::owned_retirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧵️continuation/🔣️.json")).unwrap();assert_eq!(fixture["projection"]["sourceCapacity"]["measureBeforeAllocation"],true);
    let text=fixture["textUnit"].as_str().unwrap().repeat(fixture["projection"]["textRepeats"].as_u64().unwrap()as usize);let pointer=text.as_ptr();let bytes=vec![0,255,42];
    let source=ProjectionIntrinsicDocument{payload:DslValue::Object(vec![("text".into(),DslValue::String(text)),("bytes".into(),DslValue::Bytes(bytes))])};
    let text=source.payload.as_object().unwrap()[0].1.as_str().unwrap();assert_eq!(text.as_ptr(),pointer);let quoted=serde_json::to_string(text).unwrap();
    let expected=std::mem::size_of::<(u16,FieldValue)>()+2*std::mem::size_of::<(String,DslValue)>()+"text".len()+"bytes".len()+text.len()+3;
    for budget in fixture["budgets"].as_array().unwrap(){
        let mut cursor=RetainedFieldProjection::new(&source);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let before=control.owned_bytes();assert!(cursor.measure_step(&source,0,expected,&mut control).unwrap().is_none());assert_eq!(control.owned_bytes(),before);assert!(cursor.measure_step(&source,1,expected,&mut control).unwrap().is_none());
        let mut turns=0;let measured=loop{turns+=1;assert!(turns<100000);if let Some(bytes)=cursor.measure_step(&source,budget.as_u64().unwrap()as usize,expected,&mut control).unwrap(){break bytes}};assert_eq!(measured,expected);assert_eq!(control.owned_bytes(),before,"source measurement must not allocate a payload candidate");
        let actual=loop{if let Some(value)=cursor.step(&source,budget.as_u64().unwrap()as usize,&mut control).unwrap(){break value}};let FieldValue::Record(record)=&actual else{panic!("record")};assert!(print(record,&ProjectionIntrinsicDocument::__dsl_spec(),JoinMode::Inline).contains(&quoted));assert_eq!(source.payload.as_object().unwrap()[0].1.as_str().unwrap().as_ptr(),pointer);
        let mut close=owned_retirement((actual,cursor));while !close.terminal_is_empty(){if let SnapshotRetirementStep::Pending{released_bytes,..}=close.close_step(1,3).unwrap(){assert!(released_bytes<=3)}}
    }
    let mut cursor=RetainedFieldProjection::new(&source);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);let error=loop{match cursor.measure_step(&source,1,expected-fixture["projection"]["sourceCapacity"]["overSourceBytes"].as_u64().unwrap()as usize,&mut control){Err(error)=>break error,Ok(None)=>{},Ok(Some(_))=>panic!("over-cap source admitted")}};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.owned_bytes(),0);let mut close=owned_retirement(cursor);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}
    let mut cursor=RetainedFieldProjection::new(&source);let live=std::cell::Cell::new(true);let mut accepted=|_|live.get();let mut control=NativeEncodeControl::new(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);assert!(cursor.measure_step(&source,1,expected,&mut control).unwrap().is_none());let before=cursor.position();live.set(false);assert_eq!(cursor.measure_step(&source,1,expected,&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(cursor.position(),before);let mut close=owned_retirement(cursor);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}
    let changed=ProjectionIntrinsicDocument{payload:DslValue::Null};let mut cursor=RetainedFieldProjection::new(&source);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);assert_eq!(cursor.measure_step(&changed,1,expected,&mut control).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(control.owned_bytes(),0);let mut close=owned_retirement(cursor);while !close.terminal_is_empty(){close.close_step(1,3).unwrap();}
    eprintln!("[DEBUG] original source measured before candidate allocation; exact source demand={expected}, budgets1/8/256 zero/cancel/one-byte refusal; independentSerde scalar parity");
}
