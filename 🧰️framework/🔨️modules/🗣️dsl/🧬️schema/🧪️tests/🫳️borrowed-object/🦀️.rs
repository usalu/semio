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
    let schema:serde_json::Value=serde_json::from_str(include_str!("🧬️schema/🔣️.json")).unwrap();
    let output = std::process::Command::new("bun").args(["-e", "import Ajv from 'ajv/dist/2020.js';import {Database} from 'bun:sqlite';const x=JSON.parse(process.argv[1]);if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('borrowed object occurrence fixture');const db=new Database(':memory:');try{db.run('CREATE TABLE keys(ordinal INTEGER PRIMARY KEY,value TEXT NOT NULL)');for(const [ordinal,key]of x.fixture.payloadKeys.entries())db.run('INSERT INTO keys VALUES(?,?)',[ordinal,key]);await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT value FROM keys ORDER BY ordinal').all().map(row=>row.value)));}finally{db.close();}",&serde_json::json!({"schema":schema,"fixture":fixture}).to_string()]).output().unwrap();
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
