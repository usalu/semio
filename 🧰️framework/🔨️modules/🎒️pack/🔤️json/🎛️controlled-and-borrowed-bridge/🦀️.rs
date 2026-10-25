//! 🎛️ Controlled and borrowed bridge: the controlled string conversions, the native write nodes and cursors and the controlled reader and writer. See [`super`].

use super::*;

//#region 🔖️ToFromValueBridge
/// 🔣️ `serde_json::to_string`/`from_str` analogs over [`ToValue`]/[`FromValue`] instead of
/// `Serialize`/`DeserializeOwned` — layered on the structural [`from_dsl_value`]/[`to_dsl_value`]
/// walk above (`//#region 🔖️DslValueBridge`) rather than a second one: a concurrent session
/// landed that walk in this same file while this one was in flight, so this region only adds the
/// generic string convenience pair it didn't have, instead of a duplicate `DslValue <-> Value`
/// conversion. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: the
/// pair every plugin routes JSON text through once it stops deriving
/// `serde::Serialize`/`Deserialize` in favor of `ToValue`/`FromValue`.
pub fn to_json_string<T: ToValue>(value: &T) -> String {
    to_string(&from_dsl_value(&value.to_value()))
}

/// 🔤️ `serde_json::from_str` analog over [`FromValue`] instead of `DeserializeOwned` — a parse
/// failure and a decode failure both collapse onto [`ValueError`], matching `from_value`'s own.
pub fn from_json_str<T: FromValue>(text: &str, policy: JsonMemberPolicy) -> Result<T, ValueError> {
    let value = parse(text, policy).map_err(JsonError::into_value_error)?;
    T::from_value(to_dsl_value(&value))
}

/// 🚦️ Parses and binds JSON under the caller's cumulative native ownership control.
pub fn from_json_str_controlled<T: FromValue>(text: &str, policy: JsonMemberPolicy, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<T, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(text.len())?;
        let mut reader = ControlledReader { lexer: Lexer::new(text), policy, control };
        let value = reader.value()?.guard_decoded(); reader.whitespace()?;
        if reader.lexer.pos != text.len() { return Err(JsonError::TrailingData(reader.lexer.pos).into_value_error()); }
        reader.control.scoped_stage(|control| { control.begin_stage(0)?; T::from_value_controlled(value.get(), control) })
    })
}

/// 🛫️ Writes ordered JSON under the caller's cumulative native ownership control.
pub fn to_json_string_controlled<T: ToValue>(value: &T, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?; let value = value.to_value_controlled(control)?.guard_decoded();
        let mut measure = ControlledWriter { bytes: 0, output: None }; measure.value(value.get(), control)?;
        control.charge(measure.bytes)?; let mut output = String::new(); output.try_reserve_exact(measure.bytes).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "controlled JSON output allocation failed"))?;
        let mut writer = ControlledWriter { bytes: 0, output: Some(output) }; writer.value(value.get(), control)?;
        if writer.bytes != measure.bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON output changed after admission")); } Ok(writer.output.unwrap())
    })
}

#[derive(Clone, Copy)]
struct JsonWriteFrame { pub state: u8, pub(super) index: usize, pub position: usize }

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

