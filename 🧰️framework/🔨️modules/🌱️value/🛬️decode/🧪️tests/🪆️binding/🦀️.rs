use semio_framework_os_kernel::{native_decoding::*,DslField,FieldValue,RecordValue};

#[derive(Debug,PartialEq,dsl::DslRecord)]
struct BoundFields { title:String, chunks:Vec<Vec<u8>>, labels:Vec<String> }

#[test]
fn sqlite_snapshot_native_binding_controls_typed_field_materialization() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🪆️binding/🔣️.json")).unwrap();
    let title=fixture["title"].as_str().unwrap().to_string();
    let chunks:Vec<Vec<u8>>=serde_json::from_value(fixture["chunks"].clone()).unwrap();
    let labels:Vec<String>=serde_json::from_value(fixture["labels"].clone()).unwrap();
    let expected=BoundFields{title,chunks,labels};let record=expected.__dsl_to_record();
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(fixture["maximumBytes"].as_u64().unwrap() as usize,&mut accepted);
    let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>actual.title.len());
    let script="import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());import Ajv from 'ajv/dist/2020.js';if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const f=x.fixture;const d=new Database(':memory:');d.run('CREATE TABLE chunk(id INTEGER PRIMARY KEY,bytes BLOB NOT NULL)');f.chunks.forEach((b,i)=>d.run('INSERT INTO chunk VALUES(?,?)',i+1,new Uint8Array(b)));const rows=d.query('SELECT bytes FROM chunk ORDER BY id').all();await Bun.write(Bun.stdout,JSON.stringify(rows.map(r=>Array.from(r.bytes))));d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual.chunks,serde_json::from_slice::<Vec<Vec<u8>>>(&output.stdout).unwrap());
    let mut tiny=NativeDecodeControl::new(1,&mut accepted);assert!(BoundFields::__dsl_from_record_controlled(&record,&mut tiny).is_err());assert_eq!(tiny.owned_bytes(),0);
    let count=fixture["count"].as_u64().unwrap() as usize;let cancel_at=fixture["cancelAfter"].as_u64().unwrap() as usize;
    let many=BoundFields{title:String::new(),chunks:vec![],labels:(0..count).map(|_|"x".into()).collect()}.__dsl_to_record();
    let mut reached=0;let mut cancel=|event:NativeDecodeProgress|{reached=event.completed;event.completed<cancel_at};let mut control=NativeDecodeControl::new(1_000_000,&mut cancel);control.begin_stage(0).unwrap();assert!(BoundFields::__dsl_from_record_controlled(&many,&mut control).is_err());assert_eq!(reached,cancel_at);assert!(reached<count);
    struct Unbound;impl DslField for Unbound {fn shape()->dsl::Shape{dsl::Shape::Text}fn to_value(&self)->FieldValue{FieldValue::Text(String::new())}fn from_value(_:&FieldValue)->Result<Self,String>{panic!("ordinary custom constructor must not run")}}
    let mut control=NativeDecodeControl::new(1_000,&mut accepted);assert!(Unbound::from_value_controlled(&FieldValue::Text("owner".into()),&mut control).is_err());
    let empty=RecordValue::default();assert!(BoundFields::__dsl_from_record_controlled(&empty,&mut control).is_err());
}

#[derive(Debug,PartialEq,dsl::DslEnum)]
enum BoundVariant { Label { text:String }, Empty }
#[derive(Debug,PartialEq,dsl::DslScalar)]
enum BoundOrdinal { First, Second }

