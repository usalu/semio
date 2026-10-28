//! 🪪️ Controlled header declarations retain comments and exact released identifiers.
use super::IfcMvdView;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcValue;
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{IfcMutation, set_file_description::SetFileDescription};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfcMvdDeclaration {
    pub index: usize,
    pub views: Vec<String>,
    pub known: Vec<IfcMvdView>,
    pub malformed: bool,
}

/// 🏷️ Format the exact ADD2 scope identifier without a version substitution.
pub fn ifc4x3_mvd_header_text(view: IfcMvdView) -> &'static str {
    match view {
        IfcMvdView::Reference => "ViewDefinition [ReferenceView]",
        IfcMvdView::Alignment => "ViewDefinition [Alignment-basedReferenceView]",
    }
}

fn whitespace(byte: u8) -> bool {
    byte == b' ' || (b'\t'..=b'\r').contains(&byte)
}

fn owned_text(text: &str, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<String, String> {
    let mut result = String::new();
    for (index, character) in text.chars().enumerate() {
        if index % 128 == 0 {
            tick()?;
        }
        result.push(character);
    }
    Ok(result)
}

fn declaration(text: &str, index: usize, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Option<IfcMvdDeclaration>, String> {
    tick()?;
    let bytes = text.as_bytes();
    let mut start = 0;
    let mut end = bytes.len();
    while start < end && whitespace(bytes[start]) {
        tick()?;
        start += 1;
    }
    if !bytes[start..].starts_with(b"ViewDefinition") {
        return Ok(None);
    }
    start += 14;
    if start < end && !whitespace(bytes[start]) && bytes[start] != b'[' {
        return Ok(None);
    }
    let malformed = || Some(IfcMvdDeclaration { index, views: Vec::new(), known: Vec::new(), malformed: true });
    while start < end && whitespace(bytes[start]) {
        tick()?;
        start += 1;
    }
    while end > start && whitespace(bytes[end - 1]) {
        tick()?;
        end -= 1;
    }
    if start >= end || bytes[start] != b'[' || bytes[end - 1] != b']' {
        return Ok(malformed());
    }
    start += 1;
    let mut from = start;
    let mut views = Vec::new();
    let mut known = Vec::new();
    for cursor in start..end {
        if (cursor - start) % 256 == 0 {
            tick()?;
        }
        let character = bytes[cursor];
        if character == b'[' || character == b']' && cursor != end - 1 {
            return Ok(malformed());
        }
        if character == b',' || cursor == end - 1 {
            let mut left = from;
            let mut right = cursor;
            while left < right && whitespace(bytes[left]) {
                tick()?;
                left += 1;
            }
            while right > left && whitespace(bytes[right - 1]) {
                tick()?;
                right -= 1;
            }
            if left == right {
                return Ok(malformed());
            }
            let value = &text[left..right];
            match value {
                "ReferenceView" => known.push(IfcMvdView::Reference),
                "Alignment-basedReferenceView" => known.push(IfcMvdView::Alignment),
                _ => {}
            }
            views.push(owned_text(value, tick)?);
            from = cursor + 1;
        }
    }
    Ok(Some(IfcMvdDeclaration { index, views, known, malformed: false }))
}

fn description(values: &[IfcValue]) -> Result<(&[IfcValue], &str), String> {
    let [IfcValue::Aggregate(items), IfcValue::String(level)] = values else {
        return Err("invalidMvdHeader: FILE_DESCRIPTION requires a string list and implementation level".into());
    };
    if items.is_empty() {
        return Err("invalidMvdHeader: description list must be nonempty".into());
    }
    Ok((items, level))
}

/// 🔍️ Read imported names verbatim without evaluating their population conformance.
pub fn ifc4x3_mvd_declarations(values: &[IfcValue], tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Vec<IfcMvdDeclaration>, String> {
    let (items, _) = description(values)?;
    let mut result = Vec::new();
    for (index, item) in items.iter().enumerate() {
        tick()?;
        let IfcValue::String(text) = item else { return Err("invalidMvdHeader: description entries require strings".into()) };
        if let Some(value) = declaration(text, index, tick)? {
            result.push(value);
        }
    }
    tick()?;
    Ok(result)
}

/// 📝️ Produce the ordinary mutation for central diff application and inverse history.
pub fn ifc4x3_mvd_header_mutation(values: &[IfcValue], selection: Option<IfcMvdView>, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<IfcMutation, String> {
    let (items, level) = description(values)?;
    let declarations = ifc4x3_mvd_declarations(values, tick)?;
    for entry in &declarations {
        tick()?;
        if entry.malformed {
            return Err(format!("malformedMvdDeclaration: repair description entry {} before selection", entry.index));
        }
    }
    let mut result = Vec::new();
    let mut cursor = 0;
    let mut inserted = false;
    if declarations.is_empty() {
        if let Some(view) = selection {
            result.push(IfcValue::String(ifc4x3_mvd_header_text(view).into()));
            inserted = true;
        }
    }
    for (index, item) in items.iter().enumerate() {
        tick()?;
        if declarations.get(cursor).is_some_and(|value| value.index == index) {
            cursor += 1;
            if !inserted {
                if let Some(view) = selection {
                    result.push(IfcValue::String(ifc4x3_mvd_header_text(view).into()));
                    inserted = true;
                }
            }
        } else {
            let IfcValue::String(text) = item else { return Err("invalidMvdHeader: description entries require strings".into()) };
            result.push(IfcValue::String(owned_text(text, tick)?));
        }
    }
    if result.is_empty() {
        result.push(IfcValue::String(String::new()));
    }
    let level = owned_text(level, tick)?;
    tick()?;
    Ok(IfcMutation::SetFileDescription(SetFileDescription { values: vec![IfcValue::Aggregate(result), IfcValue::String(level)] }))
}

#[cfg(test)]
#[path = "header-tests.rs"]
mod tests;
