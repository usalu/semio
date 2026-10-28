//! 🧳️ One original borrowed payload becomes bounded UI values through existing arena ownership.
use semio_framework_ui_contract::{UiMapBuilder,UiListBuilder,UiText,UiValue,UiTypedRetirementCursor};
use semio_framework_value::{DslValue,Number,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::mem::{ManuallyDrop,size_of};
pub const STAGED_PAYLOAD_DEPTH:usize=16;
pub const STAGED_PAYLOAD_MAP_MEMBERS:usize=32;
#[derive(Clone,Copy)]
pub enum StagedPayloadKind<'a>{Null,Bool(bool),Number(f64),Text(&'a str),List(usize),Map(usize)}
pub struct StagedPayloadNode<'a>{pub key:Option<&'a str>,pub value:StagedPayloadKind<'a>}
pub trait StagedPayloadSource {fn identity(&self)->usize;fn node(&self,path:&[usize])->Result<StagedPayloadNode<'_>,ValueError>;}
pub struct StagedDslPayload<'a>{pub source:&'a DslValue}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.payload")}
fn resource()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit")}
impl StagedPayloadSource for StagedDslPayload<'_>{
    fn identity(&self)->usize{self.source as*const _ as usize}
    fn node(&self,path:&[usize])->Result<StagedPayloadNode<'_>,ValueError>{
        let mut current=self.source;let mut key=None;
        for index in path{match current{DslValue::Object(rows)=>{let row=rows.get(*index).ok_or_else(invalid)?;key=Some(row.0.as_str());current=&row.1;},DslValue::Array(rows)=>{key=None;current=rows.get(*index).ok_or_else(invalid)?;},_=>return Err(invalid())}}
        let value=dsl_kind(current)?;Ok(StagedPayloadNode{key,value})
    }
}
pub(super) fn dsl_kind(current:&DslValue)->Result<StagedPayloadKind<'_>,ValueError>{
        Ok(match current{
            DslValue::Null=>StagedPayloadKind::Null,DslValue::Bool(value)=>StagedPayloadKind::Bool(*value),DslValue::String(value)=>StagedPayloadKind::Text(value),
            DslValue::Number(value)=>{let value=match value{Number::UInt(value)if *value<=9_007_199_254_740_991=>*value as f64,Number::Int(value)if (-9_007_199_254_740_991..=9_007_199_254_740_991).contains(value)=>*value as f64,Number::Float(value)if value.is_finite()&&!(*value==0.0&&value.is_sign_negative())=>*value,_=>return Err(resource())};StagedPayloadKind::Number(value)},
            DslValue::Array(rows)=>StagedPayloadKind::List(rows.len()),DslValue::Object(rows)=>StagedPayloadKind::Map(rows.len()),_=>return Err(invalid()),
        })
}
enum Collection {Map(ManuallyDrop<UiMapBuilder>),List(ManuallyDrop<UiListBuilder>)}
impl Collection {fn finish(self)->UiValue{match self{Self::Map(value)=>UiValue::Map(ManuallyDrop::into_inner(value).finish_initialized()),Self::List(value)=>UiValue::List(ManuallyDrop::into_inner(value).finish_initialized())}}}
struct Frame {collection:Collection,parent_key:Option<UiText>,next:usize,len:usize}
pub struct StagedPayloadCursor {
    frames:ManuallyDrop<[Option<Frame>;STAGED_PAYLOAD_DEPTH]>,depth:usize,path:[usize;STAGED_PAYLOAD_DEPTH],source:Option<usize>,
    pending:ManuallyDrop<Option<(Option<UiText>,UiValue)>>,retiring:UiTypedRetirementCursor,failed:bool,closing:bool,done:bool,
}
impl Default for StagedPayloadCursor{fn default()->Self{Self{frames:ManuallyDrop::new(std::array::from_fn(|_|None)),depth:0,path:[0;STAGED_PAYLOAD_DEPTH],source:None,pending:ManuallyDrop::new(None),retiring:UiTypedRetirementCursor::empty(),failed:false,closing:false,done:false}}}
impl StagedPayloadCursor{
    pub(super) fn adopt_for_close(&mut self,value:UiValue){assert!(self.terminal_is_empty());*self.pending=Some((None,value));self.closing=true;}
    pub fn demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Frame>()+2*size_of::<UiValue>(),depth:STAGED_PAYLOAD_DEPTH,..Default::default()}}
    fn advance(&mut self,source:&impl StagedPayloadSource,output:&mut Option<UiValue>)->Result<bool,ValueError>{
        if self.pending.is_some(){
            if self.depth==0{*output=self.pending.take().map(|(_,value)|value);self.done=true;return Ok(true);}
            let frame=self.frames[self.depth-1].as_mut().ok_or_else(invalid)?;
            let(key,value)=self.pending.take().unwrap();
            let result=match &mut frame.collection{
                Collection::Map(builder)=>{
                    let Some(key)=key else{*self.pending=Some((None,value));return Err(invalid());};let mut donor=Some((key,value));let result=builder.insert_initialized(&mut donor);if let Some((key,value))=donor{*self.pending=Some((Some(key),value));}result
                },
                Collection::List(builder)=>{
                    if key.is_some(){*self.pending=Some((key,value));return Err(invalid());}let mut donor=Some(value);let result=builder.push_initialized(&mut donor);if let Some(value)=donor{*self.pending=Some((None,value));}result
                },
            }.map_err(|reason|ValueError::literal(ValueRefusalKind::OwnershipLimit,reason))?;
            if result{frame.next+=1;}return Ok(result);
        }
        if self.depth>0{
            let frame=self.frames[self.depth-1].as_ref().ok_or_else(invalid)?;
            if frame.next==frame.len{let frame=self.frames[self.depth-1].take().unwrap();self.depth-=1;*self.pending=Some((frame.parent_key,frame.collection.finish()));return Ok(true);}
            if self.depth==STAGED_PAYLOAD_DEPTH{return Err(resource());}self.path[self.depth-1]=frame.next;
        }
        let node=source.node(&self.path[..self.depth])?;
        let key=node.key.map(|value|UiText::try_from_str(value).ok_or_else(resource)).transpose()?;
        let value=match node.value{
            StagedPayloadKind::Null=>UiValue::Null,StagedPayloadKind::Bool(value)=>UiValue::Bool(value),StagedPayloadKind::Number(value)if value.is_finite()&&! (value==0.0&&value.is_sign_negative())=>UiValue::Number(value),
            StagedPayloadKind::Text(value)=>UiValue::Text(UiText::try_from_str(value).ok_or_else(resource)?),
            StagedPayloadKind::Map(len)|StagedPayloadKind::List(len)=>{
                if self.depth==STAGED_PAYLOAD_DEPTH||len>semio_framework_ui_contract::UI_VALUE_MAX_ITEMS||matches!(node.value,StagedPayloadKind::Map(_))&&len>STAGED_PAYLOAD_MAP_MEMBERS{return Err(resource());}
                let collection=match node.value{StagedPayloadKind::Map(_)=>UiMapBuilder::try_new_initialized().map_err(|reason|ValueError::literal(ValueRefusalKind::UnsupportedOwner,reason))?.map(|value|Collection::Map(ManuallyDrop::new(value))),_=>UiListBuilder::try_new_initialized().map_err(|reason|ValueError::literal(ValueRefusalKind::UnsupportedOwner,reason))?.map(|value|Collection::List(ManuallyDrop::new(value)))};
                let Some(collection)=collection else{return Ok(false)};self.frames[self.depth]=Some(Frame{collection,parent_key:key,next:0,len});self.depth+=1;return Ok(true);
            },
            _=>return Err(resource()),
        };*self.pending=Some((key,value));Ok(true)
    }
    /// 🎟️ Source ownership stays private and stationary; the enclosing original work posts this complete receipt.
    pub fn step(&mut self,source:&impl StagedPayloadSource,output:&mut Option<UiValue>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.failed||self.closing{return Err(invalid());}if self.done{return Ok(RetainedCloneStep::Complete(Default::default()));}
        if self.source.is_some_and(|identity|identity!=source.identity())||output.is_some(){self.failed=true;return Err(invalid());}
        let demand=self.demand();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.source=Some(source.identity());let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match self.advance(source,output){Ok(true)=>Ok(if self.done{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),Ok(false)=>Ok(RetainedCloneStep::Progress(progress)),Err(error)=>{self.failed=true;Err(error.with_retained_progress(progress))}}
    }
    pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}
        Ok(RetirementDemand{copy_bytes:size_of::<Frame>()+size_of::<UiValue>(),release_bytes:if let Some((_,value))=self.pending.as_ref(){self.retiring.original_release_demand(value).map_err(|reason|ValueError::literal(ValueRefusalKind::InvariantViolated,reason))?}else{0},depth:STAGED_PAYLOAD_DEPTH,..Default::default()})
    }
    pub fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.closing=true;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.close_demand()?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some((_,value))=self.pending.as_mut(){let step=self.retiring.advance(value,1,grant.maximum_copy_bytes).map_err(|reason|ValueError::literal(ValueRefusalKind::InvariantViolated,reason))?;if step.complete{self.pending.take();self.retiring=UiTypedRetirementCursor::empty();}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),copied_bytes:if step.progressed{demand.copy_bytes}else{0},released_bytes:demand.release_bytes,..Default::default()}));}
        let frame=self.frames[self.depth-1].take().unwrap();self.depth-=1;*self.pending=Some((frame.parent_key,frame.collection.finish()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
    pub fn terminal_is_empty(&self)->bool{self.depth==0&&self.pending.is_none()}
}
impl Drop for StagedPayloadCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"staged payload requires original paid retirement");}}
impl RetireOwned for StagedPayloadCursor{
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for StagedPayloadCursor{
    fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{self.close_demand()}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.close(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{StagedPayloadCursor::terminal_is_empty(self)}
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.copy_bytes)}
    fn next_selected_work_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.close_demand()?.copy_bytes)}
}
