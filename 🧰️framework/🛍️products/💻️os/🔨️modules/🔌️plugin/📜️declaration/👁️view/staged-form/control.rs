//! ⌨️ Native scalar controls consume exact prepared bindings under explicit original receiving demands.
use semio_framework_ui_contract::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
pub struct StagedScalarChoice<'a>{pub key:&'a str,pub label:&'a str}
#[derive(Clone,Copy)]
pub enum StagedScalarKind<'a>{Text{value:&'a str},Choice{value:&'a str,choices:&'a[StagedScalarChoice<'a>]},Button{page:bool},ReadOnly{value:&'a str},Status}
pub struct StagedScalarControl<'a>{pub owner:usize,pub address:u64,pub controller:&'a str,pub label:&'a str,pub description:Option<&'a str>,pub disabled:bool,pub kind:StagedScalarKind<'a>}
pub struct StagedScalarCursor {identity:Option<(usize,u64)>,node:ManuallyDrop<Option<BuiltNode>>,retiring:ManuallyDrop<Option<Box<OriginalBuiltTreeRetirement>>>,phase:u8,choice:usize,failed:bool,closing:bool}
impl Default for StagedScalarCursor{fn default()->Self{Self{identity:None,node:ManuallyDrop::new(None),retiring:ManuallyDrop::new(None),phase:0,choice:0,failed:false,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.control")}
fn resource()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit")}
fn text(value:&str)->Result<UiText,ValueError>{UiText::try_from_str(value).ok_or_else(resource)}
fn fits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl StagedScalarCursor {
    pub(super) fn adopt_for_close(&mut self,node:BuiltNode){assert!(self.terminal_is_empty());*self.node=Some(node);self.closing=true;}
    pub fn demand(&self,spec:&StagedScalarControl<'_>)->Result<RetirementDemand,ValueError>{
        let mut demand=RetirementDemand{copy_bytes:size_of::<BuiltNode>()+4*size_of::<UiText>(),depth:4,..Default::default()};
        if let Some(node)=self.node.as_ref(){
            if self.phase==1{if let Component::Select(props)=&node.component{if self.choice<match &spec.kind{StagedScalarKind::Choice{choices,..}=>choices.len(),_=>0}{demand.capacity_bytes=if props.items.has_reserved_slot(){0}else{props.items.next_allocation_bytes().map_err(|_|resource())?};}}}
            if self.phase==2&&!matches!(spec.kind,StagedScalarKind::ReadOnly{..}|StagedScalarKind::Status){demand.capacity_bytes=if node.bindings.has_reserved_slot(){0}else{node.bindings.next_allocation_bytes().map_err(|_|resource())?};}
        }Ok(demand)
    }
    fn initialize(&mut self,spec:&StagedScalarControl<'_>)->Result<(),ValueError>{
        let label=Label(text(spec.label)?);let component=match &spec.kind{
            StagedScalarKind::Text{value}=>Component::Input(InputProps{value:text(value)?,kind:InputKind::Text,draft_target:Some(UiText::try_format(format_args!("stage.control.{}",spec.address)).ok_or_else(resource)?),..Default::default()}),
            StagedScalarKind::Choice{value,choices}=>{if choices.len()>32{return Err(resource());}Component::Select(SelectProps{value:text(value)?,items:Default::default(),placeholder:Some(label.clone()),appearance:SelectAppearance::Menu})},
            StagedScalarKind::Button{..}=>Component::Button(ButtonProps{icon:UiText::default(),label:label.clone()}),
            StagedScalarKind::ReadOnly{value}=>Component::Text(TextProps{value:Label(text(value)?),emphasize:None,data_attributes:None}),
            StagedScalarKind::Status=>Component::Extension(ExtensionProps{extension:text(semio_framework::action_staging::STAGED_FORM_STATUS_EXTENSION)?,props:UiValue::Null}),
        };
        let key=UiText::try_format(format_args!("stage.control.{}",spec.address)).ok_or_else(resource)?;
        let mut node=BuiltNode::try_new(key,component).map_err(|_|resource())?;node.disabled=spec.disabled;node.accessibility.label=Some(label);node.accessibility.description=spec.description.map(|value|text(value).map(Label)).transpose()?;
        *self.node=Some(node);self.phase=1;Ok(())
    }
    fn advance(&mut self,spec:&StagedScalarControl<'_>,args:&mut Option<UiValue>,output:&mut Option<BuiltNode>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let demand=self.demand(spec)?;if !fits(grant,demand){return Ok(Default::default());}
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match self.phase{
            0=>self.initialize(spec)?,
            1=>{
                if let StagedScalarKind::Choice{choices,..}=&spec.kind{if self.choice<choices.len(){
                    let node=self.node.as_mut().ok_or_else(invalid)?;let Component::Select(props)=&mut node.component else{return Err(invalid())};
                    if !props.items.has_reserved_slot(){let reserved=props.items.try_reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::literal(ValueRefusalKind::AllocationFailed,error.reason).with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;progress.retained_capacity_bytes=reserved.allocated_bytes;return Ok(progress);}
                    let choice=&choices[self.choice];if choice.key.is_empty()||choices[..self.choice].iter().any(|previous|previous.key==choice.key){return Err(invalid());}
                    let choice=SelectItem{value:text(choice.key)?,label:Label(text(choice.label)?)};props.items.try_push_reserved(choice).map_err(|_|invalid())?;self.choice+=1;return Ok(progress);
                }}self.phase=2;
            },
            2=>{
                if matches!(spec.kind,StagedScalarKind::ReadOnly{..}){if args.is_some(){return Err(invalid());}self.phase=3;return Ok(progress);}
                if matches!(spec.kind,StagedScalarKind::Status){let node=self.node.as_mut().ok_or_else(invalid)?;let Component::Extension(props)=&mut node.component else{return Err(invalid())};props.props=args.take().ok_or_else(invalid)?;self.phase=3;return Ok(progress);}
                let node=self.node.as_mut().ok_or_else(invalid)?;
                if !node.bindings.has_reserved_slot(){let reserved=node.bindings.try_reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::literal(ValueRefusalKind::AllocationFailed,error.reason).with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;progress.retained_capacity_bytes=reserved.allocated_bytes;return Ok(progress);}
                let (trigger,action)=match spec.kind{StagedScalarKind::Button{page:true}=>(Trigger::Activate,semio_framework::action_staging::STAGED_FORM_PAGE_ACTION),StagedScalarKind::Button{page:false}=>(Trigger::Activate,semio_framework::action_staging::STAGED_FORM_EVENT_ACTION),_=>(Trigger::Change,semio_framework::action_staging::STAGED_FORM_EVENT_ACTION)};
                let action=ActionId::try_v1(spec.controller,action).ok_or_else(resource)?;if args.is_none(){return Err(invalid());}
                let binding=ActionBinding{trigger,action,args:args.take(),capability:None};
                if let Err(mut binding)=node.bindings.try_push_reserved(binding){*args=binding.args.take();return Err(invalid());}self.phase=3;
            },
            3=>{*output=self.node.take();self.phase=4;},
            _=>{},
        }Ok(progress)
    }
    /// 🎟️ The enclosing original render owns immutable completed metadata and posts every returned receipt.
    pub fn step(&mut self,spec:&StagedScalarControl<'_>,args:&mut Option<UiValue>,output:&mut Option<BuiltNode>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.failed||self.closing{return Err(invalid());}if self.phase==4{return Ok(RetainedCloneStep::Complete(Default::default()));}
        if self.identity.is_some_and(|identity|identity!=(spec.owner,spec.address))||output.is_some(){self.failed=true;return Err(invalid());}
        let demand=self.demand(spec)?;if !fits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}self.identity=Some((spec.owner,spec.address));
        match self.advance(spec,args,output,grant){Ok(progress)=>Ok(if self.phase==4{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),Err(error)=>{self.failed=true;let mut progress=error.retained_progress();progress.copied_items=1;progress.copied_bytes=demand.copy_bytes;Err(error.with_retained_progress(progress))}}
    }
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.retiring.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Box<OriginalBuiltTreeRetirement>>(),release_bytes:size_of::<OriginalBuiltTreeRetirement>(),depth:1,..Default::default()});}let mut demand=owner.next_demand(body)?;demand.copy_bytes=demand.copy_bytes.max(size_of::<OriginalBuiltTreeRetirement>()+size_of::<BuiltNode>());return Ok(demand);}
        Ok(if self.node.is_some(){RetirementDemand{copy_bytes:size_of::<OriginalBuiltTreeRetirement>()+size_of::<BuiltNode>(),capacity_bytes:size_of::<OriginalBuiltTreeRetirement>(),depth:1,..Default::default()}}else{Default::default()})
    }
    pub fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.closing=true;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.close_demand(grant.maximum_copy_bytes)?;if !fits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=self.retiring.as_mut(){if owner.terminal_is_empty(){self.retiring.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}return match owner.step(grant){Ok(step)=>{let mut progress=step.progress();if progress.copied_items!=0{progress.copied_bytes=demand.copy_bytes;}Ok(RetainedCloneStep::Progress(progress))},Err(error)=>{let mut progress=error.retained_progress();progress.copied_items=1;progress.copied_bytes=demand.copy_bytes;Err(error.with_retained_progress(progress))}};}
        *self.retiring=Some(Box::new(OriginalBuiltTreeRetirement::new(self.node.take().unwrap())));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
    }
    pub fn terminal_is_empty(&self)->bool{self.node.is_none()&&self.retiring.is_none()}
}
impl Drop for StagedScalarCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"staged scalar control requires original paid retirement");}}
impl RetireOwned for StagedScalarCursor{
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for StagedScalarCursor{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.close(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{StagedScalarCursor::terminal_is_empty(self)}
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(usize::MAX)?.copy_bytes)}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.copy_bytes)}
}
