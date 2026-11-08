//! 🧪️ The original sparse vectors use one finite, original decoder Source.
use super::*;

fn quote<T:RetireOwned+Sync>(actor:&IssueProducer<T>,op:IssueOp)->(RetirementDemand,u64){
 match op{
  IssueOp::Birth|IssueOp::BirthShort(_)=>(actor.birth.advance_demand(4096,4096).unwrap(),actor.birth.advance_work_demand(4096,4096).unwrap()),
  IssueOp::Take=>(actor.birth.take_demand().unwrap(),1),
  IssueOp::Authority=>(ReadAuthority::<T>::admission_demand(),1),
  IssueOp::AuthorityClone(_)|IssueOp::AuthorityCloneShort(_)=>{let authority=actor.authority.as_ref().unwrap();(authority.clone_demand().unwrap(),authority.clone_work_demand().unwrap())},
  IssueOp::AuthorityFetch(_)=>(RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<ReadAuthority<T>>>(),depth:1,..Default::default()},1),
  IssueOp::Pick(_)=>(RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<T>>(),depth:1,..Default::default()},1),
  IssueOp::Root=>(SharedOriginalStorage::<T>::source_admission_demand().unwrap(),SharedOriginalStorage::<T>::source_admission_work_demand()),
  IssueOp::IssueBirth|IssueOp::IssueBirthShort=>(ReadLeaseIssue::<T>::admission_demand(),1),
  IssueOp::Issue=>{let issue=actor.issue.as_ref().unwrap();(issue.demand().unwrap(),issue.work_demand().unwrap())},
  IssueOp::IssueReturn=>(actor.issue.as_ref().unwrap().return_demand().unwrap(),1),
  IssueOp::IssueClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ReadLeaseIssue<T>>>(),depth:1,..Default::default()},1),
  IssueOp::Stash(_)|IssueOp::Fetch(_)=>(RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<ReadLease<T>>>(),depth:1,..Default::default()},1),
  IssueOp::CursorSet(_)=>(RetirementDemand{copy_bytes:std::mem::size_of::<usize>(),depth:1,..Default::default()},2),
  IssueOp::Visit=>(ReadAuthority::<T>::visit_demand(),ReadAuthority::<T>::visit_work_demand()),
  IssueOp::ReturnedTake=>(actor.visit.as_ref().unwrap().take_demand().unwrap(),1),
  IssueOp::VisitClear=>(ReadVisit::<T>::finish_demand(),1),
  IssueOp::LeaseShort|IssueOp::Lease|IssueOp::LeaseOriginal=>{let read=actor.lease.as_ref().unwrap();(read.next_demand(4096).unwrap(),read.next_computational_work(4096).unwrap())},
  IssueOp::LeaseClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ReadLease<T>>>(),depth:1,..Default::default()},1),
  IssueOp::RootCloseBirth=>(semio_framework_value_retirement::retirement::arc_custody::shared_retirement_birth_demand::<T>(),1),
  IssueOp::RootClose|IssueOp::RootCloseOriginal=>{let owner=actor.root_close.as_ref().unwrap();(owner.next_demand(4096).unwrap(),owner.next_computational_work(4096).unwrap())},
  IssueOp::RootCloseClear=>{let owner=actor.root_close.as_ref().unwrap();let witness=owner.native_header_release().unwrap();(witness.demand(owner.as_ref()).unwrap(),witness.work().unwrap())},
  IssueOp::AuthorityClose|IssueOp::AuthorityOriginal=>{let owner=actor.authority.as_ref().unwrap();(owner.next_demand(4096).unwrap(),owner.next_computational_work(4096).unwrap())},
  IssueOp::Erase|IssueOp::EraseShort(_)=>(ErasedReadLease::admission_demand::<T>(),1),
  IssueOp::ErasedClose|IssueOp::ErasedOriginal=>{let read=actor.erased.as_ref().unwrap();(ErasedSnapshotRetirement::next_demand(read,4096).unwrap(),ErasedSnapshotRetirement::next_computational_work(read,4096).unwrap())},
  IssueOp::ErasedClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ErasedReadLease>>(),depth:1,..Default::default()},1),
  IssueOp::SourceWrong|IssueOp::WrongClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()},1),
  IssueOp::SourceCapture=>(OriginalErasedReadSource::<T>::capture_demand(),1),
  IssueOp::SourceReturn|IssueOp::SourceReturnShort(_)=>(OriginalErasedReadSource::<T>::return_demand(),1),
  IssueOp::SourceClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<OriginalErasedReadSource<T>>>(),depth:1,..Default::default()},1),
  IssueOp::Publish(_,_)=>(ReadAuthority::<T>::publication_demand(),ReadAuthority::<T>::publication_work_demand()),
  IssueOp::TypedCapture=>(OriginalReadSource::<T>::capture_demand(),1),
  IssueOp::TypedReturn|IssueOp::TypedReturnShort(_)=>(OriginalReadSource::<T>::return_demand(),1),
  IssueOp::TypedClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<OriginalReadSource<T>>>(),depth:1,..Default::default()},1),
  IssueOp::TypedOwnerCapture=>(semio_framework_value_read::retained_clone::OriginalSourceOwner::<T,OriginalReadSource<T>>::capture_demand(),1),
  IssueOp::TypedOwnerProbe=>{let mode=semio_framework_value_read::retained_clone::OriginalSourceMode::Original;let mut demand=actor.typed_owner.demand(mode,4096).unwrap().unwrap();demand.copy_bytes+=std::mem::size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>();(demand,actor.typed_owner.work_demand(mode,4096).unwrap()+1)},
  IssueOp::ProbeClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()},1),
  IssueOp::TypedOwnerOriginal|IssueOp::TypedOwnerOriginalShort(_)|IssueOp::TypedOwnerDrive=>{let mode=semio_framework_value_read::retained_clone::OriginalSourceMode::Original;(actor.typed_owner.demand(mode,4096).unwrap().unwrap(),actor.typed_owner.work_demand(mode,4096).unwrap())},
  IssueOp::TypedOwnerUnique|IssueOp::TypedOwnerUniqueOriginal|IssueOp::TypedOwnerDriveUnique=>{let mode=semio_framework_value_read::retained_clone::OriginalSourceMode::Unique;(actor.typed_owner.demand(mode,4096).unwrap().unwrap(),actor.typed_owner.work_demand(mode,4096).unwrap())},
  IssueOp::TypedOwnerReturn=>(actor.typed_owner.return_demand(),1),
  IssueOp::OwnerCapture=>(semio_framework_value_read::retained_clone::OriginalSourceOwner::<T,OriginalErasedReadSource<T>>::capture_demand(),1),
  IssueOp::OwnerOriginal|IssueOp::OwnerOriginalShort(_)|IssueOp::OwnerDrive=>{let mode=semio_framework_value_read::retained_clone::OriginalSourceMode::Original;(actor.owner.demand(mode,4096).unwrap().unwrap(),actor.owner.work_demand(mode,4096).unwrap())},
  IssueOp::OwnerUnique|IssueOp::OwnerUniqueOriginal|IssueOp::OwnerDriveUnique=>{let mode=semio_framework_value_read::retained_clone::OriginalSourceMode::Unique;(actor.owner.demand(mode,4096).unwrap().unwrap(),actor.owner.work_demand(mode,4096).unwrap())},
  IssueOp::OwnerReturn=>(actor.owner.return_demand(),1),
  IssueOp::AuthorityClear=>(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ReadAuthority<T>>>(),depth:1,..Default::default()},1),
  _=>panic!("unsupported sparse fixture action"),
 }
}

