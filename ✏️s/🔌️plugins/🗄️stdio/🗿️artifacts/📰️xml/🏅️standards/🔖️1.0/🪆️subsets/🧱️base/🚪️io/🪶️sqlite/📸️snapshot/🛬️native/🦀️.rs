//! 🛬️ Controlled literal XML state decoding with guarded iterative native tree construction.
use crate::standards::v1_0::subsets::base::schema::snapshot::{XmlSnapshot, XmlNode, XmlAttr, XmlDeclaration, XmlQuote, XmlDoctype, XmlExternalId, XmlDtdDeclaration, XmlDocument};
use semio_framework_value::{NativeDecodeControl, ValueError, ValueRefusalKind};
use crate::standards::v1_0::subsets::base::io::sqlite::snapshot::{XmlNodeList, XmlDocumentOwner};
use semio_framework_os_kernel::{sqlite_snapshot::{SqliteSnapshotControl, SqliteSnapshotPhase, SqliteDatabaseLimits}, io_schema::IoPayload};

struct Reader<'a, 'c, 'p> { bytes: &'a [u8], position: usize, binary: bool, rows: usize, limits: SqliteDatabaseLimits, control: &'c mut NativeDecodeControl<'p> }
impl Reader<'_, '_, '_> {
    fn rows(&mut self, count: usize) -> Result<(), ValueError> { self.rows = self.rows.checked_add(count).filter(|count| *count <= self.limits.max_rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"XML native rows exceed caller ceiling"))?; Ok(()) }
    fn take(&mut self, count: usize) -> Result<&[u8], ValueError> { let end = self.position.checked_add(count).filter(|end| *end <= self.bytes.len()).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"XML native input is truncated"))?; let start = self.position; while self.position < end { let next = self.position.saturating_add(256).min(end); self.control.advance(next - self.position)?; self.position = next; } Ok(&self.bytes[start..end]) }
    fn byte(&mut self) -> Result<u8, ValueError> { Ok(self.take(1)?[0]) }
    fn expected(&mut self, expected: u8) -> Result<(), ValueError> { if self.byte()? != expected { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native literal delimiter differs")); } Ok(()) }
    fn delimiter(&mut self, expected: u8) -> Result<(), ValueError> { if !self.binary { self.expected(expected)?; } Ok(()) }
    fn peek(&self) -> Option<u8> { self.bytes.get(self.position).copied() }
    fn varint(&mut self) -> Result<u64, ValueError> { let mut value = 0u64; for shift in (0..70).step_by(7) { let byte = self.byte()?; if shift == 63 && byte > 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native varint exceeds width")); } value |= u64::from(byte & 127) << shift; if byte & 128 == 0 { return Ok(value); } } Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native varint is overlong")) }
    fn integer(&mut self) -> Result<usize, ValueError> { if self.binary { return usize::try_from(self.varint()?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"XML native ordinal exceeds platform width")); } let start = self.position; let mut value = 0usize; while self.peek().is_some_and(|byte| byte.is_ascii_digit()) { value = value.checked_mul(10).and_then(|value| value.checked_add(usize::from(self.bytes[self.position] - b'0'))).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"XML native ordinal exceeds width"))?; self.byte()?; } if self.position == start { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native ordinal is absent")); } Ok(value) }
    fn unsigned(&mut self)->Result<u64, ValueError>{if self.binary{return self.varint();}let start=self.position;let mut value=0u64;while self.peek().is_some_and(|byte|byte.is_ascii_digit()){value=value.checked_mul(10).and_then(|value|value.checked_add(u64::from(self.bytes[self.position]-b'0'))).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"XML position exceeds unsigned64"))?;self.byte()?;}if self.position==start{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML position is absent"));}if self.position-start>1&&self.bytes[start]==b'0'{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML position is not canonical"));}Ok(value)}
    fn flag(&mut self) -> Result<bool, ValueError> { let byte = self.byte()?; match (self.binary, byte) { (true, 0) | (false, b'0') => Ok(false), (true, 1) | (false, b'1') => Ok(true), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native boolean is invalid")) } }
    fn string(&mut self) -> Result<String, ValueError> {
        if self.binary {
            let count = usize::try_from(self.varint()?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"XML string length exceeds platform width"))?;
            let end = self.position.checked_add(count).filter(|end| *end <= self.bytes.len()).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"XML string is truncated"))?; let start = self.position; self.take(count)?;
            let text = self.control.borrow_text(&self.bytes[start..end])?; return self.control.copy_text(text);
        }
        let start = self.position; while self.peek().is_some_and(|byte| byte.is_ascii_hexdigit()) { let mut length = 0; while length < 256 && self.bytes.get(self.position + length).is_some_and(|byte| byte.is_ascii_hexdigit()) { length += 1; } self.take(length)?; }
        let count = self.position - start; if count % 2 != 0 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native text field has odd hexadecimal width")); }
        let mut output = self.control.allocate_vec::<u8>(count / 2)?;
        self.control.scoped_stage(|control| { control.begin_stage(count / 2)?; for chunk in self.bytes[start..self.position].chunks(512) { for pair in chunk.chunks_exact(2) { let high = digit(pair[0])?; let low = digit(pair[1])?; output.push((high << 4) | low); } control.advance(chunk.len() / 2)?; } Ok::<(), ValueError>(()) })?;
        self.control.borrow_text(&output)?; String::from_utf8(output).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"XML native text is not UTF-8"))
    }
    fn option<T>(&mut self, parse: impl FnOnce(&mut Self) -> Result<T, ValueError>) -> Result<Option<T>, ValueError> { self.delimiter(b'[')?; let some = self.flag()?; let value = if some { self.delimiter(b',')?; Some(parse(self)?) } else { None }; self.delimiter(b']')?; Ok(value) }
    fn reserve<T>(&mut self,values:&mut Vec<T>)->Result<(),ValueError>{
        if values.len()==values.capacity(){let capacity=values.capacity().checked_add(64).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"XML native frontier extent overflow"))?;let mut next=self.control.allocate_vec(capacity)?;
            self.control.scoped_stage(|control|{control.begin_stage(values.len())?;for value in values.drain(..){next.push(value);control.step()?;}Ok::<_,ValueError>(())})?;*values=next;
        }Ok(())
    }
    fn node_start(&mut self) -> Result<Start, ValueError> {
        self.rows(2)?; let tag = self.byte()?; self.delimiter(b'[')?;
        match (self.binary, tag) {
            (true, 0) | (false, b'E') => {
                let name = self.string()?; self.delimiter(b',')?; self.delimiter(b'[')?;
                let count = if self.binary { self.integer()? } else { 0 }; if self.binary { self.rows(count)?; }
                let mut attrs = self.control.allocate_vec::<XmlAttr>(count)?; let mut ordinal = 0;
                while if self.binary { ordinal < count } else { self.peek() != Some(b']') } {
                    if !self.binary { self.rows(1)?; self.reserve(&mut attrs)?; } if ordinal != 0 { self.delimiter(b',')?; } self.delimiter(b'[')?; let name = self.string()?; self.delimiter(b',')?; let value = self.string()?; self.delimiter(b']')?; attrs.push(XmlAttr { name, value }); ordinal += 1;
                }
                self.delimiter(b']')?; self.delimiter(b',')?; self.delimiter(b'[')?;
                let count = if self.binary { self.integer()? } else { 0 }; if self.binary { self.rows(count)?; }
                Ok(Start::Element(Frame { name, attrs, children: XmlNodeList(self.control.allocate_vec::<XmlNode>(count)?), remaining: count, ordinal: 0 }))
            }
            (true, 1) | (false, b'T') => { let text = self.string()?; self.delimiter(b']')?; Ok(Start::Node(XmlNode::Text { text })) }
            (true, 2) | (false, b'D') => { let text = self.string()?; self.delimiter(b']')?; Ok(Start::Node(XmlNode::CData { text })) }
            (true, 3) | (false, b'M') => { let text = self.string()?; self.delimiter(b']')?; Ok(Start::Node(XmlNode::Comment { text })) }
            (true, 4) | (false, b'P') => { let target = self.string()?; self.delimiter(b',')?; let data = self.string()?; self.delimiter(b']')?; Ok(Start::Node(XmlNode::ProcessingInstruction { target, data })) }
            _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native node kind is invalid")),
        }
    }
    fn node(&mut self) -> Result<XmlNode, ValueError> {
        let mut pending: Vec<Frame> = Vec::new(); let mut start = Some(self.node_start()?); let mut complete = XmlNodeList(self.control.allocate_vec(1)?);
        loop {
            if let Some(start) = start.take() { match start { Start::Node(node) => complete.0.push(node), Start::Element(frame) => { self.reserve(&mut pending)?; pending.push(frame); } } }
            if !complete.0.is_empty() {
                if let Some(parent) = pending.last_mut() { self.reserve(&mut parent.children.0)?; parent.children.0.push(complete.0.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native completed node is absent"))?); } else { return complete.0.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native completed node is absent")); }
            }
            let parent = pending.last_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native element frame is absent"))?;
            let next = if self.binary { parent.remaining != 0 } else { self.peek() != Some(b']') };
            if next { if self.binary { parent.remaining -= 1; } else { self.rows(1)?; if parent.ordinal != 0 { self.expected(b',')?; } } parent.ordinal += 1; start = Some(self.node_start()?); }
            else { self.delimiter(b']')?; self.delimiter(b']')?; let mut frame = pending.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native element frame is absent"))?; complete.0.push(XmlNode::Element { name: frame.name, attrs: frame.attrs, children: std::mem::take(&mut frame.children.0) }); }
        }
    }
    fn root(&mut self) -> Result<Option<XmlNode>, ValueError> {
        self.delimiter(b'[')?; let some = self.flag()?;
        if !some { self.delimiter(b']')?; return Ok(None); }
        self.delimiter(b',')?; let node = pack::value::DecodedValue::new(self.node()?, |node| crate::standards::v1_0::subsets::base::io::sqlite::snapshot::retire_nodes([node])); self.delimiter(b']')?; Ok(Some(node.take()))
    }
    fn nodes(&mut self) -> Result<XmlNodeList, ValueError> { self.delimiter(b'[')?; let count = if self.binary { self.integer()? } else { 0 }; if self.binary { self.rows(count)?; } let mut values = XmlNodeList(self.control.allocate_vec(count)?); let mut ordinal = 0;
        while if self.binary { ordinal < count } else { self.peek() != Some(b']') } { if !self.binary { self.rows(1)?; self.reserve(&mut values.0)?; } if ordinal != 0 { self.delimiter(b',')?; } let node = self.node()?; values.0.push(node); ordinal += 1; } self.delimiter(b']')?; Ok(values) }
    fn declaration(&mut self) -> Result<XmlDeclaration, ValueError> { self.rows(1)?; self.delimiter(b'[')?; let version = self.string()?; self.delimiter(b',')?; let encoding = self.option(|reader| reader.string())?; self.delimiter(b',')?; let standalone = self.option(|reader| reader.flag())?; self.delimiter(b',')?; let quote = if self.flag()? { XmlQuote::Single } else { XmlQuote::Double }; self.delimiter(b']')?; Ok(XmlDeclaration { version, encoding, standalone, quote }) }
    fn doctype(&mut self) -> Result<XmlDoctype, ValueError> { self.rows(1)?; self.delimiter(b'[')?; let prolog_position = self.unsigned()?; self.delimiter(b',')?; let name = self.string()?; self.delimiter(b',')?;
        let external_id = if self.binary { match self.byte()? { 0 => None, 1 => Some(XmlExternalId::System { system_id: self.string()? }), 2 => Some(XmlExternalId::Public { public_id: self.string()?, system_id: self.string()? }), _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native external identifier tag is invalid")) } } else { self.option(|reader| { let tag = reader.byte()?; reader.expected(b'[')?; let value = match tag { b'S' => XmlExternalId::System { system_id: reader.string()? }, b'P' => { let public_id = reader.string()?; reader.expected(b',')?; XmlExternalId::Public { public_id, system_id: reader.string()? } }, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native external identifier tag is invalid")) }; reader.expected(b']')?; Ok(value) })? };
        self.delimiter(b',')?; self.delimiter(b'[')?; let count = if self.binary { self.integer()? } else { 0 }; if self.binary { self.rows(count)?; } let mut declarations = self.control.allocate_vec(count)?; let mut ordinal = 0;
        while if self.binary { ordinal < count } else { self.peek() != Some(b']') } { if !self.binary { self.rows(1)?; self.reserve(&mut declarations)?; } if ordinal != 0 { self.delimiter(b',')?; } self.expected(if self.binary { 1 } else { b'E' })?; self.delimiter(b'[')?; let parameter = self.flag()?; self.delimiter(b',')?; let name = self.string()?; self.delimiter(b',')?; let value = self.string()?; self.delimiter(b']')?; declarations.push(XmlDtdDeclaration::Entity { parameter, name, value }); ordinal += 1; }
        self.delimiter(b']')?; self.delimiter(b']')?; Ok(XmlDoctype { prolog_position, name, external_id, declarations }) }
    fn document(&mut self) -> Result<XmlDocumentOwner, ValueError> {
        self.rows(1)?;let mut owner=XmlDocumentOwner(Some(XmlDocument::default()));let doc=owner.0.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native document owner is empty"))?;
        doc.root=self.root()?;self.delimiter(b',')?;doc.doctype=self.option(|reader|reader.doctype())?;self.delimiter(b',')?;doc.declaration=self.option(|reader|reader.declaration())?;self.delimiter(b',')?;let mut prolog=self.nodes()?;doc.prolog=std::mem::take(&mut prolog.0);self.delimiter(b',')?;let mut epilog=self.nodes()?;doc.epilog=std::mem::take(&mut epilog.0);Ok(owner)
    }
    fn snapshot(&mut self) -> Result<XmlSnapshot, ValueError> {
        if self.binary {self.expected(1)?;}else{self.expected(b'[')?;}
        let schema=self.string()?;self.delimiter(b',')?;let mut owner=self.document()?;self.delimiter(b']')?;
        if self.position!=self.bytes.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native input has trailing fields"));}
        self.control.checkpoint()?;Ok(XmlSnapshot{schema,doc:owner.0.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native document owner is empty"))?})
    }
}
fn digit(byte: u8) -> Result<u8, ValueError> { match byte { b'0'..=b'9' => Ok(byte - b'0'), b'a'..=b'f' => Ok(byte - b'a' + 10), b'A'..=b'F' => Ok(byte - b'A' + 10), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"XML native hexadecimal digit is invalid")) } }
struct Frame { name: String, attrs: Vec<XmlAttr>, children: XmlNodeList, remaining: usize, ordinal: usize }
enum Start { Node(XmlNode), Element(Frame) }

