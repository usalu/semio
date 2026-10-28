//! 🧵 STEP syntax emission retains source positions and emits at most 64 ASCII bytes per turn.
use super::{snapshot::IfcSnapshot, IfcValue, MAX_DEPTH};
use semio_framework_value::{RetireOwned, ValueError, ValueRefusalKind};
use std::fmt::{self, Write};

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }

pub(super) struct Fragment { bytes: [u8; 64], len: usize, reference: Option<u64> }
impl Fragment {
    fn new() -> Self { Self { bytes: [0; 64], len: 0, reference: None } }
    fn literal(text: &str) -> Result<Self, ValueError> { let mut value = Self::new(); value.write_str(text).map_err(|_| invalid("STEP fragment exceeds its inline bound"))?; Ok(value) }
    pub fn as_bytes(&self) -> &[u8] { &self.bytes[..self.len] }
    pub fn reference(&self) -> Option<u64> { self.reference }
}
impl Write for Fragment {
    fn write_str(&mut self, text: &str) -> fmt::Result { let end = self.len.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(fmt::Error)?; self.bytes[self.len..end].copy_from_slice(text.as_bytes()); self.len = end; Ok(()) }
}

#[derive(Clone, Copy, Default, RetireOwned)]
struct Frame { next: usize, stage: u8 }
#[derive(Clone, Copy, RetireOwned)]
struct Text { kind: u8, finish: u8, stage: u8, at: usize }

#[derive(RetireOwned)]
pub(super) struct IfcStepEmitCursor {
    document: u8,
    entity: usize,
    component: usize,
    root: u8,
    active: bool,
    depth: usize,
    frames: [Frame; MAX_DEPTH + 2],
    text: Option<Text>,
}

impl IfcStepEmitCursor {
    pub fn new() -> Self { Self { document: 0, entity: 0, component: 0, root: 0, active: false, depth: 0, frames: [Frame::default(); MAX_DEPTH + 2], text: None } }

