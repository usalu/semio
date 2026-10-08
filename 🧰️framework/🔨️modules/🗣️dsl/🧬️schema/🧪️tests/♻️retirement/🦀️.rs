//! ♻️ The neutral finite Record owner corpus agrees with actual allocator receipts.
use semio_framework_dsl_record::{ExprOp,ExprValue,FieldSpec,FieldValue,JoinMode,RecordFields,RecordLayout,RecordSpec,RecordSpecProducer,RecordValue,RetainedRecordWriter,Shape,WireEdgeLabel,WireNode,WireValue};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{DslValue,NativeDecodeControl,NativeEncodeControl,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};

#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/🔣️.json")).unwrap()}
fn spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",Shape::Text)])}
fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary:spec,decoding:|_:&mut NativeDecodeControl<'_>|Ok(spec()),encoding:|_:&mut NativeEncodeControl<'_>|Ok(spec())}}
fn record(text:&str)->RecordValue{RecordValue{fields:[(0,FieldValue::Text(text.to_owned()))].into_iter().collect()}}
fn node(options:u64)->WireNode{WireNode{id:"node🧬".into(),kind:(options&1!=0).then(||"kindö".into()),port:(options&2!=0).then(||"port".into())}}
fn wire(from:u64,edge:&serde_json::Value,label:u64,law:&serde_json::Value)->WireValue{WireValue{from:node(from),edge:edge.as_array().map(|row|(row[0].as_bool().unwrap(),node(row[1].as_u64().unwrap()))),edge_label:WireEdgeLabel{id:(label&1!=0).then(||"edge🧬".into()),kind:(label&2!=0).then(||"edgeKind".into())},properties:DslValue::from(&law["intrinsic"])}}
fn field(kind:&str,law:&serde_json::Value)->FieldValue{let text=law["text"].as_str().unwrap();match kind{
    "Bool"=>FieldValue::Bool(true),"Int"=>FieldValue::Int(-42),"UInt"=>FieldValue::UInt(42),"Float"=>FieldValue::Float(1.25),"Text"=>FieldValue::Text(text.into()),"Bytes64"=>FieldValue::Bytes64(vec![0,127,255]),"Enum"=>FieldValue::Enum(7),
    "Tuple"=>FieldValue::Tuple(vec![FieldValue::Text(text.into()),FieldValue::Absent]),"List"=>FieldValue::List(vec![FieldValue::Value(DslValue::from(&law["intrinsic"]))]),"Record"=>FieldValue::Record(record(text)),"Block"=>FieldValue::Block(Box::new(FieldValue::Text(text.into()))),"Statements"=>FieldValue::Statements(vec![("statement".into(),record(text))]),"Map"=>FieldValue::Map(vec![("ö".into(),FieldValue::Text(text.into()))]),"Value"=>FieldValue::Value(DslValue::from(&law["intrinsic"])),"Wire"=>FieldValue::Wire(wire(3,&serde_json::json!([true,3]),3,law)),"Expr"=>FieldValue::Expr(expression("Call",text)),"Absent"=>FieldValue::Absent,_=>panic!("undeclared FieldValue variant"),
}}
fn expression(kind:&str,text:&str)->ExprValue{match kind{"Num"=>ExprValue::Num(1.25),"Var"=>ExprValue::Var(text.into()),"Neg"=>ExprValue::Neg(Box::new(ExprValue::Var(text.into()))),"Binary"=>ExprValue::Binary(ExprOp::Add,Box::new(ExprValue::Var(text.into())),Box::new(ExprValue::Num(1.25))),"Call"=>ExprValue::Call(text.into(),vec![ExprValue::Var("ö".into()),ExprValue::Num(1.25)]),_=>panic!("undeclared ExprValue variant")}}
fn shape(kind:&str)->Shape{match kind{
    "Bool"=>Shape::Bool,"Int"=>Shape::Int,"UInt"=>Shape::UInt,"Float"=>Shape::Float,"Text"=>Shape::Text,"Bytes64"=>Shape::Bytes64,"Enum"=>Shape::Enum(vec![("enum🧬".into(),1)]),"Tuple"=>Shape::Tuple(Box::new(Shape::Text),Some(2)),"List"=>Shape::List(Box::new(Shape::Text)),"Record"=>Shape::Record(producer()),"Block"=>Shape::Block(Box::new(Shape::Text)),"Statements"=>Shape::Statements(vec![("statementö".into(),producer())]),"Map"=>Shape::Map(Box::new(Shape::Text)),"Value"=>Shape::Value,"Table"=>Shape::Table(producer()),"Wire"=>Shape::Wire,
    "Quantity"=>Shape::Quantity(semio_framework_dsl::unit_by_symbol("m").unwrap()),"Angle"=>Shape::Angle(semio_framework_dsl::unit_by_symbol("rad").unwrap()),"Ref"=>Shape::Ref("entity"),"Coord"=>Shape::Coord(3),"Dir"=>Shape::Dir,"Dim"=>Shape::Dim(3),"Range"=>Shape::Range,"Count"=>Shape::Count,"Expr"=>Shape::Expr,"Embed"=>Shape::Embed("language"),"EmbedFrom"=>Shape::EmbedFrom("language"),_=>panic!("undeclared Shape variant"),
}}
fn run<T:RetireOwned>(value:T,name:&str,law:&serde_json::Value){
    let (result,heap)=observe(||ControlledRetirement::new(value));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut owner=result.unwrap_or_else(|_|panic!("finite Record authority absent: {name}"));
    let mut born=0;let mut released=0;
    let demands=|owner:&ControlledRetirement<T>|{let copy=owner.next_copy_byte_demand();let release=owner.next_release_byte_demand().unwrap();(copy,owner.next_capacity_byte_demand(if copy==0{release}else{law["maximumCopyBytes"].as_u64().unwrap()as usize}).unwrap(),release)};
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){
        let ((copy,capacity,release),heap)=observe(||demands(&owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let grant=RetainedCloneGrant{maximum_items:law["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:law["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:law["maximumDepth"].as_u64().unwrap()as usize};assert!(copy<=grant.maximum_copy_bytes);
        let (zero,heap)=observe(||owner.step(RetainedCloneGrant::default()).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(demands(&owner),(copy,capacity,release));
        for denied in[(copy!=0).then_some(RetainedCloneGrant{maximum_copy_bytes:copy.saturating_sub(1),..grant}),(capacity!=0).then_some(RetainedCloneGrant{maximum_capacity_bytes:capacity.saturating_sub(1),..grant}),(release!=0).then_some(RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=observe(||owner.step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(demands(&owner),(copy,capacity,release));}
        let(step,heap)=observe(||owner.step(grant).unwrap());assert!(!heap.overflowed);assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes),"physical receipts: {name}");born+=heap.requested_bytes;released+=heap.released_bytes;if matches!(step,RetainedCloneStep::Complete(_)){break;}
    }
    assert!(owner.terminal_is_empty(),"finite Record exact grants must terminate: {name}");assert!(released>=born);let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] Record retirement {name} birth={born} release={released}; exact heap receipts and zero/below nonmovement");
}

#[test]
fn native_record_retirement_admits_finite_owned_family(){
    let law=corpus();let value=DslValue::from(&law["intrinsic"]);assert_eq!(serde_json::to_value(&value).unwrap(),law["intrinsic"]);run(value,"intrinsic serde reference",&law);
    for kind in law["fieldKinds"].as_array().unwrap(){run(field(kind.as_str().unwrap(),&law),kind.as_str().unwrap(),&law);}
    for kind in law["shapeKinds"].as_array().unwrap(){run(shape(kind.as_str().unwrap()),kind.as_str().unwrap(),&law);}
    for kind in law["expressionKinds"].as_array().unwrap(){run(expression(kind.as_str().unwrap(),law["text"].as_str().unwrap()),kind.as_str().unwrap(),&law);}
    for from in law["wireNodeOptions"].as_array().unwrap(){for edge in law["wireEdgeChoices"].as_array().unwrap(){for label in law["wireLabelOptions"].as_array().unwrap(){let value=wire(from.as_u64().unwrap(),edge,label.as_u64().unwrap(),&law);assert_eq!(serde_json::to_value(&value.properties).unwrap(),law["intrinsic"]);run(value,"Wire optional frontier",&law);}}}
    run(RecordFields::from_empty_slots(Vec::with_capacity(law["emptyFieldCapacity"].as_u64().unwrap()as usize)),"original empty reserved field buffer",&law);
    let mut nested=FieldValue::Text(law["text"].as_str().unwrap().into());for _ in 0..law["recursiveDepth"].as_u64().unwrap(){nested=FieldValue::Block(Box::new(nested));}run(nested,"recursive FieldValue",&law);
    let mut nested=Shape::Text;for _ in 0..law["recursiveDepth"].as_u64().unwrap(){nested=Shape::Block(Box::new(nested));}run(nested,"recursive Shape",&law);
    run(spec(),"declared RecordSpec",&law);
}

#[test]
fn native_record_writer_cancellation_transfers_original_typed_frontier(){
    let law=corpus();for prefix in law["writerPrefixes"].as_array().unwrap(){
        let source=RecordValue{fields:[(0,FieldValue::List(vec![FieldValue::Record(record(law["text"].as_str().unwrap())),FieldValue::Record(record("ö"))]))].into_iter().collect()};let schema=RecordSpec::new(Some("records"),RecordLayout::Inline,vec![FieldSpec::new(0,"rows",Shape::List(Box::new(Shape::Record(producer()))))]);
        let expected=semio_framework_dsl_record::print(&source,&schema,JoinMode::Inline);let mut writer=RetainedRecordWriter::new(source,schema,JoinMode::Inline,16777216);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(16777216,&mut accepted);let mut output=None;
        for _ in 0..prefix.as_u64().unwrap(){if let Some(value)=writer.step(1,&mut control).unwrap(){output=Some(value);break;}}
        if let Some(output)=output{assert_eq!(output,expected);}
        {use semio_framework_value::retirement::{RetirementCursor,RetirementStep};let birth=writer.next_birth_bytes(law["maximumCopyBytes"].as_u64().unwrap()as usize).unwrap();assert!(birth>0);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:law["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:birth,maximum_release_bytes:0,maximum_depth:law["maximumDepth"].as_u64().unwrap()as usize};
        let(zero,heap)=observe(||writer.close_step(RetainedCloneGrant::default()));assert!(matches!(zero,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let(denied,heap)=observe(||writer.close_step(RetainedCloneGrant{maximum_capacity_bytes:birth-1,..grant}));assert!(matches!(denied,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(writer.next_birth_bytes(grant.maximum_copy_bytes),Some(birth));
        let(denied,heap)=observe(||writer.close_step(RetainedCloneGrant{maximum_depth:0,..grant}));assert!(matches!(denied,RetirementStep::Failure(error)if error.kind==semio_framework_value::ValueRefusalKind::DepthLimit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(writer.next_birth_bytes(grant.maximum_copy_bytes),Some(birth));}
        run(writer,"writer original partial ownership",&law);
    }
}
