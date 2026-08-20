//! 🚪️ IO stdio.ply (1.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::ply::standards::v1_0::subsets::any::schema::PlyAnalyzer;
    use crate::artifacts::ply::PlySnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct PlyComposerComposition;

    impl ArtifactComposition for PlyComposerComposition {
        type Snapshot = PlySnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "PlyComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PlyAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PlyComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Codec
// Real ply codec. Decode supports the real §PLY header grammar: arbitrary `element <name>
// <count>` / `property <type> <name>` / `property list <count-type> <value-type> <name>`
// declarations, walked fully generically (never hardcoded to `x y z` + `vertex_indices`),
// across all three `format` variants (`ascii`, `binary_little_endian`, `binary_big_endian`),
// retaining every declared property's real type and every row's real typed cell values. Encode
// walks the same generic element/property/row model back out in whichever wire format is
// requested — round-tripping any element/property layout, not just vertex/face meshes.
use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType, PlyValue};
use crate::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

//#region 🔖️ScalarWire
/// 📏 Byte width of a PLY scalar type.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn scalar_type_size(kind: PlyScalarType) -> usize {
    match kind {
        PlyScalarType::Char | PlyScalarType::UChar => 1,
        PlyScalarType::Short | PlyScalarType::UShort => 2,
        PlyScalarType::Int | PlyScalarType::UInt | PlyScalarType::Float => 4,
        PlyScalarType::Double => 8,
    }
}

/// 🔤 Parses both long (`float`) and short (`float32`) PLY type spellings.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_scalar_type(ty: &str) -> Result<PlyScalarType, String> {
    match ty {
        "char" | "int8" => Ok(PlyScalarType::Char),
        "uchar" | "uint8" => Ok(PlyScalarType::UChar),
        "short" | "int16" => Ok(PlyScalarType::Short),
        "ushort" | "uint16" => Ok(PlyScalarType::UShort),
        "int" | "int32" => Ok(PlyScalarType::Int),
        "uint" | "uint32" => Ok(PlyScalarType::UInt),
        "float" | "float32" => Ok(PlyScalarType::Float),
        "double" | "float64" => Ok(PlyScalarType::Double),
        other => Err(format!("ply: unsupported property type '{other}'")),
    }
}

/// 🏷️ Canonical (long-form) wire spelling used on encode.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn scalar_type_wire_name(kind: PlyScalarType) -> &'static str {
    match kind {
        PlyScalarType::Char => "char",
        PlyScalarType::UChar => "uchar",
        PlyScalarType::Short => "short",
        PlyScalarType::UShort => "ushort",
        PlyScalarType::Int => "int",
        PlyScalarType::UInt => "uint",
        PlyScalarType::Float => "float",
        PlyScalarType::Double => "double",
    }
}

/// 🔢 Builds a `PlyValue` of the given scalar `kind` holding `n` (used to write a list's
/// element count in its declared `count_kind` width).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_as_value(kind: PlyScalarType, n: usize) -> PlyValue {
    match kind {
        PlyScalarType::Char => PlyValue::Char(n as i8),
        PlyScalarType::UChar => PlyValue::UChar(n as u8),
        PlyScalarType::Short => PlyValue::Short(n as i16),
        PlyScalarType::UShort => PlyValue::UShort(n as u16),
        PlyScalarType::Int => PlyValue::Int(n as i32),
        PlyScalarType::UInt => PlyValue::UInt(n as u32),
        PlyScalarType::Float => PlyValue::Float(n as f32),
        PlyScalarType::Double => PlyValue::Double(n as f64),
    }
}

/// 🔢 Reads a scalar-typed value back out as an integer (for a decoded list-count cell).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_as_usize(v: &PlyValue) -> usize {
    (match v {
        PlyValue::Char(x) => *x as i64,
        PlyValue::UChar(x) => *x as i64,
        PlyValue::Short(x) => *x as i64,
        PlyValue::UShort(x) => *x as i64,
        PlyValue::Int(x) => *x as i64,
        PlyValue::UInt(x) => *x as i64,
        PlyValue::Float(x) => *x as i64,
        PlyValue::Double(x) => *x as i64,
        PlyValue::List(_) => 0,
    })
    .max(0) as usize
}
//#endregion 🔖️ScalarWire

//#region 🔖️HeaderParse
/// ✂️ Splits raw bytes into `(header_text, body)` at the line following `end_header`. The
/// header itself is always ASCII text per spec, even for binary-format files.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_header(data: &[u8]) -> Result<(String, &[u8]), String> {
    let marker = b"end_header";
    let idx = data.windows(marker.len()).position(|w| w == marker).ok_or("ply: missing end_header")?;
    let after_marker = idx + marker.len();
    let mut nl = after_marker;
    while nl < data.len() && data[nl] != b'\n' {
        nl += 1;
    }
    if nl >= data.len() {
        return Err("ply: truncated header".into());
    }
    let body_start = nl + 1;
    let header_text = std::str::from_utf8(&data[0..body_start]).map_err(|e| format!("ply: header not utf8: {e}"))?;
    Ok((header_text.to_string(), &data[body_start..]))
}

