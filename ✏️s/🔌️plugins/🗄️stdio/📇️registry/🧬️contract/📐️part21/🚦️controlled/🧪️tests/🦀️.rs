//! 🧪️ Shared Part21 inputs retain their exact success, rejection and cancellation allocations.
use super::*;
use semio_framework_value::{ErasedSnapshotRetirement,native_decoding::NativeDecodeRetirementRecipient,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use std::cell::Cell;

fn drain(owner:&mut dyn ErasedSnapshotRetirement)->(usize,usize){
 let mut requested=0;let mut released=0;
 for _ in 0..32768{
  if owner.terminal_is_empty(){return(requested,released);}
  let copy=owner.next_copy_byte_demand().unwrap().max(256);
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
  for axis in 0..5{
   let mut denied=grant;
   match axis{0=>denied.maximum_items=0,1 if owner.next_copy_byte_demand().unwrap()>0=>denied.maximum_copy_bytes=owner.next_copy_byte_demand().unwrap()-1,2 if grant.maximum_capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if grant.maximum_release_bytes>0=>denied.maximum_release_bytes-=1,4 if grant.maximum_depth>0=>denied.maximum_depth-=1,_=>continue};
   let(result,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(denied));assert_eq!(heap,Default::default());
   match result{Ok(step)=>assert_eq!(match step{RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p)=>p},Default::default()),Err(error)=>assert!(matches!(error.kind,ValueRefusalKind::DepthLimit|ValueRefusalKind::OwnershipLimit|ValueRefusalKind::WorkLimit))}
  }
  let(result,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(grant));let progress=match result.unwrap(){RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p)=>p};
  assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);assert!(progress.copied_items<=1&&progress.copied_bytes<=grant.maximum_copy_bytes&&progress.retained_capacity_bytes<=grant.maximum_capacity_bytes&&progress.released_bytes<=grant.maximum_release_bytes);
  requested+=heap.requested_bytes;released+=heap.released_bytes;
 }
 panic!("original Part21 owner did not reach its terminal witness")
}
fn dispose<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("typed Part21 result has original retirement"));drain(&mut owner)}
#[derive(semio_framework_value::RetireOwned)]
enum Decoded{Value(Part21Value),Instance(Part21Instance),Document(Part21Document)}
impl Decoded{fn value(&self)->DslValue{match self{Self::Value(v)=>v.to_value(),Self::Instance(v)=>v.to_value(),Self::Document(v)=>v.to_value()}}}
fn decode(kind:&str,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Decoded,ValueError>{match kind{"value"=>Part21Value::from_value_controlled(value,control).map(Decoded::Value),"instance"=>Part21Instance::from_value_controlled(value,control).map(Decoded::Instance),"document"=>Part21Document::from_value_controlled(value,control).map(Decoded::Document),_=>panic!("unknown shared native input")}}

#[test]
fn part21_original_decoder_returns_every_success_and_cancelled_prefix_to_actual_recipient(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let wire=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&row["value"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  let kind=row["kind"].as_str().unwrap();let calls=Cell::new(0);let mut allow=|_|{calls.set(calls.get()+1);true};let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1048576,&mut allow);control.install_retirement_recipient(&mut recipient).unwrap();
  let(result,heap)=observe_heap_allocations_on_this_thread(||decode(kind,&wire,&mut control));assert_eq!(control.owned_bytes(),heap.requested_bytes);assert_eq!(heap.released_bytes,0);drop(control);let result=result.unwrap();
  let json:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.value())).unwrap();assert_eq!(json,row["value"]);let(c1,r1)=drain(&mut recipient);let(c2,r2)=dispose(result);assert_eq!(r1+r2,heap.requested_bytes+c1+c2);
  for cutoff in 1..=calls.get(){
   let checkpoint=Cell::new(0);let mut cancel=|_|{checkpoint.set(checkpoint.get()+1);checkpoint.get()<cutoff};let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1048576,&mut cancel);control.install_retirement_recipient(&mut recipient).unwrap();
   let(result,heap)=observe_heap_allocations_on_this_thread(||decode(kind,&wire,&mut control));assert!(control.owned_bytes()>=heap.requested_bytes);assert_eq!(heap.released_bytes,0);drop(control);assert_eq!(result.err().unwrap().kind,ValueRefusalKind::Canceled);let(c,r)=drain(&mut recipient);assert_eq!(r,heap.requested_bytes+c);
  }
  let mut allow=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut allow);let(result,heap)=observe_heap_allocations_on_this_thread(||decode(kind,&wire,&mut control));assert_eq!(result.err().unwrap().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(heap,Default::default());
 }
 println!("[DEBUG] shared Part21 input success and every native cancellation checkpoint preserve actual original owner, exact four-axis allocator events and third-party serde_json output");
}

#[test]
fn part21_rejected_nested_child_preserves_owned_parent_text_and_siblings(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let mut row=fixture["cases"][0]["value"].clone();row["values"][1]["value"]["coefficient"]=serde_json::json!(1);
 let wire=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&row.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());let mut allow=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1048576,&mut allow);control.install_retirement_recipient(&mut recipient).unwrap();
 let(result,heap)=observe_heap_allocations_on_this_thread(||Part21Value::from_value_controlled(&wire,&mut control));assert_eq!(result.err().unwrap().kind,ValueRefusalKind::InvalidValue);assert!(heap.requested_bytes>0);assert_eq!(heap.released_bytes,0);assert_eq!(control.owned_bytes(),heap.requested_bytes);drop(control);let(c,r)=drain(&mut recipient);assert_eq!(r,heap.requested_bytes+c);
 println!("[DEBUG] malformed nested decimal retains original typed-name and completed sibling until funded physical retirement");
}

#[test]
fn part21_cohort_retirement_releases_original_capacity_without_literal_byte_credit(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️sqlite-cohort/🔣️.json")).unwrap();let row=&fixture["retirementCase"];
 let original=Part21Instance{id:row["id"].as_str().unwrap().parse().unwrap(),entities:vec![(row["name"].as_str().unwrap().into(),vec![Part21Value::Typed{name:row["typedName"].as_str().unwrap().into(),items:vec![Part21Value::Str(row["value"].as_str().unwrap().into())]}])]};
 let literal=row["name"].as_str().unwrap().len()+row["typedName"].as_str().unwrap().len()+row["value"].as_str().unwrap().len();assert_eq!(literal,row["expectedTextBytes"].as_u64().unwrap()as usize);
 let name=original.entities[0].0.capacity();let Part21Value::Typed{name:typed,items}= &original.entities[0].1[0]else{panic!("typed fixture")};let Part21Value::Str(text)=&items[0]else{panic!("text fixture")};
 let backing=name+typed.capacity()+text.capacity()+original.entities.capacity()*std::mem::size_of::<(String,Vec<Part21Value>)>()+original.entities[0].1.capacity()*std::mem::size_of::<Part21Value>()+items.capacity()*std::mem::size_of::<Part21Value>();let(c,r)=dispose(original);assert_eq!(r,backing+c);
 println!("[DEBUG] literal text length does not stand in for original String, Vec or retirement-frame release capacity");
}

