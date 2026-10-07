//! 🏭️ Literal schema ownership, lazy recursion and independent metadata identity.
use semio_framework_dsl_record::FieldSpec;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::NativeEncodeControl;
use semio_framework_dsl_record::NativeSchemaControl;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_dsl_record::Shape;
use std::sync::{OnceLock,atomic::{AtomicUsize,Ordering}};

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct GeneratedLeaf{value:f64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="metadata",layout="lines")]
struct GeneratedMetadata{empty:String,#[dsl(key="世界\0=@/!")]optional:Option<u64>,nested:GeneratedLeaf,#[dsl(base64)]octets:Vec<u8>}

static LABEL:OnceLock<String>=OnceLock::new();
static ORDINARY_CALLS:AtomicUsize=AtomicUsize::new(0);
static RECURSIVE_ORDINARY_CALLS:AtomicUsize=AtomicUsize::new(0);

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn label()->&'static str{LABEL.get_or_init(||{let data=fixture();data["copy"]["seed"].as_str().unwrap().repeat(data["copy"]["repeatCount"].as_u64().unwrap()as usize)})}
fn leaf()->RecordSpec{semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,vec![FieldSpec::new(0,"value",Shape::Float)])}
fn leaf_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;let mut fields=control.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(0,"value",semio_framework_dsl_record::Shape::Float,control)?);control.step()?;semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)})}
fn leaf_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:leaf,decoding:|control|leaf_controlled(control),encoding:|control|leaf_controlled(control)}}
fn ordinary()->RecordSpec{semio_framework_dsl_record::RecordSpec::new(Some("metadata"),semio_framework_dsl_record::RecordLayout::Lines,vec![FieldSpec::new(0,"empty",Shape::Text),FieldSpec::new(1,"世界\0=@/!",Shape::UInt).optional(),FieldSpec::new(2,"nested",Shape::Record(leaf_producer())),FieldSpec::new(3,"octets",Shape::Bytes64)])}
fn controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|{
        control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;
        for(id,key,shape,optional)in[(0,"empty",semio_framework_dsl_record::Shape::Text,false),(1,"世界\0=@/!",semio_framework_dsl_record::Shape::UInt,true),(2,"nested",semio_framework_dsl_record::Shape::Record(leaf_producer()),false),(3,"octets",semio_framework_dsl_record::Shape::Bytes64,false)]{
            let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;
        }
        semio_framework_dsl_record::producer::record(Some("metadata"),semio_framework_dsl_record::RecordLayout::Lines,fields,control)
    }))
}
fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary,decoding:|control|controlled(control),encoding:|control|controlled(control)}}
fn copying_ordinary()->RecordSpec{ORDINARY_CALLS.fetch_add(1,Ordering::SeqCst);semio_framework_dsl_record::RecordSpec::new(Some(label()),semio_framework_dsl_record::RecordLayout::Inline,Vec::new())}
fn copying_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{semio_framework_dsl_record::producer::record(Some(label()),semio_framework_dsl_record::RecordLayout::Inline,control.allocate_vec(0)?,control)}
fn copying_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:copying_ordinary,decoding:|control|copying_controlled(control),encoding:|control|copying_controlled(control)}}
fn frontier_ordinary()->RecordSpec{semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,(0..1024).map(|id|semio_framework_dsl_record::FieldSpec::new(id,"field",semio_framework_dsl_record::Shape::Text)).collect())}
fn frontier_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(1024)?;let mut fields=control.allocate_vec(1024)?;for id in 0..1024{fields.push(semio_framework_dsl_record::producer::field(id,"field",semio_framework_dsl_record::Shape::Text,control)?);control.step()?;}semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)})}
fn frontier_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:frontier_ordinary,decoding:|control|frontier_controlled(control),encoding:|control|frontier_controlled(control)}}
fn recursive_ordinary()->RecordSpec{RECURSIVE_ORDINARY_CALLS.fetch_add(1,Ordering::SeqCst);semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,vec![FieldSpec::new(0,"child",Shape::Block(Box::new(Shape::Record(recursive_producer())))).optional()])}
fn recursive_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,semio_framework_value::ValueError>{
    let mut fields=control.allocate_vec(1)?;control.charge(std::mem::size_of::<Shape>())?;fields.push(semio_framework_dsl_record::producer::field(0,"child",semio_framework_dsl_record::Shape::Block(Box::new(semio_framework_dsl_record::Shape::Record(recursive_producer()))),control)?.optional());semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
}
fn recursive_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:recursive_ordinary,decoding:|control|recursive_controlled(control),encoding:|control|recursive_controlled(control)}}

