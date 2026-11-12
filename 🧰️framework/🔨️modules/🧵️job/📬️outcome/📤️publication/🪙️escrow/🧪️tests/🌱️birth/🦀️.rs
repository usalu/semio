//! 🧪️ The original publisher backing stays with its one finite Source throughout birth.
use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublisherBirth};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,OriginalEffectReply,ValueError};
use semio_framework_value_testing_allocator as _;
#[test]
fn bounded_birth_and_paid_release_use_the_same_source(){
 let law:serde_json::Value=serde_json::from_str(include_str!("case.json")).unwrap();let source_rows:serde_json::Value=serde_json::from_str(include_str!("../../../../📦️builder/🪙️return-escrow/🧪️tests/🌱️birth/case.json")).unwrap();
 let capacity=std::mem::size_of::<OriginalEscrowPublisher>();
 let stock=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:16384,maximum_capacity_bytes:capacity,maximum_release_bytes:capacity,maximum_depth:4};
 let mut bank=EffectBank::new(Wallet::new(stock,32));let mut foreign=EffectBank::new(Wallet::new(stock,32));let mut birth=OriginalEscrowPublisherBirth::new();let mut output=None;
 let mut copied=0usize;let turns=usize::try_from((law["headerBytes"].as_u64().unwrap()+law["rows"].as_u64().unwrap()*law["rowBytes"].as_u64().unwrap()+law["chunk"].as_u64().unwrap()-1)/law["chunk"].as_u64().unwrap()).unwrap()+3;
 for index in 0..turns{
  let d=birth.demand();assert!(d.copy_bytes<=law["maximumCopy"].as_u64().unwrap()as usize);
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:0,maximum_depth:d.depth};
  let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{let mut slot=None;let complete=birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());assert!(complete.is_recorded());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})}));
  let RetainedWorkPoll::Complete(turn)=turn else{panic!("publisher native step must record a reply")};let progress=turn.result.unwrap().progress();copied+=progress.copied_bytes;assert_eq!(turn.work,1);assert_eq!(marker,(progress.retained_capacity_bytes!=0).then_some(capacity));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,0));assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(heap.requested_bytes,0));
  if index==0{
   let d=birth.demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
   let (turn,marker)=foreign.run(grant,1,|effect|{let mut slot=None;birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})});
   let RetainedWorkPoll::Complete(turn)=turn else{panic!("foreign Source must record refusal")};assert!(turn.result.is_err());assert_eq!(turn.work>0,source_rows[2]["performed"].as_bool().unwrap());assert_eq!(marker,None);assert!(output.is_none());assert!(!birth.is_empty());assert!(foreign.is_idle());
   let denied=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let (turn,marker)=bank.run(denied,1,|effect|{let mut slot=None;birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})});
   let RetainedWorkPoll::Complete(turn)=turn else{panic!("denial must record its unchanged result")};assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(marker,None);assert!(output.is_none());assert!(!birth.is_empty());
  }
 }
 assert_eq!((copied,turns as u64),(OriginalEscrowPublisher::birth_demand().0,OriginalEscrowPublisher::birth_demand().2));assert!(birth.is_empty());assert!(output.as_ref().unwrap().terminal_is_empty());
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Option<Box<OriginalEscrowPublisher>>>(),maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1};
 let ((turn,_),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{assert!(effect.performed(1).unwrap());drop(output.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes,released_bytes:capacity,..Default::default()}),work:1})}));
 let RetainedWorkPoll::Complete(turn)=turn else{panic!("paid release must complete")};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(bank.take_admitted(),Some(true));bank.post(turn.result.unwrap(),grant,(0,capacity));assert!(bank.is_idle());
}

#[test]
fn partial_backing_is_returned_only_after_original_source_pays(){
 let capacity=std::mem::size_of::<OriginalEscrowPublisher>();let stock=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:4096,maximum_capacity_bytes:capacity,maximum_release_bytes:capacity,maximum_depth:4};
 let mut bank=EffectBank::new(Wallet::new(stock,8));let mut birth=OriginalEscrowPublisherBirth::new();let mut output=None;
 let d=birth.demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:0,maximum_depth:1};
 let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{let mut slot=None;birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})}));
 let RetainedWorkPoll::Complete(turn)=turn else{panic!("allocation must finish")};let progress=turn.result.unwrap().progress();assert_eq!(marker,Some(capacity));assert_eq!((heap.requested_bytes,heap.released_bytes),(capacity,0));assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(capacity,0));
 let d=birth.cancel_demand();let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:d.release_bytes-1,maximum_depth:1};
 let (turn,marker)=bank.run(denied,1,|effect|{let mut slot=None;birth.cancel_into(effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})});
 let RetainedWorkPoll::Complete(turn)=turn else{panic!("denial must finish")};assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(marker,None);assert!(!birth.is_empty());assert!(output.is_none());
 let grant=RetainedCloneGrant{maximum_release_bytes:d.release_bytes,..denied};let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{let mut slot=None;birth.cancel_into(effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})}));
 let RetainedWorkPoll::Complete(turn)=turn else{panic!("cancellation must finish")};let progress=turn.result.unwrap().progress();assert_eq!(turn.work,1);assert_eq!(marker,None);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(0,capacity));assert!(birth.is_empty());assert!(bank.is_idle());
}

