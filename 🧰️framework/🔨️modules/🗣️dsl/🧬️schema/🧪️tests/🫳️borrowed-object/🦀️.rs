use super::*;
use std::alloc::{GlobalAlloc, Layout, System};

struct ObservedDslAllocator;
std::thread_local! {
    static ALLOCATION_ENABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ALLOCATED_BYTES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static REQUESTS: std::cell::Cell<RequestObservation> = const { std::cell::Cell::new(RequestObservation::EMPTY) };
}
#[derive(Clone,Copy)]
struct RequestObservation { bytes:usize, layouts:[(usize,usize);32], length:usize, overflow:bool }
impl RequestObservation {
 const EMPTY:Self=Self{bytes:0,layouts:[(0,0);32],length:0,overflow:false};
 fn push(&mut self,bytes:usize,alignment:usize){match self.bytes.checked_add(bytes){Some(total)=>self.bytes=total,None=>self.overflow=true}if self.length<self.layouts.len(){self.layouts[self.length]=(bytes,alignment);self.length+=1;}else{self.overflow=true;}}
}
fn record_allocation(bytes: usize,alignment:usize) {
    if ALLOCATION_ENABLED.try_with(|enabled| enabled.get()).unwrap_or(false) { let _ = ALLOCATED_BYTES.try_with(|count| count.set(count.get().saturating_add(bytes)));let _=REQUESTS.try_with(|requests|{let mut value=requests.get();value.push(bytes,alignment);requests.set(value);}); }
}
unsafe impl GlobalAlloc for ObservedDslAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { record_allocation(layout.size(),layout.align()); unsafe { System.alloc(layout) } }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { record_allocation(layout.size(),layout.align()); unsafe { System.alloc_zeroed(layout) } }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 { record_allocation(size,layout.align()); unsafe { System.realloc(pointer, layout, size) } }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) { record_rejected_payload_deallocation(pointer); unsafe { System.dealloc(pointer, layout) } }
}
#[global_allocator]
static DSL_TEST_ALLOCATOR: ObservedDslAllocator = ObservedDslAllocator;

struct RestoreObservation { enabled:bool, bytes:usize, requests:RequestObservation }
impl Drop for RestoreObservation {
 fn drop(&mut self){let measured=REQUESTS.with(std::cell::Cell::get);let mut parent=self.requests;if self.enabled{for(bytes,alignment)in measured.layouts[..measured.length].iter().copied(){parent.push(bytes,alignment);}parent.overflow|=measured.overflow;}REQUESTS.with(|requests|requests.set(parent));ALLOCATED_BYTES.with(|bytes|bytes.set(if self.enabled{self.bytes.saturating_add(measured.bytes)}else{self.bytes}));ALLOCATION_ENABLED.with(|enabled|enabled.set(self.enabled));}
}
fn observe_requests<T>(operation:impl FnOnce()->T)->(T,RequestObservation){let restore=RestoreObservation{enabled:ALLOCATION_ENABLED.with(std::cell::Cell::get),bytes:ALLOCATED_BYTES.with(std::cell::Cell::get),requests:REQUESTS.with(std::cell::Cell::get)};REQUESTS.with(|requests|requests.set(RequestObservation::EMPTY));ALLOCATED_BYTES.with(|bytes|bytes.set(0));ALLOCATION_ENABLED.with(|enabled|enabled.set(true));let output=operation();let requests=REQUESTS.with(std::cell::Cell::get);drop(restore);assert!(!requests.overflow,"allocator request observation overflow");(output,requests)}