/// 📖 A fully parsed PLY header: wire format, in-order comments, and every
/// `element`/`property` declaration (rows filled in separately by body decode).
struct PlyHeader {
    format: PlyFormat,
    comments: Vec<String>,
    elements: Vec<PlyElement>,
}

/// 🧩 Parses the `ply` / `format` / `comment` / `element` / `property` header grammar.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_header_text(text: &str) -> Result<PlyHeader, String> {
    let mut lines = text.lines();
    let first = lines.next().ok_or("ply: empty header")?.trim();
    if first != "ply" {
        return Err("ply: expected 'ply' magic line".into());
    }
    let mut format: Option<PlyFormat> = None;
    let mut comments: Vec<String> = Vec::new();
    let mut elements: Vec<PlyElement> = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line == "end_header" {
            continue;
        }
        if let Some(rest) = line.strip_prefix("comment") {
            // 💬 `comment` may be followed by a space and text, or stand bare.
            comments.push(rest.strip_prefix(' ').unwrap_or(rest).to_string());
            continue;
        }
        if line.starts_with("obj_info") {
            continue;
        } // 🕳️ not modeled (documented deviation).
        if let Some(rest) = line.strip_prefix("format ") {
            let mut parts = rest.split_whitespace();
            let kind = parts.next().ok_or("ply: missing format kind")?;
            format = Some(match kind {
                "ascii" => PlyFormat::Ascii,
                "binary_little_endian" => PlyFormat::BinaryLittleEndian,
                "binary_big_endian" => PlyFormat::BinaryBigEndian,
                other => return Err(format!("ply: unsupported format '{other}'")),
            });
        } else if let Some(rest) = line.strip_prefix("element ") {
            let mut parts = rest.split_whitespace();
            let name = parts.next().ok_or("ply: element missing name")?.to_string();
            let count: usize = parts.next().ok_or("ply: element missing count")?.parse().map_err(|e| format!("ply: bad element count: {e}"))?;
            elements.push(PlyElement { name, count, properties: Vec::new(), rows: Vec::new() });
        } else if let Some(rest) = line.strip_prefix("property ") {
            let el = elements.last_mut().ok_or("ply: property declared before any element")?;
            let mut parts = rest.split_whitespace();
            let first_tok = parts.next().ok_or("ply: empty property declaration")?;
            if first_tok == "list" {
                let count_kind = parse_scalar_type(parts.next().ok_or("ply: list property missing count type")?)?;
                let value_kind = parse_scalar_type(parts.next().ok_or("ply: list property missing value type")?)?;
                let name = parts.next().ok_or("ply: list property missing name")?.to_string();
                el.properties.push(PlyProperty::List { name, count_kind, value_kind });
            } else {
                let kind = parse_scalar_type(first_tok)?;
                let name = parts.next().ok_or("ply: property missing name")?.to_string();
                el.properties.push(PlyProperty::Scalar { name, kind });
            }
        }
    }
    let format = format.ok_or("ply: missing format line")?;
    Ok(PlyHeader { format, comments, elements })
}
//#endregion 🔖️HeaderParse

//#region 🔖️BinaryScalarIo
/// 📥 Reads one scalar of `kind` at `data[*pos..]`, advancing `*pos` by its width.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_scalar_bin(kind: PlyScalarType, data: &[u8], pos: &mut usize, big: bool) -> Result<PlyValue, String> {
    let size = scalar_type_size(kind);
    if *pos + size > data.len() {
        return Err("ply: truncated binary body".into());
    }
    let b = &data[*pos..*pos + size];
    let v = match kind {
        PlyScalarType::Char => PlyValue::Char(b[0] as i8),
        PlyScalarType::UChar => PlyValue::UChar(b[0]),
        PlyScalarType::Short => PlyValue::Short(if big { i16::from_be_bytes([b[0], b[1]]) } else { i16::from_le_bytes([b[0], b[1]]) }),
        PlyScalarType::UShort => PlyValue::UShort(if big { u16::from_be_bytes([b[0], b[1]]) } else { u16::from_le_bytes([b[0], b[1]]) }),
        PlyScalarType::Int => PlyValue::Int(if big { i32::from_be_bytes(b.try_into().unwrap()) } else { i32::from_le_bytes(b.try_into().unwrap()) }),
        PlyScalarType::UInt => PlyValue::UInt(if big { u32::from_be_bytes(b.try_into().unwrap()) } else { u32::from_le_bytes(b.try_into().unwrap()) }),
        PlyScalarType::Float => PlyValue::Float(if big { f32::from_be_bytes(b.try_into().unwrap()) } else { f32::from_le_bytes(b.try_into().unwrap()) }),
        PlyScalarType::Double => PlyValue::Double(if big { f64::from_be_bytes(b.try_into().unwrap()) } else { f64::from_le_bytes(b.try_into().unwrap()) }),
    };
    *pos += size;
    Ok(v)
}

