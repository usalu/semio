use super::*;
#[test]
fn bounded_history_set_retains_original_keys_and_each_physical_allocation(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use semio_framework_value::{retained_clone::RetainedCloneGrant,retirement::controlled::ControlledRetirement};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧺️set.json")).unwrap();
    for copy in law["copyBytes"].as_array().unwrap(){
        let ((mut owner,mut taken),source)=observe(||{let mut owner=HistoryFoldSet::new();for key in law["clearedKeys"].as_array().unwrap(){owner.insert(key.as_str().unwrap().to_owned());}(owner,Vec::with_capacity(law["taken"].as_array().unwrap().len()+1))});
        let detached_pointer=owner.get(law["reusedKey"].as_str().unwrap()).unwrap().as_ptr();
        let (_,cleared)=observe(||owner.clear());assert!(owner.is_empty());assert!(!owner.terminal_is_empty());assert_eq!((cleared.requested_bytes,cleared.released_bytes),(law["clearCapacityBytes"].as_u64().unwrap() as usize,law["clearReleaseBytes"].as_u64().unwrap() as usize));
        let (_,inserted)=observe(||{for key in law["keys"].as_array().unwrap(){owner.insert(key.as_str().unwrap().to_owned());}});
        let current_pointer=owner.get(law["reusedKey"].as_str().unwrap()).unwrap().as_ptr();assert_ne!(current_pointer,detached_pointer);
        let (detached,heap)=observe(||owner.0.extract_slot_if(0,|_,_|false).unwrap());assert_eq!(detached.0.as_ptr(),detached_pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.get(law["reusedKey"].as_str().unwrap()).unwrap().as_ptr(),current_pointer);taken.push(detached.0);
        assert_eq!(owner.len(),law["expected"].as_array().unwrap().len());assert_eq!(owner.first().map(String::as_str),Some(law["expected"][0].as_str().unwrap()));assert_eq!(owner.last().map(String::as_str),Some(law["expected"][2].as_str().unwrap()));
        assert_eq!(serde_json::to_value(&owner).unwrap(),law["expected"]);
        let actual:Vec<&str>=owner.iter().map(String::as_str).collect();assert_eq!(actual,law["expected"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>());
        for key in law["taken"].as_array().unwrap(){let key=key.as_str().unwrap();let pointer=owner.iter().find(|stored|stored.as_str()==key).unwrap().as_ptr();let (original,heap)=observe(||if key==law["taken"][0].as_str().unwrap(){owner.take(key).unwrap()}else{owner.pop_first().unwrap()});assert_eq!(original,key);assert_eq!(original.as_ptr(),pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));taken.push(original);}
        let (mut retirement,handoff)=observe(||ControlledRetirement::new((owner,taken)).unwrap_or_else(|_|panic!("original membership requires typed retirement")));assert_eq!((handoff.requested_bytes,handoff.released_bytes),(law["handoffCapacityBytes"].as_u64().unwrap() as usize,law["handoffReleaseBytes"].as_u64().unwrap() as usize));
        let (mut born,mut freed,mut refused,mut turns)=(0,0,0,0);
        while !retirement.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy.as_u64().unwrap() as usize).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
            if grant.maximum_release_bytes>law["maximumReleaseBytes"].as_u64().unwrap() as usize{for _ in 0..2{let (step,heap)=observe(||retirement.step(RetainedCloneGrant {maximum_release_bytes:law["maximumReleaseBytes"].as_u64().unwrap() as usize,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(retirement.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refused+=1;}}
            let (step,heap)=observe(||retirement.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;turns+=1;assert!(turns<100000);
        }
        assert!(refused>0);assert_eq!(source.requested_bytes-source.released_bytes+inserted.requested_bytes-inserted.released_bytes+born,freed);let (_,drop_heap)=observe(||drop(retirement));assert_eq!((drop_heap.requested_bytes,drop_heap.released_bytes),(0,0));eprintln!("[DEBUG] Original generic membership originalKeys=true takeBirth=0 takeRelease=0 copy={copy} source={} admittedBirth={born} physicalRelease={freed} eightByteRefusals={refused} turns={turns} terminalDrop=0",source.requested_bytes-source.released_bytes+inserted.requested_bytes-inserted.released_bytes);
    }
}
fn check<K:Ord,V>(tree:&HistoryFoldIndex<K,V>,root:Option<usize>)->(usize,usize){let Some(root)=root else{return(0,0)};let(left,left_count)=check(tree,tree.nodes[root].left);let(right,right_count)=check(tree,tree.nodes[root].right);assert!(left.abs_diff(right)<=1);assert_eq!(tree.nodes[root].height,1+left.max(right));assert_eq!(tree.nodes[root].count,1+left_count+right_count);(1+left.max(right),1+left_count+right_count)}
#[test]
fn bounded_history_owned_index_matches_independent_json_and_preserves_arena_custody(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut owned=HistoryFoldIndex::<String,String>::new();
    let mut independent=serde_json::Map::new();
    for operation in law["operations"].as_array().unwrap(){
        let allocation=owned.nodes.capacity()*std::mem::size_of::<FoldIndexNode<String,String>>();
        if let Some(set)=operation.get("set"){let key=set[0].as_str().unwrap().to_owned();let value=set[1].as_str().unwrap().to_owned();let actual=owned.insert(key.clone(),value.clone());let expected=independent.insert(key,serde_json::Value::String(value));assert_eq!(actual,expected.map(|value|value.as_str().unwrap().to_owned()));}
        else{let key=operation["remove"].as_str().unwrap();assert_eq!(owned.remove(key),independent.remove(key).map(|value|value.as_str().unwrap().to_owned()));assert_eq!(owned.nodes.capacity()*std::mem::size_of::<FoldIndexNode<String,String>>(),allocation);}
        let actual:serde_json::Map<String,serde_json::Value>=owned.iter().map(|(key,value)|(key.clone(),serde_json::Value::String(value.clone()))).collect();assert_eq!(actual,independent);
        for(key,value)in &independent{assert_eq!(owned.get(key),Some(&value.as_str().unwrap().to_owned()));}
        check(&owned,owned.root);
    }
    let expected:HistoryFoldIndex<String,String>=independent.iter().map(|(key,value)|(key.clone(),value.as_str().unwrap().to_owned())).rev().collect();assert_eq!(owned,expected);
    let mut extended=HistoryFoldIndex::<String,String>::new();extended.extend(independent.iter().map(|(key,value)|(key.clone(),value.as_str().unwrap().to_owned())));assert_eq!(extended,owned);assert_eq!(extended.clone(),owned);
    let node_slots=owned.nodes.len();let backing=owned.nodes.capacity();let original_keys=owned.displaced_keys.len();assert!(original_keys>=4);
    for(key,value)in independent{assert_eq!(owned.pop_first(),Some((key,value.as_str().unwrap().to_owned())));assert_eq!((owned.nodes.len(),owned.nodes.capacity()),(node_slots,backing));}
    assert!(owned.is_empty());assert!(owned.nodes.iter().all(|node|node.entry.is_none()));assert_eq!(owned.displaced_keys.len(),original_keys);
    println!("[DEBUG] owned fold AVL index matches independent JSON after every insert/remove; all rotations balance; live equality ignores arena history; removed node slots and key allocations remain owned");
}

#[test]
fn bounded_history_owned_index_tail_handoff_preserves_original_key_and_arena_after_every_balance() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut owner = HistoryFoldIndex::<String, String>::new();
    let mut oracle = std::collections::BTreeMap::<String, String>::new();
    for operation in law["operations"].as_array().unwrap() {
        if let Some(set) = operation.get("set") {
            owner.insert(set[0].as_str().unwrap().to_owned(), set[1].as_str().unwrap().to_owned());
            oracle.insert(set[0].as_str().unwrap().to_owned(), set[1].as_str().unwrap().to_owned());
        } else {
            assert_eq!(owner.remove(operation["remove"].as_str().unwrap()), oracle.remove(operation["remove"].as_str().unwrap()));
        }
    }
    let slots = owner.nodes.len();
    let capacity = owner.nodes.capacity();
    let displaced = owner.displaced_keys.len();
    while let Some(expected) = oracle.pop_last() {
        let pointer = owner.keys().next_back().unwrap().as_ptr();
        let (actual, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.pop_last().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(actual.0.as_ptr(), pointer);
        assert_eq!(actual, expected);
        assert_eq!((owner.nodes.len(), owner.nodes.capacity(), owner.displaced_keys.len()), (slots, capacity, displaced));
        check(&owner, owner.root);
        assert_eq!(owner.len(), oracle.len());
    }
    assert_eq!(owner.pop_last(), None);
    assert!(owner.nodes.iter().all(|node| node.entry.is_none()));
    assert!(!owner.terminal_is_empty());
    assert_eq!(law["expected"]["tailHandoff"], "greatest-first-original-row-with-retained-arena");
    eprintln!("[DEBUG] original AVL greatest-first row handoff equals standard BTreeMap; every balance valid, same key pointer, zero heap, retained native arena");
}

/// 🌱️ Lazy Copy-key entry preserves the original payload, arena and every physical close receipt.
#[test]
fn original_fold_index_lazy_entry_preserves_source_and_full_grant_receipts(){
    use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌱️entry/🔣️.json")).unwrap();
    for copy in law["copyGrants"].as_array().unwrap(){
        let (mut owner,source)=observe(||{let mut owner=HistoryFoldIndex::<u64,String>::new();for row in law["sourceRows"].as_array().unwrap(){let mut value=String::with_capacity(law["sourceCapacityBytes"].as_u64().unwrap() as usize);value.push_str(row[1].as_str().unwrap());owner.insert(row[0].as_u64().unwrap(),value);}owner});
        assert_eq!(source.released_bytes,0);let occupied=law["occupiedKey"].as_u64().unwrap();let pointer=owner.get(&occupied).unwrap().as_ptr();let arena=owner.nodes.backing_ptr(0).unwrap();let mut occupied_calls=0;let mut vacant_calls=0;
        let (_,borrow)=observe(||{let value=owner.entry(occupied).or_insert_with(||{occupied_calls+=1;String::from("unused")});assert_eq!(value.as_ptr(),pointer);let (key,value)=owner.get_key_value(&occupied).unwrap();assert_eq!(*key,occupied);assert_eq!(value.as_ptr(),pointer);});
        assert_eq!(occupied_calls,law["expected"]["occupiedLazyCalls"].as_u64().unwrap());assert_eq!((borrow.requested_bytes,borrow.released_bytes),(0,0));
        let (_,vacant)=observe(||{owner.entry(law["vacantKey"].as_u64().unwrap()).or_insert_with(||{vacant_calls+=1;String::new()});owner.entry(law["modifiedKey"].as_u64().unwrap()).and_modify(|value|value.push('!')).or_default();for(_,value)in owner.slot_entries_mut(){if !value.is_empty(){value.push('?');}}});
        assert_eq!(vacant_calls,law["expected"]["vacantLazyCalls"].as_u64().unwrap());assert_eq!((vacant.requested_bytes,vacant.released_bytes),(0,0));assert_eq!(owner.nodes.backing_ptr(0).unwrap(),arena);assert_eq!(owner.get(&occupied).unwrap().as_ptr(),pointer);check(&owner,owner.root);
        let actual:serde_json::Map<String,serde_json::Value>=owner.iter().map(|(key,value)|(key.to_string(),serde_json::Value::String(value.clone()))).collect();assert_eq!(actual,law["expected"]["map"].as_object().unwrap().clone());
        let (mut retirement,birth)=observe(||ControlledRetirement::new(owner).unwrap_or_else(|(error,_)|panic!("original index refused: {error}")));assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));let mut added=0;let mut released=0;let mut turns=0;
        while !retirement.terminal_is_empty(){turns+=1;assert!(turns<65536);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy.as_u64().unwrap() as usize).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
            let denied=RetainedCloneGrant{maximum_items:0,..grant};let (step,heap)=observe(||retirement.step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let capacity=grant.maximum_capacity_bytes.checked_sub(1).map(|capacity|RetainedCloneGrant{maximum_capacity_bytes:capacity,..grant});
            let depth=grant.maximum_depth.checked_sub(1).map(|depth|RetainedCloneGrant{maximum_depth:depth,..grant});
            let release=(retirement.next_copy_byte_demand().unwrap()==0).then(||grant.maximum_release_bytes.checked_sub(1).map(|release|RetainedCloneGrant{maximum_release_bytes:release,..grant})).flatten();
            let zero_copy=(retirement.next_copy_byte_demand().unwrap()!=0).then_some(RetainedCloneGrant{maximum_copy_bytes:0,..grant});
            for denied in [capacity,depth,release,zero_copy].into_iter().flatten(){let (result,heap)=observe(||retirement.step(denied));if denied.maximum_depth<grant.maximum_depth{assert!(matches!(result,Err(error) if error.kind==semio_framework_value::ValueRefusalKind::DepthLimit));}else{assert_eq!(result.unwrap().progress(),RetainedCloneProgress::default());}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
            let (step,heap)=observe(||retirement.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));added+=heap.requested_bytes;released+=heap.released_bytes;
        }
        assert_eq!(released,source.requested_bytes+added);let (_,drop_heap)=observe(||drop(retirement));assert_eq!((drop_heap.requested_bytes,drop_heap.released_bytes),(0,0));eprintln!("[DEBUG] original fold index lazy Copy entry samePayload=true sameArena=true borrowBirth=0 occupiedCalls={occupied_calls} vacantCalls={vacant_calls} copy={copy} source={} admitted={added} physical={released} turns={turns} terminalDropFree=0",source.requested_bytes);
    }
}

/// 🎟️ Original index insertion admits each page and preserves every real key and heap receipt.
#[test]
fn original_fold_index_admitted_insert_keeps_each_physical_page(){
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️insertion/🔣️.json")).unwrap();
 for copy in law["copyGrants"].as_array().unwrap(){
  let capacity=law["sourceCapacityBytes"].as_u64().unwrap()as usize;
  let (mut owner,source)=observe(||{let mut owner=HistoryFoldIndex::<String,String>::new();for row in law["sourceRows"].as_array().unwrap(){let mut key=String::with_capacity(capacity);key.push_str(row[0].as_str().unwrap());let mut value=String::with_capacity(capacity);value.push_str(row[1].as_str().unwrap());owner.insert(key,value);}owner});
  let original_key=owner.first_key_value().unwrap().0.as_ptr();let mut original=source.requested_bytes-source.released_bytes;let mut admitted=0;let mut releases=0;let mut reservation_turns=0;
  let (mut displaced,backing)=observe(||Vec::<String>::with_capacity(law["insertions"].as_array().unwrap().len()));original+=backing.requested_bytes-backing.released_bytes;
  for row in law["insertions"].as_array().unwrap(){
   let ((mut key,mut value),input)=observe(||{let mut key=String::with_capacity(capacity);key.push_str(row[0].as_str().unwrap());let mut value=String::with_capacity(capacity);value.push_str(row[1].as_str().unwrap());(key,value)});original+=input.requested_bytes-input.released_bytes;let pointers=(key.as_ptr(),value.as_ptr());
   loop{
    let copy=owner.next_insert_copy_byte_demand(&key).unwrap();let capacity=owner.next_insert_capacity_byte_demand(&key,copy).unwrap();let release=owner.next_insert_release_byte_demand(&key).unwrap();let depth=owner.next_insert_depth_demand(&key).unwrap();
    assert_eq!((copy,release),(law["expected"]["copyBytes"].as_u64().unwrap()as usize,law["expected"]["releaseBytes"].as_u64().unwrap()as usize));if capacity==0{break;}
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
    for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (result,heap)=observe(||owner.reserve_insert_step(&key,denied));let (_,progress)=result.unwrap_err();assert_eq!(progress,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((key.as_ptr(),value.as_ptr()),pointers);assert_eq!(owner.first_key_value().unwrap().0.as_ptr(),original_key);}
    let (progress,heap)=observe(||owner.reserve_insert_step(&key,grant).unwrap_or_else(|(error,_)|panic!("original index reservation: {error}")));assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(heap.requested_bytes,heap.released_bytes));admitted+=heap.requested_bytes;releases+=heap.released_bytes;reservation_turns+=1;
   }
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:owner.next_insert_depth_demand(&key).unwrap()};
   let (denied,heap)=observe(||owner.insert_reserved(key,value,RetainedCloneGrant{maximum_items:0,..grant}));let (_,returned_key,returned_value)=denied.unwrap_err();key=returned_key;value=returned_value;assert_eq!((key.as_ptr(),value.as_ptr()),pointers);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let (result,heap)=observe(||owner.insert_reserved(key,value,grant));let (previous,progress)=result.unwrap_or_else(|(error,_,_)|panic!("original index placement: {error}"));assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some(previous)=previous{displaced.push(previous);}assert_eq!(owner.first_key_value().unwrap().0.as_ptr(),original_key);check(&owner,owner.root);
  }
  assert_eq!(serde_json::to_value(&owner).unwrap(),law["expected"]["map"]);assert!(reservation_turns>1);
  let (mut close,handoff)=observe(||ControlledRetirement::new((owner,displaced)).unwrap_or_else(|(error,_)|panic!("original inserted owner refused: {error}")));assert_eq!((handoff.requested_bytes,handoff.released_bytes),(0,0));let mut turns=0;
  while !close.terminal_is_empty(){turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap()as usize,maximum_capacity_bytes:close.next_capacity_byte_demand(copy.as_u64().unwrap()as usize).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let (step,heap)=observe(||close.step(grant).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));admitted+=heap.requested_bytes;releases+=heap.released_bytes;}
  let (_,terminal)=observe(||drop(close));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,law["expected"]["terminalDropBytes"].as_u64().unwrap()as usize));assert_eq!(original+admitted,releases);
  eprintln!("[DEBUG] original paged Index insertion copy={} rows=140 reserveTurns={reservation_turns} original={original} admitted={admitted} physical={releases} closeTurns={turns} sameOriginalKey=true placementBirth0/free0 terminalDrop0",copy);
 }
}

