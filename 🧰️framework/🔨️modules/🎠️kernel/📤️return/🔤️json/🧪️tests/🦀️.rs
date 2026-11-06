use super::*;
use semio_framework_pack_json::{JsonBorrowedSinkCursor,JsonBorrowNoContext,JsonNativeSource};
use semio_framework_value::{RetainedCloneGrant,RetirementDemand,NativeEncodeControl};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn write(source:&impl JsonWriteSource)->String{
 let mut cursor=JsonBorrowedSinkCursor::empty(64).unwrap();let mut observer=|_|true;let mut control=NativeEncodeControl::new(100000,&mut observer);let mut output=Vec::new();
 for _ in 0..100000{if cursor.is_complete(){break}let mut bytes=[0;7];let g=grant(cursor.next_demands(bytes.len(),4096).unwrap());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(source,&mut bytes,g,1,&mut control));let step=step.result.unwrap();assert!(step.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress.retained_capacity_bytes,step.progress.released_bytes));output.extend_from_slice(&bytes[..step.written_bytes]);}
 assert!(cursor.is_complete());for _ in 0..100000{if cursor.terminal_is_empty(){break}let g=grant(cursor.close_demands(4096).unwrap());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(g));let p=step.result.unwrap().progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}assert!(cursor.terminal_is_empty());String::from_utf8(output).unwrap()
}
struct Root<'a,T:KernelJson>(&'a T);
impl<T:KernelJson> JsonNativeBorrow for Root<'_,T>{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{self.0.json_node(m,c)}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{self.0.json_child(m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{self.0.json_key(m,i,c)}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{self.0.json_edge(m,i,cursor,c)}
}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🔣️.json")).unwrap()}
#[test]
fn retained_turn_borrows_the_actual_full_wire_graph_and_physically_closes_the_sink(){
 let _arena=semio_ui_contract_values::testkit::UiArenaFixture::new();let fixture=fixture();let row=&fixture["cases"][0];let input=serde_json::to_string(&row["input"]).unwrap();let turn:TurnResult=serde_json::from_str(&input).unwrap();let expected=row["expected"].as_str().unwrap();assert_eq!(serde_json::to_string(&Ok::<_,()>(&turn)).unwrap(),expected);let source=KernelTurnJsonSource::try_borrow(&turn).unwrap().unwrap();assert_eq!(write(&source),expected);
}
#[test]
fn ui_values_components_and_patch_pages_match_serde_without_cloning_the_retained_payload(){
 let _arena=semio_ui_contract_values::testkit::UiArenaFixture::new();let fixture=fixture();
 for row in fixture["cases"].as_array().unwrap().iter().skip(1){let expected=row["expected"].as_str().unwrap();let text=serde_json::to_string(&row["input"]).unwrap();match row["kind"].as_str().unwrap(){
  "uiMap"=>{let value:semio_ui_contract::UiValue=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let context=KernelJsonContext{arena:ui_value_arena_registry().borrow().unwrap().unwrap()};let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&context}),expected);},
  "uiComponent"=>{let value:semio_ui_contract::Component=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  "patch"=>{let value:semio_ui_contract::UiPatch=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  "numberBound"=>{let value:semio_ui_contract::UiNumberBound=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  "rowAction"=>{let value:semio_ui_contract::RowAction=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  "decimal"=>{let value:semio_framework_kernel_identity::ArtifactHandle=serde_json::from_str(&text).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),expected);let root=Root(&value);assert_eq!(write(&JsonNativeSource{root:&root,context:&JsonBorrowNoContext}),expected);},
  _=>panic!("unexpected shared output fixture kind"),
 }}
}
