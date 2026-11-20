//! 🧺️ Funded publication of a group of original solid roots.

use super::*;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

pub struct CompoundPublication {
    owner:usize,
    revision:u64,
    expected:usize,
    members:Vec<SolidId>,
    label:PersistentLabel,
    handle:[u8;64],
}
semio_framework_value::artifact_retire_struct!(CompoundPublication {owner,revision,expected,members,label,handle});

impl CompoundPublication {
    pub fn handle(&self)->&str{std::str::from_utf8(&self.handle).expect("digest is ASCII")}
    pub fn complete(&self)->bool{self.members.len()==self.expected}
    pub fn next_member(&self)->Option<usize>{(self.members.capacity()!=0&&!self.complete()).then_some(self.members.len())}

    fn check(&self,kernel:&BrepRegistry)->Result<(),ValueError>{
        if self.owner!=std::ptr::from_ref(kernel)as usize||self.revision!=kernel.shape_mutation_revision{
            return Err(ValueError::invariant("compound original source changed"));
        }
        Ok(())
    }
}

impl BrepRegistry {
    /// 🧮️ Admits an original group without borrowing or copying member handles.
    pub fn plan_compound(&mut self,count:usize)->Result<CompoundPublication,ValueError>{
        if count==0||count>65_536{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"compound requires between one and 65,536 solid roots"));}
        let label=self.body.new_label();
        Ok(CompoundPublication{owner:std::ptr::from_ref(&*self)as usize,revision:self.shape_mutation_revision,expected:count,
            members:Vec::new(),label,handle:analytic_publication::primitive_handle(b"Compound:",label.0)})
    }

    fn compound_solid(&self,source:&str)->Result<SolidId,ValueError>{
        match self.original_query_entity(source)?{
            Entity::Solid(id,_) if self.body.solids.contains(*id)=>Ok(*id),
            _=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"compound input must name an original solid")),
        }
    }

    /// 📏️ Quotes one group buffer reservation or one original member reference.
    pub fn compound_preparation_demand(&self,plan:&CompoundPublication,source:Option<&str>)->Result<RetirementDemand,ValueError>{
        plan.check(self)?;
        if plan.complete(){return Ok(Default::default());}
        if plan.members.capacity()==0{
            return Ok(RetirementDemand{capacity_bytes:plan.expected*size_of::<SolidId>(),depth:1,..Default::default()});
        }
        let source=source.ok_or_else(||ValueError::invariant("compound member is missing"))?;
        let id=self.compound_solid(source)?;
        let depth=self.live.next_contains_depth_demand(source)?.max(self.body.solids.next_remove_depth_demand(id)?);
        Ok(RetirementDemand{copy_bytes:size_of::<SolidId>(),depth,..Default::default()})
    }

    /// 🧩️ Retains one exact native solid root in the admitted member backing.
    pub fn step_compound_preparation(&self,plan:&mut CompoundPublication,source:Option<&str>,effect:&mut RetainedAllocationEffect<'_>)->Result<Progress,ValueError>{
        let grant=effect.grant();
        let demand=self.compound_preparation_demand(plan,source)?;
        if plan.complete()||!retained_clone_grant_admits(grant,demand)||!effect.performed(1)?{return Ok(Default::default());}
        let mut result=Progress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if plan.members.capacity()==0{
            plan.members.try_reserve_exact(plan.expected).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"compound member backing allocation failed"))?;
            result.retained_capacity_bytes=plan.members.capacity()*size_of::<SolidId>();
            if !result.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"compound member capacity exceeded its grant").with_retained_progress(result));}
        }else{
            plan.members.push(self.compound_solid(source.expect("original source was quoted"))?);
        }
        Ok(result)
    }

    pub fn compound_publication_demand(&self,plan:&CompoundPublication,copy:usize)->Result<RetirementDemand,ValueError>{
        plan.check(self)?;
        if !plan.complete()||self.live.contains_key(plan.handle()){return Err(ValueError::invariant("compound original member list is not unpublished and complete"));}
        Ok(RetirementDemand{capacity_bytes:self.live.next_insert_capacity_byte_demand(plan.handle(),copy)?,
            depth:self.live.next_insert_depth_demand(plan.handle())?,..Default::default()})
    }

    pub fn reserve_compound_publication(&mut self,plan:&CompoundPublication,grant:Grant)->Result<Progress,ValueError>{
        self.compound_publication_demand(plan,grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(plan.handle(),grant).map_err(|(error,receipt)|error.with_retained_progress(receipt))
    }

    pub fn publish_compound(&mut self,plan:&mut CompoundPublication,key:String,grant:Grant)->Result<Progress,(ValueError,String)>{
        if key!=plan.handle(){return Err((ValueError::invariant("compound original live key changed"),key));}
        if let Err(error)=self.compound_publication_demand(plan,grant.maximum_copy_bytes){return Err((error,key));}
        let members=std::mem::take(&mut plan.members);
        let (last,receipt)=self.live.insert_reserved(key,Entity::Compound(members,plan.label),grant)
            .map_err(|(error,key,entity)|{
                if let Entity::Compound(members,_)=entity{plan.members=members;}
                (error,key)
            })?;
        assert!(last.is_none());
        self.advance_live_revision();
        Ok(receipt)
    }
}
