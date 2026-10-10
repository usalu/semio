//! 📨️ Owned child emission preparation retains typed operation owners across every refusal.
use crate::{store,protocol,app::{ChildEmit,PluginCloseStep,PluginLifecycleStep}};
use semio_framework_diagnostic::{Fault,FaultFrom};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};

type BuiltinAdmit<M>=fn(&mut Option<M>,&mut Option<Box<dyn ErasedSnapshotRetirement>>,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
type BuiltinDemand<M>=fn(&Option<M>)->Result<RetirementDemand,ValueError>;

include!("♻️retirement/🦀️.rs");

fn yields(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes}
fn owned_message_bytes(error:&ValueError)->usize{match &error.message{std::borrow::Cow::Borrowed(_)=>0,std::borrow::Cow::Owned(message)=>message.capacity()}}
fn value_fault(error:ValueError)->Fault{retirement_refusal_fault(&error)}

pub enum ChildEmitPreparationStep {
    Pending(RetainedCloneProgress),
    Ready(RetainedCloneProgress),
    Refused(Fault,RetainedCloneProgress),
}
impl ChildEmitPreparationStep {
    /// 🎟️ Preserves the actual producer receipt independently of readiness.
    pub fn progress(&self)->RetainedCloneProgress{match self{Self::Pending(progress)|Self::Ready(progress)|Self::Refused(_,progress)=>*progress}}
}
/// 🪵️ Applying child metadata and its original typed source remain one retained owner.
pub struct OwnedChildEmit {
    metadata:Option<ChildEmit>,
    mutations:Option<store::MemberStoreOwnedBatch>,
}
impl OwnedChildEmit {
    pub(crate) fn new(metadata:ChildEmit,mutations:store::MemberStoreOwnedBatch)->Self{Self{metadata:Some(metadata),mutations:Some(mutations)}}
    pub fn metadata(&self)->Option<&ChildEmit>{self.metadata.as_ref()}
    pub fn mutations<M:Send+'static>(&self)->Option<&[M]>{self.mutations.as_ref()?.mutations::<M>()}
    pub(crate) fn take_parts(&mut self)->Option<(ChildEmit,store::MemberStoreOwnedBatch)>{
        if self.metadata.is_none()||self.mutations.is_none(){return None;}
        Some((self.metadata.take().unwrap(),self.mutations.take().unwrap()))
    }
    pub fn retirement_demands(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if let Some(batch)=self.mutations.as_ref(){let mut demand=batch.next_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"owned child source depth overflow"))?;return Ok(demand);}
        if let Some(metadata)=self.metadata.as_ref(){
            if metadata.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
            let mut demand=metadata.retirement_demands()?;
            demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"owned metadata depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
    /// 🎟️ Keeps logical work independent from original typed source and metadata physical release.
    pub fn close_granted(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if demand.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"owned child close exceeds admitted depth"));}
        if demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.copy_bytes>grant.maximum_copy_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(batch)=self.mutations.as_mut(){
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let step=batch.close_granted(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,batch.terminal_is_empty(),"owned child source")?;
            if batch.terminal_is_empty(){self.mutations.take();}
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(metadata)=self.metadata.as_mut(){
            if metadata.terminal_is_empty(){self.metadata.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let step=metadata.close_one(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,metadata.terminal_is_empty(),"original owned child metadata")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn terminal_is_empty(&self)->bool{self.metadata.is_none()&&self.mutations.is_none()}
}
impl Drop for OwnedChildEmit{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"applying child emission dropped before original typed source and metadata returned");}
}
trait ChildEmitPreparationOwner: Send {
    fn as_any_mut(&mut self)->&mut dyn std::any::Any;
    fn matches_source(&self,mutation_type:std::any::TypeId,slot:&str,child_id:&str,maximum_operations:usize)->bool;
    fn step(&mut self, grant:RetainedCloneGrant)->Result<ChildEmitPreparationStep,Fault>;
    fn step_demands(&self)->Result<RetirementDemand,ValueError>;
    fn preparation_is_ready(&self)->bool;
    fn take_ready(&mut self)->Option<ChildEmit>;
    fn next_owned_capacity_byte_demand(&self)->usize;
    fn take_ready_owned(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(ChildEmit,store::MemberStoreOwnedBatch,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>;
    fn terminal_is_empty(&self)->bool;
    fn retirement_demands(&self, body:usize)->Result<RetirementDemand,ValueError>;
    fn refusal(&self)->Option<&::protocol::ProtocolError>;
    fn retirement_refusal(&self)->Option<&semio_framework_value::ValueError>;
    fn accepted_prefix(&self)->Option<&ChildEmit>;
    fn retained_operation_count(&self)->usize;
}
pub struct ChildEmitPreparation {
    owner:Box<dyn ChildEmitPreparationOwner>,
}
impl ChildEmitPreparation {
    pub fn of<S,M>(slot:impl Into<String>,child_id:impl Into<String>,operations:Vec<M>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+semio_framework_value::retirement::RetireOwned {
        Self::with_factory_kind::<S,M>(slot.into(),child_id.into(),operations,None,Some((store::artifact_retirement_admit_owned::<M>,store::artifact_retirement_owned_birth_demands::<M>)))
    }
    /// 📦️ Retains the original typed mutation vector for direct member admission.
    pub fn of_owned<S,M>(slot:impl Into<String>,child_id:impl Into<String>,operations:Vec<M>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+semio_framework_value::retirement::RetireOwned {
        let mut preparation=Self::with_factory_kind::<S,M>(slot.into(),child_id.into(),Vec::new(),None,Some((store::artifact_retirement_admit_owned::<M>,store::artifact_retirement_owned_birth_demands::<M>)));
        let owner=preparation.owner.as_any_mut().downcast_mut::<TypedChildEmitPreparation<M>>().unwrap();
        owner.operation_count=operations.len();
        owner.backing_bytes=operations.capacity().checked_mul(std::mem::size_of::<M>()).expect("typed source vector layout");
        owner.remaining.take();
        *owner.owned_source=Some(operations);
        owner.batch_capacity_bytes=store::MemberStoreOwnedBatch::scaffold_byte_demand::<M>();
        owner.take_batch=Some(|values,grant|store::MemberStoreOwnedBatch::try_new(values,grant));
        preparation
    }
    /// 🌱️ Carries the explicit member source with the same retained typed operation prefix.
    pub fn with_genesis<S,M>(slot: impl Into<String>, child_id: impl Into<String>, genesis: crate::app::ChildEmitGenesis, operations: Vec<M>) -> Self
    where M: protocol::SemanticMutation<S> + protocol::OpBinary + semio_framework_value::retirement::RetireOwned {
        let mut preparation=Self::of_owned::<S,M>(slot,child_id,operations);
        preparation.owner.as_any_mut().downcast_mut::<TypedChildEmitPreparation<M>>().expect("exact typed genesis operation owner").prefix.as_mut().expect("retained genesis prefix").genesis=Some(genesis);
        preparation
    }
    pub fn with_factory<S,M>(slot:String,child_id:String,operations:Vec<M>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+Send+'static {
        Self::with_factory_kind::<S,M>(slot,child_id,operations,Some(factory),None)
    }
    fn with_factory_kind<S,M>(slot:String,child_id:String,operations:Vec<M>,factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,retire_builtin:Option<(BuiltinAdmit<M>,BuiltinDemand<M>)>)->Self
    where M:protocol::SemanticMutation<S>+::protocol::OpBinary+Send+'static {
        let operation_count=operations.len();
        let backing_bytes=operations.capacity().checked_mul(std::mem::size_of::<M>()).expect("owned mutation vector allocation has a representable layout");
        Self{owner:Box::new(TypedChildEmitPreparation::<M>{
            remaining:ManuallyDrop::new(Some(operations.into_iter())),owned_source:ManuallyDrop::new(None),take_batch:None,batch_capacity_bytes:0,backing_bytes,operation_count,
            current:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),retired_schema:ManuallyDrop::new(None),
            prefix:Some(ChildEmit::open(slot,child_id,0)),cause:ManuallyDrop::new(None),close_refusal:ManuallyDrop::new(None),factory:ManuallyDrop::new(factory),closing_factory:ManuallyDrop::new(None),retire_builtin,
            schema_parts:|operation|{let semantics=protocol::SemanticMutation::<S>::semantics(operation);(semantics.entity,semantics.kind)},schema_offset:0,schema_started:false,
            closing:false,ready:false,
        })}
    }
    /// 🪪️ Checks the original typed operation source and literal child address without encoding or transferring owners.
    pub fn matches_source<M:'static>(&self,slot:&str,child_id:&str,maximum_operations:usize)->bool{self.owner.matches_source(std::any::TypeId::of::<M>(),slot,child_id,maximum_operations)}
    pub fn take_retirement_provider<M:Send+'static>(&mut self)->Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>{
        let owner=self.owner.as_any_mut().downcast_mut::<TypedChildEmitPreparation<M>>()?;
        if owner.retire_builtin.is_some()||owner.remaining.is_some()||owner.current.is_some()||owner.retirement.is_some(){return None;}
        owner.factory.take()
    }
    /// 🎟️ Advances one preparation unit under the complete caller grant: copy paces encoding, capacity funds births, release retires displaced owners.
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<ChildEmitPreparationStep,Fault>{self.owner.step(grant)}
    /// 📏️ The minimal grant on every independent axis that the next `step` needs.
    pub fn step_demands(&self)->Result<RetirementDemand,ValueError>{self.owner.step_demands()}
    pub fn preparation_is_ready(&self)->bool{self.owner.preparation_is_ready()}
    pub fn take_ready(&mut self)->Option<ChildEmit>{self.owner.take_ready()}
    pub fn next_owned_capacity_byte_demand(&self)->usize{self.owner.next_owned_capacity_byte_demand()}
    pub fn take_ready_owned(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(ChildEmit,store::MemberStoreOwnedBatch,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{self.owner.take_ready_owned(grant)}
    pub fn begin_close(&mut self){self.owner.begin_close()}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{self.owner.close_step(grant)}
    pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
    pub fn owner_cell_bytes(&self)->usize{std::mem::size_of_val(self.owner.as_ref())}
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owner.retirement_demands(body)}
    pub fn refusal(&self)->Option<&::protocol::ProtocolError>{self.owner.refusal()}
    pub fn retirement_refusal(&self)->Option<&semio_framework_value::ValueError>{self.owner.retirement_refusal()}
    pub fn accepted_prefix(&self)->Option<&ChildEmit>{self.owner.accepted_prefix()}
    pub fn retained_operation_count(&self)->usize{self.owner.retained_operation_count()}
}
impl Drop for ChildEmitPreparation {
    fn drop(&mut self){assert!(std::thread::panicking()||self.owner.terminal_is_empty(),"child emission preparation dropped before its exact typed owners and accepted prefix were returned or retired");}
}
struct TypedChildEmitPreparation<M> {
    remaining:ManuallyDrop<Option<std::vec::IntoIter<M>>>,
    owned_source:ManuallyDrop<Option<Vec<M>>>,
    batch_capacity_bytes:usize,
    take_batch:Option<fn(Vec<M>,semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(store::MemberStoreOwnedBatch,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,Vec<M>)>>,
    backing_bytes:usize,
    operation_count:usize,
    current:ManuallyDrop<Option<M>>,
    retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    retired_schema:ManuallyDrop<Option<String>>,
    prefix:Option<ChildEmit>,
    cause:ManuallyDrop<Option<::protocol::ProtocolError>>,
    close_refusal:ManuallyDrop<Option<semio_framework_value::ValueError>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    closing_factory:ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    retire_builtin:Option<(BuiltinAdmit<M>,BuiltinDemand<M>)>,
    schema_parts:fn(&M)->(&'static str,&'static str),
    schema_offset:usize,
    schema_started:bool,
    closing:bool,
    ready:bool,
}
impl<M> TypedChildEmitPreparation<M>
where M:Send+'static {
    fn retire_stage_is_empty(&self)->bool{
        self.close_refusal.is_none()&&self.retired_schema.is_none()&&self.retirement.is_none()&&self.current.is_none()&&self.owned_source.as_ref().is_none_or(Vec::is_empty)&&self.remaining.as_ref().is_none_or(|owner|owner.len()==0)&&self.backing_bytes==0&&self.remaining.is_none()&&self.owned_source.is_none()
    }
    fn retire_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(error)=self.close_refusal.as_ref(){return Ok(RetirementDemand{release_bytes:owned_message_bytes(error),depth:1,..Default::default()});}
        if let Some(schema)=self.retired_schema.as_ref(){return Ok(RetirementDemand{release_bytes:schema.capacity(),depth:1,..Default::default()});}
        if let Some(retirement)=self.retirement.as_ref(){return store::artifact_retirement_box_demands(retirement,body);}
        if let Some(operation)=self.current.as_ref(){
            return match self.retire_builtin{
                Some((_,demand))=>demand(&self.current),
                None=>Ok(RetirementDemand{capacity_bytes:self.factory.as_ref().expect("nonterminal owned mutation retains its exact retirement factory").retirement_birth_bytes(operation),depth:2,..Default::default()}),
            };
        }
        if self.owned_source.as_ref().is_some_and(|owner|!owner.is_empty())||self.remaining.as_ref().is_some_and(|owner|owner.len()!=0){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if self.backing_bytes!=0{return Ok(RetirementDemand{release_bytes:self.backing_bytes,depth:1,..Default::default()});}
        Ok(RetirementDemand{depth:usize::from(!self.retire_stage_is_empty()),..Default::default()})
    }
    fn retire_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let idle=RetainedCloneProgress::default();
        if self.retire_stage_is_empty(){return Ok(RetainedCloneStep::Complete(idle));}
        let demand=self.retire_demands(grant.maximum_copy_bytes).map_err(value_fault)?;
        if grant.maximum_depth<demand.depth{return Err(Fault::from("owned-child-emission-retirement-depth"));}
        if yields(grant,demand){return Ok(RetainedCloneStep::Progress(idle));}
        if let Some(error)=self.close_refusal.as_ref(){
            if !self.closing{return Err(retirement_refusal_fault(error));}
            self.close_refusal.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle}));
        }
        if self.retired_schema.is_some(){
            self.retired_schema.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle}));
        }
        if self.retirement.is_some(){
            return match store::artifact_retirement_box_close_step(&mut self.retirement,grant){
                Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),
                Err(error)=>{let fault=retirement_refusal_fault(&error);*self.close_refusal=Some(error);Err(fault)}
            };
        }
        if self.current.is_some(){
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let outcome=match self.retire_builtin{
                Some((admit,_))=>admit(&mut self.current,&mut self.retirement,grant),
                None=>{
                    let operation=self.current.take().expect("observed current operation");
                    match self.factory.as_ref().expect("nonterminal owned mutation retains its exact retirement factory").retire_owned(operation,child){
                        Ok((box_,progress))=>{
                            *self.retirement=Some(box_);
                            if !progress.fits(child)||progress.retained_capacity_bytes!=demand.capacity_bytes{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned mutation constructor changed its receipt"))}else{Ok(RetainedCloneStep::Progress(progress))}
                        },
                        Err((error,operation))=>{*self.current=Some(operation);Err(error)}
                    }
                }
            };
            return outcome.map_err(|error|{let fault=retirement_refusal_fault(&error);*self.close_refusal=Some(error);fault});
        }
        if self.owned_source.as_ref().is_some_and(|owner|!owner.is_empty()){
            *self.current=self.owned_source.as_mut().unwrap().pop();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}));
        }
        if self.remaining.as_ref().is_some_and(|owner|owner.len()!=0){
            *self.current=self.remaining.as_mut().expect("remaining owner exists").next();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}));
        }
        if self.backing_bytes!=0{
            let released_bytes=self.backing_bytes;self.backing_bytes=0;self.remaining.take();self.owned_source.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..idle}));
        }
        self.remaining.take();self.owned_source.take();
        Ok(RetainedCloneStep::Complete(idle))
    }
    fn schema_length(&self)->Result<Option<usize>,ValueError>{
        let Some(operation)=self.owned_source.as_ref().and_then(|source|source.first())else{return Ok(None)};
        let(entity,kind)=(self.schema_parts)(operation);
        entity.len().checked_add(1).and_then(|length|length.checked_add(kind.len())).map(Some).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"owned-child-schema-layout-overflow"))
    }
}
impl<M> ChildEmitPreparationOwner for TypedChildEmitPreparation<M>
where M:Send+'static {
    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}
    fn matches_source(&self,mutation_type:std::any::TypeId,slot:&str,child_id:&str,maximum_operations:usize)->bool{
        mutation_type==std::any::TypeId::of::<M>()&&!self.closing&&!self.ready&&self.operation_count!=0&&self.operation_count<=maximum_operations&&self.prefix.as_ref().is_some_and(|prefix|prefix.slot==slot&&prefix.child_id==child_id)
    }
    fn preparation_is_ready(&self)->bool{self.ready}
    fn step_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.ready||self.cause.is_some(){return Ok(Default::default());}
        if self.retired_schema.is_some()||self.retirement.is_some()||self.current.is_some(){return self.retire_demands(0);}
        if self.owned_source.is_some(){
            let Some(length)=self.schema_length()?else{return Ok(Default::default())};
            let prefix=self.prefix.as_ref().expect("exact owned prefix");
            if !self.schema_started{return Ok(RetirementDemand{depth:1,..Default::default()});}
            if prefix.op_schema.0.capacity()<length{return Ok(RetirementDemand{capacity_bytes:length,release_bytes:prefix.op_schema.0.capacity(),depth:1,..Default::default()});}
            if self.schema_offset<length{return Ok(RetirementDemand{copy_bytes:4.min(length-self.schema_offset),depth:1,..Default::default()});}
            return Ok(Default::default());
        }
        if self.remaining.as_ref().is_some_and(|owner|owner.len()!=0){return Ok(RetirementDemand{depth:1,..Default::default()});}
        self.retire_demands(0)
    }
    fn step(&mut self,grant:RetainedCloneGrant)->Result<ChildEmitPreparationStep,Fault>{
        let idle=RetainedCloneProgress::default();
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(ChildEmitPreparationStep::Pending(idle));}
        if self.closing{return Err(Fault::from("owned-child-emission-step-after-close"));}
        if let Some(cause)=self.cause.as_ref(){return Ok(ChildEmitPreparationStep::Refused(cause.to_fault(),idle));}
        if self.ready{return Ok(ChildEmitPreparationStep::Ready(idle));}
        let demand=self.step_demands().map_err(value_fault)?;
        if grant.maximum_depth<demand.depth||yields(grant,demand){return Ok(ChildEmitPreparationStep::Pending(idle));}
        if self.retired_schema.is_some()||self.retirement.is_some()||self.current.is_some(){return Ok(ChildEmitPreparationStep::Pending(self.retire_one(grant)?.progress()));}
        if self.owned_source.is_some(){
            if let Some(length)=self.schema_length().map_err(value_fault)?{
                let(entity,kind)=(self.schema_parts)(self.owned_source.as_ref().and_then(|source|source.first()).expect("quoted first operation"));
                let prefix=self.prefix.as_mut().expect("exact owned prefix");
                if !self.schema_started{
                    *self.retired_schema=Some(std::mem::take(&mut prefix.op_schema.0));self.schema_started=true;
                    return Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}));
                }
                if prefix.op_schema.0.capacity()<length{
                    let released_bytes=prefix.op_schema.0.capacity();
                    prefix.op_schema.0.try_reserve_exact(length).map_err(|_|Fault::from("owned-child-schema-allocation-refused"))?;
                    return Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:prefix.op_schema.0.capacity(),released_bytes,..idle}));
                }
                if self.schema_offset<length{
                    let before=self.schema_offset;let mut offset=before;let mut remaining=grant.maximum_copy_bytes.min(64);
                    for part in [entity,".",kind]{
                        if offset>=part.len(){offset-=part.len();continue;}
                        let mut count=(part.len()-offset).min(remaining);while count!=0&&!part.is_char_boundary(offset+count){count-=1;}
                        if count==0{break;}prefix.op_schema.0.push_str(&part[offset..offset+count]);self.schema_offset+=count;remaining-=count;offset=0;if remaining==0{break;}
                    }
                    return Ok(ChildEmitPreparationStep::Pending(RetainedCloneProgress{copied_items:1,copied_bytes:self.schema_offset-before,..idle}));
                }
            }
            self.ready=true;return Ok(ChildEmitPreparationStep::Ready(RetainedCloneProgress{copied_items:1,..idle}));
        }
        if self.remaining.as_ref().is_some_and(|source|source.len()!=0){return Err(Fault::from("owned-child-wire-preview-awaits-original-incremental-encoding"));}
        let step=self.retire_one(grant)?;let progress=step.progress();
        if !matches!(step,RetainedCloneStep::Complete(_))||progress!=idle||self.factory.is_some(){return Ok(ChildEmitPreparationStep::Pending(progress));}
        self.ready=true;Ok(ChildEmitPreparationStep::Ready(RetainedCloneProgress{copied_items:1,..idle}))
    }
    fn take_ready(&mut self)->Option<ChildEmit>{
        if !self.ready||self.closing||self.retired_schema.is_some()||self.current.is_some()||self.retirement.is_some()||self.remaining.is_some()||self.owned_source.is_some(){return None;}
        if self.factory.is_some(){return None;}
        self.prefix.take()
    }
    fn next_owned_capacity_byte_demand(&self)->usize{if self.owned_source.is_some(){self.batch_capacity_bytes}else{0}}
    fn take_ready_owned(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(ChildEmit,store::MemberStoreOwnedBatch,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{
        if !self.ready||self.closing||self.retired_schema.is_some()||self.current.is_some()||self.retirement.is_some(){return Ok(None);}
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<self.batch_capacity_bytes{return Ok(None);}
        let Some(take)=self.take_batch else{return Ok(None)};
        let Some(source)=self.owned_source.take()else{return Ok(None)};
        match take(source,grant){
            Ok((batch,progress))=>{self.backing_bytes=0;Ok(Some((self.prefix.take().expect("ready typed owner retains its wire prefix"),batch,progress)))},
            Err((error,source))=>{*self.owned_source=Some(source);if error.kind==semio_framework_value::ValueRefusalKind::AllocationFailed{Ok(None)}else{Err(error)}}
        }
    }
    fn begin_close(&mut self){self.closing=true;}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{
        let idle=RetainedCloneProgress::default();
        if !self.closing{return Err(Fault::from("owned-child-emission-close-before-admission"));}
        if self.terminal_is_empty(){return Ok(PluginLifecycleStep::Complete(idle));}
        let demand=match self.retirement_demands(grant.maximum_copy_bytes){
            Ok(demand)=>demand,
            Err(error)=>{let fault=retirement_refusal_fault(&error);*self.close_refusal=Some(error);return Err(fault);}
        };
        if grant.maximum_depth<demand.depth{return Err(Fault::from("owned-child-emission-close-depth"));}
        if yields(grant,demand){return Ok(PluginLifecycleStep::Progress(idle));}
        if !self.retire_stage_is_empty(){
            let step=self.retire_one(grant)?;
            if step.progress()!=idle{return Ok(PluginLifecycleStep::Progress(step.progress()));}
        }
        if let Some(prefix)=self.prefix.as_mut(){
            if prefix.terminal_is_empty(){self.prefix.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));}
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let step=prefix.close_one(child).map_err(value_fault)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,prefix.terminal_is_empty(),"original child preparation prefix").map_err(value_fault)?;
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if self.cause.is_some(){
            let step=match close_protocol_owned_cause_one(&mut self.cause,grant){
                Ok(step)=>step,
                Err(error)=>{let fault=retirement_refusal_fault(&error);*self.close_refusal=Some(error);return Err(fault);}
            };
            if !matches!(step,PluginLifecycleStep::Complete(_)){return Ok(step);}
        }
        if let Some(factory)=self.factory.take(){
            let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;
            *self.closing_factory=Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));
        }
        if let Some(factory)=self.closing_factory.as_mut(){
            let step=factory.step(grant).map_err(value_fault)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,factory.terminal_is_empty(),"original child preparation factory").map_err(value_fault)?;
            if factory.terminal_is_empty(){drop(self.closing_factory.take());}
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        Ok(PluginLifecycleStep::Complete(idle))
    }
    fn terminal_is_empty(&self)->bool{
        self.remaining.is_none()&&self.owned_source.is_none()&&self.current.is_none()&&self.retirement.is_none()&&self.retired_schema.is_none()&&self.prefix.is_none()&&self.cause.is_none()&&self.close_refusal.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()&&self.backing_bytes==0
    }
    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.retire_stage_is_empty(){return self.retire_demands(body);}
        if let Some(prefix)=self.prefix.as_ref(){
            if prefix.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
            let mut demand=prefix.retirement_demands()?;
            demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original prefix depth overflow"))?;
            return Ok(demand);
        }
        if self.cause.is_some(){return ::protocol::protocol_error_retirement_demand(&self.cause);}
        if self.factory.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(factory)=self.closing_factory.as_ref(){return factory.demands(body);}
        Ok(Default::default())
    }
    fn refusal(&self)->Option<&::protocol::ProtocolError>{self.cause.as_ref()}
    fn retirement_refusal(&self)->Option<&semio_framework_value::ValueError>{self.close_refusal.as_ref()}
    fn accepted_prefix(&self)->Option<&ChildEmit>{self.prefix.as_ref()}
    fn retained_operation_count(&self)->usize{self.owned_source.as_ref().map_or(0,Vec::len)+self.remaining.as_ref().map_or(0,ExactSizeIterator::len)+usize::from(self.current.is_some())+usize::from(self.retirement.is_some())}
}
impl<M> Drop for TypedChildEmitPreparation<M>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||(self.remaining.is_none()&&self.owned_source.is_none()&&self.current.is_none()&&self.retirement.is_none()&&self.retired_schema.is_none()&&self.prefix.is_none()&&self.cause.is_none()&&self.close_refusal.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()),"typed child emission lost actual operation, prefix or encoder refusal owners");
        unsafe{ManuallyDrop::drop(&mut self.remaining);ManuallyDrop::drop(&mut self.owned_source);ManuallyDrop::drop(&mut self.current);ManuallyDrop::drop(&mut self.retirement);ManuallyDrop::drop(&mut self.cause);ManuallyDrop::drop(&mut self.retired_schema);ManuallyDrop::drop(&mut self.close_refusal);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.closing_factory);}
    }
}


pub(crate) fn close_protocol_owned_cause_one(cause:&mut Option<::protocol::ProtocolError>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,ValueError>{
    ::protocol::close_protocol_error_one(cause,grant).map(|step|PluginLifecycleStep::retained(step,cause.is_none()))
}

pub(crate) fn retirement_refusal_fault(error:&semio_framework_value::ValueError)->Fault{
    Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,"interactive-job.child-emission-retirement-refused",error.message.as_ref()).with_param("refusalKind",error.kind.as_str()).with_retained_progress(error.retained_progress())
}

#[cfg(test)]
#[path="🎟️receipt/🧪️tests/🦀️.rs"]
mod original_receipt_tests;
