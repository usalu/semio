//! 🪢️ One original-funded projection leases the render host's actual immutable source Arc.
use super::{IfcSnapshot,wire::SnapshotSource};
use super::{cursor_step,owned_step,refused};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneSourceAdmission,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

#[derive(Clone,Copy)]
struct Binding{original:OriginalSourceBinding,pointer:usize,revision:[u8;32]}
semio_framework_value::artifact_retire_leaf!(Binding);
type Admission=RetainedCloneSourceAdmission<IfcSnapshot,Arc<IfcSnapshot>>;
fn project(owner:&Arc<IfcSnapshot>)->&IfcSnapshot{owner}
pub struct SourceLoan{admission:ManuallyDrop<Option<Admission>>,source:ManuallyDrop<Option<SnapshotSource>>,returned:ManuallyDrop<Option<Arc<IfcSnapshot>>>,binding:Option<Binding>,retirement:RetainedCloneClose,closing:bool}
impl Default for SourceLoan{fn default()->Self{Self{admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),binding:None,retirement:Default::default(),closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"staging.original-source-loan")}
impl SourceLoan{
    fn metadata_bytes()->usize{2*size_of::<Self>()+size_of::<Arc<IfcSnapshot>>()}
    pub fn demands(&self)->Result<RetirementDemand,ValueError>{
        if self.source.is_some(){return Ok(Default::default());}
        match self.admission.as_ref(){
            Some(admission)=>{let mut demand=admission.next_demand()?;demand.copy_bytes=demand.copy_bytes.max(Self::metadata_bytes());Ok(demand)},
            None=>Ok(RetirementDemand{copy_bytes:Self::metadata_bytes(),depth:1,..Default::default()}),
        }
    }
    pub fn source(&self)->Option<&SnapshotSource>{self.source.as_ref()}
    pub fn advance(&mut self,owner:&Arc<IfcSnapshot>,expected:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let pointer=expected as *const _ as usize;
        if self.closing||!std::ptr::eq(owner.as_ref(),expected)||self.binding.is_some_and(|binding|!binding.original.matches(cx)||binding.pointer!=pointer||binding.revision!=revision){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging source loan cancelled").into());}
        if self.source.is_some(){return Ok(true);}
        let grant=cx.retained_grant();let demand=self.demands()?;
        if cx.should_yield()||!retained_clone_grant_admits(grant,demand){return Ok(false);}
        if self.admission.is_none(){
            *self.admission=Some(SnapshotSource::admission_borrowed(Arc::clone(owner),project));
            self.binding=Some(Binding{original:OriginalSourceBinding::borrow(cx),pointer,revision});
            let posted=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:Self::metadata_bytes(),..Default::default()});cx.consume_fuel(1);
            return posted.map(|()|false).map_err(Into::into);
        }
        let turn=match self.admission.as_mut(){
            Some(admission)=>if admission.phase()=="ready"{admission.take_output(&mut self.source,grant)}else{admission.advance(grant)},
            None=>refused(invalid()),
        };
        if self.source.is_some(){*self.admission=None;}
        cx.consume_fuel(turn.work);
        match turn.result{
            Ok(step)=>{cx.consume_retained(step.progress())?;Ok(self.source.is_some())},
            Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx)),
        }
    }
    pub fn empty(&self)->bool{self.admission.is_none()&&self.source.is_none()&&self.returned.is_none()&&self.retirement.is_empty()}
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Arc<IfcSnapshot>>(false,&None,body);}
        if let Some(admission)=self.admission.as_ref(){return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()};}
        if self.returned.is_some(){return self.retirement.next_owner_demands::<Arc<IfcSnapshot>>(true,&None,body);}
        if self.source.is_some(){return self.retirement.next_owner_demands::<SnapshotSource>(true,&None,body);}
        Ok(Default::default())
    }
    pub fn close_owned(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        if !self.retirement.is_empty(){return self.retirement.step_granted(grant);}
        if let Some(admission)=self.admission.as_mut(){
            let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,grant)}else{
                let mut inputs=None;
                let turn=admission.return_inputs(&mut inputs,grant);
                if let Some((owner,()))=inputs{*self.returned=Some(owner);}
                turn
            };
            if admission.terminal_is_empty()||self.source.is_some(){*self.admission=None;}
            return turn;
        }
        if self.returned.is_some(){return owned_step(self.retirement.begin_granted(&mut self.returned,grant),"source loan returned authority omitted its owner");}
        if self.source.is_some(){return owned_step(self.retirement.begin_granted(&mut self.source,grant),"source loan source omitted its owner");}
        RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
    }
}
impl RetireOwned for SourceLoan{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for SourceLoan{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if !self.closing{return RetirementStep::BudgetExhausted;}let turn=self.close_owned(grant);let empty=self.empty();cursor_step(turn,empty)}
    fn terminal_is_empty(&self)->bool{self.empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.copy_bytes)}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{!self.empty()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{self.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.close_demand(4096).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(4096)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.empty().then_some(size_of::<Self>())}
}
impl Drop for SourceLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"source loan requires original paid retirement");if self.empty(){unsafe{ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.returned);}}}}
