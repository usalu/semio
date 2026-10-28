//! 📏️ Heap-free native JSON reservation census for original retained receiving.
use semio_framework_value::{DslValue, Number, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use serde::{Deserialize,Serialize};

#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct StagedFormLimits {pub utf8_bytes:usize,pub nodes:usize,pub depth:usize}
impl StagedFormLimits{
    pub const FRAMEWORK:Self=Self{utf8_bytes:1048576,nodes:65536,depth:64};
    pub fn admitted(self)->Result<Self,ValueError>{let caps=Self::FRAMEWORK;if self.utf8_bytes==0||self.utf8_bytes>caps.utf8_bytes||self.nodes==0||self.nodes>caps.nodes||self.depth==0||self.depth>caps.depth{Err(invalid())}else{Ok(self)}}
    /// 📦️ Standalone Value record framing remains separate from the ordinary view mount ceiling.
    pub fn wire_bytes(self)->Result<usize,ValueError>{let limits=self.admitted()?;limits.nodes.checked_mul(32).and_then(|bytes|bytes.checked_add(limits.utf8_bytes)).and_then(|bytes|bytes.checked_add(32)).ok_or_else(resource)}
}
#[derive(Clone,Copy,Default,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase")]
pub struct StagedFormCensus {pub utf8_bytes:usize,pub nodes:usize,pub depth:usize}
#[derive(Clone,Copy,Default,semio_framework_value::RetireOwned)]
struct Frame {phase:u8,next:usize,selected:usize,offset:usize}
#[derive(semio_framework_value::RetireOwned)]
pub struct StagedFormCensusCursor {limits:StagedFormLimits,source:Option<usize>,frames:[Frame;64],length:usize,metrics:StagedFormCensus,canonical_keys:bool,finished:bool,failed:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.source-invalid")}
fn resource()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit")}
fn char_bytes(character:char)->usize{match character{'"'|'\\'|'\u{8}'|'\t'|'\n'|'\u{c}'|'\r'=>2,c if (c as u32)<32=>6,c=>c.len_utf8()}}
impl StagedFormCensusCursor{
    pub fn new(limits:StagedFormLimits)->Result<Self,ValueError>{Ok(Self{limits:limits.admitted()?,source:None,frames:[Frame::default();64],length:1,metrics:Default::default(),canonical_keys:false,finished:false,failed:false})}
    /// 🔐️ Production wire maps must keep strict UTF8 key order, including every nested object.
    pub fn new_wire(limits:StagedFormLimits)->Result<Self,ValueError>{let mut cursor=Self::new(limits)?;cursor.canonical_keys=true;Ok(cursor)}
    pub fn result(&self)->Option<StagedFormCensus>{self.finished.then_some(self.metrics)}
    pub fn demand(&self)->RetirementDemand{if self.finished{Default::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<Frame>()+std::mem::size_of::<StagedFormCensus>()+3*std::mem::size_of::<usize>(),depth:self.length.max(1),..Default::default()}}}
    fn reserve(&mut self,bytes:usize)->Result<(),ValueError>{let next=self.metrics.utf8_bytes.checked_add(bytes).ok_or_else(resource)?;if next>self.limits.utf8_bytes{return Err(resource());}self.metrics.utf8_bytes=next;Ok(())}
    fn pop(&mut self){self.length-=1;self.finished=self.length==0;}
    fn push(&mut self)->Result<(),ValueError>{if self.length>=self.limits.depth||self.length>=self.frames.len(){return Err(resource());}self.frames[self.length]=Frame::default();self.length+=1;Ok(())}
    fn value<'a>(&self,root:&'a DslValue)->Result<&'a DslValue,ValueError>{
        let mut value=root;
        for frame in &self.frames[..self.length-1]{value=match value{DslValue::Array(values)=>values.get(frame.selected),DslValue::Object(values)=>values.get(frame.selected).map(|(_,value)|value),_=>None}.ok_or_else(invalid)?;}
        Ok(value)
    }
    fn advance(&mut self,root:&DslValue)->Result<(),ValueError>{
        let value=self.value(root)?;let top=self.length-1;
        match self.frames[top].phase{
            0=>{
                self.metrics.nodes=self.metrics.nodes.checked_add(1).ok_or_else(resource)?;if self.metrics.nodes>self.limits.nodes{return Err(resource());}self.metrics.depth=self.metrics.depth.max(self.length);
                match value{
                    DslValue::Null=>{self.reserve(4)?;self.pop();},
                    DslValue::Bool(value)=>{self.reserve(if *value{4}else{5})?;self.pop();},
                    DslValue::Number(Number::Float(value)) if !value.is_finite()=>return Err(invalid()),
                    DslValue::Number(_)=>{self.reserve(32)?;self.pop();},
                    DslValue::String(_)=>{self.reserve(2)?;self.frames[top].phase=3;},
                    DslValue::Array(_)|DslValue::Object(_)=>{self.reserve(2)?;self.frames[top].phase=1;},
                    _=>return Err(invalid()),
                }
            },
            1=>{
                let length=match value{DslValue::Array(values)=>values.len(),DslValue::Object(values)=>values.len(),_=>return Err(invalid())};
                let next=self.frames[top].next;if next==length{self.pop();return Ok(());}
                self.reserve(usize::from(next>0))?;self.frames[top].selected=next;self.frames[top].next+=1;
                if matches!(value,DslValue::Object(_)){self.reserve(2)?;self.frames[top].phase=if self.canonical_keys&&next>0{4}else{2};self.frames[top].offset=0;}else{self.push()?;}
            },
            2=>{
                let DslValue::Object(values)=value else{return Err(invalid())};let key=&values.get(self.frames[top].selected).ok_or_else(invalid)?.0;
                if let Some(character)=key.get(self.frames[top].offset..).ok_or_else(invalid)?.chars().next(){self.reserve(char_bytes(character))?;self.frames[top].offset+=character.len_utf8();}else{self.reserve(1)?;self.frames[top].phase=1;self.push()?;}
            },
            3=>{
                let DslValue::String(text)=value else{return Err(invalid())};
                if let Some(character)=text.get(self.frames[top].offset..).ok_or_else(invalid)?.chars().next(){self.reserve(char_bytes(character))?;self.frames[top].offset+=character.len_utf8();}else{self.pop();}
            },
            4=>{
                let DslValue::Object(values)=value else{return Err(invalid())};let selected=self.frames[top].selected;
                let current=values.get(selected).ok_or_else(invalid)?.0.as_bytes();let previous=values.get(selected.checked_sub(1).ok_or_else(invalid)?).ok_or_else(invalid)?.0.as_bytes();
                let offset=self.frames[top].offset;
                match(current.get(offset),previous.get(offset)){
                    (Some(left),Some(right))if left==right=>self.frames[top].offset+=1,
                    (Some(left),Some(right))if left>right=>{self.frames[top].phase=2;self.frames[top].offset=0;},
                    (Some(_),None)=>{self.frames[top].phase=2;self.frames[top].offset=0;},
                    _=>return Err(invalid()),
                }
            },
            _=>return Err(invalid()),
        }
        Ok(())
    }
    /// 🧮️ One original source action returns its full receipt for the caller's retained ledger.
    pub fn step(&mut self,source:&DslValue,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let address=source as *const _ as usize;
        if self.failed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"refused staged census cannot resume"));}
        if self.source.is_some_and(|original|original!=address){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged census requires its original stationary source"));}
        if self.finished{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.demand();
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        self.source=Some(address);if let Err(error)=self.advance(source){self.failed=true;return Err(error.with_retained_progress(progress));}
        Ok(if self.finished{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
