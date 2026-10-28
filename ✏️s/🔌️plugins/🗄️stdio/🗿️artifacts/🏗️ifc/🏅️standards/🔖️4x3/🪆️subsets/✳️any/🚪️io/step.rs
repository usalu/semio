//! 🚪 IFC4X3_ADD2 source STEP boundary and registered native graph composition.
use pack::snapshot::ArtifactPack;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::io::{AnalyzeSource, ComposeError, ComposeSource, ComposerEntry, Composition, Confidence};
use semio_framework_plugin::{ArtifactComposer, ArtifactComposition, ArtifactDeserializer, ArtifactSerializer};
use semio_s_artifact_stdio_contract::part21::{self, Part21Header, Part21Value};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{self, IfcSnapshot, IfcValue};
use semio_s_artifact_stdio_txt::TxtSnapshot;
use std::collections::BTreeSet;
use std::fmt;
use std::sync::OnceLock;

#[path = "step-source.rs"]
mod source;
use source::SourceWalk;
#[path = "native-step-emit.rs"]
mod emit;
#[path = "native-step-write.rs"]
mod writing;
#[path = "step-native.rs"]
mod native;
pub use native::Ifc4x3IntoTxtNative;
#[path="native-step-read.rs"]
mod reading;
#[path="native-step-import.rs"]
mod importing;
pub use reading::{read as receive_ifc4x3_step_txt,NativeStepReadError};
pub use importing::TxtIntoIfc4x3Native;

/// 🫴️ Publishes natural STEP inside the UTF-8 artifact envelope through the original encoder owner.
pub fn write_ifc4x3_step_pack(source: &IfcSnapshot, owner: &mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, semio_framework_value::ValueError> { writing::encode(source, owner) }

pub const IFC4X3_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4x3"), subset: SubsetId("*") };
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
pub const IFC4X3_STEP_SCHEMA: &str = "IFC4X3_ADD2";
const MAX_SOURCE_BYTES: usize = 256 * 1024 * 1024;
const MAX_RECORD_BYTES: usize = 8 * 1024 * 1024;
const MAX_DEPTH: usize = 64;
const EMPTY_HEADER: &str = "ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;";
const END: &str = "ENDSEC;END-ISO-10303-21;";

/// 📍 Import refusals identify original UTF-8 bytes; export uses produced byte positions or zero before source exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfcStepError {
    pub code: &'static str,
    pub message: String,
    pub offset: usize,
}
impl fmt::Display for IfcStepError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(output, "{} at {}: {}", self.code, self.offset, self.message)
    }
}
impl std::error::Error for IfcStepError {}

/// ⏱ A caller-owned checkpoint describing actual source work; total zero denotes an unknown total.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcStepProgress {
    pub phase: &'static str,
    pub completed: usize,
    pub total: usize,
}
fn refusal(code: &'static str, message: impl Into<String>, offset: usize) -> IfcStepError {
    IfcStepError { code, message: message.into(), offset }
}
fn checkpoint(control: &mut dyn FnMut(IfcStepProgress) -> bool, phase: &'static str, completed: usize, total: usize) -> Result<(), IfcStepError> {
    if control(IfcStepProgress { phase, completed, total }) {
        Ok(())
    } else {
        Err(refusal("io.cancelled", "IFC exchange cancelled before publication", if phase == "scan" || phase == "parse" { completed } else { 0 }))
    }
}

struct Records<'a> {
    text: &'a str,
    at: usize,
    max_record_bytes: usize,
}
impl<'a> Records<'a> {
    fn next(&mut self, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<Option<&'a str>, IfcStepError> {
        let start = self.at;
        let mut source = SourceWalk::new(self.text, start, 0, self.text.len(), start.saturating_add(self.max_record_bytes));
        let mut depth = 0usize;
        source.trivia(control)?;
        while let Some(byte) = source.peek(control)? {
            match byte {
                b'\'' | b'"' => source.quoted(byte, control)?,
                b'(' => { depth += 1; if depth > MAX_DEPTH + 2 { return Err(refusal("step.depth", "STEP nesting exceeds the bounded source depth", source.at)); } source.at += 1; }
                b')' => { depth = depth.checked_sub(1).ok_or_else(|| refusal("step.syntax", "Unbalanced closing parenthesis", source.at))?; source.at += 1; }
                b';' if depth == 0 => { source.at += 1; self.at = source.at; return Ok(Some(&self.text[start..self.at])); }
                _ => { if source.prefix(b"/*", control)?.is_some() || source.prefix(b"\\N\\", control)?.is_some() || source.prefix(b"\\F\\", control)?.is_some() { source.trivia(control)?; } else { source.at += 1; } }
            }
        }
        self.at = source.at;
        if strip_trivia(&self.text[start..], control)?.is_empty() { Ok(None) } else { Err(refusal("step.syntax", "Missing STEP record terminator", start)) }
    }
}

