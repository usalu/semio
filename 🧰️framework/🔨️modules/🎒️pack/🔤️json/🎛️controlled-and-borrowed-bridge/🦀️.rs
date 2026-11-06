//! 🎛️ Controlled and borrowed bridge: the controlled string conversions, the native write nodes and cursors and the controlled reader and writer. See [`super`].

use super::*;

pub fn to_json_string<T: ToValue>(value: &T) -> String {
    to_string(&from_dsl_value(&value.to_value()))
}

pub fn from_json_str<T: FromValue>(text: &str, policy: JsonMemberPolicy) -> Result<T, ValueError> {
    let value = parse(text, policy).map_err(JsonError::into_value_error)?;
    T::from_value(to_dsl_value(&value))
}

pub fn from_json_str_controlled<T: FromValue>(text: &str, policy: JsonMemberPolicy, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<T, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(text.len())?;
        let mut reader = ControlledReader { lexer: Lexer::new(text), policy, control };
        let value = reader.value()?.guard_decoded(); reader.whitespace()?;
        if reader.lexer.pos != text.len() { return Err(JsonError::TrailingData(reader.lexer.pos).into_value_error()); }
        reader.control.scoped_stage(|control| { control.begin_stage(0)?; T::from_value_controlled(value.get(), control) })
    })
}

pub fn to_json_string_controlled<T: ToValue>(value: &T, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?; let value = value.to_value_controlled(control)?.guard_decoded();
        let mut measure = ControlledWriter { bytes: 0, output: None, metadata:0, pending:None, pending_work:0, pending_performed:false }; measure.value(value.get(), control)?;
        control.charge(measure.bytes)?; let mut output = String::new(); output.try_reserve_exact(measure.bytes).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON output allocation failed"))?;
        let mut writer = ControlledWriter { bytes: 0, output: Some(output), metadata:0, pending:None, pending_work:0, pending_performed:false }; writer.value(value.get(), control)?;
        if writer.bytes != measure.bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON output changed after admission")); } Ok(writer.output.unwrap())
    })
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct JsonWriteFrame { pub state: u8, pub(super) index: usize, pub position: usize, pub(super) source_index:usize, pub(super) next_cursor:usize }

enum JsonNativeNode<'a> { Value(&'a DslValue), Byte(u8) }

fn json_native_node<'a>(source: &'a DslValue, path: &[usize]) -> Result<JsonNativeNode<'a>, ValueError> {
    let mut value = source;
    for (depth, index) in path.iter().enumerate() {
        value = match value {
            DslValue::Array(values) => values.get(*index),
            DslValue::Object(values) => values.get(*index).map(|(_, value)| value),
            DslValue::Bytes(values) if depth + 1 == path.len() => return values.get(*index).copied().map(JsonNativeNode::Byte).ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON byte path is absent")),
            _ => None,
        }.ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "JSON writer path is absent"))?;
    }
    Ok(JsonNativeNode::Value(value))
}

#[derive(Clone,Copy)]
pub enum JsonWriteNode<'a>{Null,Bool(bool),Number(semio_framework_value::Number),Float32(f32),QuotedUnsigned(u128),String(&'a str),NativeString(&'a dyn semio_framework_value::paged::Utf8Text),Array(usize),Object(usize)}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JsonWriteEdge{pub child:usize,pub cursor:usize}

pub trait JsonWriteSource{
    fn traversal_edge_at_path(&self,_path:&[usize],index:usize,_cursor:usize)->Result<JsonWriteEdge,ValueError>{Ok(JsonWriteEdge{child:index,cursor:0})}
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>;
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>;
}

impl JsonWriteSource for DslValue{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{Ok(match json_native_node(self,path)?{
        JsonNativeNode::Byte(value)=>JsonWriteNode::Number(semio_framework_value::Number::UInt(u64::from(value))),
        JsonNativeNode::Value(value)=>match value{Self::Null=>JsonWriteNode::Null,Self::Bool(value)=>JsonWriteNode::Bool(*value),Self::Number(value)=>JsonWriteNode::Number(*value),Self::String(value)=>JsonWriteNode::String(value),Self::Array(value)=>JsonWriteNode::Array(value.len()),Self::Bytes(value)=>JsonWriteNode::Array(value.len()),Self::Object(value)=>JsonWriteNode::Object(value.len())},
    })}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match json_native_node(self,path)?{JsonNativeNode::Value(Self::Object(entries))=>entries.get(index).map(|(key,_)|key.as_str()).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"JSON source key is absent")),_=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON source key owner is absent"))}}
}

pub struct JsonBorrowedWriteCursor {
    #[doc(hidden)] pub frames: Vec<JsonWriteFrame>, #[doc(hidden)] pub path: Vec<usize>, writer: ControlledWriter, #[doc(hidden)] pub phase: u8, pub(super) normal_step_progress:RetainedCloneProgress,last_work:u64,pending_action:Option<u64>,pending_extra:RetainedCloneProgress,allocation_actual:Option<usize>,allocation_progress:Option<semio_framework_value::native_encoding::NativeEncodeAllocationProgress>,allocation_pending:Option<semio_framework_value::native_encoding::NativeEncodeAllocationOutcome>,
}

