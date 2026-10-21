//! 🔒️ Immutable canonical rows lend their original addresses and close every funded output frontier.
use super::{ArtifactCanonicalJsonTree as Tree,ArtifactCanonicalJsonTreeCursor as Cursor,Node,ArtifactCanonicalJsonText as Text,RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError};
use semio_framework_value::{shared_issuance::SealedShared, retirement::RetireOwned, shared_issuance::SharedIssuer, retained_clone::RetainedCloneSource};
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
#[derive(semio_framework_value::RetireOwned)]
struct Row{id:usize,label:&'static str}
impl Clone for Row{fn clone(&self)->Self{panic!("canonical immutable row must remain borrowed")}}
impl Tree for Row{
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.id),1=>Ok(&self.label),_=>Err(super::refusal("immutable row child is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("id".into()),1=>Ok("label".into()),_=>Err(super::refusal("immutable row key is absent"))}}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn sealed<T:RetireOwned+Send+Sync+'static>(value:T)->SealedShared<T>{let d=RetirementDemand{copy_bytes:std::mem::size_of::<T>()+std::mem::size_of::<SealedShared<T>>(),capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()};SealedShared::admit(value,SharedIssuer::owned(),grant(d)).unwrap_or_else(|(error,_)|panic!("original immutable fixture row:{error}")).0}
fn table(n:usize)->PersistentList<SealedShared<Row>>{
 if n==0{return Default::default();}
 let mut leaves=Vec::new();
 for first in (0..n).step_by(16){let len=(n-first).min(16);let rows=std::array::from_fn(|i|if i<len{Some(sealed(sealed(Row{id:first+i,label:"β"})))}else{None});leaves.push(PersistentList::admit_leaf(rows,len,grant(PersistentList::<SealedShared<Row>>::page_birth_demand())).unwrap_or_else(|(error,_)|panic!("original immutable fixture page:{error}")).0);}
 if leaves.len()==1{return leaves.pop().unwrap();}
 let mut leaves=leaves.into_iter();let children=std::array::from_fn(|_|leaves.next());PersistentList::admit_branch(children,grant(PersistentList::<SealedShared<Row>>::page_birth_demand())).unwrap_or_else(|(error,_)|panic!("original immutable fixture branch:{error}")).0
}
fn source<T:RetireOwned+Send+Sync+'static>(value:T)->RetainedCloneSource<T>{let birth=RetainedCloneSource::<T>::owned_constructor_demand::<()>();let d=RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()};RetainedCloneSource::admit_owned(value,(),grant(d)).unwrap_or_else(|(error,_,_)|panic!("original immutable tree source:{error}")).0}
fn close<T:RetireOwned+Send+Sync+'static>(source:&mut RetainedCloneSource<T>){for turn in 0..100000{if source.terminal_is_empty(){return;}let first=source.next_close_demand(4096).unwrap();let d=source.next_close_demand(first.copy_bytes.max(4096)).unwrap();let g=grant(d);let(step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999,"original immutable source closure");}}
/// 👁️ Native kind, key and child views retain every actual row address with no payload Clone or heap work.
#[test]
fn canonical_immutable_table_views_preserve_original_rows_and_zero_heap(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){let n=case["rows"].as_u64().unwrap()as usize;let mut original=source(table(n));let tree=original.borrow().get();let(_,heap)=observe_heap_allocations_on_this_thread(||{assert!(matches!(tree.canonical_tree_node().unwrap(),Node::Array(len)if len==n));for i in 0..n{let root=tree.get(i).unwrap();let child=tree.canonical_tree_child(i).unwrap();assert_eq!(child as*const dyn Tree as*const (),root as*const SealedShared<Row> as*const ());assert!(matches!(child.canonical_tree_node().unwrap(),Node::Object(2)));assert_eq!(child.canonical_tree_child(0).unwrap()as*const dyn Tree as*const (),&root.get().id as*const usize as*const ());assert!(matches!(child.canonical_tree_key(1).unwrap(),Text::Contiguous("label")));}assert!(tree.canonical_tree_child(n).is_err());});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));close(&mut original);assert_eq!(observe_heap_allocations_on_this_thread(||drop(original)).1.released_bytes,0);eprintln!("[DEBUG] canonical immutable rows{n}: original child/scalar addresses, borrowed kind/key heap0, payloadClone0");}
}
/// 🧭️ Every normal output frontier can cancel while original sources and all physical receipts remain conserved.
#[test]
fn canonical_immutable_table_every_frontier_matches_wire_oracle_and_closes(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){let n=case["rows"].as_u64().unwrap()as usize;let oracle=case["output"].as_str().unwrap().as_bytes();let mut complete_turns=None;
 for cut in 0..10000{if complete_turns.is_some_and(|turns|cut>turns){break;}let mut original=source(table(n));let projection_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:original.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()};let projection=original.project_owned(0,|value|value as&dyn Tree,projection_grant).unwrap().0;let(mut cursor,_)=Cursor::admit(projection,grant(Cursor::constructor_demand())).unwrap_or_else(|(error,_)|panic!("immutable canonical cursor:{error}"));let mut output=Vec::new();let mut turns=0;
 while !cursor.terminal_is_empty(){let d=cursor.next_demand().unwrap();let g=grant(d);for axis in 0..5{let mut denied=g;let active=match axis{0=>{denied.maximum_items=0;true},1 if d.copy_bytes>0=>{denied.maximum_copy_bytes=d.copy_bytes-1;true},2 if d.capacity_bytes>0=>{denied.maximum_capacity_bytes=d.capacity_bytes-1;true},3 if d.release_bytes>0=>{denied.maximum_release_bytes=d.release_bytes-1;true},4 if d.depth>0=>{denied.maximum_depth=d.depth-1;true},_=>false};if active{let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut[0;1],denied).unwrap());assert_eq!(step.ownership.progress(),Default::default());assert_eq!(step.written_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}}
 if turns==cut{cursor.begin_close();}let mut byte=[0;1];let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut byte,grant(cursor.next_demand().unwrap())).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.ownership.progress().retained_capacity_bytes,step.ownership.progress().released_bytes));output.extend_from_slice(&byte[..step.written_bytes]);turns+=1;assert!(turns<10000);}
 assert_eq!(output,oracle[..output.len()]);if cursor.is_complete(){assert_eq!(output,oracle);complete_turns=Some(complete_turns.map_or(turns,|prior:usize|prior.min(turns)));}close(&mut original);let(_,heap)=observe_heap_allocations_on_this_thread(||drop((cursor,original)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 assert!(complete_turns.is_some());eprintln!("[DEBUG] canonical immutable rows{n}: every original output frontier closes, independent JSON exact, 5-axis denial heap0");
 }
}