pub(crate) fn turn<T:RetireOwned+Sync>(bank:&mut source_bank::Bank,native:&mut NativeDecodeControl<'_>,frame:&mut NativeReceivingEffect<OriginalRetirementFailure>,actor:&mut OriginalEffectAuthority<IssueProducer<T>>,op:IssueOp)->RetainedCloneStep{turn_with_copy(bank,native,frame,actor,op,None)}
pub(crate) fn turn_with_copy<T:RetireOwned+Sync>(bank:&mut source_bank::Bank,native:&mut NativeDecodeControl<'_>,frame:&mut NativeReceivingEffect<OriginalRetirementFailure>,actor:&mut OriginalEffectAuthority<IssueProducer<T>>,op:IssueOp,offered:Option<usize>)->RetainedCloneStep{
 let(mut d,work)=actor.inspect(|owner|quote(owner,op));assert!(d.copy_bytes<=4096);if let Some(copy)=offered{assert!(copy<d.copy_bytes);d.copy_bytes=copy}let g=grant(d);let mut heap=(0,0);
 for _ in 0..65536{
  if actor.inspect(IssueProducer::selected_birth){let poll=bank.run_with_turn(native,|source,control|control.with_retirement(g,|funding|funding.with_original_close(|original|{let mut c=IssueContext{turn:source,op,body:4096,original:None};actor.drive_original_birth(&mut c,original,g)}))).unwrap();if let RetainedWorkPoll::Complete(done)=poll{done.result.unwrap();}continue}
  let original=matches!(op,IssueOp::LeaseOriginal|IssueOp::ErasedOriginal|IssueOp::OwnerUniqueOriginal|IssueOp::TypedOwnerUniqueOriginal|IssueOp::RootCloseOriginal|IssueOp::AuthorityOriginal);
  let(poll,observed)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run_with_turn(native,|source,control|control.receive_original_effect(d,work,frame,actor,|native,effect,actor,action|if original{native.with_retirement(g,|funding|funding.with_original_close(|loan|{let mut c=IssueContext{turn:source,op,body:4096,original:Some(loan)};actor.drive(&mut c,effect,action)}))}else{let mut c=IssueContext{turn:source,op,body:4096,original:None};actor.drive(&mut c,effect,action)})).unwrap());heap.0+=observed.0;heap.1+=observed.1;
  if let NativeReceivingEffectPoll::Complete(done)=poll{let step=done.result.unwrap();assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if offered.is_some()||matches!(op,IssueOp::BirthShort(_)|IssueOp::IssueBirthShort|IssueOp::LeaseShort|IssueOp::AuthorityCloneShort(_)|IssueOp::EraseShort(_)|IssueOp::OwnerOriginalShort(_)|IssueOp::SourceReturnShort(_)|IssueOp::TypedReturnShort(_)|IssueOp::TypedOwnerOriginalShort(_)){assert_eq!(done.work,0);assert_eq!(step.progress(),Default::default())}return step}
 }
 panic!("sparse original action did not settle")
}

