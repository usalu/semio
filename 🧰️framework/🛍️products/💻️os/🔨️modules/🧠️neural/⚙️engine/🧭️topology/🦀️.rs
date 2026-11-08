//! 🧭️ Original tree topology preparation retains source identities and admitted backing.

use super::Tree;
use std::{borrow::Borrow,cmp::Ordering,mem::ManuallyDrop,sync::Arc};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementPollExit, ValueError, ValueRefusalKind, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close}, retirement::{RetireOwned, controlled::ControlledRetirement, arc_custody::{SharedControlledRetirement, shared_retirement_birth_bytes}}};
use semio_framework_value_persistent::{list::PagedList, list::PagedListError, list::PagedListReservationFailure, list::PagedListReservationOutcome};

const TOPOLOGY_ITEMS:usize=1_048_576;

struct TreeSourceLease(Arc<Tree>);
impl RetireOwned for TreeSourceLease {
    type RetirementCursor=SharedControlledRetirement<Tree>;
    fn retirement_seed(self)->Self::RetirementCursor{SharedControlledRetirement::lease(self.0)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(shared_retirement_birth_bytes::<Tree>())}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Clone)]
struct NodeKey {source:Arc<Tree>,index:usize}
impl Borrow<str> for NodeKey {fn borrow(&self)->&str{&self.source.neurons[self.index].id}}
impl PartialEq for NodeKey {fn eq(&self,other:&Self)->bool{self.cmp(other)==Ordering::Equal}}
impl Eq for NodeKey {}
impl PartialOrd for NodeKey {fn partial_cmp(&self,other:&Self)->Option<Ordering>{Some(self.cmp(other))}}
impl Ord for NodeKey {fn cmp(&self,other:&Self)->Ordering{self.source.neurons[self.index].id.cmp(&other.source.neurons[other.index].id)}}
impl RetireOwned for NodeKey {
    type RetirementCursor=SharedControlledRetirement<Tree>;
    fn retirement_seed(self)->Self::RetirementCursor{SharedControlledRetirement::lease(self.source)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(shared_retirement_birth_bytes::<Tree>())}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Clone,Copy,Default)]
struct TopologyRow {incoming:usize,outgoing:Option<usize>}
#[derive(Clone,Copy)]
struct TopologyEdge {target:usize,next:Option<usize>}
semio_framework_value::artifact_retire_leaf!(TopologyRow);
semio_framework_value::artifact_retire_leaf!(TopologyEdge);

#[derive(Default,semio_framework_value::RetireOwned)]
struct TopologyOwners {
    nodes:HistoryFoldIndex<NodeKey,usize>,
    rows:PagedList<TopologyRow,TOPOLOGY_ITEMS>,
    edges:PagedList<TopologyEdge,TOPOLOGY_ITEMS>,
    ready:HistoryFoldIndex<NodeKey,()>,
    next_ready:HistoryFoldIndex<NodeKey,()>,
    order:PagedList<usize,TOPOLOGY_ITEMS>,
    selected:Option<NodeKey>,
    source:Option<TreeSourceLease>,
}

impl TopologyOwners {
    fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.selected.is_none()&&self.nodes.terminal_is_empty()&&self.ready.terminal_is_empty()&&self.next_ready.terminal_is_empty()&&self.rows.capacity()==0&&self.edges.capacity()==0&&self.order.capacity()==0}
}

/// 🧵️ Retains the original topology input and every normal or closing cursor allocation.
pub struct BudgetedEvalTopology {
    owners:ManuallyDrop<TopologyOwners>,
    retirement:Option<ControlledRetirement<TopologyOwners>>,
    phase:u8,
    cursor:usize,
    node_has_key:bool,
    edge_cursor:Option<usize>,
    ready_candidate:Option<usize>,
    closing:bool,
    progress:RetainedCloneProgress,
}

fn page_error(error:PagedListError)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason)}