#[test]
fn sqlite_snapshot_native_encoding_record_backing_matches_actual_allocator_requests(){
 use crate::native_encoding::EncodedRecord;
 use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
 fn construct(count:usize,start:u16,integer:i64,control:&mut NativeEncodeControl<'_>)->Result<EncodedRecord,semio_framework_value::ValueError>{let mut record=EncodedRecord::new(count,control)?;for index in 0..count{record.insert(start+index as u16,FieldValue::Int(integer+index as i64))?;}Ok(record)}
 fn no_backing_request(refused:&RequestObservation,baseline:&RequestObservation){for layout in &refused.layouts[..refused.length]{assert!(!baseline.layouts[..baseline.length].contains(layout),"refused constructor requested an observed backing layout {layout:?}");}}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/💰️record-backing/🔣️.json")).unwrap();let start=fixture["primitive"]["fieldIdStart"].as_u64().unwrap()as u16;let integer=fixture["primitive"]["integerStart"].as_i64().unwrap();
 for count in fixture["cardinalities"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;let mut accepted=|_|true;let mut control=NativeEncodeControl::new(usize::MAX,&mut accepted);
  let(result,baseline)=observe_requests(||construct(count,start,integer,&mut control));let record=result.unwrap();let paid=control.owned_bytes();assert_eq!(record.as_record().fields.len(),count);for index in 0..count{assert!(matches!(record.as_record().get(start+index as u16),Some(FieldValue::Int(value))if *value==integer+index as i64));}drop(record);assert!(paid>=baseline.bytes,"count {count}: admitted {paid} bytes before actual full backing requests {}",baseline.bytes);if count==0{assert_eq!(baseline.bytes,0);assert_eq!(paid,0);}else{assert!(baseline.bytes>0);}
  let mut accepted=|_|true;let mut exact=NativeEncodeControl::new(baseline.bytes,&mut accepted);let(result,requests)=observe_requests(||construct(count,start,integer,&mut exact));let record=result.unwrap_or_else(|error|panic!("count {count}: measured exact backing allowance {} refused: {error:?}",baseline.bytes));assert_eq!(requests.bytes,baseline.bytes);assert_eq!(exact.owned_bytes(),baseline.bytes);drop(record);
  if count==0{continue;}
  for maximum in [baseline.bytes-1,0]{let mut accepted=|_|true;let mut denied=NativeEncodeControl::new(maximum,&mut accepted);let(result,requests)=observe_requests(||construct(count,start,integer,&mut denied));let error=match result{Err(error)=>error,Ok(record)=>{drop(record);panic!("count {count}: refused allowance {maximum} created backing")}};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(denied.owned_bytes(),0);no_backing_request(&requests,&baseline);}
  let mut accepted=|_|true;let mut cumulative=NativeEncodeControl::new(baseline.bytes.checked_mul(2).unwrap(),&mut accepted);for turn in 1..=2{let(result,requests)=observe_requests(||construct(count,start,integer,&mut cumulative));let record=result.unwrap();assert_eq!(requests.bytes,baseline.bytes);drop(record);assert_eq!(cumulative.owned_bytes(),baseline.bytes*turn);}let(result,requests)=observe_requests(||construct(count,start,integer,&mut cumulative));let error=match result{Err(error)=>error,Ok(record)=>{drop(record);panic!("retirement refunded cumulative backing")}};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(cumulative.owned_bytes(),baseline.bytes*2);no_backing_request(&requests,&baseline);
 }
}

#[test]
fn sqlite_snapshot_native_encoding_borrowed_object_occurrences_keep_payload_allocations_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let bytes = fixture["payloadBytes"].as_u64().unwrap() as usize;
    let depth = fixture["depth"].as_u64().unwrap() as usize;
    let wide = fixture["unsigned"].as_str().unwrap().parse::<u64>().unwrap();
    let bits = u64::from_str_radix(fixture["binary64Bits"].as_str().unwrap(), 16).unwrap();
    let mut value = DslValue::Object(vec![("zPayload".into(), DslValue::String("x".repeat(bytes))), ("wide".into(), DslValue::uint(wide)), ("exceptional".into(), DslValue::Number(Number::Float(f64::from_bits(bits))))]);
    for _ in 0..depth { value = DslValue::Object(vec![("z".into(), DslValue::Bool(true)), ("a".into(), value)]); }
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "value", Shape::Value)]);
    let mut record = RecordValue::default(); record.fields.insert(1, FieldValue::Value(value));
    ALLOCATED_BYTES.with(|count| count.set(0)); ALLOCATION_ENABLED.with(|enabled| enabled.set(true));
    let text = print(&record, &spec, JoinMode::Document);
    ALLOCATION_ENABLED.with(|enabled| enabled.set(false));
    let allocated = ALLOCATED_BYTES.with(|count| count.get());
    assert!(allocated <= bytes * fixture["allocationMultiplier"].as_u64().unwrap() as usize, "printing allocated {allocated} bytes for a {bytes}-byte payload at depth {depth}");
    assert!(text.find("z=").unwrap() < text.find("a=").unwrap());
    let restored = parse(&text, &spec, &ParseOptions { limits: Limits { max_bytes: bytes * 2, ..Limits::default() }, mode: SourceMode::Document }).unwrap();
    let FieldValue::Value(node) = restored.get(1).unwrap() else { panic!("owned value"); }; let mut node = node;
    for _ in 0..depth { let DslValue::Object(entries)=node else {panic!("ordered wrapper")};assert_eq!(entries.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>(),fixture["wrapperKeys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>());assert!(matches!(entries[0].1,DslValue::Bool(true)));node=&entries[1].1; }
    let DslValue::Object(entries)=node else {panic!("ordered payload")};
    assert_eq!(entries.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>(),fixture["payloadKeys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(node.get("wide").unwrap().as_u64(), Some(wide));
    assert_eq!(node.get("exceptional").unwrap().as_f64().unwrap().to_bits(), bits);
    assert_eq!(node.get("zPayload").unwrap().as_str().unwrap().len(), bytes);
    let output = std::process::Command::new("bun").args(["-e", "import {Database} from 'bun:sqlite';const x=JSON.parse(process.argv[1]);const db=new Database(':memory:');try{db.run('CREATE TABLE keys(ordinal INTEGER PRIMARY KEY,value TEXT NOT NULL)');for(const [ordinal,key]of x.fixture.payloadKeys.entries())db.run('INSERT INTO keys VALUES(?,?)',[ordinal,key]);await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT value FROM keys ORDER BY ordinal').all().map(row=>row.value)));}finally{db.close();}",&serde_json::json!({"fixture":fixture}).to_string()]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let ordered: Vec<String> = serde_json::from_slice(&output.stdout).unwrap();
    let positions = ordered.iter().map(|key| text.find(&format!("{key}=")).unwrap()).collect::<Vec<_>>();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(parse("value={ a=", &spec, &ParseOptions::default()).is_err());
}


#[derive(Clone,Copy,Default)]
struct RejectedPayloadReclamation{leaves:usize,blocks:usize}
#[derive(Clone,Copy)]
struct RejectedPayloadObservation{leaf:usize,leaf_seen:bool,blocks:*mut(usize,bool),length:usize,reclaimed:RejectedPayloadReclamation}
std::thread_local!{static REJECTED_PAYLOAD_DEALLOCATION:std::cell::Cell<Option<RejectedPayloadObservation>>=const{std::cell::Cell::new(None)};}
struct RestoreRejectedPayloadObservation(Option<RejectedPayloadObservation>);
impl Drop for RestoreRejectedPayloadObservation{fn drop(&mut self){REJECTED_PAYLOAD_DEALLOCATION.with(|cell|cell.set(self.0));}}
fn observe_rejected_payload_deallocation<T>(leaf:usize,blocks:&mut[(usize,bool)],operation:impl FnOnce()->T)->(T,RejectedPayloadReclamation){
 let observation=RejectedPayloadObservation{leaf,leaf_seen:false,blocks:blocks.as_mut_ptr(),length:blocks.len(),reclaimed:RejectedPayloadReclamation::default()};let restore=RestoreRejectedPayloadObservation(REJECTED_PAYLOAD_DEALLOCATION.with(|cell|cell.replace(Some(observation))));let result=operation();let reclaimed=REJECTED_PAYLOAD_DEALLOCATION.with(std::cell::Cell::get).unwrap().reclaimed;drop(restore);(result,reclaimed)
}
fn record_rejected_payload_deallocation(pointer:*mut u8){
 let _=REJECTED_PAYLOAD_DEALLOCATION.try_with(|cell|{if let Some(mut observation)=cell.get(){let address=pointer as usize;if address==observation.leaf{if !observation.leaf_seen{observation.leaf_seen=true;observation.reclaimed.leaves+=1;}}let blocks=unsafe{std::slice::from_raw_parts_mut(observation.blocks,observation.length)};if let Some((_,seen))=blocks.iter_mut().find(|(block,_)|*block==address){if !*seen{*seen=true;observation.reclaimed.blocks+=1;}}cell.set(Some(observation));}});
}
#[test]
fn sqlite_snapshot_native_encoding_record_denied_insert_reclaims_deep_owned_payload(){
 use crate::native_encoding::EncodedRecord;
 use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/💰️record-backing/🛑️rejected-insert/🔣️.json")).unwrap();
 let stack=usize::try_from(fixture["stackBytes"].as_u64().unwrap()).unwrap();
 std::thread::Builder::new().stack_size(stack).spawn(move||{
  let slots=usize::try_from(fixture["slots"].as_u64().unwrap()).unwrap();let id=u16::try_from(fixture["existing"]["id"].as_u64().unwrap()).unwrap();let denied_id=u16::try_from(fixture["rejected"]["id"].as_u64().unwrap()).unwrap();let depth=usize::try_from(fixture["rejected"]["depth"].as_u64().unwrap()).unwrap();let payload_bytes=usize::try_from(fixture["rejected"]["payloadBytes"].as_u64().unwrap()).unwrap();let byte=u8::try_from(fixture["rejected"]["byte"].as_u64().unwrap()).unwrap();
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(usize::MAX,&mut accept);let(result,backing)=observe_requests(||EncodedRecord::new(slots,&mut control));let mut record=result.unwrap();let paid=control.owned_bytes();assert_eq!(paid,backing.bytes);record.insert(id,FieldValue::Int(fixture["existing"]["integer"].as_i64().unwrap())).unwrap();let capacity=record.as_record().fields.capacity();let position=record.as_record().get(id).unwrap()as *const FieldValue;
  let payload=||{let bytes=vec![byte;payload_bytes];let leaf=bytes.as_ptr()as usize;let mut blocks=Vec::with_capacity(depth);let mut value=FieldValue::Bytes64(bytes);for _ in 0..depth{let block=Box::new(value);blocks.push((block.as_ref()as *const FieldValue as usize,false));value=FieldValue::Block(block);} (value,leaf,blocks)};
  let(incoming,replaced_leaf,mut replaced_blocks)=payload();let(result,requests)=observe_requests(||record.insert(id,incoming));result.unwrap();for layout in &requests.layouts[..requests.length]{assert!(!backing.layouts[..backing.length].contains(layout),"same-ID insertion requested record backing {layout:?}");}assert_eq!(record.as_record().fields.capacity(),capacity);assert_eq!(record.as_record().get(id).unwrap()as *const FieldValue,position);assert_eq!(control.owned_bytes(),paid);
  let((result,requests),reclaimed)=observe_rejected_payload_deallocation(replaced_leaf,&mut replaced_blocks,||observe_requests(||record.insert(id,FieldValue::Int(fixture["replacement"]["integer"].as_i64().unwrap()))));result.unwrap();assert_eq!(reclaimed.leaves,fixture["expected"]["reclaimedLeaves"].as_u64().unwrap()as usize);assert_eq!(reclaimed.blocks,fixture["expected"]["reclaimedBlocks"].as_u64().unwrap()as usize);for layout in &requests.layouts[..requests.length]{assert!(!backing.layouts[..backing.length].contains(layout),"replacement requested record backing {layout:?}");}
  let(incoming,denied_leaf,mut denied_blocks)=payload();let((result,requests),reclaimed)=observe_rejected_payload_deallocation(denied_leaf,&mut denied_blocks,||observe_requests(||record.insert(denied_id,incoming)));let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(reclaimed.leaves,fixture["expected"]["reclaimedLeaves"].as_u64().unwrap()as usize);assert_eq!(reclaimed.blocks,fixture["expected"]["reclaimedBlocks"].as_u64().unwrap()as usize);
  assert_eq!(record.as_record().fields.len(),fixture["expected"]["length"].as_u64().unwrap()as usize);assert_eq!(record.as_record().fields.capacity(),capacity);assert_eq!(record.as_record().get(id).unwrap()as *const FieldValue,position);assert!(matches!(record.as_record().get(id),Some(FieldValue::Int(value))if *value==fixture["expected"]["integer"].as_i64().unwrap()));assert!(record.as_record().get(denied_id).is_none());assert_eq!(control.owned_bytes(),paid);for layout in &requests.layouts[..requests.length]{assert!(!backing.layouts[..backing.length].contains(layout),"rejected insert requested record backing {layout:?}");}drop(record);assert_eq!(control.owned_bytes(),paid);
 }).unwrap().join().unwrap();
}

#[derive(semio_framework_dsl_record_derive::DslEnum)]
enum TaggedCursorVariant { Document{text:String}, Blob{text:String} }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct TaggedCursorRecord {
 #[dsl(statements)] many:Vec<TaggedCursorVariant>,
 #[dsl(statements,block)] nested:Vec<TaggedCursorVariant>,
 #[dsl(statements,block)] maybe:Option<TaggedCursorVariant>,
 #[dsl(statements,block)] required:Box<TaggedCursorVariant>,
}
fn tagged_cursor_variant_spec(keyword:&'static str)->crate::BorrowedRecordSpec{use crate::{BorrowedFieldSpec as F,BorrowedShape as H};const FIELDS:&[F]=&[F::new(0,"text",H::Text)];crate::BorrowedRecordSpec{keyword:Some(keyword),layout:RecordLayout::Inline,fields:FIELDS}}
fn tagged_cursor_document()->crate::BorrowedRecordSpec{tagged_cursor_variant_spec("document")}
fn tagged_cursor_blob()->crate::BorrowedRecordSpec{tagged_cursor_variant_spec("blob")}
fn tagged_cursor_statements()->crate::BorrowedShape{crate::BorrowedShape::Statements(&[("document",tagged_cursor_document),("blob",tagged_cursor_blob)])}
#[test]
fn sqlite_snapshot_native_borrowed_tagged_cursor_uses_actual_four_family_views_and_canonical_text(){
 use crate::{DslField,native_encoding::FieldProjectionView as V,BorrowedFieldSpec as F,BorrowedShape as H};use semio_framework_value::NativeEncodeControl;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🏷️tagged/🔣️.json")).unwrap();let text=fixture["text"].as_str().unwrap();
 let variant=|keyword:&str|match keyword{"document"=>TaggedCursorVariant::Document{text:text.into()},"blob"=>TaggedCursorVariant::Blob{text:text.into()},_=>panic!("closed tagged variant")};
 let source=TaggedCursorRecord{many:vec![variant("document"),variant("blob")],nested:vec![variant("blob"),variant("document")],maybe:Some(variant("document")),required:Box::new(variant("blob"))};
 let output=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const f=JSON.parse(process.argv[1]);if(Buffer.byteLength(f.text,"utf8")!==f.textBytes)throw Error("closed tagged corpus");const db=new Database(":memory:");try{db.run("CREATE TABLE projection(slot INTEGER,ordinal INTEGER,tag TEXT,value TEXT)");for(const[slot,item]of f.slots.entries())for(const[ordinal,tag]of item.variants.entries())db.run("INSERT INTO projection VALUES(?,?,?,?)",[slot,ordinal,tag,f.text]);console.log(JSON.stringify(db.query("SELECT slot,ordinal,tag,value,length(CAST(value AS BLOB)) AS bytes FROM projection ORDER BY slot,ordinal").all()));}finally{db.close();}"#]).arg(fixture.to_string()).output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let rows:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();
 for row in rows.as_array().unwrap(){let slot=row["slot"].as_u64().unwrap()as usize;let ordinal=row["ordinal"].as_u64().unwrap()as usize;let block=fixture["slots"][slot]["block"].as_bool().unwrap();let mut path=vec![slot];if block{assert!(matches!(DslField::projection_view(&source,&path).unwrap(),V::Block));path.push(0);}assert!(matches!(DslField::projection_view(&source,&path).unwrap(),V::Statements(n)if n==fixture["slots"][slot]["variants"].as_array().unwrap().len()));assert_eq!(DslField::projection_key(&source,&path,ordinal).unwrap(),row["tag"].as_str().unwrap());path.push(ordinal);assert!(matches!(DslField::projection_view(&source,&path).unwrap(),V::Record(ids)if ids==[0]));path.push(0);let V::Text(projected)=DslField::projection_view(&source,&path).unwrap()else{panic!("actual variant text")};assert_eq!(projected,row["value"].as_str().unwrap());assert_eq!(projected.len(),row["bytes"].as_u64().unwrap()as usize);
  let original=match slot{0=>&source.many[ordinal],1=>&source.nested[ordinal],2=>source.maybe.as_ref().unwrap(),3=>source.required.as_ref(),_=>unreachable!()};let original=match original{TaggedCursorVariant::Document{text}|TaggedCursorVariant::Blob{text}=>text};assert_eq!(projected.as_ptr(),original.as_ptr(),"tagged projection borrows the original backing");
 }
 const FIELDS:&[F]=&[F::new(0,"many",H::Statements(&[("document",tagged_cursor_document),("blob",tagged_cursor_blob)])),F::new(1,"nested",H::Block(tagged_cursor_statements)),F{optional:true,..F::new(2,"maybe",H::Block(tagged_cursor_statements))},F::new(3,"required",H::Block(tagged_cursor_statements))];let spec=crate::BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};let ordinary=print(&source.__dsl_to_record(),&TaggedCursorRecord::__dsl_spec(),JoinMode::Document);
 let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let(result,requests)=observe_requests(||crate::measure_print_borrowed(&source,&spec,ordinary.len(),&mut control));assert_eq!(result.unwrap(),ordinary.len());assert_eq!(requests.bytes,0);assert_eq!(control.owned_bytes(),0);
 let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);assert!(crate::measure_print_borrowed(&source,&spec,ordinary.len()-1,&mut control).is_err());let mut cancel=|_|false;let mut control=NativeEncodeControl::new(0,&mut cancel);assert!(crate::measure_print_borrowed(&source,&spec,ordinary.len(),&mut control).is_err());
 let mut empty=source;empty.maybe=None;assert!(matches!(DslField::projection_view(&empty,&[2,0]).unwrap(),V::Statements(0)));assert!(DslField::projection_key(&empty,&[2,0],0).is_err());assert!(DslField::projection_view(&empty,&[3,0,1]).is_err());
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct BorrowedNumericCoordinates{
 #[dsl(coord)]coord1:[f64;1],
 #[dsl(coord)]coord2:[f64;2],
 #[dsl(coord)]coord3:[f64;3],
 #[dsl(dir)]dir3:[f64;3],
}
#[test]
fn sqlite_snapshot_native_borrowed_coordinates_and_direction_keep_authored_sigils_and_every_word(){
 use crate::{DslField,BorrowedFieldSpec as F,BorrowedShape as H,native_encoding::{FieldProjectionSource,FieldProjectionView as V}};use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📍️coordinates/🔣️.json")).unwrap();
 const FIELDS:&[F]=&[F::new(0,"coord1",H::Coord(1)),F::new(1,"coord2",H::Coord(2)),F::new(2,"coord3",H::Coord(3)),F::new(3,"dir3",H::Dir)];let spec=crate::BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:FIELDS};
 for case in fixture["cases"].as_array().unwrap(){let bits=u64::from_str_radix(case["word"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let source=BorrowedNumericCoordinates{coord1:[value;1],coord2:[value;2],coord3:[value;3],dir3:[value;3]};let ordinary=print(&source.__dsl_to_record(),&BorrowedNumericCoordinates::__dsl_spec(),JoinMode::Document);assert_eq!(ordinary,case["text"].as_str().unwrap());assert_eq!(ordinary.len(),case["bytes"].as_u64().unwrap()as usize);
 let out=std::process::Command::new("bun").args(["-e",r#"import{Database}from'bun:sqlite';const f=JSON.parse(process.argv[1]),word=process.argv[2],actual=process.argv[3];const c=f.cases.find(c=>c.word===word);if(actual!==c.text)throw Error('literal wire');let raw;if(c.atom.startsWith('nan64_'))raw=BigInt('0x'+c.atom.slice(6));else{const value=c.atom==='inf'?Infinity:c.atom==='-inf'?-Infinity:Number(c.atom);const b=new ArrayBuffer(8),v=new DataView(b);v.setFloat64(0,value,false);raw=v.getBigUint64(0,false)}if(raw!==BigInt('0x'+word))throw Error('independent IEEE word');const db=new Database(':memory:');try{db.run('CREATE TABLE emitted(word TEXT,wire TEXT)');db.run('INSERT INTO emitted VALUES(?,?)',[word,c.text]);console.log(JSON.stringify(db.query('SELECT wire,length(CAST(wire AS BLOB)) AS bytes FROM emitted WHERE word=?').get(word)))}finally{db.close()}"#]).arg(fixture.to_string()).arg(case["word"].as_str().unwrap()).arg(&ordinary).output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let interpreted:serde_json::Value=serde_json::from_slice(&out.stdout).unwrap();assert_eq!(interpreted["wire"],case["text"]);assert_eq!(interpreted["bytes"],case["bytes"]);
 for(slot,width)in[1usize,2,3,3].into_iter().enumerate(){assert!(matches!(DslField::projection_view(&source,&[slot]).unwrap(),V::Tuple(n)if n==width));for index in 0..width{assert!(matches!(DslField::projection_view(&source,&[slot,index]).unwrap(),V::Float(value)if value.to_bits()==bits));}}
 let mut yes=|_|true;let mut control=NativeEncodeControl::new(0,&mut yes);let(result,requests)=observe_requests(||crate::measure_print_borrowed(&source,&spec,ordinary.len(),&mut control));assert_eq!(result.unwrap(),ordinary.len());assert_eq!(requests.bytes,0);assert_eq!(control.owned_bytes(),0);
 let mut yes=|_|true;let mut control=NativeEncodeControl::new(0,&mut yes);assert_eq!(crate::measure_print_borrowed(&source,&spec,ordinary.len()-1,&mut control).unwrap_err().kind,ValueRefusalKind::WorkLimit);
 let mut stop=|_|false;let mut control=NativeEncodeControl::new(0,&mut stop);assert_eq!(crate::measure_print_borrowed(&source,&spec,ordinary.len(),&mut control).unwrap_err().kind,ValueRefusalKind::Canceled);
 struct Wrong<'a>{source:&'a BorrowedNumericCoordinates,arity:bool}
 impl FieldProjectionSource for Wrong<'_>{fn projection_view(&self,path:&[usize])->Result<V<'_>,semio_framework_value::ValueError>{if path==[0]&&self.arity{return Ok(V::Tuple(2))}if path==[0,0]&&!self.arity{return Ok(V::Text("wrong"))}DslField::projection_view(self.source,path)}fn projection_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{DslField::projection_key(self.source,path,index)}}
 for arity in[true,false]{let mut yes=|_|true;let mut control=NativeEncodeControl::new(0,&mut yes);assert_eq!(crate::measure_print_borrowed(&Wrong{source:&source,arity},&spec,usize::MAX,&mut control).unwrap_err().kind,ValueRefusalKind::InvalidValue);}
 }
}

#[derive(semio_framework_dsl_record_derive::DslScalar)]
enum AuthoredBorrowedScalar{Ready,#[dsl(key="finished")]Done}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="leaf")]
struct AuthoredBorrowedLeaf{text:String}
#[derive(semio_framework_dsl_record_derive::DslEnum)]
enum AuthoredBorrowedVariant{Document{text:String},#[dsl(key="vacant")]Empty,#[dsl(key="alias")]Leaf(Box<AuthoredBorrowedLeaf>)}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="owner",layout="lines")]
struct AuthoredBorrowedOwner{
 #[dsl(key="identity",positional,defines="owner")]id:String,
 maybe:Option<i64>,
 #[dsl(list)]list:Vec<u64>,
 #[dsl(tuple)]tuple:Vec<f64>,
 #[dsl(statements)]many:Vec<AuthoredBorrowedVariant>,
 #[dsl(statements,block)]nested:Vec<AuthoredBorrowedVariant>,
 map:std::collections::BTreeMap<String,bool>,
 #[dsl(statements)]optional:Option<AuthoredBorrowedVariant>,
 #[dsl(statements)]required:Box<AuthoredBorrowedVariant>,
 #[dsl(base64)]bytes:Vec<u8>,
 #[dsl(table)]rows:Vec<AuthoredBorrowedLeaf>,
 #[dsl(unit="m")]length:f64,
 #[dsl(angle="°")]rotation:f64,
 #[dsl(refs="owner")]reference:String,
 #[dsl(key="language-key")]language:String,
 #[dsl(lang_from="language")]code:String,
 #[dsl(lang="jack")]jack:String,
 #[dsl(coord)]coord:[f64;3],
 #[dsl(dir)]direction:[f64;3],
 scalar:AuthoredBorrowedScalar,
 recursive:std::collections::BTreeMap<String,AuthoredBorrowedOwner>,
}
fn authored_borrowed_kind(shape:crate::BorrowedShape)->&'static str{
 use crate::BorrowedShape as H;
 match shape{
 H::Text=>"Text",H::Int=>"Int",H::UInt=>"UInt",H::Float=>"Float",H::Bool=>"Bool",
 H::List(inner)=>{assert!(matches!(inner(),H::UInt));"List(UInt)"},
 H::Tuple(inner,None)=>{assert!(matches!(inner(),H::Float));"Tuple(Float)"},
 H::Statements(_)=>"Statements",
 H::Block(inner)=>{assert!(matches!(inner(),H::Statements(_)));"Block(Statements)"},
 H::Map(inner)=>match inner(){H::Bool=>"Map(Bool)",H::Record(make)=>{assert_eq!(make().keyword,Some("owner"));"Map(Record)"},_=>panic!("authored map shape")},
 H::Bytes64=>"Bytes64",H::Table(make)=>{assert_eq!(make().keyword,Some("leaf"));"Table"},
 H::Quantity(unit)=>{assert_eq!(unit.symbol,"m");"Quantity(m)"},H::Angle(unit)=>{assert_eq!(unit.symbol,"°");"Angle(°)"},
 H::Ref("owner")=>"Ref(owner)",H::EmbedFrom("language-key")=>"EmbedFrom(language-key)",H::Embed("jack")=>"Embed(jack)",H::Coord(3)=>"Coord(3)",H::Dir=>"Dir",H::Enum(_)=>"Enum",
 _=>panic!("closed authored schema shape"),
 }
}
#[test]
fn child_authored_borrowed_schema_preserves_static_metadata_and_lazy_owner_edges(){
 use crate::{BorrowedDslField,BorrowedDslRecord,BorrowedDslVariants,BorrowedShape as H};
 const SPEC:crate::BorrowedRecordSpec=<AuthoredBorrowedOwner as BorrowedDslRecord>::RECORD;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🏭️authored-static/🔣️.json")).unwrap();
 assert_eq!(SPEC.keyword,fixture["record"]["keyword"].as_str());assert_eq!(SPEC.layout,RecordLayout::Lines);assert_eq!(SPEC.fields.len(),fixture["record"]["fields"].as_array().unwrap().len());
 for(field,expected)in SPEC.fields.iter().zip(fixture["record"]["fields"].as_array().unwrap()){
  assert_eq!(field.id as u64,expected["id"].as_u64().unwrap());assert_eq!(field.key,expected["key"].as_str().unwrap());assert_eq!(field.position.map(u64::from),expected["position"].as_u64());assert_eq!(field.optional,expected["optional"].as_bool().unwrap());assert_eq!(field.defines,expected["defines"].as_str());assert!(!field.flatten&&!field.is_call_name);assert_eq!(authored_borrowed_kind(field.shape),expected["kind"].as_str().unwrap());
 }
 let H::Quantity(length)=SPEC.fields[11].shape else{panic!("quantity owner")};let H::Angle(angle)=SPEC.fields[12].shape else{panic!("angle owner")};assert!(std::ptr::eq(length,semio_framework_dsl::unit_by_symbol("m").unwrap()));assert!(std::ptr::eq(angle,semio_framework_dsl::unit_by_symbol("°").unwrap()));
 let H::Enum(labels)=<AuthoredBorrowedScalar as BorrowedDslField>::SHAPE else{panic!("scalar labels")};for((key,ordinal),expected)in labels.iter().zip(fixture["scalar"].as_array().unwrap()){assert_eq!(*key,expected["key"].as_str().unwrap());assert_eq!(*ordinal as u64,expected["ordinal"].as_u64().unwrap());}assert_eq!(labels.len(),2);
 let variants=<AuthoredBorrowedVariant as BorrowedDslVariants>::VARIANTS;assert_eq!(variants.len(),3);
 for((key,make),expected)in variants.iter().zip(fixture["variants"].as_array().unwrap()){assert_eq!(*key,expected["key"].as_str().unwrap());let spec=make();assert_eq!(spec.keyword,expected["recordKeyword"].as_str());assert_eq!(spec.fields.len(),expected["fields"].as_array().unwrap().len());for(field,expected)in spec.fields.iter().zip(expected["fields"].as_array().unwrap()){assert_eq!(field.id as u64,expected["id"].as_u64().unwrap());assert_eq!(field.key,expected["key"].as_str().unwrap());assert_eq!(authored_borrowed_kind(field.shape),expected["kind"].as_str().unwrap());}}
 let values=[AuthoredBorrowedVariant::Document{text:String::new()},AuthoredBorrowedVariant::Empty,AuthoredBorrowedVariant::Leaf(Box::new(AuthoredBorrowedLeaf{text:String::new()}))];
 const PRODUCER:crate::BorrowedRecordSpecProducer=crate::BorrowedRecordSpecProducer::of::<AuthoredBorrowedOwner>();
 let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accepted);
 let(_,requests)=observe_requests(||{
  for _ in 0..fixture["readRepetitions"].as_u64().unwrap(){
   let produced=PRODUCER.encode(&mut control).unwrap();assert_eq!(produced.fields.as_ptr(),SPEC.fields.as_ptr());
   let repeated=<AuthoredBorrowedOwner as BorrowedDslRecord>::RECORD;assert_eq!(repeated.fields.as_ptr(),SPEC.fields.as_ptr());for field in repeated.fields{std::hint::black_box(authored_borrowed_kind(field.shape));}
   let H::Map(inner)=repeated.fields[20].shape else{panic!("recursive owner")};let H::Record(make)=inner()else{panic!("lazy record")};assert_eq!(make().fields.as_ptr(),SPEC.fields.as_ptr());
   for(index,value)in values.iter().enumerate(){let(key,ordinal,spec)=value.projected_borrowed_variant_identity();assert_eq!(ordinal,index);assert_eq!(key,variants[index].0);assert_eq!(spec.fields.as_ptr(),variants[index].1().fields.as_ptr());}
  }
 });
 assert_eq!(control.owned_bytes(),0);
 let mut refused=|_|false;let mut canceled=semio_framework_value::NativeEncodeControl::new(0,&mut refused);assert_eq!(PRODUCER.encode(&mut canceled).err().unwrap().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);
 assert_eq!(requests.bytes,fixture["expectedAllocatedBytes"].as_u64().unwrap()as usize);assert_eq!(requests.length,0);
 eprintln!("[DEBUG] actual authored static schema fields21 kinds11 refinements7 variants3 repeated256 actual allocator requests0; recursive/source pointers preserved");
}
