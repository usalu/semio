//! 🧺️ The native collection corpus clones and retires through one original finite Source bank.
use semio_framework_value_retained_clone::retained_clone::*;
use semio_framework_value_retained_clone::{RetainedAllocationEffect,RetainedWorkPoll,RetainedWorkOutcome,RetirementDemand,ValueError,OriginalEffectAuthority,OriginalEffectAction,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,OriginalRetirementFailure};
use semio_framework_value_retirement::{ErasedSnapshotRetirement,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_clone_source_testing::{admit_owned,FixtureSource,FixtureCloseStep};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]enum Op{Advance,Take,Start,Close,Retire,Drain,Clear}
struct Context<'a,M:NativeCollectionClone>{source:RetainedCloneRef<'a,M>,op:Op,body:usize,maximum:u64}
struct Producer<M:NativeCollectionClone>{cursor:ManuallyDrop<M::Cursor>,output:ManuallyDrop<Option<M>>,retirement:ManuallyDrop<Option<ControlledRetirement<M>>>,native:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,selected:Option<(semio_framework_value_retained_clone::OriginalEffectInvocationIdentity,Op,usize,u64)>}
impl<M:NativeCollectionClone> Producer<M>{fn new()->Self{Self{cursor:ManuallyDrop::new(M::retained_clone_cursor()),output:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),native:ManuallyDrop::new(None),selected:None}}}
impl<M:NativeCollectionClone> OriginalEffectProducer for Producer<M>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context<'a,M>where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{self.call(c,effect,reply,false)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{self.call(c,effect,reply,true)}
}
impl<M:NativeCollectionClone> Producer<M>{
 fn call<'r>(&mut self,c:&mut Context<'_,M>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  let key=(effect.invocation().unwrap().identity(),c.op,c.body,c.maximum);if self.selected.is_some_and(|saved|saved!=key){return Err(ValueError::invariant("collection law changed the original operation"))}
  if c.op==Op::Take{self.selected=Some(key);if self.native.is_none(){let native=OriginalEffectReply::borrow(&mut self.native)?;if recover{self.cursor.recover_take(&mut self.output,effect,native)?;}else{self.cursor.take(&mut self.output,effect,native)?;}}let mut scope=reply.scope(&mut self.selected,key)?;forward_original_value_reply(&mut self.native,scope.borrow()?)?;return Ok(scope.forwarded())}
  let mut scope=reply.scope(&mut self.selected,key)?;let child=scope.borrow()?;match c.op{
   Op::Advance=>{let maximum=c.maximum;if recover{self.cursor.recover_advance_into(c.source,c.body,maximum,effect,child)?;}else{self.cursor.advance_into(c.source,c.body,maximum,effect,child)?;}},
   Op::Start=>{if recover{self.cursor.recover_begin_close_into(effect,child)?;}else{self.cursor.begin_close_into(effect,child)?;}},
   Op::Close=>{if recover{self.cursor.recover_close_into(c.body,effect,child)?;}else{self.cursor.close_into(c.body,effect,child)?;}},
   Op::Retire=>{assert!(effect.performed(1)?);let original=self.output.take().expect("original cloned output");*self.retirement=Some(ControlledRetirement::retain(original));child.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:retire_demand::<M>().copy_bytes,..Default::default()})));},
   Op::Drain=>{let owner=self.retirement.as_mut().unwrap();if recover{owner.recover_step_into(c.body,effect,child)?;}else{owner.step_into(c.body,effect,child)?;}},
   Op::Clear=>{let owner=self.retirement.as_ref().unwrap();let evidence=owner.native_header_release().expect("actual empty controlled header");evidence.demand(owner)?;assert!(effect.performed(1)?);self.retirement.take();child.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<ControlledRetirement<M>>>(),..Default::default()})));},
   Op::Take=>unreachable!()
  }Ok(scope.forwarded())
 }
}
impl<M:NativeCollectionClone> Drop for Producer<M>{fn drop(&mut self){let empty=self.cursor.terminal_is_empty()&&self.output.is_none()&&self.retirement.is_none()&&self.native.is_none()&&self.selected.is_none();assert!(empty||std::thread::panicking(),"native collection law retains original ownership");if empty{unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.retirement);ManuallyDrop::drop(&mut self.native)}}}}
fn retire_demand<M:NativeCollectionClone>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<M>>()+size_of::<Option<ControlledRetirement<M>>>(),depth:1,..Default::default()}}

fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drive<M:NativeCollectionClone>(source:&mut FixtureSource<M>,authority:&mut OriginalEffectAuthority<Producer<M>>,op:Op,demand:RetirementDemand,units:u64)->RetainedCloneProgress{drive_granted(source,authority,op,65536,units,exact(demand))}
fn drive_granted<M:NativeCollectionClone>(source:&mut FixtureSource<M>,authority:&mut OriginalEffectAuthority<Producer<M>>,op:Op,body:usize,units:u64,requested:RetainedCloneGrant)->RetainedCloneProgress{
 let mut action:Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>=None;let mut reply=None;let mut observed=(0,0);let mut work=0;
 source.with_bank(|original,bank|{let left=bank.loan(1,requested.maximum_copy_bytes);let grant=RetainedCloneGrant{maximum_items:left.maximum_items.min(requested.maximum_items),maximum_copy_bytes:left.maximum_copy_bytes.min(requested.maximum_copy_bytes),maximum_capacity_bytes:left.maximum_capacity_bytes.min(requested.maximum_capacity_bytes),maximum_release_bytes:left.maximum_release_bytes.min(requested.maximum_release_bytes),maximum_depth:left.maximum_depth.min(requested.maximum_depth)};for _ in 0..4096{let((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,units,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut Context{source:original,op,body,maximum:65536},effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;work+=poll.work();if let Some(actual)=actual{assert_eq!(actual,observed.0)}if matches!(poll,RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.finish(&mut action).unwrap();let step=reply.take().expect("original recorded collection reply").unwrap();let progress=step.progress();assert!(progress==Default::default()||work>0);assert_eq!(bank.take_admitted(),if work==0{None}else{Some(true)});bank.post(progress,grant,observed);return progress}}panic!("native collection original action did not settle")}).unwrap()
}

fn run<M:NativeCollectionClone>(original:M,bank:EffectBank,inspect:impl Fn(&M))->(usize,usize){run_until(original,bank,inspect,None)}
fn run_until<M:NativeCollectionClone>(original:M,bank:EffectBank,inspect:impl Fn(&M),stop:Option<usize>)->(usize,usize){
 let count=original.entry_count();let grant=bank.loan(1,65536);let mut bank=bank;let(source,_)=admit_owned(&mut bank,original,(),grant).result.unwrap_or_else(|_|panic!("actual original collection Source admission refused"));let mut source=FixtureSource::from_admitted(source,bank).unwrap_or_else(|_|panic!("actual same fixture bank"));let mut authority=OriginalEffectAuthority::new(Producer::<M>::new());let mut turns=0;let mut quoted=0;let mut copied=0;
 for _ in 0..4096{if authority.producer().cursor.output_ready()||stop==Some(turns){break}let(d,w)=source.with_bank(|r,_|{let c=&authority.producer().cursor;(c.advance_demands(r,65536,65536).unwrap(),c.advance_work_demand(r,65536,65536).unwrap())}).unwrap();assert!(w>0);
  if stop.is_none(){for denied in [RetainedCloneGrant{maximum_items:0,..exact(d)},RetainedCloneGrant{maximum_depth:0,..exact(d)}]{assert_eq!(drive_granted(&mut source,&mut authority,Op::Advance,65536,w,denied),Default::default());}
   if d.copy_bytes>0{let body=d.copy_bytes-1;let(selected,units)=source.with_bank(|r,_|{let c=&authority.producer().cursor;(c.advance_demands(r,body,65536).unwrap(),c.advance_work_demand(r,body,65536).unwrap())}).unwrap();let request=RetainedCloneGrant{maximum_copy_bytes:body,..exact(selected)};let progress=drive_granted(&mut source,&mut authority,Op::Advance,body,units,request);assert!(progress.fits(request));assert!(progress.copied_bytes<=selected.copy_bytes);if selected.copy_bytes>body{assert_eq!(progress,Default::default())}copied+=progress.copied_items;}
  }
  if authority.producer().cursor.output_ready(){break}let(d,w)=source.with_bank(|r,_|{let c=&authority.producer().cursor;(c.advance_demands(r,65536,65536).unwrap(),c.advance_work_demand(r,65536,65536).unwrap())}).unwrap();quoted+=1;copied+=drive(&mut source,&mut authority,Op::Advance,d,w).copied_items;turns+=1;
 }
 if stop.is_none(){assert!(authority.producer().cursor.output_ready());let d=authority.producer().cursor.take_demand().unwrap();let w=authority.producer().cursor.take_work_demand().unwrap();drive(&mut source,&mut authority,Op::Take,d,w);inspect(authority.producer().output.as_ref().unwrap());assert!(copied>=count);}
 let d=authority.producer().cursor.begin_close_demand().unwrap();let w=authority.producer().cursor.begin_close_work_demand().unwrap();drive(&mut source,&mut authority,Op::Start,d,w);
 for _ in 0..4096{if authority.producer().cursor.terminal_is_empty(){break}let c=&authority.producer().cursor;assert!(!c.requires_original_birth(),"collection law needs actual original Birth bridge");let d=c.close_demands(65536).unwrap();let w=c.close_work_demand(65536).unwrap();drive(&mut source,&mut authority,Op::Close,d,w);}assert!(authority.producer().cursor.terminal_is_empty());
 if authority.producer().output.is_some(){drive(&mut source,&mut authority,Op::Retire,retire_demand::<M>(),1);for _ in 0..4096{let owner=authority.producer().retirement.as_ref().unwrap();if owner.terminal_is_empty(){break}assert!(!owner.requires_original_birth(),"collection output needs actual original Birth bridge");let d=owner.next_demand(65536).unwrap();let w=owner.next_computational_work(65536).unwrap();drive(&mut source,&mut authority,Op::Drain,d,w);}assert!(authority.producer().retirement.as_ref().unwrap().terminal_is_empty());drive(&mut source,&mut authority,Op::Clear,RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<M>>>(),depth:1,..Default::default()},1);}assert!(authority.into_producer().is_ok());
 for _ in 0..4096{if source.terminal_is_empty(){break}match source.close_once().unwrap(){FixtureCloseStep::Blocked|FixtureCloseStep::Denied=>panic!("funded original collection Source close stalled"),_=>{}}}assert!(source.terminal_is_empty());(turns,quoted)
}
fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128},100000))}
fn map<K:NativeSortedKey,V>(input:Vec<(K,V)>,bank:&mut EffectBank)->NativeSortedMap<K,V>{let mut input=Some(input);let mut a=NativeMapAdmission::default();leaf(bank,a.capture_demand(),|e,r|{a.capture_from(&mut input,e,r).unwrap();});for _ in 0..4096{if a.is_validated(){break}leaf(bank,a.advance_demand().unwrap(),|e,r|{a.advance_into(e,r).unwrap();})}let mut output=None;leaf(bank,a.publication_demand(),|e,r|{a.publish_into(&mut output,e,r).unwrap();});output.unwrap()}
fn set<K:NativeSortedKey>(input:Vec<K>,bank:&mut EffectBank)->NativeSortedSet<K>{let mut input=Some(input);let mut a=NativeSetAdmission::default();leaf(bank,a.capture_demand(),|e,r|{a.capture_from(&mut input,e,r).unwrap();});for _ in 0..4096{if a.is_validated(){break}leaf(bank,a.advance_demand().unwrap(),|e,r|{a.advance_into(e,r).unwrap();})}let mut output=None;leaf(bank,a.publication_demand(),|e,r|{a.publish_into(&mut output,e,r).unwrap();});output.unwrap()}
fn leaf(bank:&mut EffectBank,d:RetirementDemand,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'_,RetainedCloneStep,ValueError>)){let grant=bank.loan(1,d.copy_bytes);let mut reply=None;let((poll,actual),heap)=observe_retirement_allocations(||bank.run(grant,1,|e|{let before=e.performed_work();operation(e,OriginalEffectReply::borrow(&mut reply).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:e.performed_work()-before})}));assert!(matches!(poll,RetainedWorkPoll::Complete(_)));assert_eq!(bank.take_admitted(),Some(true));let step=reply.unwrap().unwrap();assert_eq!(actual,None);bank.post(step.progress(),grant,heap)}
#[test]fn standard_collection_clones_equal_clone_and_serde_oracles_per_fixture_row(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(fixture["schema"],"framework.value.retained-clone.std-collections.v1");for row in fixture["cases"].as_array().unwrap(){let mut bank=bank();match row["kind"].as_str().unwrap(){
  "hash-map"=>{let expected:std::collections::HashMap<String,String>=serde_json::from_value(row["value"].clone()).unwrap();let mut entries=expected.iter().map(|(k,v)|(k.clone(),v.clone())).collect::<Vec<_>>();entries.sort_by(|a,b|a.0.cmp(&b.0));let original=map(entries,&mut bank);run(original,bank,|output|assert_eq!(output.entries().iter().cloned().collect::<std::collections::HashMap<_,_>>(),expected));},
  "btree-map"=>{let expected:std::collections::BTreeMap<String,Vec<String>>=serde_json::from_value(row["value"].clone()).unwrap();let original=map(expected.iter().map(|(k,v)|(k.clone(),v.clone())).collect(),&mut bank);run(original,bank,|output|assert_eq!(output.entries().iter().cloned().collect::<std::collections::BTreeMap<_,_>>(),expected));},
  "hash-set"=>{let expected:std::collections::HashSet<String>=serde_json::from_value(row["value"].clone()).unwrap();let mut entries=expected.iter().cloned().collect::<Vec<_>>();entries.sort();let original=set(entries,&mut bank);run(original,bank,|output|assert_eq!(output.entries().iter().cloned().collect::<std::collections::HashSet<_>>(),expected));},
  "btree-set"=>{let expected:std::collections::BTreeSet<u32>=serde_json::from_value(row["value"].clone()).unwrap();let original=set(expected.iter().copied().collect(),&mut bank);run(original,bank,|output|assert_eq!(output.entries().iter().copied().collect::<std::collections::BTreeSet<_>>(),expected));},
  _=>unreachable!()
 }}
}

