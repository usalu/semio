//! 📦️ Container and XML edits of the OOXML engine: the independent zip read and write and the root element edits. See [`super`].

use super::*;

//#region 🔖️Container
/// 📦️ Independent container read: every zip entry in its stored order, verbatim.
pub fn read_parts(input: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(input.to_vec())).map_err(|error| format!("independent OPC reader could not open the package: {error}"))?;
    let mut parts = Vec::new();
    for index in 0..archive.len() {
        let mut member = archive.by_index(index).map_err(|error| format!("independent OPC reader could not read entry {index}: {error}"))?;
        if member.is_dir() {
            continue;
        }
        let name = member.name().to_string();
        let mut bytes = Vec::new();
        member.read_to_end(&mut bytes).map_err(|error| format!("independent OPC reader could not inflate entry {name:?}: {error}"))?;
        parts.push((name, bytes));
    }
    Ok(parts)
}

/// 📦️ Independent container write: a brand-new deflate archive assembled from `parts`, never a
/// patch of the input bytes — rebuilding the whole container is what makes this a second
/// producer rather than a byte editor.
pub fn write_parts(parts: &[(String, Vec<u8>)]) -> Result<Vec<u8>, String> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = zip::ZipWriter::new(&mut cursor);
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for (name, bytes) in parts {
            writer.start_file(name.clone(), options).map_err(|error| format!("independent OPC writer could not start entry {name:?}: {error}"))?;
            writer.write_all(bytes).map_err(|error| format!("independent OPC writer could not write entry {name:?}: {error}"))?;
        }
        writer.finish().map_err(|error| format!("independent OPC writer could not finish the package: {error}"))?;
    }
    Ok(cursor.into_inner())
}

pub fn part_bytes<'a>(parts: &'a [(String, Vec<u8>)], path: &str) -> Option<&'a [u8]> {
    parts.iter().find(|(name, _)| name == path).map(|(_, bytes)| bytes.as_slice())
}

pub fn set_part(parts: &mut Vec<(String, Vec<u8>)>, path: &str, bytes: Vec<u8>) {
    match parts.iter_mut().find(|(name, _)| name == path) {
        Some(existing) => existing.1 = bytes,
        None => parts.push((path.to_string(), bytes)),
    }
}

pub fn remove_part(parts: &mut Vec<(String, Vec<u8>)>, path: &str) -> bool {
    let before = parts.len();
    parts.retain(|(name, _)| name != path);
    parts.len() != before
}

/// 📰️ Whether an entry is XML this engine may parse. Media parts (`.png`, `.jpeg`, …) and the
/// `.bin` OLE payloads a real deck carries are left byte-for-byte alone.
pub fn is_xml_part(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with(".xml") || lower.ends_with(".rels")
}

pub(super) fn contains(haystack: &[u8], needle: &str) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|window| window == needle.as_bytes())
}
//#endregion 🔖️Container

//#region 🔖️XmlEdits
pub(super) fn attributes_of(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
    start
        .attributes()
        .map(|attribute| {
            let attribute = attribute.map_err(|error| error.to_string())?;
            let value = attribute.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
            Ok((attribute.key.as_ref().to_string(), value.to_string()))
        })
        .collect()
}

pub(super) fn element_name(start: &BytesStart) -> Result<String, String> {
    Ok(start.name().as_ref().to_string())
}

fn rebuilt(name: &str, attrs: &[(String, String)]) -> BytesStart<'static> {
    let mut rebuilt = BytesStart::new(name.to_string());
    for (key, value) in attrs {
        rebuilt.push_attribute((key.as_str(), value.as_str()));
    }
    rebuilt
}

/// ✍️ Streams `part` through `quick-xml`, offering every element's attribute list to `edit`
/// (`(name, depth, attrs) -> changed`), and re-emits the whole document with `quick-xml`'s own
/// writer. Declaration, doctype, comments, processing instructions, text, entity references and
/// CDATA are written back unchanged.
fn rewrite_elements(part: &[u8], mut edit: impl FnMut(&str, usize, &mut Vec<(String, String)>) -> bool) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
    let mut reader = Reader::from_str(text);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut depth = 0usize;
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::Eof => break,
            Event::Start(start) => {
                let name = element_name(&start)?;
                let mut attrs = attributes_of(&start)?;
                let changed = edit(&name, depth, &mut attrs);
                depth += 1;
                let emitted = if changed { rebuilt(&name, &attrs) } else { start.into_owned() };
                writer.write_event(Event::Start(emitted)).map_err(|error| error.to_string())?;
            }
            Event::Empty(start) => {
                let name = element_name(&start)?;
                let mut attrs = attributes_of(&start)?;
                let changed = edit(&name, depth, &mut attrs);
                let emitted = if changed { rebuilt(&name, &attrs) } else { start.into_owned() };
                writer.write_event(Event::Empty(emitted)).map_err(|error| error.to_string())?;
            }
            Event::End(end) => {
                depth = depth.saturating_sub(1);
                writer.write_event(Event::End(end)).map_err(|error| error.to_string())?;
            }
            other => writer.write_event(other).map_err(|error| error.to_string())?,
        }
    }
    Ok(writer.into_inner().into_inner())
}