pub(crate) fn decode(payload: &IoPayload, control: &mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<XmlSnapshot, ValueError> {
    let limits = control.limits(); let length = match payload { IoPayload::Text(text) => text.len(), IoPayload::Binary(bytes) => bytes.len() }; if length > limits.max_file_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"XML native file exceeds caller ceiling")); } control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, length)?;
    let mut callback = |event: pack::value::native_decoding::NativeDecodeProgress| control.checkpoint(SqliteSnapshotPhase::DecodeNative, event.completed, event.total).is_ok(); let mut native = NativeDecodeControl::new(limits.max_value_bytes, &mut callback);
    let (bytes, binary) = match payload { IoPayload::Binary(bytes) => (store::semio_format::unwrap_binary_controlled(bytes, "stdio.xml", store::semio_format::Component::Pack, 1, &mut native).map_err(|error|match error{store::semio_format::SemioError::DecodingControl(error)=>error,error=>ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())})?, true), IoPayload::Text(text) => (store::semio_format::split_text_preamble_controlled(text, "stdio.xml", store::semio_format::Component::Dsl, 1, &mut native).map_err(|error|match error{store::semio_format::SemioError::DecodingControl(error)=>error,error=>ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())})?.as_bytes(), false) };
    native.begin_stage(bytes.len())?; Reader { bytes, position: 0, binary, rows: 0, limits, control: &mut native }.snapshot()
}

