use super::*;
use crate::{DiffAlgebra,MutationDiff,MutationApplyResult,MutationOutcome,MutationLeafDescriptor,RetainedMutation,RetainedMutationOwner,begin_retained_mutation,retained_mutation_birth_demand,advance_retained_mutation,RetainedMutationStep};
use semio_framework_value::{retirement::{RetireOwned,OwnedValueRetirementFactory},retained_clone::RetainedCloneSource};
#[derive(Debug,semio_framework_value::RetireOwned)]
struct Row{id:String,name:String,value:f64,untouched:Vec<u64>}
#[derive(semio_framework_value::RetireOwned)]
struct Snapshot{rows:Vec<Row>}
#[derive(Clone,Default,Debug,PartialEq,serde::Deserialize,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_value::RetireOwned)]
#[value(default)]
struct Patch{name:Option<String>,value:Option<f64>}
crate::retained_row_patch!(pub fields for Patch => Row{name:String,value:f64});
#[derive(Clone,Debug,PartialEq,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_value::RetireOwned)]
struct Operation{id:String,patch:Patch}
#[derive(Clone,Default,PartialEq,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue)]
struct Diff{}
impl DiffAlgebra<Snapshot> for Diff{fn is_empty(&self)->bool{true}}
impl MutationDiff<Snapshot> for Diff{}
impl Mutation<Snapshot> for Operation{type Diff=Diff;const DESCRIPTORS:&'static[MutationLeafDescriptor]=&[];fn descriptor(&self)->&'static MutationLeafDescriptor{panic!("fixture semantic descriptor unavailable")}}
impl RetainedMutation<Snapshot> for Operation{type Preparation=RetainedRowPreparation<Snapshot,Self,Self,1>;}
struct Guard{complete:bool}
impl RetainedRowGuard<Snapshot,Row,Patch> for Guard{
 fn new()->Self{Self{complete:false}}
 fn demand(&self,_:RetainedCloneRef<'_,Snapshot>,_:RetainedCloneRef<'_,Row>,_:RetainedCloneRef<'_,Patch>,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Option<f64>>()+size_of::<bool>(),depth:1,..Default::default()})}
 fn advance(&mut self,before:RetainedCloneRef<'_,Snapshot>,row:RetainedCloneRef<'_,Row>,patch:RetainedCloneRef<'_,Patch>,grant:RetainedCloneGrant)->Result<RetainedRowGuardStep,ValueError>{let demand=self.demand(before,row,patch,grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedRowGuardStep::Progress(Default::default()));}assert!(!self.complete);self.complete=true;let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok(if patch.get().value.is_some_and(|value|value<0.){RetainedRowGuardStep::Ready(receipt,RetainedRowDecision::Refuse,Some(RetainedDiagnosticSpec{level:semio_framework_diagnostic::Severity::Fatal,code:"mutation.invariant",message:"The authored field is inadmissible."}))}else if patch.get().value.is_none()&&patch.get().name.is_none(){RetainedRowGuardStep::Ready(receipt,RetainedRowDecision::Noop,None)}else{RetainedRowGuardStep::Ready(receipt,RetainedRowDecision::Admit,None)})}
 fn begin_close(&mut self){}
 fn close_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{true}
}
impl RetainedRowMutation<Snapshot,Self,1> for Operation{
 type Row=Row;type Patch=Patch;type Guard=Guard;
 fn rows(base:&Snapshot)->&Vec<Row>{&base.rows}
 fn rows_mut(base:&mut Snapshot)->&mut Vec<Row>{&mut base.rows}
 fn row_key(row:&Row)->&str{&row.id}
 fn key(&self)->&String{&self.id}
 fn patch(&self)->&Patch{&self.patch}
 fn target(&self,index:usize)->&String{assert_eq!(index,0);&self.id}
 fn from_inverse([id]:[String;1],patch:Patch)->Self{Self{id,patch}}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(op:impl FnOnce()->T)->(T,(usize,usize)){let(result,born,released)=crate::test_allocation::observe_backing(op);(result,(born,released))}
fn source<T:RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{match RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}){Ok((source,_))=>source,Err((error,_,()))=>panic!("original row fixture source:{error}")}}
fn close_source<T:RetireOwned+Sync>(source:&mut RetainedCloneSource<T>){while !source.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});source.close_step(quote).unwrap();}}
fn close(owner:&mut RetainedMutationOwner<Snapshot,Operation>){let factory=OwnedValueRetirementFactory::<Operation>::default();let context=RetainedMutationCloseContext{mutations:&factory};owner.begin_close();let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<2000);let quote=grant(owner.close_demand(context,4096).unwrap());let(step,heap)=observe(||owner.close_step(context,quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
fn snapshot(rows:&serde_json::Value)->Snapshot{Snapshot{rows:rows.as_array().unwrap().iter().map(|row|Row{id:row["id"].as_str().unwrap().into(),name:row["name"].as_str().unwrap().into(),value:row["value"].as_f64().unwrap(),untouched:vec![42;256]}).collect()}}
fn projection(base:&Snapshot)->serde_json::Value{serde_json::Value::Array(base.rows.iter().map(|row|serde_json::json!({"id":row.id,"name":row.name,"value":row.value})).collect())}
fn settle_rows(rows:Vec<Operation>,messages:Vec<super::super::super::MutationMessage>){let factory=OwnedValueRetirementFactory::<Operation>::default();let(mut owner,_)=FactoryVectorRetirement::admit(rows,grant(FactoryVectorRetirement::<Operation>::constructor_demand())).unwrap();while !owner.terminal_is_empty(){let quote=grant(owner.demands(&factory,4096).unwrap());owner.close_step(&factory,quote).unwrap();}let mut ledger=MutationMessageLedgerRetirement::new(String::new(),messages);while !ledger.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(4096).unwrap(),release_bytes:ledger.next_release_byte_demand().unwrap(),depth:ledger.next_depth_demand().unwrap()});ledger.close_step(quote).unwrap();}}