fn writer_metadata_demand<S:JsonWriteSource+?Sized>(cursor:&JsonBorrowedWriteCursor,frame:JsonWriteFrame,node:&JsonWriteNode<'_>,source:&S)->Result<(usize,usize),ValueError>{
 let word=size_of::<usize>();let frame_bytes=size_of::<JsonWriteFrame>();let finish=frame_bytes+word+if cursor.frames.len()>1{2*word}else{0};let child=2*frame_bytes+3*word;
 let(extra,raw)=match(frame.state,node){
  (0,JsonWriteNode::Null|JsonWriteNode::Bool(_)|JsonWriteNode::Number(_))=>(finish,1),
  (0,JsonWriteNode::String(_)|JsonWriteNode::NativeString(_)|JsonWriteNode::Array(_)|JsonWriteNode::Object(_))=>(size_of::<u8>(),1),
  (1,JsonWriteNode::Array(length))=>if frame.index==*length{(finish,1)}else{(word+child,usize::from(frame.index!=0))},
  (2,JsonWriteNode::Object(length))=>if frame.index==*length{(finish,1)}else{(size_of::<u8>()+word,1+usize::from(frame.index!=0))},
  (3,JsonWriteNode::Object(_))=>{let text=source.object_key_at_path(&cursor.path,frame.index)?;let remaining=text.get(frame.position..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON object key position changed"))?;if remaining.is_empty(){(size_of::<u8>(),2)}else{(word,1)}},
  (5,JsonWriteNode::String(text))=>{let remaining=text.get(frame.position..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON text position changed"))?;if remaining.is_empty(){(finish,1)}else{(word,1)}},
  (4,JsonWriteNode::Object(_))=>(size_of::<u8>()+word+child,0),
  (6,JsonWriteNode::NativeString(text))=>{if frame.index==text.text_chunk_count(){(finish,1)}else{let remaining=text.text_chunk(frame.index).and_then(|text|text.get(frame.position..)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text position changed"))?;if remaining.is_empty(){(2*word,0)}else{(word,1)}}},
  _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer source state changed")),
 };Ok((frame_bytes+extra,raw))
}
impl JsonBorrowedWriteCursor {
    fn retain_bound_progress(&mut self){if self.pending_action.is_some(){self.writer.pending=Some(self.normal_step_progress);self.writer.pending_performed=self.last_work!=0;self.writer.pending_work=self.last_work}}
    /// ▶️ Starts without copying or scanning a source owner.
    pub fn new() -> Self { Self { frames: Vec::new(), path: Vec::new(), writer: ControlledWriter { bytes: 0, output: None, metadata:0, pending:None, pending_work:0, pending_performed:false }, phase: 0, normal_step_progress:Default::default(),last_work:0,pending_action:None,pending_extra:Default::default(),allocation_actual:None,allocation_progress:None,allocation_pending:None } }
    /// 📍️ Returns the current canonical output byte count and measure/write phase.
    pub fn progress(&self) -> (usize, bool) { (self.writer.bytes, self.phase == 2) }
    pub fn measured_bytes(&self)->Option<usize>{(self.phase==1&&self.frames.is_empty()).then_some(self.writer.bytes)}
    /// 📦️ Identifies the fully written owner before its separately funded move.
    pub fn completed_output_ready(&self)->bool{self.phase==2&&self.frames.is_empty()&&self.writer.output.is_some()}
    /// 📤️ Moves the finished writer output without traversing or encoding the Source again.
    pub fn take_completed_output(&mut self)->Option<String>{if !self.completed_output_ready(){return None}self.phase=3;self.writer.output.take()}
    pub fn restart_measurement(&mut self)->Result<RetainedCloneProgress,ValueError>{
        if self.measured_bytes().is_none()||self.writer.output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON measurement still owns an unfinished source or output"))}
        self.path.clear();self.writer.bytes=0;self.frames.push(JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0});
        Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<JsonWriteFrame>()+size_of::<usize>(),..Default::default()})
    }
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.writer.pending.unwrap_or(self.normal_step_progress)}
    pub fn last_turn_work(&self)->u64{if self.writer.pending_performed{self.writer.pending_work}else{self.last_work}}
    /// 🧳️ Lends the original allocation markers for explicit original-port recovery.
    pub fn allocation_state_mut(&mut self)->(&mut Option<usize>,&mut Option<semio_framework_value::native_encoding::NativeEncodeAllocationProgress>,&mut Option<semio_framework_value::native_encoding::NativeEncodeAllocationOutcome>){(&mut self.allocation_actual,&mut self.allocation_progress,&mut self.allocation_pending)}
    /// 🪢️ Binds the next actual turn to its original quoted work and physical sources.
    pub fn bind_original_turn(&mut self,settlement:&semio_framework_value::native_encoding::NativeEncodeEffectSettlement,extra:RetainedCloneProgress)->Result<(),ValueError>{if self.pending_action.is_some()||self.writer.pending.is_some()||!settlement.effect_ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original turn is not ready"))}self.pending_action=Some(settlement.identity());self.pending_extra=extra;Ok(())}
    /// 🧾️ Clears a retained turn only with the exact completed original receipt.
    pub fn acknowledge_posted_turn(&mut self,receipt:semio_framework_value::native_encoding::NativeEncodeEffectReceipt)->Result<(),(ValueError,semio_framework_value::native_encoding::NativeEncodeEffectReceipt)>{let expected=match self.writer.pending{Some(progress)=>match progress.checked_add(self.pending_extra){Ok(total)=>total,Err(error)=>return Err((error,receipt))},None=>return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original turn is absent"),receipt))};if self.pending_action!=Some(receipt.identity())||expected!=receipt.progress()||!self.writer.pending_performed{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original receipt does not match its retained turn"),receipt))}self.pending_action=None;self.pending_extra=Default::default();self.writer.pending=None;self.writer.pending_performed=false;self.writer.pending_work=0;Ok(())}
    /// ↩️ Clears only a matching original reservation canceled before any effect.
    pub fn acknowledge_canceled_turn(&mut self,cancellation:semio_framework_value::native_encoding::NativeEncodeEffectCancellation)->Result<(),(ValueError,semio_framework_value::native_encoding::NativeEncodeEffectCancellation)>{if self.pending_action!=Some(cancellation.identity())||self.writer.pending_performed{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original cancellation does not match its retained turn"),cancellation))}self.pending_action=None;self.pending_extra=Default::default();self.writer.pending=None;self.writer.pending_work=0;Ok(())}
    /// 🧮️ Borrows both raw components if their combined receipt cannot be represented.
    pub fn original_receipt_components(&self)->Option<(RetainedCloneProgress,RetainedCloneProgress)>{self.writer.pending.map(|progress|(progress,self.pending_extra))}
    /// 📏️ Quotes one actual scalar write, natural frame birth or final output allocation.
    pub fn normal_step_demands<S:JsonWriteSource+?Sized>(&self,source:&S)->Result<RetirementDemand,ValueError>{self.normal_step_demands_with_copy(source,0)}
    pub fn normal_step_demands_with_copy<S:JsonWriteSource+?Sized>(&self,source:&S,copy:usize)->Result<RetirementDemand,ValueError>{
        let mut demand=RetirementDemand{depth:self.path.len()+1,..Default::default()};
        if self.phase==0{demand.capacity_bytes=(MAX_DEPTH as usize+1)*size_of::<JsonWriteFrame>();demand.copy_bytes=size_of::<Vec<JsonWriteFrame>>()+size_of::<u8>();return Ok(demand);}if self.phase==4{demand.capacity_bytes=MAX_DEPTH as usize*size_of::<usize>();demand.copy_bytes=size_of::<Vec<usize>>()+size_of::<u8>();return Ok(demand);}if self.phase==5{demand.copy_bytes=size_of::<JsonWriteFrame>()+size_of::<u8>();return Ok(demand);}
        if self.frames.is_empty(){if self.phase==1{demand.capacity_bytes=self.writer.bytes;demand.copy_bytes=size_of::<ControlledWriter>()+size_of::<JsonWriteFrame>()+size_of::<u8>();}else if self.phase==2{demand.copy_bytes=size_of::<Option<String>>()+size_of::<u8>();}return Ok(demand);}
        let frame=*self.frames.last().unwrap();let node=source.node_at_path(&self.path)?;let(metadata,raw)=writer_metadata_demand(self,frame,&node,source)?;
        let mut measured=ControlledWriter{bytes:0,output:None,metadata:0,pending:None,pending_work:0,pending_performed:false};let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(isize::MAX as usize,&mut accepted);
        match frame.state{
            0=>match node{JsonWriteNode::Null=>measured.raw("null",&mut control)?,JsonWriteNode::Bool(value)=>measured.raw(if value{"true"}else{"false"},&mut control)?,JsonWriteNode::Number(value)=>measured.number(value,&mut control)?,JsonWriteNode::Float32(value)=>JsonWriteOutput::float32(&mut measured,value,&mut control)?,JsonWriteNode::QuotedUnsigned(value)=>JsonWriteOutput::quoted_unsigned(&mut measured,value,&mut control)?,_=>measured.bytes=1},
            1=>{let JsonWriteNode::Array(length)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next array source changed"));};measured.bytes=usize::from(frame.index==length||frame.index!=0);},
            2=>{let JsonWriteNode::Object(length)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next object source changed"));};measured.bytes=if frame.index==length{1}else{1+usize::from(frame.index!=0)};},
            3|5=>{let text=if frame.state==3{source.object_key_at_path(&self.path,frame.source_index)?}else{let JsonWriteNode::String(text)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next string source changed"));};text};let remaining=text.get(frame.position..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next text position changed"))?;if remaining.is_empty(){measured.bytes=if frame.state==3{2}else{1};}else{let length=json_text_block(remaining,copy).0;for character in remaining[..length].chars(){measured.character(character,&mut control)?;}}},
            6=>{let JsonWriteNode::NativeString(text)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next native string source changed"));};if frame.index==text.text_chunk_count(){measured.bytes=1;}else{let body=text.text_chunk(frame.index).and_then(|text|text.get(frame.position..)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next native text position changed"))?;if !body.is_empty(){let length=json_text_block(body,copy).0;for character in body[..length].chars(){measured.character(character,&mut control)?;}}}},
            _=>{},
        }
        let writing=self.phase==2;let raw_metadata=raw.checked_mul(size_of::<usize>()).and_then(|value|value.checked_mul(1+usize::from(writing))).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer metadata demand overflow"))?;demand.copy_bytes=metadata.checked_add(raw_metadata).and_then(|value|value.checked_add(if writing{measured.bytes}else{0})).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer copy demand overflow"))?;let _=copy;Ok(demand)
    }
    #[doc(hidden)] pub fn retained_capacity_bytes(&self)->Result<usize,ValueError>{
        self.frames.capacity().checked_mul(size_of::<JsonWriteFrame>()).and_then(|frames|self.path.capacity().checked_mul(size_of::<usize>()).and_then(|path|frames.checked_add(path))).and_then(|bytes|bytes.checked_add(self.writer.output.as_ref().map_or(0,|output|output.capacity()))).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer physical backing extent overflow"))
    }
    pub fn step<S:JsonWriteSource+?Sized>(&mut self,source:&S,maximum_units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
        if self.writer.pending.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original turn is unsettled"))}
        self.step_with_demand(source,maximum_units,control,grant).map(|(output,_)|output)
    }
    /// 📏️ Returns the actual unmet cursor demand without traversing the Source a second time.
    pub fn step_with_demand<S:JsonWriteSource+?Sized>(&mut self,source:&S,maximum_units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Result<(Option<String>,Option<RetirementDemand>),ValueError>{
        self.normal_step_progress=Default::default();self.last_work=0;let mut unmet=None;
        self.step_original(source,maximum_units,control,grant,&mut unmet).map(|output|(output,unmet)).map_err(|error|error.with_retained_progress(self.normal_step_progress))
    }
    fn step_original<S:JsonWriteSource+?Sized>(&mut self,source:&S,maximum_units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:RetainedCloneGrant,unmet:&mut Option<RetirementDemand>)->Result<Option<String>,ValueError>{
        if self.phase==3{return Ok(None)}
        for _ in 0..maximum_units {
            let remaining=normal_remaining(grant,self.normal_step_progress)?;let demand=self.normal_step_demands_with_copy(source,remaining.maximum_copy_bytes)?;
            if !normal_permit(demand,remaining)?{*unmet=Some(demand);break;}
            let next_work=self.last_work.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original JSON writer work overflow"))?;
            control.admit_turn_capacity(remaining.maximum_capacity_bytes)?;control.checkpoint()?;
            if self.phase==3{return Ok(None);}
            if self.frames.is_empty()&&self.phase==2{
                self.normal_step_progress=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<String>>()+size_of::<u8>(),..Default::default()})?;
                self.phase=3;self.last_work=next_work;self.retain_bound_progress();return Ok(self.writer.output.take());
            }
            if self.allocation_actual.is_some()||self.allocation_progress.is_some()||self.allocation_pending.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original allocation is unsettled"))}
            if self.phase==0||self.phase==4{
                let frames=self.phase==0;let count=if frames{MAX_DEPTH as usize+1}else{MAX_DEPTH as usize};let layout=if frames{std::alloc::Layout::array::<JsonWriteFrame>(count)}else{std::alloc::Layout::array::<usize>(count)}.map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer frontier extent overflow"))?;let bytes=layout.size();let copied=if frames{size_of::<Vec<JsonWriteFrame>>()}else{size_of::<Vec<usize>>()}+size_of::<u8>();
                let receipt=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:copied,retained_capacity_bytes:bytes,released_bytes:0})?;
                let outcome=control.allocate_effect(bytes,&mut self.allocation_actual,&mut self.allocation_progress,&mut |actual|{let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){*actual=Some(0);self.last_work=next_work;return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"JSON writer frontier allocation failed"))}if frames{self.frames=unsafe{Vec::from_raw_parts(pointer.cast::<JsonWriteFrame>(),0,count)};self.phase=4}else{self.path=unsafe{Vec::from_raw_parts(pointer.cast::<usize>(),0,count)};self.phase=5}*actual=Some(bytes);self.normal_step_progress=receipt;self.last_work=next_work;if self.pending_action.is_some(){self.writer.pending=Some(receipt);self.writer.pending_performed=true;self.writer.pending_work=next_work}Ok(())});
                if !outcome.admitted||outcome.primary.is_some()||outcome.forwarded_quote.is_some()||outcome.scoped_quote.is_some()||outcome.extent_refusal.is_some()||outcome.forwarded_refusal.is_some()||outcome.scoped_refusal.is_some()||outcome.actual!=Some(bytes){self.allocation_pending=Some(outcome);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original allocation refused"))}self.allocation_actual=None;normal_remaining(grant,self.normal_step_progress)?;control.step()?;continue
            }
            if self.phase==5{let receipt=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<JsonWriteFrame>()+size_of::<u8>(),..Default::default()})?;self.frames.push(JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0});self.phase=1;self.normal_step_progress=receipt;self.last_work=next_work;self.retain_bound_progress();normal_remaining(grant,self.normal_step_progress)?;control.step()?;continue}
            if self.frames.is_empty()&&self.phase==1{
                let bytes=self.writer.bytes;let layout=std::alloc::Layout::array::<u8>(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON output extent overflow"))?;let receipt=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ControlledWriter>()+size_of::<JsonWriteFrame>()+size_of::<u8>(),retained_capacity_bytes:bytes,released_bytes:0})?;
                let outcome=control.allocate_effect(bytes,&mut self.allocation_actual,&mut self.allocation_progress,&mut |actual|{let output=if bytes==0{String::new()}else{let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){*actual=Some(0);self.last_work=next_work;if self.pending_action.is_some(){self.writer.pending=Some(self.normal_step_progress);self.writer.pending_performed=true;self.writer.pending_work=next_work}return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"controlled JSON output allocation failed"))}unsafe{String::from_raw_parts(pointer,0,bytes)}};*actual=Some(bytes);self.writer=ControlledWriter{bytes:0,output:Some(output),metadata:self.writer.metadata,pending:None,pending_work:0,pending_performed:false};self.frames.push(JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0});self.phase=2;self.normal_step_progress=receipt;self.last_work=next_work;if self.pending_action.is_some(){self.writer.pending=Some(receipt);self.writer.pending_performed=true;self.writer.pending_work=next_work}Ok(())});
                if !outcome.admitted||outcome.primary.is_some()||outcome.forwarded_quote.is_some()||outcome.scoped_quote.is_some()||outcome.extent_refusal.is_some()||outcome.forwarded_refusal.is_some()||outcome.scoped_refusal.is_some()||outcome.actual!=Some(bytes){self.allocation_pending=Some(outcome);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer original output allocation refused"))}self.allocation_actual=None;normal_remaining(grant,self.normal_step_progress)?;control.step()?;continue
            }
            let retained_before=self.retained_capacity_bytes()?;
            let copied_before=self.writer.output.as_ref().map_or(0,|output|output.len());let metadata_before=self.writer.metadata;let mut frame_copied=0usize;if !self.frames.is_empty(){self.writer.pending=Some(self.normal_step_progress);self.writer.pending_work=next_work;self.writer.pending_performed=false;}
            let frontier_before=(self.phase,self.frames.len(),self.path.len(),self.writer.bytes);
            let result=(||->Result<(),ValueError>{
                match self.advance(source,control,remaining.maximum_copy_bytes){Ok(copied)=>frame_copied=copied,Err(error)=>{frame_copied=error.retained_progress().copied_bytes;return Err(error)}}
                Ok(())
            })();
            let retained_after=self.retained_capacity_bytes()?;
            let copied_after=self.writer.output.as_ref().map_or(0,|output|output.len());
            if retained_after<retained_before||copied_after<copied_before||self.writer.metadata<metadata_before{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer released an ungranted normal backing"))}
            let changed=result.is_ok()||retained_after!=retained_before||copied_after!=copied_before||self.writer.metadata!=metadata_before||frame_copied!=0||frontier_before!=(self.phase,self.frames.len(),self.path.len(),self.writer.bytes);
            if changed{self.last_work=next_work;}
            self.normal_step_progress=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:usize::from(changed),copied_bytes:(copied_after-copied_before).checked_add(frame_copied).and_then(|value|value.checked_add(self.writer.metadata-metadata_before)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer copied receipt overflow"))?,retained_capacity_bytes:retained_after-retained_before,released_bytes:0})?;
            if self.pending_action.is_some(){self.writer.pending=Some(self.normal_step_progress);self.writer.pending_performed=changed;self.writer.pending_work=next_work}else{self.writer.pending=None;self.writer.pending_performed=false;self.writer.pending_work=0}normal_remaining(grant,self.normal_step_progress)?;
            result?;control.step()?;
        }
        Ok(None)
    }
    fn finish_node(&mut self) { self.frames.pop(); if !self.frames.is_empty() { self.path.pop(); } }
    pub(super) fn child(&mut self, index: usize) -> Result<(), ValueError> {
        if self.path.len() >= MAX_DEPTH as usize { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "retained JSON writer exceeds depth limit")); }
        self.path.push(index); self.frames.push(JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0}); Ok(())
    }
    pub(super) fn advance<S:JsonWriteSource+?Sized>(&mut self, source:&S, control: &mut semio_framework_value::NativeEncodeControl<'_>,copy:usize) -> Result<usize, ValueError> {advance_source(&mut self.frames,&mut self.path,&mut self.writer,source,control,copy)}
}

