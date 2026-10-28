//! 🧮️ Parser and grammar cursor: whole-document parsing, the retained grammar cursor with its frames and the value projection with its retirement owners. See [`super`].

use super::*;

//#region 🔖️Parser
/// 🌳️ Parses one whole JSON document from `input`, rejecting trailing non-whitespace bytes.
// 🚫️async: R9 pure in-memory parse, no I/O.
pub fn parse(input: &str, policy: JsonMemberPolicy) -> Result<Value, JsonError> {
    let mut cursor = JsonParseCursor::new(policy);
    let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(usize::MAX,&mut accepted);
    loop { if let Some(value) = cursor.step(input, 4096,&mut control,RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:isize::MAX as usize,maximum_capacity_bytes:isize::MAX as usize,maximum_release_bytes:isize::MAX as usize,maximum_depth:MAX_DEPTH as usize+4})? { return Ok(value); } }
}

/// 📖️ [`parse`] over raw bytes — errors with [`JsonError::InvalidUtf8`] if `input` is not UTF-8.
pub fn parse_bytes(input: &[u8], policy: JsonMemberPolicy) -> Result<Value, JsonError> {
    let text = std::str::from_utf8(input).map_err(|_| JsonError::InvalidUtf8)?;
    parse(text, policy)
}

pub(super) fn normal_permit(demand:RetirementDemand,grant:RetainedCloneGrant)->Result<bool,ValueError>{
    if grant.maximum_items==0{return Ok(false);}
    if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"JSON normal frontier exceeds original admitted depth"));}
    Ok(demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes)
}