/// 🔎️ A canonical JSON scalar or collection borrowed directly from the retained source owner.
pub enum JsonWriteNode<'a>{Null,Bool(bool),Number(semio_framework_value::Number),String(&'a str),NativeString(&'a dyn semio_framework_value::paged::Utf8Text),Array(usize),Object(usize)}

/// 🌱️ Source owners provide ordinal views without projecting or copying their payload first.
pub trait JsonWriteSource{
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

/// 🧵️ Retains physical writer state while borrowing the unchanged semantic owner per grant.
pub struct JsonBorrowedWriteCursor {
    #[doc(hidden)] pub frames: Vec<JsonWriteFrame>, #[doc(hidden)] pub path: Vec<usize>, writer: ControlledWriter, #[doc(hidden)] pub phase: u8, pub(super) normal_step_progress:RetainedCloneProgress,
}

impl JsonBorrowedWriteCursor {
    /// ▶️ Starts without copying or scanning a source owner.
    pub fn new() -> Self { Self { frames: Vec::new(), path: Vec::new(), writer: ControlledWriter { bytes: 0, output: None }, phase: 0, normal_step_progress:Default::default() } }
    /// 📍️ Returns the current canonical output byte count and measure/write phase.
    pub fn progress(&self) -> (usize, bool) { (self.writer.bytes, self.phase == 2) }
    /// ⏱️ Advances at most the supplied structural or scalar-character transitions.
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_step_progress}
    /// 📏️ Quotes one actual scalar write, natural frame birth or final output allocation.
    pub fn normal_step_demands<S:JsonWriteSource+?Sized>(&self,source:&S)->Result<RetirementDemand,ValueError>{
        let mut demand=RetirementDemand{depth:self.path.len()+1,..Default::default()};
        if self.phase==0{demand.capacity_bytes=(MAX_DEPTH as usize+1)*size_of::<JsonWriteFrame>()+MAX_DEPTH as usize*size_of::<usize>();return Ok(demand);}
        if self.frames.is_empty(){if self.phase==1{demand.capacity_bytes=self.writer.bytes;}return Ok(demand);}
        if self.phase!=2{return Ok(demand);}
        let frame=*self.frames.last().unwrap();let node=source.node_at_path(&self.path)?;
        let mut measured=ControlledWriter{bytes:0,output:None};let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(isize::MAX as usize,&mut accepted);
        match frame.state{
            0=>match node{JsonWriteNode::Null=>measured.raw("null",&mut control)?,JsonWriteNode::Bool(value)=>measured.raw(if value{"true"}else{"false"},&mut control)?,JsonWriteNode::Number(value)=>measured.number(value,&mut control)?,_=>measured.bytes=1},
            1=>{let JsonWriteNode::Array(length)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next array source changed"));};measured.bytes=usize::from(frame.index==length||frame.index!=0);},
            2=>{let JsonWriteNode::Object(length)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next object source changed"));};measured.bytes=if frame.index==length{1}else{1+usize::from(frame.index!=0)};},
            3|5=>{let text=if frame.state==3{source.object_key_at_path(&self.path,frame.index)?}else{let JsonWriteNode::String(text)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next string source changed"));};text};if let Some(character)=text.get(frame.position..).and_then(|text|text.chars().next()){measured.character(character,&mut control)?;}else{measured.bytes=if frame.state==3{2}else{1};}},
            6=>{let JsonWriteNode::NativeString(text)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON next native string source changed"));};if frame.index==text.text_chunk_count(){measured.bytes=1;}else if let Some(character)=text.text_chunk(frame.index).and_then(|text|text.get(frame.position..)).and_then(|text|text.chars().next()){measured.character(character,&mut control)?;}},
            _=>{},
        }
        demand.copy_bytes=measured.bytes;Ok(demand)
    }
    #[doc(hidden)] pub fn retained_capacity_bytes(&self)->Result<usize,ValueError>{
        self.frames.capacity().checked_mul(size_of::<JsonWriteFrame>()).and_then(|frames|self.path.capacity().checked_mul(size_of::<usize>()).and_then(|path|frames.checked_add(path))).and_then(|bytes|bytes.checked_add(self.writer.output.as_ref().map_or(0,|output|output.capacity()))).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"JSON writer physical backing extent overflow"))
    }
    pub fn step<S:JsonWriteSource+?Sized>(&mut self,source:&S,maximum_units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
        self.normal_step_progress=Default::default();
        self.step_original(source,maximum_units,control,grant).map_err(|error|error.with_retained_progress(self.normal_step_progress))
    }
    fn step_original<S:JsonWriteSource+?Sized>(&mut self,source:&S,maximum_units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
        for _ in 0..maximum_units {
            let remaining=normal_remaining(grant,self.normal_step_progress)?;let demand=self.normal_step_demands(source)?;
            if !normal_permit(demand,remaining)?{break;}
            control.admit_turn_capacity(remaining.maximum_capacity_bytes)?;control.checkpoint()?;
            if self.phase==3{return Ok(None);}
            if self.frames.is_empty()&&self.phase==2{
                self.normal_step_progress=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:1,..Default::default()})?;
                self.phase=3;return Ok(self.writer.output.take());
            }
            let retained_before=self.retained_capacity_bytes()?;
            let copied_before=self.writer.output.as_ref().map_or(0,|output|output.len());
            let frontier_before=(self.phase,self.frames.len(),self.path.len(),self.writer.bytes);
            let result=(||->Result<(),ValueError>{
                if self.phase==0{
                    self.frames=control.allocate_vec(MAX_DEPTH as usize+1)?;
                    self.path=control.allocate_vec(MAX_DEPTH as usize)?;
                    self.frames.push(JsonWriteFrame{state:0,index:0,position:0});self.phase=1;
                }else if self.frames.is_empty(){
                    control.charge(self.writer.bytes)?;let mut output=String::new();
                    output.try_reserve_exact(self.writer.bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"controlled JSON output allocation failed"))?;
                    self.writer=ControlledWriter{bytes:0,output:Some(output)};self.frames.push(JsonWriteFrame{state:0,index:0,position:0});self.phase=2;
                }else{self.advance(source,control)?;}
                Ok(())
            })();
            let retained_after=self.retained_capacity_bytes()?;
            let copied_after=self.writer.output.as_ref().map_or(0,|output|output.len());
            if retained_after<retained_before||copied_after<copied_before{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON writer released an ungranted normal backing"))}
            let changed=result.is_ok()||retained_after!=retained_before||copied_after!=copied_before||frontier_before!=(self.phase,self.frames.len(),self.path.len(),self.writer.bytes);
            self.normal_step_progress=self.normal_step_progress.checked_add(RetainedCloneProgress{copied_items:usize::from(changed),copied_bytes:copied_after-copied_before,retained_capacity_bytes:retained_after-retained_before,released_bytes:0})?;
            normal_remaining(grant,self.normal_step_progress)?;
            result?;control.step()?;
        }
        Ok(None)
    }
    fn finish_node(&mut self) { self.frames.pop(); if !self.frames.is_empty() { self.path.pop(); } }
    pub(super) fn child(&mut self, index: usize) -> Result<(), ValueError> {
        if self.path.len() >= MAX_DEPTH as usize { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "retained JSON writer exceeds depth limit")); }
        self.path.push(index); self.frames.push(JsonWriteFrame { state: 0, index: 0, position: 0 }); Ok(())
    }
    pub(super) fn advance<S:JsonWriteSource+?Sized>(&mut self, source:&S, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        let frame = *self.frames.last().expect("writer frontier is inhabited");
        let node = source.node_at_path(&self.path)?;
        match frame.state {
            0 => {
                match node {
                    JsonWriteNode::Null => self.writer.raw("null", control)?,
                    JsonWriteNode::Bool(value) => self.writer.raw(if value { "true" } else { "false" }, control)?,
                    JsonWriteNode::Number(value) => self.writer.number(value, control)?,
                    JsonWriteNode::String(_) => { self.writer.raw("\"", control)?; self.frames.last_mut().unwrap().state = 5; return Ok(()); }
                    JsonWriteNode::NativeString(_) => {self.writer.raw("\"",control)?;self.frames.last_mut().unwrap().state=6;return Ok(());}
                    JsonWriteNode::Array(_) => { self.writer.raw("[", control)?; self.frames.last_mut().unwrap().state = 1; return Ok(()); }
                    JsonWriteNode::Object(_) => { self.writer.raw("{", control)?; self.frames.last_mut().unwrap().state = 2; return Ok(()); }
                }
                self.finish_node();
            }
            1 => {
                let JsonWriteNode::Array(length)=node else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON array source changed"))};
                if frame.index == length { self.writer.raw("]", control)?; self.finish_node(); }
                else { if frame.index != 0 { self.writer.raw(",", control)?; } self.frames.last_mut().unwrap().index += 1; self.child(frame.index)?; }
            }
            2 => {
                let JsonWriteNode::Object(length) = node else { return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON object source changed")); };
                if frame.index == length { self.writer.raw("}", control)?; self.finish_node(); }
                else { if frame.index != 0 { self.writer.raw(",", control)?; } self.writer.raw("\"", control)?; let next = self.frames.last_mut().unwrap(); next.state = 3; next.position = 0; }
            }
            3 | 5 => {
                let text = match node { JsonWriteNode::String(text) if frame.state == 5 => text, JsonWriteNode::Object(_) if frame.state == 3 => source.object_key_at_path(&self.path,frame.index)?, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"JSON text source changed")) };
                let remaining=text.get(frame.position..).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"JSON text source position changed"))?;
                if let Some(character) = remaining.chars().next() { self.writer.character(character, control)?; self.frames.last_mut().unwrap().position += character.len_utf8(); }
                else { self.writer.raw("\"", control)?; if frame.state == 5 { self.finish_node(); } else { self.writer.raw(":", control)?; self.frames.last_mut().unwrap().state = 4; } }
            }
            4 => { let next = self.frames.last_mut().unwrap(); next.state = 2; next.index += 1; self.child(frame.index)?; }
            6 => {
                let JsonWriteNode::NativeString(text)=node else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text source changed"));};
                if frame.index==text.text_chunk_count(){self.writer.raw("\"",control)?;self.finish_node();}
                else {
                    let body=text.text_chunk(frame.index).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text chunk absent"))?;
                    let remaining=body.get(frame.position..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"JSON native text offset changed"))?;
                    if let Some(character)=remaining.chars().next(){self.writer.character(character,control)?;self.frames.last_mut().unwrap().position+=character.len_utf8();}
                    else {let next=self.frames.last_mut().unwrap();next.index+=1;next.position=0;}
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

/// 🌳️ Owns its semantic source and the canonical borrowed writer state.
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

impl semio_framework_value::retirement::RetireOwned for JsonBorrowedWriteCursor {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::*;sequence(vec![deferred(self.frames),deferred(JsonProjectionIterator::new(self.path)),deferred(self.writer.output)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.frames),deferred_birth_bytes::<JsonProjectionIterator<usize>>(),deferred_birth_bytes_for(&self.writer.output)])}
    fn controlled_retirement_supported()->bool{true}
}