impl BudgetedEvalTopology {
    /// 🌱️ Takes the exact preborn original tree without decomposing or cloning its payload.
    pub fn new(source:Arc<Tree>)->Self{Self{owners:ManuallyDrop::new(TopologyOwners{source:Some(TreeSourceLease(source)),..Default::default()}),retirement:None,phase:0,cursor:0,node_has_key:false,edge_cursor:None,ready_candidate:None,closing:false,progress:Default::default()}}
    pub fn source(&self)->Option<&Arc<Tree>>{self.owners.source.as_ref().map(|source|&source.0)}
    pub fn is_complete(&self)->bool{self.phase==5}
    pub fn order(&self)->impl Iterator<Item=&usize>{self.owners.order.iter()}
    pub fn order_at(&self,index:usize)->Option<usize>{self.owners.order.get(index).copied()}
    fn key(&self,index:usize)->NodeKey{NodeKey{source:self.source().unwrap().clone(),index}}
    fn page_demands<T>(list:&PagedList<T,TOPOLOGY_ITEMS>,extent:usize)->Result<RetirementDemand,ValueError>{
        Ok(RetirementDemand{capacity_bytes:list.next_exact_capacity_allocation_bytes(extent).map_err(page_error)?.unwrap_or(0),depth:if list.has_reserved_slot(){1}else{list.next_reserve_depth_demand().map_err(page_error)?},..Default::default()})
    }
    fn index_demands<V>(&self,index:&HistoryFoldIndex<NodeKey,V>,key:&NodeKey,copy:usize)->Result<RetirementDemand,ValueError>{
        Ok(RetirementDemand{copy_bytes:index.next_insert_copy_byte_demand(key)?,capacity_bytes:index.next_insert_capacity_byte_demand(key,copy)?,release_bytes:index.next_insert_release_byte_demand(key)?,depth:index.next_insert_depth_demand(key)?})
    }
    fn read_depth(&self)->Result<usize,ValueError>{self.owners.nodes.first_key_value().map_or(Ok(1),|(key,_)|self.owners.nodes.next_insert_depth_demand(key))}
    fn edge_endpoints(&self)->Option<(usize,usize)>{let edge=self.source()?.synapses.get(self.cursor)?;Some((*self.owners.nodes.get(edge.from.as_str())?,*self.owners.nodes.get(edge.to.as_str())?))}