    fn root<'a>(&self, source: &'a IfcSnapshot) -> Result<&'a [IfcValue], ValueError> {
        match self.root {
            0 => Ok(&source.header.file_description), 1 => Ok(&source.header.file_name), 2 => Ok(&source.header.file_schema),
            3 => { let entity = source.entities.get(self.entity).ok_or_else(|| invalid("STEP source entity is absent"))?; if self.component == 0 { Ok(&entity.args) } else { entity.complex.get(self.component - 1).map(|component| component.args.as_slice()).ok_or_else(|| invalid("STEP source component is absent")) } }
            _ => Err(invalid("STEP source root is invalid")),
        }
    }

    fn values<'a>(&self, source: &'a IfcSnapshot, depth: usize) -> Result<&'a [IfcValue], ValueError> {
        let mut values = self.root(source)?;
        for frame in &self.frames[..depth] { values = match values.get(frame.next) { Some(IfcValue::Aggregate(values)) => values, Some(IfcValue::TypedValue(value)) => &value.items, _ => return Err(invalid("STEP source path lost its aggregate")) }; }
        Ok(values)
    }

    fn value<'a>(&self, source: &'a IfcSnapshot, depth: usize) -> Result<&'a IfcValue, ValueError> { self.values(source, depth)?.get(self.frames[depth].next).ok_or_else(|| invalid("STEP source value is absent")) }

    fn name<'a>(&self, source: &'a IfcSnapshot) -> Result<&'a str, ValueError> {
        let entity = source.entities.get(self.entity).ok_or_else(|| invalid("STEP source entity is absent"))?;
        if self.component == 0 { Ok(&entity.name) } else { entity.complex.get(self.component - 1).map(|value| value.name.as_str()).ok_or_else(|| invalid("STEP source component is absent")) }
    }

    fn text_source<'a>(&self, source: &'a IfcSnapshot, text: Text) -> Result<&'a str, ValueError> {
        if text.finish == 0 { return self.name(source); }
        if text.finish == 2 { return match self.value(source, self.depth.checked_sub(1).ok_or_else(|| invalid("STEP typed value has no parent"))?)? { IfcValue::TypedValue(value) => Ok(&value.name), _ => Err(invalid("STEP typed source name is absent")) }; }
        match self.value(source, self.depth)? { IfcValue::String(value) | IfcValue::Binary(value) | IfcValue::Enum(value) => Ok(value), _ => Err(invalid("STEP source text is absent")) }
    }

    fn finish_text(&mut self, finish: u8) { self.text = None; if finish == 1 { self.frames[self.depth].next += 1; self.frames[self.depth].stage = 1; } else if finish == 2 { self.frames[self.depth].stage = 0; } }

    fn text_fragment(&mut self, source: &IfcSnapshot) -> Result<Fragment, ValueError> {
        let mut text = self.text.ok_or_else(|| invalid("STEP text cursor is absent"))?;
        let value = self.text_source(source, text)?;
        if text.stage == 0 {
            if matches!(text.kind, 0 | 3) && value.is_empty() { return Err(invalid("STEP identifier is empty")); }
            if text.kind == 2 {
                let bytes = value.as_bytes();
                if bytes.is_empty() || !matches!(bytes[0], b'0'..=b'3') || bytes.len() == 1 && bytes[0] != b'0' || bytes.len() > 1 && bytes[1].to_digit_value().is_none_or(|digit| digit >= 1 << (4 - (bytes[0] - b'0'))) { return Err(invalid("STEP binary unused high bits are invalid")); }
            }
            text.stage = 1; self.text = Some(text);
            if text.kind != 0 { return Fragment::literal(match text.kind { 1 => "'", 2 => "\"", 3 => ".", _ => return Err(invalid("STEP text kind is invalid")) }); }
        }
        let mut fragment = Fragment::new();
        while text.at < value.len() && fragment.len <= 48 {
            let character = value[text.at..].chars().next().ok_or_else(|| invalid("STEP source text lost UTF8"))?;
            match text.kind {
                0 | 3 => { let byte = character as u32; if byte > 127 || !(character.is_ascii_uppercase() || character == '_' || text.at > 0 && character.is_ascii_digit()) { return Err(invalid("STEP identifier is not canonical uppercase ASCII")); } fragment.write_char(character).map_err(|_| invalid("STEP inline name fragment overflow"))?; }
                2 => { if !character.is_ascii_digit() && !('A'..='F').contains(&character) { return Err(invalid("STEP binary is not canonical uppercase hexadecimal")); } fragment.write_char(character).map_err(|_| invalid("STEP inline binary fragment overflow"))?; }
                1 => { match character { '\'' => fragment.write_str("''"), '\\' => fragment.write_str("\\\\"), ' '..='~' => fragment.write_char(character), value if value as u32 <= 0xFFFF => write!(&mut fragment, "\\X2\\{:04X}\\X0\\", value as u32), value => write!(&mut fragment, "\\X4\\{:08X}\\X0\\", value as u32) }.map_err(|_| invalid("STEP inline string fragment overflow"))?; }
                _ => return Err(invalid("STEP text kind is invalid")),
            }
            text.at += character.len_utf8();
        }
        if text.at == value.len() {
            if text.kind == 0 { self.finish_text(text.finish); }
            else if fragment.len <= 63 { fragment.write_str(match text.kind { 1 => "'", 2 => "\"", 3 => ".", _ => unreachable!() }).map_err(|_| invalid("STEP inline closing delimiter overflow"))?; self.finish_text(text.finish); }
            else { self.text = Some(text); }
        } else { self.text = Some(text); }
        Ok(fragment)
    }

    fn begin(&mut self, root: u8) { self.root = root; self.depth = 0; self.frames[0] = Frame::default(); self.active = true; }
    fn scalar_done(&mut self) { self.frames[self.depth].next += 1; self.frames[self.depth].stage = 1; }

    fn sequence(&mut self, source: &IfcSnapshot) -> Result<Fragment, ValueError> {
        loop {
            if self.frames[self.depth].stage == 3 { self.text = Some(Text { kind: 0, finish: 2, stage: 0, at: 0 }); return self.text_fragment(source); }
            if self.frames[self.depth].stage == 0 { self.frames[self.depth].stage = 2; return Fragment::literal("("); }
            let values = self.values(source, self.depth)?;
            if self.frames[self.depth].next == values.len() {
                if self.depth == 0 { self.active = false; } else { self.depth -= 1; self.scalar_done(); }
                return Fragment::literal(")");
            }
            if self.frames[self.depth].stage == 1 { self.frames[self.depth].stage = 2; return Fragment::literal(","); }
            if self.depth > MAX_DEPTH { return Err(invalid("STEP value nesting exceeds its admitted bound")); }
            let value = self.value(source, self.depth)?;
            let mut fragment = Fragment::new();
            match value {
                IfcValue::Unset => fragment.write_str("$"), IfcValue::Derived => fragment.write_str("*"),
                IfcValue::Integer(value) => write!(&mut fragment, "{value}"),
                IfcValue::Reference(value) if *value > 0 => { fragment.reference = Some(*value); write!(&mut fragment, "#{value}") },
                IfcValue::Reference(_) => return Err(invalid("STEP reference identifiers must be positive")),
                IfcValue::Real(value) => { if !value.is_finite() { return Err(invalid("STEP real must be finite")); } write!(&mut fragment, "{value:?}").map_err(|_| invalid("STEP real formatting exceeds inline bound"))?; let exponent = fragment.as_bytes().iter().position(|byte| matches!(byte, b'e' | b'E')); let mantissa = exponent.unwrap_or(fragment.len); if !fragment.as_bytes()[..mantissa].contains(&b'.') { fragment.bytes.copy_within(mantissa..fragment.len, mantissa + 1); fragment.bytes[mantissa] = b'.'; fragment.len += 1; } for byte in &mut fragment.bytes[..fragment.len] { if *byte == b'e' { *byte = b'E'; } } Ok(()) }
                IfcValue::String(_) | IfcValue::Binary(_) | IfcValue::Enum(_) => { let kind = match value { IfcValue::String(_) => 1, IfcValue::Binary(_) => 2, _ => 3 }; self.text = Some(Text { kind, finish: 1, stage: 0, at: 0 }); return self.text_fragment(source); }
                IfcValue::Aggregate(_) | IfcValue::TypedValue(_) => { let typed = matches!(value, IfcValue::TypedValue(_)); self.depth += 1; self.frames[self.depth] = Frame { stage: if typed { 3 } else { 0 }, ..Default::default() }; continue; }
            }.map_err(|_| invalid("STEP scalar formatting exceeds inline bound"))?;
            self.scalar_done(); return Ok(fragment);
        }
    }

    pub fn next(&mut self, source: &IfcSnapshot) -> Result<Option<Fragment>, ValueError> {
        loop {
            if self.text.is_some() { let fragment = self.text_fragment(source)?; if !fragment.as_bytes().is_empty() { return Ok(Some(fragment)); } continue; }
            if self.active { return self.sequence(source).map(Some); }
            let fragment = match self.document {
                0 => { self.document = 1; Fragment::literal("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION")? }
                1 => { self.begin(0); self.document = 2; continue; }
                2 => { self.document = 3; Fragment::literal(";\nFILE_NAME")? }
                3 => { self.begin(1); self.document = 4; continue; }
                4 => { self.document = 5; Fragment::literal(";\nFILE_SCHEMA")? }
                5 => { self.begin(2); self.document = 6; continue; }
                6 => { self.document = 7; Fragment::literal(";\nENDSEC;\nDATA;\n")? }
                7 => { if self.entity == source.entities.len() { self.document = 14; continue; } self.component = 0; self.document = 8; let id = source.entities[self.entity].id; if id == 0 { return Err(invalid("STEP instance identifiers must be positive")); } let mut fragment = Fragment::new(); write!(&mut fragment, "#{id}=").map_err(|_| invalid("STEP instance prefix exceeds inline bound"))?; fragment }
                8 => { self.document = 9; if source.entities[self.entity].complex.is_empty() { continue; } Fragment::literal("(")? }
                9 => { self.document = 10; self.text = Some(Text { kind: 0, finish: 0, stage: 0, at: 0 }); continue; }
                10 => { self.begin(3); self.document = 11; continue; }
                11 => { if self.component < source.entities[self.entity].complex.len() { self.component += 1; self.document = 9; Fragment::literal(" ")? } else { self.document = 12; continue; } }
                12 => { self.document = 13; if source.entities[self.entity].complex.is_empty() { continue; } Fragment::literal(")")? }
                13 => { self.document = 7; self.entity += 1; Fragment::literal(";\n")? }
                14 => { self.document = 15; Fragment::literal("ENDSEC;\nEND-ISO-10303-21;\n")? }
                15 => return Ok(None),
                _ => return Err(invalid("STEP document cursor is invalid")),
            };
            return Ok(Some(fragment));
        }
    }
}

trait HexDigit { fn to_digit_value(self) -> Option<u8>; }
impl HexDigit for u8 { fn to_digit_value(self) -> Option<u8> { match self { b'0'..=b'9' => Some(self - b'0'), b'A'..=b'F' => Some(self - b'A' + 10), _ => None } } }