pub(super) fn normal_remaining(grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<RetainedCloneGrant,ValueError>{
    if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON original normal receipt exceeded its caller grant"));}
    Ok(RetainedCloneGrant{maximum_items:grant.maximum_items-progress.copied_items,maximum_copy_bytes:grant.maximum_copy_bytes-progress.copied_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes-progress.retained_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes-progress.released_bytes,..grant})
}

type JsonCandidates<T> = semio_framework_value_persistent::list::PagedList<T,{usize::MAX}>;

#[doc(hidden)]
pub struct JsonFrame<V:JsonParsedValue> {
    pub object: bool, pub state: u8, item_count:u64, pub values: JsonCandidates<V>, pub entries: JsonCandidates<(String,V)>,
    pub key: Option<String>, key_offset: usize, probe: usize, pub compare: usize, pub(super) duplicate: Option<usize>,
    pub array:Vec<V>, pub(super) members:Vec<(String,V)>, pub(super) admitted:bool, pub(super) reverse:usize,
}

#[doc(hidden)]
pub struct JsonStringScan {pub start:usize,pub position:usize,pub bytes:usize,pub output:Vec<u8>,pending_scalar:u32,pending_width:u8,pending_index:u8,writing:bool,pub(super) admitted:bool}

impl JsonStringScan{
    pub(super) fn next_byte(&self)->u8{let scalar=self.pending_scalar;let width=self.pending_width;let index=self.pending_index;if width==1{return scalar as u8}if index==0{return match width{2=>0xc0|(scalar>>6)as u8,3=>0xe0|(scalar>>12)as u8,_=>0xf0|(scalar>>18)as u8}}0x80|((scalar>>(6*u32::from(width-index-1)))&63)as u8}
    fn write_byte(&mut self,receipt:&mut RetainedCloneProgress)->Result<(),JsonError>{if self.output.len()>=self.bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON string changed after admission").into())}self.output.push(self.next_byte());self.pending_index+=1;receipt.copied_bytes=1;Ok(())}
}

#[doc(hidden)]
pub enum JsonLexeme { String(JsonStringScan), Number(NumberScan) }

fn json_candidate_slot<T>(owner:&mut JsonCandidates<T>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,JsonError> {
    if owner.has_reserved_slot() {return Ok(true);}
    let bytes=owner.next_allocation_bytes().map_err(ValueError::from)?;control.charge(bytes)?;
    match owner.reserve_one(bytes){Ok(progress)=>receipt.retained_capacity_bytes=progress.allocated_bytes,Err(error)=>{receipt.retained_capacity_bytes=error.allocated_bytes;return Err(ValueError::from(error.refusal()).into());}}Ok(false)
}

fn json_candidate_demand<T>(owner:&JsonCandidates<T>,demand:&mut RetirementDemand)->Result<(),JsonError>{
    if owner.has_reserved_slot(){demand.depth=demand.depth.max(owner.next_push_depth_demand().map_err(ValueError::from)?);}
    else{demand.capacity_bytes=owner.next_allocation_bytes().map_err(ValueError::from)?;demand.depth=demand.depth.max(owner.next_reserve_depth_demand().map_err(ValueError::from)?);}
    Ok(())
}

/// 🧵️ Retains the canonical JSON grammar and admitted candidates while borrowing unchanged source.
pub type JsonParseCursor=JsonGrammarCursor<Value>;

/// 🖱️ One retained grammar moves admitted semantic cells directly into its declared first-party output.
pub struct JsonGrammarCursor<V:JsonParsedValue> {
    #[doc(hidden)] pub position: usize, validated_position:usize, #[doc(hidden)] pub limits:JsonReadLimits, #[doc(hidden)] pub policy: JsonMemberPolicy, #[doc(hidden)] pub frames: JsonCandidates<JsonFrame<V>>, #[doc(hidden)] pub lexeme: Option<JsonLexeme>,
    #[doc(hidden)] pub pending: Option<V>, #[doc(hidden)] pub result: Option<V>, retired: JsonCandidates<V>, obsolete:Option<V>, pub(super) complete: bool, pub(super) normal_step_progress:RetainedCloneProgress,
}

impl<V:JsonParsedValue> JsonGrammarCursor<V> {
    /// 🌱️ Starts parsing without copying, scanning, or allocating for the source.
    pub fn new(policy: JsonMemberPolicy) -> Self { Self { position:0, validated_position:0, limits:JsonReadLimits{maximum_bytes:u64::MAX,maximum_allocation_bytes:usize::MAX,maximum_depth:MAX_DEPTH as usize,maximum_items:u64::MAX}, policy, frames:Default::default(), lexeme:None, pending:None, result:None, retired:Default::default(),obsolete:None,complete:false,normal_step_progress:Default::default() } }
    /// 📍️ Returns the measured source byte offset.
    pub fn position(&self) -> usize { self.position }
    /// 🧭️ Identifies the existing grammar or physical candidate frontier.
    pub fn phase(&self)->&'static str {match &self.lexeme {Some(JsonLexeme::String(scan))=>if scan.writing {"materialize-string"}else {"measure-string"},Some(JsonLexeme::Number(_))=>"number",None=>match self.frames.last(){Some(frame) if frame.state==8=>if frame.object {"materialize-object"}else {"materialize-array"},Some(frame)=>if frame.object {"collect-object"}else {"collect-array"},None=>"grammar"}}}
    /// ⏱️ Advances admitted grammar transitions under this operation's cumulative decode authority.
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_step_progress}
    /// 📏️ Quotes the actual next original semantic or physical frontier before any effect.
    pub fn normal_step_demands<S:JsonReadSource+?Sized>(&self,input:&S)->Result<RetirementDemand,JsonError>{
        if self.validated_position<input.byte_len(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        let mut demand=RetirementDemand{depth:self.frames.len()+1,..Default::default()};
        if self.obsolete.is_some(){json_candidate_demand(&self.retired,&mut demand)?;return Ok(demand);}
        if let Some(frame)=self.frames.last().filter(|frame|frame.state==8){
            if !frame.admitted{demand.capacity_bytes=if frame.object{frame.entries.len()*size_of::<(String,V)>()}else{frame.values.len()*size_of::<V>()};}
            else if frame.object&&!frame.entries.is_empty(){demand.depth=demand.depth.max(frame.entries.next_pop_depth_demand().map_err(ValueError::from)?);}
            else if !frame.object&&!frame.values.is_empty(){demand.depth=demand.depth.max(frame.values.next_pop_depth_demand().map_err(ValueError::from)?);}
            else if frame.entries.is_empty()&&frame.values.is_empty(){
                if !frame.entries.terminal_is_empty(){demand.release_bytes=frame.entries.next_release_allocation_bytes().map_err(ValueError::from)?;demand.depth=demand.depth.max(frame.entries.next_release_depth_demand().map_err(ValueError::from)?);}
                else if !frame.values.terminal_is_empty(){demand.release_bytes=frame.values.next_release_allocation_bytes().map_err(ValueError::from)?;demand.depth=demand.depth.max(frame.values.next_release_depth_demand().map_err(ValueError::from)?);}
            }
            return Ok(demand);
        }
        if self.pending.is_some(){if let Some(frame)=self.frames.last(){if let Some(index)=frame.duplicate{json_candidate_demand(&self.retired,&mut demand)?;if self.retired.has_reserved_slot(){demand.depth=demand.depth.max(frame.entries.next_get_depth_demand(index).map_err(ValueError::from)?);}}else if frame.object{json_candidate_demand(&frame.entries,&mut demand)?;}else{json_candidate_demand(&frame.values,&mut demand)?;}}return Ok(demand);}
        if self.lexeme.is_none()&&self.result.is_none()&&self.frames.last().map_or(true,|frame|if frame.object{frame.state==4}else{matches!(frame.state,0|2)})&&matches!(input.byte_at(self.position),Some(b'{'|b'[')){self.admit_container_depth()?;}
        if let Some(frame)=self.frames.last().filter(|frame|frame.state==6){demand.depth=demand.depth.max(frame.entries.next_get_depth_demand(frame.probe).map_err(ValueError::from)?);}
        if let Some(JsonLexeme::String(scan))=&self.lexeme{
            if scan.writing&&!scan.admitted{demand.capacity_bytes=scan.bytes;}
            else if scan.writing{let mut position=scan.position;demand.copy_bytes=if scan.pending_index<scan.pending_width{1}else{usize::from(borrowed_read_source::json_character(input,&mut position)?.is_some())};}
        }else if self.lexeme.is_none()&&matches!(input.byte_at(self.position),Some(b'{'|b'[')){json_candidate_demand(&self.frames,&mut demand)?;}
        Ok(demand)
    }
    pub fn step(&mut self,input:&str,maximum_units:usize,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<V>,JsonError>{
        self.step_source(input,maximum_units,control,grant,true)
    }
    pub fn step_source<S:JsonReadSource+?Sized>(&mut self,input:&S,maximum_units:usize,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant,known_utf8:bool)->Result<Option<V>,JsonError>{
        self.normal_step_progress=Default::default();
        let result=(||{
        for _ in 0..maximum_units{
            let remaining=normal_remaining(grant,self.normal_step_progress)?;let demand=self.normal_step_demands(input)?;
            if !normal_permit(demand,remaining)?{break;}
            control.checkpoint()?;
            if self.complete{return Ok(self.result.take());}
            control.admit_turn_capacity(remaining.maximum_capacity_bytes)?;
            let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
            let operation_result=(||{
                if self.validated_position<input.byte_len(){if known_utf8{self.validated_position=input.byte_len();}else{let character=borrowed_read_source::character(input,self.validated_position)?;self.validated_position+=character.len_utf8();}Ok(())}
                else{control.scoped_maximum(self.limits.maximum_allocation_bytes,|control|self.advance(input,control,remaining,&mut receipt))}
            })();
            self.normal_step_progress=self.normal_step_progress.checked_add(receipt)?;
            operation_result?;
            normal_remaining(grant,self.normal_step_progress)?;
            control.step()?;
            if self.complete{return Ok(self.result.take());}
        }
        Ok(None)
        })();
        match result{
            Err(JsonError::Native(error))=>{
                Err(JsonError::Native(error.with_retained_progress(self.normal_step_progress)))
            },
            result=>result,
        }
    }
    pub(super) fn advance<S:JsonReadSource+?Sized>(&mut self,input:&S,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<(),JsonError> {
        if self.obsolete.is_some() {
            if !json_candidate_slot(&mut self.retired,control,receipt)? {return Ok(());}
            self.retired.push_reserved(self.obsolete.take().unwrap()).unwrap_or_else(|_|unreachable!());return Ok(());
        }
        if self.frames.last().is_some_and(|frame|frame.state==8) {return self.materialize_frame(control,grant,receipt);}
        if self.pending.is_some() {
            if let Some(frame)=self.frames.last_mut() {
                if frame.object {
                    if let Some(index)=frame.duplicate {
                        if !json_candidate_slot(&mut self.retired,control,receipt)? {return Ok(());}
                        let old=std::mem::replace(&mut frame.entries.get_mut(index).unwrap().1,self.pending.take().unwrap());self.retired.push_reserved(old).unwrap_or_else(|_|unreachable!());self.obsolete=frame.key.take().map(V::json_string);frame.duplicate=None;
                    } else {
                        if !json_candidate_slot(&mut frame.entries,control,receipt)? {return Ok(());}
                        frame.entries.push_reserved((frame.key.take().unwrap(),self.pending.take().unwrap())).unwrap_or_else(|_|unreachable!());
                    }
                    frame.state=5;
                } else {
                    if !json_candidate_slot(&mut frame.values,control,receipt)? {return Ok(());}
                    frame.values.push_reserved(self.pending.take().unwrap()).unwrap_or_else(|_|unreachable!());frame.state=1;
                }
            } else {self.result=self.pending.take();}
            return Ok(());
        }
        if let Some(lexeme)=&mut self.lexeme {
            match lexeme {
                JsonLexeme::String(scan)=>{
                    if scan.writing && !scan.admitted {control.charge(scan.bytes)?;scan.output.try_reserve_exact(scan.bytes).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"JSON string allocation failed"))?;receipt.retained_capacity_bytes=scan.output.capacity();scan.admitted=true;return Ok(());}
                    if scan.writing&&scan.pending_index<scan.pending_width{scan.write_byte(receipt)?;return Ok(());}
                    let mut position=if scan.writing{scan.position}else{self.position};
                    let character=borrowed_read_source::json_character(input,&mut position)?;
                    if scan.writing{scan.position=position;}else{self.position=position;}
                    if let Some(character)=character {
                        if scan.writing {scan.pending_scalar=character as u32;scan.pending_width=character.len_utf8()as u8;scan.pending_index=0;scan.write_byte(receipt)?;}else {scan.bytes=scan.bytes.checked_add(character.len_utf8()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"JSON string size overflow"))?;}
                        return Ok(());
                    }
                    if !scan.writing {scan.writing=true;scan.position=scan.start;return Ok(());}
                    if scan.output.len()!=scan.bytes {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON string changed after measurement").into());}
                    let Some(JsonLexeme::String(scan))=self.lexeme.take() else {unreachable!()};let text=unsafe{String::from_utf8_unchecked(scan.output)};
                    if let Some(frame)=self.frames.last_mut().filter(|frame|frame.object && matches!(frame.state,0|2)) {frame.key=Some(text);frame.state=6;frame.probe=0;frame.compare=0;frame.duplicate=None;}
                    else {self.pending=Some(V::json_string(text));}
                },
                JsonLexeme::Number(number)=>{
                    if let Some(value)=number.step(input)? {self.position=number.position;self.lexeme=None;self.pending=Some(V::json_number(value));}
                    else {self.position=number.position;}
                },
            }
            return Ok(());
        }
        if let Some(frame)=self.frames.last_mut().filter(|frame|frame.state==6) {
            if frame.probe==frame.entries.len() {frame.state=3;return Ok(());}
            let name=&frame.entries.get(frame.probe).unwrap().0;let key=frame.key.as_ref().unwrap();
            if name.len()!=key.len() || name.as_bytes().get(frame.compare)!=key.as_bytes().get(frame.compare) {frame.probe+=1;frame.compare=0;return Ok(());}
            frame.compare+=1;
            if frame.compare>key.len() {
                if self.policy==JsonMemberPolicy::Reject {return Err(JsonError::DuplicateMember {name:frame.key.take().unwrap(),offset:frame.key_offset});}
                frame.duplicate=Some(frame.probe);frame.state=3;
            }
            return Ok(());
        }
        let byte=input.byte_at(self.position);
        if matches!(byte,Some(b' '|b'\t'|b'\n'|b'\r')) {self.position+=1;return Ok(());}
        if self.result.is_some() {if byte.is_some() {return Err(JsonError::TrailingData(self.position));}self.complete=true;return Ok(());}
        let error=||JsonError::UnexpectedByte {found:byte.unwrap_or(0),offset:self.position};
        let Some(byte)=byte else {return Err(JsonError::UnexpectedEof)};
        if let Some(frame)=self.frames.last_mut() {
            if frame.object {
                match frame.state {
                    0|2=>{if byte==b'}' && frame.state==0 {self.close_frame();return Ok(());}if byte!=b'"' {return Err(error());}frame.key_offset=self.position;},
                    3=>{if byte!=b':' {return Err(error());}frame.state=4;self.position+=1;return Ok(());},
                    5=>{if byte==b'}' {self.close_frame();return Ok(());}if byte!=b',' {return Err(error());}frame.state=2;self.position+=1;return Ok(());},
                    _=>{},
                }
            } else {
                if frame.state==0 && byte==b']' {self.close_frame();return Ok(());}
                if frame.state==1 {if byte==b']' {self.close_frame();return Ok(());}if byte!=b',' {return Err(error());}frame.state=2;self.position+=1;return Ok(());}
            }
        }
        let declared_item=self.frames.len().checked_sub(1).filter(|index|matches!(self.frames[*index].state,0|2));
        if let Some(index)=declared_item{if self.frames[index].item_count>=self.limits.maximum_items{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"JSON declared collection extent exceeds caller limit").into());}}
        match byte {
            b'"'=>{self.position+=1;self.lexeme=Some(JsonLexeme::String(JsonStringScan {start:self.position,position:self.position,bytes:0,output:Vec::new(),pending_scalar:0,pending_width:0,pending_index:0,writing:false,admitted:false}));},
            b'-'|b'0'..=b'9'=>self.lexeme=Some(JsonLexeme::Number(NumberScan::new(self.position))),
            b'{'|b'['=>{
                self.admit_container_depth()?;
                if !json_candidate_slot(&mut self.frames,control,receipt)?{return Ok(());}
                self.frames.push_reserved(JsonFrame {object:byte==b'{',state:0,item_count:0,values:Default::default(),entries:Default::default(),key:None,key_offset:0,probe:0,compare:0,duplicate:None,array:Vec::new(),members:Vec::new(),admitted:false,reverse:0}).unwrap_or_else(|_|unreachable!("original JSON grammar reserved its exact next frame"));self.position+=1;
            },
            b't'|b'f'|b'n'=>{let (text,value)=match byte {b't'=>("true",V::json_bool(true)),b'f'=>("false",V::json_bool(false)),_=>("null",V::json_null())};if !borrowed_read_source::starts_with(input,self.position,text) {return Err(error());}self.position+=text.len();self.pending=Some(value);},
            _=>return Err(error()),
        }
        if let Some(index)=declared_item{self.frames[index].item_count+=1;}
        Ok(())
    }
    fn admit_container_depth(&self)->Result<(),JsonError>{let maximum=self.limits.maximum_depth.min(MAX_DEPTH as usize);if self.frames.len()>=maximum{Err(JsonError::MaxDepthExceeded(maximum as u32))}else{Ok(())}}
    fn close_frame(&mut self) {self.frames.last_mut().unwrap().state=8;self.position+=1;}
    fn materialize_frame(&mut self,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<(),JsonError> {
        let frame=self.frames.last_mut().unwrap();
        if !frame.admitted {if frame.object {frame.members=control.allocate_vec(frame.entries.len())?;receipt.retained_capacity_bytes=frame.members.capacity()*size_of::<(String,V)>();}else {frame.array=control.allocate_vec(frame.values.len())?;receipt.retained_capacity_bytes=frame.array.capacity()*size_of::<V>();}frame.admitted=true;return Ok(());}
        if frame.object {if let Some(entry)=frame.entries.pop(){frame.members.push(entry);return Ok(());}}else if let Some(value)=frame.values.pop(){frame.array.push(value);return Ok(());}
        if !frame.entries.terminal_is_empty() {let progress=frame.entries.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;receipt.released_bytes=progress.released_allocation_bytes;return Ok(());}
        if !frame.values.terminal_is_empty() {let progress=frame.values.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;receipt.released_bytes=progress.released_allocation_bytes;return Ok(());}
        let length=if frame.object {frame.members.len()}else {frame.array.len()};
        if frame.reverse<length/2 {let opposite=length-1-frame.reverse;if frame.object {frame.members.swap(frame.reverse,opposite);}else {frame.array.swap(frame.reverse,opposite);}frame.reverse+=1;return Ok(());}
        let frame=self.frames.pop().unwrap();self.pending=Some(if frame.object {V::json_object(frame.members)}else {V::json_array(frame.array)});Ok(())
    }
}

impl semio_framework_value::retirement::RetireOwned for Number {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

impl semio_framework_value::retirement::RetireOwned for Object {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.0)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)}
    fn controlled_retirement_supported()->bool{true}
}