/// 📤 Writes one scalar value in the requested endianness.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_scalar_bin(out: &mut Vec<u8>, v: &PlyValue, big: bool) {
    match v {
        PlyValue::Char(x) => out.push(*x as u8),
        PlyValue::UChar(x) => out.push(*x),
        PlyValue::Short(x) => out.extend_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() }),
        PlyValue::UShort(x) => out.extend_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() }),
        PlyValue::Int(x) => out.extend_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() }),
        PlyValue::UInt(x) => out.extend_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() }),
        PlyValue::Float(x) => out.extend_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() }),
        PlyValue::Double(x) => out.extend_from_slice(&if big { x.to_be_bytes() } else { x.to_le_bytes() }),
        PlyValue::List(_) => {} // 🕳️ nested lists never appear as a top-level scalar write.
    }
}
//#endregion 🔖️BinaryScalarIo

//#region 🔖️AsciiScalarIo
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_scalar_ascii(kind: PlyScalarType, tok: &str) -> Result<PlyValue, String> {
    let bad = |e: std::num::ParseIntError| format!("ply: bad scalar value '{tok}': {e}");
    let bad_f = |e: std::num::ParseFloatError| format!("ply: bad scalar value '{tok}': {e}");
    Ok(match kind {
        PlyScalarType::Char => PlyValue::Char(tok.parse().map_err(bad)?),
        PlyScalarType::UChar => PlyValue::UChar(tok.parse().map_err(bad)?),
        PlyScalarType::Short => PlyValue::Short(tok.parse().map_err(bad)?),
        PlyScalarType::UShort => PlyValue::UShort(tok.parse().map_err(bad)?),
        PlyScalarType::Int => PlyValue::Int(tok.parse().map_err(bad)?),
        PlyScalarType::UInt => PlyValue::UInt(tok.parse().map_err(bad)?),
        PlyScalarType::Float => PlyValue::Float(tok.parse().map_err(bad_f)?),
        PlyScalarType::Double => PlyValue::Double(tok.parse().map_err(bad_f)?),
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn format_scalar_ascii(v: &PlyValue) -> String {
    match v {
        PlyValue::Char(x) => x.to_string(),
        PlyValue::UChar(x) => x.to_string(),
        PlyValue::Short(x) => x.to_string(),
        PlyValue::UShort(x) => x.to_string(),
        PlyValue::Int(x) => x.to_string(),
        PlyValue::UInt(x) => x.to_string(),
        PlyValue::Float(x) => x.to_string(),
        PlyValue::Double(x) => x.to_string(),
        PlyValue::List(_) => String::new(),
    }
}
//#endregion 🔖️AsciiScalarIo

//#region 🔖️BodyDecode
/// 📚 Decodes an `ascii`-format body against the parsed header's element/property declarations,
/// producing fully typed rows for EVERY element (not just `vertex`/`face`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_body_ascii(body: &str, header_elements: &[PlyElement]) -> Result<Vec<PlyElement>, String> {
    let mut tokens = body.split_whitespace();
    let mut out = Vec::with_capacity(header_elements.len());
    for el in header_elements {
        let mut rows = Vec::with_capacity(el.count);
        for _ in 0..el.count {
            let mut values = Vec::with_capacity(el.properties.len());
            for prop in &el.properties {
                match prop {
                    PlyProperty::Scalar { kind, .. } => {
                        let tok = tokens.next().ok_or("ply: unexpected eof in ascii body")?;
                        values.push(parse_scalar_ascii(*kind, tok)?);
                    }
                    PlyProperty::List { count_kind, value_kind, .. } => {
                        let n_tok = tokens.next().ok_or("ply: unexpected eof reading list count")?;
                        let n = value_as_usize(&parse_scalar_ascii(*count_kind, n_tok)?);
                        let mut items = Vec::with_capacity(n);
                        for _ in 0..n {
                            let vt = tokens.next().ok_or("ply: unexpected eof reading list value")?;
                            items.push(parse_scalar_ascii(*value_kind, vt)?);
                        }
                        values.push(PlyValue::List(items));
                    }
                }
            }
            rows.push(PlyRow { values });
        }
        out.push(PlyElement { name: el.name.clone(), count: el.count, properties: el.properties.clone(), rows });
    }
    Ok(out)
}

/// 📚 Binary counterpart of `decode_body_ascii` — same element/property walk, reading each
/// declared scalar/list at its real byte width (endianness-aware) instead of tokenizing text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_body_binary(body: &[u8], header_elements: &[PlyElement], big: bool) -> Result<Vec<PlyElement>, String> {
    let mut pos = 0usize;
    let mut out = Vec::with_capacity(header_elements.len());
    for el in header_elements {
        let mut rows = Vec::with_capacity(el.count);
        for _ in 0..el.count {
            let mut values = Vec::with_capacity(el.properties.len());
            for prop in &el.properties {
                match prop {
                    PlyProperty::Scalar { kind, .. } => values.push(read_scalar_bin(*kind, body, &mut pos, big)?),
                    PlyProperty::List { count_kind, value_kind, .. } => {
                        let n = value_as_usize(&read_scalar_bin(*count_kind, body, &mut pos, big)?);
                        let mut items = Vec::with_capacity(n);
                        for _ in 0..n {
                            items.push(read_scalar_bin(*value_kind, body, &mut pos, big)?);
                        }
                        values.push(PlyValue::List(items));
                    }
                }
            }
            rows.push(PlyRow { values });
        }
        out.push(PlyElement { name: el.name.clone(), count: el.count, properties: el.properties.clone(), rows });
    }
    Ok(out)
}
//#endregion 🔖️BodyDecode

/// 🏗️ Builds the header text for `format`, walking every element's real name/count and every
/// property's real declaration — generic, not canonicalized to a fixed vertex/face layout.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_text(format: PlyFormat, comments: &[String], elements: &[PlyElement]) -> String {
    let fmt_line = match format {
        PlyFormat::Ascii => "format ascii 1.0\n",
        PlyFormat::BinaryLittleEndian => "format binary_little_endian 1.0\n",
        PlyFormat::BinaryBigEndian => "format binary_big_endian 1.0\n",
    };
    let mut out = String::new();
    out.push_str("ply\n");
    out.push_str(fmt_line);
    for c in comments {
        out.push_str(&format!("comment {c}\n"));
    }
    for el in elements {
        out.push_str(&format!("element {} {}\n", el.name, el.count));
        for prop in &el.properties {
            match prop {
                PlyProperty::Scalar { name, kind } => out.push_str(&format!("property {} {}\n", scalar_type_wire_name(*kind), name)),
                PlyProperty::List { name, count_kind, value_kind } => {
                    out.push_str(&format!("property list {} {} {}\n", scalar_type_wire_name(*count_kind), scalar_type_wire_name(*value_kind), name));
                }
            }
        }
    }
    out.push_str("end_header\n");
    out
}