/// 🪹️ Every detached original row resumes in its same retained slot without growing the arena.
#[test]
fn original_fold_index_reuses_each_detached_slot_without_heap_growth(){
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️insertion/🔣️.json")).unwrap();let reuse=&law["reuse"];
 for copy in law["copyGrants"].as_array().unwrap(){
  let (mut owner,source)=observe(||{let mut owner=HistoryFoldIndex::<u64,String>::new();for row in reuse["sourceRows"].as_array().unwrap(){let mut value=String::with_capacity(law["sourceCapacityBytes"].as_u64().unwrap()as usize);value.push_str(row[1].as_str().unwrap());owner.insert(row[0].as_u64().unwrap(),value);}owner});
  let slots=owner.slot_count();let capacity=owner.nodes.capacity();let arena=owner.nodes.backing_ptr(0).unwrap();
  for turn in 0..reuse["rounds"].as_u64().unwrap()as usize{
   let mode=reuse["modes"][turn%reuse["modes"].as_array().unwrap().len()].as_str().unwrap();let key=match mode{"pop-first"=>*owner.first_key_value().unwrap().0,"pop-last"=>*owner.iter().next_back().unwrap().0,_=>*owner.iter().nth(turn%owner.len()).unwrap().0};
   let slot=owner.locate(&key).unwrap();let (depth,heap)=observe(||owner.next_extract_slot_depth_demand(slot).unwrap());assert!(depth>0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let pointer=owner.get(&key).unwrap().as_ptr();let (row,removed)=observe(||match mode{"pop-first"=>owner.pop_first().unwrap(),"pop-last"=>owner.pop_last().unwrap(),_=>owner.remove_entry(&key).unwrap()});assert_eq!((removed.requested_bytes,removed.released_bytes),(0,0));assert_eq!(row.1.as_ptr(),pointer);
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:owner.next_insert_depth_demand(&row.0).unwrap()};
   let (quote,query)=observe(||(owner.next_insert_copy_byte_demand(&row.0).unwrap(),owner.next_insert_capacity_byte_demand(&row.0,0).unwrap(),owner.next_insert_release_byte_demand(&row.0).unwrap()));assert_eq!(quote,(0,reuse["expected"]["capacityBytes"].as_u64().unwrap()as usize,0));assert_eq!((query.requested_bytes,query.released_bytes),(0,0));
   let (denied,refusal)=observe(||owner.insert_reserved(row.0,row.1,RetainedCloneGrant{maximum_items:0,..grant}));let (_,key,value)=denied.unwrap_err();assert_eq!(value.as_ptr(),pointer);assert_eq!((refusal.requested_bytes,refusal.released_bytes),(0,0));
   let (placed,heap)=observe(||owner.insert_reserved(key,value,grant));let (previous,progress)=placed.unwrap_or_else(|(error,_,_)|panic!("original vacant slot: {error}"));assert!(previous.is_none());assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   assert_eq!(owner.slot_count()-slots,reuse["expected"]["slotGrowth"].as_u64().unwrap()as usize);assert_eq!(owner.locate(&key),Some(slot));assert_eq!(owner.nodes.capacity(),capacity);assert_eq!(owner.nodes.backing_ptr(0).unwrap(),arena);assert_eq!(owner.get(&key).unwrap().as_ptr(),pointer);check(&owner,owner.root);
  }
  for lazy in [false,true]{let key=*owner.first_key_value().unwrap().0;let slot=owner.locate(&key).unwrap();let pointer=owner.get(&key).unwrap().as_ptr();let row=owner.remove_entry(&key).unwrap();let (_,heap)=observe(||{let resumed=if lazy{owner.entry(row.0).or_insert_with(||row.1)}else{owner.get_or_insert(row.0,row.1)};assert_eq!(resumed.as_ptr(),pointer);});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.locate(&key),Some(slot));assert_eq!(owner.slot_count(),slots);check(&owner,owner.root);}
  assert_eq!(serde_json::to_value(&owner).unwrap(),reuse["expected"]["map"]);let (mut close,handoff)=observe(||ControlledRetirement::new(owner).unwrap_or_else(|_|panic!("original reused index authority")));assert_eq!((handoff.requested_bytes,handoff.released_bytes),(0,0));let(mut born,mut freed,mut turns)=(0,0,0);
  while !close.terminal_is_empty(){turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap()as usize,maximum_capacity_bytes:close.next_capacity_byte_demand(copy.as_u64().unwrap()as usize).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let (step,heap)=observe(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}
  assert_eq!(source.requested_bytes-source.released_bytes+born,freed);let (_,terminal)=observe(||drop(close));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));eprintln!("[DEBUG] original Index reused slots copy={copy} rounds=36 slotGrowth0 sameSlot=true sameValue=true sameBacking=true original={} admitted={born} physical={freed} turns={turns} terminalDrop0",source.requested_bytes-source.released_bytes);
 }
}

