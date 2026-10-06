//! 📝️ Text representation codec surface for `stdio.tiff` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type TiffDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v6_0::subsets::document::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashMap};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

/// 🔢️ Generic numeric-token parser (`u8`/`u16`/`u32`/`i8`/`i16`/`i32`/`f32`/`f64`/`usize`, every
/// scalar this grammar carries) — `f32`/`f64`'s `Display`/`FromStr` round-trip exactly for every
/// finite value this codec ever produces (same assumption `svg`'s `ViewBox` float fields make).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_num<T: std::str::FromStr>(s: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    s.parse::<T>().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

/// 📃️ Generic bracketed comma list (`[e1,e2,...]`) — every `Vec<T>` in this grammar (IFD entries,
/// an IFD list, a numeric value list) uses this same shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_byte_order(b: TiffByteOrder) -> String {
    match b {
        TiffByteOrder::LittleEndian => "0".to_string(),
        TiffByteOrder::BigEndian => "1".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_byte_order(s: &str) -> Result<TiffByteOrder, String> {
    match s {
        "0" => Ok(TiffByteOrder::LittleEndian),
        "1" => Ok(TiffByteOrder::BigEndian),
        other => Err(format!("byte order: unknown code {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_field_type(k: TiffFieldType) -> String {
    k.to_u16().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_field_type(s: &str) -> Result<TiffFieldType, String> {
    TiffFieldType::from_u16(parse_num::<u16>(s)?)
}

/// 📦️ `TiffValues` — single-uppercase-letter tag prefix immediately followed by the bracketed
/// positional payload (same convention `svg`'s `enc_xml_node`/gif's enum codecs use): `B`=Byte,
/// `A`=Ascii, `S`=Short, `L`=Long, `R`=Rational, `E`=SByte, `U`=Undefined, `H`=SShort, `G`=SLong,
/// `Q`=SRational, `F`=Float, `D`=Double. `Byte`/`Undefined` (raw octets) and `Ascii` (text) are hex;
/// every numeric list is decimal comma-separated; `Rational`/`SRational` pairs nest as `[n,d]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_values(v: &TiffValues) -> String {
    match v {
        TiffValues::Byte(b) => format!("B[{}]", hex_encode(b)),
        TiffValues::Ascii(bytes) => format!("A[{}]", hex_encode(bytes)),
        TiffValues::Short(v) => format!("S{}", enc_list(v, |x| x.to_string())),
        TiffValues::Long(v) => format!("L{}", enc_list(v, |x| x.to_string())),
        TiffValues::Rational(v) => format!("R{}", enc_list(v, |(n, d)| format!("[{n},{d}]"))),
        TiffValues::SByte(v) => format!("E{}", enc_list(v, |x| x.to_string())),
        TiffValues::Undefined(b) => format!("U[{}]", hex_encode(b)),
        TiffValues::SShort(v) => format!("H{}", enc_list(v, |x| x.to_string())),
        TiffValues::SLong(v) => format!("G{}", enc_list(v, |x| x.to_string())),
        TiffValues::SRational(v) => format!("Q{}", enc_list(v, |(n, d)| format!("[{n},{d}]"))),
        TiffValues::Float(v) => format!("F{}", enc_list(v, |x| x.bits.to_string())),
        TiffValues::Double(v) => format!("D{}", enc_list(v, |x| x.bits.to_string())),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_values(s: &str) -> Result<TiffValues, String> {
    let (tag, rest) = s.split_at(1);
    let pair = |s: &str| -> Result<(u32, u32), String> {
        let parts = split_top_level(strip_brackets(s)?, ',');
        let [n, d] = parts.as_slice() else { return Err(format!("rational: expected 2 fields, got {}", parts.len())) };
        Ok((parse_num::<u32>(n)?, parse_num::<u32>(d)?))
    };
    let spair = |s: &str| -> Result<(i32, i32), String> {
        let parts = split_top_level(strip_brackets(s)?, ',');
        let [n, d] = parts.as_slice() else { return Err(format!("srational: expected 2 fields, got {}", parts.len())) };
        Ok((parse_num::<i32>(n)?, parse_num::<i32>(d)?))
    };
    match tag {
        "B" => Ok(TiffValues::Byte(hex_decode(strip_brackets(rest)?)?)),
        "A" => Ok(TiffValues::Ascii(hex_decode(strip_brackets(rest)?)?)),
        "S" => Ok(TiffValues::Short(dec_list(rest, parse_num::<u16>)?)),
        "L" => Ok(TiffValues::Long(dec_list(rest, parse_num::<u32>)?)),
        "R" => Ok(TiffValues::Rational(dec_list(rest, pair)?)),
        "E" => Ok(TiffValues::SByte(dec_list(rest, parse_num::<i8>)?)),
        "U" => Ok(TiffValues::Undefined(hex_decode(strip_brackets(rest)?)?)),
        "H" => Ok(TiffValues::SShort(dec_list(rest, parse_num::<i16>)?)),
        "G" => Ok(TiffValues::SLong(dec_list(rest, parse_num::<i32>)?)),
        "Q" => Ok(TiffValues::SRational(dec_list(rest, spair)?)),
        "F" => Ok(TiffValues::Float(dec_list(rest, |word| parse_num::<u32>(word).map(|bits| crate::schema::snapshot::TiffBinary32 { bits }))?)),
        "D" => Ok(TiffValues::Double(dec_list(rest, |word| parse_num::<u64>(word).map(|bits| crate::schema::snapshot::TiffBinary64 { bits }))?)),
        other => Err(format!("tiff values: unknown tag {other:?}")),
    }
}

/// 🏷️ One IFD entry: `[tag,values]`; the value variant owns the field type.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tag(t: &TiffTag) -> String {
    format!("[{},{}]", t.tag, enc_values(&t.values))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tag(s: &str) -> Result<TiffTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [tag, values] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(TiffTag { tag: parse_num::<u16>(tag)?, values: dec_values(values)? })
}

pub(crate) fn enc_storage(storage:&TiffStorage)->String {
    let kind=match storage.kind{TiffStorageKind::None=>0,TiffStorageKind::Strips=>1,TiffStorageKind::Tiles=>2};
    format!("[{kind},{},{},{}]",enc_field_type(storage.offsets_kind),enc_field_type(storage.byte_counts_kind),enc_list(&storage.chunks,|chunk|hex_encode(chunk)))
}

pub(crate) fn dec_storage(value:&str)->Result<TiffStorage,String>{
    let parts=split_top_level(strip_brackets(value)?,',');let[kind,offsets,counts,chunks]=parts.as_slice()else{return Err(format!("storage: expected 4 fields, got {}",parts.len()))};
    let kind=match *kind{"0"=>TiffStorageKind::None,"1"=>TiffStorageKind::Strips,"2"=>TiffStorageKind::Tiles,other=>return Err(format!("storage: unknown kind {other:?}"))};
    Ok(TiffStorage{kind,offsets_kind:dec_field_type(offsets)?,byte_counts_kind:dec_field_type(counts)?,chunks:dec_list(chunks,hex_decode)?})
}

/// 🗂️ One IFD: `[<entries-list>,<canonical-storage>]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifd(ifd: &TiffIfd) -> String {
    format!("[{},{}]", enc_list(&ifd.entries, enc_tag), enc_storage(&ifd.storage))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifd(s: &str) -> Result<TiffIfd, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [entries, storage] = parts.as_slice() else { return Err(format!("ifd: expected 2 fields, got {}", parts.len())) };
    Ok(TiffIfd { entries: dec_list(entries, dec_tag)?, storage: dec_storage(storage)? })
}

/// 🔺️ Tag-id-keyed `entries` triple: `[removed];[modified];[added]`, `modified`/`added` entries
/// are `tag:kind:values` (colon-separated — safe since `kind` is bare decimal and `values` never
/// contains a literal `:`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tags_diff(d: &TiffTagsDiff) -> String {
    let removed = d.removed.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.tag, enc_values(&m.values))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.tag, enc_values(&a.values))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tags_diff(body: &str) -> Result<TiffTagsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("tags diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_num::<u16>).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (tag_s, values_s) = entry.split_once(':').ok_or_else(|| format!("tag modified: bad entry {entry:?}"))?;
            Ok(TiffTagModified { tag: parse_num::<u16>(tag_s)?, values: dec_values(values_s)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (tag_s, values_s) = entry.split_once(':').ok_or_else(|| format!("tag added: bad entry {entry:?}"))?;
            Ok(TiffTagAdded { tag: parse_num::<u16>(tag_s)?, values: dec_values(values_s)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(TiffTagsDiff { removed, modified, added })
}

/// 🔺️ One IFD's own delta: `[<tags-triple>];<pixels>` — the bracketed (so `split_top_level(_, ';')`
/// sees it as ONE section) tag triple, then `-` for "strip bytes unchanged" or the new bytes as hex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifd_diff(d: &TiffIfdDiff) -> String {
    let storage = match &d.storage {
        Some(storage) => enc_storage(storage),
        None => "-".to_string(),
    };
    format!("[{}];{storage}", enc_tags_diff(&d.entries))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifd_diff(body: &str) -> Result<TiffIfdDiff, String> {
    let two = split_top_level(body, ';');
    let [entries_s, storage_s] = two.as_slice() else { return Err(format!("ifd diff: expected 2 sections, got {}", two.len())) };
    let storage = match *storage_s {
        "-" => None,
        other => Some(dec_storage(other)?),
    };
    Ok(TiffIfdDiff { entries: dec_tags_diff(strip_brackets(entries_s)?)?, storage })
}

/// 🗂️ Index-keyed `ifds` triple: `[removed];[modified];[added]`, `modified` entries are
/// `index:<ifd-diff>` (recursive), `added` entries are `index:<ifd>`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifds_diff(d: &TiffIfdsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_ifd_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_ifd(&a.ifd))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifds_diff(body: &str) -> Result<TiffIfdsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("ifds diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_num::<usize>).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("ifd modified: bad entry {entry:?}"))?;
            Ok(TiffIfdModified { index: parse_num::<usize>(idx)?, diff: dec_ifd_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("ifd added: bad entry {entry:?}"))?;
            Ok(TiffIfdAdded { index: parse_num::<usize>(idx)?, ifd: dec_ifd(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(TiffIfdsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_tiff_diff(d: &TiffDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.byte_order {
        tokens.push(format!("byte-order={}", enc_byte_order(v)));
    }
    if let Some(v) = &d.ifds {
        tokens.push(format!("ifds={}", enc_ifds_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_tiff_diff(line: &str) -> Result<TiffDiff, String> {
    let mut d = TiffDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("byte-order=") {
            d.byte_order = Some(dec_byte_order(rest)?);
        } else if let Some(rest) = token.strip_prefix("ifds=") {
            d.ifds = Some(dec_ifds_diff(rest)?);
        } else {
            return Err(format!("tiff diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for TiffDiff {
fn print_diff(&self) -> String {
    print_tiff_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_tiff_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v6_0::subsets::document::schema::diff::*;
use crate::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
}
pub use diff_wire_codec::*;
