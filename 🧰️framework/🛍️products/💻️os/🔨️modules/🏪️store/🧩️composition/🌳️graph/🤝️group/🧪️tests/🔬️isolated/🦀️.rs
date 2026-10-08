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
#[derive(Clone,Copy)]struct ArtifactStoreOneItemGrant{maximum_items:usize,maximum_copy_bytes:usize,maximum_capacity_bytes:usize,maximum_release_bytes:usize,maximum_depth:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct RetirementDemand{pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
extern crate self as semio_framework_value;
#[derive(Debug)]pub struct ValueError;
#[derive(Debug)]pub enum ValueRefusalKind { UnsupportedOwner }
impl ValueError { pub fn literal(_:ValueRefusalKind,_:&str)->Self { Self } }
pub mod retained_clone {
 #[derive(Clone,Copy,Default,Debug,PartialEq,Eq)]pub struct RetainedCloneGrant { pub maximum_items:usize,pub maximum_copy_bytes:usize,pub maximum_capacity_bytes:usize,pub maximum_release_bytes:usize,pub maximum_depth:usize }
 #[derive(Clone,Copy,Default,Debug,PartialEq,Eq)]pub struct RetainedCloneProgress {pub copied_items:usize,pub copied_bytes:usize,pub retained_capacity_bytes:usize,pub released_bytes:usize}
 impl RetainedCloneProgress { pub fn fits(self,g:RetainedCloneGrant)->bool {self.copied_items<=g.maximum_items&&self.copied_bytes<=g.maximum_copy_bytes&&self.retained_capacity_bytes<=g.maximum_capacity_bytes&&self.released_bytes<=g.maximum_release_bytes} }
 #[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum RetainedCloneStep {Progress(RetainedCloneProgress),Complete(RetainedCloneProgress)}
 impl RetainedCloneStep {pub fn progress(self)->RetainedCloneProgress { match self {Self::Progress(p)|Self::Complete(p)=>p} }}
}
use retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
mod test_allocation {pub fn observe_backing<T>(run:impl FnOnce()->T)->(T,usize,usize){crate::observe(run)}}
const OWNED_DOCUMENT_MAXIMUM_MEMBERS:usize=1024;
#[path = @MODULE@]
mod composition_group;
#[path = @LINK_MODULE@]
mod composition_links;
use composition_group::{GroupOwnsPreparation,GroupOwnsStep,GroupOwnsError};
struct CompositionGraph{owns:composition_group::OwnsRoot,owns_group:Option<usize>,owns_authority:Option<Arc<()>>,owns_generation:u64,owns_retiring:Option<(String,(String,String))>,links:composition_links::LinkRoot}
impl CompositionGraph { @COLD_CLOSE@
 fn terminal_is_empty(&self)->bool {self.owns_group.is_none()&&self.owns.is_empty()&&self.owns.capacity()==0&&self.owns_retiring.is_none()&&self.owns_authority.is_none()&&self.links.capacity()==0}
}
fn graph()->CompositionGraph{CompositionGraph{owns:Default::default(),owns_group:None,owns_authority:Some(Arc::new(())),owns_generation:0,owns_retiring:None,links:Default::default()}}
fn seal_root(graph: &CompositionGraph, preparation: &mut GroupOwnsPreparation) -> Result<GroupOwnsStep, GroupOwnsError> {
    let demand=preparation.next_preparation_demands(graph);let grant=preparation_grant(demand);
    for refused in [ArtifactStoreOneItemGrant{maximum_items:0,..grant},ArtifactStoreOneItemGrant{maximum_depth:0,..grant},ArtifactStoreOneItemGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(1),..grant},ArtifactStoreOneItemGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}] {
        if refused.maximum_items==grant.maximum_items&&refused.maximum_depth==grant.maximum_depth&&refused.maximum_copy_bytes==grant.maximum_copy_bytes&&refused.maximum_capacity_bytes==grant.maximum_capacity_bytes{continue;}
        let(answer,allocated,released)=crate::test_allocation::observe_backing(||graph.seal_owns_group(preparation,refused));assert_eq!(answer,Ok(GroupOwnsStep::Blocked));assert_eq!((allocated,released),(0,0));assert_eq!(preparation.next_preparation_demands(graph),demand);
    }
    let(answer,allocated,released)=crate::test_allocation::observe_backing(||graph.seal_owns_group(preparation,grant));assert_eq!(allocated,demand.capacity_bytes);assert_eq!(released,0);assert!(demand.copy_bytes<=4096);answer
}
fn preparation_grant(demand: semio_framework_value::RetirementDemand) -> ArtifactStoreOneItemGrant { ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth } }
fn group_grant(preparation: &GroupOwnsPreparation) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: preparation.next_close_copy_byte_demand(), maximum_capacity_bytes: 0, maximum_release_bytes: preparation.next_close_release_byte_demand(), maximum_depth: preparation.next_close_depth_demand() } }
fn close_graph(graph: &mut CompositionGraph) {
    for _ in 0..10000 {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: graph.next_close_copy_byte_demand().unwrap(), maximum_capacity_bytes: graph.next_close_capacity_byte_demand(0).unwrap(), maximum_release_bytes: graph.next_close_release_byte_demand().unwrap(), maximum_depth: graph.next_close_depth_demand().unwrap() };
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_step(grant));
        let step = result.unwrap(); assert_eq!(allocated, 0); assert_eq!(released, step.progress().released_bytes); assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(graph.terminal_is_empty()); return; }
    }
    panic!("graph close finite");
}
@PRIVATE_GROUP_GRANT@
fn close_group(graph: &mut CompositionGraph, preparation: &mut GroupOwnsPreparation) {
    for _ in 0..10000 {
        let grant = group_grant(preparation);
        assert_eq!(private_group_grant(grant),grant,"private forwarding retains exact caller work/capacity/release/depth");
        for refused in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(1), ..grant }] {
            if refused == grant { continue; }
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_owns_group(preparation, refused));
            assert_eq!((allocated, released), (0, 0)); assert_eq!(step, RetainedCloneStep::Progress(Default::default())); assert_eq!(group_grant(preparation), grant);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_owns_group(preparation, grant));
        assert_eq!(allocated, 0); assert_eq!(released, step.progress().released_bytes); assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert_eq!(released, 0); assert!(preparation.terminal_is_empty()); return; }
    }
    panic!("ownership group reaches its exact terminal owner");
}