thread_local! { static FIELD_RETIREMENTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)}; }
struct RetainedField { retired:bool }
impl Drop for RetainedField { fn drop(&mut self){if !self.retired{FIELD_RETIREMENTS.with(|count|count.set(count.get()+1000));}} }
impl DslField for RetainedField {
    fn shape()->dsl::Shape{dsl::Shape::Text}
    fn to_value(&self)->FieldValue{FieldValue::Text("owned".into())}
    fn from_value(_:&FieldValue)->Result<Self,String>{Err("ordinary retained binding disabled".into())}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,String>{control.step()?;match value{FieldValue::Text(_)=>Ok(Self{retired:false}),_=>Err("expected retained Text".into())}}
    fn retire_decoded(mut self){self.retired=true;FIELD_RETIREMENTS.with(|count|count.set(count.get()+1));}
}
struct StagedField;
impl DslField for StagedField {
    fn shape()->dsl::Shape{dsl::Shape::Text}
    fn to_value(&self)->FieldValue{FieldValue::Text("stage".into())}
    fn from_value(_:&FieldValue)->Result<Self,String>{Err("ordinary staged binding disabled".into())}
    fn from_value_controlled(_:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,String>{control.begin_stage(2)?;control.charge(8)?;control.step()?;control.step()?;Ok(Self)}
}
#[derive(dsl::DslRecord)]
struct StagedRecord { child:StagedField, later:bool }

#[test]
fn sqlite_snapshot_native_binding_restores_declared_parent_workload() {
    let expected=StagedRecord{child:StagedField,later:true};let record=expected.__dsl_to_record();
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(4).unwrap();control.step().unwrap();let actual=StagedRecord::__dsl_from_record_controlled(&record,&mut control).unwrap();assert!(actual.later);control.step().unwrap();control.step().unwrap();assert!(control.step().is_err());assert_eq!(control.owned_bytes(),8);
    let mut incomplete=record.clone();incomplete.fields.remove(&StagedRecord::__dsl_spec().fields[1].id);let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(4).unwrap();control.step().unwrap();let error=StagedRecord::__dsl_from_record_controlled(&incomplete,&mut control).err().unwrap();assert!(error.message.contains("missing field"),"{}",error.message);control.advance(3).unwrap();assert_eq!(control.owned_bytes(),8);
}
#[derive(dsl::DslRecord)]
struct RetainedRecord { fields:Vec<RetainedField>, later:bool }

#[test]
fn sqlite_snapshot_native_binding_retires_partial_owned_fields() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🪆️binding/🔣️.json")).unwrap();
    let script="import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const f=x.fixture;const d=new Database(':memory:');d.run('CREATE TABLE work(id INTEGER PRIMARY KEY,completed INTEGER NOT NULL)');for(let i=1;i<=f.count;i++)d.run('INSERT INTO work VALUES(?,?)',i,i+f.retirement.initialWork);await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT COUNT(*) AS count FROM work WHERE completed <= ?').get(f.cancelAfter)));d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let independent:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(independent["count"],fixture["retirement"]["canceledFields"]);
    let count=fixture["retirement"]["completedFields"].as_u64().unwrap() as usize;
    let mut accepted=|_:NativeDecodeProgress|true;
    let spec=RetainedRecord::__dsl_spec();let fields_id=spec.fields[0].id;let later_id=spec.fields[1].id;
    let mut record=RecordValue::default();record.fields.insert(fields_id,FieldValue::List(vec![FieldValue::Text("owned".into());count]));
    FIELD_RETIREMENTS.with(|value|value.set(0));let mut control=NativeDecodeControl::new(65536,&mut accepted);assert!(RetainedRecord::__dsl_from_record_controlled(&record,&mut control).is_err());assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),count);
    record.fields.insert(later_id,FieldValue::Bool(true));FIELD_RETIREMENTS.with(|value|value.set(0));let mut control=NativeDecodeControl::new(65536,&mut accepted);let actual=RetainedRecord::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),0);DslField::retire_decoded(actual);assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),count);
    record.fields.insert(fields_id,FieldValue::List(vec![FieldValue::Text("owned".into()),FieldValue::Bool(false)]));FIELD_RETIREMENTS.with(|value|value.set(0));let mut control=NativeDecodeControl::new(65536,&mut accepted);assert!(RetainedRecord::__dsl_from_record_controlled(&record,&mut control).is_err());assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),1);
    record.fields.insert(fields_id,FieldValue::List(vec![FieldValue::Text("owned".into());fixture["count"].as_u64().unwrap() as usize]));FIELD_RETIREMENTS.with(|value|value.set(0));let boundary=fixture["cancelAfter"].as_u64().unwrap() as usize;let mut observed=None;let mut canceled=|event:NativeDecodeProgress|{if event.completed>=boundary{observed=Some(event);false}else{true}};let mut control=NativeDecodeControl::new(65536,&mut canceled);assert!(RetainedRecord::__dsl_from_record_controlled(&record,&mut control).is_err());drop(control);let observed=observed.unwrap();assert_eq!(observed.completed,boundary);assert_eq!(observed.total,fixture["count"].as_u64().unwrap() as usize);assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),observed.completed);assert_eq!(observed.completed,fixture["retirement"]["canceledFields"].as_u64().unwrap() as usize);
}

