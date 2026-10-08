//! 🧪️ Draft supersession retains every original input allocation through ordered cancellation.
use super::*;
use std::mem::size_of;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use semio_framework_value::{retirement::controlled::ControlledRetirement, retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

fn text(value:&str,capacity:usize)->String {let mut result=String::with_capacity(capacity);result.push_str(value);result}
fn original(law:&serde_json::Value)->HistoryInputDrafts {
    let mut owner=HistoryInputDrafts::new();
    for row in law["entries"].as_array().unwrap() {
        let id=MutationId(text(row["id"].as_str().unwrap(),law["textCapacity"].as_u64().unwrap()as usize));
        let replacement=if row["withdrawn"].as_bool().unwrap() {InputReplacement::Withdrawn}else {let mut payload=Vec::with_capacity(law["payloadCapacity"].as_u64().unwrap()as usize);for value in row["payload"].as_array().unwrap(){payload.push(value.as_u64().unwrap()as u8);}InputReplacement::Input {schema:text(row["schema"].as_str().unwrap(),law["textCapacity"].as_u64().unwrap()as usize),payload}};
        owner.insert(id,replacement);
    }
    owner
}
fn pointers(owner:&HistoryInputDrafts)->Vec<Option<(*const u8,*const u8,usize,usize)>> {
    owner.rows.iter().map(|row|match row {Some(InputReplacement::Input {schema,payload})=>Some((schema.as_ptr(),payload.as_ptr(),schema.capacity(),payload.capacity())),_=>None}).collect()
}
fn close(owner:(HistoryInputDrafts,Vec<(MutationId,InputReplacement)>),held:usize,copy:usize) {
    let(cursor,heap)=observe_heap_allocations_on_this_thread(||ControlledRetirement::new(owner).unwrap_or_else(|_|panic!("original draft controlled authority")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut cursor=cursor;let(mut allocated,mut released)=(0,0);
    for turn in 0..100000 {
        let((work,capacity,release,depth),heap)=observe_heap_allocations_on_this_thread(||(cursor.next_copy_byte_demand().unwrap(),cursor.next_capacity_byte_demand(copy).unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.max(work),maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        for below in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_copy_bytes:work.saturating_sub(1),..grant},RetainedCloneGrant {maximum_capacity_bytes:capacity.saturating_sub(1),..grant},RetainedCloneGrant {maximum_release_bytes:release.saturating_sub(1),..grant},RetainedCloneGrant {maximum_depth:depth.saturating_sub(1),..grant}] {
            if below.maximum_items!=0&&below.maximum_copy_bytes>=work&&below.maximum_capacity_bytes>=capacity&&below.maximum_release_bytes>=release&&below.maximum_depth>=depth {continue;}
            let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.step(below));match step {Ok(step)=>assert_eq!(step.progress(),RetainedCloneProgress::default()),Err(error)=>assert!(matches!(error.kind,semio_framework_value::ValueRefusalKind::DepthLimit|semio_framework_value::ValueRefusalKind::OwnershipLimit|semio_framework_value::ValueRefusalKind::WorkLimit))}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0),"undergrant turn{turn}");
        }
        let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.step(grant).unwrap());assert_eq!(step.progress().retained_capacity_bytes,heap.requested_bytes,"birth turn{turn}");assert_eq!(step.progress().released_bytes,heap.released_bytes,"release turn{turn}");assert!(step.progress().fits(grant));allocated+=heap.requested_bytes;released+=heap.released_bytes;
        if cursor.terminal_is_empty(){break;}assert!(turn<99999,"original draft cleanup stalled");
    }
    assert_eq!(released,held+allocated);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
#[test]
fn history_drafts_preserve_original_replaced_input_rows_and_every_arena_on_cancel() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for copy in law["copyGrants"].as_array().unwrap(){for cancel in law["cancelAt"].as_array().unwrap(){
        let(owner,heap)=observe_heap_allocations_on_this_thread(||original(&law));let mut owner=owner;let held=heap.requested_bytes-heap.released_bytes;
        let expected=law["expectedOrder"].as_array().unwrap().iter().map(|id|id.as_str().unwrap()).collect::<Vec<_>>();assert_eq!(owner.keys().map(|id|id.0.as_str()).collect::<Vec<_>>(),expected);
        let mut oracle=serde_json::Map::new();for row in law["entries"].as_array().unwrap(){oracle.insert(row["id"].as_str().unwrap().to_owned(),row.clone());}for(id,value)in &owner {let row=&oracle[&id.0];match value {InputReplacement::Withdrawn=>assert!(row["withdrawn"].as_bool().unwrap()),InputReplacement::Input {schema,payload}=>{assert_eq!(schema,row["schema"].as_str().unwrap());assert_eq!(payload.iter().copied().map(u64::from).collect::<Vec<_>>(),row["payload"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()).collect::<Vec<_>>());}}}
        let original=pointers(&owner);assert_eq!(owner.rows.len(),law["entries"].as_array().unwrap().len());let row_backing=owner.rows.allocated_bytes();let mut taken=Vec::with_capacity(expected.len()+11);let taken_capacity=taken.capacity()*size_of::<(MutationId,InputReplacement)>();
        for _ in 0..cancel.as_u64().unwrap(){let(pair,heap)=observe_heap_allocations_on_this_thread(||owner.pop_first());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some(pair)=pair {taken.push(pair);}else{break;}}
        assert!(!owner.terminal_is_empty());assert_eq!(owner.rows.allocated_bytes(),row_backing);for(index,pointer)in original.iter().enumerate(){if let Some(current)=owner.rows.get(index).unwrap(){if let InputReplacement::Input {schema,payload}=current {assert_eq!(Some((schema.as_ptr(),payload.as_ptr(),schema.capacity(),payload.capacity())),*pointer);}}}
        close((owner,taken),held+taken_capacity,copy.as_u64().unwrap()as usize);
    }}
    println!("[DEBUG] native draft sorted replacement agrees with independent JSON; displaced inputs and empty/unused schema-payload capacities remain original; ordered pop0heap; five-axis refusal0effects; every birth/release physical; terminal Drop0heap");
}
