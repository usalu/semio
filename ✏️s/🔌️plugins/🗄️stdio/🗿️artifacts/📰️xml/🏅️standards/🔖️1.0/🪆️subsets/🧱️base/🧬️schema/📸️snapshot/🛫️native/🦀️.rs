//! 🛫️ Literal XML snapshot fields are counted and emitted under cumulative ownership admission.
use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind};
use super::{XmlSnapshot, XmlNode, XmlDeclaration, XmlDoctype, XmlExternalId, XmlDtdDeclaration};
use semio_framework_os_kernel::{sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteDatabaseLimits}, io_schema::IoPayload};

struct Writer<'a, 'b, 'c> { control: &'a mut NativeEncodeControl<'b>, output: Option<&'c mut Vec<u8>>, count: usize, rows: usize, limits: SqliteDatabaseLimits, encoding: SnapshotEncoding }
impl Writer<'_, '_, '_> {
    fn bytes(&mut self, bytes: &[u8]) -> Result<(), ValueError> {
        self.count.checked_add(bytes.len()).filter(|count| *count <= self.limits.max_file_bytes && *count <= self.limits.max_value_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"XML native output exceeds caller ceiling"))?;
        for chunk in bytes.chunks(256) { if let Some(output) = &mut self.output { if output.len().checked_add(chunk.len()).filter(|size| *size <= output.capacity()).is_none() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"XML output exceeds admitted capacity")); } output.extend_from_slice(chunk); } self.count += chunk.len(); self.control.advance(chunk.len())?; } Ok(())
    }
    fn rows(&mut self, count: usize) -> Result<(), ValueError> { self.rows = self.rows.checked_add(count).filter(|count| *count <= self.limits.max_rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"XML native entity count exceeds caller ceiling"))?; Ok(()) }
    fn delimiter(&mut self, text: &[u8]) -> Result<(), ValueError> { if self.encoding == SnapshotEncoding::Text { self.bytes(text)?; } Ok(()) }
    fn byte(&mut self, byte: u8) -> Result<(), ValueError> { self.bytes(&[byte]) }
    fn varint(&mut self, mut value: u64) -> Result<(), ValueError> { let mut bytes = [0u8; 10]; let mut count = 0; loop { bytes[count] = (value & 127) as u8; value >>= 7; if value != 0 { bytes[count] |= 128; } count += 1; if value == 0 { break; } } self.bytes(&bytes[..count]) }
    fn unsigned(&mut self, value: u64) -> Result<(), ValueError> { if self.encoding == SnapshotEncoding::Binary { return self.varint(value); } let mut bytes = [0u8; 20]; let mut start = bytes.len(); let mut value = value; loop { start -= 1; bytes[start] = b'0' + (value % 10) as u8; value /= 10; if value == 0 { break; } } self.bytes(&bytes[start..]) }
    fn integer(&mut self,value:usize)->Result<(), ValueError>{self.unsigned(value as u64)}
    fn string(&mut self, text: &str) -> Result<(), ValueError> {
        if self.encoding == SnapshotEncoding::Binary { self.varint(u64::try_from(text.len()).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"XML native string exceeds width"))?)?; return self.bytes(text.as_bytes()); }
        const DIGITS: &[u8; 16] = b"0123456789abcdef"; let mut bytes = [0u8; 512];
        for chunk in text.as_bytes().chunks(256) { for (index, byte) in chunk.iter().enumerate() { bytes[2 * index] = DIGITS[usize::from(byte >> 4)]; bytes[2 * index + 1] = DIGITS[usize::from(byte & 15)]; } self.bytes(&bytes[..chunk.len() * 2])?; } Ok(())
    }
    fn flag(&mut self, value: bool) -> Result<(), ValueError> { if self.encoding == SnapshotEncoding::Binary { self.byte(u8::from(value)) } else { self.byte(if value { b'1' } else { b'0' }) } }
    fn option<T>(&mut self, value: Option<&T>, emit: impl FnOnce(&mut Self, &T) -> Result<(), ValueError>) -> Result<(), ValueError> { self.delimiter(b"[")?; self.flag(value.is_some())?; if let Some(value) = value { self.delimiter(b",")?; emit(self, value)?; } self.delimiter(b"]") }
    fn push<'n>(&mut self,pending:&mut Vec<Action<'n>>,action:Action<'n>)->Result<(),ValueError>{
        if pending.len()==pending.capacity(){let capacity=pending.capacity().checked_add(64).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"XML native frontier extent overflow"))?;let mut next=self.control.allocate_vec(capacity)?;
            self.control.scoped_stage(|control|{control.begin_stage(pending.len())?;for value in pending.drain(..){next.push(value);control.step()?;}Ok::<_,ValueError>(())})?;*pending=next;
        }pending.push(action);Ok(())
    }
    fn node(&mut self, node: &XmlNode) -> Result<(), ValueError> {
        let mut pending = Vec::new(); self.push(&mut pending, Action::Node(node))?;
        while let Some(action) = pending.pop() { match action { Action::Literal(text) => self.bytes(text)?, Action::Node(node) => {
            self.rows(2)?;
            match node {
                XmlNode::Element { name, attrs, children } => {
                    self.rows(attrs.len().checked_add(children.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"XML native relationship count overflow"))?)?;
                    if self.encoding == SnapshotEncoding::Binary { self.byte(0)?; } else { self.bytes(b"E[")?; }
                    self.string(name)?; self.delimiter(b",[")?; if self.encoding == SnapshotEncoding::Binary { self.integer(attrs.len())?; }
                    for (index, attr) in attrs.iter().enumerate() { if index != 0 { self.delimiter(b",")?; } self.delimiter(b"[")?; self.string(&attr.name)?; self.delimiter(b",")?; self.string(&attr.value)?; self.delimiter(b"]")?; }
                    self.delimiter(b"],[")?; if self.encoding == SnapshotEncoding::Binary { self.integer(children.len())?; } else { self.push(&mut pending, Action::Literal(b"]]"))?; }
                    for (index, child) in children.iter().enumerate().rev() { self.push(&mut pending, Action::Node(child))?; if index != 0 && self.encoding == SnapshotEncoding::Text { self.push(&mut pending, Action::Literal(b","))?; } }
                }
                XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => {
                    if self.encoding == SnapshotEncoding::Binary { self.byte(match node { XmlNode::Text { .. } => 1, XmlNode::CData { .. } => 2, _ => 3 })?; } else { self.bytes(match node { XmlNode::Text { .. } => b"T[", XmlNode::CData { .. } => b"D[", _ => b"M[" })?; }
                    self.string(text)?; self.delimiter(b"]")?;
                }
                XmlNode::ProcessingInstruction { target, data } => { if self.encoding == SnapshotEncoding::Binary { self.byte(4)?; } else { self.bytes(b"P[")?; } self.string(target)?; self.delimiter(b",")?; self.string(data)?; self.delimiter(b"]")?; }
            }
        } } } Ok(())
    }
    fn nodes(&mut self, nodes: &[XmlNode]) -> Result<(), ValueError> { self.rows(nodes.len())?; self.delimiter(b"[")?; if self.encoding == SnapshotEncoding::Binary { self.integer(nodes.len())?; } for (index, node) in nodes.iter().enumerate() { if index != 0 { self.delimiter(b",")?; } self.node(node)?; } self.delimiter(b"]") }
    fn declaration(&mut self, declaration: &XmlDeclaration) -> Result<(), ValueError> { self.rows(1)?; self.delimiter(b"[")?; self.string(&declaration.version)?; self.delimiter(b",")?; self.option(declaration.encoding.as_ref(), |writer, value| writer.string(value))?; self.delimiter(b",")?; self.option(declaration.standalone.as_ref(), |writer, value| writer.flag(*value))?; self.delimiter(b",")?; self.flag(!declaration.quote.is_double())?; self.delimiter(b"]") }
    fn doctype(&mut self, doctype: &XmlDoctype) -> Result<(), ValueError> {
        self.rows(1usize.checked_add(doctype.declarations.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"XML DTD row count overflow"))?)?; self.delimiter(b"[")?; self.unsigned(doctype.prolog_position)?; self.delimiter(b",")?; self.string(&doctype.name)?; self.delimiter(b",")?;
        if self.encoding == SnapshotEncoding::Binary { match &doctype.external_id { None => self.byte(0)?, Some(XmlExternalId::System { system_id }) => { self.byte(1)?; self.string(system_id)?; }, Some(XmlExternalId::Public { public_id, system_id }) => { self.byte(2)?; self.string(public_id)?; self.string(system_id)?; } } } else { self.option(doctype.external_id.as_ref(), |writer, external| { match external { XmlExternalId::System { system_id } => { writer.bytes(b"S[")?; writer.string(system_id)?; }, XmlExternalId::Public { public_id, system_id } => { writer.bytes(b"P[")?; writer.string(public_id)?; writer.delimiter(b",")?; writer.string(system_id)?; } } writer.bytes(b"]") })?; }
        self.delimiter(b",[")?; if self.encoding == SnapshotEncoding::Binary { self.integer(doctype.declarations.len())?; }
        for (index, declaration) in doctype.declarations.iter().enumerate() { if index != 0 { self.delimiter(b",")?; } let XmlDtdDeclaration::Entity { parameter, name, value } = declaration; if self.encoding == SnapshotEncoding::Binary { self.byte(1)?; } else { self.bytes(b"E[")?; } self.flag(*parameter)?; self.delimiter(b",")?; self.string(name)?; self.delimiter(b",")?; self.string(value)?; self.delimiter(b"]")?; }
        self.delimiter(b"]]")
    }
    fn document(&mut self, doc: super::sqlite::XmlDocumentView<'_>) -> Result<(), ValueError> {
        self.rows(1)?; self.option(doc.root, |writer, node| writer.node(node))?; self.delimiter(b",")?; self.option(doc.doctype, |writer, value| writer.doctype(value))?; self.delimiter(b",")?; self.option(doc.declaration, |writer, value| writer.declaration(value))?; self.delimiter(b",")?; self.nodes(doc.prolog)?; self.delimiter(b",")?; self.nodes(doc.epilog)
    }
    fn snapshot(&mut self, snapshot: &XmlSnapshot) -> Result<(), ValueError> {
        if self.encoding == SnapshotEncoding::Binary { let token = b"stdio.xml.pack v1"; self.bytes(b"\x89SEM\r\n\x1a\n")?; self.bytes(&(token.len() as u32).to_le_bytes())?; self.bytes(token)?; self.byte(1)?; } else { self.bytes(b"semio stdio.xml.dsl v1\n[")?; }
        self.string(&snapshot.schema)?; self.delimiter(b",")?; self.document(super::sqlite::XmlDocumentView::from(&snapshot.doc))?; self.delimiter(b"]")
    }
}
enum Action<'a> { Node(&'a XmlNode), Literal(&'static [u8]) }

pub(super) fn encode(snapshot: &XmlSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<IoPayload, ValueError> {
    let limits = control.limits(); control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 0)?;
    let mut callback = |event: pack::value::native_encoding::NativeEncodeProgress| control.checkpoint(SqliteSnapshotPhase::EncodeNative, event.completed, event.total).is_ok();
    let mut native = NativeEncodeControl::new(limits.max_value_bytes, &mut callback); native.begin_stage(0)?;
    let mut measure = Writer { control: &mut native, output: None, count: 0, rows: 0, limits, encoding }; measure.snapshot(snapshot)?; let count = measure.count;
    native.begin_stage(count)?; let mut bytes = native.allocate_vec(count)?;
    let mut writer = Writer { control: &mut native, output: Some(&mut bytes), count: 0, rows: 0, limits, encoding }; writer.snapshot(snapshot)?; if writer.count != count || bytes.len() != count { return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"XML native measured and emitted lengths differ")); } native.checkpoint()?;
    match encoding { SnapshotEncoding::Binary => Ok(IoPayload::Binary(bytes)), SnapshotEncoding::Text => Ok(IoPayload::Text(String::from_utf8(bytes).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"XML native output is not UTF-8"))?)) }
}

