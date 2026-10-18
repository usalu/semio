//! 🧭️ Reachability walk: the arena entity reference, the reach set with its marks and the resumable reachability job. See [`super`].

use super::*;

// #region ♻️Reachability
/// ♻️ One arena entity a [`Body::reachable_from`] walk can start from or land on — the traversal
/// root type for garbage collection (see [`Body::compact`]) and for the set of entities a live
/// engine handle keeps alive across a compaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityRef {
    Vertex(VertexId),
    Edge(EdgeId),
    Coedge(CoedgeId),
    Loop(LoopId),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId),
    Curve3(Curve3Id),
    Curve2(Curve2Id),
    Surface(SurfaceId),
}

semio_framework_value::artifact_retire_leaf!(EntityRef);

/// ♻️ The set of arena entities one [`Body::reachable_from`] walk visited, keyed by store — every
/// id NOT present here, for a store's current [`Store::ids`], is exactly what [`Body::compact`]
/// is safe to free.
#[derive(Clone, Debug, Default)]
pub struct ReachSet {
    pub vertices: HistoryFoldSet<VertexId>,
    pub edges: HistoryFoldSet<EdgeId>,
    pub coedges: HistoryFoldSet<CoedgeId>,
    pub loops: HistoryFoldSet<LoopId>,
    pub faces: HistoryFoldSet<FaceId>,
    pub shells: HistoryFoldSet<ShellId>,
    pub solids: HistoryFoldSet<SolidId>,
    pub curves3: HistoryFoldSet<Curve3Id>,
    pub curves2: HistoryFoldSet<Curve2Id>,
    pub surfaces: HistoryFoldSet<SurfaceId>,
}

semio_framework_value::artifact_retire_struct!(ReachSet {vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces});

impl ReachSet {
    fn mark_demand(&self,root:EntityRef,copy:usize)->Result<RetirementDemand,ValueError> {
        macro_rules! demand {($set:expr,$id:expr)=>{if $set.contains(&$id){Ok(RetirementDemand {depth:1,..Default::default()})}else{Ok(RetirementDemand {copy_bytes:$set.next_insert_copy_byte_demand(&$id)?,capacity_bytes:$set.next_insert_capacity_byte_demand(&$id,copy)?,release_bytes:$set.next_insert_release_byte_demand(&$id)?,depth:$set.next_insert_depth_demand(&$id)?})}}}
        match root {
            EntityRef::Vertex(id)=>demand!(self.vertices,id),
            EntityRef::Edge(id)=>demand!(self.edges,id),
            EntityRef::Coedge(id)=>demand!(self.coedges,id),
            EntityRef::Loop(id)=>demand!(self.loops,id),
            EntityRef::Face(id)=>demand!(self.faces,id),
            EntityRef::Shell(id)=>demand!(self.shells,id),
            EntityRef::Solid(id)=>demand!(self.solids,id),
            EntityRef::Curve3(id)=>demand!(self.curves3,id),
            EntityRef::Curve2(id)=>demand!(self.curves2,id),
            EntityRef::Surface(id)=>demand!(self.surfaces,id),
        }
    }
    fn mark_step(&mut self,root:EntityRef,grant:RetainedCloneGrant)->Result<(Option<bool>,RetainedCloneProgress),(ValueError,RetainedCloneProgress)> {
        macro_rules! mark {($set:expr,$id:expr)=>{if $set.contains(&$id){Ok((Some(false),RetainedCloneProgress{copied_items:1,..Default::default()}))}else if $set.next_insert_capacity_byte_demand(&$id,grant.maximum_copy_bytes).map_err(|error|(error,Default::default()))?!=0{$set.reserve_insert_step(&$id,grant).map(|receipt|(None,receipt))}else{$set.insert_reserved($id,grant).map(|(added,receipt)|(Some(added),receipt)).map_err(|(error,_)|(error,Default::default()))}}}
        match root {
            EntityRef::Vertex(id)=>mark!(self.vertices,id),
            EntityRef::Edge(id)=>mark!(self.edges,id),
            EntityRef::Coedge(id)=>mark!(self.coedges,id),
            EntityRef::Loop(id)=>mark!(self.loops,id),
            EntityRef::Face(id)=>mark!(self.faces,id),
            EntityRef::Shell(id)=>mark!(self.shells,id),
            EntityRef::Solid(id)=>mark!(self.solids,id),
            EntityRef::Curve3(id)=>mark!(self.curves3,id),
            EntityRef::Curve2(id)=>mark!(self.curves2,id),
            EntityRef::Surface(id)=>mark!(self.surfaces,id),
        }
    }
    fn clear_membership(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.coedges.clear();
        self.loops.clear();
        self.faces.clear();
        self.shells.clear();
        self.solids.clear();
        self.curves3.clear();
        self.curves2.clear();
        self.surfaces.clear();
    }
}

