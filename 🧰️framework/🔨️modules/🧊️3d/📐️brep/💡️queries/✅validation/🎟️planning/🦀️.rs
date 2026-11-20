use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

enum PlanningAppend {None,Shell,ShellFace(usize),Edge,Face}

fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original validation planning source changed")}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"original validation planning backing allocation failed")}
fn vector_demand<T>(values:&Vec<T>,extent:usize)->Result<RetirementDemand,ValueError>{
    if values.capacity()==0{Ok(RetirementDemand{capacity_bytes:extent.checked_mul(std::mem::size_of::<T>()).ok_or_else(refusal)?,depth:1,..Default::default()})}
    else if values.len()<values.capacity(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<T>(),depth:1,..Default::default()})}
    else{Err(refusal())}
}
fn reserve<T>(values:&mut Vec<T>,extent:usize)->Result<RetainedCloneProgress,ValueError>{
    if !values.is_empty()||values.capacity()!=0{return Err(refusal());}
    values.try_reserve_exact(extent).map_err(|_|allocation())?;
    Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:values.capacity().checked_mul(std::mem::size_of::<T>()).ok_or_else(refusal)?,..Default::default()})
}

impl BodyValidationJob {
    fn planning_append(&self,body:&Body,scope:Option<&ReachSet>)->Result<PlanningAppend,ValueError>{
        Ok(match self.planning_stage{
            0=>PlanningAppend::None,
            1=>{
                if let Some(solid)=self.planning_solid{
                    let Some(solid)=body.solids.get(solid)else{return Ok(PlanningAppend::None);};
                    let Some(shell)=(if self.planning_shell==0{Some(solid.outer)}else{solid.inners.get(self.planning_shell-1).copied()})else{return Ok(PlanningAppend::None);};
                    if let Some(index)=self.planning_face{
                        if let Some(shell)=body.shells.get(shell){if index<shell.faces.len(){PlanningAppend::ShellFace(shell.faces.len())}else{PlanningAppend::None}}else{PlanningAppend::None}
                    }else{PlanningAppend::Shell}
                }else{PlanningAppend::None}
            },
            2=>if scoped_slot!(body,scope,edges,self.planning_slot).is_some(){PlanningAppend::Edge}else{PlanningAppend::None},
            3=>if scoped_slot!(body,scope,faces,self.planning_slot).is_some(){PlanningAppend::Face}else{PlanningAppend::None},
            4=>PlanningAppend::None,
            _=>return Err(refusal()),
        })
    }
    /// 🎟️ Quotes one original topology planning event independently of subsequent validation work.
    pub fn planning_demands(&self,body:&Body)->Result<RetirementDemand,ValueError>{self.planning_demands_for(body,None)}

    pub(super) fn planning_demands_for(&self,body:&Body,scope:Option<&ReachSet>)->Result<RetirementDemand,ValueError>{
        if self.source_body!=body as *const Body as usize||self.phase!=BodyValidationPhase::CollectingTopology{return Err(refusal());}
        match self.planning_append(body,scope)?{
            PlanningAppend::None=>Ok(RetirementDemand{depth:1,..Default::default()}),
            PlanningAppend::Shell=>{
                let mut demand=vector_demand(&self.shells,self.planning_shell_extent)?;
                if demand.copy_bytes!=0{demand.copy_bytes=size_of::<semio_framework_3d_brep_representation::arena::SolidId>()+size_of::<semio_framework_3d_brep_representation::arena::ShellId>();}
                Ok(demand)
            },
            PlanningAppend::ShellFace(extent)=>vector_demand(&self.shells.last().ok_or_else(refusal)?.faces,extent),
            PlanningAppend::Edge=>vector_demand(&self.edges,scope.map_or_else(||body.edges.len(),|reach|reach.edges.len())),
            PlanningAppend::Face=>vector_demand(&self.faces,scope.map_or_else(||body.faces.len(),|reach|reach.faces.len())),
        }
    }
    /// ⏱️ Retains empty backing before copying one original entity, with actual physical currencies.
    pub fn step_planning_funded(&mut self,body:&Body,units:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(RetainedCloneProgress,usize),ValueError>{
        self.step_planning_for(body,None,units,effect)
    }

    pub(super) fn step_planning_for(&mut self,body:&Body,scope:Option<&ReachSet>,units:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(RetainedCloneProgress,usize),ValueError>{
        let grant=effect.grant();
        let demand=self.planning_demands_for(body,scope)?;
        if units==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((Default::default(),0));}
        let append=self.planning_append(body,scope)?;
        if !effect.performed(1)? { return Ok((Default::default(),0)); }
        let receipt=if demand.capacity_bytes!=0{
            match append{
                PlanningAppend::Shell=>reserve(&mut self.shells,self.planning_shell_extent)?,
                PlanningAppend::ShellFace(extent)=>reserve(&mut self.shells.last_mut().ok_or_else(refusal)?.faces,extent)?,
                PlanningAppend::Edge=>reserve(&mut self.edges,scope.map_or_else(||body.edges.len(),|reach|reach.edges.len()))?,
                PlanningAppend::Face=>reserve(&mut self.faces,scope.map_or_else(||body.faces.len(),|reach|reach.faces.len()))?,
                PlanningAppend::None=>return Err(refusal()),
            }
        }else{
            self.collect_selected_topology(body,scope);
            RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
        };
        self.units_done+=1;self.units_total=self.units_total.max(self.units_done.saturating_add(1));
        if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original validation planning allocator exceeded admitted backing").with_retained_progress(receipt));}
        Ok((receipt,1))
    }
}
