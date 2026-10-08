use super::*;
use pack::value::{retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::controlled::ControlledRetirement};

#[test]
fn component_reference_table_preserves_original_labels_and_wire_map_then_retires_exact_backings(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","import Ajv from 'ajv';const x=JSON.parse(await Bun.stdin.text());const valid=new Ajv({strict:true}).compile(x.schema);for(const table of x.fixture.tables)if(!valid(table))throw Error('schema');console.log(JSON.stringify(x.fixture.tables.map(table=>Object.entries(table).sort(([a],[b])=>a<b?-1:a>b?1:0))));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=oracle.wait_with_output().unwrap();assert!(output.status.success());let reference:Vec<Vec<(String,Vec<String>)>>=serde_json::from_slice(&output.stdout).unwrap();
    for row in reference{for work in fixture["copyGrants"].as_array().unwrap(){let work=work.as_u64().unwrap()as usize;
        let entries=row.clone();let original=entries.capacity()*std::mem::size_of::<(String,Vec<String>)>()+entries.iter().map(|(key,labels)|key.capacity()+labels.capacity()*std::mem::size_of::<String>()+labels.iter().map(String::capacity).sum::<usize>()).sum::<usize>();let pointers=entries.iter().map(|(_,labels)|labels.iter().map(|s|s.as_ptr()).collect::<Vec<_>>()).collect::<Vec<_>>();
        let(table,heap)=observe_retirement_allocations(||ComponentReferenceTable::from_entries(entries));assert_eq!(heap,(0,0));assert_eq!(table.iter().map(|(key,labels)|(key.clone(),labels.clone())).collect::<Vec<_>>(),row);for((_,labels),pointers)in table.iter().zip(pointers){assert_eq!(labels.iter().map(|s|s.as_ptr()).collect::<Vec<_>>(),pointers);}
        let third_party=row.clone().into_iter().collect::<std::collections::BTreeMap<_,_>>();let wire=semio_framework_pack_json::from_dsl_value(&pack::value::ToValue::to_value(&table));let actual:serde_json::Value=serde_json::from_str(&wire.to_string()).unwrap();assert_eq!(actual,serde_json::to_value(&third_party).unwrap());
        let(mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(table).unwrap_or_else(|_|panic!("component table typed authority")));assert_eq!(heap,(0,0));let mut born=0;let mut physical=0;
        for _ in 0..100000{if owner.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:owner.next_capacity_byte_demand(work).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;physical+=heap.1;assert!(!matches!(step,RetainedCloneStep::Complete(_))||owner.terminal_is_empty());}
        assert!(owner.terminal_is_empty());assert_eq!(original+born,physical);println!("[DEBUG] Component reference table work={work} original={original} born={born} physical={physical}");
    }}
}

struct ObservedAllocator;
thread_local! { static RETIREMENT_ALLOCATION_EVENTS: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) }; }
fn retirement_allocation_event(requested: usize, released: usize) { let _ = RETIREMENT_ALLOCATION_EVENTS.try_with(|events| { if let Some((allocated, freed)) = events.get() { events.set(Some((allocated + requested, freed + released))); } }); }
pub(crate) fn observe_retirement_allocations<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    RETIREMENT_ALLOCATION_EVENTS.with(|events| { assert!(events.replace(Some((0, 0))).is_none()); });
    let result = operation();
    let events = RETIREMENT_ALLOCATION_EVENTS.with(|events| events.replace(None).unwrap());
    (result, events)
}
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{let pointer=unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{let pointer=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{let grown=unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)};if !grown.is_null(){retirement_allocation_event(size,layout.size());}grown}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){retirement_allocation_event(0,layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;
