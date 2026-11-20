//! 🌳️ Funded reachability over the original native geometry.

use super::{Brep, TopologyAlias};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError};
use semio_framework_3d_brep_representation::topology::{EntityRef,ReachabilityJob,ReachabilityStep};

/// 🌳️ Collects the exact native roots of one retained preview before its validation.
pub struct PreviewReach {
    walk:ReachabilityJob,
    revision:u64,
    root:usize,
    units:usize,
    depth:usize,
    done:bool,
}
semio_framework_value::artifact_retire_struct!(PreviewReach{walk,revision,root,units,depth,done});

impl PreviewReach {
    pub fn new(revision:u64)->Self{
        Self{walk:ReachabilityJob::new(),revision,root:0,units:0,depth:1,done:false}
    }

    pub fn done(&self)->bool{self.done}

    pub fn depth(&self)->usize{self.depth}

    pub fn units(&self)->usize{self.units}

    pub fn scope(&self)->&semio_framework_3d_brep_representation::topology::ReachSet{self.walk.keep()}

    pub fn check(&self,kernel:&Brep)->Result<(),ValueError>{
        if self.revision!=kernel.preview_validation_revision(){return Err(ValueError::invariant("preview validation source changed"));}
        Ok(())
    }

    pub fn demand(&self,kernel:&Brep,handle:&str,copy:usize)->Result<RetirementDemand,ValueError>{
        self.check(kernel)?;
        if self.walk.walk_is_complete(){
            return Ok(RetirementDemand{depth:kernel.tessellation_source_depth_demand(handle)?.max(1),..Default::default()});
        }
        Ok(RetirementDemand{
            copy_bytes:self.walk.next_copy_byte_demand()?,
            capacity_bytes:self.walk.next_capacity_byte_demand(copy)?,
            release_bytes:self.walk.next_release_byte_demand()?,
            depth:self.walk.next_depth_demand()?.max(1),
            ..Default::default()
        })
    }

    pub fn step(&mut self,kernel:&Brep,handle:&str,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
        let grant=effect.grant();
        let demand=self.demand(kernel,handle,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)||!effect.performed(1)?{
            return Ok(Default::default());
        }
        self.depth=self.depth.max(demand.depth);
        self.units+=1;
        if self.walk.walk_is_complete(){
            if let Some(root)=kernel.preview_validation_root(handle,self.root)?{
                self.walk.begin_root(root).map_err(|_|ValueError::invariant("original preview root is already under validation"))?;
                self.root+=1;
            }else{self.done=true;}
            return Ok(RetainedCloneProgress{copied_items:1,..Default::default()});
        }
        match self.walk.step(kernel.tessellation_body(),grant)?{
            ReachabilityStep::Working(receipt)|ReachabilityStep::Ready(receipt)=>Ok(receipt),
            ReachabilityStep::Failed{error,progress}=>Err(error.with_retained_progress(progress)),
            ReachabilityStep::Cancelled(receipt)=>Err(ValueError::invariant("preview root observation was retired").with_retained_progress(receipt)),
        }
    }
}


/// 🔗️ An original topology family that can be shared without copying its geometry.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReachedTopology { Vertex, Edge, Face, Shell }
semio_framework_value::artifact_retire_leaf!(ReachedTopology);

impl ReachedTopology {
    pub fn channel(self) -> &'static str {
        match self { Self::Vertex => "vertex", Self::Edge => "edge", Self::Face => "face", Self::Shell => "shell" }
    }
}

/// 🕸️ Retains original reach and ranks members by arena identity.
pub struct NativeTopologyReach { owner: usize, scope: PreviewReach }
semio_framework_value::artifact_retire_struct!(NativeTopologyReach { owner, scope });

macro_rules! by_kind {
    ($kind:expr,$scope:expr,$set:ident,$variant:ident,$expr:expr) => {
        match $kind {
            ReachedTopology::Vertex => { let $set = &$scope.vertices.0; let $variant = EntityRef::Vertex; $expr }
            ReachedTopology::Edge => { let $set = &$scope.edges.0; let $variant = EntityRef::Edge; $expr }
            ReachedTopology::Face => { let $set = &$scope.faces.0; let $variant = EntityRef::Face; $expr }
            ReachedTopology::Shell => { let $set = &$scope.shells.0; let $variant = EntityRef::Shell; $expr }
        }
    };
}

impl NativeTopologyReach {
    pub fn new(kernel: &Brep) -> Self {
        Self { owner: std::ptr::from_ref(kernel) as usize, scope: PreviewReach::new(kernel.preview_validation_revision()) }
    }

    pub fn demand(&self, kernel: &Brep, handle: &str, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.check(kernel)?;
        self.scope.demand(kernel, handle, copy)
    }

    pub fn step(&mut self, kernel: &Brep, handle: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, RetainedCloneProgress), ValueError> {
        self.check(kernel)?;
        let receipt = self.scope.step(kernel, handle, effect)?;
        Ok((self.scope.done(), receipt))
    }

    pub fn member_depth(&self, kernel: &Brep, kind: ReachedTopology, rank: usize) -> Result<usize, ValueError> {
        self.check(kernel)?;
        by_kind!(kind, self.scope.scope(), set, variant, {
            let depth = set.next_rank_depth_demand(rank)?;
            let (id, ()) = set.ranked(rank).ok_or_else(|| ValueError::invariant("original topology member is absent"))?;
            Ok(depth.max(kernel.reached_topology_alias_depth(variant(*id))?))
        })
    }

    pub fn member_alias(&self, kernel: &Brep, kind: ReachedTopology, rank: usize) -> Result<TopologyAlias, ValueError> {
        self.check(kernel)?;
        let member = by_kind!(kind, self.scope.scope(), set, variant, set.ranked(rank).map(|(id, ())| variant(*id)));
        kernel.reached_topology_alias(member.ok_or_else(|| ValueError::invariant("original topology member is absent"))?)
    }

    fn check(&self, kernel: &Brep) -> Result<(), ValueError> {
        if self.owner != std::ptr::from_ref(kernel) as usize {
            return Err(ValueError::invariant("retained topology scope belongs to another kernel"));
        }
        self.scope.check(kernel)
    }

    pub fn count(&self, kind: ReachedTopology) -> usize {
        by_kind!(kind,self.scope.scope(),set,_variant,set.len())
    }
}