impl<S:JsonWriteSource+semio_framework_value::retirement::RetireOwned+'static> semio_framework_value::retirement::RetireOwned for JsonWriteCursor<S> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { use semio_framework_value::retirement::*;sequence(vec![deferred(self.source),deferred(self.state.frames),deferred(JsonProjectionIterator::new(self.state.path)),deferred(self.state.writer.output)]) }
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::*;sequence_birth_bytes(&[deferred_birth_bytes_for(&self.source),deferred_birth_bytes_for(&self.state.frames),deferred_birth_bytes::<JsonProjectionIterator<usize>>(),deferred_birth_bytes_for(&self.state.writer.output)])}
    fn controlled_retirement_supported()->bool{S::controlled_retirement_supported()}
}

struct ControlledWriter { pub bytes: usize, pub output: Option<String> }

impl ControlledWriter {
    pub(super) fn character(&mut self, character: char, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        match character {
            '"' => self.raw("\\\"", control), '\\' => self.raw("\\\\", control), '\u{0008}' => self.raw("\\b", control), '\u{000c}' => self.raw("\\f", control), '\n' => self.raw("\\n", control), '\r' => self.raw("\\r", control), '\t' => self.raw("\\t", control),
            character if (character as u32) < 0x20 => { use fmt::Write as _; let mut scalar = ScalarText::new(); write!(scalar, "\\u{:04x}", character as u32).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON escape overflow"))?; self.raw(scalar.text(), control) }
            character => { let mut bytes = [0; 4]; self.raw(character.encode_utf8(&mut bytes), control) }
        }
    }
    pub(super) fn raw(&mut self, text: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        self.bytes = self.bytes.checked_add(text.len()).filter(|bytes| *bytes<=isize::MAX as usize&&(self.output.is_none()||*bytes<=control.maximum_bytes())).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "controlled JSON output exceeds caller limit"))?;
        if self.output.as_ref().is_some_and(|output| self.bytes > output.capacity()) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON output exceeds admitted allocation")); }
        if text.len() <= 65536 { if let Some(output) = self.output.as_mut() { output.push_str(text); } return Ok(()); }
        control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(text.len())?; let mut position = 0; while position < text.len() { let mut end = position.saturating_add(65536).min(text.len()); while !text.is_char_boundary(end) { end -= 1; } if let Some(output) = self.output.as_mut() { output.push_str(&text[position..end]); } control.advance(end - position)?; position = end; } Ok(()) })
    }
    pub(super) fn string(&mut self, text: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        self.raw("\"", control)?;
        control.scoped_stage(|control| -> Result<(), ValueError> { control.begin_stage(text.len())?; for character in text.chars() { self.character(character, control)?; control.advance(character.len_utf8())?; } Ok(()) })?;
        self.raw("\"", control)
    }
    pub(super) fn number(&mut self, number: semio_framework_value::Number, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), ValueError> {
        use fmt::Write as _; control.checkpoint()?; let mut scalar = ScalarText::new();
        match number { semio_framework_value::Number::UInt(value) => write!(scalar, "{value}"), semio_framework_value::Number::Int(value) => write!(scalar, "{value}"), semio_framework_value::Number::Float(value) => write_float_to(value, &mut scalar) }.map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "controlled JSON scalar overflow"))?;
        self.raw(scalar.text(), control)?; control.checkpoint()
    }
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
