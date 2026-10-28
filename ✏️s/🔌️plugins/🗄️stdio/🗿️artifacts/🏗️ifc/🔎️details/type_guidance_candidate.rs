//! 🪪️ Guided readiness prepares the actual original-owned central type candidate without publishing it.
use super::*;
use super::{candidate_rules::IfcTypeCandidate,constituent_roster::ConstituentRoster,domain_work::DomainWork,pointer::PathAdmission,type_guidance_request::RequestCopy,wire::SnapshotSource};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_plugin::action_staging::StagedFormRequest;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneClose,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::{ManuallyDrop,size_of};
type PartsCursor=<Vec<IfcComplexType> as RetainedClone>::Cursor;
type ValueCursor=<IfcValue as RetainedClone>::Cursor;
#[derive(Clone,Copy)]
struct Binding{original:OriginalSourceBinding,source:usize,lease:usize,revision:[u8;32]}
pub(super) struct GuidedCandidate<C:IfcDetailsProfile>{
    request:RequestCopy,args:ManuallyDrop<Option<DslValue>>,event:ManuallyDrop<Option<editing::SnapshotEditEvent>>,pending:ManuallyDrop<Option<Vec<IfcMutation>>>,parts:ManuallyDrop<Option<Vec<IfcComplexType>>>,value:ManuallyDrop<Option<IfcValue>>,
    address:PathAdmission,roster:ConstituentRoster,domain:DomainWork,rules:C::CandidateRules,parts_cursor:PartsCursor,value_cursor:ValueCursor,retirement:RetainedCloneClose,binding:Option<Binding>,phase:u8,closing:bool,valid:bool,
}
impl<C:IfcDetailsProfile> Default for GuidedCandidate<C>{fn default()->Self{Self{request:Default::default(),args:ManuallyDrop::new(None),event:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),parts:ManuallyDrop::new(None),value:ManuallyDrop::new(None),address:Default::default(),roster:Default::default(),domain:Default::default(),rules:Default::default(),parts_cursor:Default::default(),value_cursor:Default::default(),retirement:Default::default(),binding:None,phase:0,closing:false,valid:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"staging.original-type-candidate")}
impl<C:IfcDetailsProfile> GuidedCandidate<C>{
    fn metadata()->usize{2*size_of::<Option<Binding>>()+4*size_of::<u8>()+4*size_of::<bool>()+2*size_of::<editing::SnapshotEditEvent>()+2*size_of::<DslValue>()}
    pub(super) fn request(&self)->Option<&StagedFormRequest>{self.request.request()}
    pub(super) fn valid(&self)->bool{self.phase==9&&self.valid}
    fn event(&self)->Result<&editing::SnapshotEditEvent,ValueError>{self.event.as_ref().ok_or_else(invalid)}
    pub(super) fn demands(&self,source:&SnapshotSource,snapshot:&IfcSnapshot,body:usize)->Result<RetirementDemand,ValueError>{
        let bytes=Self::metadata();
        let child=match self.phase{
            1=>self.request.demands(body)?,
            4=>self.address.demands(),
            5=>self.roster.demands(self.event()?,Some(source),&self.parts_cursor,&self.value_cursor,snapshot,body)?,
            6=>self.rules.demands(snapshot,self.pending.as_deref().ok_or_else(invalid)?)?,
            7=>self.domain.demands(),
            _=>Default::default(),
        };
        Ok(RetirementDemand{copy_bytes:bytes.checked_add(child.copy_bytes).ok_or_else(invalid)?,capacity_bytes:child.capacity_bytes,release_bytes:child.release_bytes,depth:child.depth.max(1)})
    }
    pub(super) fn advance(&mut self,donor:&mut Option<StagedFormRequest>,source:&SnapshotSource,snapshot:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let pointer=snapshot as *const _ as usize;let lease=source as *const _ as usize;
        if self.closing||!std::ptr::eq(source.try_authority()?.as_ref(),snapshot)||self.binding.is_some_and(|binding|!binding.original.matches(cx)||binding.source!=pointer||binding.lease!=lease||binding.revision!=revision){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging candidate cancelled").into());}
        if self.phase==9{return Ok(true);}
        let grant=cx.retained_grant();let body=grant.maximum_copy_bytes.min(4096);let demand=self.demands(source,snapshot,body)?;
        if cx.should_yield()||!retained_clone_grant_admits(grant,demand){return Ok(false);}
        let before=cx.retained_progress();let mut next=self.phase;
        let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(Self::metadata()),..grant};
        match self.phase{
            0=>{self.binding=Some(Binding{original:OriginalSourceBinding::borrow(cx),source:pointer,lease,revision});next=1;},
            1=>{if cx.with_retained_loan(child_grant,|cx|self.request.advance(donor,cx))?{next=2;}},
            2=>{if cx.with_retained_loan(child_grant,|cx|self.request.take_args(&mut self.args,cx))?{next=3;}},
            3=>{
                if self.event.is_some(){return Err(invalid().into());}
                let DslValue::Object(fields)=self.args.as_mut().ok_or_else(invalid)? else{return Err(invalid().into());};
                if fields.len()!=2||fields.iter().filter(|(key,_)|key=="path").count()!=1||fields.iter().filter(|(key,_)|key=="value").count()!=1{return Err(invalid().into());}
                let path=fields.iter().position(|(key,_)|key=="path").ok_or_else(invalid)?;let value=1-path;
                if !matches!(fields[path].1,DslValue::String(_)){return Err(invalid().into());}
                let DslValue::String(path)=std::mem::replace(&mut fields[path].1,DslValue::Null)else{unreachable!()};
                let value=std::mem::replace(&mut fields[value].1,DslValue::Null);
                *self.event=Some(editing::SnapshotEditEvent::SetValue{path,value});next=4;
            },
            4=>{match cx.with_retained_loan(child_grant,|cx|self.address.advance(self.event.as_ref().ok_or_else(invalid)?,snapshot,|_,_|None,cx))?{Some(true)=>{if !ConstituentRoster::applies(self.event()?,&self.address){return Err(invalid().into());}next=5;},Some(false)=>return Err(invalid().into()),None=>{}}},
            5=>{
                if self.roster.fallback(){return Err(invalid().into());}
                cx.with_retained_loan(child_grant,|cx|self.roster.advance::<C>(self.event.as_mut().ok_or_else(invalid)?,snapshot,&self.address,Some(source),&mut self.parts_cursor,&mut self.parts,&mut self.pending,&mut self.value_cursor,&mut self.value,cx))?;
                if self.pending.is_some()&&!self.roster.receiving_primitives(){if self.roster.type_admission().is_none(){return Err(invalid().into());}next=6;}
            },
            6=>{
                let event=self.event.as_ref().ok_or_else(invalid)?;let mutations=self.pending.as_deref().ok_or_else(invalid)?;
                if let Some(candidate)=self.roster.type_admission().and_then(|admission|IfcTypeCandidate::new::<C>(admission,event,snapshot,&self.address,mutations)){
                    match cx.with_retained_loan(child_grant,|cx|self.rules.advance(&candidate,cx))?{IfcCandidateRuleStep::Pending=>{},IfcCandidateRuleStep::Complete=>next=7,IfcCandidateRuleStep::Invalid|IfcCandidateRuleStep::Incomplete=>{self.valid=false;next=9;}}
                }else{next=7;}
            },
            7=>{if cx.with_retained_loan(child_grant,|cx|self.domain.advance::<C>(self.event.as_ref().ok_or_else(invalid)?,snapshot,self.pending.as_deref().ok_or_else(invalid)?,&self.address,self.roster.type_admission(),cx))?{self.valid=true;next=9;}},
            _=>return Err(invalid().into()),
        }
        if next!=self.phase{self.phase=next;let delegated=cx.retained_progress()!=before;cx.consume_retained(RetainedCloneProgress{copied_items:usize::from(!delegated),copied_bytes:Self::metadata(),..Default::default()})?;if !delegated{cx.consume_fuel(1);}}
        Ok(self.phase==9)
    }
    fn begin_close(&mut self){self.closing=true;self.request.begin_close();self.rules.begin_close();self.address.begin_close();self.roster.begin_close();self.domain.begin_close();self.parts_cursor.begin_close();self.value_cursor.begin_close();}
    fn empty(&self)->bool{self.request.empty()&&self.rules.terminal_is_empty()&&self.address.terminal_is_empty()&&self.roster.terminal_is_empty()&&self.domain.terminal_is_empty()&&self.parts_cursor.terminal_is_empty()&&self.value_cursor.terminal_is_empty()&&self.args.is_none()&&self.event.is_none()&&self.pending.is_none()&&self.parts.is_none()&&self.value.is_none()&&self.retirement.is_empty()}
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.rules.terminal_is_empty(){return self.rules.close_demands(body);}
        if !self.domain.terminal_is_empty(){return Ok(self.domain.close_demands());}
        if !self.roster.terminal_is_empty(){return self.roster.close_demands(body);}
        if !self.address.terminal_is_empty(){return Ok(self.address.close_demands());}
        if !self.parts_cursor.terminal_is_empty(){return self.parts_cursor.close_demands(body);}
        if !self.value_cursor.terminal_is_empty(){return self.value_cursor.close_demands(body);}
        if !self.request.empty(){return self.request.close_demand(body);}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<DslValue>(false,&None,body);}
        macro_rules! owner{($value:expr,$kind:ty)=>{if ($value).is_some(){return self.retirement.next_owner_demands::<$kind>(true,&None,body);}};}
        owner!(&self.value,IfcValue);owner!(&self.parts,Vec<IfcComplexType>);owner!(&self.pending,Vec<IfcMutation>);owner!(&self.event,editing::SnapshotEditEvent);owner!(&self.args,DslValue);
        Ok(Default::default())
    }
    fn close_owned(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.rules.terminal_is_empty(){return self.rules.close_step(grant);}
        if !self.domain.terminal_is_empty(){return self.domain.close_step(grant);}
        if !self.roster.terminal_is_empty(){return self.roster.close_step(grant);}
        if !self.address.terminal_is_empty(){return self.address.close_step(grant);}
        if !self.parts_cursor.terminal_is_empty(){return self.parts_cursor.close_step(grant);}
        if !self.value_cursor.terminal_is_empty(){return self.value_cursor.close_step(grant);}
        if !self.request.empty(){return self.request.close_owned(grant);}
        if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}
        macro_rules! owner{($value:expr)=>{if ($value).is_some(){return self.retirement.begin_granted($value,grant)?.ok_or_else(invalid);}};}
        owner!(&mut self.value);owner!(&mut self.parts);owner!(&mut self.pending);owner!(&mut self.event);owner!(&mut self.args);Ok(RetainedCloneStep::Complete(Default::default()))
    }
}
impl<C:IfcDetailsProfile+'static> RetireOwned for GuidedCandidate<C>{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl<C:IfcDetailsProfile+'static> RetirementCursor for GuidedCandidate<C>{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if !self.closing{return RetirementStep::BudgetExhausted;}match self.close_owned(grant){Ok(RetainedCloneStep::Complete(progress))if progress==Default::default()&&self.empty()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{self.empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.copy_bytes)}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{!self.empty()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.close_demand(4096).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.empty().then_some(size_of::<Self>())}
}
impl<C:IfcDetailsProfile> Drop for GuidedCandidate<C>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"guided candidate requires original paid retirement");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.args);ManuallyDrop::drop(&mut self.event);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.parts);ManuallyDrop::drop(&mut self.value);}}}}