pub struct JsonWriteCursor<S:JsonWriteSource>{#[doc(hidden)] pub source:Option<S>,#[doc(hidden)] pub state:JsonBorrowedWriteCursor}

impl<S:JsonWriteSource> JsonWriteCursor<S>{
    pub fn new(value:S)->Self{Self{source:Some(value),state:JsonBorrowedWriteCursor::new()}}
    pub fn progress(&self)->(usize,bool){self.state.progress()}
    pub fn take_source(&mut self)->Option<S>{if self.state.phase==3{self.source.take()}else{None}}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.state.normal_step_progress()}
    pub fn normal_step_demands(&self)->Result<RetirementDemand,ValueError>{self.state.normal_step_demands(self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer source is absent"))?)}
    pub fn step(&mut self,maximum_units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
        let source=self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer source is absent"))?;
        self.state.step(source,maximum_units,control,grant)
    }
}

impl semio_framework_value::retirement::RetireOwned for JsonWriteFrame {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::retirement::leaf(self) }
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

impl Drop for JsonBorrowedWriteCursor{fn drop(&mut self){assert!(std::thread::panicking()||semio_framework_value::retirement::RetireOwned::retirement_ready(self),"original JSON cursor receipt remains unresolved")}}
impl semio_framework_value::retirement::RetireOwned for JsonBorrowedWriteCursor {
    fn retirement_ready(&self)->bool{self.writer.pending.is_none()&&self.pending_action.is_none()&&semio_framework_value::retirement::RetireOwned::retirement_ready(&self.allocation_progress)}
    fn retirement(mut self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(std::mem::take(&mut self.frames)),deferred(JsonProjectionIterator::new(std::mem::take(&mut self.path))),deferred(self.writer.output.take()),deferred(self.allocation_progress.take()),deferred(self.allocation_pending.take())])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.frames),deferred_birth_bytes::<JsonProjectionIterator<usize>>(),deferred_birth_bytes_for(&self.writer.output),deferred_birth_bytes_for(&self.allocation_progress),deferred_birth_bytes_for(&self.allocation_pending)])}
    fn controlled_retirement_supported()->bool{true}
}

