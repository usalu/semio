//! 🩹️ Original patch JSON retains its exact source view and decoded cells until typed admission or retirement.
use super::super::{kernel,SnapshotPatch,SNAPSHOT_PATCH_MAX_BYTES,SNAPSHOT_PATCH_MAX_INDEX,SNAPSHOT_PATCH_MAX_SEGMENTS};
use semio_framework_pack_json::{JsonReadSource,JsonReadLimits,JsonMemberPolicy,JsonSourceCursor};
use semio_framework_value::{DslValue,Number,NativeDecodeControl,ValueError,ValueRefusalKind};

#[derive(Clone,Copy)]
struct PatchSource<'source>(kernel::codec::ByteSpan<'source>);
impl JsonReadSource for PatchSource<'_>{fn byte_len(&self)->usize{self.0.len()}fn byte_at(&self,index:usize)->Option<u8>{self.0.get(index).copied()}}

/// 🪟️ One caller retains the original operation borrow and every partially decoded owner across refusal.
pub struct SnapshotPatchReadCursor<'source>{parser:JsonSourceCursor<PatchSource<'source>,DslValue>,candidate:Option<DslValue>,patch:Option<SnapshotPatch>,admitted:bool}
impl<'source> SnapshotPatchReadCursor<'source>{
    /// 🛂️ Captures the complete original decoder policy before input read or semantic allocation.
    pub fn new(source:kernel::codec::ByteSpan<'source>,options:&kernel::codec::PackDecodeOptions)->Result<Self,ValueError>{
        let limits=JsonReadLimits{maximum_bytes:options.limits.max_file_len.min(SNAPSHOT_PATCH_MAX_BYTES as u64),maximum_allocation_bytes:options.limits.max_total_alloc.min(usize::MAX as u64)as usize,maximum_depth:usize::from(options.limits.max_depth),maximum_items:options.limits.max_items};
        Ok(Self{parser:JsonSourceCursor::new_with_limits(PatchSource(source),JsonMemberPolicy::Reject,limits)?,candidate:None,patch:None,admitted:false})
    }
    /// ⏱️ Advances the original grammar while retaining accepted semantic cells on typed refusal.
    pub fn step(&mut self,maximum_units:usize,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
        if maximum_units==0{return Ok(false);}if self.candidate.is_some(){return Ok(true);}
        if let Some(value)=self.parser.step(maximum_units,control).map_err(|error|error.into_value_error())?{self.candidate=Some(value);return Ok(true);}Ok(false)
    }
    /// 👓️ Borrows the exact accepted semantic candidate without cloning or transferring it.
    pub fn candidate(&self)->Option<&DslValue>{self.candidate.as_ref()}
    /// 📍️ Reports the original source position retained by the grammar.
    pub fn position(&self)->usize{self.parser.position()}
    /// 🧬️ Validates original fields and pointer syntax before moving admitted cells into one typed patch.
    pub fn admit_patch(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        control.scoped_stage(|control|{control.begin_stage(0)?;self.admit_patch_inner(control)})
    }
    fn admit_patch_inner(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        if self.admitted{return Err(refusal("patch candidate was already admitted"));}
        let Some(DslValue::Object(fields))=self.candidate.as_ref()else{return Err(refusal("patch candidate must be a parsed object"));};
        control.checkpoint()?;
        let kind=match text(fields,"operation")?{"set"=>0,"insert"=>1,"remove"=>2,"move"=>3,"rename"=>4,"splice"=>5,_=>return Err(refusal("unknown patch operation"))};
        for(key,_)in fields{control.step()?;let allowed=key=="operation"||key=="path"||match kind{0=>key=="value",1=>key=="value"||key=="index",2=>false,3=>key=="from"||key=="index",4=>key=="key",5=>matches!(key.as_str(),"offset"|"remove"|"value"|"continued"),_=>unreachable!()};if !allowed{return Err(refusal("unknown original patch field"));}}
        pointer(text(fields,"path")?,control)?;
        if kind==3{pointer(text(fields,"from")?,control)?;}if kind==4{text(fields,"key")?;}
        if matches!(kind,0|1|5)&&member(fields,"value").is_none(){return Err(refusal("patch value is absent"));}
        if matches!(kind,1|3){if let Some(value)=member(fields,"index"){ordinal(value)?;}}
        if kind==5{ordinal(member(fields,"offset").ok_or_else(||refusal("splice offset is absent"))?)?;ordinal(member(fields,"remove").ok_or_else(||refusal("splice removal count is absent"))?)?;
            if !matches!(member(fields,"value"),Some(DslValue::Array(_)|DslValue::Object(_)|DslValue::String(_))){return Err(refusal("splice requires an array, object or text value"));}
            if let Some(value)=member(fields,"continued"){if !matches!(value,DslValue::Bool(_)){return Err(refusal("splice continued must be boolean"));}}
        }
        control.checkpoint()?;
        let Some(DslValue::Object(fields))=self.candidate.as_mut()else{unreachable!()};
        let index=member(fields,"index").map(ordinal).transpose()?;
        let continued=matches!(member(fields,"continued"),Some(DslValue::Bool(true)));
        let path=take_text(fields,"path");
        self.patch=Some(match kind{0=>SnapshotPatch::Set{path,value:take(fields,"value")},1=>SnapshotPatch::Insert{path,value:take(fields,"value"),index},2=>SnapshotPatch::Remove{path},3=>SnapshotPatch::Move{from:take_text(fields,"from"),path,index},4=>SnapshotPatch::Rename{path,key:take_text(fields,"key")},5=>SnapshotPatch::Splice{path,offset:ordinal(member(fields,"offset").unwrap()).unwrap(),remove:ordinal(member(fields,"remove").unwrap()).unwrap(),value:take(fields,"value"),continued},_=>unreachable!()});
        self.admitted=true;Ok(())
    }
    /// 🎁️ Transfers the single admitted typed patch while retaining the consumed parse scaffold.
    pub fn take_patch(&mut self)->Option<SnapshotPatch>{self.patch.take()}
    /// ♻️ Returns the parser, semantic scaffold and any untaken typed patch to the actual retirement owner.
    pub fn into_retirement(self)->Box<dyn semio_framework_value::ErasedSnapshotRetirement>{Box::new(PatchReadRetirement{owners:[Some(self.parser.into_retirement()),Some(semio_framework_value::retirement::owned_retirement((self.candidate,self.patch)))]})}
}
struct PatchReadRetirement{owners:[Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>;2]}
impl semio_framework_value::ErasedSnapshotRetirement for PatchReadRetirement{
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_value::SnapshotRetirementStep,ValueError>{
        use semio_framework_value::SnapshotRetirementStep;
        if maximum_items==0||maximum_bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        for slot in &mut self.owners{let Some(owner)=slot.as_mut()else{continue};if !owner.terminal_is_empty(){return owner.close_step(1,maximum_bytes).map(|step|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>other});}let bytes=std::mem::size_of_val(&**owner);if bytes>maximum_bytes{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}slot.take();return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:bytes});}Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{self.owners.iter().all(Option::is_none)}
    fn next_close_byte_demand(&self)->usize{self.owners.iter().find_map(|owner|owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()})).unwrap_or(0)}
}
impl Drop for PatchReadRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.owners.iter().all(Option::is_none),"patch read retirement retains actual parser/candidate owners");}}
fn refusal(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn member<'value>(fields:&'value[(String,DslValue)],name:&str)->Option<&'value DslValue>{fields.iter().find(|(key,_)|key==name).map(|(_,value)|value)}
fn text<'value>(fields:&'value[(String,DslValue)],name:&str)->Result<&'value str,ValueError>{match member(fields,name){Some(DslValue::String(value))=>Ok(value),_=>Err(refusal("required patch field must be text"))}}
fn ordinal(value:&DslValue)->Result<u64,ValueError>{let value=match value{DslValue::Number(Number::UInt(value))=>*value,DslValue::Number(Number::Int(value))if *value>=0=>*value as u64,_=>return Err(refusal("patch ordinal must be a nonnegative integer"))};if value>SNAPSHOT_PATCH_MAX_INDEX||usize::try_from(value).is_err(){return Err(refusal("patch ordinal exceeds the original index authority"));}Ok(value)}
fn pointer(path:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
    if path.is_empty(){return Ok(());}if !path.starts_with('/'){return Err(refusal("patch pointer must be empty or begin with slash"));}
    control.scoped_stage(|control|{control.begin_stage(path.len())?;let mut position=0;let mut segments=0;let bytes=path.as_bytes();while position<bytes.len(){control.checkpoint()?;match bytes[position]{b'/'=>{segments+=1;if segments>SNAPSHOT_PATCH_MAX_SEGMENTS{return Err(refusal("patch pointer exceeds original segment authority"));}},b'~'=>{if !matches!(bytes.get(position+1),Some(b'0'|b'1')){return Err(refusal("patch pointer escape must be tilde-zero or tilde-one"));}position+=1;control.step()?;},_=>{}}position+=1;control.step()?;}Ok(())})
}
fn take(fields:&mut[(String,DslValue)],name:&str)->DslValue{let value=&mut fields.iter_mut().find(|(key,_)|key==name).expect("admitted original field").1;std::mem::replace(value,DslValue::Null)}
fn take_text(fields:&mut[(String,DslValue)],name:&str)->String{match take(fields,name){DslValue::String(value)=>value,_=>unreachable!("admitted original text field")}}
