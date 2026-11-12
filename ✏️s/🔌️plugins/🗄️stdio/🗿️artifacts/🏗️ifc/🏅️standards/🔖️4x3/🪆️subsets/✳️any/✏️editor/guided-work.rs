//! 🧭️ The retained controller prepares one donated request against its actual canonical document.
use super::*;
use semio_plugin::action_staging::StagedFormRequest;
use semio_framework_job::{admit_original_construction_settled,OriginalConstructionAdmission,OriginalConstructionDemand,OriginalSourceBinding};
use crate::details::{costed,owned_step,receive_effect,retire_self,settled};
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome};
use semio_framework_value::retained_clone::RetainedCloneClose;
use crate::templates::application::work::staging_native::{IfcTemplateSourceWork,IfcPreparedTemplateSource};
use semio_plugin_decl_staged_form::{StagedBodyCursor,StagedBodyOutput,StagedBodySource};
use semio_framework_value::{RetainedCloneStep,RetirementDemand};
use crate::details::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource,Ifc4x3DetailsProfile};

#[derive(Clone,Copy,PartialEq,Eq)]
enum GuidedKind{Template,TypeInput}
impl GuidedKind{fn action(action:&str)->Option<Self>{if action==APPLY_TEMPLATE_ACTION{Some(Self::Template)}else if action==editing::SET_SNAPSHOT_VALUE_ACTION_ID{Some(Self::TypeInput)}else{None}}}
enum GuidedProducer{
    Template{work:Option<IfcTemplateSourceWork>,source:Option<IfcPreparedTemplateSource>},
    TypeInput{work:Option<IfcTypeInputSourceWork<Ifc4x3DetailsProfile>>,source:Option<IfcPreparedTypeInputSource>},
}
impl GuidedProducer{
    fn kind(&self)->GuidedKind{match self{Self::Template{..}=>GuidedKind::Template,Self::TypeInput{..}=>GuidedKind::TypeInput}}
    fn new(kind:GuidedKind)->Self{match kind{GuidedKind::Template=>Self::Template{work:Some(Default::default()),source:None},GuidedKind::TypeInput=>Self::TypeInput{work:Some(Default::default()),source:None}}}
    fn received(&self)->bool{match self{Self::Template{work,..}=>work.as_ref().is_some_and(IfcTemplateSourceWork::original_request_received),Self::TypeInput{work,..}=>work.as_ref().is_some_and(IfcTypeInputSourceWork::original_request_received)}}
    fn has_source(&self)->bool{match self{Self::Template{source,..}=>source.is_some(),Self::TypeInput{source,..}=>source.is_some()}}
    fn context_matches(&self,window:Option<&str>,locale:semio_ui_locale::Locale)->Result<bool,ValueError>{match self{Self::Template{work,source}=>if let Some(source)=source{source.context_matches(window,locale)}else{work.as_ref().ok_or_else(invalid)?.context_matches(window,locale)},Self::TypeInput{work,source}=>if let Some(source)=source{source.context_matches(window,locale)}else{work.as_ref().ok_or_else(invalid)?.context_matches(window,locale)}}}
    fn receive(&mut self,request:&mut Option<StagedFormRequest>,snapshot:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Template{work,..}=>work.as_mut().ok_or_else(invalid)?.receive(request,snapshot,revision,cx),Self::TypeInput{work,..}=>work.as_mut().ok_or_else(invalid)?.receive(request,snapshot,revision,cx)}}
    fn advance(&mut self,owner:&std::sync::Arc<IfcSnapshot>,snapshot:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Template{work,..}=>work.as_mut().ok_or_else(invalid)?.advance(snapshot,revision,cx),Self::TypeInput{work,..}=>work.as_mut().ok_or_else(invalid)?.advance(owner,revision,cx)}}
    fn take(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Template{work,source}=>work.as_mut().ok_or_else(invalid)?.take(source,cx),Self::TypeInput{work,source}=>work.as_mut().ok_or_else(invalid)?.take(source,cx)}}
    fn body_demand(&self,body:&StagedBodyCursor)->Result<RetirementDemand,ValueError>{match self{Self::Template{source,..}=>body.demand(source.as_ref().ok_or_else(invalid)?),Self::TypeInput{source,..}=>body.demand(source.as_ref().ok_or_else(invalid)?)}}
    fn step_body(&self,body:&mut StagedBodyCursor,output:&mut StagedBodyOutput,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Template{source,..}=>build_body(source.as_ref().ok_or_else(invalid)?,body,output,cx),Self::TypeInput{source,..}=>build_body(source.as_ref().ok_or_else(invalid)?,body,output,cx)}}
}