/// 🧺️ Membership uses original admitted index pages and retains both incoming and stored key custody.
#[test]
fn original_fold_membership_admitted_insert_keeps_each_original_key(){
 use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️insertion/🔣️.json")).unwrap();
 for copy in law["copyGrants"].as_array().unwrap(){
  let capacity=law["sourceCapacityBytes"].as_u64().unwrap()as usize;
  let (mut set,source)=observe(||{let mut set=HistoryFoldSet::<String>::new();for row in law["sourceRows"].as_array().unwrap(){let mut key=String::with_capacity(capacity);key.push_str(row[0].as_str().unwrap());set.insert(key);}set});let original_pointer=set.first().unwrap().as_ptr();let(mut original,mut admitted,mut freed)=(source.requested_bytes-source.released_bytes,0,0);
  for row in law["insertions"].as_array().unwrap(){let(mut key,birth)=observe(||{let mut key=String::with_capacity(capacity);key.push_str(row[0].as_str().unwrap());key});original+=birth.requested_bytes-birth.released_bytes;let pointer=key.as_ptr();let expected=!set.contains(&key);
   for _ in 0..1000{let(copy,capacity,release,depth)=(set.next_insert_copy_byte_demand(&key).unwrap(),set.next_insert_capacity_byte_demand(&key,0).unwrap(),set.next_insert_release_byte_demand(&key).unwrap(),set.next_insert_depth_demand(&key).unwrap());assert_eq!((copy,release),(0,0));if capacity==0{break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};let(result,heap)=observe(||set.reserve_insert_step(&key,RetainedCloneGrant{maximum_capacity_bytes:capacity-1,..grant}));let(_,receipt)=result.unwrap_err();assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(receipt,heap)=observe(||set.reserve_insert_step(&key,grant).unwrap_or_else(|_|panic!("original membership reserve")));assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));admitted+=heap.requested_bytes;freed+=heap.released_bytes;}
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:set.next_insert_depth_demand(&key).unwrap()};let(result,heap)=observe(||set.insert_reserved(key,RetainedCloneGrant{maximum_items:0,..grant}));let(_,returned)=result.unwrap_err();key=returned;assert_eq!(key.as_ptr(),pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(result,heap)=observe(||set.insert_reserved(key,grant));let(inserted,receipt)=result.unwrap_or_else(|_|panic!("original membership placement"));assert_eq!(inserted,expected);assert_eq!(receipt,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if inserted{assert_eq!(set.get(row[0].as_str().unwrap()).unwrap().as_ptr(),pointer);}assert_eq!(set.first().unwrap().as_ptr(),original_pointer);
  }
  assert_eq!(serde_json::to_value(&set).unwrap(),law["setExpectedKeys"]);let(mut close,handoff)=observe(||ControlledRetirement::new(set).unwrap_or_else(|_|panic!("original membership close")));assert_eq!((handoff.requested_bytes,handoff.released_bytes),(0,0));let mut turns=0;
  while !close.terminal_is_empty(){turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap()as usize,maximum_capacity_bytes:close.next_capacity_byte_demand(copy.as_u64().unwrap()as usize).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,heap)=observe(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));admitted+=heap.requested_bytes;freed+=heap.released_bytes;}
  assert_eq!(original+admitted,freed);let(_,terminal)=observe(||drop(close));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));eprintln!("[DEBUG] original admitted membership copy={copy} keys=140 original={original} admitted={admitted} physical={freed} turns={turns} originalPointers=true incomingHandback=true placement0/0 terminalDrop0");
 }
}