impl semio_framework_value::retirement::RetireOwned for Value {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;match self {Self::String(value)=>value.retirement(),Self::Array(value)=>value.retirement(),Self::Object(value)=>value.retirement(),Self::Number(value)=>leaf(value),Self::Bool(value)=>leaf(value),Self::Null=>leaf(())}}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;match self {Self::String(value)=>value.retirement_birth_bytes(),Self::Array(value)=>value.retirement_birth_bytes(),Self::Object(value)=>value.retirement_birth_bytes(),Self::Number(value)=>value.retirement_birth_bytes(),Self::Bool(value)=>value.retirement_birth_bytes(),Self::Null=>().retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool{true}
}

impl<V:JsonParsedValue> semio_framework_value::retirement::RetireOwned for JsonFrame<V> {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(self.values),deferred(self.entries),deferred(self.key),deferred(self.array),deferred(self.members)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.values),deferred_birth_bytes_for(&self.entries),deferred_birth_bytes_for(&self.key),deferred_birth_bytes_for(&self.array),deferred_birth_bytes_for(&self.members)])}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}

impl semio_framework_value::retirement::RetireOwned for JsonLexeme {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;match self {Self::String(value)=>bytes(value.output),Self::Number(value)=>leaf(value)}}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;match self {Self::String(_)=>Some(bytes_birth_bytes()),Self::Number(_)=>Some(leaf_birth_bytes::<NumberScan>())}}
    fn controlled_retirement_supported()->bool{true}
}