    /// 🪙️ Quotes each next actual backing currency from the existing source and cursor.
    pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology is closing"))}
        let source=self.source().unwrap();
        if source.neurons.len()>TOPOLOGY_ITEMS||source.synapses.len()>TOPOLOGY_ITEMS{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original topology exceeds its declared extent"))}
        match self.phase {
            0 if self.cursor<source.neurons.len()=>if self.node_has_key{Self::page_demands(&self.owners.rows,source.neurons.len())}else{self.index_demands(&self.owners.nodes,&self.key(self.cursor),copy)},
            1 if self.cursor<source.synapses.len()&&self.edge_endpoints().is_some()=>{let mut demand=Self::page_demands(&self.owners.edges,source.synapses.len())?;demand.depth=demand.depth.max(self.read_depth()?);Ok(demand)},
            2 if self.cursor<self.owners.rows.len()&&self.owners.rows[self.cursor].incoming==0=>self.index_demands(&self.owners.ready,&self.key(self.cursor),copy),
            3 if self.owners.selected.is_some()=>Self::page_demands(&self.owners.order,source.neurons.len()),
            3=>Ok(RetirementDemand{depth:self.owners.ready.first_key_value().map_or(Ok(1),|(key,_)|self.owners.ready.next_insert_depth_demand(key))?,..Default::default()}),
            4=>if let Some(index)=self.ready_candidate{self.index_demands(&self.owners.next_ready,&self.key(index),copy)}else{Ok(RetirementDemand{depth:1,..Default::default()})},
            5=>Ok(Default::default()),
            _=>Ok(RetirementDemand{depth:1,..Default::default()}),
        }
    }
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demands(0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.next_demands(copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demands(0)?.release_bytes)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demands(0)?.depth)}

    fn reserve_page<T>(list:&mut PagedList<T,TOPOLOGY_ITEMS>,extent:usize,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<RetainedCloneProgress,ValueError>{
        match list.reserve_exact_capacity_one(extent,effect).result {
            Ok(PagedListReservationOutcome::Denied)=>Ok(Default::default()),
            Ok(PagedListReservationOutcome::Ready(progress))=>Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:progress.allocated_bytes,..Default::default()}),
            Err(PagedListReservationFailure::Work(error))=>Err(error),
            Err(PagedListReservationFailure::Allocation{error,copied_bytes})=>{*receipt=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0||copied_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()};Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))}
        }
    }
    fn reserve_index<V>(index:&mut HistoryFoldIndex<NodeKey,V>,key:&NodeKey,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<Option<RetainedCloneProgress>,ValueError>{
        if index.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes)?==0{return Ok(None)}
        index.reserve_insert_step(key,grant).map(Some).map_err(|(error,progress)|{*receipt=progress;error})
    }

    /// ⏱️ Advances one original topology phase and preserves the complete physical receipt.
    pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        self.progress=Default::default();let result=self.step_source(effect).and_then(|step|{if effect.performed_work()==0&&step.progress()!=RetainedCloneProgress::default()&&!effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original topology event exceeds its work ceiling"))}Ok(step)});if let Ok(step)=&result{self.progress=step.progress();}result.map_err(|error|error.with_retained_progress(self.progress))
    }
    fn step_source(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        let grant=effect.grant();let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.next_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original topology exceeds admitted depth"))}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if self.is_complete(){return Ok(RetainedCloneStep::Complete(empty))}
        let mut progress=RetainedCloneProgress{copied_items:1,..empty};
        let nodes=self.source().unwrap().neurons.len();
        match self.phase {
            0 if self.cursor==nodes=>{self.phase=1;self.cursor=0;},
            0 if !self.node_has_key=>{
                let key=self.key(self.cursor);
                if self.owners.nodes.contains_key(Borrow::<str>::borrow(&key)){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original topology has duplicate node identity"))}
                if let Some(receipt)=Self::reserve_index(&mut self.owners.nodes,&key,grant,&mut self.progress)?{return Ok(RetainedCloneStep::Progress(receipt))}
                let (_,receipt)=self.owners.nodes.insert_reserved(key,self.cursor,grant).map_err(|(error,_,_)|error)?;progress=receipt;self.node_has_key=true;
            },
            0=>{
                if !self.owners.rows.has_reserved_slot(){return Self::reserve_page(&mut self.owners.rows,nodes,effect,&mut self.progress).map(RetainedCloneStep::Progress)}
                self.owners.rows.push_reserved(TopologyRow::default()).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology row reservation vanished"))?;self.cursor+=1;self.node_has_key=false;
            },
            1 if self.cursor==self.source().unwrap().synapses.len()=>{self.phase=2;self.cursor=0;},
            1=>{
                if let Some((from,to))=self.edge_endpoints(){
                    if !self.owners.edges.has_reserved_slot(){let extent=self.source().unwrap().synapses.len();return Self::reserve_page(&mut self.owners.edges,extent,effect,&mut self.progress).map(RetainedCloneStep::Progress)}
                    let index=self.owners.edges.len();let next=self.owners.rows[from].outgoing;
                    self.owners.edges.push_reserved(TopologyEdge{target:to,next}).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology edge reservation vanished"))?;
                    self.owners.rows[from].outgoing=Some(index);self.owners.rows[to].incoming=self.owners.rows[to].incoming.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original topology indegree overflow"))?;
                }
                self.cursor+=1;
            },
            2 if self.cursor==self.owners.rows.len()=>{self.phase=3;},
            2=>{
                if self.owners.rows[self.cursor].incoming==0{
                    let key=self.key(self.cursor);if let Some(receipt)=Self::reserve_index(&mut self.owners.ready,&key,grant,&mut self.progress)?{return Ok(RetainedCloneStep::Progress(receipt))}
                    let (_,receipt)=self.owners.ready.insert_reserved(key,(),grant).map_err(|(error,_,_)|error)?;progress=receipt;
                }
                self.cursor+=1;
            },
            3 if self.owners.selected.is_some()=>{
                if !self.owners.order.has_reserved_slot(){return Self::reserve_page(&mut self.owners.order,nodes,effect,&mut self.progress).map(RetainedCloneStep::Progress)}
                let index=self.owners.selected.as_ref().unwrap().index;self.owners.order.push_reserved(index).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology order reservation vanished"))?;self.edge_cursor=self.owners.rows[index].outgoing;self.phase=4;
            },
            3=>{
                if let Some((key,()))=self.owners.ready.pop_first(){self.owners.selected=Some(key);}
                else if !self.owners.next_ready.is_empty(){let owners=&mut*self.owners;std::mem::swap(&mut owners.ready,&mut owners.next_ready);}
                else if self.owners.order.len()==nodes{self.phase=5;}
                else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original topology contains a cycle"))}
            },
            4 if self.ready_candidate.is_some()=>{
                let key=self.key(self.ready_candidate.unwrap());if let Some(receipt)=Self::reserve_index(&mut self.owners.next_ready,&key,grant,&mut self.progress)?{return Ok(RetainedCloneStep::Progress(receipt))}
                let (_,receipt)=self.owners.next_ready.insert_reserved(key,(),grant).map_err(|(error,_,_)|error)?;progress=receipt;self.ready_candidate=None;
            },
            4=>{
                if let Some(index)=self.edge_cursor{let edge=self.owners.edges[index];let incoming=&mut self.owners.rows[edge.target].incoming;*incoming=incoming.checked_sub(1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology indegree underflow"))?;if *incoming==0{self.ready_candidate=Some(edge.target);}self.edge_cursor=edge.next;}
                else{self.owners.selected=None;self.phase=3;}
            },
            _=>unreachable!("original topology phase"),
        }
        Ok(if self.is_complete(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    /// 🧹️ Begins closure without changing any original backing owner.
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        Ok(RetirementDemand{depth:usize::from(!self.owners.terminal_is_empty()),..Default::default()})
    }
    /// ⛽️ Quotes the selected retirement child step or the one source handoff.
    pub fn next_close_work(&self,body:usize)->Result<u64,ValueError>{if let Some(owner)=self.retirement.as_ref(){return owner.next_computational_work(body)}Ok(u64::from(!self.owners.terminal_is_empty()))}
    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        self.progress=Default::default();let grant=effect.grant();
        let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
            if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology close was not begun").into())}
            if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
            let demand=self.next_close_demands(grant.maximum_copy_bytes)?;
            if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original topology close exceeds admitted depth").into())}
            if let Some(owner)=self.retirement.as_mut(){let result=RetirementPollExit::resume(owner.step(effect));self.progress=owner.step_progress();let step=result?;let step=admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"Neural original topology source")?;if owner.terminal_is_empty(){self.retirement=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
            if self.owners.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original topology source handoff exceeds its work ceiling").into())}
            self.retirement=Some(ControlledRetirement::retain(std::mem::take(&mut*self.owners)));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
        })();
        crate::settle_close(effect,result,&mut self.progress)
    }
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.retirement.is_none()&&self.owners.terminal_is_empty()}
}

impl Drop for BudgetedEvalTopology {
    fn drop(&mut self){if !self.terminal_is_empty(){assert!(std::thread::panicking(),"original topology must finish explicit close before Drop");return;}unsafe{ManuallyDrop::drop(&mut self.owners);}}
}
/// 🧹️ Owns a closing topology as its retirement cursor.
pub struct TopologyRetirement(BudgetedEvalTopology);
impl semio_framework_value::retirement::RetirementCursor for TopologyRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.0.next_close_demands(body) }
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_close_work(body)}
    fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{semio_framework_value::retirement::erased_cursor_turn(self.0.close_step(effect))}
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for BudgetedEvalTopology {
    type RetirementCursor=TopologyRetirement;
    fn retirement_seed(mut self)->TopologyRetirement{self.begin_close();TopologyRetirement(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TopologyRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