#[test]
fn sqlite_snapshot_native_schema_metadata_preserves_literal_fields_and_independent_identity(){
    use std::{io::Write,process::{Command,Stdio}};
    let data=fixture();
    let script="import{Database}from'bun:sqlite';const{x}=JSON.parse(await Bun.stdin.text());const d=new Database(':memory:');d.run('CREATE TABLE field(id INTEGER PRIMARY KEY,key TEXT NOT NULL,shape TEXT NOT NULL,optional INTEGER NOT NULL)');x.fields.forEach(r=>d.run('INSERT INTO field VALUES(?,?,?,?)',r.id,r.key,r.shape,r.optional?1:0));await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT id,key,shape,optional FROM field ORDER BY id').all().map(r=>({...r,optional:!!r.optional}))));d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"x":data}).to_string().as_bytes()).unwrap();let oracle=child.wait_with_output().unwrap();assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));assert_eq!(serde_json::from_slice::<serde_json::Value>(&oracle.stdout).unwrap(),data["fields"]);
    let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let mut accept=|_|true;let output=producer().encode(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut accept)).unwrap();let mut accept=|_|true;let input=producer().decode(&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut accept)).unwrap();
    let mut accept=|_|true;let generated_output=GeneratedMetadata::__dsl_spec_producer().encode(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut accept)).unwrap();let mut accept=|_|true;let generated_input=GeneratedMetadata::__dsl_spec_producer().decode(&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut accept)).unwrap();
    for spec in [&output,&input,&generated_output,&generated_input]{
        let rows:Vec<_>=spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":match field.shape{Shape::Text=>"text",Shape::UInt=>"uint",Shape::Record(_)=>"record",Shape::Bytes64=>"bytes",_=>panic!("shape")},"optional":field.optional})).collect();assert_eq!(serde_json::json!(rows),data["fields"]);
        let graph=dsl::os_pack::PackSchemaGraph::of(spec);let ordinary_graph=dsl::os_pack::PackSchemaGraph::of(&ordinary());assert_eq!(graph.canonical_bytes(),ordinary_graph.canonical_bytes());let expected=*blake3::hash(&ordinary_graph.canonical_bytes()).as_bytes();assert_eq!(dsl::os_pack::schema_hash(spec),expected);assert_eq!(dsl::os_pack::schema_hash_controlled(spec,&mut NativeEncodeControl::new(maximum,&mut |_|true)).unwrap(),expected);assert_eq!(dsl::os_pack::schema_hash_controlled(spec,&mut NativeDecodeControl::new(maximum,&mut |_|true)).unwrap(),expected);
    }
}

#[test]
fn sqlite_snapshot_native_schema_metadata_refuses_before_frontier_and_cancels_inside_literal_copy(){
    let data=fixture();let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let tiny=data["tinyMaximumBytes"].as_u64().unwrap()as usize;let source=label();ORDINARY_CALLS.store(0,Ordering::SeqCst);
    let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(tiny,&mut accept);assert_eq!(frontier_producer().encode(&mut control).unwrap_err().kind.as_str(),data["refusals"]["ownership"].as_str().unwrap());assert_eq!(control.owned_bytes(),0);
    let mut accept=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(tiny,&mut accept);assert_eq!(frontier_producer().decode(&mut control).unwrap_err().kind.as_str(),data["refusals"]["ownership"].as_str().unwrap());assert_eq!(control.owned_bytes(),0);
    let at=data["copy"]["cancelAt"].as_u64().unwrap()as usize;let mut reached=false;let mut cancel=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if event.total==source.len()&&event.completed>=at&&event.completed<event.total{reached=true;false}else{true}};assert_eq!(copying_producer().encode(&mut NativeEncodeControl::new(maximum,&mut cancel)).unwrap_err().kind.as_str(),data["refusals"]["cancellation"].as_str().unwrap());assert!(reached);assert_eq!(ORDINARY_CALLS.load(Ordering::SeqCst),0);
    let mut reached=false;let mut cancel=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==source.len()&&event.completed>=at&&event.completed<event.total{reached=true;false}else{true}};assert_eq!(copying_producer().decode(&mut NativeDecodeControl::new(maximum,&mut cancel)).unwrap_err().kind.as_str(),data["refusals"]["cancellation"].as_str().unwrap());assert!(reached);assert_eq!(ORDINARY_CALLS.load(Ordering::SeqCst),0);
}