impl<V:JsonParsedValue> semio_framework_value::retirement::RetireOwned for JsonGrammarCursor<V> {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(self.frames),deferred(self.lexeme),deferred(self.pending),deferred(self.result),deferred(self.retired),deferred(self.obsolete)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.frames),deferred_birth_bytes_for(&self.lexeme),deferred_birth_bytes_for(&self.pending),deferred_birth_bytes_for(&self.result),deferred_birth_bytes_for(&self.retired),deferred_birth_bytes_for(&self.obsolete)])}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}

struct JsonMemberOrder {pub phase:u8,build:usize,pub(super) end:usize,pub(super) root:usize,pub(super) child:usize,pub(super) offset:usize,continuation:u8}

impl JsonMemberOrder {
    pub(super) fn new(length:usize)->Self {Self {phase:0,build:length/2,end:length,root:0,child:0,offset:0,continuation:1}}
    fn compare(&mut self,entries:&[(String,DslValue)],left:usize,right:usize)->Option<std::cmp::Ordering> {let left=entries[left].0.as_bytes().get(self.offset);let right=entries[right].0.as_bytes().get(self.offset);if left!=right || left.is_none() {self.offset=0;Some(left.cmp(&right))}else {self.offset+=1;None}}
    pub(super) fn step(&mut self,entries:&mut [(String,DslValue)])->bool {
        match self.phase {
            0=>{if self.end<2 {return true;}self.phase=1;},
            1=>{if self.build>0 {self.build-=1;self.root=self.build;self.continuation=1;self.phase=2;}else {self.phase=5;}},
            2=>{self.child=self.root.saturating_mul(2).saturating_add(1);if self.child>=self.end {self.phase=self.continuation;}else {self.offset=0;self.phase=if self.child+1<self.end {3}else {4};}},
            3=>{if let Some(order)=self.compare(entries,self.child+1,self.child) {if order==std::cmp::Ordering::Greater {self.child+=1;}self.phase=4;}},
            4=>{if let Some(order)=self.compare(entries,self.child,self.root) {if order==std::cmp::Ordering::Greater {entries.swap(self.child,self.root);self.root=self.child;self.phase=2;}else {self.phase=self.continuation;}}},
            _=>{if self.end<=1 {return true;}self.end-=1;entries.swap(0,self.end);self.root=0;self.continuation=5;self.phase=2;},
        }
        false
    }
}

