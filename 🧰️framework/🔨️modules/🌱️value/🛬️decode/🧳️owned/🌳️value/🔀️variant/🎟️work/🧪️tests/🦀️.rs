use crate::*;
use crate::owned::tree::{NativeValueSource,NativeTreeKind};
use crate::owned::{NativeRecordSchema,NativeSchemaFields,NativeSchemaLeaf};
use crate::owned::tree::variant::*;
use crate::owned::{NativeAdvanceSelection,NativeReceiver};
use crate::{DslValue,OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError,RetainedWorkOutcome};
use crate::retained_clone::RetainedCloneRef;
use crate::driver;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
fn all(i:usize)->Option<NativeRecordSchema>{["empty","tuple","record"].get(i).map(|name|NativeRecordSchema::Leaf(NativeSchemaLeaf{name,ordinal:i as u32}))}
fn units(i:usize)->Option<NativeRecordSchema>{if i==0{all(i)}else if i<3{Some(NativeRecordSchema::Skip)}else{None}}
fn data(i:usize)->Option<NativeRecordSchema>{if i==0{Some(NativeRecordSchema::Skip)}else{all(i)}}
fn tag(i:usize)->Option<NativeRecordSchema>{(i==0).then_some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"kind",ordinal:0}))}
type Reply<'r,E>=OriginalEffectReply<'r,RetainedCloneStep,E>;
type Done<'r,E>=Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>;
struct Selector{inner:NativeVariantSelector<DslValue>,closing:bool,taken:bool}
impl NativeReceiver<DslValue,u64> for Selector{
 fn demands(&self,i:RetainedCloneRef<'_,DslValue>,b:usize,m:u64)->Result<RetirementDemand,ValueError>{self.inner.demands(i,b,m)}
 fn select_advance(&self,i:RetainedCloneRef<'_,DslValue>,b:usize,m:u64)->Result<NativeAdvanceSelection,ValueError>{self.inner.select_advance(i,b,m)}
 fn advance_nested(&mut self,i:RetainedCloneRef<'_,DslValue>,c:&mut NativeDecodeControl<'_>,w:OriginalSourceTurnWitness<'_>,b:usize,m:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{self.inner.advance_nested(i,c,w,b,m,g)}
 fn advance_local<'r>(&mut self,i:RetainedCloneRef<'_,DslValue>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{self.inner.advance_local(i,b,e,r)}
 fn advance_pending(&self)->bool{self.inner.advance_pending()}
 fn output_ready(&self)->bool{self.inner.result().is_some()&&!self.taken&&!self.closing}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn take<'r>(&mut self,o:&mut Option<u64>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{if !crate::retained_clone_grant_admits(e.grant(),self.take_demand()?)||e.maximum_work_units()==0{return Ok(r.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !e.performed(1)?{return Ok(r.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"selector take exceeded its original work").into())))}*o=Some(self.inner.result().ok_or_else(||ValueError::invariant("selector has no selection"))?.index as u64);self.taken=true;Ok(r.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
 fn is_closing(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(self.inner.begin_close_demand())}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(self.inner.begin_close_work_demand())}
 fn begin_close_into<'r>(&mut self,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{let turn=self.inner.begin_close_paid(e);if turn.result.is_ok(){self.closing=true}Ok(r.complete(turn.result.map_err(Into::into)))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{Ok(r.pending())}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.inner.close_demands(b)}
 fn close_work_demand(&self,b:usize)->Result<u64,ValueError>{self.inner.close_work_demand(b)}
 fn close_step<'r>(&mut self,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.close_into(b,e,r)}
 fn recover_close_step<'r>(&mut self,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.recover_close_into(b,e,r)}
 fn close_original_step<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.close_original_into(o,b,e,r)}
 fn recover_close_original_step<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.recover_close_original_into(o,b,e,r)}
 fn close_original_birth_step(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.inner.close_original_birth_step(o,g)}
 fn requires_original_control(&self)->bool{self.inner.requires_original_control()}
 fn requires_original_birth(&self)->bool{self.inner.requires_original_birth()}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}
#[test]fn actual_native_variant_selection_preserves_wire_case_and_cancelled_original_ownership(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 for case in law["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()))){
  let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let source=driver::admit(&mut bank,DslValue::from(case["input"].clone()));
  let spec=NativeVariantSpec{all:unsafe{NativeSchemaFields::native(all)},units:unsafe{NativeSchemaFields::native(units)},data:unsafe{NativeSchemaFields::native(data)},count:3,tag:case["tagged"].as_bool().unwrap().then_some(unsafe{NativeSchemaFields::native(tag)})};
  let(inner,heap)=crate::observe_retirement_allocations(||NativeVariantSelector::new(spec));assert_eq!(heap,(0,0));let mut selector=Selector{inner,closing:false,taken:false};
  let selected=driver::receive_cut(&mut selector,source.borrow(),&mut bank,4096,cut);
  if cut.is_none(){assert_eq!(selected,Ok(Some(case["selected"].as_u64().unwrap())));let result=selector.inner.result().unwrap();assert!(!result.direct);assert_eq!(serde_json::Value::from(source.borrow().get()),case["input"]);}else{assert!(matches!(selected,Ok(None)|Err(_)),"{selected:?}")}
  driver::close(&mut selector,&mut bank,4096);assert!(selector.terminal_is_empty());drop(selector);driver::close_source(source,&mut bank);
 }}
}
