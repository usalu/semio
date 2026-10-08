use std::sync::{Arc,atomic::{AtomicU8,AtomicBool,AtomicUsize,Ordering}};
use std::alloc::{GlobalAlloc,Layout,System};
use std::collections::{HashMap,HashSet};
struct Heap;
static OBSERVING:AtomicBool=AtomicBool::new(false);
static ALLOCATED:AtomicUsize=AtomicUsize::new(0);
static RELEASED:AtomicUsize=AtomicUsize::new(0);
unsafe impl GlobalAlloc for Heap {
 unsafe fn alloc(&self,layout:Layout)->*mut u8 { let pointer=System.alloc(layout);if OBSERVING.load(Ordering::Relaxed)&&!pointer.is_null(){ALLOCATED.fetch_add(layout.size(),Ordering::Relaxed);}pointer }
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){if OBSERVING.load(Ordering::Relaxed){RELEASED.fetch_add(layout.size(),Ordering::Relaxed);}System.dealloc(pointer,layout)}
 unsafe fn realloc(&self,pointer:*mut u8,old:Layout,new:usize)->*mut u8 {let result=System.realloc(pointer,old,new);if OBSERVING.load(Ordering::Relaxed)&&!result.is_null(){ALLOCATED.fetch_add(new,Ordering::Relaxed);RELEASED.fetch_add(old.size(),Ordering::Relaxed);}result}
}
#[global_allocator]static HEAP:Heap=Heap;
fn observe<T>(run:impl FnOnce()->T)->(T,usize,usize){ALLOCATED.store(0,Ordering::Relaxed);RELEASED.store(0,Ordering::Relaxed);OBSERVING.store(true,Ordering::Relaxed);let result=run();OBSERVING.store(false,Ordering::Relaxed);(result,ALLOCATED.load(Ordering::Relaxed),RELEASED.load(Ordering::Relaxed))}
pub(crate) struct ArtifactGroupVisibility{state:AtomicU8}
impl ArtifactGroupVisibility{fn pending(&self)->bool{self.state.load(Ordering::Acquire)==0}fn committed(&self)->bool{self.state.load(Ordering::Acquire)==1}}
mod os_vcs{pub(crate) use crate::ArtifactGroupVisibility;}
#[derive(Clone,Copy)]struct ArtifactStoreOneItemGrant{maximum_items:usize,maximum_bytes:usize}
#[derive(Debug,PartialEq)]enum SnapshotRetirementStep{Pending{released_items:usize,released_bytes:usize},Complete}
const OWNED_DOCUMENT_MAXIMUM_MEMBERS:usize=1024;
#[path = @MODULE@]
mod composition_group;
use composition_group::{GroupOwnsPreparation,GroupOwnsStep,GroupOwnsError};
struct CompositionGraph{owns:composition_group::OwnsRoot,owns_group:Option<usize>,owns_authority:Arc<()>,owns_generation:u64,owns_retiring:Option<(String,(String,String))>,links:HashMap<String,HashSet<String>>,retiring:Vec<Vec<u8>>}
impl CompositionGraph { @COLD_CLOSE@ }
fn graph()->CompositionGraph{CompositionGraph{owns:Default::default(),owns_group:None,owns_authority:Arc::new(()),owns_generation:0,owns_retiring:None,links:HashMap::new(),retiring:Vec::new()}}
fn close(graph:&mut CompositionGraph,preparation:&mut GroupOwnsPreparation){for _ in 0..10000{let demand=preparation.next_close_byte_demand();if demand!=0{let (step,a,r)=observe(||graph.close_owns_group(preparation,ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:demand-1}));assert_eq!((a,r),(0,0));assert_eq!(step,SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});assert_eq!(preparation.next_close_byte_demand(),demand);}let (step,a,r)=observe(||graph.close_owns_group(preparation,ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:demand}));assert_eq!(a,0);match step{SnapshotRetirementStep::Complete=>{assert_eq!(r,0);return},SnapshotRetirementStep::Pending{released_bytes,..}=>assert_eq!(r,released_bytes)}}panic!("terminal close")}
fn edge(graph:&CompositionGraph,preparation:&mut GroupOwnsPreparation,row:(&str,&str,&str))->Result<usize,GroupOwnsError>{for turn in 1..10000{let demand=preparation.next_edge_byte_demand(graph,row.0,row.1,row.2)?;let result=graph.prepare_owns_group_edge(preparation,row.0,row.1,row.2,ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:demand})?;if result==GroupOwnsStep::EdgePrepared{return Ok(turn)}}panic!("edge finite")}
fn main(){
 let rows:&[(&str,&str,&str)]=@SIBLINGS@;
 let mut graph=graph();graph.owns.insert("before".into(),("prior".into(),"old".into()));
 let visibility=Arc::new(ArtifactGroupVisibility{state:AtomicU8::new(0)});
 let (result,a,r)=observe(||graph.begin_owns_group(&visibility,rows.len()));assert_eq!((a,r),(0,0));let mut preparation=result.unwrap();
 for row in rows.iter().rev(){edge(&graph,&mut preparation,*row).unwrap();assert_eq!(graph.owns.len(),1);}
 for _ in 0..10000{if graph.seal_owns_group(&mut preparation,ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:65536}).unwrap()==GroupOwnsStep::RootPrepared{break}}
 assert!(graph.owns_group_ready(&preparation));visibility.state.store(1,Ordering::Release);let (result,a,r)=observe(||graph.commit_owns_group(&mut preparation));assert_eq!(result,Ok(()));assert_eq!((a,r),(0,0));assert_eq!(graph.owns_generation,1);for row in rows{assert_eq!(graph.owns.get(row.2),Some(&(row.0.into(),row.1.into())))}close(&mut graph,&mut preparation);
 let (live_step,live_allocated,live_released)=observe(||graph.close_step(1,0));
 println!("[DEBUG] actual live ownership root zero-byte structural close allocated={live_allocated} released={live_released} step={live_step:?}");
 let expected_live=(live_allocated,live_released);
 for _ in 0..10000 { let demand=graph.next_close_byte_demand();if demand>0 {let (step,a,r)=observe(||graph.close_step(1,demand-1));assert_eq!((a,r),(0,0));assert_eq!(step,SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}let (step,a,r)=observe(||graph.close_step(1,demand));assert_eq!(a,0);match step{SnapshotRetirementStep::Complete=>{assert_eq!(r,0);break},SnapshotRetirementStep::Pending{released_bytes,..}=>assert_eq!(r,released_bytes)} }
 assert_eq!(expected_live,(0,0),"moving one retained live row must not clone a key or allocate retirement storage");
 let initial:&[(&str,&str,&str)]=@INITIAL@;let cycle:(&str,&str,&str)=@CYCLE@;
 let mut graph=CompositionGraph{owns:Default::default(),owns_group:None,owns_authority:Arc::new(()),owns_generation:0,owns_retiring:None,links:HashMap::new(),retiring:Vec::new()};for row in initial{graph.owns.insert(row.2.into(),(row.0.into(),row.1.into()));}let visibility=Arc::new(ArtifactGroupVisibility{state:AtomicU8::new(0)});let mut preparation=graph.begin_owns_group(&visibility,1).unwrap();assert_eq!(edge(&graph,&mut preparation,cycle),Err(GroupOwnsError::Cycle));assert_eq!(graph.owns.len(),initial.len());visibility.state.store(2,Ordering::Release);close(&mut graph,&mut preparation);
 println!("[DEBUG] actual production graph module: sibling root zero-allocation commit, exact physical String/vector close,65-hop cycle retained; integrated kernel proof remains separate");
}