fn strip_trivia<'a>(record: &'a str, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<&'a str, IfcStepError> {
    let mut source = SourceWalk::new(record, 0, 0, 0, record.len());
    source.trivia(control)?;
    Ok(&record[source.at..])
}
fn command(record: &str, expected: &str, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<bool, IfcStepError> {
    let mut source = SourceWalk::new(record, 0, 0, 0, record.len());
    source.trivia(control)?;
    if !source.consume(expected.as_bytes(), control)? { return Ok(false); }
    source.trivia(control)?;
    Ok(source.peek(control)?.is_none())
}
fn keyword(record: &str, expected: &str, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<bool, IfcStepError> {
    let mut source = SourceWalk::new(record, 0, 0, 0, record.len());
    source.trivia(control)?;
    if !source.consume(expected.as_bytes(), control)? { return Ok(false); }
    Ok(!source.peek(control)?.is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_'))
}

fn validate_header(header: &Part21Header, origins: &[usize; 3]) -> Result<(), IfcStepError> {
    let text_list = |values: &[Part21Value]| !values.is_empty() && values.iter().all(|value| matches!(value, Part21Value::Str(_)));
    if !matches!(header.file_description.as_slice(), [Part21Value::List(values), Part21Value::Str(_)] if text_list(values)) {
        return Err(refusal("step.header", "FILE_DESCRIPTION requires a nonempty string list and implementation level", origins[0]));
    }
    if !matches!(header.file_name.as_slice(), [Part21Value::Str(_), Part21Value::Str(_), Part21Value::List(authors), Part21Value::List(organizations), Part21Value::Str(_), Part21Value::Str(_), Part21Value::Str(_)] if text_list(authors) && text_list(organizations))
    {
        return Err(refusal("step.header", "FILE_NAME requires seven correctly typed attributes", origins[1]));
    }
    if !matches!(header.file_schema.as_slice(), [Part21Value::List(names)] if matches!(names.as_slice(), [Part21Value::Str(name)] if name == IFC4X3_STEP_SCHEMA)) {
        return Err(refusal("step.schema", "FILE_SCHEMA must contain exactly IFC4X3_ADD2", origins[2]));
    }
    Ok(())
}

#[derive(Default)]
struct ValueOrigin {
    offset: usize,
    children: Vec<ValueOrigin>,
}
struct InstanceOrigin {
    id: usize,
    components: Vec<(usize, Vec<ValueOrigin>)>,
}
#[derive(Default)]
struct SourceOrigins {
    headers: [usize; 3],
    instances: Vec<InstanceOrigin>,
}
struct SourceToken {
    kind: u8,
    offset: usize,
}
struct OriginCursor {
    tokens: Vec<SourceToken>,
    at: usize,
}

impl OriginCursor {
    fn new(record: &str, base: usize, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<Self, IfcStepError> {
        let mut source = SourceWalk::new(record, 0, base, 0, record.len());
        let mut tokens = Vec::new();
        loop {
            source.trivia(control)?;
            let Some(byte) = source.peek(control)? else { break; };
            let start = source.at;
            let kind = match byte {
                b'\'' | b'"' => { source.quoted(byte, control)?; byte }
                b'#' => { source.at += 1; while source.peek(control)?.is_some_and(|byte| byte.is_ascii_digit()) { source.at += 1; } b'#' }
                b'(' | b')' | b',' | b'=' | b';' | b'$' | b'*' => { source.at += 1; byte }
                _ => {
                    source.at += 1;
                    while source.peek(control)?.is_some_and(|byte| byte != b' ' && !b"(),=;\"'#$*".contains(&byte)) {
                        if source.prefix(b"/*", control)?.is_some() || source.prefix(b"\\N\\", control)?.is_some() || source.prefix(b"\\F\\", control)?.is_some() { break; }
                        source.at += 1;
                    }
                    b'w'
                }
            };
            tokens.push(SourceToken { kind, offset: base + start });
        }
        Ok(Self { tokens, at: 0 })
    }
    fn take(&mut self, kind: u8) -> Result<usize, IfcStepError> {
        let token = self.tokens.get(self.at).ok_or_else(|| refusal("step.origin", "Missing original source token", self.tokens.last().map_or(0, |token| token.offset)))?;
        if token.kind != kind {
            return Err(refusal("step.origin", "Shared grammar and source token spans differ", token.offset));
        }
        self.at += 1;
        Ok(token.offset)
    }
    fn items(&mut self, values: &[Part21Value]) -> Result<Vec<ValueOrigin>, IfcStepError> {
        self.take(b'(')?;
        let mut origins = Vec::with_capacity(values.len());
        for (index, value) in values.iter().enumerate() {
            if index > 0 {
                self.take(b',')?;
            }
            origins.push(self.value(value)?);
        }
        self.take(b')')?;
        Ok(origins)
    }
    fn value(&mut self, value: &Part21Value) -> Result<ValueOrigin, IfcStepError> {
        if let Part21Value::List(values) = value {
            let offset = self.tokens.get(self.at).ok_or_else(|| refusal("step.origin", "Missing aggregate source token", self.tokens.last().map_or(0, |token| token.offset)))?.offset;
            return Ok(ValueOrigin { offset, children: self.items(values)? });
        }
        let kind = match value {
            Part21Value::Ref(_) => b'#',
            Part21Value::Str(_) => b'\'',
            Part21Value::Binary(_) => b'"',
            Part21Value::Unset => b'$',
            Part21Value::Derived => b'*',
            _ => b'w',
        };
        let offset = self.take(kind)?;
        let children = if let Part21Value::Typed { items, .. } = value { self.items(items)? } else { vec![] };
        Ok(ValueOrigin { offset, children })
    }
    fn instance(&mut self, instance: &part21::Part21Instance) -> Result<InstanceOrigin, IfcStepError> {
        let id = self.take(b'#')?;
        self.take(b'=')?;
        let complex = self.tokens.get(self.at).is_some_and(|token| token.kind == b'(');
        if complex {
            self.take(b'(')?;
        }
        let mut components = Vec::with_capacity(instance.entities.len());
        for (_, args) in &instance.entities {
            components.push((self.take(b'w')?, self.items(args)?));
        }
        if complex {
            self.take(b')')?;
        }
        self.take(b';')?;
        Ok(InstanceOrigin { id, components })
    }
}

fn original_parse_offset(error: &part21::Part21Error, record: &str, base: usize, prefix_bytes: usize) -> usize {
    let at = match error {
        part21::Part21Error::UnexpectedEof { at, .. }
        | part21::Part21Error::UnexpectedChar { at, .. }
        | part21::Part21Error::UnsupportedEscape { at, .. }
        | part21::Part21Error::InvalidNumber { at, .. }
        | part21::Part21Error::MissingLiteral { at, .. } => *at,
    };
    base + at.saturating_sub(prefix_bytes).min(record.len())
}

fn validate_value(value: &IfcValue, ids: &BTreeSet<u64>, depth: usize, visited: &mut usize, origin: Option<&ValueOrigin>, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
    let offset = origin.map_or(0, |origin| origin.offset);
    if depth > MAX_DEPTH {
        return Err(refusal("step.depth", "Value nesting exceeds the source depth bound", offset));
    }
    *visited += 1;
    checkpoint(control, "values", *visited, 0)?;
    match value {
        IfcValue::Reference(id) if !ids.contains(id) => Err(refusal("step.reference", format!("Reference #{id} has no target"), offset)),
        IfcValue::Real(value) if !value.is_finite() => Err(refusal("step.real", "STEP real must be finite", offset)),
        IfcValue::Binary(value) if !part21::is_part21_binary(value) => Err(refusal("step.binary", "Invalid canonical STEP binary content", offset)),
        IfcValue::Aggregate(items) => {
            for (index, item) in items.iter().enumerate() {
                validate_value(item, ids, depth + 1, visited, origin.and_then(|origin| origin.children.get(index)), control)?;
            }
            Ok(())
        }
        IfcValue::TypedValue(typed) => {
            if !valid_name(&typed.name) {
                return Err(refusal("step.name", "Invalid typed value name", offset));
            }
            for (index, item) in typed.items.iter().enumerate() {
                validate_value(item, ids, depth + 1, visited, origin.and_then(|origin| origin.children.get(index)), control)?;
            }
            Ok(())
        }
        IfcValue::Enum(value) if !valid_name(value) => Err(refusal("step.enum", "Invalid enumeration literal", offset)),
        _ => Ok(()),
    }
}
fn valid_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_uppercase() || byte == b'_') && bytes.all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}
fn validate_graph(snapshot: &IfcSnapshot, origins: Option<&SourceOrigins>, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
    if snapshot.schema != snapshot::STDIO_IFC_DOCUMENT_SCHEMA {
        return Err(refusal("step.graph-schema", "Native IFC graph must identify stdio.ifc", 0));
    }
    let text_list = |values: &[IfcValue]| !values.is_empty() && values.iter().all(|value| matches!(value, IfcValue::String(_)));
    if !matches!(snapshot.header.file_description.as_slice(), [IfcValue::Aggregate(values), IfcValue::String(_)] if text_list(values)) {
        return Err(refusal("step.header", "FILE_DESCRIPTION requires a nonempty string list and implementation level", 0));
    }
    if !matches!(snapshot.header.file_name.as_slice(), [IfcValue::String(_), IfcValue::String(_), IfcValue::Aggregate(authors), IfcValue::Aggregate(organizations), IfcValue::String(_), IfcValue::String(_), IfcValue::String(_)] if text_list(authors) && text_list(organizations))
    {
        return Err(refusal("step.header", "FILE_NAME requires seven correctly typed attributes", 0));
    }
    if !matches!(snapshot.header.file_schema.as_slice(), [IfcValue::Aggregate(names)] if matches!(names.as_slice(), [IfcValue::String(name)] if name == IFC4X3_STEP_SCHEMA)) {
        return Err(refusal("step.schema", "FILE_SCHEMA must contain exactly IFC4X3_ADD2", 0));
    }
    let mut ids = BTreeSet::new();
    for (index, entity) in snapshot.entities.iter().enumerate() {
        let offset = origins.and_then(|origins| origins.instances.get(index)).map_or(0, |origin| origin.id);
        checkpoint(control, "identities", ids.len(), snapshot.entities.len())?;
        if entity.id == 0 {
            return Err(refusal("step.id", "Instance identifiers must be positive", offset));
        }
        if !ids.insert(entity.id) {
            return Err(refusal("step.duplicate-id", "Instance identifiers must be unique", offset));
        }
    }
    let mut visited = 0;
    for (index, entity) in snapshot.entities.iter().enumerate() {
        checkpoint(control, "references", index, snapshot.entities.len())?;
        let components = std::iter::once((&entity.name, &entity.args)).chain(entity.complex.iter().map(|component| (&component.name, &component.args)));
        for (component_index, (name, args)) in components.enumerate() {
            let origin = origins.and_then(|origins| origins.instances.get(index)).and_then(|origin| origin.components.get(component_index));
            if !valid_name(name) {
                return Err(refusal("step.name", "Invalid IFC entity name", origin.map_or(0, |origin| origin.0)));
            }
            for (value_index, value) in args.iter().enumerate() {
                validate_value(value, &ids, 0, &mut visited, origin.and_then(|origin| origin.1.get(value_index)), control)?;
            }
        }
    }
    Ok(())
}

/// 🧩 Validate the owned ADD2 source graph without formatting or allocating a STEP document.
pub fn validate_ifc4x3_graph_controlled(snapshot: &IfcSnapshot, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
    checkpoint(control, "begin", 0, snapshot.entities.len())?;
    validate_graph(snapshot, None, control)
}

/// 📥 Parse checked IFC4X3_ADD2 source using bounded records and the shared STEP grammar.
pub fn parse_ifc4x3_step_controlled(text: &str, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<IfcSnapshot, IfcStepError> {
    if text.len() > MAX_SOURCE_BYTES {
        return Err(refusal("step.source-limit", "IFC source exceeds its admitted byte bound", 0));
    }
    checkpoint(control, "begin", 0, text.len())?;
    let mut records = Records { text, at: 0, max_record_bytes: MAX_RECORD_BYTES };
    for expected in ["ISO-10303-21;", "HEADER;"] {
        let record = records.next(control)?.ok_or_else(|| refusal("step.syntax", "Missing exchange header", records.at))?;
        if !command(record, expected, control)? {
            return Err(refusal("step.syntax", format!("Expected {expected}"), records.at));
        }
    }
    let mut header_source = String::from("ISO-10303-21;HEADER;");
    let mut origins = SourceOrigins::default();
    let mut header_segments = Vec::new();
    for (index, expected) in ["FILE_DESCRIPTION", "FILE_NAME", "FILE_SCHEMA"].iter().enumerate() {
        let record = records.next(control)?.ok_or_else(|| refusal("step.header", "Missing mandatory HEADER record", records.at))?;
        if !keyword(record, expected, control)? {
            return Err(refusal("step.header", format!("Expected exactly one {expected} in mandatory order"), records.at));
        }
        let original_start = records.at - record.len();
        origins.headers[index] = original_start + record.len() - strip_trivia(record, control)?.len();
        header_segments.push((header_source.len(), original_start, record.len()));
        header_source.push_str(record);
    }
    for expected in ["ENDSEC;", "DATA;"] {
        let record = records.next(control)?.ok_or_else(|| refusal("step.syntax", "Missing DATA section", records.at))?;
        if !command(record, expected, control)? {
            return Err(refusal("step.header", format!("Expected {expected}"), records.at));
        }
    }
    header_source.push_str("ENDSEC;DATA;ENDSEC;END-ISO-10303-21;");
    let document = part21::parse_part21(&header_source).map_err(|error| {
        let synthetic = original_parse_offset(&error, &header_source, 0, 0);
        let original = header_segments.iter().find(|(start, _, length)| synthetic >= *start && synthetic < start + length).map_or(origins.headers[0], |(start, original, _)| original + synthetic - start);
        refusal("step.header", error.to_string(), original)
    })?;
    validate_header(&document.header, &origins.headers)?;
    let mut snapshot = snapshot::from_part21_document(snapshot::STDIO_IFC_DOCUMENT_SCHEMA, &document);
    loop {
        let record = records.next(control)?.ok_or_else(|| refusal("step.syntax", "Missing DATA ENDSEC", records.at))?;
        if command(record, "ENDSEC;", control)? {
            break;
        }
        if !strip_trivia(record, control)?.starts_with('#') {
            return Err(refusal("step.syntax", "DATA requires an instance record", records.at));
        }
        let isolated = format!("{EMPTY_HEADER}{record}{END}");
        let record_start = records.at - record.len();
        let parsed = part21::parse_part21(&isolated).map_err(|error| refusal("step.syntax", error.to_string(), original_parse_offset(&error, record, record_start, EMPTY_HEADER.len())))?;
        if parsed.instances.len() != 1 || parsed.instances[0].entities.is_empty() {
            return Err(refusal("step.syntax", "Expected one complete STEP instance", records.at));
        }
        let origin = OriginCursor::new(record, record_start, control)?.instance(&parsed.instances[0])?;
        for instance in &parsed.instances {
            for (index, (_, args)) in instance.entities.iter().enumerate() {
                let component = origin.components.get(index).ok_or_else(|| refusal("step.origin", "Missing original component source", record_start))?;
                validate_decimal_values(args, &component.1, 0)?;
            }
        }
        origins.instances.push(origin);
        snapshot.entities.extend(snapshot::from_part21_document(snapshot::STDIO_IFC_DOCUMENT_SCHEMA, &parsed).entities);
        checkpoint(control, "parse", records.at, text.len())?;
    }
    let terminator = records.next(control)?.ok_or_else(|| refusal("step.syntax", "Missing exchange terminator", records.at))?;
    if !command(terminator, "END-ISO-10303-21;", control)? || records.next(control)?.is_some() {
        return Err(refusal("step.syntax", "Invalid or trailing exchange terminator", records.at));
    }
    validate_graph(&snapshot, Some(&origins), control)?;
    checkpoint(control, "complete", text.len(), text.len())?;
    Ok(snapshot)
}

fn validate_decimal_values(values: &[Part21Value], origins: &[ValueOrigin], depth: usize) -> Result<(), IfcStepError> {
    if depth > MAX_DEPTH {
        return Err(refusal("step.depth", "Value nesting exceeds the source depth bound", origins.first().map_or(0, |origin| origin.offset)));
    }
    for (index, value) in values.iter().enumerate() {
        let origin = origins.get(index).ok_or_else(|| refusal("step.origin", "Missing original value source", origins.last().map_or(0, |origin| origin.offset)))?;
        match value {
            Part21Value::Real(value) if !value.to_f64().is_some_and(f64::is_finite) => return Err(refusal("step.real", "STEP real exceeds finite binary64", origin.offset)),
            Part21Value::List(items) | Part21Value::Typed { items, .. } => validate_decimal_values(items, &origin.children, depth + 1)?,
            _ => {}
        }
    }
    Ok(())
}

/// 📥 Import a complete checked source graph without semantic EXPRESS conformance claims.
pub fn parse_ifc4x3_step(text: &str) -> Result<IfcSnapshot, IfcStepError> {
    parse_ifc4x3_step_controlled(text, &mut |_| true)
}

/// 📤 Regenerate source through the shared writer, checking cancellation between graph records.
pub fn write_ifc4x3_step_controlled(snapshot: &IfcSnapshot, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<String, IfcStepError> {
    write_with_limits(snapshot, control, MAX_SOURCE_BYTES, MAX_RECORD_BYTES)
}

fn source_limits(text: &str, maximum_bytes: usize, maximum_record_bytes: usize, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
    if text.len() > maximum_bytes {
        return Err(refusal("step.source-limit", "Serialized IFC exceeds the complete source byte bound", maximum_bytes));
    }
    let mut records = Records { text, at: 0, max_record_bytes: maximum_record_bytes };
    while records.next(control)?.is_some() {}
    Ok(())
}

fn write_with_limits(snapshot: &IfcSnapshot, control: &mut dyn FnMut(IfcStepProgress) -> bool, maximum_bytes: usize, maximum_record_bytes: usize) -> Result<String, IfcStepError> {
    checkpoint(control, "begin", 0, snapshot.entities.len())?;
    validate_graph(snapshot, None, control)?;
    let mut cursor = emit::IfcStepEmitCursor::new();
    let mut output = String::new();
    let mut record_bytes = 0usize;
    let mut quote = 0u8;
    let mut pending_quote = false;
    while let Some(fragment) = cursor.next(snapshot).map_err(|error| refusal("step.writer", error.to_string(), output.len()))? {
        if output.len().saturating_add(fragment.as_bytes().len()) > maximum_bytes { return Err(refusal("step.source-limit", "Serialized IFC exceeds its source byte bound", output.len())); }
        for &byte in fragment.as_bytes() {
            record_bytes = record_bytes.checked_add(1).ok_or_else(|| refusal("step.record-limit", "STEP record byte extent overflow", output.len()))?;
            if record_bytes > maximum_record_bytes { return Err(refusal("step.record-limit", "Serialized STEP record exceeds its source unit", output.len())); }
            if quote == b'\'' && pending_quote { pending_quote = false; if byte == b'\'' { continue; } quote = 0; }
            if quote == b'\'' { if byte == b'\'' { pending_quote = true; } continue; }
            if quote == b'"' { if byte == b'"' { quote = 0; } continue; }
            match byte { b'\'' | b'"' => quote = byte, b';' => record_bytes = 0, _ => {} }
        }
        checkpoint(control, "write-text", output.len(), 0)?;
        output.push_str(std::str::from_utf8(fragment.as_bytes()).map_err(|_| refusal("step.writer", "STEP emitter did not produce ASCII text", output.len()))?);
    }
    checkpoint(control, "complete", snapshot.entities.len(), snapshot.entities.len())?;
    Ok(output)
}

/// 📤 Export natural IFC source independently of the native graph pack representation.
pub fn write_ifc4x3_step(snapshot: &IfcSnapshot) -> Result<String, IfcStepError> {
    write_ifc4x3_step_controlled(snapshot, &mut |_| true)
}

/// 🔌 Connect source parsing checkpoints to the original framework IO cancellation authority.
pub fn parse_ifc4x3_step_io(text: &str, control: &mut semio_framework_os_kernel::io::control::IoRunControl<'_, '_>) -> Result<IfcSnapshot, IfcStepError> {
    parse_ifc4x3_step_controlled(text, &mut |_| control.checkpoint().is_ok())
}

/// 🔌 Connect source writing checkpoints to the original framework IO cancellation authority.
pub fn write_ifc4x3_step_io(snapshot: &IfcSnapshot, control: &mut semio_framework_os_kernel::io::control::IoRunControl<'_, '_>) -> Result<String, IfcStepError> {
    write_ifc4x3_step_controlled(snapshot, &mut |_| control.checkpoint().is_ok())
}

fn compose_error(error: impl fmt::Display) -> ComposeError {
    ComposeError { message: error.to_string(), diagnostics: vec![] }
}
fn pack_error(error: impl fmt::Display) -> semio_framework_os_kernel::PackError {
    semio_framework_os_kernel::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
}

/// 🎹 Compose one source, refusing ambiguous multi-source replacement or foreign schema bytes.
pub struct Ifc4x3Composer;
impl ArtifactComposition for Ifc4x3Composer {
    type Snapshot = IfcSnapshot;
    const WRITES: Dialect = IFC4X3_DIALECT;
    fn reads() -> &'static [Dialect] {
        &[IFC4X3_DIALECT, TXT_DIALECT]
    }
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let [source] = sources else {
            return Err(compose_error("IFC4x3 composition requires exactly one source"));
        };
        let snapshot = match (&source.dialect, &source.payload) {
            (dialect, AnalyzeSource::Text(text)) if *dialect == TXT_DIALECT || *dialect == IFC4X3_DIALECT => parse_ifc4x3_step(text).map_err(compose_error)?,
            (dialect, AnalyzeSource::Binary(bytes)) if *dialect == IFC4X3_DIALECT => {
                IfcSnapshot::decode_pack(bytes).or_else(|_| std::str::from_utf8(bytes).map_err(pack_error).and_then(|text| parse_ifc4x3_step(text).map_err(pack_error))).map_err(compose_error)?
            }
            (dialect, AnalyzeSource::Binary(bytes)) if *dialect == TXT_DIALECT => {
                let text = TxtSnapshot::decode_pack(bytes).map_err(compose_error)?.to_body();
                parse_ifc4x3_step(&text).map_err(compose_error)?
            }
            _ => return Err(compose_error("IFC4x3 composer does not read the supplied dialect")),
        };
        validate_graph(&snapshot, None, &mut |_| true).map_err(compose_error)?;
        Ok(Composition { snapshot, confidence: Confidence::High, diagnostics: vec![] })
    }
}
impl ArtifactComposer for Ifc4x3Composer {
    type Snapshot = IfcSnapshot;
    const WRITES: Dialect = IFC4X3_DIALECT;
    fn reads() -> &'static [Dialect] {
        <Self as ArtifactComposition>::reads()
    }
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        <Self as ArtifactComposition>::compose(sources)
    }
}

