use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
#[test]
fn retained_raw_admission_has_no_failure_destructor_or_undeclared_allocation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let raw=if row["maximumRawBytes"]=="usize-max"{usize::MAX}else{row["maximumRawBytes"].as_u64().unwrap() as usize};let work=row["maximumWorkItems"].as_u64().unwrap() as usize;
  let((mut buffer,refusal),heap)=observe(||admit_raw(raw,work));assert_eq!(heap.released_bytes,0);assert_eq!(heap.requested_bytes,row["expectedRawCapacity"].as_u64().unwrap() as usize);assert_eq!(buffer.capacity(),heap.requested_bytes);assert_eq!(refusal.map(ArtifactRetainedAdmissionRefusal::id),row["expectedRefusal"].as_str());
  let capacity=buffer.capacity();let(_,heap)=observe(||drop(std::mem::take(&mut buffer)));assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,capacity);
 }
 eprintln!("[DEBUG] retained raw admission rejects exact original capacity without source cleanup");
}