impl<S:JsonWriteSource+semio_framework_value::retirement::RetireOwned+'static> semio_framework_value::retirement::RetireOwned for JsonWriteCursor<S> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { use semio_framework_value::retirement::*;sequence(vec![deferred(self.source),deferred(self.state.frames),deferred(JsonProjectionIterator::new(self.state.path)),deferred(self.state.writer.output),deferred(self.state.allocation_progress),deferred(self.state.allocation_pending)]) }
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.source),deferred_birth_bytes_for(&self.state.frames),deferred_birth_bytes::<JsonProjectionIterator<usize>>(),deferred_birth_bytes_for(&self.state.writer.output),deferred_birth_bytes_for(&self.state.allocation_progress),deferred_birth_bytes_for(&self.state.allocation_pending)])}
    fn controlled_retirement_supported()->bool{S::controlled_retirement_supported()}
    fn retirement_ready(&self)->bool{self.source.as_ref().is_none_or(semio_framework_value::retirement::RetireOwned::retirement_ready)&&semio_framework_value::retirement::RetireOwned::retirement_ready(&self.state)}
}

fn json_text_block(text:&str,copy:usize)->(usize,usize){let(mut source,mut encoded)=(0,0);for character in text.chars(){let bytes=character.len_utf8();let escaped=match character{'"'|'\\'|'\u{0008}'|'\u{000c}'|'\n'|'\r'|'\t'=>2,c if(c as u32)<0x20=>6,_=>bytes};if source+bytes>64||source!=0&&encoded+escaped>copy{break}source+=bytes;encoded+=escaped;if encoded>copy{break}}(source,encoded)}