/// 🧫️ Required generic row preparation follows portable projections with cold decision/inverse/apply forbidden.
#[test]
fn retained_row_mutation_composes_guard_original_inverse_and_owned_commit(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let event=Operation{id:case["target"].as_str().unwrap().into(),patch:serde_json::from_value(case["patch"].clone()).unwrap()};let mut event=source(event);let mut original=source(snapshot(&case["rows"]));let mut post=snapshot(&case["rows"]);let pointer=post.rows.as_ptr();let sibling=post.rows[0].untouched.as_ptr();
  let(mut owner,_)=begin_retained_mutation::<Snapshot,Operation>(grant(retained_mutation_birth_demand::<Snapshot,Operation>())).unwrap();let mut turns=0;
  let(rows,messages)=loop{turns+=1;assert!(turns<2000,"bounded row owner stalled");let quote=grant(owner.demand(event.borrow(),original.borrow(),&post,256).unwrap());let(step,heap)=observe(||advance_retained_mutation(&mut owner,event.borrow(),original.borrow(),&mut post,RetainedCloneGrant::default()).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());let(step,heap)=observe(||advance_retained_mutation(&mut owner,event.borrow(),original.borrow(),&mut post,quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));match step{RetainedMutationStep::Progress(_)=>{},RetainedMutationStep::Admitted(_,rows,messages)=>{assert!(case["admitted"].as_bool().unwrap());break(rows,messages)},RetainedMutationStep::Refused(_,messages)=>{assert!(!case["admitted"].as_bool().unwrap());break(Vec::new(),messages)}}};
  assert_eq!(rows.len(),case["inverseCount"].as_u64().unwrap()as usize);if let Some(row)=rows.first(){assert_eq!(row.id,case["target"].as_str().unwrap());assert_eq!(row.patch.name.as_deref(),case["patch"].get("name").map(|_|case["rows"][0]["name"].as_str().unwrap()));assert_eq!(row.patch.value,case["patch"].get("value").map(|_|case["rows"][0]["value"].as_f64().unwrap()));}
  let actual=projection(&post);let expected=snapshot(&case["expected"]);assert_eq!(actual,projection(&expected));assert_eq!(post.rows.as_ptr(),pointer);assert_eq!(post.rows[0].untouched.as_ptr(),sibling);if !case["admitted"].as_bool().unwrap(){assert_eq!(messages.len(),1);assert_eq!(messages[0].target,vec![case["target"].as_str().unwrap().to_owned()]);}
  close(&mut owner);settle_rows(rows,messages);close_source(&mut event);close_source(&mut original);
  eprintln!("[DEBUG] generic retained row {:?}: guard-before-inverse, exact sparse inverse, original list/sibling pointers, cold calls0, turns={turns}",case["id"]);
 }
}

/// 🛑️ Partial preflight, inverse and commit frontiers keep original inverse factory custody until cancellation closes.
#[test]
fn retained_row_mutation_cancellation_closes_every_composed_frontier(){
 for cancel in 0..90{
  let mut event=source(Operation{id:"r1".into(),patch:Patch{name:Some("After".into()),value:Some(2.)}});let mut original=source(Snapshot{rows:vec![Row{id:"r1".into(),name:"Before".into(),value:1.,untouched:vec![]}]});let mut post=Snapshot{rows:vec![Row{id:"r1".into(),name:"Before".into(),value:1.,untouched:vec![]}]};let(mut owner,_)=begin_retained_mutation::<Snapshot,Operation>(grant(retained_mutation_birth_demand::<Snapshot,Operation>())).unwrap();
  for _ in 0..cancel{let quote=grant(owner.demand(event.borrow(),original.borrow(),&post,128).unwrap());match advance_retained_mutation(&mut owner,event.borrow(),original.borrow(),&mut post,quote).unwrap(){RetainedMutationStep::Progress(_)=>{},RetainedMutationStep::Admitted(_,rows,messages)=>{settle_rows(rows,messages);break;},RetainedMutationStep::Refused(_,messages)=>{settle_rows(vec![],messages);break;}}}
  close(&mut owner);close_source(&mut event);close_source(&mut original);
 }
}