/// 🧩️ One enclosing owner's native emission state, with cumulative admission.
pub struct XmlNativeEmission<'a,'p,'o>{pub control:&'a mut NativeEncodeControl<'p>,pub output:Option<&'o mut Vec<u8>>,pub count:&'a mut usize,pub rows:&'a mut usize,pub limits:SqliteDatabaseLimits,pub encoding:SnapshotEncoding}
/// 📰️ Emits only the typed XML document component into the caller's current stream.
pub fn emit_xml_native_document(doc:super::sqlite::XmlDocumentView<'_>,state:XmlNativeEmission<'_, '_, '_>)->Result<(), ValueError>{
 let mut writer=Writer{control:state.control,output:state.output,count:*state.count,rows:*state.rows,limits:state.limits,encoding:state.encoding};
 let result=(||{writer.delimiter(b"[")?;writer.document(doc)?;writer.delimiter(b"]")})();
 *state.count=writer.count;*state.rows=writer.rows;result
}

/// 🌱️ Emits one actual XML node component without a document wrapper.
pub fn emit_xml_native_node(node:&XmlNode,state:XmlNativeEmission<'_, '_, '_>)->Result<(), ValueError>{let mut writer=Writer{control:state.control,output:state.output,count:*state.count,rows:*state.rows,limits:state.limits,encoding:state.encoding};let result=writer.node(node);*state.count=writer.count;*state.rows=writer.rows;result}

#[path="🪆️fields/🦀️.rs"]
mod fields;
pub use fields::emit_xml_native_snapshot_fields;