fn close_lease(bank:&mut source_bank::Bank,native:&mut NativeDecodeControl<'_>,frame:&mut NativeReceivingEffect<OriginalRetirementFailure>,actor:&mut OriginalEffectAuthority<IssueProducer<u64>>,index:usize){
 turn(bank,native,frame,actor,IssueOp::Fetch(index));
 for _ in 0..65536{if actor.inspect(|p|p.lease.as_ref().unwrap().terminal_is_empty()){break}let original=actor.inspect(|p|p.lease.as_ref().unwrap().requires_original_control());turn(bank,native,frame,actor,if original{IssueOp::LeaseOriginal}else{IssueOp::Lease});}
 assert!(actor.inspect(|p|p.lease.as_ref().unwrap().terminal_is_empty()));turn(bank,native,frame,actor,IssueOp::LeaseClear);
}

fn finish_visit(bank:&mut source_bank::Bank,native:&mut NativeDecodeControl<'_>,frame:&mut NativeReceivingEffect<OriginalRetirementFailure>,actor:&mut OriginalEffectAuthority<IssueProducer<u64>>)->Option<u64>{
 turn(bank,native,frame,actor,IssueOp::Visit);let value=actor.inspect(|p|p.visit.as_ref().unwrap().root().copied());
 if value.is_some(){turn(bank,native,frame,actor,IssueOp::ReturnedTake);}turn(bank,native,frame,actor,IssueOp::VisitClear);
 if value.is_some(){turn(bank,native,frame,actor,IssueOp::RootCloseBirth);for _ in 0..65536{if actor.inspect(|p|p.root_close.as_ref().unwrap().terminal_is_empty()){break}let original=actor.inspect(|p|p.root_close.as_ref().unwrap().requires_original_control());turn(bank,native,frame,actor,if original{IssueOp::RootCloseOriginal}else{IssueOp::RootClose});}assert!(actor.inspect(|p|p.root_close.as_ref().unwrap().terminal_is_empty()));turn(bank,native,frame,actor,IssueOp::RootCloseClear);}
 value
}