#[test]
fn sqlite_snapshot_native_binding_controls_declared_variants() {
    use dsl::DslVariants;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let mut progress=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(65536,&mut progress);
    for row in fixture["variants"].as_array().unwrap(){let expected=if row["keyword"]=="label"{BoundVariant::Label{text:row["text"].as_str().unwrap().into()}}else{BoundVariant::Empty};let (keyword,record)=expected.to_named_record();assert_eq!(BoundVariant::from_named_record_controlled(&keyword,&record,&mut control).unwrap(),expected);}
    assert_eq!(BoundOrdinal::from_value_controlled(&FieldValue::Enum(1),&mut control).unwrap(),BoundOrdinal::Second);
    assert!(BoundOrdinal::from_value_controlled(&FieldValue::Enum(2),&mut control).is_err());
    let mut canceled=|_:NativeDecodeProgress|false;let mut control=NativeDecodeControl::new(65536,&mut canceled);assert!(BoundVariant::from_named_record_controlled("empty",&RecordValue::default(),&mut control).is_err());assert!(BoundOrdinal::from_value_controlled(&FieldValue::Enum(0),&mut control).is_err());
}

#[test]
fn sqlite_snapshot_native_binding_controlled_pack_keeps_one_cumulative_budget() {
    use dsl::{PackDecodeOptions,PackEncodeOptions,PackLimits,pack_rt};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let expected=BoundFields{title:fixture["title"].as_str().unwrap().into(),chunks:serde_json::from_value(fixture["chunks"].clone()).unwrap(),labels:serde_json::from_value(fixture["labels"].clone()).unwrap()};
    let bytes=pack_rt::encode_record_body(&BoundFields::__dsl_spec(),&expected.__dsl_to_record(),&PackEncodeOptions::default()).unwrap();
    let mut progress=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(65536,&mut progress);
    let record=pack_rt::decode_record_body_exact_controlled(&bytes,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).unwrap();let parsed=control.owned_bytes();
    let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>parsed);
    let count=fixture["count"].as_u64().unwrap() as usize;let cutoff=fixture["cancelAfter"].as_u64().unwrap() as usize;let many=BoundFields{title:String::new(),chunks:vec![],labels:vec!["x".into();count]};let bytes=pack_rt::encode_record_body(&BoundFields::__dsl_spec(),&many.__dsl_to_record(),&PackEncodeOptions::default()).unwrap();
    let mut reached=0;let mut canceled=|event:NativeDecodeProgress|{reached=event.completed;event.completed<cutoff};let mut control=NativeDecodeControl::new(1_000_000,&mut canceled);assert!(pack_rt::decode_record_body_exact_controlled(&bytes,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).is_err());assert_eq!(reached,cutoff);assert!(reached<count);
    let mut control=NativeDecodeControl::new(16384,&mut progress);let options=PackDecodeOptions{limits:PackLimits{max_total_alloc:16384,..Default::default()},..Default::default()};assert!(pack_rt::decode_record_body_exact_controlled(&bytes,&BoundFields::__dsl_spec(),&options,&mut control).is_err());assert!(control.owned_bytes()<=16384);
}

#[test]
fn sqlite_snapshot_native_binding_controlled_document_cancels_during_inflation() {
    use dsl::{PackDecodeOptions,PackEncodeOptions,pack_rt};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let expected=BoundFields{title:fixture["title"].as_str().unwrap().repeat(fixture["count"].as_u64().unwrap() as usize*16),chunks:serde_json::from_value(fixture["chunks"].clone()).unwrap(),labels:serde_json::from_value(fixture["labels"].clone()).unwrap()};
    let encoded=pack_rt::encode_document(&BoundFields::__dsl_spec(),&expected.__dsl_to_record(),&PackEncodeOptions::default()).unwrap();
    let script="import{inflateRawSync}from'node:zlib';const x=JSON.parse(await Bun.stdin.text());const b=Buffer.from(x.bytes);let p=32;function v(){let n=0,s=0;for(;;){const x=b[p++];n+=(x&127)*2**s;if(!(x&128))return n;s+=7;if(s>63)throw Error('varint')}}let found=false;while(p<b.length-84){const kind=b[p++],flags=b[p++],stored=v(),raw=flags&1?v():stored;const payload=b.subarray(p,p+stored);p+=stored+4;const decoded=flags&1?inflateRawSync(payload):payload;if(decoded.length!==raw)throw Error('length');if(decoded.includes(Buffer.from(x.title)))found=true;}await Bun.write(Bun.stdout,JSON.stringify(found));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":encoded,"title":expected.title}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(serde_json::from_slice::<bool>(&output.stdout).unwrap(),true);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(16_000_000,&mut accepted);let (record,_)=pack_rt::decode_document_controlled(&encoded,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).unwrap();let parsed=control.owned_bytes();let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>parsed);
    let mut interior=false;let mut cancel=|event:NativeDecodeProgress|{if event.completed>=256&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(16_000_000,&mut cancel);assert!(pack_rt::decode_document_controlled(&encoded,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).is_err());assert!(interior);
    let mut control=NativeDecodeControl::new(64,&mut accepted);assert!(pack_rt::decode_document_controlled(&encoded,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).is_err());assert!(control.owned_bytes()<=64);
}

