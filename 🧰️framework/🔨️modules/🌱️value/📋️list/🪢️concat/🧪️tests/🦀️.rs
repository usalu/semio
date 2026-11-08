//! 🧪️ Real original vectors retain payload pointers under their single finite Source bank.
use super::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementFailure,retirement::controlled::ControlledRetirement};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_test_fixtures::fixture_text;
#[derive(crate::RetireOwned)]#[value(crate="crate")]
struct Row{ordinal:usize,text:String}
impl Clone for Row{fn clone(&self)->Self{panic!("original concatenation must never clone payload rows")}}
fn law()->serde_json::Value{serde_json::from_str(include_str!("🎟️work/🔣️.json")).unwrap()}
fn grant(bank:&EffectBank,d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096,"native copy quote exceeds the original body");let g=bank.loan(1,d.copy_bytes);let g=RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..g};assert!(retained_clone_grant_admits(g,d));g}
fn denied(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut out=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{out.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g})}if d.capacity_bytes>0{out.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g})}if d.release_bytes>0{out.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g})}if d.depth>0{out.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g})}out}
fn paid<'a,'b,P>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<P>,context:&mut P::Context<'a,'b>,g:RetainedCloneGrant,units:u64)->RetainedCloneStep where 'b:'a,P:OriginalEffectProducer<Output=RetainedCloneStep,Failure=OriginalRetirementFailure>{
 let(mut action,mut output)=(None,None);let mut heap=(0usize,0usize);let mut complete=false;
 for _ in 0..law()["stock"]["maximumTurns"].as_u64().unwrap(){let((poll,actual),observed)=crate::observe_retirement_allocations(||bank.run(g,units,|effect|{let before=effect.performed_boundary();let ready=authority.drive(context,effect,&mut action).unwrap();let work=effect.performed_since(before).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));heap.0+=observed.0;heap.1+=observed.1;if let Some(actual)=actual{assert_eq!(actual,heap.0)}if matches!(poll,RetainedWorkPoll::Pending{..}){continue}let p=match action.as_ref().unwrap().recorded_reply().unwrap(){Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert!(p.fits(g));assert_ne!(bank.take_admitted(),Some(false));bank.post(p,g,heap);complete=true;break}
 assert!(complete,"original concat action exceeded its preserved bound");
 let d=OriginalEffectAuthority::<P>::transfer_reply_demand().unwrap();let g=grant(bank,d);let((poll,actual),heap)=crate::observe_retirement_allocations(||bank.run(g,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(context,&mut action,&mut output,effect))));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native reply transfer pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap().unwrap(),g,heap);
 let result=output.as_ref().unwrap().as_ref().map(|step|*step).unwrap();let d=RetirementDemand{copy_bytes:size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()};let g=grant(bank,d);let((poll,actual),heap)=crate::observe_retirement_allocations(||bank.run(g,1,|effect|{assert!(effect.performed(1).unwrap());drop(output.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:1})}));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native reply retirement pending")};assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.unwrap(),g,heap);result
}
struct Driver<T:RetireOwned>(std::marker::PhantomData<T>);
struct Context<'a,T:RetireOwned>{owner:&'a mut OriginalVectorConcat<T>,input:&'a mut Option<Vec<T>>,output:&'a mut Option<Vec<T>>,source:OriginalSourceTurnIdentity,op:u8}
impl<T:RetireOwned> OriginalEffectProducer for Driver<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context<'a,T>where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{
  match c.op{2=>return c.owner.advance_into(4096,e,r),5=>return c.owner.close_into(4096,e,r),_=>{}}
  let mut result=None;let reply=OriginalEffectReply::borrow(&mut result)?;match c.op{0=>{c.owner.push_into(c.input,e,reply)?},1=>{c.owner.finish_into(e,reply)?},3=>{c.owner.take_into(c.output,e,reply)?},4=>{c.owner.begin_close_into(e,reply)?},_=>return Err(ValueError::invariant("unknown concat operation"))};Ok(r.complete(result.expect("closed native concat leaf reply").map_err(Into::into)))
 }
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{if c.op==5{c.owner.recover_close_into(4096,e,r)}else{Err(ValueError::invariant("closed native concat leaf has no pending callback"))}}
}
fn turn<T:RetireOwned>(bank:&mut EffectBank,authority:&mut OriginalEffectAuthority<Driver<T>>,owner:&mut OriginalVectorConcat<T>,input:&mut Option<Vec<T>>,output:&mut Option<Vec<T>>,op:u8,g:RetainedCloneGrant,work:u64)->RetainedCloneStep{let source=bank.source_witness().identity();paid(bank,authority,&mut Context{owner,input,output,source,op},g,work)}
struct RetireDriver<T:RetireOwned>(std::marker::PhantomData<T>);
impl<T:RetireOwned> OriginalEffectProducer for RetireDriver<T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=(&'a mut ControlledRetirement<T>,OriginalSourceTurnIdentity)where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.1)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.0.step_into(4096,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{c.0.recover_step_into(4096,e,r)}
}
fn drain<T:RetireOwned>(value:T,bank:&mut EffectBank){let mut owner=ControlledRetirement::retain(value);let mut authority=OriginalEffectAuthority::new(RetireDriver::<T>(std::marker::PhantomData));for _ in 0..law()["stock"]["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}assert!(!owner.requires_original_birth(),"original concat fixture needs its actual original birth loan");let d=owner.next_demand(4096).unwrap();let units=ErasedSnapshotRetirement::next_computational_work(&owner,4096).unwrap();let g=grant(bank,d);let source=bank.source_witness().identity();paid(bank,&mut authority,&mut(&mut owner,source),g,units);}assert!(owner.terminal_is_empty());assert!(authority.into_producer().is_ok());}
fn check(chunks:Vec<Vec<String>>,expected:Vec<String>,cut:Option<usize>){
 let law=law();let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let mut ordinal=0;let mut inputs:Vec<Option<Vec<Row>>>=chunks.into_iter().map(|rows|Some(rows.into_iter().map(|text|{let row=Row{ordinal,text};ordinal+=1;row}).collect())).collect();let pointers:Vec<*const u8>=inputs.iter().flat_map(|rows|rows.as_ref().unwrap().iter().map(|row|row.text.as_ptr())).collect();
 let mut owner=OriginalVectorConcat::<Row>::new();let mut authority=OriginalEffectAuthority::new(Driver::<Row>(std::marker::PhantomData));let mut output=None;let mut turns=0;let mut stopped=false;
 for input in &mut inputs{while cut.is_none_or(|cut|turns<cut){let d=owner.push_demand().unwrap();let g=grant(&bank,d);let units=owner.push_work_demand().unwrap();for no in denied(g,d){assert_eq!(turn(&mut bank,&mut authority,&mut owner,input,&mut output,0,no,units).progress(),Default::default());assert_eq!(owner.push_demand().unwrap(),d)}assert_eq!(turn(&mut bank,&mut authority,&mut owner,input,&mut output,0,g,0).progress(),Default::default());let step=turn(&mut bank,&mut authority,&mut owner,input,&mut output,0,g,units);turns+=1;if matches!(step,RetainedCloneStep::Complete(_)){assert!(input.is_none());break}}if cut.is_some_and(|cut|turns==cut){stopped=true;break}}
 let mut input=None;
 if !stopped{let d=owner.finish_demand().unwrap();let g=grant(&bank,d);for no in denied(g,d){assert_eq!(turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,1,no,1).progress(),Default::default())}turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,1,g,1);turns+=1;while !owner.ready()&&cut.is_none_or(|cut|turns<cut){let d=owner.demand(4096).unwrap();let g=grant(&bank,d);let units=owner.advance_work_demand(4096).unwrap();for no in denied(g,d){assert_eq!(turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,2,no,units).progress(),Default::default());assert_eq!(owner.demand(4096).unwrap(),d)}turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,2,g,units);turns+=1;assert!(turns<=law["stock"]["maximumTurns"].as_u64().unwrap()as usize)}}
 if cut.is_none(){assert!(owner.ready());let d=owner.take_demand().unwrap();let g=grant(&bank,d);for no in denied(g,d){assert_eq!(turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,3,no,1).progress(),Default::default());assert!(output.is_none())}turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,3,g,1);let output=output.take().unwrap();assert_eq!(output.iter().map(|row|row.text.as_str()).collect::<Vec<_>>(),expected.iter().map(String::as_str).collect::<Vec<_>>());for(row,pointer)in output.iter().zip(&pointers){assert_eq!(row.text.as_ptr(),*pointer)}drain(output,&mut bank)}
 let g=grant(&bank,owner.begin_close_demand());turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,4,g,1);
 for _ in 0..law["stock"]["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}assert!(!owner.requires_original_birth(),"concat child requires actual original birth loan");let d=owner.close_demand(4096).unwrap();let g=grant(&bank,d);let units=owner.close_work_demand(4096).unwrap();for no in denied(g,d){assert_eq!(turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,5,no,units).progress(),Default::default());assert_eq!(owner.close_demand(4096).unwrap(),d)}turn(&mut bank,&mut authority,&mut owner,&mut input,&mut output,5,g,units);}
 assert!(owner.terminal_is_empty());assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));assert!(authority.into_producer().is_ok());for input in inputs.into_iter().flatten(){for row in &input{assert_eq!(row.text.as_ptr(),pointers[row.ordinal])}drain(input,&mut bank)}assert!(bank.is_idle());assert!(bank.into_wallet().is_ok());
}
#[test]
fn original_vector_concat_preserves_original_rows_and_cancelled_custody(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪢️concat/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize))){
  let chunks=law["chunks"].as_array().unwrap().iter().map(|rows|rows.as_array().unwrap().iter().map(|row|row.as_str().unwrap().to_string()).collect()).collect();let expected=law["expected"].as_array().unwrap().iter().map(|row|row.as_str().unwrap().to_string()).collect();check(chunks,expected,cut);
 }
}
#[test]
fn original_vector_concat_wide_payload_and_chunk_directory_remain_bounded(){
 for cut in[Some(0),Some(1),Some(7),Some(19),Some(71),None]{let chunks=(0..257).map(|index|if index==128{vec!["雪".repeat(21846)]}else{vec![index.to_string()]}).collect::<Vec<_>>();let expected=(0..257).map(|index|if index==128{"雪".repeat(21846)}else{index.to_string()}).collect();check(chunks,expected,cut);}
}

#[test]
fn original_vector_concat_phases_actual_storage_and_preserves_serde_flattened_identity(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🎟️work/🔣️.json")).unwrap();
 let chunks:Vec<Vec<String>>=serde_json::from_value(law["chunks"].clone()).unwrap();
 let expected:Vec<String>=serde_json::from_value(law["expected"].clone()).unwrap();
 assert_eq!(serde_json::to_value(chunks.iter().flatten().collect::<Vec<_>>()).unwrap(),law["expected"]);
 check(chunks,expected,None);
}