#[test]
fn sqlite_snapshot_native_schema_metadata_known_frontier_reports_exact_work(){
    let data=fixture();let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let count=data["frontier"]["fieldCount"].as_u64().unwrap()as usize;let at=data["frontier"]["cancelAt"].as_u64().unwrap()as usize;let mut reached=false;
    let mut cancel=|event:semio_framework_value::native_encoding::NativeEncodeProgress|if event.total==count&&event.completed>=at&&event.completed<event.total{reached=true;false}else{true};assert!(frontier_producer().encode(&mut NativeEncodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
    let mut reached=false;let mut cancel=|event:semio_framework_value::native_decoding::NativeDecodeProgress|if event.total==count&&event.completed>=at&&event.completed<event.total{reached=true;false}else{true};assert!(frontier_producer().decode(&mut NativeDecodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
}

#[test]
fn sqlite_snapshot_native_schema_lazy_recursive_physical_text_never_materializes_ordinary_metadata(){
    let data=fixture();let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let depth=data["recursiveDepth"].as_u64().unwrap()as usize;RECURSIVE_ORDINARY_CALLS.store(0,Ordering::SeqCst);
    let mut value=semio_framework_dsl_record::RecordValue::default();value.fields.insert(0,semio_framework_dsl_record::FieldValue::Absent);for _ in 0..depth{let mut parent=semio_framework_dsl_record::RecordValue::default();parent.fields.insert(0,semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Record(value))));value=parent;}
    let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut accept);let spec=recursive_producer().encode(&mut control).unwrap();assert_eq!(RECURSIVE_ORDINARY_CALLS.load(Ordering::SeqCst),0);let text=semio_framework_dsl_record::print_controlled(&value,&spec,semio_framework_dsl_record::JoinMode::Inline,usize::MAX,&mut control).unwrap();assert_eq!(RECURSIVE_ORDINARY_CALLS.load(Ordering::SeqCst),0,"physical Text bypassed the declared controlled metadata producer");
    let mut accept=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(maximum,&mut accept);let spec=recursive_producer().decode(&mut control).unwrap();let reconstructed=semio_framework_dsl_record::parse_exact_controlled(&text,&spec,&semio_framework_dsl_record::ParseOptions::default(),&mut control).unwrap();assert_eq!(reconstructed,value);assert_eq!(RECURSIVE_ORDINARY_CALLS.load(Ordering::SeqCst),0,"physical parser bypassed the declared controlled metadata producer");
}

#[test]
fn sqlite_snapshot_native_schema_lazy_recursive_physical_pack_never_materializes_ordinary_metadata(){
    let data=fixture();let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let depth=data["recursiveDepth"].as_u64().unwrap()as usize;
    let mut value=semio_framework_dsl_record::RecordValue::default();value.fields.insert(0,semio_framework_dsl_record::FieldValue::Absent);for _ in 0..depth{let mut parent=semio_framework_dsl_record::RecordValue::default();parent.fields.insert(0,semio_framework_dsl_record::FieldValue::Block(Box::new(semio_framework_dsl_record::FieldValue::Record(value))));value=parent;}
    let bytes=dsl::os_pack::encode_document(&recursive_ordinary(),&value,&Default::default()).unwrap();RECURSIVE_ORDINARY_CALLS.store(0,Ordering::SeqCst);
    let mut accept=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(maximum,&mut accept);let spec=recursive_producer().decode(&mut control).unwrap();let(reconstructed,report)=dsl::os_pack::decode_document_controlled(&bytes,&spec,&Default::default(),&mut control).unwrap();assert!(!report.schema_drift);assert_eq!(reconstructed,value);assert_eq!(RECURSIVE_ORDINARY_CALLS.load(Ordering::SeqCst),0,"physical Pack hash bypassed the declared controlled metadata producer");
}

#[test]
fn sqlite_snapshot_native_schema_hash_controls_owned_frontier_and_incremental_literal_hashing(){
    let data=fixture();let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let at=data["frontier"]["cancelAt"].as_u64().unwrap()as usize;let count=data["frontier"]["fieldCount"].as_u64().unwrap()as usize;let spec=frontier_ordinary();
    let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut accept);assert!(dsl::os_pack::schema_hash_controlled(&spec,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
    let mut reached=false;let mut cancel=|event:semio_framework_value::native_encoding::NativeEncodeProgress|if event.total==count&&event.completed>=at&&event.completed<event.total{reached=true;false}else{true};assert!(dsl::os_pack::schema_hash_controlled(&spec,&mut NativeEncodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
    let graph=dsl::os_pack::PackSchemaGraph{records:vec![vec![dsl::os_pack::PackSchemaField{id:0,key:label().into(),optional:false,flatten:false,shape:dsl::os_pack::PackSchemaShape::Text}]]};let expected=*blake3::hash(&graph.canonical_bytes()).as_bytes();assert_eq!(graph.hash_controlled(&mut NativeEncodeControl::new(maximum,&mut |_|true)).unwrap(),expected);assert_eq!(graph.hash_controlled(&mut NativeDecodeControl::new(maximum,&mut |_|true)).unwrap(),expected);
    let mut reached=false;let mut cancel=|event:semio_framework_value::native_encoding::NativeEncodeProgress|if event.total==label().len()&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true};assert!(graph.hash_controlled(&mut NativeEncodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
    let mut reached=false;let mut cancel=|event:semio_framework_value::native_decoding::NativeDecodeProgress|if event.total==label().len()&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true};assert!(graph.hash_controlled(&mut NativeDecodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
}