/// 🏗️ Encodes `snap` in the given wire `format` (ascii / binary LE / binary BE). Comments ARE
/// re-emitted into the header on encode (as real `comment <text>\n` lines, matching
/// `parse_header_text`'s decode side).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_ply_with_format(snap: &PlySnapshot, format: PlyFormat) -> Result<Vec<u8>, String> {
    let mut out = header_text(format, &snap.comments, &snap.elements).into_bytes();
    match format {
        PlyFormat::Ascii => {
            for el in &snap.elements {
                for row in &el.rows {
                    let mut parts: Vec<String> = Vec::with_capacity(el.properties.len());
                    for (i, prop) in el.properties.iter().enumerate() {
                        let v = row.values.get(i).ok_or("ply: row missing value for declared property")?;
                        match prop {
                            PlyProperty::Scalar { .. } => parts.push(format_scalar_ascii(v)),
                            PlyProperty::List { .. } => match v {
                                PlyValue::List(items) => {
                                    parts.push(items.len().to_string());
                                    parts.extend(items.iter().map(format_scalar_ascii));
                                }
                                _ => return Err("ply: list property row value is not a list".into()),
                            },
                        }
                    }
                    out.extend_from_slice(parts.join(" ").as_bytes());
                    out.push(b'\n');
                }
            }
        }
        PlyFormat::BinaryLittleEndian | PlyFormat::BinaryBigEndian => {
            let big = format == PlyFormat::BinaryBigEndian;
            for el in &snap.elements {
                for row in &el.rows {
                    for (i, prop) in el.properties.iter().enumerate() {
                        let v = row.values.get(i).ok_or("ply: row missing value for declared property")?;
                        match prop {
                            PlyProperty::Scalar { .. } => push_scalar_bin(&mut out, v, big),
                            PlyProperty::List { count_kind, .. } => match v {
                                PlyValue::List(items) => {
                                    push_scalar_bin(&mut out, &count_as_value(*count_kind, items.len()), big);
                                    for item in items {
                                        push_scalar_bin(&mut out, item, big);
                                    }
                                }
                                _ => return Err("ply: list property row value is not a list".into()),
                            },
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}

/// 🏗️ Canonical encode — ascii wire format, matches the DSL/pack default.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_ply(snap: &PlySnapshot) -> Result<Vec<u8>, String> {
    encode_ply_with_format(snap, PlyFormat::Ascii)
}

/// 🔍 Decodes any of the three wire formats, dispatching on the header's own `format` line.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_ply(data: &[u8]) -> Result<PlySnapshot, String> {
    let (header_str, body) = split_header(data)?;
    let header = parse_header_text(&header_str)?;
    let elements = match header.format {
        PlyFormat::Ascii => {
            let body_text = std::str::from_utf8(body).map_err(|e| format!("ply: ascii body not utf8: {e}"))?;
            decode_body_ascii(body_text, &header.elements)?
        }
        PlyFormat::BinaryLittleEndian => decode_body_binary(body, &header.elements, false)?,
        PlyFormat::BinaryBigEndian => decode_body_binary(body, &header.elements, true)?,
    };
    Ok(PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: header.format, comments: header.comments, elements })
}
//#endregion 🔖️Codec

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::ply::standards::v1_0::subsets::any::schema::PlyComposer as PlyRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PlyRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::ply::schema::diff::PlyElementsDiff;
    use crate::artifacts::ply::schema::mutations::apply_ply_mutation;
    use crate::artifacts::ply::schema::{demo_ply_snapshot, empty_ply_snapshot};
    use crate::artifacts::ply::{PlyDiff, PlyMutation};
    use protocol::command::DiffAlgebra;
    use protocol::{Mutation, MutationDiff};

    #[semio_framework_async_macros::async_test]
    async fn missing_element_target_is_rejected_before_mutation() {
        let base = PlySnapshot::default();
        let diff = PlyDiff { elements: Some(PlyElementsDiff { removed: vec!["missing".into()], ..Default::default() }), ..Default::default() };
        let error = diff.apply(&base).await.expect_err("missing element target must be rejected");
        assert_eq!(error.code, "invalid-remove-target");
        assert_eq!(error.target, vec!["elements", "missing"]);
        assert_eq!(base, PlySnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_ply_snapshot();
        assert_eq!(snapshot.schema, STDIO_PLY_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = empty_ply_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(&text).await.expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(&bytes).await.expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️MeshFixture
    /// 🔺 A tetrahedron expressed as real `vertex`/`face` elements: 4 vertices, 4 triangular
    /// faces (via a `list uchar int vertex_indices` property) — small enough to hand-check,
    /// non-trivial enough (mixed-sign coords, several list-shaped faces) to catch layout bugs.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tetrahedron() -> PlySnapshot {
        let vertex_props = vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float }];
        let face_props = vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }];
        let vertex_rows: Vec<PlyRow> = [(0.0f32, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)].into_iter().map(|(x, y, z)| PlyRow { values: vec![PlyValue::Float(x), PlyValue::Float(y), PlyValue::Float(z)] }).collect();
        let face_rows: Vec<PlyRow> = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]].into_iter().map(|idx: [i32; 3]| PlyRow { values: vec![PlyValue::List(idx.into_iter().map(PlyValue::Int).collect())] }).collect();
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: Vec::new(),
            elements: vec![PlyElement { name: "vertex".into(), count: 4, properties: vertex_props, rows: vertex_rows }, PlyElement { name: "face".into(), count: 4, properties: face_props, rows: face_rows }],
        }
    }
    //#endregion 🔖️MeshFixture

    #[semio_framework_async_macros::async_test]
    async fn ascii_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::Ascii).expect("encode ascii");
        let decoded = decode_ply(&bytes).expect("decode ascii");
        assert_eq!(decoded.elements, snap.elements);
        assert_eq!(decoded.format, PlyFormat::Ascii);
    }

    #[semio_framework_async_macros::async_test]
    async fn binary_little_endian_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::BinaryLittleEndian).expect("encode binary LE");
        assert!(bytes.starts_with(b"ply\nformat binary_little_endian 1.0\n"), "header must declare binary_little_endian");
        let decoded = decode_ply(&bytes).expect("decode binary LE");
        assert_eq!(decoded.elements, snap.elements, "binary LE elements must exactly match the original");
    }

    #[semio_framework_async_macros::async_test]
    async fn binary_big_endian_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::BinaryBigEndian).expect("encode binary BE");
        let decoded = decode_ply(&bytes).expect("decode binary BE");
        assert_eq!(decoded.elements, snap.elements, "binary BE elements must exactly match the original");
    }

    #[semio_framework_async_macros::async_test]
    async fn binary_decode_skips_unmodeled_properties() {
        let header = "ply\nformat binary_little_endian 1.0\nelement vertex 1\nproperty float x\nproperty float y\nproperty float z\nproperty float nx\nproperty float ny\nproperty float nz\nend_header\n";
        let mut bytes = header.as_bytes().to_vec();
        for f in [1.5f32, 2.5, 3.5, 9.0, 9.0, 9.0] {
            bytes.extend_from_slice(&f.to_le_bytes());
        }
        let decoded = decode_ply(&bytes).expect("decode with retained normal properties");
        assert_eq!(decoded.elements.len(), 1);
        let vertex = &decoded.elements[0];
        assert_eq!(vertex.properties.len(), 6, "all 6 declared properties retained, none dropped");
        assert_eq!(vertex.rows[0].values[0], PlyValue::Float(1.5));
        assert_eq!(vertex.rows[0].values[3], PlyValue::Float(9.0), "nx retained, not silently discarded");
    }

    #[semio_framework_async_macros::async_test]
    async fn ascii_decode_rejects_truncated_body() {
        let header = "ply\nformat ascii 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nend_header\n1 2 3\n";
        let err = decode_ply(header.as_bytes()).unwrap_err();
        assert!(err.contains("eof"), "unexpected error: {err}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_end_header_is_rejected() {
        let err = decode_ply(b"ply\nformat ascii 1.0\n").unwrap_err();
        assert!(err.contains("end_header"));
    }

    #[semio_framework_async_macros::async_test]
    async fn comments_are_retained_in_order() {
        let text = "ply\nformat ascii 1.0\ncomment first\ncomment second\nelement vertex 0\nproperty float x\nend_header\n";
        let decoded = decode_ply(text.as_bytes()).expect("decode with comments");
        assert_eq!(decoded.comments, vec!["first".to_string(), "second".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn arbitrary_named_element_round_trips() {
        let props = vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }, PlyProperty::List { name: "endpoints".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::UShort }];
        let rows = vec![PlyRow { values: vec![PlyValue::Double(2.5), PlyValue::List(vec![PlyValue::UShort(3), PlyValue::UShort(7)])] }];
        let snap = PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), format: PlyFormat::Ascii, comments: vec![], elements: vec![PlyElement { name: "edge".into(), count: 1, properties: props, rows }] };
        let bytes = encode_ply(&snap).expect("encode");
        let decoded = decode_ply(&bytes).expect("decode");
        assert_eq!(decoded.elements, snap.elements);
    }
    //#endregion

    //#region 🔖️LawFixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn law_base() -> PlySnapshot {
        tetrahedron()
    }
    //#endregion

    //#region 🔖️MutationDiffLaw
    /// 1️⃣ `mutation_diff_law`: ∀ variant, `m.diff(base).diff().apply(base)` matches
    /// `apply_ply_mutation`'s in-place result, and the returned diff equals `m.diff(base)`.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = law_base();
        let variants = vec![
            PlyMutation::NoMutation,
            PlyMutation::SetFormat { format: PlyFormat::BinaryLittleEndian },
            PlyMutation::InsertComment { index: 0, comment: "hello".into() },
            PlyMutation::AddElement {
                index: 0,
                element: PlyElement { name: "material".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "shininess".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(0.5)] }] },
            },
            PlyMutation::RemoveElement { name: "face".into() },
            PlyMutation::InsertRow { element_name: "vertex".into(), index: 1, row: PlyRow { values: vec![PlyValue::Float(9.0), PlyValue::Float(9.0), PlyValue::Float(9.0)] } },
            PlyMutation::RemoveRow { element_name: "vertex".into(), index: 0 },
            PlyMutation::SetRowProperty { element_name: "vertex".into(), row_index: 0, property_name: "x".into(), value: PlyValue::Float(42.0) },
            PlyMutation::SetSnapshot { snapshot: PlySnapshot::default() },
        ];
        for m in variants {
            let mut snapshot = base.clone();
            let returned = apply_ply_mutation(&mut snapshot, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
            assert_eq!(snapshot, expected_diff.await.diff().apply(&base).expect("valid mutation diff"), "apply_ply_mutation result must equal diff.diff().apply(base) for {m:?}");
        }
    }
    //#endregion

    //#region 🔖️InverseLaw
    /// 2️⃣ `inverse_law`: mutation-level round trip for every variant, plus diff-level
    /// `d.diff().inverse(base).apply(&d.diff().apply(base)) == base`.
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = law_base();
        let variants = vec![
            PlyMutation::SetFormat { format: PlyFormat::BinaryBigEndian },
            PlyMutation::InsertComment { index: 0, comment: "note".into() },
            PlyMutation::AddElement { index: 2, element: PlyElement { name: "edge".into(), count: 0, properties: vec![], rows: vec![] } },
            PlyMutation::RemoveElement { name: "face".into() },
            PlyMutation::InsertRow { element_name: "vertex".into(), index: 0, row: PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(1.0)] } },
            PlyMutation::RemoveRow { element_name: "vertex".into(), index: 2 },
            PlyMutation::SetRowProperty { element_name: "vertex".into(), row_index: 1, property_name: "y".into(), value: PlyValue::Float(-1.0) },
        ];
        for m in variants {
            let mut snapshot = base.clone();
            let d = apply_ply_mutation(&mut snapshot, &m);
            for inv in <PlyMutation as Mutation<PlySnapshot>>::inverse(&m, &base) {
                let mut undone = snapshot.clone();
                apply_ply_mutation(&mut undone, &inv);
                assert_eq!(undone, base, "mutation-level inverse must restore base for {m:?}");
            }
            let d_inv = d.diff().inverse(&base);
            let mutated = d.diff().apply(&base).expect("valid forward diff");
            assert_eq!(d_inv.apply(&mutated).expect("valid inverse diff"), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion

    //#region 🔖️AbsorbLaw
    /// 3️⃣ `absorb_law`: curated op pairs (Insert+Remove-before, Insert+Insert-same-index,
    /// Add+SetField, Modify+Remove per key kind) plus associativity.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_insert_then_remove_before() {
        let base = law_base();
        let m1 = PlyMutation::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(9.0), PlyValue::Float(9.0), PlyValue::Float(9.0)] } };
        let mut mid = base.clone();
        let d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::RemoveRow { element_name: "vertex".into(), index: 0 };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after, "absorb(d1,d2).apply(base) == d2.diff().apply(d1.diff().apply(base))");
        let rows_diff = merged.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
        assert_eq!(rows_diff.removed, vec![0], "base index 0 removed");
        assert_eq!(rows_diff.added.len(), 1, "the surviving insert, shifted to final index 1");
        assert_eq!(rows_diff.added[0].index, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_insert_insert_same_index_both_survive() {
        let base = law_base();
        let m1 = PlyMutation::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(1.0)] } };
        let mut mid = base.clone();
        let d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(2.0), PlyValue::Float(2.0), PlyValue::Float(2.0)] } };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after, "both inserts must survive absorb");
        let rows_diff = merged.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
        assert_eq!(rows_diff.added.len(), 2, "both inserts survive (fixes the op-slot LWW bug the recipe bans)");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_add_element_then_set_row_property_patches_into_added() {
        let base = law_base();
        let new_element = PlyElement { name: "material".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "shininess".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(0.1)] }] };
        let m1 = PlyMutation::AddElement { index: 2, element: new_element };
        let mut mid = base.clone();
        let d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::SetRowProperty { element_name: "material".into(), row_index: 0, property_name: "shininess".into(), value: PlyValue::Float(0.9) };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after);
        let ed = merged.elements.as_ref().unwrap();
        assert!(ed.modified.iter().all(|m| m.name != "material"), "no separate modified entry for the added element");
        let added = ed.added.iter().find(|a| a.element.name == "material").expect("material still in added[]");
        assert_eq!(added.element.rows[0].values[0], PlyValue::Float(0.9), "patched directly into the carried added payload");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_modify_then_remove_name_keyed() {
        let base = law_base();
        let m1 = PlyMutation::SetRowProperty { element_name: "face".into(), row_index: 0, property_name: "vertex_indices".into(), value: PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)]) };
        let mut mid = base.clone();
        let d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::RemoveElement { name: "face".into() };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after);
        let ed = merged.elements.as_ref().unwrap();
        assert!(ed.modified.iter().all(|m| m.name != "face"), "modified-of-removed collapses away");
        assert!(ed.removed.contains(&"face".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_modify_then_remove_index_keyed() {
        let base = law_base();
        let m1 = PlyMutation::SetRowProperty { element_name: "vertex".into(), row_index: 1, property_name: "x".into(), value: PlyValue::Float(5.0) };
        let mut mid = base.clone();
        let d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::RemoveRow { element_name: "vertex".into(), index: 1 };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), after);
        let rows_diff = merged.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
        assert!(rows_diff.modified.iter().all(|m| m.index != 1), "modified-of-removed row collapses away");
        assert!(rows_diff.removed.contains(&1));
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_associativity() {
        let base = law_base();
        let m1 = PlyMutation::SetFormat { format: PlyFormat::BinaryLittleEndian };
        let m2 = PlyMutation::InsertComment { index: 0, comment: "x".into() };
        let m3 = PlyMutation::RemoveElement { name: "face".into() };
        let mut s1 = base.clone();
        let d1 = apply_ply_mutation(&mut s1, &m1);
        let mut s2 = s1.clone();
        let d2 = apply_ply_mutation(&mut s2, &m2);
        let mut s3 = s2.clone();
        let d3 = apply_ply_mutation(&mut s3, &m3);

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut right_inner = d2.diff().clone();
        right_inner.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(right_inner);

        assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "associativity: (d1∘d2)∘d3 == d1∘(d2∘d3) applied");
        assert_eq!(left.apply(&base).expect("valid associated diff"), s3, "both associations must equal sequential application");
    }
    //#endregion

    //#region 🔖️BetweenRoundtripLaw
    /// 4️⃣ `between_roundtrip_law`: `between(a,b).apply(a) == b` on synthetic fixtures.
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = law_base();
        let mut b = a.clone();
        b.format = PlyFormat::BinaryBigEndian;
        b.comments = vec!["hello".into()];
        b.elements[0].rows[0].values[0] = PlyValue::Float(100.0);
        b.elements.remove(1); // drop "face" entirely
        b.elements.push(PlyElement { name: "edge".into(), count: 0, properties: vec![], rows: vec![] });

        let d = PlyDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");
        let back = PlyDiff::between(&b, &a);
        assert_eq!(back.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");
    }
    //#endregion

    //#region 🔖️CodecRetentionLaw
    /// 5️⃣ `codec_retention_law`: decode→encode is byte-preserving for ascii/binary fixtures.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        for format in [PlyFormat::Ascii, PlyFormat::BinaryLittleEndian, PlyFormat::BinaryBigEndian] {
            let snap = tetrahedron();
            let encoded = encode_ply_with_format(&snap, format).expect("encode");
            let decoded = decode_ply(&encoded).expect("decode");
            let re_encoded = encode_ply_with_format(&decoded, format).expect("re-encode");
            assert_eq!(encoded, re_encoded, "decode→encode must be byte-preserving for {format:?}");
        }
    }
    //#endregion

    //#region 🔖️FieldSweep
    /// 6️⃣ `field_sweep`: `sweep_a`/`sweep_b` differ in EVERY mutable field.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> PlySnapshot {
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec!["a".into()],
            elements: vec![
                PlyElement {
                    name: "vertex".into(),
                    count: 2,
                    properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }],
                    rows: vec![PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0)] }, PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0)] }],
                },
                PlyElement {
                    name: "face".into(),
                    count: 1,
                    properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                    rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
                },
            ],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> PlySnapshot {
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::BinaryLittleEndian,
            comments: vec!["a".into(), "b".into()],
            elements: vec![
                PlyElement {
                    name: "vertex".into(),
                    count: 1,
                    properties: vec![PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Double }, PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Double }],
                    rows: vec![PlyRow { values: vec![PlyValue::Double(9.0), PlyValue::Double(9.0)] }],
                },
                PlyElement { name: "edge".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }], rows: vec![PlyRow { values: vec![PlyValue::Double(3.5)] }] },
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let ab = PlyDiff::between(&a, &b);
        assert!(ab.await.format.is_some(), "format field must be exercised");
        assert!(ab.await.comments.is_some(), "comments field must be exercised");
        let ab_elements = ab.await.elements.as_ref().expect("elements diff must be present");
        assert!(!ab_elements.removed.is_empty(), "sweep must exercise a removed element (face)");
        assert!(!ab_elements.added.is_empty(), "sweep must exercise an added element (edge)");
        assert!(!ab_elements.modified.is_empty(), "sweep must exercise a modified element (vertex)");
        let vertex_mod = ab_elements.modified.iter().find(|m| m.name == "vertex").expect("vertex modified");
        assert!(vertex_mod.diff.properties.is_some(), "properties weak-replace must be exercised");
        assert!(vertex_mod.diff.rows.is_some(), "rows triple must be exercised (schema-change scope cut path)");
        assert_eq!(ab.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");

        let ba = PlyDiff::between(&b, &a);
        let ba_elements = ba.await.elements.as_ref().expect("reverse elements diff must be present");
        assert!(!ba_elements.removed.is_empty(), "reverse direction: edge removed");
        assert!(!ba_elements.added.is_empty(), "reverse direction: face added");
        assert_eq!(ba.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");

        assert!(PlyDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
        assert!(PlyDiff::between(&b, &b).is_empty(), "between(b,b) must be empty");
    }

    /// 🧪 Direct row-level triple sweep (not routed through the schema-change scope cut).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_row_triple_both_directions() {
        let common_props = vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Int }];
        let a = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec![],
            elements: vec![PlyElement { name: "point".into(), count: 2, properties: common_props.clone(), rows: vec![PlyRow { values: vec![PlyValue::Int(1)] }, PlyRow { values: vec![PlyValue::Int(2)] }] }],
        };
        let b = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec![],
            elements: vec![PlyElement { name: "point".into(), count: 3, properties: common_props, rows: vec![PlyRow { values: vec![PlyValue::Int(99)] }, PlyRow { values: vec![PlyValue::Int(2)] }, PlyRow { values: vec![PlyValue::Int(3)] }] }],
        };
        let ab = PlyDiff::between(&a, &b);
        let ab_rows = ab.await.elements.as_ref().unwrap().modified[0].diff.rows.as_ref().expect("rows diff");
        assert!(!ab_rows.modified.is_empty(), "row 0 modified (1 -> 99)");
        assert!(!ab_rows.added.is_empty(), "row 2 added (b longer)");
        assert!(ab_rows.removed.is_empty(), "b is longer, no removed tail in this direction");
        assert_eq!(ab.apply(&a).expect("valid forward diff"), b);

        let ba = PlyDiff::between(&b, &a);
        let ba_rows = ba.await.elements.as_ref().unwrap().modified[0].diff.rows.as_ref().expect("rows diff");
        assert!(!ba_rows.removed.is_empty(), "a is shorter, removed tail in this direction");
        assert_eq!(ba.apply(&b).expect("valid backward diff"), a);
    }
    //#endregion

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
    /// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::ply::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_ply_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_ply_snapshot();
            let packed = store::ArtifactPack::encode_pack(&demo);
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack, ascii) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk (ascii) did not consume every byte");

            for format in [PlyFormat::BinaryLittleEndian, PlyFormat::BinaryBigEndian] {
                let raw = encode_ply_with_format(&demo, format).expect("encode binary variant");
                let trace = dsl::walk_protocol(&pack_spec, &raw).unwrap_or_else(|e| panic!("walk_protocol(pack, {format:?}) failed @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, raw.len(), "pack walk ({format:?}) did not consume every byte");
            }

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().await.unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_ply_snapshot();

            let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).await.expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_ply_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_ply_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).await.expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_ply_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_ply_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
