//! 🧪️ Original cancellation Arc and waiter backing survive every denied or blocked turn.
use super::*;
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
fn policy()->RetainedCloneGrant{let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let p=&law["policy"];RetainedCloneGrant{maximum_items:p["items"].as_u64().unwrap()as usize,maximum_copy_bytes:p["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:p["release"].as_u64().unwrap()as usize,maximum_depth:p["depth"].as_u64().unwrap()as usize}}
fn identity(owner:&CancelTokenRetirement)->Option<(u8,usize)>{if let Some(current)=owner.current.as_ref(){Some((0,Arc::as_ptr(&current.0)as usize))}else{owner.node.as_ref().map(|node|(1,node as*const CancelNode as usize))}}
fn close(mut owner:CancelTokenRetirement,physical:usize)->usize{
 let grant=policy();let mut released=0;let mut turns=0;
 while !owner.terminal_is_empty(){turns+=1;assert!(turns<=512);let pointer=identity(&owner);let(demand,heap)=observe(||owner.retirement_demands().unwrap());assert_eq!(heap,(0,0));
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant})].into_iter().flatten(){let(step,heap)=observe(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(identity(&owner),pointer);}
  let(step,heap)=observe(||owner.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);released+=progress.released_bytes;
 }
 assert_eq!(released,physical);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));eprintln!("[DEBUG] actual original cancellation turns={turns} originalbirth={physical} physical={released} undergrantsHeap0 terminalDrop0 fixedbody4096");turns
}
#[test]
fn cancel_token_retirement_original_ancestor_nodes_release_without_recursive_drop(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for nodes in law["nodes"].as_array().unwrap(){let nodes=nodes.as_u64().unwrap()as usize;let(owner,heap)=observe(||{let mut token=CancelToken::root_now();for _ in 1..nodes{token=token.child_now();}CancelTokenRetirement::from_token(token)});assert_eq!(heap.0,nodes*shared_retirement_allocation_bytes::<CancelNode>());assert_eq!(close(owner,heap.0-heap.1),nodes*2);}
}
#[test]
fn cancel_token_retirement_shared_weak_and_registered_waiter_keep_exact_original(){
 for kind in 0..3{let((mut owner,mut shared,mut weak),heap)=observe(||{let token=CancelToken::root_now();let shared=(kind==0).then(||token.clone());let weak=(kind==1).then(||Arc::downgrade(&token.0));if kind==2{let mut waiters=token.0.waiters.lock();waiters.try_reserve_exact(8).unwrap();assert_eq!(waiters.capacity(),8);waiters.push((1,Waker::noop().clone()));}(CancelTokenRetirement::from_token(token),shared,weak)});let pointer=identity(&owner);let grant=policy();let(error,blocked)=observe(||owner.close_step(grant).unwrap_err());assert_eq!(blocked,(0,0));assert_eq!(identity(&owner),pointer);let expected=match kind{0=>CancelTokenRetirementBlocked::SharedAlias,1=>CancelTokenRetirementBlocked::WeakAlias,_=>CancelTokenRetirementBlocked::RegisteredWaiter};assert!(matches!(error,CancelTokenRetirementError::Blocked(actual)if actual==expected));
  let(_,cleared)=observe(||{drop(shared.take());drop(weak.take());if kind==2{owner.current.as_ref().unwrap().0.waiters.lock().clear();}});assert_eq!(cleared,(0,0));let turns=close(owner,heap.0-heap.1);assert_eq!(turns,if kind==2{3}else{2});eprintln!("[DEBUG] actual cancellation original blocked={expected:?} preserved exactpointer and backing; external alias/noop-waiter removal heap0");
 }
}

