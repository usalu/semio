use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

#[test]
fn original_projection_retirement_keeps_the_exact_issuer_and_all_physical_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let registry=store::SnapshotReadRegistryHandle::new();let source=Arc::new(row["owner"].as_str().unwrap().to_owned());let pointer=Arc::as_ptr(&source);
  let lease=registry.try_issue(Arc::clone(&source)).unwrap_or_else(|_|panic!("original projection read issue failed"));let(index,generation)=(lease.index,lease.generation);
  let read=store::SnapshotRead::new(Arc::clone(&source),lease);let mut root=SnapshotReadProjection::new(Arc::clone(&source));
  assert!(!root.has_original_issuer());assert!(root.bind(read).is_ok());assert!(root.has_original_issuer());assert_eq!(root.get()as*const String,pointer);
  let mut cursor=Cursor{root,active:ManuallyDrop::new(None)};
  let (denied,heap)=observe(||cursor.step(RetainedCloneGrant::default()).unwrap());assert_eq!(denied,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.root.get()as*const String,pointer);
  let (receipt,heap)=observe(||cursor.step(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap());assert_eq!(receipt.copied_items,1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(cursor.root.projection.is_none());assert_eq!(cursor.root.read.as_ref().unwrap().get::<String>().unwrap()as*const String,pointer);
  let mut born=0;let mut released=0;let mut pumped=false;
  for _ in 0..1024{
   if cursor.terminal_is_empty(){break;}
   if registry.returned.load(std::sync::atomic::Ordering::Acquire)!=0{
    let (result,heap)=observe(||registry.try_admit_one_returned::<String,Arc<String>>(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},|original,_|Ok((original,Default::default()))).unwrap());let original=result.0.unwrap();assert_eq!(Arc::as_ptr(&original),pointer);assert_eq!(original.as_str(),row["owner"].as_str().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(observe(||drop(original)).1.released_bytes,0);pumped=true;
   }
   let demand=cursor.demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for axis in 0..5{let mut denied=grant;let active=match axis{0=>{denied.maximum_items=0;true},1=>{denied.maximum_copy_bytes=demand.copy_bytes.saturating_sub(1);demand.copy_bytes>0},2=>{denied.maximum_capacity_bytes=demand.capacity_bytes.saturating_sub(1);demand.capacity_bytes>0},3=>{denied.maximum_release_bytes=demand.release_bytes.saturating_sub(1);demand.release_bytes>0},_=>{denied.maximum_depth=demand.depth.saturating_sub(1);demand.depth>0}};if active{let(result,heap)=observe(||cursor.step(denied));if let Ok(receipt)=result{assert_eq!(receipt,Default::default());}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}}
   let(receipt,heap)=observe(||cursor.step(grant).unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
  }
  assert!(cursor.terminal_is_empty());assert!(pumped);assert!(!registry.contains(index,generation));assert!(registry.terminal_is_empty());assert_eq!(born,released);assert_eq!(observe(||drop(cursor)).1.released_bytes,0);
  assert_eq!(serde_json::json!({"projectionHeapRelease":0,"originalPreserved":source.as_str()==row["owner"].as_str().unwrap(),"returnedToOriginalIssuer":pumped}),row["expected"]);
 }
 eprintln!("[DEBUG] original draft/transient projection closes through its exact Store read issuer with original pointer and physical receipts");
}