#[doc(hidden)]
pub struct JsonProjectionIterator<T> {pub values:std::vec::IntoIter<T>,pub allocation:usize}

impl<T> JsonProjectionIterator<T> {pub(super) fn new(values:Vec<T>)->Self {let allocation=values.capacity().saturating_mul(size_of::<T>());Self {values:values.into_iter(),allocation}}pub(super) fn next(&mut self)->Option<T>{self.values.next()}pub(super) fn len(&self)->usize{self.values.len()}}

struct JsonProjectionIteratorRetirement<T:semio_framework_value::retirement::RetireOwned> {pub values:std::mem::ManuallyDrop<std::vec::IntoIter<T>>,pub remaining:usize,pub released:bool}

impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for JsonProjectionIteratorRetirement<T> {
        fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let capacity_bytes=if self.released||self.values.len()==0{0}else{self.values.as_slice().last().unwrap().retirement_birth_bytes().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"JSON iterator child has no admitted constructor"))?};Ok(semio_framework_value::RetirementDemand{capacity_bytes,release_bytes:if !self.released&&self.values.len()==0{self.remaining}else{0},depth:usize::from(!self.released),..Default::default()})}

fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}if self.released{return RetirementStep::Complete;}
        if let Some(value)=self.values.next_back(){return RetirementStep::Child(value.retirement());}
        if self.remaining>grant.maximum_release_bytes{return RetirementStep::BudgetExhausted;}
        let bytes=std::mem::take(&mut self.remaining);unsafe{std::mem::ManuallyDrop::drop(&mut self.values)};self.released=true;if bytes==0{RetirementStep::Complete}else{RetirementStep::Bytes(bytes)}
    }
    fn terminal_is_empty(&self)->bool{self.released}
    fn next_close_byte_demand(&self)->Option<usize>{Some(if !self.released&&self.values.len()==0{self.remaining}else{0})}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{if self.released||self.values.len()==0{Some(0)}else{self.values.as_slice().last()?.retirement_birth_bytes()}}
    fn terminal_release_bytes(&self)->Option<usize>{self.released.then_some(size_of::<Self>())}
}

