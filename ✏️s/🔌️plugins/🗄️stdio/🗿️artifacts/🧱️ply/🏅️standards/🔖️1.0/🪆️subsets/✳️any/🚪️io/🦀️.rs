//! 🚪️ IO stdio.ply (1.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_0::subsets::any::schema::PlyAnalyzer;
    use crate::PlySnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct PlyComposerComposition;

    impl ArtifactComposition for PlyComposerComposition {
        type Snapshot = PlySnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
            let analysis = PlyAnalyzer::analyze(&native);
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
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType, PlyValue};
use crate::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

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
    use crate::standards::v1_0::subsets::any::schema::PlyComposer as PlyRawAnyComposer;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