/// 🧩️ One enclosing owner's borrowed native cursor and cumulative ownership control.
pub struct XmlNativeInput<'a,'p,'i>{pub bytes:&'i[u8],pub position:&'a mut usize,pub rows:&'a mut usize,pub limits:SqliteDatabaseLimits,pub binary:bool,pub control:&'a mut NativeDecodeControl<'p>}
/// 📰️ Constructs one owned XML document component without consuming enclosing fields.
pub fn read_xml_native_document(state:XmlNativeInput<'_, '_, '_>)->Result<XmlDocument, ValueError>{
 if state.bytes.len()>state.limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"XML enclosing native input exceeds caller ceiling"));}
 let mut reader=Reader{bytes:state.bytes,position:*state.position,binary:state.binary,rows:*state.rows,limits:state.limits,control:state.control};
 let result=(||{reader.delimiter(b'[')?;let mut owner=reader.document()?;reader.delimiter(b']')?;Ok(owner.0.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"XML native document owner is empty"))?)})();
 *state.position=reader.position;*state.rows=reader.rows;result
}

/// 🌱️ Constructs one actual XML node under the enclosing cumulative cursor.
pub fn read_xml_native_node(state:XmlNativeInput<'_, '_, '_>)->Result<XmlNode, ValueError>{if state.bytes.len()>state.limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"XML enclosing native input exceeds caller ceiling"));}let mut reader=Reader{bytes:state.bytes,position:*state.position,binary:state.binary,rows:*state.rows,limits:state.limits,control:state.control};let result=reader.node();*state.position=reader.position;*state.rows=reader.rows;result}

#[path="🪆️fields/🦀️.rs"]
mod fields;
pub use fields::read_xml_native_snapshot_fields;
