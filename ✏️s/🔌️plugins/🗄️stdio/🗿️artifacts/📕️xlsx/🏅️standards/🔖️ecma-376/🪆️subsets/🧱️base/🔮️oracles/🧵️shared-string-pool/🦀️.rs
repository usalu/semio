//! 🧵️ Shared strings of the XLSX oracle: the shared string pool codec, its projection and its inverse spec. See [`super`].

use super::*;

//#region 🔖️SharedStringPool
/// 📑️ The raw `xl/sharedStrings.xml` pool, read and written through the `zip` + `quick-xml` pairing
/// this owner already registers and the six OOXML conformance subsets already run on
/// (`semio_s_plugin_stdio_document_test_oracle::ooxml`), NOT through `calamine`/`rust_xlsxwriter`.
///
/// 🪞️ This replaces a documented gap rather than papering over one. `calamine`'s `Xlsx<RS>::strings`
/// and `read_shared_strings` are both private, and `rust_xlsxwriter` populates its own table only as
/// a byproduct of `write_string`, so neither can address the pool BY INDEX — which is exactly what
/// this catalog's three shared-string kinds do. The conclusion drawn from that was that no second
/// producer existed and the three kinds had to return their input unchanged. That conclusion was
/// wrong: the pool is a part of an OPC package, and a second producer for a PART is the container
/// codec plus an XML reader/writer, which this owner has linked all along. `calamine` and
/// `rust_xlsxwriter` remain the reference pairing for the GRID; the pool is a storage-layer part and
/// gets the reference pairing a part deserves.
///
/// 📐️ ECMA-376 §18.4: `sst` holds `si` items, each a string item whose text is its `t` runs
/// concatenated (a rich-text `si` splits its text across `r`/`t` children). `count` is the number of
/// cell references into the pool and `uniqueCount` the number of items; a rewrite here can only know
/// the second, so it writes `uniqueCount` and leaves `count` equal to it — an ECMA-376-legal
/// declaration, and the same one every writer that rebuilds a pool from scratch emits.
pub mod shared_strings {
    use semio_s_plugin_stdio_document_test_oracle::ooxml::{part_bytes, read_parts, set_part, write_parts};
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use quick_xml::XmlVersion;

    pub const PART: &str = "xl/sharedStrings.xml";
    const SST_NAMESPACE: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";

