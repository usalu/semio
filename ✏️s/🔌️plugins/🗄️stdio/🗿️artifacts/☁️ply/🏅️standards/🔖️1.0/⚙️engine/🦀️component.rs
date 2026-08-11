//! ⚙️ PlyEngine — real ply codec.
//!
//! Decode supports the real §PLY header grammar: arbitrary `element <name> <count>` /
//! `property <type> <name>` / `property list <count-type> <value-type> <name>` declarations,
//! walked fully generically (never hardcoded to `x y z` + `vertex_indices` — that hardcoding,
//! and the `MeshVertex`/`MeshTriangle` types it fed, is exactly what Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL kills), across all three `format` variants (`ascii`,
//! `binary_little_endian`, `binary_big_endian`), retaining every declared property's real type
//! and every row's real typed cell values. Encode walks the same generic element/property/row
//! model back out in whichever of the three wire formats is requested — round-tripping any
//! element/property layout, not just vertex/face meshes.

use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType, PlyValue};
use crate::artifacts::ply::{PlyArtifact, PlyDiff, PlyMutation, PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

//#region 🔖️ScalarWire
/// 📏 Byte width of a PLY scalar type.
fn scalar_type_size(kind: PlyScalarType) -> usize {
    match kind {
        PlyScalarType::Char | PlyScalarType::UChar => 1,
        PlyScalarType::Short | PlyScalarType::UShort => 2,
        PlyScalarType::Int | PlyScalarType::UInt | PlyScalarType::Float => 4,
        PlyScalarType::Double => 8,
    }
}

/// 🔤 Parses both long (`float`) and short (`float32`) PLY type spellings.
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
fn split_header(data: &[u8]) -> Result<(String, &[u8]), String> {
    let marker = b"end_header";
    let idx = data.windows(marker.len()).position(|w| w == marker).ok_or("ply: missing end_header")?;
    let after_marker = idx + marker.len();
    let mut nl = after_marker;
    while nl < data.len() && data[nl] != b'\n' { nl += 1; }
    if nl >= data.len() { return Err("ply: truncated header".into()); }
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
fn parse_header_text(text: &str) -> Result<PlyHeader, String> {
    let mut lines = text.lines();
    let first = lines.next().ok_or("ply: empty header")?.trim();
    if first != "ply" { return Err("ply: expected 'ply' magic line".into()); }
    let mut format: Option<PlyFormat> = None;
    let mut comments: Vec<String> = Vec::new();
    let mut elements: Vec<PlyElement> = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line == "end_header" { continue; }
        if let Some(rest) = line.strip_prefix("comment") {
            // 💬 `comment` may be followed by a space and text, or stand bare.
            comments.push(rest.strip_prefix(' ').unwrap_or(rest).to_string());
            continue;
        }
        if line.starts_with("obj_info") { continue; } // 🕳️ not modeled (documented deviation).
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
fn read_scalar_bin(kind: PlyScalarType, data: &[u8], pos: &mut usize, big: bool) -> Result<PlyValue, String> {
    let size = scalar_type_size(kind);
    if *pos + size > data.len() { return Err("ply: truncated binary body".into()); }
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

//#region 🔖️Codec
/// 🏗️ Builds the header text for `format`, walking every element's real name/count and every
/// property's real declaration — generic, not canonicalized to a fixed vertex/face layout
/// (unlike the pre-rewrite engine; see module doc). P2-FG3 bugfix: `comments` are now genuinely
/// re-emitted as real `comment <text>\n` lines (see the doc comment on the call site below for
/// why this scope cut needed closing).
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

/// 🏗️ Encodes `snap` in the given wire `format` (ascii / binary LE / binary BE). P2-FG3
/// bugfix: `comments` ARE now re-emitted into the header on encode (as real `comment <text>\n`
/// lines, matching `parse_header_text`'s own decode side, which already retained them) — the
/// pre-FG3 code left this a documented scope cut ("comments are pure metadata... re-emission is
/// a straightforward follow-up"), but that silently broke `decode_ply(encode_ply(snap))`'s own
/// round-trip for any snapshot with non-empty `comments`, and made the snapshot text grammar's
/// own `comment-line` production permanently unreachable by this artifact's real `print_dsl`
/// output. Closed here, not deferred further.
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
                                    for item in items { push_scalar_bin(&mut out, item, big); }
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
pub fn encode_ply(snap: &PlySnapshot) -> Result<Vec<u8>, String> {
    encode_ply_with_format(snap, PlyFormat::Ascii)
}

/// 🔍 Decodes any of the three wire formats, dispatching on the header's own `format` line.
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

/// 🌱 Empty persisted snapshot.
pub fn empty_ply_snapshot() -> PlySnapshot {
    PlySnapshot::default()
}
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::ply::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<PlySnapshot, PlyMutation>(STDIO_PLY_DOCUMENT_SCHEMA));
}

/// 📌️ P2-FG3: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// own exemplar pattern (`📷️png/…/⚙️engine/🦀️component.rs`'s `register_pilot_languages`) —
/// `stdio.ply`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None`, matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role, even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below). Previously this registered ONLY the `Document` role — the other 4 are new this wave.
///
/// `register_schema_spec` (P2-M3's `FullResolver` insertion API) is deliberately NOT called here
/// — see this wave's report `mechanism_gaps`: it requires `fn() -> RecordSpec`, and `stdio.ply`
/// has no derivable `RecordSpec` by design (`PlySnapshot`/`PlyDiff`/`PlyMutation`'s
/// `ArtifactDsl`/`ArtifactPack`/`DiffCodec`/`OpText`/`OpBinary` are ALL hand-rolled because
/// `PlyProperty`/`PlyValue` are genuine data-carrying enums with no derivable `DslField` impl —
/// same root cause json/csv/zip/png's own `register_pilot_languages` doc comments already
/// document for their own hand-rolled facets).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ply", extension: Some("ply"), role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::ply::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ply::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::ply::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ply::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ply"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ply.op", extension: None, role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::ply::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ply::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::ply::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ply::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ply.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ply.diff", extension: None, role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::ply::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ply::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.ply.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ply.pack", extension: None, role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::ply::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ply::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ply.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ply.spr", extension: None, role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::ply::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ply::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ply.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.ply`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::ply::schema::ply_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.ply` artifact engine.
pub struct PlyEngine {
    artifact_state: PlyArtifact,
    snapshot_state: PlySnapshot,
}

impl PlyEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: PlySnapshot) -> Self {
        let artifact_state = PlyArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️DemoSnapshot
/// ✅️ P2-FG3: the representative `PlySnapshot` every conformance law and the shipped
/// `📚️examples/🎬️demo/🖼️assets` fixtures are built from — a `vertex` element (2 rows, plain
/// scalar `float` properties) and a `face` element (1 row, a `list uchar int vertex_indices`
/// property, exercising the count-prefixed list-cell shape) plus one comment. `format:
/// PlyFormat::Ascii` deliberately — `print_dsl`/`parse_dsl` always render/read the CANONICAL
/// ascii encoding regardless of `format` (see `../🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s
/// own `HandcraftedArtifactCodecs` doc comment), so a demo snapshot whose OWN `format` field
/// isn't `Ascii` would make `fixture_honesty_law`'s `parse_dsl(print_dsl(demo)) == demo` fail —
/// the DSL/text facet's own format-normalization would silently overwrite it. The Pack facet
/// (which DOES respect `self.format`, see the P2-FG3 bugfix in that same file) is exercised
/// against genuine BINARY bytes separately, by `protocol_walk_law` calling
/// `encode_ply_with_format` directly with a non-ascii format — see that test.
pub fn demo_ply_snapshot() -> PlySnapshot {
    PlySnapshot {
        schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["semio demo".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![
                    PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float },
                    PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float },
                    PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float },
                ],
                rows: vec![
                    PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::Float(0.0)] },
                    PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(0.5), PlyValue::Float(-1.5)] },
                ],
            },
            PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1)])] }],
            },
        ],
    }
}
//#endregion 🔖️DemoSnapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::ply::schema::mutations::apply_ply_mutation;
    use protocol::command::DiffAlgebra;
    use protocol::{Mutation, MutationDiff};

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_ply_snapshot();
        assert_eq!(snapshot.schema, STDIO_PLY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_ply_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️MeshFixture
    /// 🔺 A tetrahedron expressed as real `vertex`/`face` elements: 4 vertices, 4 triangular
    /// faces (via a `list uchar int vertex_indices` property) — small enough to hand-check,
    /// non-trivial enough (mixed-sign coords, several list-shaped faces) to catch layout bugs.
    fn tetrahedron() -> PlySnapshot {
        let vertex_props = vec![
            PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float },
            PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float },
            PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float },
        ];
        let face_props = vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }];
        let vertex_rows: Vec<PlyRow> = [(0.0f32, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]
            .into_iter()
            .map(|(x, y, z)| PlyRow { values: vec![PlyValue::Float(x), PlyValue::Float(y), PlyValue::Float(z)] })
            .collect();
        let face_rows: Vec<PlyRow> = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]]
            .into_iter()
            .map(|idx: [i32; 3]| PlyRow { values: vec![PlyValue::List(idx.into_iter().map(PlyValue::Int).collect())] })
            .collect();
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: Vec::new(),
            elements: vec![
                PlyElement { name: "vertex".into(), count: 4, properties: vertex_props, rows: vertex_rows },
                PlyElement { name: "face".into(), count: 4, properties: face_props, rows: face_rows },
            ],
        }
    }
    //#endregion 🔖️MeshFixture

    #[test]
    fn ascii_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::Ascii).expect("encode ascii");
        let decoded = decode_ply(&bytes).expect("decode ascii");
        assert_eq!(decoded.elements, snap.elements);
        assert_eq!(decoded.format, PlyFormat::Ascii);
    }

    #[test]
    fn binary_little_endian_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::BinaryLittleEndian).expect("encode binary LE");
        assert!(bytes.starts_with(b"ply\nformat binary_little_endian 1.0\n"), "header must declare binary_little_endian");
        let decoded = decode_ply(&bytes).expect("decode binary LE");
        assert_eq!(decoded.elements, snap.elements, "binary LE elements must exactly match the original");
    }

    #[test]
    fn binary_big_endian_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::BinaryBigEndian).expect("encode binary BE");
        let decoded = decode_ply(&bytes).expect("decode binary BE");
        assert_eq!(decoded.elements, snap.elements, "binary BE elements must exactly match the original");
    }

    #[test]
    fn binary_decode_skips_unmodeled_properties() {
        // Hand-crafted binary_little_endian stream with per-vertex normals (nx/ny/nz) declared
        // in the header but not otherwise special-cased — proves decode walks the real
        // header-declared property list (retaining nx/ny/nz as real typed cells) instead of
        // assuming a fixed 12-byte stride.
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

    #[test]
    fn ascii_decode_rejects_truncated_body() {
        let header = "ply\nformat ascii 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nend_header\n1 2 3\n";
        let err = decode_ply(header.as_bytes()).unwrap_err();
        assert!(err.contains("eof"), "unexpected error: {err}");
    }

    #[test]
    fn missing_end_header_is_rejected() {
        let err = decode_ply(b"ply\nformat ascii 1.0\n").unwrap_err();
        assert!(err.contains("end_header"));
    }

    #[test]
    fn comments_are_retained_in_order() {
        let text = "ply\nformat ascii 1.0\ncomment first\ncomment second\nelement vertex 0\nproperty float x\nend_header\n";
        let decoded = decode_ply(text.as_bytes()).expect("decode with comments");
        assert_eq!(decoded.comments, vec!["first".to_string(), "second".to_string()]);
    }

    #[test]
    fn arbitrary_named_element_round_trips() {
        // 🎲 A wholly custom element name ("edge") with a mixed scalar/list property set —
        // proves the model isn't secretly hardcoded to "vertex"/"face" despite the fixture
        // helper above always using those names for readability.
        let props = vec![
            PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double },
            PlyProperty::List { name: "endpoints".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::UShort },
        ];
        let rows = vec![PlyRow { values: vec![PlyValue::Double(2.5), PlyValue::List(vec![PlyValue::UShort(3), PlyValue::UShort(7)])] }];
        let snap = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec![],
            elements: vec![PlyElement { name: "edge".into(), count: 1, properties: props, rows }],
        };
        let bytes = encode_ply(&snap).expect("encode");
        let decoded = decode_ply(&bytes).expect("decode");
        assert_eq!(decoded.elements, snap.elements);
    }
    //#endregion

    //#region 🔖️LawFixtures
    fn law_base() -> PlySnapshot {
        tetrahedron()
    }
    //#endregion

    //#region 🔖️MutationDiffLaw
    /// 1️⃣ `mutation_diff_law`: ∀ variant, `m.diff(base).apply(base)` matches
    /// `apply_ply_mutation`'s in-place result, and the returned diff equals `m.diff(base)`.
    #[test]
    fn mutation_diff_law() {
        let base = law_base();
        let variants = vec![
            PlyMutation::NoMutation,
            PlyMutation::SetFormat { format: PlyFormat::BinaryLittleEndian },
            PlyMutation::InsertComment { index: 0, comment: "hello".into() },
            PlyMutation::AddElement {
                index: 0,
                element: PlyElement {
                    name: "material".into(),
                    count: 1,
                    properties: vec![PlyProperty::Scalar { name: "shininess".into(), kind: PlyScalarType::Float }],
                    rows: vec![PlyRow { values: vec![PlyValue::Float(0.5)] }],
                },
            },
            PlyMutation::RemoveElement { name: "face".into() },
            PlyMutation::InsertRow {
                element_name: "vertex".into(),
                index: 1,
                row: PlyRow { values: vec![PlyValue::Float(9.0), PlyValue::Float(9.0), PlyValue::Float(9.0)] },
            },
            PlyMutation::RemoveRow { element_name: "vertex".into(), index: 0 },
            PlyMutation::SetRowProperty { element_name: "vertex".into(), row_index: 0, property_name: "x".into(), value: PlyValue::Float(42.0) },
            PlyMutation::SetSnapshot { snapshot: PlySnapshot::default() },
        ];
        for m in variants {
            let mut snapshot = base.clone();
            let returned = apply_ply_mutation(&mut snapshot, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
            assert_eq!(snapshot, expected_diff.apply(&base), "apply_ply_mutation result must equal diff.apply(base) for {m:?}");
        }
    }
    //#endregion

    //#region 🔖️InverseLaw
    /// 2️⃣ `inverse_law`: mutation-level round trip for every variant, plus diff-level
    /// `d.inverse(base).apply(&d.apply(base)) == base`.
    #[test]
    fn inverse_law() {
        let base = law_base();
        let variants = vec![
            PlyMutation::SetFormat { format: PlyFormat::BinaryBigEndian },
            PlyMutation::InsertComment { index: 0, comment: "note".into() },
            PlyMutation::AddElement {
                index: 2,
                element: PlyElement { name: "edge".into(), count: 0, properties: vec![], rows: vec![] },
            },
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
            let d_inv = d.inverse(&base);
            assert_eq!(d_inv.apply(&d.apply(&base)), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion

    //#region 🔖️AbsorbLaw
    /// 3️⃣ `absorb_law`: curated op pairs (Insert+Remove-before, Insert+Insert-same-index,
    /// Add+SetField, Modify+Remove per key kind) plus associativity.
    #[test]
    fn absorb_law_insert_then_remove_before() {
        // Insert(2) + Remove(0) on `vertex` rows → {removed:[0], added:[(1,f)]}.
        let base = law_base();
        let m1 = PlyMutation::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(9.0), PlyValue::Float(9.0), PlyValue::Float(9.0)] } };
        let mut mid = base.clone();
        let mut d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::RemoveRow { element_name: "vertex".into(), index: 0 };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after, "absorb(d1,d2).apply(base) == d2.apply(d1.apply(base))");
        let rows_diff = d1.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
        assert_eq!(rows_diff.removed, vec![0], "base index 0 removed");
        assert_eq!(rows_diff.added.len(), 1, "the surviving insert, shifted to final index 1");
        assert_eq!(rows_diff.added[0].index, 1);
    }

    #[test]
    fn absorb_law_insert_insert_same_index_both_survive() {
        let base = law_base();
        let m1 = PlyMutation::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(1.0)] } };
        let mut mid = base.clone();
        let mut d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::InsertRow { element_name: "vertex".into(), index: 2, row: PlyRow { values: vec![PlyValue::Float(2.0), PlyValue::Float(2.0), PlyValue::Float(2.0)] } };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after, "both inserts must survive absorb");
        let rows_diff = d1.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
        assert_eq!(rows_diff.added.len(), 2, "both inserts survive (fixes the op-slot LWW bug the recipe bans)");
    }

    #[test]
    fn absorb_law_add_element_then_set_row_property_patches_into_added() {
        // Insert(1,f) + SetField(1,v) → patch-into-added, at the ELEMENT granularity: an
        // AddElement (whole element, real `properties` on the carried payload) followed by a
        // SetRowProperty targeting one of ITS rows must patch directly into the added payload
        // rather than surfacing as a separate `modified` entry.
        let base = law_base();
        let new_element = PlyElement {
            name: "material".into(),
            count: 1,
            properties: vec![PlyProperty::Scalar { name: "shininess".into(), kind: PlyScalarType::Float }],
            rows: vec![PlyRow { values: vec![PlyValue::Float(0.1)] }],
        };
        let m1 = PlyMutation::AddElement { index: 2, element: new_element };
        let mut mid = base.clone();
        let mut d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::SetRowProperty { element_name: "material".into(), row_index: 0, property_name: "shininess".into(), value: PlyValue::Float(0.9) };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
        let ed = d1.elements.as_ref().unwrap();
        assert!(ed.modified.iter().all(|m| m.name != "material"), "no separate modified entry for the added element");
        let added = ed.added.iter().find(|a| a.element.name == "material").expect("material still in added[]");
        assert_eq!(added.element.rows[0].values[0], PlyValue::Float(0.9), "patched directly into the carried added payload");
    }

    #[test]
    fn absorb_law_modify_then_remove_name_keyed() {
        // Modify+Remove per key kind — name-keyed (`elements`): modifying "face" then removing
        // it collapses to a pure removal, no dangling modified entry.
        let base = law_base();
        let m1 = PlyMutation::SetRowProperty { element_name: "face".into(), row_index: 0, property_name: "vertex_indices".into(), value: PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)]) };
        let mut mid = base.clone();
        let mut d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::RemoveElement { name: "face".into() };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
        let ed = d1.elements.as_ref().unwrap();
        assert!(ed.modified.iter().all(|m| m.name != "face"), "modified-of-removed collapses away");
        assert!(ed.removed.contains(&"face".to_string()));
    }

    #[test]
    fn absorb_law_modify_then_remove_index_keyed() {
        // Modify+Remove per key kind — index-keyed (`rows`, within the SAME element).
        let base = law_base();
        let m1 = PlyMutation::SetRowProperty { element_name: "vertex".into(), row_index: 1, property_name: "x".into(), value: PlyValue::Float(5.0) };
        let mut mid = base.clone();
        let mut d1 = apply_ply_mutation(&mut mid, &m1);
        let m2 = PlyMutation::RemoveRow { element_name: "vertex".into(), index: 1 };
        let mut after = mid.clone();
        let d2 = apply_ply_mutation(&mut after, &m2);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base), after);
        let rows_diff = d1.elements.as_ref().unwrap().modified.iter().find(|m| m.name == "vertex").unwrap().diff.rows.as_ref().unwrap();
        assert!(rows_diff.modified.iter().all(|m| m.index != 1), "modified-of-removed row collapses away");
        assert!(rows_diff.removed.contains(&1));
    }

    #[test]
    fn absorb_law_associativity() {
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

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_inner = d2.clone();
        right_inner.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_inner);

        assert_eq!(left.apply(&base), right.apply(&base), "associativity: (d1∘d2)∘d3 == d1∘(d2∘d3) applied");
        assert_eq!(left.apply(&base), s3, "both associations must equal sequential application");
    }
    //#endregion

    //#region 🔖️BetweenRoundtripLaw
    /// 4️⃣ `between_roundtrip_law`: `between(a,b).apply(a) == b` on synthetic fixtures.
    #[test]
    fn between_roundtrip_law() {
        let a = law_base();
        let mut b = a.clone();
        b.format = PlyFormat::BinaryBigEndian;
        b.comments = vec!["hello".into()];
        b.elements[0].rows[0].values[0] = PlyValue::Float(100.0);
        b.elements.remove(1); // drop "face" entirely
        b.elements.push(PlyElement { name: "edge".into(), count: 0, properties: vec![], rows: vec![] });

        let d = PlyDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) == b");
        let back = PlyDiff::between(&b, &a);
        assert_eq!(back.apply(&b), a, "between(b,a).apply(b) == a");
    }
    //#endregion

    //#region 🔖️CodecRetentionLaw
    /// 5️⃣ `codec_retention_law`: decode→encode is byte-preserving for ascii/binary fixtures
    /// (up to documented normalization: comments are not re-emitted, see `encode_ply_with_format`).
    #[test]
    fn codec_retention_law() {
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
    /// 6️⃣ `field_sweep`: `sweep_a`/`sweep_b` differ in EVERY mutable field — `format`,
    /// `comments`, and `elements` (one removed, one modified in every sub-field incl. its
    /// `properties` weak-replace, one added) — with every tri-state/collection kind exercised
    /// and both `between` directions asserted (deliberately DIFFERENT element counts and row
    /// counts, per the recipe's mandatory asymmetric-length rule — see the ticket's F1 note on
    /// why a same-length collection can never show both `removed` and `added` from one call).
    fn sweep_a() -> PlySnapshot {
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec!["a".into()],
            elements: vec![
                // "vertex": will be MODIFIED (property replace + row remove + row modify + row add).
                PlyElement {
                    name: "vertex".into(),
                    count: 2,
                    properties: vec![
                        PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float },
                        PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float },
                    ],
                    rows: vec![
                        PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0)] },
                        PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0)] },
                    ],
                },
                // "face": will be REMOVED.
                PlyElement {
                    name: "face".into(),
                    count: 1,
                    properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                    rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
                },
            ],
        }
    }

    fn sweep_b() -> PlySnapshot {
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::BinaryLittleEndian, // format: changed
            comments: vec!["a".into(), "b".into()], // comments: changed (whole-vec replace)
            elements: vec![
                // "vertex": properties WEAK-REPLACED (nx/ny/nz instead of x/y) — schema-change
                // scope cut, whole-rows-replace — one row removed relative to base's 2, one added.
                PlyElement {
                    name: "vertex".into(),
                    count: 1,
                    properties: vec![
                        PlyProperty::Scalar { name: "nx".into(), kind: PlyScalarType::Double },
                        PlyProperty::Scalar { name: "ny".into(), kind: PlyScalarType::Double },
                    ],
                    rows: vec![PlyRow { values: vec![PlyValue::Double(9.0), PlyValue::Double(9.0)] }],
                },
                // "edge": ADDED.
                PlyElement {
                    name: "edge".into(),
                    count: 1,
                    properties: vec![PlyProperty::Scalar { name: "weight".into(), kind: PlyScalarType::Double }],
                    rows: vec![PlyRow { values: vec![PlyValue::Double(3.5)] }],
                },
            ],
        }
    }

    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let ab = PlyDiff::between(&a, &b);
        assert!(ab.format.is_some(), "format field must be exercised");
        assert!(ab.comments.is_some(), "comments field must be exercised");
        let ab_elements = ab.elements.as_ref().expect("elements diff must be present");
        assert!(!ab_elements.removed.is_empty(), "sweep must exercise a removed element (face)");
        assert!(!ab_elements.added.is_empty(), "sweep must exercise an added element (edge)");
        assert!(!ab_elements.modified.is_empty(), "sweep must exercise a modified element (vertex)");
        let vertex_mod = ab_elements.modified.iter().find(|m| m.name == "vertex").expect("vertex modified");
        assert!(vertex_mod.diff.properties.is_some(), "properties weak-replace must be exercised");
        assert!(vertex_mod.diff.rows.is_some(), "rows triple must be exercised (schema-change scope cut path)");
        assert_eq!(ab.apply(&a), b, "between(a,b).apply(a) == b");

        let ba = PlyDiff::between(&b, &a);
        let ba_elements = ba.elements.as_ref().expect("reverse elements diff must be present");
        assert!(!ba_elements.removed.is_empty(), "reverse direction: edge removed");
        assert!(!ba_elements.added.is_empty(), "reverse direction: face added");
        assert_eq!(ba.apply(&b), a, "between(b,a).apply(b) == a");

        assert!(PlyDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
        assert!(PlyDiff::between(&b, &b).is_empty(), "between(b,b) must be empty");
    }

    /// 🧪 Direct row-level triple sweep (not routed through the schema-change scope cut): same
    /// element name/properties on both sides so `rows_between` exercises its OWN
    /// removed/modified/added shape directly, asymmetric row counts across the two directions.
    #[test]
    fn field_sweep_row_triple_both_directions() {
        let common_props = vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Int }];
        let a = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec![],
            elements: vec![PlyElement {
                name: "point".into(),
                count: 2,
                properties: common_props.clone(),
                rows: vec![PlyRow { values: vec![PlyValue::Int(1)] }, PlyRow { values: vec![PlyValue::Int(2)] }],
            }],
        };
        let b = PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec![],
            elements: vec![PlyElement {
                name: "point".into(),
                count: 3,
                properties: common_props,
                rows: vec![PlyRow { values: vec![PlyValue::Int(99)] }, PlyRow { values: vec![PlyValue::Int(2)] }, PlyRow { values: vec![PlyValue::Int(3)] }],
            }],
        };
        let ab = PlyDiff::between(&a, &b);
        let ab_rows = ab.elements.as_ref().unwrap().modified[0].diff.rows.as_ref().expect("rows diff");
        assert!(!ab_rows.modified.is_empty(), "row 0 modified (1 -> 99)");
        assert!(!ab_rows.added.is_empty(), "row 2 added (b longer)");
        assert!(ab_rows.removed.is_empty(), "b is longer, no removed tail in this direction");
        assert_eq!(ab.apply(&a), b);

        let ba = PlyDiff::between(&b, &a);
        let ba_rows = ba.elements.as_ref().unwrap().modified[0].diff.rows.as_ref().expect("rows diff");
        assert!(!ba_rows.removed.is_empty(), "a is shorter, removed tail in this direction");
        assert_eq!(ba.apply(&b), a);
    }
    //#endregion

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG3: per-artifact conformance laws (§4 of `📖️grammar-recipe.md`'s checklist) —
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives here (the engine's own test region), not
    /// any framework file — mirrors png/gif89a's own `conformance_laws` module shape exactly.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::ply::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio`
        /// files parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a
        /// clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar (the real ASCII PLY header+body
        /// syntax) recognizes real `print_dsl` output for the demo snapshot — same preamble-
        /// stripped body reconstruction `m5_handcrafted_grammar_conformance`'s own
        /// `dsl_body_from_fixture` uses, so this is a direct proof this artifact will pass that
        /// harness once graduated, not merely an analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_ply_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `PlyMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `PlyDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty diff and every collection-triple shape.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// the snapshot Pack facet's `encode_pack` for BOTH the ascii demo snapshot AND (calling
        /// `encode_ply_with_format` directly, bypassing `ArtifactPack`) a genuine
        /// `binary_little_endian`/`binary_big_endian` rendering of the same demo (envelope-
        /// unwrapped first, matching how `m5_handcrafted_protocol_conformance` itself feeds
        /// `walk_protocol`) — every demo mutation's `encode_op`, and every demo diff's
        /// `encode_diff` — asserting `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_ply_snapshot();
            let packed = store::ArtifactPack::encode_pack(&demo);
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack, ascii) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk (ascii) did not consume every byte");

            for format in [PlyFormat::BinaryLittleEndian, PlyFormat::BinaryBigEndian] {
                let raw = encode_ply_with_format(&demo, format).expect("encode binary variant");
                let trace = dsl::walk_protocol(&pack_spec, &raw).unwrap_or_else(|e| panic!("walk_protocol(pack, {format:?}) failed @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, raw.len(), "pack walk ({format:?}) did not consume every byte — proves the SAME magic+opaque-tail protocol genuinely spans all 3 format variants");
            }

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_ply_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again (the pre-FG3 fixtures were the
        /// literal placeholder strings `"Hello, stdio.ply!"`/`"Hello, stdio.txt!"`).
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_ply_snapshot();

            let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_ply_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_ply_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_ply_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_ply_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
