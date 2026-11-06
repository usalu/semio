//! 🧮️ Every receiving turn charges exactly the heap it requested to the decode control, per receiver family.
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneSource};
use semio_framework_value::{DslValue,Number,RetirementDemand,ValueError};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeConstructionRecipient,NativeDecodeControl,NativeReceive};
use semio_framework_value_native_owned::native_decoding::owned::NativeReceivingOriginal;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}}
#[derive(Default)] struct Turn(semio_framework_value::RetainedCloneProgress);
impl NativeDecodeConstructionRecipient for Turn{
 fn submitted(&self)->semio_framework_value::RetainedCloneProgress{self.0}fn grant(&mut self,_:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(grant())}
 fn accept(&mut self,p:RetainedCloneGrant,_:semio_framework_value::RetainedCloneProgress)->Result<(),ValueError>{self.0=self.0.checked_add(p)?;Ok(())}
}
fn source(value:DslValue)->RetainedCloneSource<DslValue>{
 let mut admission=RetainedCloneSource::<DslValue>::admission_owned(value,());let mut out=None;
 for _ in 0..64{let ready=admission.phase()=="ready";let turn=if ready{admission.take_output(&mut out,grant())}else{admission.advance(grant())};turn.result.unwrap();if ready{break}}
 out.expect("source admitted")
}
fn law<T:NativeReceive<DslValue>+Sync>(value:DslValue){
 let mut owner=NativeReceivingOriginal::<DslValue,T>::new(source(value));
 let mut accept=|_|true;let mut control=NativeDecodeControl::new_retained(&mut accept);
 for turn in 0..10000{
  if owner.output_ready(){break}
  let before=control.owned_bytes();
  let(result,heap)=observe(||control.with_construction_recipient(&mut Turn::default(),|control|owner.advance(control,4096,grant()).result));
  result.unwrap_or_else(|error|panic!("receiving turn {turn} refused: {}",error.message()));
  assert_eq!(control.owned_bytes()-before,heap.requested_bytes,"turn {turn} charged a different byte count than it allocated");
 }
 assert!(owner.output_ready(),"receiving did not finish");
 let(output,_)=owner.take(grant()).result.unwrap().expect("funded take");
 let mut close=ControlledRetirement::new(output).unwrap_or_else(|_|panic!("output has controlled retirement"));
 for _ in 0..10000{if close.terminal_is_empty(){break}close.step(grant()).unwrap();}
 owner.begin_close();
 for _ in 0..10000{if owner.terminal_is_empty(){break}owner.close_step(grant()).result.unwrap();}
 assert!(owner.terminal_is_empty());
}
fn text(value:&str)->DslValue{DslValue::String(value.into())}
fn object(entries:Vec<(&str,DslValue)>)->DslValue{DslValue::Object(entries.into_iter().map(|(key,value)|(key.to_string(),value)).collect())}

#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
struct Record{first:String,second:String}
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
enum Choice{Named{first:String},Other}

#[test]fn scalar_text_receiving_charges_its_allocations(){law::<String>(text("text"));}
#[test]fn sequence_receiving_charges_its_allocations(){law::<Vec<String>>(DslValue::Array(vec![text("a"),text("bc")]));}
#[test]fn bulk_sequence_receiving_charges_its_allocations(){law::<Vec<u64>>(DslValue::Array(vec![DslValue::Number(Number::UInt(1)),DslValue::Number(Number::UInt(2))]));}
#[test]fn tuple_receiving_charges_its_allocations(){law::<(String,String)>(DslValue::Array(vec![text("a"),text("b")]));}
#[test]fn array_receiving_charges_its_allocations(){law::<[String;2]>(DslValue::Array(vec![text("a"),text("b")]));}
#[test]fn persistent_list_receiving_charges_its_allocations(){law::<semio_framework_value_persistent::list::persistent::PersistentList<String>>(DslValue::Array(vec![text("a"),text("bc"),text("def")]));}
#[test]fn derived_record_receiving_charges_its_allocations(){law::<Record>(object(vec![("first",text("a")),("second",text("b"))]));}
#[test]fn derived_variant_receiving_charges_its_allocations(){law::<Choice>(object(vec![("Named",object(vec![("first",text("a"))]))]));}