#[test]
fn cancel_token_borrowed_original_alias_return_keeps_root_waiters_and_exact_pointer(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪢️alias-return/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["policy"].clone()).unwrap();
 for count in law["aliases"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;let root=CancelToken::root_now();let pointer=Arc::as_ptr(&root.0);let waiter_capacity=law["waiterCapacity"].as_u64().unwrap()as usize;root.0.waiters.lock().try_reserve_exact(waiter_capacity).unwrap();assert_eq!(root.0.waiters.lock().capacity(),waiter_capacity);root.0.waiters.lock().push((1,Waker::noop().clone()));let mut owners:Vec<_>=(0..count).map(|_|CancelTokenRetirement::from_token(root.clone())).collect();assert_eq!(Arc::strong_count(&root.0),count+1);
  for (index,owner) in owners.iter_mut().enumerate(){let original=identity(owner);for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=observe(||owner.return_alias_step(&root,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(identity(owner),original);}
   let(step,heap)=observe(||owner.return_alias_step(&root,grant).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),serde_json::from_value::<RetainedCloneProgress>(law["receipt"].clone()).unwrap());assert!(owner.terminal_is_empty());assert_eq!(Arc::as_ptr(&root.0),pointer);assert_eq!(Arc::strong_count(&root.0),count-index);assert_eq!(root.0.waiters.lock().len(),1);
  }
  root.0.waiters.lock().clear();close(CancelTokenRetirement::from_token(root),shared_retirement_allocation_bytes::<CancelNode>()+waiter_capacity*size_of::<(u64,Waker)>());
 }
}
#[test]
fn cancel_token_borrowed_alias_return_rejects_foreign_root_and_keeps_original(){
 let original=CancelToken::root_now();let foreign=CancelToken::root_now();let mut retained=CancelTokenRetirement::from_token(original.clone());let pointer=identity(&retained);
 let ((matching,unrelated),heap)=observe(||(retained.is_original_alias_witness(&original),retained.is_original_alias_witness(&foreign)));
 assert!(matching);assert!(!unrelated);assert_eq!(heap,(0,0));assert_eq!(identity(&retained),pointer);assert_eq!(Arc::strong_count(&original.0),2);
 retained.return_alias_step(&original,policy()).unwrap();close(CancelTokenRetirement::from_token(original),shared_retirement_allocation_bytes::<CancelNode>());close(CancelTokenRetirement::from_token(foreign),shared_retirement_allocation_bytes::<CancelNode>());
 let root=CancelToken::root_now();let other=CancelToken::root_now();let mut owner=CancelTokenRetirement::from_token(root.clone());let pointer=identity(&owner);let grant=policy();let(error,heap)=observe(||owner.return_alias_step(&other,grant).unwrap_err());assert_eq!(heap,(0,0));assert!(matches!(error,CancelTokenRetirementError::Refused(error)if error.kind==ValueRefusalKind::InvariantViolated));assert_eq!(identity(&owner),pointer);owner.return_alias_step(&root,policy()).unwrap();close(CancelTokenRetirement::from_token(root),shared_retirement_allocation_bytes::<CancelNode>());close(CancelTokenRetirement::from_token(other),shared_retirement_allocation_bytes::<CancelNode>());
}
#[test]
fn cancel_token_borrowed_alias_return_concurrent_weak_and_strong_peers_never_release_original_header(){
 let root=CancelToken::root_now();let pointer=Arc::as_ptr(&root.0)as usize;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<CancelToken>(),maximum_depth:1,..Default::default()};
 std::thread::scope(|scope|{let source=&root;let peer=scope.spawn(move||{for _ in 0..4096{let weak=Arc::downgrade(&source.0);let alias=source.clone();assert_eq!(Arc::as_ptr(&alias.0)as usize,pointer);drop(alias);drop(weak);}});for _ in 0..4096{let mut owner=CancelTokenRetirement::from_token(root.clone());let(step,heap)=observe(||owner.return_alias_step(&root,grant).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress().released_bytes,0);assert!(owner.terminal_is_empty());assert_eq!(Arc::as_ptr(&root.0)as usize,pointer);}peer.join().unwrap();});assert_eq!(Arc::strong_count(&root.0),1);assert_eq!(Arc::weak_count(&root.0),0);close(CancelTokenRetirement::from_token(root),shared_retirement_allocation_bytes::<CancelNode>());eprintln!("[DEBUG] actual4096 witnessed aliases return with heap0 while4096 strong/weak races retain originalroot; final Arc allocation paid once");
}