impl<T:semio_framework_value::retirement::RetireOwned> Drop for JsonProjectionIteratorRetirement<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.released,"JSON projection backing retired before terminal-empty");}}

impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for JsonProjectionIterator<T> {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(JsonProjectionIteratorRetirement {values:std::mem::ManuallyDrop::new(self.values),remaining:self.allocation,released:false})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<JsonProjectionIteratorRetirement<T>>())}
    fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}

#[doc(hidden)]
pub struct JsonProjectionFrame {pub values:JsonProjectionIterator<Value>,pub entries:JsonProjectionIterator<(String,Value)>,pub array:Vec<DslValue>,pub object:Vec<(String,DslValue)>,pub key:Option<String>,is_object:bool,order:Option<JsonMemberOrder>,pub(super) admitted:bool}

#[doc(hidden)]
pub struct JsonCompletedFrameRetirement {pub frame:std::mem::ManuallyDrop<JsonProjectionFrame>,pub phase:u8}

impl JsonCompletedFrameRetirement {
    pub(super) fn new(frame:JsonProjectionFrame)->Result<Self,(ValueError,JsonProjectionFrame)>{if frame.values.len()!=0||frame.entries.len()!=0||!frame.array.is_empty()||!frame.object.is_empty()||frame.key.is_some(){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"completed JSON input frame still owns an untransferred value"),frame));}Ok(Self{frame:std::mem::ManuallyDrop::new(frame),phase:0})}
    #[doc(hidden)] pub fn release_demand(&self)->usize{match self.phase{0=>self.frame.values.allocation,1=>self.frame.entries.allocation,2=>self.frame.array.capacity()*size_of::<DslValue>(),3=>self.frame.object.capacity()*size_of::<(String,DslValue)>(),_=>0}}
}