#[test]
fn sqlite_snapshot_native_binding_controls_artifact_child_identity() {
    use semio_framework_os_kernel::{ArtifactChild,io_schema::ArtifactRef};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔗️child/🔣️.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🔗️child/🔣️.json")).unwrap();
    let strings=["childId","artifactId","artifactKind","standard","subset"].map(|key|fixture[key].as_str().unwrap());let total=strings.iter().map(|text|text.len()).sum::<usize>();let expected=ArtifactRef{artifact_id:strings[1].into(),dialect:semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:strings[2].into(),standard:strings[3].into(),subset:strings[4].into()}};
    let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(strings[0].into()));record.fields.insert(1,<ArtifactRef as dsl::DslField>::to_value(&expected));let value=FieldValue::Record(record);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(total,&mut accepted);let actual=ArtifactChild::<()>::from_value_controlled(&value,&mut control).unwrap();assert_eq!(actual.child_id,strings[0]);assert_eq!(actual.target,expected);assert_eq!(control.owned_bytes(),total);
    let mut control=NativeDecodeControl::new(total-1,&mut accepted);assert!(ArtifactChild::<()>::from_value_controlled(&value,&mut control).is_err());assert!(control.owned_bytes()<=total-1);
    let repeated=strings[1].repeat(fixture["repeatCount"].as_u64().unwrap() as usize);let long_target=ArtifactRef{artifact_id:repeated,dialect:expected.dialect.clone()};let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(strings[0].into()));record.fields.insert(1,<ArtifactRef as dsl::DslField>::to_value(&long_target));let mut interior=false;let cutoff=fixture["cancelAt"].as_u64().unwrap() as usize;let mut cancel=|event:NativeDecodeProgress|{if event.completed>=cutoff&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(1_000_000,&mut cancel);assert!(ArtifactChild::<()>::from_value_controlled(&FieldValue::Record(record),&mut control).is_err());assert!(interior);
    let script="import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const d=new Database(':memory:');d.run('CREATE TABLE identity(child TEXT,id TEXT,kind TEXT,standard TEXT,subset TEXT)');const f=x.fixture;d.run('INSERT INTO identity VALUES(?,?,?,?,?)',f.childId,f.artifactId,f.artifactKind,f.standard,f.subset);await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT child,id,kind,standard,subset FROM identity').get()));d.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture,"schema":schema}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let row:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(row,serde_json::json!({"child":actual.child_id,"id":actual.target.artifact_id,"kind":actual.target.dialect.artifact_kind,"standard":actual.target.dialect.standard,"subset":actual.target.dialect.subset}));
}

#[test]
fn sqlite_snapshot_native_binding_controlled_text_preserves_owned_literals() {
    use dsl::schema::{parse_exact_controlled,ParseOptions,JoinMode};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let expected=BoundFields{title:fixture["title"].as_str().unwrap().repeat(16384),chunks:serde_json::from_value(fixture["chunks"].clone()).unwrap(),labels:serde_json::from_value(fixture["labels"].clone()).unwrap()};let spec=BoundFields::__dsl_spec();let text=dsl::schema::print(&expected.__dsl_to_record(),&spec,JoinMode::Document);
    let script=r#"const x=JSON.parse(await Bun.stdin.text());const m=x.text.match(/title=("(?:[^"\\]|\\.)*")/);if(!m)throw Error('title');const literal=m[1].replace(/\\u\{([0-9a-fA-F]+)\}/g,(_,n)=>{const v=parseInt(n,16);return v<65536?'\\u'+v.toString(16).padStart(4,'0'):String.fromCodePoint(v)});await Bun.write(Bun.stdout,JSON.stringify(JSON.parse(literal)===x.title));"#;
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"text":text,"title":expected.title}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(serde_json::from_slice::<bool>(&output.stdout).unwrap(),true);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(16_000_000,&mut accepted);let record=parse_exact_controlled(&text,&spec,&ParseOptions::default(),&mut control).unwrap();let parsed=control.owned_bytes();let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>parsed);
    let mut interior=false;let mut canceled=|event:NativeDecodeProgress|{if event.completed>=256&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(16_000_000,&mut canceled);assert!(parse_exact_controlled(&text,&spec,&ParseOptions::default(),&mut control).is_err());assert!(interior);
    let mut control=NativeDecodeControl::new(64,&mut accepted);assert!(parse_exact_controlled(&text,&spec,&ParseOptions::default(),&mut control).is_err());assert!(control.owned_bytes()<=64);
    let mut control=NativeDecodeControl::new(16_000_000,&mut accepted);assert!(parse_exact_controlled(&(text+" foreign=1"),&spec,&ParseOptions::default(),&mut control).is_err());
}