use semio_framework_job_payload::{OriginalReturnEscrowTable,OriginalReturnEscrowBirth};
 #[test]
 fn resident_birth_rejects_foreign_source_and_returns_its_actual_backing(){
  let law:serde_json::Value=serde_json::from_str(include_str!("../../../../📦️builder/🪙️return-escrow/🧪️tests/🌱️birth/case.json")).unwrap();
  use semio_framework_value_testing_effect_bank::{EffectBank,Wallet,RetainedWorkPoll,OriginalEffectReply};
  let capacity=std::mem::size_of::<OriginalReturnEscrowTable>();let stock=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:4096,maximum_capacity_bytes:capacity,maximum_release_bytes:capacity,maximum_depth:4};
  let mut original=EffectBank::new(Wallet::new(stock,8));let mut foreign=EffectBank::new(Wallet::new(stock,8));let mut birth=OriginalReturnEscrowBirth::new();let mut output=None;
  let d=birth.demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:0,maximum_depth:1};
  let denied=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant};let (turn,marker)=original.run(denied,1,|effect|{let mut slot=None;birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})});let RetainedWorkPoll::Complete(turn)=turn else{panic!("allocation denial must finish")};assert_eq!(turn.work>0,law[0]["performed"].as_bool().unwrap());assert_eq!(marker,None);assert!(birth.is_empty());assert!(output.is_none());
  let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.run(grant,1,|effect|{let mut slot=None;let complete=birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());assert!(complete.is_recorded());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})}));
  let RetainedWorkPoll::Complete(turn)=turn else{panic!("allocation must record its reply")};let p=turn.result.unwrap().progress();assert_eq!(marker,Some(capacity));assert_eq!(heap.requested_bytes,capacity);assert_eq!(original.take_admitted(),Some(true));original.post(p,grant,(capacity,0));assert!(!birth.is_empty());
  let row=birth.demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:row.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
  let (turn,marker)=foreign.run(grant,1,|effect|{let mut slot=None;birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})});
  let RetainedWorkPoll::Complete(turn)=turn else{panic!("foreign refusal must be recorded")};assert!(turn.result.is_err());assert_eq!(turn.work,0);assert_eq!(marker,None);assert!(output.is_none());assert!(!birth.is_empty());
  let d=birth.cancel_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:d.release_bytes,maximum_depth:1};
  let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.run(grant,1,|effect|{let mut slot=None;birth.cancel_into(effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})}));
  let RetainedWorkPoll::Complete(turn)=turn else{panic!("cancellation must record its reply")};let p=turn.result.unwrap().progress();assert_eq!(turn.work,1);assert_eq!(marker,None);assert_eq!(heap.released_bytes,capacity);assert_eq!(original.take_admitted(),Some(true));original.post(p,grant,(0,capacity));assert!(birth.is_empty());assert!(original.is_idle());
 }

#[test]
fn table_birth_initializes_all_rows_with_one_paid_source(){
 let capacity=std::mem::size_of::<OriginalReturnEscrowTable>();let stock=RetainedCloneGrant{maximum_items:80,maximum_copy_bytes:65536,maximum_capacity_bytes:capacity,maximum_release_bytes:capacity,maximum_depth:4};
 let mut bank=EffectBank::new(Wallet::new(stock,80));let mut birth=OriginalReturnEscrowBirth::new();let mut output=None;let mut copied=0usize;
 for _ in 0..OriginalReturnEscrowTable::birth_demand().2{
  let d=birth.demand();assert!(d.copy_bytes<=4096);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:0,maximum_depth:1};
  let ((turn,marker),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{let mut slot=None;birth.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})}));
  let RetainedWorkPoll::Complete(turn)=turn else{panic!("table birth step must finish")};let progress=turn.result.unwrap().progress();copied+=progress.copied_bytes;assert_eq!(turn.work,1);assert_eq!(marker,(progress.retained_capacity_bytes!=0).then_some(capacity));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,0));assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(heap.requested_bytes,0));
 }
 assert_eq!(copied,OriginalReturnEscrowTable::birth_demand().0);assert!(birth.is_empty());assert!(output.as_ref().unwrap().is_empty());
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../📦️builder/🪙️return-escrow/🧪️tests/🌱️birth/case.json")).unwrap();let mut second=OriginalReturnEscrowBirth::new();let d=second.demand();let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let (turn,marker)=bank.run(denied,1,|effect|{let mut slot=None;second.advance_into(&mut output,effect,OriginalEffectReply::borrow(&mut slot).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:slot.unwrap(),work:effect.performed_work()})});let RetainedWorkPoll::Complete(turn)=turn else{panic!("occupied output must finish")};assert_eq!(turn.work>0,law[4]["performed"].as_bool().unwrap());assert_eq!(marker,None);assert!(second.is_empty());assert!(output.is_some());
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>(),maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1};
 let ((turn,_),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|{assert!(effect.performed(1).unwrap());drop(output.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes,released_bytes:capacity,..Default::default()}),work:1})}));
 let RetainedWorkPoll::Complete(turn)=turn else{panic!("table release must finish")};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(bank.take_admitted(),Some(true));bank.post(turn.result.unwrap(),grant,(0,capacity));assert!(bank.is_idle());
}