#[test]fn standard_collection_clones_cancel_at_every_turn_without_abandoning_ownership(){
 let map_oracle:std::collections::HashMap<String,String>=(0..5).map(|index|(format!("k{index}"),format!("v{index}"))).collect();
 let tree_oracle:std::collections::BTreeMap<String,Vec<String>>=(0..5).map(|index|(format!("k{index}"),vec![format!("v{index}")])).collect();
 let set_oracle:std::collections::HashSet<String>=(0..5).map(|index|format!("k{index}")).collect();
 let ordered_oracle:std::collections::BTreeSet<u32>=(0..5).collect();
 for stop in 0..64{
  let mut b=bank();let mut rows=map_oracle.iter().map(|(k,v)|(k.clone(),v.clone())).collect::<Vec<_>>();rows.sort_by(|a,b|a.0.cmp(&b.0));let input=map(rows,&mut b);run_until(input,b,|_|{},Some(stop));
  let mut b=bank();let input=map(tree_oracle.iter().map(|(k,v)|(k.clone(),v.clone())).collect(),&mut b);run_until(input,b,|_|{},Some(stop));
  let mut b=bank();let mut rows=set_oracle.iter().cloned().collect::<Vec<_>>();rows.sort();let input=set(rows,&mut b);run_until(input,b,|_|{},Some(stop));
  let mut b=bank();let input=set(ordered_oracle.iter().copied().collect(),&mut b);run_until(input,b,|_|{},Some(stop));
 }
}
#[test]fn standard_collection_option_and_nesting_clone_through_the_entry_cursor(){
 let mut b=bank();let inner=set(vec![1u32,2],&mut b);let input=map(vec![("none".to_owned(),None),("some".to_owned(),Some(inner))],&mut b);let expected=serde_json::json!({"none":null,"some":[1,2]});run(input,b,|output|{let actual=output.entries().iter().map(|(key,value)|(key.clone(),value.as_ref().map(|set|set.entries().iter().copied().collect::<std::collections::BTreeSet<_>>()))).collect::<std::collections::BTreeMap<_,_>>();assert_eq!(serde_json::to_value(actual).unwrap(),expected);});
}
#[test]fn standard_collection_scalar_clones_are_fully_quoted_turn_by_turn(){
 let oracle:std::collections::HashMap<u32,u64>=(0..9).map(|index|(index,u64::from(index)*3)).collect();let mut entries=oracle.iter().map(|(k,v)|(*k,*v)).collect::<Vec<_>>();entries.sort_by_key(|e|e.0);let mut b=bank();let input=map(entries,&mut b);let(turns,quoted)=run(input,b,|output|{assert_eq!(output.entries().iter().copied().collect::<std::collections::HashMap<_,_>>(),oracle);});assert_eq!(turns,quoted,"scalar entry cursors quote every advance");
 let oracle:std::collections::BTreeSet<u32>=(0..9).collect();let mut b=bank();let input=set(oracle.iter().copied().collect(),&mut b);let(turns,quoted)=run(input,b,|output|assert_eq!(output.entries().iter().copied().collect::<std::collections::BTreeSet<_>>(),oracle));assert_eq!(turns,quoted);
}