struct ControlledWriter { pub bytes: usize, pub output: Option<String>, metadata:usize, pending:Option<RetainedCloneProgress>, pending_work:u64, pending_performed:bool }

impl ControlledWriter {
    fn next_pending(&self,bytes:usize)->Result<Option<RetainedCloneProgress>,ValueError>{let Some(mut progress)=self.pending else{return Ok(None)};if !self.pending_performed{progress.copied_items=progress.copied_items.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer item receipt overflow"))?}progress.copied_bytes=progress.copied_bytes.checked_add(bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer copy receipt overflow"))?;Ok(Some(progress))}
    fn mark_pending(&mut self,progress:Option<RetainedCloneProgress>){if let Some(progress)=progress{self.pending=Some(progress);self.pending_performed=true}}
    pub(super) fn character(&mut self, character: char, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {JsonWriteOutput::character(self,character,control)}
    pub(super) fn raw(&mut self, text: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        let bytes=self.bytes.checked_add(text.len()).filter(|bytes|*bytes<=isize::MAX as usize&&(self.output.is_none()||*bytes<=control.maximum_bytes())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"controlled JSON output exceeds caller limit"))?;
        let metadata=self.metadata.checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer metadata receipt overflow"))?;let pending=self.next_pending(size_of::<usize>())?;self.bytes=bytes;self.metadata=metadata;self.mark_pending(pending);
        if self.output.as_ref().is_some_and(|output|self.bytes>output.capacity()){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"controlled JSON output exceeds admitted allocation"))}
        if text.len()<=65536{if self.output.is_some(){let metadata=self.metadata.checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer metadata receipt overflow"))?;let pending=self.next_pending(size_of::<usize>()+text.len())?;self.output.as_mut().unwrap().push_str(text);self.metadata=metadata;self.mark_pending(pending);}return Ok(())}
        control.scoped_stage(|control|->Result<(),ValueError>{control.begin_stage(text.len())?;let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}if self.output.is_some(){let metadata=self.metadata.checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer metadata receipt overflow"))?;let pending=self.next_pending(size_of::<usize>()+end-position)?;self.output.as_mut().unwrap().push_str(&text[position..end]);self.metadata=metadata;self.mark_pending(pending);}control.advance(end-position)?;position=end;}Ok(())})
    }
    pub(super) fn string(&mut self, text: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        self.raw("\"", control)?;
        control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(text.len())?; for character in text.chars() { self.character(character, control)?; control.advance(character.len_utf8())?; } Ok(()) })?;
        self.raw("\"", control)
    }
    pub(super) fn number(&mut self, number: semio_framework_value::Number, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {JsonWriteOutput::number(self,number,control)}
    pub(super) fn value(&mut self, value: &DslValue, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        control.scoped_depth(MAX_DEPTH as usize + 1, |control| { control.checkpoint()?; match value {
            DslValue::Null => self.raw("null", control), DslValue::Bool(value) => self.raw(if *value { "true" } else { "false" }, control), DslValue::Number(value) => self.number(*value, control), DslValue::String(value) => self.string(value, control),
            DslValue::Array(values) => { self.raw("[", control)?; control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(values.len())?; for (index, value) in values.iter().enumerate() { if index > 0 { self.raw(",", control)?; } self.value(value, control)?; control.step()?; } Ok(()) })?; self.raw("]", control) },
            DslValue::Bytes(values) => { self.raw("[", control)?; control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(values.len())?; for (index, value) in values.iter().enumerate() { if index > 0 { self.raw(",", control)?; } self.number(semio_framework_value::Number::UInt(u64::from(*value)), control)?; control.step()?; } Ok(()) })?; self.raw("]", control) },
            DslValue::Object(entries) => { self.raw("{", control)?; control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(entries.len())?; for (index, (key, value)) in entries.iter().enumerate() { if index > 0 { self.raw(",", control)?; } self.string(key, control)?; self.raw(":", control)?; self.value(value, control)?; control.step()?; } Ok(()) })?; self.raw("}", control) }
        } })
    }
}