impl semio_framework_value::retirement::RetirementCursor for JsonCompletedFrameRetirement {
        fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{release_bytes:self.release_demand(),depth:usize::from(!self.terminal_is_empty()),..Default::default()})}

fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}if self.phase==4{return RetirementStep::Complete;}let bytes=self.release_demand();if bytes>grant.maximum_release_bytes{return RetirementStep::BudgetExhausted;}match self.phase{0=>drop(std::mem::replace(&mut self.frame.values,JsonProjectionIterator::new(Vec::new()))),1=>drop(std::mem::replace(&mut self.frame.entries,JsonProjectionIterator::new(Vec::new()))),2=>drop(std::mem::take(&mut self.frame.array)),3=>drop(std::mem::take(&mut self.frame.object)),_=>unreachable!()}self.phase+=1;if bytes==0{RetirementStep::Advanced}else{RetirementStep::Bytes(bytes)}}
    fn terminal_is_empty(&self)->bool{self.phase==4}
    fn next_close_byte_demand(&self)->Option<usize>{Some(self.release_demand())}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl Drop for JsonCompletedFrameRetirement {fn drop(&mut self){if self.phase!=4{assert!(std::thread::panicking(),"completed JSON input frame must release its original backing before drop");return;}unsafe{std::mem::ManuallyDrop::drop(&mut self.frame);}}}

