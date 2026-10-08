use super::*;
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
        assert_eq!(source.released_bytes,0);let occupied=law["occupiedKey"].as_u64().unwrap();let pointer=owner.get(&occupied).unwrap().as_ptr();let arena=owner.nodes.as_ptr();let mut occupied_calls=0;let mut vacant_calls=0;
        let (_,borrow)=observe(||{let value=owner.entry(occupied).or_insert_with(||{occupied_calls+=1;String::from("unused")});assert_eq!(value.as_ptr(),pointer);let (key,value)=owner.get_key_value(&occupied).unwrap();assert_eq!(*key,occupied);assert_eq!(value.as_ptr(),pointer);});
        assert_eq!(occupied_calls,law["expected"]["occupiedLazyCalls"].as_u64().unwrap());assert_eq!((borrow.requested_bytes,borrow.released_bytes),(0,0));
        let (_,vacant)=observe(||{owner.entry(law["vacantKey"].as_u64().unwrap()).or_insert_with(||{vacant_calls+=1;String::new()});owner.entry(law["modifiedKey"].as_u64().unwrap()).and_modify(|value|value.push('!')).or_default();for(_,value)in owner.slot_entries_mut(){if !value.is_empty(){value.push('?');}}});
        assert_eq!(vacant_calls,law["expected"]["vacantLazyCalls"].as_u64().unwrap());assert_eq!((vacant.requested_bytes,vacant.released_bytes),(0,0));assert_eq!(owner.nodes.as_ptr(),arena);assert_eq!(owner.get(&occupied).unwrap().as_ptr(),pointer);check(&owner,owner.root);
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