pub(super) struct GuidedWork{
    request:Option<StagedFormRequest>,producer:GuidedProducer,body:Option<StagedBodyCursor>,output:Option<StagedBodyOutput>,phase:u8,original:Option<GuidedBinding>,retirement:RetainedCloneClose,closing:bool,
}
#[derive(Clone,Copy)]
struct GuidedBinding(OriginalSourceBinding);
semio_framework_value::artifact_retire_leaf!(GuidedBinding);
impl GuidedWork{fn new(kind:GuidedKind)->Self{Self{request:None,producer:GuidedProducer::new(kind),body:Some(Default::default()),output:Some(Default::default()),phase:0,original:None,retirement:Default::default(),closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.native-controller")}
fn demand(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn metadata(cx:&mut StepContext<'_>,bytes:usize,change:impl FnOnce())->Result<bool,OriginalTurnFailure>{
    let grant=cx.retained_grant();if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"io.cancelled").into());}if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false);}change();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
impl GuidedWork{
    pub(super) fn supports(action:&str)->bool{GuidedKind::action(action).is_some()}
    pub(super) fn receive(slot:&mut Option<Box<Self>>,adm:&mut Option<OriginalConstructionAdmission>,donor:&mut Option<StagedFormRequest>,cx:&mut StepContext<'_>)->Result<Option<()>,OriginalTurnFailure>{
        let kind=GuidedKind::action(&donor.as_ref().ok_or_else(invalid)?.token.action).ok_or_else(invalid)?;
        if slot.is_none(){let bytes=std::mem::size_of::<Self>();let physical=OriginalConstructionDemand{items:1,copy_bytes:std::mem::size_of::<Box<Self>>()+bytes,capacity_bytes:bytes,release_bytes:0,depth:1};admit_original_construction_settled(cx,adm.get_or_insert_with(Default::default),slot,physical,1,|_,_|costed(Ok::<_,ValueError>((Box::new(Self::new(kind)),RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:bytes,..Default::default()}))))?;return Ok(None);}
        let work=slot.as_deref_mut().unwrap();if work.phase!=0||work.request.is_some()||donor.is_none()||work.original.is_some()||work.producer.kind()!=kind{return Err(invalid().into());}let binding=GuidedBinding(OriginalSourceBinding::borrow(cx));
        Ok(metadata(cx,2*std::mem::size_of::<StagedFormRequest>()+std::mem::size_of::<GuidedBinding>(),||{work.request=donor.take();work.original=Some(binding);})?.then_some(()))
    }
    pub(super) fn demand(&self)->Result<RetirementDemand,ValueError>{
        if self.phase==3{return self.producer.body_demand(self.body.as_ref().ok_or_else(invalid)?);}
        let mut required=demand(std::mem::size_of::<GuidedProducer>()+2*std::mem::size_of::<StagedFormRequest>()+std::mem::size_of::<Self>());if let GuidedProducer::TypeInput{work,..}=&self.producer{let child=work.as_ref().ok_or_else(invalid)?.demands()?;required.copy_bytes=required.copy_bytes.max(child.copy_bytes);required.capacity_bytes=required.capacity_bytes.max(child.capacity_bytes);required.release_bytes=required.release_bytes.max(child.release_bytes);required.depth=required.depth.max(child.depth);}Ok(required)
    }
    pub(super) fn render(&mut self,source:&std::sync::Arc<IfcSnapshot>,doc:&ArtifactView<'_,IfcSnapshot>,revision:[u8;32],view:&semio_plugin::ViewModel,cx:&mut StepContext<'_>)->semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree>{
        if self.original.is_none_or(|binding|!binding.0.matches(cx)){return Err(invalid().into());}let matches=if let Some(request)=&self.request{request.context_matches(view.window_id.as_deref(),view.locale)}else{self.producer.context_matches(view.window_id.as_deref(),view.locale)?};if !matches{return Err(invalid().into());}
        match self.phase{
            0=>{if self.producer.received(){metadata(cx,std::mem::size_of::<u8>(),||self.phase=1)?;}else{self.producer.receive(&mut self.request,doc.snapshot,revision,cx)?;}},
            1=>{if self.producer.advance(source,doc.snapshot,revision,cx)?{metadata(cx,std::mem::size_of::<u8>(),||self.phase=2)?;}},
            2=>{if self.producer.has_source(){metadata(cx,std::mem::size_of::<u8>(),||self.phase=3)?;}else{self.producer.take(cx)?;}},
            3=>{if self.producer.step_body(self.body.as_mut().ok_or_else(invalid)?,self.output.as_mut().ok_or_else(invalid)?,cx)?{metadata(cx,std::mem::size_of::<u8>(),||self.phase=4)?;}},
            4=>return self.output.as_mut().ok_or_else(invalid)?.take_tree(cx).map_err(Into::into),
            _=>return Err(invalid().into()),
        }Ok(None)
    }
}
fn build_body(source:&impl StagedBodySource,body:&mut StagedBodyCursor,output:&mut StagedBodyOutput,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    let output=output.donor()?;let demand=body.demand(source)?;let mut complete=false;
    receive_effect(cx,demand,|native,effect|{let grant=effect.grant();let result=body.step(source,native,output,grant);complete=matches!(result,Ok(RetainedCloneStep::Complete(_)));RetainedWorkOutcome{result,work:effect.performed_work()}})?;
    Ok(complete)
}
impl GuidedWork{
    pub(super) fn begin_close(&mut self){self.closing=true;}
    pub(super) fn empty(&self)->bool{self.request.is_none()&&self.producer.empty()&&self.body.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
    pub(super) fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<StagedFormRequest>(false,&None,body);}
        match &self.producer{
            GuidedProducer::Template{work,source}=>{if work.is_some(){return self.retirement.next_owner_demands::<IfcTemplateSourceWork>(true,&None,body);}if source.is_some(){return self.retirement.next_owner_demands::<IfcPreparedTemplateSource>(true,&None,body);}}
            GuidedProducer::TypeInput{work,source}=>{if work.is_some(){return self.retirement.next_owner_demands::<IfcTypeInputSourceWork<Ifc4x3DetailsProfile>>(true,&None,body);}if source.is_some(){return self.retirement.next_owner_demands::<IfcPreparedTypeInputSource>(true,&None,body);}}
        }
        if self.body.is_some(){return self.retirement.next_owner_demands::<StagedBodyCursor>(true,&None,body);}
        if self.output.is_some(){return self.retirement.next_owner_demands::<StagedBodyOutput>(true,&None,body);}
        if self.request.is_some(){return self.retirement.next_owner_demands::<StagedFormRequest>(true,&None,body);}
        Ok(Default::default())
    }
    pub(super) fn close_owned(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        if !self.retirement.is_empty(){return settled(self.retirement.step_granted(body,effect));}
        match &mut self.producer{
            GuidedProducer::Template{work,source}=>{if work.is_some(){return owned_step(self.retirement.begin_granted(work,effect),"Guided template work omitted its owner");}if source.is_some(){return owned_step(self.retirement.begin_granted(source,effect),"Guided template source omitted its owner");}}
            GuidedProducer::TypeInput{work,source}=>{if work.is_some(){return owned_step(self.retirement.begin_granted(work,effect),"Guided type input work omitted its owner");}if source.is_some(){return owned_step(self.retirement.begin_granted(source,effect),"Guided type input source omitted its owner");}}
        }
        if self.body.is_some(){return owned_step(self.retirement.begin_granted(&mut self.body,effect),"Guided body omitted its owner");}
        if self.output.is_some(){return owned_step(self.retirement.begin_granted(&mut self.output,effect),"Guided output omitted its owner");}
        if self.request.is_some(){return owned_step(self.retirement.begin_granted(&mut self.request,effect),"Guided request omitted its owner");}
        RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
    }
}
impl GuidedProducer{
    fn empty(&self)->bool{match self{Self::Template{work,source}=>work.is_none()&&source.is_none(),Self::TypeInput{work,source}=>work.is_none()&&source.is_none()}}
}
retire_self!([] GuidedWork);