#[derive(Debug,PartialEq,dsl::DslRecord)]
struct ChunkField { #[dsl(base64)] bytes:Vec<u8> }
#[derive(Debug,PartialEq,dsl::DslRecord)]
struct ChunkFields { chunks:Vec<ChunkField> }

#[test]
fn sqlite_snapshot_native_binding_controls_chunk_reconstruction(){
    use dsl::{PackDecodeOptions,PackEncodeOptions,pack_rt};use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();let expected=ChunkFields{chunks:serde_json::from_value::<Vec<Vec<u8>>>(fixture["chunks"].clone()).unwrap().into_iter().map(|bytes|ChunkField{bytes}).collect()};let mut options=PackEncodeOptions::default();options.chunk_threshold=1;options.chunk_size=2;let spec=ChunkFields::__dsl_spec();let bytes=pack_rt::encode_document(&spec,&expected.__dsl_to_record(),&options).unwrap();let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(1_000_000,&mut accepted);let mut verified=PackDecodeOptions::default();verified.verification=semio_framework_os_kernel::VerificationLevel::Full;let(record,_)=pack_rt::decode_document_controlled(&bytes,&spec,&verified,&mut control).unwrap();assert_eq!(ChunkFields::__dsl_from_record_controlled(&record,&mut control).unwrap(),expected);
    let repeated=expected.chunks[0].bytes.repeat(fixture["count"].as_u64().unwrap() as usize*64);let length=repeated.len();let large=ChunkFields{chunks:vec![ChunkField{bytes:repeated}]};let mut identity=options.clone();identity.codec=semio_framework_os_kernel::CodecId(0);identity.chunk_size=length as u64;let large_bytes=pack_rt::encode_document(&spec,&large.__dsl_to_record(),&identity).unwrap();
    for verification in [semio_framework_os_kernel::VerificationLevel::Trusted,semio_framework_os_kernel::VerificationLevel::Standard]{let mut interior=false;let mut canceled=|event:NativeDecodeProgress|{if event.total==length&&event.completed>=4096&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(1_000_000,&mut canceled);let mut decode=PackDecodeOptions::default();decode.verification=verification;assert!(pack_rt::decode_document_controlled(&large_bytes,&spec,&decode,&mut control).is_err());assert!(interior);}
    let mut large_work=false;let mut observed=|event:NativeDecodeProgress|{large_work|=event.total==length;true};let mut control=NativeDecodeControl::new(64_000,&mut observed);assert!(pack_rt::decode_document_controlled(&large_bytes,&spec,&verified,&mut control).is_err());assert!(control.owned_bytes()<=64_000);drop(control);assert!(!large_work);
    let file=semio_framework_os_kernel::os_io::resolve_ready(semio_framework_os_kernel::os_pack::format::PackFile::open_manifest(bytes.as_slice(),&verified.limits,verified.verification)).unwrap();let range=file.chunk_range(semio_framework_os_kernel::ChunkId(0)).unwrap();let mut corrupted=bytes.clone();corrupted[range.offset as usize]^=1;let mut control=NativeDecodeControl::new(1_000_000,&mut accepted);assert!(pack_rt::decode_document_controlled(&corrupted,&spec,&verified,&mut control).is_err());
    let script="import{inflateRawSync}from'node:zlib';const x=JSON.parse(await Bun.stdin.text()),b=Buffer.from(x.bytes);let p=32;function v(){let n=0,s=0;for(;;){const x=b[p++];n+=(x&127)*2**s;if(!(x&128))return n;s+=7;if(s>63)throw Error('varint')}}const chunks=[];while(p<b.length-84){const kind=b[p++],flags=b[p++],stored=v(),raw=flags&1?v():stored,payload=b.subarray(p,p+stored);p+=stored+4;const decoded=flags&1?inflateRawSync(payload):payload;if(decoded.length!==raw)throw Error('length');if(kind===5)chunks.push(Array.from(decoded));}await Bun.write(Bun.stdout,JSON.stringify(chunks));";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":bytes}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let chunks:Vec<Vec<u8>>=serde_json::from_slice(&output.stdout).unwrap();let expected_chunks:Vec<Vec<u8>>=expected.chunks.iter().flat_map(|chunk|chunk.bytes.chunks(2).map(|bytes|bytes.to_vec())).collect();assert_eq!(chunks,expected_chunks);
}