struct ControlledReader<'text, 'control, 'progress> { pub(super) lexer: Lexer<'text>, pub policy: JsonMemberPolicy, pub control: &'control mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'progress> }

impl ControlledReader<'_, '_, '_> {
    pub(super) fn error(&self) -> ValueError { JsonError::UnexpectedByte { found: self.lexer.peek_byte().unwrap_or(0), offset: self.lexer.pos }.into_value_error() }
    pub(super) fn advance(&mut self, count: usize) -> Result<(), ValueError> { self.lexer.pos += count; self.control.advance(count) }
    fn whitespace(&mut self) -> Result<(), ValueError> { while matches!(self.lexer.peek_byte(), Some(b' ' | b'\t' | b'\r' | b'\n')) { self.advance(1)?; } Ok(()) }
    fn reserve<T>(&mut self, values: &mut Vec<T>) -> Result<(), ValueError> {
        if values.len() == values.capacity() {
            let count = values.capacity().max(1).checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON collection overflow"))?;
            let bytes = count.checked_mul(size_of::<T>()).filter(|bytes| *bytes <= isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON collection overflow"))?;
            self.control.charge(bytes)?; values.try_reserve_exact(count - values.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON allocation failed"))?;
        } Ok(())
    }
    pub(super) fn string(&mut self) -> Result<String, ValueError> {
        self.advance(1)?; let body = self.lexer.pos; let mut length = 0usize;
        loop { let start = self.lexer.pos; let character = json_character(&mut self.lexer).map_err(JsonError::into_value_error)?; self.control.advance(self.lexer.pos - start)?; let Some(character) = character else { break }; length = length.checked_add(character.len_utf8()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON string overflow"))?; }
        let input = self.lexer.input; let end = self.lexer.pos; self.control.charge(length)?;
        self.control.scoped_stage(|control| {
            control.begin_stage(end - body)?; let mut output = String::new(); output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON text allocation failed"))?;
            let mut decoder = Lexer { input, pos: body };
            loop { let start = decoder.pos; let character = json_character(&mut decoder).map_err(JsonError::into_value_error)?; control.advance(decoder.pos - start)?; let Some(character) = character else { break }; output.push(character); }
            if output.len() != length || decoder.pos != end { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON string changed after admission")); } Ok(output)
        })
    }
    pub(super) fn value(&mut self) -> Result<DslValue, ValueError> {
        self.control.scoped_depth(MAX_DEPTH as usize + 1, |control| { let mut reader = ControlledReader { lexer: Lexer { input: self.lexer.input, pos: self.lexer.pos }, policy: self.policy, control }; let result = reader.value_inner(); self.lexer.pos = reader.lexer.pos; result })
    }
    fn value_inner(&mut self) -> Result<DslValue, ValueError> {
        self.whitespace()?; self.control.checkpoint()?;
        match self.lexer.peek_byte() {
            Some(b'"') => self.string().map(DslValue::String),
            Some(b't' | b'f' | b'n') => { let (text, value) = match self.lexer.peek_byte().unwrap() { b't' => ("true", DslValue::Bool(true)), b'f' => ("false", DslValue::Bool(false)), _ => ("null", DslValue::Null) }; if !self.lexer.input[self.lexer.pos..].starts_with(text) { return Err(self.error()); } self.advance(text.len())?; Ok(value) }
            Some(b'-' | b'0'..=b'9') => {
                let start = self.lexer.pos; while matches!(self.lexer.peek_byte(), Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')) { self.advance(1)?; }
                let mut number = Lexer::new(&self.lexer.input[start..self.lexer.pos]); let Token::Number(value) = number.read_number().map_err(JsonError::into_value_error)? else { unreachable!() }; if number.pos != number.input.len() { return Err(JsonError::InvalidNumber(start).into_value_error()); }
                Ok(DslValue::Number(match value { Number::UInt(value) => semio_framework_value::Number::UInt(value), Number::Int(value) => semio_framework_value::Number::Int(value), Number::Float(value) => semio_framework_value::Number::Float(value) }))
            }
            Some(b'[') => {
                self.advance(1)?; self.whitespace()?; let mut values = Vec::<DslValue>::new().guard_decoded();
                if self.lexer.peek_byte() == Some(b']') { self.advance(1)?; return Ok(DslValue::Array(values.take())); }
                loop { let value = self.value()?.guard_decoded(); self.reserve(values.get_mut())?; values.get_mut().push(value.take()); self.whitespace()?; match self.lexer.peek_byte() { Some(b',') => self.advance(1)?, Some(b']') => { self.advance(1)?; return Ok(DslValue::Array(values.take())); }, _ => return Err(self.error()) } }
            }
            Some(b'{') => {
                self.advance(1)?; self.whitespace()?; let mut entries = semio_framework_value::DecodedValue::new(Vec::new(), |entries: Vec<(String, DslValue)>| { for (_, value) in entries { FromValue::retire_decoded(value); } });
                if self.lexer.peek_byte() == Some(b'}') { self.advance(1)?; return Ok(DslValue::Object(entries.take())); }
                loop {
                    self.whitespace()?; if self.lexer.peek_byte() != Some(b'"') { return Err(self.error()); } let offset = self.lexer.pos; let key = self.string()?;
                    let duplicate = self.control.scoped_stage(|control| -> Result<Option<usize>, ValueError> { control.begin_stage(entries.get().len())?; for (index, (name, _)) in entries.get().iter().enumerate() { let equal = controlled_key_equal(name, &key, control)?; control.step()?; if equal { return Ok(Some(index)); } } Ok(None) })?;
                    if duplicate.is_some() && self.policy == JsonMemberPolicy::Reject { return Err(JsonError::DuplicateMember { name: key, offset }.into_value_error()); }
                    self.whitespace()?; if self.lexer.peek_byte() != Some(b':') { return Err(self.error()); } self.advance(1)?; let value = self.value()?.guard_decoded();
                    if let Some(index) = duplicate { FromValue::retire_decoded(std::mem::replace(&mut entries.get_mut()[index].1, value.take())); } else { self.reserve(entries.get_mut())?; entries.get_mut().push((key, value.take())); }
                    self.whitespace()?; match self.lexer.peek_byte() { Some(b',') => self.advance(1)?, Some(b'}') => { self.advance(1)?; return Ok(DslValue::Object(entries.take())); }, _ => return Err(self.error()) }
                }
            }
            _ => Err(self.error())
        }
    }
}

fn controlled_key_equal(left: &str, right: &str, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<bool, ValueError> {
    if left.len() != right.len() { return Ok(false); } control.scoped_stage(|control| { control.begin_stage(left.len())?; let mut position = 0; while position < left.len() { let end = position.saturating_add(65536).min(left.len()); let equal = left.as_bytes()[position..end] == right.as_bytes()[position..end]; control.advance(end - position)?; if !equal { return Ok(false); } position = end; } Ok(true) })
}

pub(super) fn json_character(lexer:&mut Lexer<'_>)->Result<Option<char>,JsonError>{borrowed_read_source::json_character(lexer.input,&mut lexer.pos)}
//#endregion 🔖️ToFromValueBridge

trait JsonWriteOutput{
 fn frame_copy(&mut self)->Result<(),ValueError>{Ok(())}
 fn raw(&mut self,text:&str,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>;
 fn character(&mut self,character:char,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{
        match character {
            '"' => self.raw("\\\"", control), '\\' => self.raw("\\\\", control), '\u{0008}' => self.raw("\\b", control), '\u{000c}' => self.raw("\\f", control), '\n' => self.raw("\\n", control), '\r' => self.raw("\\r", control), '\t' => self.raw("\\t", control),
            character if (character as u32) < 0x20 => { use fmt::Write as _; let mut scalar = ScalarText::new(); write!(scalar, "\\u{:04x}", character as u32).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON escape overflow"))?; self.raw(scalar.text(), control) }
            character => { let mut bytes = [0; 4]; self.raw(character.encode_utf8(&mut bytes), control) }
        }
    }
 fn float32(&mut self,value:f32,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{control.checkpoint()?;let mut scalar=ScalarText::new();write_float32_to(value,&mut scalar).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"controlled binary32 JSON scalar overflow"))?;self.raw(scalar.text(),control)?;control.checkpoint()}
 fn quoted_unsigned(&mut self,mut value:u128,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{
  let mut bytes=[b'"';41];let mut start=40;
  loop{start-=1;bytes[start]=b'0'+(value%10)as u8;value/=10;if value==0{break}}
  start-=1;bytes[start]=b'"';let text=std::str::from_utf8(&bytes[start..]).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"quoted unsigned scalar is not ASCII"))?;
  self.raw(text,control)
 }
 fn number(&mut self,number:semio_framework_value::Number,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{
        use fmt::Write as _; control.checkpoint()?; let mut scalar = ScalarText::new();
        match number { semio_framework_value::Number::UInt(value) => write!(scalar, "{value}"), semio_framework_value::Number::Int(value) => write!(scalar, "{value}"), semio_framework_value::Number::Float(value) => write_float_to(value, &mut scalar) }.map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON scalar overflow"))?;
        self.raw(scalar.text(), control)?; control.checkpoint()
    }
}
impl JsonWriteOutput for ControlledWriter{fn frame_copy(&mut self)->Result<(),ValueError>{let pending=self.next_pending(size_of::<JsonWriteFrame>())?;self.mark_pending(pending);Ok(())}fn raw(&mut self,text:&str,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{ControlledWriter::raw(self,text,control)}}

fn finish_source_node(frames:&mut Vec<JsonWriteFrame>,path:&mut Vec<usize>)->usize{frames.pop();let mut bytes=size_of::<JsonWriteFrame>()+size_of::<usize>();if !frames.is_empty(){path.pop();bytes+=2*size_of::<usize>();}bytes}
fn child_source_node(frames:&mut Vec<JsonWriteFrame>,path:&mut Vec<usize>,index:usize)->Result<usize,ValueError>{if path.len()>=MAX_DEPTH as usize||path.len()==path.capacity()||frames.len()==frames.capacity(){return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"JSON source exceeds its admitted traversal frontier"))}path.push(index);frames.push(JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0});Ok(2*size_of::<JsonWriteFrame>()+3*size_of::<usize>())}
fn advance_source<S:JsonWriteSource+?Sized,W:JsonWriteOutput>(frames:&mut Vec<JsonWriteFrame>,path:&mut Vec<usize>,writer:&mut W,source:&S,control:&mut semio_framework_value::NativeEncodeControl<'_>,copy:usize)->Result<usize,ValueError>{let mut copied=size_of::<JsonWriteFrame>();let result=(||->Result<(),ValueError>{
        let frame = *frames.last().expect("writer frontier is inhabited");
        writer.frame_copy()?;
        let node = source.node_at_path(path.as_slice())?;
        match frame.state {
            0 => {
                match node {
                    JsonWriteNode::Null => writer.raw("null", control)?,
                    JsonWriteNode::Bool(value) => writer.raw(if value { "true" } else { "false" }, control)?,
                    JsonWriteNode::Number(value) => writer.number(value, control)?,
                    JsonWriteNode::Float32(value) => writer.float32(value,control)?,
                    JsonWriteNode::QuotedUnsigned(value) => writer.quoted_unsigned(value, control)?,
                    JsonWriteNode::String(_) => { writer.raw("\"", control)?; frames.last_mut().unwrap().state = 5;copied+=size_of::<u8>(); return Ok(()); }
                    JsonWriteNode::NativeString(_) => {writer.raw("\"",control)?;frames.last_mut().unwrap().state=6;copied+=size_of::<u8>();return Ok(());}
                    JsonWriteNode::Array(_) => { writer.raw("[", control)?; frames.last_mut().unwrap().state = 1;copied+=size_of::<u8>(); return Ok(()); }
                    JsonWriteNode::Object(_) => { writer.raw("{", control)?; frames.last_mut().unwrap().state = 2;copied+=size_of::<u8>(); return Ok(()); }
                }
                copied+=finish_source_node(frames,path);
            }
            1 => {
                let JsonWriteNode::Array(length)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON array source changed"))};
                if frame.index == length { writer.raw("]", control)?; copied+=finish_source_node(frames,path); }
                else { let edge=source.traversal_edge_at_path(path.as_slice(),frame.index,frame.next_cursor)?;if frame.index != 0 { writer.raw(",", control)?; }let next=frames.last_mut().unwrap();next.index+=1;next.next_cursor=edge.cursor;copied+=2*size_of::<usize>(); copied+=child_source_node(frames,path,edge.child)?; }
            }
            2 => {
                let JsonWriteNode::Object(length) = node else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON object source changed")); };
                if frame.index == length { writer.raw("}", control)?; copied+=finish_source_node(frames,path); }
                else { let edge=source.traversal_edge_at_path(path.as_slice(),frame.index,frame.next_cursor)?;if frame.index != 0 { writer.raw(",", control)?; } writer.raw("\"", control)?; let next = frames.last_mut().unwrap(); next.state = 3; next.position = 0;next.source_index=edge.child;next.next_cursor=edge.cursor;copied+=size_of::<u8>()+3*size_of::<usize>(); }
            }
            3 | 5 => {
                let text = match node { JsonWriteNode::String(text) if frame.state == 5 => text, JsonWriteNode::Object(_) if frame.state == 3 => source.object_key_at_path(path.as_slice(),frame.source_index)?, _ => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON text source changed")) };
                let remaining=text.get(frame.position..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON text source position changed"))?;
                if !remaining.is_empty() { let (length,_)=json_text_block(remaining,copy);for character in remaining[..length].chars(){writer.character(character,control)?;frames.last_mut().unwrap().position+=character.len_utf8();}copied+=size_of::<usize>(); }
                else { writer.raw("\"", control)?; if frame.state == 5 { copied+=finish_source_node(frames,path); } else { writer.raw(":", control)?; frames.last_mut().unwrap().state = 4;copied+=size_of::<u8>(); } }
            }
            4 => { let next = frames.last_mut().unwrap(); next.state = 2; next.index += 1;copied+=size_of::<u8>()+size_of::<usize>(); copied+=child_source_node(frames,path,frame.source_index)?; }
            6 => {
                let JsonWriteNode::NativeString(text)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text source changed"));};
                if frame.index==text.text_chunk_count(){writer.raw("\"",control)?;copied+=finish_source_node(frames,path);}
                else {
                    let body=text.text_chunk(frame.index).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text chunk absent"))?;
                    let remaining=body.get(frame.position..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text offset changed"))?;
                    if !remaining.is_empty(){let(length,_)=json_text_block(remaining,copy);for character in remaining[..length].chars(){writer.character(character,control)?;frames.last_mut().unwrap().position+=character.len_utf8();}copied+=size_of::<usize>();}
                    else {let next=frames.last_mut().unwrap();next.index+=1;next.position=0;copied+=2*size_of::<usize>();}
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    })();result.map(|_|copied).map_err(|error|error.with_retained_progress(RetainedCloneProgress{copied_bytes:copied,..Default::default()}))}

#[path="🫗️sink/🦀️.rs"]
mod borrowed_sink;
pub use borrowed_sink::{JsonBorrowedSinkCursor,JsonBorrowedSinkStep};