/// 🔎️ The root element's `(name, attributes)`, read independently.
pub fn root_element(part: &[u8]) -> Result<(String, Vec<(String, String)>), String> {
    let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
    let mut reader = Reader::from_str(text);
    loop {
        match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
            Event::Eof => return Err("part has no root element".to_string()),
            Event::Start(start) | Event::Empty(start) => return Ok((element_name(&start)?, attributes_of(&start)?)),
            _ => {}
        }
    }
}

pub fn root_attribute(part: &[u8], name: &str) -> Result<Option<String>, String> {
    Ok(root_element(part)?.1.into_iter().find(|(key, _)| key == name).map(|(_, value)| value))
}

/// ✍️ Sets — or, with `None`, removes — one attribute on the ROOT element only.
pub fn set_root_attribute(part: &[u8], name: &str, value: Option<&str>) -> Result<Vec<u8>, String> {
    let mut seen_root = false;
    rewrite_elements(part, |_, depth, attrs| {
        if depth != 0 || seen_root {
            return false;
        }
        seen_root = true;
        match (attrs.iter().position(|(key, _)| key == name), value) {
            (Some(index), Some(value)) => attrs[index].1 = value.to_string(),
            (Some(index), None) => {
                attrs.remove(index);
            }
            (None, Some(value)) => attrs.push((name.to_string(), value.to_string())),
            (None, None) => return false,
        }
        true
    })
}

/// ✍️ Replaces every attribute value that exactly equals one of `from` with `to`, on every
/// element of the part. A namespace declaration is an ordinary attribute at the XML level, which
/// is why one primitive covers `xmlns`, `xmlns:w`, `xmlns:a` and whatever prefixed alias a real
/// package happens to use.
pub fn replace_attribute_values(part: &[u8], from: &[&str], to: &str) -> Result<Vec<u8>, String> {
    rewrite_elements(part, |_, _, attrs| {
        let mut changed = false;
        for (_, value) in attrs.iter_mut() {
            if from.contains(&value.as_str()) && value != to {
                *value = to.to_string();
                changed = true;
            }
        }
        changed
    })
}

/// ✍️ Replaces the leading `from` prefix of every attribute value that starts with it by `to` —
/// the relationship-base swap, where `…/officeDocument/2006/relationships/officeDocument` becomes
/// `…/ooxml/officeDocument/relationships/officeDocument` without touching the package
/// relationship types (`…/package/2006/relationships/…`) both conformance classes share.
pub fn replace_attribute_value_prefixes(part: &[u8], from: &[&str], to: &str) -> Result<Vec<u8>, String> {
    rewrite_elements(part, |_, _, attrs| {
        let mut changed = false;
        for (_, value) in attrs.iter_mut() {
            for prefix in from {
                if value.starts_with(prefix) && *prefix != to {
                    *value = format!("{to}{}", &value[prefix.len()..]);
                    changed = true;
                    break;
                }
            }
        }
        changed
    })
}

/// ➕️ Appends `markup` as the last child of the part's ROOT element. The fragment is parsed by
/// `quick-xml` and forwarded event by event, never spliced as text, so malformed markup fails
/// loudly instead of producing a package that only looks edited.
pub fn append_root_child(part: &[u8], markup: &str) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
    let mut reader = Reader::from_str(text);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut depth = 0usize;
    let mut inserted = false;
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::Eof => break,
            Event::Start(start) => {
                depth += 1;
                writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
            }
            Event::End(end) => {
                if depth == 1 && !inserted {
                    let mut fragment = Reader::from_str(markup);
                    loop {
                        match fragment.read_event().map_err(|error| format!("quick-xml rejected the fragment at byte {}: {error}", fragment.error_position()))? {
                            Event::Eof => break,
                            event => writer.write_event(event).map_err(|error| error.to_string())?,
                        }
                    }
                    inserted = true;
                }
                depth = depth.saturating_sub(1);
                writer.write_event(Event::End(end)).map_err(|error| error.to_string())?;
            }
            other => writer.write_event(other).map_err(|error| error.to_string())?,
        }
    }
    if !inserted {
        return Err("part has no root element to append a child to".to_string());
    }
    Ok(writer.into_inner().into_inner())
}

/// ➖️ Drops every direct child of the ROOT element named `name`, subtree and all.
pub fn remove_root_children(part: &[u8], name: &str) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
    let mut reader = Reader::from_str(text);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut depth = 0usize;
    let mut skipping = 0usize;
    let mut removed = false;
    loop {
        let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
        match event {
            Event::Eof => break,
            Event::Start(start) => {
                let matched = depth == 1 && element_name(&start)? == name;
                depth += 1;
                if skipping > 0 || matched {
                    skipping += 1;
                    removed = true;
                    continue;
                }
                writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
            }
            Event::Empty(start) => {
                if skipping > 0 {
                    continue;
                }
                if depth == 1 && element_name(&start)? == name {
                    removed = true;
                    continue;
                }
                writer.write_event(Event::Empty(start)).map_err(|error| error.to_string())?;
            }
            Event::End(end) => {
                depth = depth.saturating_sub(1);
                if skipping > 0 {
                    skipping -= 1;
                    continue;
                }
                writer.write_event(Event::End(end)).map_err(|error| error.to_string())?;
            }
            other => {
                if skipping > 0 {
                    continue;
                }
                writer.write_event(other).map_err(|error| error.to_string())?;
            }
        }
    }
    if !removed {
        return Err(format!("part declares no root child named {name:?} to remove"));
    }
    Ok(writer.into_inner().into_inner())
}
//#endregion 🔖️XmlEdits
