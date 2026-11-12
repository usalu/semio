//! 🧪️ Original cancellation Arc and waiter backing survive every denied or blocked turn.
use super::*;
#[path="../../../🧪️testing/🪙️effect/🦀️.rs"]
mod effect_turn;
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
fn policy()->RetainedCloneGrant{let law:serde_json::Value=serde_json::from_str(crate::text("🛑️cancel/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let p=&law["policy"];RetainedCloneGrant{maximum_items:p["items"].as_u64().unwrap()as usize,maximum_copy_bytes:p["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:p["release"].as_u64().unwrap()as usize,maximum_depth:p["depth"].as_u64().unwrap()as usize}}
fn identity(owner:&CancelTokenRetirement)->Option<(u8,usize)>{if let Some(current)=owner.current(){Some((0,Arc::as_ptr(&current.0)as usize))}else{owner.node().map(|node|(1,node as*const CancelNode as usize))}}
fn close(mut owner:CancelTokenRetirement,physical:usize)->usize{
 let mut tn=effect_turn::Turn::new();
 let grant=policy();let mut released=0;let mut turns=0;
 while !owner.terminal_is_empty(){turns+=1;assert!(turns<=512);let pointer=identity(&owner);let(demand,heap)=observe(||owner.retirement_demands().unwrap());assert_eq!(heap,(0,0));
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant})].into_iter().flatten(){let(turn,heap)=observe(||tn.run(denied,|e|owner.close_step(e)));assert_eq!(turn.work,0);let step=turn.result.unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(identity(&owner),pointer);}
  let(turn,heap)=observe(||tn.run(grant,|e|owner.close_step(e)));assert_eq!(turn.work,1);let progress=turn.result.unwrap().progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);released+=progress.released_bytes;
 }
 assert_eq!(released,physical);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));turns
}
#[test]
fn cancel_token_retirement_original_ancestor_nodes_release_without_recursive_drop(){
 let law:serde_json::Value=serde_json::from_str(crate::text("🛑️cancel/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for nodes in law["nodes"].as_array().unwrap(){let nodes=nodes.as_u64().unwrap()as usize;let(owner,heap)=observe(||{let mut token=CancelToken::root_now();for _ in 1..nodes{token=token.child_now();}CancelTokenRetirement::from_token(token)});assert_eq!(heap.0,nodes*shared_retirement_allocation_bytes::<CancelNode>());assert_eq!(close(owner,heap.0-heap.1),nodes*2);}
}
#[test]
fn cancel_token_retirement_shared_weak_and_registered_waiter_keep_exact_original(){
 let mut tn=effect_turn::Turn::new();
 for kind in 0..3{let((mut owner,mut shared,mut weak),heap)=observe(||{let token=CancelToken::root_now();let shared=(kind==0).then(||token.clone());let weak=(kind==1).then(||Arc::downgrade(&token.0));if kind==2{let mut waiters=token.0.waiters.lock();waiters.try_reserve_exact(8).unwrap();assert_eq!(waiters.capacity(),8);waiters.push((1,Waker::noop().clone()));}(CancelTokenRetirement::from_token(token),shared,weak)});let pointer=identity(&owner);let grant=policy();let(turn,blocked)=observe(||tn.run(grant,|e|owner.close_step(e)));assert_eq!(turn.work,1);let error=turn.result.unwrap_err();assert_eq!(blocked,(0,0));assert_eq!(identity(&owner),pointer);let expected=match kind{0=>CancelTokenRetirementBlocked::SharedAlias,1=>CancelTokenRetirementBlocked::WeakAlias,_=>CancelTokenRetirementBlocked::RegisteredWaiter};assert!(matches!(error,CancelTokenRetirementError::Blocked(actual)if actual==expected));
  let(_,cleared)=observe(||{drop(shared.take());drop(weak.take());if kind==2{owner.current().unwrap().0.waiters.lock().clear();}});assert_eq!(cleared,(0,0));let turns=close(owner,heap.0-heap.1);assert_eq!(turns,if kind==2{3}else{2});
 }
}

#[test]
fn cancel_token_borrowed_original_alias_return_keeps_root_waiters_and_exact_pointer(){
 let mut tn=effect_turn::Turn::new();
 let law:serde_json::Value=serde_json::from_str(crate::text("🛑️cancel/♻️retirement/🧪️tests/🧫️fixtures/🪢️alias-return/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["policy"].clone()).unwrap();
 for count in law["aliases"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;let root=CancelToken::root_now();let pointer=Arc::as_ptr(&root.0);let waiter_capacity=law["waiterCapacity"].as_u64().unwrap()as usize;root.0.waiters.lock().try_reserve_exact(waiter_capacity).unwrap();assert_eq!(root.0.waiters.lock().capacity(),waiter_capacity);root.0.waiters.lock().push((1,Waker::noop().clone()));let mut owners:Vec<_>=(0..count).map(|_|CancelTokenRetirement::from_token(root.clone())).collect();assert_eq!(Arc::strong_count(&root.0),count+1);
  for (index,owner) in owners.iter_mut().enumerate(){let original=identity(owner);for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(turn,heap)=observe(||tn.run(denied,|e|owner.return_alias_step(&root,e)));assert_eq!(turn.work,0);let step=turn.result.unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(identity(owner),original);}
   let(turn,heap)=observe(||tn.run(grant,|e|owner.return_alias_step(&root,e)));assert_eq!(turn.work,1);let step=turn.result.unwrap();assert_eq!(heap,(0,0));assert_eq!(step.progress(),serde_json::from_value::<RetainedCloneProgress>(law["receipt"].clone()).unwrap());assert!(owner.terminal_is_empty());assert_eq!(Arc::as_ptr(&root.0),pointer);assert_eq!(Arc::strong_count(&root.0),count-index);assert_eq!(root.0.waiters.lock().len(),1);
  }
  root.0.waiters.lock().clear();close(CancelTokenRetirement::from_token(root),shared_retirement_allocation_bytes::<CancelNode>()+waiter_capacity*size_of::<(u64,Waker)>());
 }
}
#[test]
fn cancel_token_borrowed_alias_return_rejects_foreign_root_and_keeps_original(){
 let mut tn=effect_turn::Turn::new();
 let original=CancelToken::root_now();let foreign=CancelToken::root_now();let mut retained=CancelTokenRetirement::from_token(original.clone());let pointer=identity(&retained);
 let ((matching,unrelated),heap)=observe(||(retained.is_original_alias_witness(&original),retained.is_original_alias_witness(&foreign)));
 assert!(matching);assert!(!unrelated);assert_eq!(heap,(0,0));assert_eq!(identity(&retained),pointer);assert_eq!(Arc::strong_count(&original.0),2);
 tn.run(policy(),|e|retained.return_alias_step(&original,e)).result.unwrap();close(CancelTokenRetirement::from_token(original),shared_retirement_allocation_bytes::<CancelNode>());close(CancelTokenRetirement::from_token(foreign),shared_retirement_allocation_bytes::<CancelNode>());
 let root=CancelToken::root_now();let other=CancelToken::root_now();let mut owner=CancelTokenRetirement::from_token(root.clone());let pointer=identity(&owner);let grant=policy();let(error,heap)=observe(||tn.run(grant,|e|owner.return_alias_step(&other,e)).result.unwrap_err());assert_eq!(heap,(0,0));assert!(matches!(error,CancelTokenRetirementError::Refused(error)if error.kind==ValueRefusalKind::InvariantViolated));assert_eq!(identity(&owner),pointer);tn.run(policy(),|e|owner.return_alias_step(&root,e)).result.unwrap();close(CancelTokenRetirement::from_token(root),shared_retirement_allocation_bytes::<CancelNode>());close(CancelTokenRetirement::from_token(other),shared_retirement_allocation_bytes::<CancelNode>());
}
#[test]
fn cancel_token_borrowed_alias_return_concurrent_weak_and_strong_peers_never_release_original_header(){
 let mut tn=effect_turn::Turn::new();
 let root=CancelToken::root_now();let pointer=Arc::as_ptr(&root.0)as usize;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<CancelToken>(),maximum_depth:1,..Default::default()};
 std::thread::scope(|scope|{let source=&root;let peer=scope.spawn(move||{for _ in 0..4096{let weak=Arc::downgrade(&source.0);let alias=source.clone();assert_eq!(Arc::as_ptr(&alias.0)as usize,pointer);drop(alias);drop(weak);}});for _ in 0..4096{let mut owner=CancelTokenRetirement::from_token(root.clone());let(turn,heap)=observe(||tn.run(grant,|e|owner.return_alias_step(&root,e)));assert_eq!(turn.work,1);let step=turn.result.unwrap();assert_eq!(heap,(0,0));assert_eq!(step.progress().released_bytes,0);assert!(owner.terminal_is_empty());assert_eq!(Arc::as_ptr(&root.0)as usize,pointer);}peer.join().unwrap();});assert_eq!(Arc::strong_count(&root.0),1);assert_eq!(Arc::weak_count(&root.0),0);close(CancelTokenRetirement::from_token(root),shared_retirement_allocation_bytes::<CancelNode>());
}
#[test]
fn cancel_token_retire_owned_erases_the_typed_ladder_and_refuses_registered_waiters(){
 use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
 let grant=policy();
 let expected=2*shared_retirement_allocation_bytes::<CancelNode>();
 let mut cursor=Box::new(CancelToken::root_now().child_now().retirement_seed())as Box<dyn RetirementCursor>;let mut released=0;let mut turns=0;
 while !cursor.terminal_is_empty(){turns+=1;assert!(turns<=16);match effect_turn::close_step(grant,cursor.as_mut()).0{RetirementStep::Progress(progress,_)=>released+=progress.released_bytes,RetirementStep::Complete(_)=>break,_=>panic!("original cancellation retirement turn")}}
 assert_eq!(released,expected);assert!(cursor.terminal_is_empty());
 let waiting=CancelToken::root_now();waiting.0.waiters.lock().push((1,Waker::noop().clone()));let weak=Arc::downgrade(&waiting.0);let mut cursor=Box::new(waiting.retirement_seed())as Box<dyn RetirementCursor>;
 assert!(matches!(effect_turn::close_step(grant,cursor.as_mut()).0,RetirementStep::Failure(error,_)if error.primary().unwrap().kind==ValueRefusalKind::UnsupportedOwner));assert!(!cursor.terminal_is_empty());
 weak.upgrade().unwrap().waiters.lock().clear();drop(weak);
 while !cursor.terminal_is_empty(){turns+=1;assert!(turns<=32);match effect_turn::close_step(grant,cursor.as_mut()).0{RetirementStep::Progress(_,_)=>{}, RetirementStep::Complete(_)=>break,_=>panic!("cleared cancellation retirement turn")}}
}

#[test]
fn original_owned_node_quote_does_not_lock_waiter_storage(){
 let mut tn=effect_turn::Turn::new();
 let law:serde_json::Value=serde_json::from_str(crate::text("🛑️cancel/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let quote=&law["ownedWaiterQuote"];assert!(!quote["requiresLock"].as_bool().unwrap());
 let capacity=quote["capacity"].as_u64().unwrap() as usize;let token=CancelToken::root_now();token.0.waiters.lock().try_reserve_exact(capacity).unwrap();assert_eq!(token.0.waiters.lock().capacity(),capacity);
 let mut owner=CancelTokenRetirement::from_token(token);
 let turn=tn.run(policy(),|e|owner.close_step(e));assert_eq!(turn.work,1);turn.result.unwrap();
 let node=owner.node().expect("original node captured");let held=node.waiters.lock();
 let release=quote["releaseElements"].as_u64().unwrap() as usize*size_of::<(u64,Waker)>();assert_eq!(owner.retirement_demands().unwrap().release_bytes,release);drop(held);
 let mut cursor=owner.into_cursor();assert_eq!(cursor.next_computational_work(1).unwrap(),1);let mut released=0;while !cursor.terminal_is_empty(){if let(RetirementStep::Progress(progress,_),_)=effect_turn::close_step(policy(),&mut cursor){released+=progress.released_bytes}}assert_eq!(released,release);
}
