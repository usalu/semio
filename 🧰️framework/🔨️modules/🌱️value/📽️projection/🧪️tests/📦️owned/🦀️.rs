use semio_framework_test_fixtures::fixture_text;
use super::*;

#[derive(crate::RetireOwned,crate::SnapshotProjection)]
#[value(crate="crate")]
struct OwnedRecord{schema:String,bytes:Vec<u8>,dictionary:Option<Vec<u8>>}
impl Clone for OwnedRecord{fn clone(&self)->Self{panic!("whole record projection must never clone")}}

fn project_owned(count:usize,schema:&str,octet:u8,optional:bool,cut:Option<usize>){
 let value=OwnedRecord{schema:schema.into(),bytes:vec![octet;count],dictionary:optional.then(||vec![octet;count])};
 let d=RetainedCloneSource::<OwnedRecord>::owned_constructor_demand::<()>();
 let grant=funded(RetirementDemand{copy_bytes:RetainedCloneSource::<OwnedRecord>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()});
 let(mut source,_)=RetainedCloneSource::admit_owned(value,(),grant).map_err(|(e,..)|e).unwrap();
 let pointer=source.borrow().get().bytes.as_ptr();let optional_pointer=source.borrow().get().dictionary.as_ref().map(Vec::as_ptr);
 let((mut cursor,p),heap)=observe_retirement_allocations(||OwnedRecord::begin_projection(funded(OwnedRecord::projection_birth_demand())).unwrap());
 assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
 let mut turns=0;
 while !cursor.ready()&&cut.is_none_or(|cut|turns<cut){
  let d=cursor.demands(source.borrow(),4096).unwrap();let grant=funded(d);
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){
   let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,denied).unwrap());
   assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.demands(source.borrow(),4096).unwrap(),d);
  }
  let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,grant).unwrap());
  assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;assert!(turns<count.saturating_mul(4).saturating_add(schema.len()).saturating_add(256));
 }
 if cursor.ready()&&cut.is_none(){
  let d=cursor.take_demand().unwrap();let grant=funded(d);
  assert!(cursor.take(RetainedCloneGrant{maximum_items:0,..grant}).unwrap().is_none());
  let(output,p)=cursor.take(grant).unwrap().unwrap();assert!(p.fits(grant));
  assert_eq!(output.schema,schema);assert_eq!(output.bytes,vec![octet;count]);assert_eq!(output.dictionary,optional.then(||vec![octet;count]));
  if count>0{assert_ne!(output.bytes.as_ptr(),pointer);}
  drain(output);
 }
 cursor.begin_close();for _ in 0..100000{
  if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let grant=funded(d);
  let(step,heap)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
  let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
 }
 assert!(cursor.terminal_is_empty());assert_eq!(source.borrow().get().bytes.as_ptr(),pointer);assert_eq!(source.borrow().get().bytes.len(),count);assert_eq!(source.borrow().get().dictionary.as_ref().map(Vec::as_ptr),optional_pointer);
 assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
 for _ in 0..256usize.saturating_add(schema.len()).saturating_add(count.saturating_mul(if optional{32}else{16})){if source.terminal_is_empty(){break;}let d=RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};source.close_step(funded(d)).unwrap();}
 assert!(source.terminal_is_empty());
 eprintln!("[DEBUG] owned projection bytes={count} cut={cut:?} turns={turns} original stable, exact heap receipts, zero whole-record clones, terminal closure");
}

#[test]
fn owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📦️owned/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let count=row["count"].as_u64().unwrap()as usize;let schema=row["schema"].as_str().unwrap();let octet=row["octet"].as_u64().unwrap()as u8;let optional=row["optionalPayload"].as_bool().unwrap();project_owned(count,schema,octet,optional,None);for cut in corpus["cancelCuts"].as_array().unwrap(){project_owned(count,schema,octet,optional,Some(cut.as_u64().unwrap()as usize));}}
}
