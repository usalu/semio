//! 🪟️ One private completed page becomes ordinary accessible controls without graph queries or model copies.
use super::*;
use semio_framework::action_staging::{StagedFormStatus,StagedFormBinding,StagedFormPageBinding,StagedFormBindingEdit};
use semio_framework_ui_contract::*;
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy)]
pub enum StagedBodyBinding<'a>{Action(&'a StagedFormBinding),Page(&'a StagedFormPageBinding)}
pub struct StagedBodyControl<'a>{pub scalar:StagedScalarControl<'a>,pub required:bool,pub error:Option<&'a str>,pub binding:Option<StagedBodyBinding<'a>>}
/// 🧭️ Every accessor indexes already-completed private metadata; source preparation owns all expensive queries.
pub trait StagedBodySource{fn identity(&self)->usize;fn title(&self)->&str;fn status(&self)->&StagedFormStatus;fn controls(&self)->usize;fn control(&self,index:usize)->Result<StagedBodyControl<'_>,ValueError>;}
pub struct StagedBodyCursor{
    source:Option<(usize,usize)>,root:ManuallyDrop<Option<BuiltNode>>,field:ManuallyDrop<Option<BuiltNode>>,leaf:ManuallyDrop<Option<BuiltNode>>,args:ManuallyDrop<Option<UiValue>>,
    payload:StagedPayloadCursor,scalar:StagedScalarCursor,retiring:StagedScalarCursor,phase:u8,index:usize,closing:bool,failed:bool,
}
impl Default for StagedBodyCursor{fn default()->Self{Self{source:None,root:ManuallyDrop::new(None),field:ManuallyDrop::new(None),leaf:ManuallyDrop::new(None),args:ManuallyDrop::new(None),payload:Default::default(),scalar:Default::default(),retiring:Default::default(),phase:0,index:0,closing:false,failed:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.body")}
fn resource()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit")}
fn text(value:&str)->Result<UiText,ValueError>{UiText::try_from_str(value).ok_or_else(resource)}
fn metadata()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<StagedPayloadCursor>()+size_of::<StagedScalarCursor>()+2*size_of::<BuiltNode>(),depth:4,..Default::default()}}
fn fits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn paid(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
pub fn staged_body_composition(source:&impl StagedBodySource)->Result<(),ValueError>{
    let caps=semio_framework::action_staging::STAGED_FORM_BODY_LIMITS;if source.controls()>caps.controls{return Err(resource());}let(mut content,mut pages)=(0usize,0usize);
    for index in 0..source.controls(){if matches!(source.control(index)?.binding,Some(StagedBodyBinding::Page(_))){pages+=1;}else{content+=1;}if content>caps.content||pages>caps.pages{return Err(resource());}}Ok(())
}
fn control_resources(control:&StagedBodyControl<'_>)->Result<(),ValueError>{
    if let StagedScalarKind::Choice{choices,..}=control.scalar.kind{let limit=UiText::default().capacity();if choices.len()>32||choices.iter().any(|choice|choice.key.len()>limit){return Err(resource());}if let Some(StagedBodyBinding::Action(binding))=control.binding{if binding.edits.len()>8{return Err(invalid());}for edit in &binding.edits{if let StagedFormBindingEdit::Choice{options,..}=edit{if options.len()>32{return Err(invalid());}if options.iter().any(|option|option.key.len()>limit){return Err(resource());}}}}}Ok(())
}
fn control_binding(control:&StagedBodyControl<'_>,status:&StagedFormStatus)->bool{
    match(control.scalar.kind,control.binding){
        (StagedScalarKind::ReadOnly{..},None)=>true,
        (StagedScalarKind::Button{page:true},Some(StagedBodyBinding::Page(binding)))=>binding.token==status.token,
        (kind,Some(StagedBodyBinding::Action(binding)))=>{
            if binding.token!=status.token||binding.edits.is_empty()||binding.edits.len()>8{return false;}
            let mut text=0;let mut choice=None;for edit in &binding.edits{match edit{StagedFormBindingEdit::Text{..}=>text+=1,StagedFormBindingEdit::Choice{options,..}=>{if choice.is_some(){return false;}choice=Some(options);},_=>{}}}
            match kind{StagedScalarKind::Text{..}=>text==1&&choice.is_none(),StagedScalarKind::Choice{choices,..}=>text==0&&choices.len()<=32&&choice.is_some_and(|options|options.len()==choices.len()&&options.iter().zip(choices).all(|(option,choice)|option.key.len()<=size_of::<UiText>()&&choice.key.len()<=size_of::<UiText>()&&option.key==choice.key)),StagedScalarKind::Button{page:false}=>text==0&&choice.is_none(),_=>false}
        },_=>false,
    }
}
fn container(key:UiText,label:&str,role:ContainerRole,required:Option<bool>,error:Option<&str>)->Result<BuiltNode,ValueError>{
    BuiltNode::try_new(key,Component::Container(ContainerProps{role,label:Some(Label(text(label)?)),description:None,required,error:error.map(text).transpose()?,default_open:None,drop_overlay:None})).map_err(|_|resource())
}
impl StagedBodyCursor{
    fn status_spec<'a>(&self,source:&'a impl StagedBodySource)->StagedScalarControl<'a>{StagedScalarControl{owner:source.identity(),address:u64::MAX,controller:"framework",label:source.title(),description:None,disabled:false,kind:StagedScalarKind::Status}}
    pub fn demand(&self,source:&impl StagedBodySource)->Result<RetirementDemand,ValueError>{Ok(match self.phase{
        1|4=>self.payload.demand(),2=>self.scalar.demand(&self.status_spec(source))?,3|8=>self.root.as_ref().ok_or_else(invalid)?.children.initialized_push_demand(),5=>self.scalar.demand(&source.control(self.index)?.scalar)?,7=>self.field.as_ref().ok_or_else(invalid)?.children.initialized_push_demand(),_=>metadata(),
    })}
    fn advance(&mut self,source:&impl StagedBodySource,native:&mut NativeDecodeControl<'_>,output:&mut Option<BuiltNode>,grant:RetainedCloneGrant,demand:RetirementDemand)->Result<RetainedCloneProgress,ValueError>{
        match self.phase{
            0=>{staged_body_composition(source)?;*self.root=Some(container(text("stage.form")?,source.title(),ContainerRole::Form,None,None)?);self.phase=1;},
            1=>{let step=self.payload.step(&StagedStatusPayload{status:source.status()},&mut self.args,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;}return Ok(step.progress());},
            2=>{let step=self.scalar.step(&self.status_spec(source),&mut self.args,&mut self.leaf,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3;}return Ok(step.progress());},
            3=>{let progress=self.root.as_mut().ok_or_else(invalid)?.children.push_initialized(&mut self.leaf,native,grant)?;if self.leaf.is_none(){self.phase=9;}return Ok(progress);},
            4=>{let control=source.control(self.index)?;control_resources(&control)?;if !control_binding(&control,source.status()){return Err(invalid());}if let Some(binding)=control.binding{let step=match binding{StagedBodyBinding::Action(value)=>self.payload.step(&StagedBindingPayload::Action(value),&mut self.args,grant),StagedBodyBinding::Page(value)=>self.payload.step(&StagedBindingPayload::Page(value),&mut self.args,grant)}?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5;}return Ok(step.progress());}self.phase=5;},
            5=>{let control=source.control(self.index)?;let step=self.scalar.step(&control.scalar,&mut self.args,&mut self.leaf,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=6;}return Ok(step.progress());},
            6=>{let control=source.control(self.index)?;if control.scalar.address==u64::MAX{return Err(invalid());}for prior in 0..self.index{if source.control(prior)?.scalar.address==control.scalar.address{return Err(invalid());}}let key=UiText::try_format(format_args!("stage.field.{}",control.scalar.address)).ok_or_else(resource)?;*self.field=Some(container(key,control.scalar.label,ContainerRole::Field,Some(control.required),control.error)?);self.phase=7;},
            7=>{let progress=self.field.as_mut().ok_or_else(invalid)?.children.push_initialized(&mut self.leaf,native,grant)?;if self.leaf.is_none(){self.phase=8;}return Ok(progress);},
            8=>{let progress=self.root.as_mut().ok_or_else(invalid)?.children.push_initialized(&mut self.field,native,grant)?;if self.field.is_none(){self.index+=1;self.phase=9;}return Ok(progress);},
            9=>{self.payload=Default::default();self.scalar=Default::default();self.phase=if self.index<source.controls(){4}else{10};},
            10=>{*output=self.root.take();self.phase=11;},
            _=>{},
        }Ok(paid(demand))
    }
    /// 🎟️ The enclosing original source supplies native receiving and posts the unchanged physical receipt.
    pub fn step(&mut self,source:&impl StagedBodySource,native:&mut NativeDecodeControl<'_>,output:&mut Option<BuiltNode>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.failed||self.closing{return Err(invalid());}if self.phase==11{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let identity=(source.identity(),source.status()as*const _ as usize);if self.source.is_some_and(|old|old!=identity)||output.is_some(){self.failed=true;return Err(invalid());}
        let demand=self.demand(source)?;if !fits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}self.source=Some(identity);
        match self.advance(source,native,output,grant,demand){Ok(progress)=>Ok(if self.phase==11{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),Err(error)=>{self.failed=true;let mut progress=error.retained_progress();if progress.copied_items==0{progress.copied_items=1;progress.copied_bytes=demand.copy_bytes;}Err(error.with_retained_progress(progress))}}
    }
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.scalar.terminal_is_empty(){return self.scalar.close_demand(body);}if !self.payload.terminal_is_empty(){return self.payload.close_demand();}if !self.retiring.terminal_is_empty(){return self.retiring.close_demand(body);}Ok(if self.terminal_is_empty(){Default::default()}else{metadata()})
    }
    pub fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.closing=true;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.close_demand(grant.maximum_copy_bytes)?;if !fits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.scalar.terminal_is_empty(){self.scalar.close(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
        else if !self.payload.terminal_is_empty(){self.payload.close(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
        else if !self.retiring.terminal_is_empty(){self.retiring.close(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
        else{if let Some(value)=self.args.take(){self.payload.adopt_for_close(value);}else if let Some(node)=self.leaf.take().or_else(||self.field.take()).or_else(||self.root.take()){self.retiring=Default::default();self.retiring.adopt_for_close(node);}Ok(RetainedCloneStep::Progress(paid(demand)))}
    }
    pub fn terminal_is_empty(&self)->bool{self.root.is_none()&&self.field.is_none()&&self.leaf.is_none()&&self.args.is_none()&&self.payload.terminal_is_empty()&&self.scalar.terminal_is_empty()&&self.retiring.terminal_is_empty()}
}
impl Drop for StagedBodyCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"staged body requires original paid retirement");}}
impl RetireOwned for StagedBodyCursor{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for StagedBodyCursor{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.close(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}fn terminal_is_empty(&self)->bool{StagedBodyCursor::terminal_is_empty(self)}fn next_close_byte_demand(&self)->Option<usize>{Some(0)}fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(usize::MAX)?.copy_bytes)}fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.copy_bytes)}
}