impl semio_framework_value::retirement::RetireOwned for JsonCompletedFrameRetirement {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

/// 🎒️ Moves a parsed JSON candidate into admitted canonical values without payload clones.
pub struct JsonValueProjection {#[doc(hidden)] pub pending:Option<Value>,#[doc(hidden)] pub output:Option<DslValue>,#[doc(hidden)] pub frames:Vec<JsonProjectionFrame>,pub(super) ordered:bool,#[doc(hidden)] pub retirement:Option<JsonCompletedFrameRetirement>,pub(super) normal_step_progress:RetainedCloneProgress}

impl JsonValueProjection {
    /// 🎚️ Takes ownership of the existing parsed candidate.
    pub fn new(value:Value)->Self {Self {pending:Some(value),output:None,frames:Vec::new(),ordered:false,retirement:None,normal_step_progress:Default::default()}}
    /// 🧬️ Projects the same candidate with canonical member order for dependency identity.
    pub fn new_ordered(value:Value)->Self {let mut cursor=Self::new(value);cursor.ordered=true;cursor}
    /// 📥️ Exposes the current admitted value or consumed input frontier.
    pub fn phase(&self)->&'static str {if self.retirement.is_some(){"project-retire"}else if self.frames.last().is_some_and(|frame|!frame.admitted)||self.pending.as_ref().is_some_and(|value|matches!(value,Value::Array(_)|Value::Object(_))){"project-admit"}else{"project"}}
    /// 🪜️ Moves admitted values and drains consumed input under the same work and decode controls.
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_step_progress}
    /// 👣️ Reads actual admitted output birth and original input backing release independently.
    pub fn normal_step_demands(&self)->Result<RetirementDemand,ValueError>{
        if let Some(retirement)=&self.retirement{return Ok(RetirementDemand{copy_bytes:if semio_framework_value::retirement::RetirementCursor::terminal_is_empty(retirement){size_of::<u8>()}else{0},release_bytes:retirement.release_demand(),depth:1,..Default::default()});}
        let mut demand=RetirementDemand{depth:self.frames.len()+1,..Default::default()};
        if let Some(frame)=self.frames.last().filter(|frame|!frame.admitted){demand.capacity_bytes=if frame.is_object{frame.entries.len()*size_of::<(String,DslValue)>()}else{frame.values.len()*size_of::<DslValue>()};}
        else if self.frames.capacity()==0&&self.pending.as_ref().is_some_and(|value|matches!(value,Value::Array(_)|Value::Object(_))){demand.capacity_bytes=(MAX_DEPTH as usize+2)*size_of::<JsonProjectionFrame>();}
        Ok(demand)
    }
    pub fn step(&mut self,maximum_units:usize,maximum_bytes:usize,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<DslValue>,ValueError>{
        self.normal_step_progress=Default::default();if maximum_bytes==0{return Ok(None);}
        for _ in 0..maximum_units{
            let remaining=normal_remaining(grant,self.normal_step_progress)?;let demand=self.normal_step_demands()?;if !normal_permit(demand,remaining)?{break;}
            control.checkpoint()?;let before=control.owned_bytes();control.admit_turn_capacity(remaining.maximum_capacity_bytes)?;
            let mut released_bytes=0;let value=if let Some(retirement)=self.retirement.as_mut(){
                use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
                if retirement.terminal_is_empty(){self.retirement=None;}
                else{match retirement.close_step(remaining){RetirementStep::Failure(error)=>return Err(error),RetirementStep::Bytes(bytes)=>released_bytes=bytes,RetirementStep::Progress(progress)=>released_bytes=progress.released_bytes,RetirementStep::BudgetExhausted=>break,_=>{}}}
                None
            }else{self.advance(1,maximum_bytes,control)?};
            self.normal_step_progress=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:control.owned_bytes()-before,released_bytes})?;normal_remaining(grant,self.normal_step_progress)?;
            if value.is_some(){return Ok(value);}
        }
        Ok(None)
    }
    pub(super) fn advance(&mut self,maximum_units:usize,maximum_bytes:usize,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Option<DslValue>,ValueError> {
        if maximum_bytes==0{return Ok(None);}
        for _ in 0..maximum_units {
            control.checkpoint()?;control.step()?;
            if let Some(frame)=self.frames.last_mut().filter(|frame|!frame.admitted){if frame.is_object{frame.object=control.allocate_vec(frame.entries.len())?;}else{frame.array=control.allocate_vec(frame.values.len())?;}frame.admitted=true;continue;}
            if self.pending.as_ref().is_some_and(|value|matches!(value,Value::Array(_)|Value::Object(_)))&&self.frames.capacity()==0{self.frames=control.allocate_vec(MAX_DEPTH as usize+2)?;continue;}
            if let Some(value)=self.output.take() {if let Some(frame)=self.frames.last_mut() {if frame.is_object {frame.object.push((frame.key.take().unwrap(),value));}else {frame.array.push(value);}}else {return Ok(Some(value));}}
            else if self.pending.is_some() {
                if self.frames.len()>MAX_DEPTH as usize{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"JSON projection exceeds nesting limit"));}
                let value=self.pending.take().unwrap();self.output=match value {
                    Value::Null=>Some(DslValue::Null),Value::Bool(value)=>Some(DslValue::Bool(value)),Value::String(value)=>Some(DslValue::String(value)),
                    Value::Number(value)=>Some(DslValue::Number(match value {Number::UInt(value)=>semio_framework_value::Number::UInt(value),Number::Int(value)=>semio_framework_value::Number::Int(value),Number::Float(value)=>semio_framework_value::Number::Float(value)})),
                    Value::Array(values)=>{self.frames.push(JsonProjectionFrame {values:JsonProjectionIterator::new(values),entries:JsonProjectionIterator::new(Vec::new()),array:Vec::new(),object:Vec::new(),key:None,is_object:false,order:None,admitted:false});None},
                    Value::Object(object)=>{self.frames.push(JsonProjectionFrame {values:JsonProjectionIterator::new(Vec::new()),entries:JsonProjectionIterator::new(object.0),array:Vec::new(),object:Vec::new(),key:None,is_object:true,order:None,admitted:false});None},
                };
            } else if let Some(frame)=self.frames.last_mut() {
                if frame.is_object {if let Some((key,value))=frame.entries.next() {frame.key=Some(key);self.pending=Some(value);continue;}if self.ordered {let order=frame.order.get_or_insert_with(||JsonMemberOrder::new(frame.object.len()));if !order.step(&mut frame.object) {continue;}}}
                else if let Some(value)=frame.values.next() {self.pending=Some(value);continue;}
                let mut frame=self.frames.pop().unwrap();self.output=Some(if frame.is_object {DslValue::Object(std::mem::take(&mut frame.object))}else {DslValue::Array(std::mem::take(&mut frame.array))});self.retirement=Some(match JsonCompletedFrameRetirement::new(frame){Ok(owner)=>owner,Err((error,frame))=>{self.frames.push(frame);return Err(error);}});
            } else {return Ok(None);}
        }
        Ok(None)
    }
}

impl semio_framework_value::retirement::RetireOwned for JsonProjectionFrame {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(self.values),deferred(self.entries),deferred(self.array),deferred(self.object),deferred(self.key)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.values),deferred_birth_bytes_for(&self.entries),deferred_birth_bytes_for(&self.array),deferred_birth_bytes_for(&self.object),deferred_birth_bytes_for(&self.key)])}
    fn controlled_retirement_supported()->bool{true}
}

impl semio_framework_value::retirement::RetireOwned for JsonValueProjection {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(self.pending),deferred(self.output),deferred(self.frames),deferred(self.retirement)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.pending),deferred_birth_bytes_for(&self.output),deferred_birth_bytes_for(&self.frames),deferred_birth_bytes_for(&self.retirement)])}
    fn controlled_retirement_supported()->bool{true}
}
//#endregion 🔖️Parser
