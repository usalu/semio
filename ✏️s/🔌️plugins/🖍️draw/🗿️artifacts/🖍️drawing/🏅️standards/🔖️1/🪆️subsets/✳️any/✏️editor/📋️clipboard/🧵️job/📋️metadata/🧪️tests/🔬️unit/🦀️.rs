//! 🧪️ Metadata pages and keys are funded before their first allocation and survive logical removal.
use super::*;
#[test]
fn clipboard_metadata_replays_neutral_json_patch_membership_and_exact_backing(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧫️fixtures/🔣️.json")).unwrap();
    let (mut table,heap)=observe(Table::<()>::new);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let mut keys=Vec::<String>::new();
    for row in fixture["metadataCases"].as_array().unwrap(){let key=row["key"].as_str().unwrap();let admitted=table.admitted;let ((added,live),heap)=observe(||{let added=if row["operation"]=="insert"{table.insert_key(key).unwrap()}else{table.remove(key);false};(added,table.len())});assert_eq!(heap.released_bytes,0);assert_eq!(heap.requested_bytes,table.admitted-admitted);assert_eq!(added,row["added"].as_bool().unwrap());assert_eq!(live,row["live"].as_array().unwrap().len());if !keys.iter().any(|candidate|candidate==key){keys.push(key.into());}let mut actual=keys.iter().filter(|key|table.contains(key)).cloned().collect::<Vec<_>>();actual.sort();assert_eq!(actual,serde_json::from_value::<Vec<String>>(row["live"].clone()).unwrap());}
    let mut owner=ControlledRetirement::new(table).unwrap_or_else(|_|panic!("Metadata owns typed pages"));
    for _ in 0..10000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let (step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(heap.released_bytes,step.progress().released_bytes);}
    assert!(owner.terminal_is_empty());let (_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    eprintln!("[DEBUG] Clipboard metadata exact preallocated backing/key claims, retained tombstones and physical terminal retirement");
}