#[derive(Clone,Copy)]
struct ReachFrame {pub(super) root:EntityRef,marked:bool,cursor:usize,start:Option<CoedgeId>,pub(super) next:Option<CoedgeId>,probes:usize}

semio_framework_value::artifact_retire_leaf!(ReachFrame);

enum ReachChild {Done,Skip,Entity(EntityRef)}

impl ReachFrame {
    pub(super) fn new(root:EntityRef)->Self{Self{root,marked:false,cursor:0,start:None,next:None,probes:0}}
    fn child(&mut self,body:&Body)->ReachChild {
        let cursor=self.cursor;self.cursor+=1;
        let child=match self.root {
            EntityRef::Solid(id)=>body.solids.get(id).and_then(|solid|if cursor==0{Some(EntityRef::Shell(solid.outer))}else{solid.inners.get(cursor-1).copied().map(EntityRef::Shell)}),
            EntityRef::Shell(id)=>body.shells.get(id).and_then(|shell|shell.faces.get(cursor).copied().map(EntityRef::Face)),
            EntityRef::Face(id)=>{let Some(face)=body.faces.get(id)else{return ReachChild::Done};match cursor{0=>Some(EntityRef::Surface(face.surface)),1=>{let Some(outer)=face.outer else{return ReachChild::Skip};Some(EntityRef::Loop(outer))},_=>face.inners.get(cursor-2).copied().map(EntityRef::Loop)}},
            EntityRef::Loop(id)=>{if self.start.is_none(){let Some(loop_)=body.loops.get(id)else{return ReachChild::Done};self.start=Some(loop_.first);self.next=self.start;}if self.probes>=body.coedges.slot_count() || (self.probes!=0 && self.next==self.start){return ReachChild::Done;}let Some(next)=self.next else{return ReachChild::Done};self.next=body.coedges.get(next).map(|coedge|coedge.next);self.probes+=1;Some(EntityRef::Coedge(next))},
            EntityRef::Coedge(id)=>{let Some(coedge)=body.coedges.get(id)else{return ReachChild::Done};match cursor{0=>{let Some(pcurve)=coedge.pcurve else{return ReachChild::Skip};Some(EntityRef::Curve2(pcurve))},1=>Some(EntityRef::Edge(coedge.edge)),_=>None}},
            EntityRef::Edge(id)=>body.edges.get(id).and_then(|edge|match cursor{0=>Some(EntityRef::Curve3(edge.curve)),1=>Some(EntityRef::Vertex(edge.v0)),2=>Some(EntityRef::Vertex(edge.v1)),_=>None}),
            _=>None,
        };
        child.map_or(ReachChild::Done,ReachChild::Entity)
    }
}

/// 🧭️ Borrows one original topology member per turn while retaining the same physical membership arenas.
pub struct ReachabilityJob {pub(super) keep:ReachSet,frames:[Option<ReachFrame>;7],pub(super) depth:usize,pub(super) cancelled:bool}