pub(crate) fn run_sparse(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).unwrap();assert_eq!(schema["const"],fixture);
 let stock:serde_json::Value=serde_json::from_str(include_str!("../../🎟️work/case.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let issued=row["issued"].as_u64().unwrap()as usize;let mut bank=source_bank::Bank::from_stock(&stock["stock"],false);let mut recipient=NativeDecodeRetirementRecipient::new();let mut observer=|_|true;let mut native=NativeDecodeControl::new(stock["stock"]["capacityBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut inputs=std::array::from_fn(|_|None);for(index,input)in inputs.iter_mut().enumerate().take(issued){*input=Some(index as u64)}
  let mut actor=OriginalEffectAuthority::new(IssueProducer{birth:ManuallyDrop::new(ReadRegistryBirth::new()),issued:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),aliases:ManuallyDrop::new(std::array::from_fn(|_|None)),value:None,inputs:ManuallyDrop::new(inputs),stored:ManuallyDrop::new(std::array::from_fn(|_|None)),root:ManuallyDrop::new(None),issue:ManuallyDrop::new(None),lease:ManuallyDrop::new(None),visit:ManuallyDrop::new(None),erased:ManuallyDrop::new(None),root_close:ManuallyDrop::new(None),typed_source:ManuallyDrop::new(None),typed_owner:ManuallyDrop::new(Default::default()),typed_action:None,typed_reply:ManuallyDrop::new(None),wrong_reply:ManuallyDrop::new(None),source:ManuallyDrop::new(None),owner:ManuallyDrop::new(Default::default()),owner_action:None,owner_reply:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),cursor_reply:ManuallyDrop::new(None),turn:bank.turn.identity()});let mut frame=NativeReceivingEffect::default();
  for _ in 0..65536{if actor.inspect(|p|p.birth.ready()){break}turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Birth);}assert!(actor.inspect(|p|p.birth.ready()));turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Take);turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Authority);
  for index in 0..issued{turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Pick(index));turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Root);turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::IssueBirth);for _ in 0..16{if actor.inspect(|p|p.lease.is_some()){break}turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Issue);}assert_eq!(actor.inspect(|p|p.lease.as_ref().unwrap().id().index),index as u16);turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::IssueReturn);turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::IssueClear);turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::Stash(index));}
  turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::CursorSet(row["cursor"].as_u64().unwrap()as usize));
  for index in row["returned"].as_array().unwrap(){close_lease(&mut bank,&mut native,&mut frame,&mut actor,index.as_u64().unwrap()as usize)}
  let expected:Vec<Option<u64>>=serde_json::from_value(row["visits"].clone()).unwrap();let actual:Vec<_>=expected.iter().map(|_|finish_visit(&mut bank,&mut native,&mut frame,&mut actor)).collect();assert_eq!(actual,expected,"{}",row["name"]);
  for index in 0..issued{if actor.inspect(|p|p.stored[index].is_some()){close_lease(&mut bank,&mut native,&mut frame,&mut actor,index)}}
  while actor.inspect(|p|p.authority.as_ref().unwrap().registry().unwrap().has_returned()){finish_visit(&mut bank,&mut native,&mut frame,&mut actor);}
  for _ in 0..65536{if actor.inspect(|p|p.authority.as_ref().unwrap().terminal_is_empty()){break}let original=actor.inspect(|p|p.authority.as_ref().unwrap().requires_original_control());turn(&mut bank,&mut native,&mut frame,&mut actor,if original{IssueOp::AuthorityOriginal}else{IssueOp::AuthorityClose});}
  assert!(actor.inspect(|p|p.authority.as_ref().unwrap().terminal_is_empty()));turn(&mut bank,&mut native,&mut frame,&mut actor,IssueOp::AuthorityClear);assert!(actor.inspect(IssueProducer::terminal));drop(actor.into_producer().ok().unwrap());
 }
}

