//! 🧪️ Original ordered sparse decisions share SQLite output and genuine typed source custody.
use protocol::{RetainedDiffSequence,RetainedDiff,RetainedDiffFragment,RetainedDiffApply,RetainedDiffPreflightAdvance,RetainedDiffPreflightStep,RetainedFieldPreparation,RetainedDiagnosticPreparation,RetainedDiagnosticSpec,MutationDiff,DiffAlgebra,MutationMessage,ApplyCapability};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,FromValue,retained_clone::{RetainedCloneRef,RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_snapshot_projection::snapshot_projection::{SnapshotProjection,SnapshotProjectionCursor,OwnedFieldProjection};
use std::mem::{ManuallyDrop,size_of};
use std::sync::atomic::{AtomicUsize,Ordering};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
static PROJECTIONS:AtomicUsize=AtomicUsize::new(0);
static PROJECTION_GATE:std::sync::Mutex<()>=std::sync::Mutex::new(());
#[derive(Debug,PartialEq,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone,semio_framework_value::ToValue,semio_framework_value::FromValue)]
struct Snapshot{a:i64,b:i64,text:String}
impl Clone for Snapshot{fn clone(&self)->Self{panic!("ordered native candidate must use actual SnapshotProjection")}}
struct Projection(OwnedFieldProjection<Snapshot>);
impl SnapshotProjection for Snapshot{
 type Output=Self;
 type Projection=Projection;
 fn projection_birth_demand()->RetirementDemand{OwnedFieldProjection::<Self>::birth_demand()}
 fn begin_projection(g:RetainedCloneGrant)->Result<(Self::Projection,RetainedCloneProgress),ValueError>{OwnedFieldProjection::<Self>::begin(g).map(|(cursor,p)|(Projection(cursor),p))}
}
impl SnapshotProjectionCursor<Snapshot> for Projection{
 fn demands(&self,s:RetainedCloneRef<'_,Snapshot>,body:usize)->Result<RetirementDemand,ValueError>{self.0.demands(s,body)}
 fn advance(&mut self,s:RetainedCloneRef<'_,Snapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{PROJECTIONS.fetch_add(1,Ordering::Relaxed);self.0.advance(s,body,g)}
 fn ready(&self)->bool{self.0.ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{self.0.take_demand()}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(Snapshot,RetainedCloneProgress)>,ValueError>{self.0.take(g)}
 fn begin_close(&mut self){self.0.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.0.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
#[derive(Clone,Default,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
struct Edit{#[value(default)]set_a:Option<i64>,#[value(default)]set_b:Option<i64>,#[value(default)]require_a:Option<i64>,#[value(default)]warning:bool}
struct EditApply{a:RetainedFieldPreparation<i64>,b:RetainedFieldPreparation<i64>,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,retirement:semio_framework_value::retained_clone::RetainedCloneClose,phase:u8,part:u8,refused:bool,closing:bool}
impl RetainedDiffFragment<Snapshot> for Edit{type FragmentApply=EditApply;}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original ordered fixture is outside its typed frontier")}
fn control()->usize{size_of::<usize>()*4+size_of::<bool>()*2+size_of::<u8>()*2}
fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:control(),depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=control();d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-control(),maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=control();p}
impl RetainedDiffApply<Snapshot,Edit> for EditApply{
 fn new()->Self{Self{a:Default::default(),b:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),retirement:Default::default(),phase:0,part:0,refused:false,closing:false}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,Edit>,_:RetainedCloneRef<'_,Snapshot>,_:&Snapshot,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase==6{return Err(invalid());}Ok(match self.phase{
   0=>scalar(),
   1 if d.get().set_a.is_some()&&self.a.value().is_none()=>nested(self.a.demand(d.project(1,|d|d.set_a.as_ref().unwrap()),body)?),
   2 if d.get().set_b.is_some()&&self.b.value().is_none()=>nested(self.b.demand(d.project(2,|d|d.set_b.as_ref().unwrap()),body)?),
   3 if self.refused||d.get().warning=>nested(RetainedDiagnosticPreparation::<0>::start_demand()),
   4=>nested(self.diagnostic.demand([],body)?),
   5=>nested(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()}),
   _=>scalar(),
  })
 }
 fn preflight(&mut self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize,g:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{
  let demand=self.preflight_demand(d,s,post,body)?;if !protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()));}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{self.refused=d.get().require_a.is_some_and(|expected|expected!=s.get().a);self.phase=if self.refused{3}else{1};},
   1=>if d.get().set_a.is_some()&&self.a.value().is_none(){p=receipt(self.a.advance(d.project(1,|d|d.set_a.as_ref().unwrap()),body,child(g))?.progress());}else{self.phase=2;},
   2=>if d.get().set_b.is_some()&&self.b.value().is_none(){p=receipt(self.b.advance(d.project(2,|d|d.set_b.as_ref().unwrap()),body,child(g))?.progress());}else{self.phase=3;},
   3=>if self.refused||d.get().warning{p=receipt(self.diagnostic.start(RetainedDiagnosticSpec{level:if self.refused{semio_framework_diagnostic::Severity::Fatal}else{semio_framework_diagnostic::Severity::Warning},code:"sequence.original",message:"Original ordered sparse row."},child(g))?);self.phase=4;}else{self.phase=6;return Ok(RetainedDiffPreflightAdvance::Ready(p,true));},
   4=>{let step=self.diagnostic.advance([],body,child(g))?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5;}},
   5=>{let(rows,progress)=self.diagnostic.take(child(g))?.ok_or_else(invalid)?;*self.messages=Some(rows);self.phase=6;return Ok(RetainedDiffPreflightAdvance::Ready(receipt(progress),!self.refused));},
   _=>return Err(invalid()),
  }Ok(RetainedDiffPreflightAdvance::Progress(p))
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{self.messages.take().unwrap_or_default()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,Edit>,_:RetainedCloneRef<'_,Snapshot>,_:&Snapshot,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=6||self.refused{return Err(invalid());}Ok(match self.part{0 if d.get().set_a.is_some()=>nested(self.a.exchange_demand()?),1 if d.get().set_b.is_some()=>nested(self.b.exchange_demand()?),_=>scalar()})}
 fn commit(&mut self,d:RetainedCloneRef<'_,Edit>,s:RetainedCloneRef<'_,Snapshot>,post:&mut Snapshot,g:RetainedCloneGrant,c:ApplyCapability)->Result<RetainedCloneStep,ValueError>{
  let demand=self.commit_demand(d,s,post,g.maximum_copy_bytes)?;if !protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let p=match self.part{0 if d.get().set_a.is_some()=>receipt(self.a.install(&mut post.a,c,child(g))?),1 if d.get().set_b.is_some()=>receipt(self.b.install(&mut post.b,c,child(g))?),_=>RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}};self.part+=1;Ok(if self.part==2{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn begin_close(&mut self){self.closing=true;self.a.begin_close();self.b.begin_close();self.diagnostic.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.a.terminal_is_empty(){return self.a.close_demand(body);}if !self.b.terminal_is_empty(){return self.b.close_demand(body);}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body);}self.retirement.next_owner_demands::<Vec<MutationMessage>>(self.messages.is_some(),&None,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.a.terminal_is_empty(){return self.a.close_step(g).map(|step|RetainedCloneStep::Progress(step.progress()));}if !self.b.terminal_is_empty(){return self.b.close_step(g).map(|step|RetainedCloneStep::Progress(step.progress()));}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_step(g).map(|step|RetainedCloneStep::Progress(step.progress()));}if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|step|RetainedCloneStep::Progress(step.progress()));}if let Some(step)=self.retirement.begin_granted(&mut self.messages,g)?{return Ok(step);}Ok(RetainedCloneStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{self.a.terminal_is_empty()&&self.b.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.messages.is_none()&&self.retirement.is_empty()}
}
impl Drop for EditApply{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original edit needs funded closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.messages);}}}}
#[derive(Clone,Default,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
struct Sequence{edits:Vec<Edit>}
impl DiffAlgebra<Snapshot> for Sequence{fn is_empty(&self)->bool{self.edits.iter().all(|edit|edit.set_a.is_none()&&edit.set_b.is_none()&&edit.require_a.is_none()&&!edit.warning)}}
impl MutationDiff<Snapshot> for Sequence{}
struct SequenceApply(RetainedDiffSequence<Snapshot,Edit>);
impl RetainedDiff<Snapshot> for Sequence{type Apply=SequenceApply;}
impl RetainedDiffApply<Snapshot,Sequence> for SequenceApply{
 fn new()->Self{Self(RetainedDiffSequence::new())}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize)->Result<RetirementDemand,ValueError>{self.0.preflight_demand(d.project(0,|d|&d.edits),s,post,body)}
 fn preflight(&mut self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize,g:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RetainedDiffPreflightAdvance,ValueError>{self.0.preflight(d.project(0,|d|&d.edits),s,post,body,g,&mut *receiving)}
 fn take_messages(&mut self)->Vec<MutationMessage>{self.0.take_messages()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&Snapshot,body:usize)->Result<RetirementDemand,ValueError>{self.0.commit_demand(d.project(0,|d|&d.edits),s,post,body)}
 fn commit(&mut self,d:RetainedCloneRef<'_,Sequence>,s:RetainedCloneRef<'_,Snapshot>,post:&mut Snapshot,g:RetainedCloneGrant,c:ApplyCapability)->Result<RetainedCloneStep,ValueError>{self.0.commit(d.project(0,|d|&d.edits),s,post,g,c)}
 fn begin_close(&mut self){self.0.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.0.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn denied(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut grants=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{grants.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes>0{grants.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes>0{grants.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}if d.depth>0{grants.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g});}grants}
fn exact<T>(run:impl FnOnce()->T,p:impl FnOnce(&T)->RetainedCloneProgress)->T{let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);let p=p(&value);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(!heap.overflowed);value}
fn source<T:RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{let d=RetainedCloneSource::<T>::owned_constructor_demand::<()>();RetainedCloneSource::admit_owned(value,(),grant(RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})).map_err(|(e,..)|e).unwrap().0}
fn close_source<T:RetireOwned+Sync>(source:&mut RetainedCloneSource<T>){while !source.terminal_is_empty(){let d=source.next_close_demand(4096).unwrap();exact(||source.close_step(grant(d)).unwrap(),|step|step.progress());}}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();while !owner.terminal_is_empty(){let d=owner.next_demand(4096).unwrap();exact(||owner.step(grant(d)).unwrap(),|step|step.progress());}}
fn decode<T:FromValue>(value:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn run(row:&serde_json::Value,base:&serde_json::Value,body:usize,cut:Option<usize>,commit_cut:Option<usize>){let _projections=PROJECTION_GATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let mut receiving_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1<<24,&mut receiving_callback);
 PROJECTIONS.store(0,Ordering::Relaxed);let mut original=source(decode::<Snapshot>(base));let mut post=decode::<Snapshot>(base);let pointer=post.text.as_ptr();let mut diff=source(Sequence{edits:decode(&row["edits"])});let(mut owner,_)=exact(||protocol::begin_retained_diff::<Snapshot,Sequence>(grant(protocol::retained_diff_birth_demand::<Snapshot,Sequence>())).unwrap(),|value|value.1);let mut turns=0;let mut settled=None;
 while cut.is_none_or(|cut|turns<cut){
  let d=owner.preflight_demand(diff.borrow(),original.borrow(),&post,body).unwrap();for denied in denied(grant(d),d){let step=exact(||protocol::preflight_retained_diff(&mut owner,diff.borrow(),original.borrow(),&post,body,denied,&mut receiving).unwrap(),|step|step.progress());assert_eq!(step.progress(),Default::default());assert_eq!(owner.preflight_demand(diff.borrow(),original.borrow(),&post,body).unwrap(),d);}
  let step=exact(||protocol::preflight_retained_diff(&mut owner,diff.borrow(),original.borrow(),&post,body,grant(d),&mut receiving).unwrap(),|step|step.progress());assert_eq!(&post,original.borrow().get());assert_eq!(post.text.as_ptr(),pointer);turns+=1;
  match step{RetainedDiffPreflightStep::Admitted(_,rows)=>{assert_eq!(rows.len(),row["messageCount"].as_u64().unwrap()as usize);drain(rows);settled=Some(true);break;},RetainedDiffPreflightStep::Refused(_,rows)=>{assert_eq!(rows.len(),row["messageCount"].as_u64().unwrap()as usize);drain(rows);settled=Some(false);break;},_=>{}}
 }
 if cut.is_none(){assert_eq!(settled,Some(row["accepted"].as_bool().unwrap()));if settled==Some(true){let mut commit_turns=0;let mut complete=false;while commit_cut.is_none_or(|cut|commit_turns<cut){let d=owner.commit_demand(diff.borrow(),original.borrow(),&post,body).unwrap();for denied in denied(grant(d),d){let step=exact(||protocol::commit_retained_diff(&mut owner,diff.borrow(),original.borrow(),&mut post,denied).unwrap(),|step|step.progress());assert_eq!(step.progress(),Default::default());}let step=exact(||protocol::commit_retained_diff(&mut owner,diff.borrow(),original.borrow(),&mut post,grant(d)).unwrap(),|step|step.progress());commit_turns+=1;if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break;}}if complete{assert_eq!(post,decode::<Snapshot>(&row["expected"]));}if commit_cut.is_none(){assert!(complete);}}else{assert_eq!(&post,original.borrow().get());}if diff.borrow().get().edits.len()<=1{assert_eq!(PROJECTIONS.load(Ordering::Relaxed),0);assert_eq!(post.text.as_ptr(),pointer);}}
 owner.begin_close();while !owner.terminal_is_empty(){let d=owner.close_demand(body).unwrap();for denied in denied(grant(d),d){let step=exact(||owner.close_step(denied).unwrap(),|step|step.progress());assert_eq!(step.progress(),Default::default());}exact(||owner.close_step(grant(d)).unwrap(),|step|step.progress());}
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));close_source(&mut diff);close_source(&mut original);drain(post);
}
#[test]
fn retained_diff_sequence_matches_original_intermediate_sqlite_values_and_refusal(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){for body in[1,4096]{run(row,&law["base"],body,None,None);}}
}
#[test]
fn retained_diff_sequence_cancellation_closes_original_sources_diagnostics_and_candidates(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){run(row,&law["base"],1,Some(cut.as_u64().unwrap()as usize),None);if row["accepted"]==true{run(row,&law["base"],1,None,Some(cut.as_u64().unwrap()as usize));}}}
}
#[test]
fn retained_diff_sequence_wide_original_projection_and_every_custody_cut_are_genuine(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let text=law["wideText"]["text"].as_str().unwrap().repeat(law["wideText"]["repeat"].as_u64().unwrap()as usize);let mut base=law["base"].clone();base["text"]=text.clone().into();
 for index in[1,2,3]{let mut row=law["cases"][index].clone();if row["accepted"]==true{row["expected"]["text"]=text.clone().into();}for body in[1,4096]{run(&row,&base,body,None,None);}for cut in[0,1,7,19,71,1025]{run(&row,&base,1,Some(cut),None);}if row["accepted"]==true{for cut in[0,1,7]{run(&row,&base,1,None,Some(cut));}}}
}