semio_framework_value::artifact_retire_struct!(ReachabilityJob {keep,frames,depth,cancelled});

/// 🪙️ Logical traversal readiness remains separate from the original membership allocation retirement.
pub enum ReachabilityStep {Working(RetainedCloneProgress),Ready(RetainedCloneProgress),Cancelled(RetainedCloneProgress),Failed {error:ValueError,progress:RetainedCloneProgress}}

impl ReachabilityStep {pub fn progress(&self)->RetainedCloneProgress {match self {Self::Working(progress)|Self::Ready(progress)|Self::Cancelled(progress)|Self::Failed{progress,..}=>*progress}}}

impl Default for ReachabilityJob {fn default()->Self{Self::new()}}

impl ReachabilityJob {
    pub fn new()->Self{Self{keep:ReachSet::default(),frames:[None;7],depth:0,cancelled:false}}
    pub fn begin_root(&mut self,root:EntityRef)->Result<(),EntityRef>{if self.depth!=0 || self.cancelled{return Err(root)}self.frames[0]=Some(ReachFrame::new(root));self.depth=1;Ok(())}
    pub fn keep(&self)->&ReachSet{&self.keep}
    pub fn walk_is_complete(&self)->bool{self.depth==0 && !self.cancelled}
    pub fn cancel(&mut self){self.cancelled=true;}
    pub fn restart(&mut self){self.keep.clear_membership();self.frames=[None;7];self.depth=0;self.cancelled=false;}
    pub fn into_set(self)->ReachSet{assert!(self.walk_is_complete(),"original reach walk requires logical completion");self.keep}
    pub(super) fn demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.depth==0 || self.cancelled{return Ok(Default::default())}let frame=self.frames[self.depth-1].as_ref().unwrap();let mut demand=if frame.marked{RetirementDemand{depth:1,..Default::default()}}else{self.keep.mark_demand(frame.root,copy)?};demand.depth=demand.depth.checked_add(self.depth).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original reachability depth overflow"))?;Ok(demand)}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demand(copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.release_bytes)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
    pub fn step(&mut self,body:&Body,grant:RetainedCloneGrant)->Result<ReachabilityStep,ValueError>{
        let empty=RetainedCloneProgress::default();if self.cancelled{return Ok(ReachabilityStep::Cancelled(empty))}if self.depth==0{return Ok(ReachabilityStep::Ready(empty))}
        let demand=self.demand(grant.maximum_copy_bytes)?;if grant.maximum_items==0 || grant.maximum_copy_bytes<demand.copy_bytes || grant.maximum_capacity_bytes<demand.capacity_bytes || grant.maximum_release_bytes<demand.release_bytes || grant.maximum_depth<demand.depth{return Ok(ReachabilityStep::Working(empty))}
        let slot=self.depth-1;let frame=self.frames[slot].unwrap();
        let progress=if !frame.marked {
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-self.depth,..grant};match self.keep.mark_step(frame.root,child){Ok((marked,progress))=>{if let Some(marked)=marked{if marked{self.frames[slot].as_mut().unwrap().marked=true;}else{self.frames[slot]=None;self.depth-=1;}}progress},Err((error,progress))=>return Ok(ReachabilityStep::Failed{error,progress})}
        }else{
            match self.frames[slot].as_mut().unwrap().child(body){ReachChild::Done=>{self.frames[slot]=None;self.depth-=1;},ReachChild::Skip=>{},ReachChild::Entity(root)=>{if self.depth==self.frames.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology exceeds its structural reachability depth"))}self.frames[self.depth]=Some(ReachFrame::new(root));self.depth+=1;}}
            RetainedCloneProgress{copied_items:1,..empty}
        };
        Ok(if self.depth==0{ReachabilityStep::Ready(progress)}else{ReachabilityStep::Working(progress)})
    }
}
// #endregion ♻️Reachability