    /// 👁️ Every `si` item of the real pool, in document order, each one's `t` runs concatenated.
    pub fn read_pool(input: &[u8]) -> Result<Vec<String>, String> {
        let parts = read_parts(input)?;
        let Some(bytes) = part_bytes(&parts, PART) else { return Ok(Vec::new()) };
        let text = std::str::from_utf8(bytes).map_err(|error| format!("{PART} is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut pool: Vec<String> = Vec::new();
        let mut in_item = false;
        let mut in_text = false;
        let mut current = String::new();
        loop {
            match reader.read_event().map_err(|error| format!("independent XML reader could not read {PART}: {error}"))? {
                Event::Start(start) => match local_name(start.name().as_ref()) {
                    "si" => {
                        in_item = true;
                        current.clear();
                    }
                    "t" if in_item => in_text = true,
                    _ => {}
                },
                Event::Text(run) if in_text => current.push_str(&run.xml10_content()),
                Event::GeneralRef(reference) if in_text => current.push_str(&resolve_reference(&reference.xml10_content())?),
                Event::End(end) => match local_name(end.name().as_ref()) {
                    "t" => in_text = false,
                    "si" if in_item => {
                        in_item = false;
                        pool.push(std::mem::take(&mut current));
                    }
                    _ => {}
                },
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(pool)
    }

    /// ✍️ Rewrites `xl/sharedStrings.xml` from `pool` alone and reassembles the whole container from
    /// its parts — never a patch of the input bytes. Every other part is carried through verbatim.
    pub fn write_pool(input: &[u8], pool: &[String]) -> Result<Vec<u8>, String> {
        let mut parts = read_parts(input)?;
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>");
        xml.push_str(&format!("<sst xmlns=\"{SST_NAMESPACE}\" count=\"{}\" uniqueCount=\"{}\">", pool.len(), pool.len()));
        for item in pool {
            xml.push_str("<si><t xml:space=\"preserve\">");
            xml.push_str(&escape(item));
            xml.push_str("</t></si>");
        }
        xml.push_str("</sst>");
        set_part(&mut parts, PART, xml.into_bytes());
        write_parts(&parts)
    }

    /// 🔗️ Whether any worksheet cell of the package references pool entry `index` (`<c t="s"><v>index</v></c>`, ECMA-376
    /// §18.3.1.4). Removing such an entry would leave the cell's reference dangling, which this vocabulary refuses.
    pub fn is_referenced(input: &[u8], index: usize) -> Result<bool, String> {
        let parts = read_parts(input)?;
        for (path, bytes) in parts.iter().filter(|(path, _)| path.starts_with("xl/worksheets/") && path.ends_with(".xml")) {
            let text = std::str::from_utf8(bytes).map_err(|error| format!("{path} is not valid utf-8: {error}"))?;
            let mut reader = Reader::from_str(text);
            let (mut shared_cell, mut in_value, mut value) = (false, false, String::new());
            loop {
                match reader.read_event().map_err(|error| format!("independent XML reader could not read {path}: {error}"))? {
                    Event::Start(start) => match local_name(start.name().as_ref()) {
                        "c" => shared_cell = start.attributes().flatten().any(|attribute| attribute.key.as_ref() == "t" && attribute.normalized_value(XmlVersion::Explicit1_0).is_ok_and(|kind| kind == "s")),
                        "v" if shared_cell => {
                            in_value = true;
                            value.clear();
                        }
                        _ => {}
                    },
                    Event::Text(run) if in_value => value.push_str(&run.xml10_content()),
                    Event::End(end) => match local_name(end.name().as_ref()) {
                        "v" if in_value => {
                            in_value = false;
                            if value.trim().parse::<usize>().ok() == Some(index) {
                                return Ok(true);
                            }
                        }
                        "c" => shared_cell = false,
                        _ => {}
                    },
                    Event::Eof => break,
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    /// 🔣️ The five predefined XML entities plus numeric character references — every general
    /// reference ECMA-376 §18.4 string content may legally carry without a DTD.
    fn resolve_reference(name: &str) -> Result<String, String> {
        Ok(match name {
            "amp" => "&".to_string(),
            "lt" => "<".to_string(),
            "gt" => ">".to_string(),
            "apos" => "'".to_string(),
            "quot" => "\"".to_string(),
            other => {
                let code = match other.strip_prefix("#x").or_else(|| other.strip_prefix("#X")) {
                    Some(hex) => u32::from_str_radix(hex, 16).map_err(|error| format!("{PART} carries an unreadable character reference &{other};: {error}"))?,
                    None => other.strip_prefix('#').ok_or_else(|| format!("{PART} carries an undeclared entity reference &{other};"))?.parse().map_err(|error| format!("{PART} carries an unreadable character reference &{other};: {error}"))?,
                };
                char::from_u32(code).ok_or_else(|| format!("{PART} carries character reference &{other}; which is not a Unicode scalar"))?.to_string()
            }
        })
    }

    fn local_name(name: &str) -> &str {
        match name.find(':') {
            Some(index) => &name[index + 1..],
            None => name,
        }
    }

    fn escape(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        for character in text.chars() {
            match character {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                other => out.push(other),
            }
        }
        out
    }
}

/// 👁️ Projects the raw shared-string pool the three index-addressed kinds actually move — read out
/// of the BYTES by the independent `zip` + `quick-xml` implementation, not tracked by the caller.
/// Deliberately separate from [`project_xlsx_workbook`]: the pool is a STORAGE-layer optimisation
/// (ECMA-376 §18.4 — a workbook whose cells carry inline strings instead is semantically the same
/// workbook), and the grid kinds run through `rust_xlsxwriter`, which legitimately renormalises the
/// pool while preserving every cell value. Holding a grid mutation to a pool it never claimed to
/// preserve would report a false divergence; holding a pool mutation to the pool is the real test.
pub fn project_shared_string_pool(input: &[u8]) -> Result<Json, String> {
    let pool = shared_strings::read_pool(input)?;
    Ok(Json::Object(vec![
        ("sharedStringCount".to_string(), Json::Number(pool.len() as f64)),
        ("sharedStrings".to_string(), Json::Array(pool.into_iter().map(Json::String).collect())),
    ]))
}

/// ↩️ The undo of one of the three pool kinds, read out of `base` by the independent implementation
/// alone.
///
/// ⚠️ `remove-shared-string` at an INTERIOR index has no inverse in this vocabulary, and that is a
/// property of the vocabulary rather than of this oracle: `XlsxMutation::InsertSharedString` carries
/// only a `value` and appends at `shared_strings.len()`
/// (`../🧬️schema/🧬️mutations/🦀️.rs:145`), so nothing in the catalog can put a string back at
/// position 7 of 229. Only removing the LAST entry is invertible, which is why this case's Examples
/// row addresses index 228 of the real 229-entry pool. Reported rather than worked around; the
/// production `inverse()` at that file's line 173 answers `SetSharedString`, which restores neither
/// the length nor the element that shifted into the hole.
pub fn shared_string_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let pool = shared_strings::read_pool(base)?;
    let params = mutation_params(forward);
    let index = match params.get("index") {
        Some(Json::Number(value)) => value.max(0.0) as usize,
        _ => 0,
    };
    let object = |pairs: Vec<(&str, Json)>| Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let spec = |kind: &str, pairs: Vec<(&str, Json)>| object(vec![("kind", Json::String(kind.to_string())), ("params", object(pairs))]);
    Ok(match forward.str("kind").as_str() {
        "insert-shared-string" => spec("remove-shared-string", vec![("index", Json::Number(pool.len() as f64))]),
        "remove-shared-string" => {
            let value = pool.get(index).ok_or_else(|| format!("remove-shared-string has no inverse: index {index} is outside the base's {}-entry pool", pool.len()))?;
            if index + 1 != pool.len() {
                return Err(format!(
                    "remove-shared-string at index {index} of {} has no inverse in this vocabulary: insert-shared-string carries only a value and appends, so no declared kind can put a string back at an interior position",
                    pool.len()
                ));
            }
            spec("insert-shared-string", vec![("value", Json::String(value.clone()))])
        }
        "set-shared-string" => {
            let value = pool.get(index).ok_or_else(|| format!("set-shared-string has no inverse: index {index} is outside the base's {}-entry pool", pool.len()))?;
            spec("set-shared-string", vec![("index", Json::Number(index as f64)), ("value", Json::String(value.clone()))])
        }
        other => return Err(format!("{other:?} is not one of this catalog's shared-string kinds")),
    })
}
//#endregion 🔖️SharedStringPool