/// 📥 Typed UTF-8 artifact import into the shared native IFC graph.
pub struct Ifc4x3FromTxt;
impl ArtifactDeserializer for Ifc4x3FromTxt {
    type From = TxtSnapshot;
    type Into = IfcSnapshot;
    const FROM: Dialect = TXT_DIALECT;
    const INTO: Dialect = IFC4X3_DIALECT;
    async fn deserialize(from: &Self::From) -> Result<Self::Into, semio_framework_os_kernel::PackError> {
        parse_ifc4x3_step(&from.to_body()).map_err(pack_error)
    }
}

/// 📤 Typed graph export to natural UTF-8 IFC source.
pub struct Ifc4x3ToTxt;
impl ArtifactSerializer for Ifc4x3ToTxt {
    type From = IfcSnapshot;
    type Into = TxtSnapshot;
    const FROM: Dialect = IFC4X3_DIALECT;
    const INTO: Dialect = TXT_DIALECT;
    async fn serialize(from: &Self::From) -> Result<Self::Into, semio_framework_os_kernel::PackError> {
        Ok(TxtSnapshot::from_body(&write_ifc4x3_step(from).map_err(pack_error)?))
    }
}

/// 📇 Actual import and export entries consumed by the artifact declaration and registry.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::composer_entry_of::<Ifc4x3Composer>(), semio_framework_plugin::serializer_entry_of::<Ifc4x3ToTxt>()]).as_slice()
}

#[cfg(test)]
#[path = "🧪️tests/native.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/native_step_emit.rs"]
mod emit_tests;

#[cfg(test)]
#[path = "🧪️tests/native_step_write.rs"]
mod write_tests;