fn edge(graph:&CompositionGraph,preparation:&mut GroupOwnsPreparation,row:(&str,&str,&str))->Result<usize,GroupOwnsError>{
 for turn in 1..10000{
  let demand=preparation.next_edge_demands(graph,row.0,row.1,row.2)?;let grant=preparation_grant(demand);
  for refused in [ArtifactStoreOneItemGrant{maximum_items:0,..grant},ArtifactStoreOneItemGrant{maximum_depth:0,..grant},ArtifactStoreOneItemGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(1),..grant},ArtifactStoreOneItemGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}]{
   if refused.maximum_items==grant.maximum_items&&refused.maximum_depth==grant.maximum_depth&&refused.maximum_copy_bytes==grant.maximum_copy_bytes&&refused.maximum_capacity_bytes==grant.maximum_capacity_bytes{continue;}
   let(answer,allocated,released)=observe(||graph.prepare_owns_group_edge(preparation,row.0,row.1,row.2,refused));assert_eq!(answer,Ok(GroupOwnsStep::Blocked));assert_eq!((allocated,released),(0,0));assert_eq!(preparation.next_edge_demands(graph,row.0,row.1,row.2)?,demand);
  }
  let(answer,allocated,released)=observe(||graph.prepare_owns_group_edge(preparation,row.0,row.1,row.2,grant));assert_eq!(allocated,demand.capacity_bytes);assert_eq!(released,0);assert!(demand.copy_bytes<=4096);if answer?==GroupOwnsStep::EdgePrepared{return Ok(turn)}
 }panic!("edge finite")
}
fn main(){
 let rows:&[(&str,&str,&str)]=@SIBLINGS@;
 let mut graph=graph();graph.owns.insert("before".into(),("prior".into(),"old".into()));
 let visibility=Arc::new(ArtifactGroupVisibility{state:AtomicU8::new(0)});
 let (result,a,r)=observe(||graph.begin_owns_group(&visibility,rows.len()));assert_eq!((a,r),(0,0));let mut preparation=result.unwrap();
 for row in rows.iter().rev(){edge(&graph,&mut preparation,*row).unwrap();assert_eq!(graph.owns.len(),1);}
 for _ in 0..10000{if seal_root(&graph,&mut preparation).unwrap()==GroupOwnsStep::RootPrepared{break}}
 assert!(graph.owns_group_ready(&preparation));visibility.state.store(1,Ordering::Release);let (result,a,r)=observe(||graph.commit_owns_group(&mut preparation));assert_eq!(result,Ok(()));assert_eq!((a,r),(0,0));assert_eq!(graph.owns_generation,1);for row in rows{assert_eq!(graph.owns.get(row.2),Some(&(row.0.into(),row.1.into())))}close_group(&mut graph,&mut preparation);
 let bytes=graph.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 let (live_step,live_allocated,live_released)=observe(||graph.close_step(grant));assert_eq!(live_step.unwrap(),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));assert_eq!((live_allocated,live_released),(0,0));close_graph(&mut graph);
 println!("[DEBUG] actual forest row move copied={bytes} allocation0 release0; final Arc/String/Vec backing admitted separately");
 let initial:&[(&str,&str,&str)]=@INITIAL@;let cycle:(&str,&str,&str)=@CYCLE@;
 let mut graph=CompositionGraph{owns:Default::default(),owns_group:None,owns_authority:Some(Arc::new(())),owns_generation:0,owns_retiring:None,links:Default::default()};for row in initial{graph.owns.insert(row.2.into(),(row.0.into(),row.1.into()));}let visibility=Arc::new(ArtifactGroupVisibility{state:AtomicU8::new(0)});let mut preparation=graph.begin_owns_group(&visibility,1).unwrap();assert_eq!(edge(&graph,&mut preparation,cycle),Err(GroupOwnsError::Cycle));assert_eq!(graph.owns.len(),initial.len());visibility.state.store(2,Ordering::Release);close_group(&mut graph,&mut preparation);close_graph(&mut graph);
 println!("[DEBUG] actual production graph module: sibling root zero-allocation commit, exact physical String/vector close,65-hop cycle retained; integrated kernel proof remains separate");
 let row:(&str,&str,&str)=@EMPTY_ROW@;let mut zero=crate::graph();zero.owns.insert(row.2.into(),(row.0.into(),row.1.into()));let visibility=Arc::new(ArtifactGroupVisibility{state:AtomicU8::new(0)});let mut empty=zero.begin_owns_group(&visibility,0).unwrap();for _ in 0..10000{if seal_root(&zero,&mut empty).unwrap()==GroupOwnsStep::RootPrepared{break;}}assert!(zero.owns_group_ready(&empty));assert_eq!(zero.owns.len(),1);visibility.state.store(1,Ordering::Release);zero.commit_owns_group(&mut empty).unwrap();assert_eq!(zero.owns.get(row.2),Some(&(row.0.into(),row.1.into())));close_group(&mut zero,&mut empty);close_graph(&mut zero);println!("[DEBUG] empty additions preserve original root through exact copy/capacity admission and terminal closure");
 let links:&[(&str,&[&str])]=@LINKS@;
 let (mut linked,births,released)=observe(||{let mut linked=CompositionGraph{owns:Default::default(),owns_group:None,owns_authority:Some(Arc::new(())),owns_generation:0,owns_retiring:None,links:Default::default()};for (source,targets) in links{linked.links.insert((*source).into(),targets.iter().map(|target|(*target).to_owned()).collect());}linked});let births=births-released;
 let mut total=0;for _ in 0..10000{let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:linked.next_close_copy_byte_demand().expect("original adjacency needs native controlled owner"),maximum_capacity_bytes:linked.next_close_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:linked.next_close_release_byte_demand().unwrap(),maximum_depth:linked.next_close_depth_demand().unwrap()};assert!(grant.maximum_copy_bytes<=4096);let(step,allocated,released)=observe(||linked.close_step(grant));let step=step.unwrap();assert_eq!(allocated,0);assert_eq!(released,step.progress().released_bytes);assert!(step.progress().fits(grant));total+=released;if matches!(step,RetainedCloneStep::Complete(_)){break;}}assert!(linked.terminal_is_empty());assert_eq!(total,births);let(_,allocated,released)=observe(||drop(linked));assert_eq!((allocated,released),(0,0));println!("[DEBUG] actual original adjacency backing={births} exactRelease={total} terminalDrop0");
}
