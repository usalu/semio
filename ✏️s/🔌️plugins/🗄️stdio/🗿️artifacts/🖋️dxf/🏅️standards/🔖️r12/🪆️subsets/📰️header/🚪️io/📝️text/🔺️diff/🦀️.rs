//! 📝️ Text representation codec surface for `stdio.dxf` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DxfDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfArc, DxfCircle, DxfInsert, DxfLine, DxfOther, DxfPolyline, DxfSolid, DxfText, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::standards::v_r12::subsets::any::io::text::snapshot::entity_codec::*;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_var_diff(d: &DxfHeaderVarDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.group_code, |v| v.to_string()), encode_option(&d.value, enc_dxf_value), encode_option(&d.extra_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_var_diff(s: &str) -> Result<DxfHeaderVarDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [group_code, value, extra] = parts.as_slice() else { return Err(format!("header var diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfHeaderVarDiff { group_code: decode_option(group_code, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, value: decode_option(value, dec_dxf_value)?, extra_group_codes: decode_option(extra, dec_group_codes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layer_diff(d: &DxfLayerDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.color, |v| v.to_string()), encode_option(&d.linetype, |v| enc_str(v)), encode_option(&d.flags, |v| v.to_string()), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layer_diff(s: &str) -> Result<DxfLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color, linetype, flags, unknown] = parts.as_slice() else { return Err(format!("layer diff: expected 4 fields, got {}", parts.len())) };
    Ok(DxfLayerDiff {
        color: decode_option(color, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        linetype: decode_option(linetype, dec_str)?,
        flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        unknown_group_codes: decode_option(unknown, dec_group_codes)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_diff(d: &DxfStyleDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.flags, |v| v.to_string()), encode_option(&d.font_name, |v| enc_str(v)), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_diff(s: &str) -> Result<DxfStyleDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [flags, font_name, unknown] = parts.as_slice() else { return Err(format!("style diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfStyleDiff { flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, font_name: decode_option(font_name, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetype_diff(d: &DxfLinetypeDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.flags, |v| v.to_string()), encode_option(&d.description, |v| enc_str(v)), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetype_diff(s: &str) -> Result<DxfLinetypeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [flags, description, unknown] = parts.as_slice() else { return Err(format!("linetype diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfLinetypeDiff { flags: decode_option(flags, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, description: decode_option(description, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

/// 🌳 `DxfEntityDiff` itself needs a tag (like `SvgNodeDiff`/`XmlNode`) since it appears standalone
/// at the `entities=`/`blocks=`'s nested `modified` entry position, not always inside an
/// already-disambiguating container. `R`=Replace (carries a WHOLE `DxfEntity`, itself tag-prefixed
/// — a tag-inside-a-tag, unambiguous since the outer bracket always closes the inner one first),
/// else one letter per typed entity kind (same letters `enc_dxf_entity` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff(d: &DxfEntityDiff) -> String {
    match d {
        DxfEntityDiff::Replace { entity } => format!("R[{}]", enc_dxf_entity(entity)),
        DxfEntityDiff::Line(x) => format!("L[{},{},{},{}]", encode_option(&x.start, enc_point3), encode_option(&x.end, enc_point3), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Circle(x) => format!("C[{},{},{},{}]", encode_option(&x.center, enc_point3), encode_option(&x.radius, |v| enc_f64(*v)), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Arc(x) => format!(
            "A[{},{},{},{},{},{}]",
            encode_option(&x.center, enc_point3),
            encode_option(&x.radius, |v| enc_f64(*v)),
            encode_option(&x.start_angle, |v| enc_f64(*v)),
            encode_option(&x.end_angle, |v| enc_f64(*v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Polyline(x) => format!(
            "W[{},{},{},{}]",
            encode_option(&x.vertices, |v| enc_vertices(v)),
            encode_option(&x.closed, |v| if *v { "1".to_string() } else { "0".to_string() }),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Text(x) => format!(
            "T[{},{},{},{},{}]",
            encode_option(&x.position, enc_point3),
            encode_option(&x.height, |v| enc_f64(*v)),
            encode_option(&x.value, |v| enc_str(v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Solid(x) => format!("S[{},{},{}]", encode_option(&x.points, enc_points4), encode_option(&x.layer, |v| enc_str(v)), encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))),
        DxfEntityDiff::Insert(x) => format!(
            "I[{},{},{},{},{},{}]",
            encode_option(&x.block_name, |v| enc_str(v)),
            encode_option(&x.position, enc_point3),
            encode_option(&x.scale, enc_point3),
            encode_option(&x.rotation, |v| enc_f64(*v)),
            encode_option(&x.layer, |v| enc_str(v)),
            encode_option(&x.unknown_group_codes, |v| enc_group_codes(v))
        ),
        DxfEntityDiff::Other(x) => format!("O[{}]", encode_option(&x.group_codes, |v| enc_group_codes(v))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff(s: &str) -> Result<DxfEntityDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "R" => Ok(DxfEntityDiff::Replace { entity: dec_dxf_entity(inner)? }),
        "L" => {
            let parts = split_top_level(inner, ',');
            let [start, end, layer, unknown] = parts.as_slice() else { return Err(format!("line diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Line(DxfLineDiff { start: decode_option(start, dec_point3)?, end: decode_option(end, dec_point3)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "C" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, layer, unknown] = parts.as_slice() else { return Err(format!("circle diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Circle(DxfCircleDiff { center: decode_option(center, dec_point3)?, radius: decode_option(radius, dec_f64)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "A" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, start_angle, end_angle, layer, unknown] = parts.as_slice() else { return Err(format!("arc diff: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Arc(DxfArcDiff {
                center: decode_option(center, dec_point3)?,
                radius: decode_option(radius, dec_f64)?,
                start_angle: decode_option(start_angle, dec_f64)?,
                end_angle: decode_option(end_angle, dec_f64)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "W" => {
            let parts = split_top_level(inner, ',');
            let [vertices, closed, layer, unknown] = parts.as_slice() else { return Err(format!("polyline diff: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Polyline(DxfPolylineDiff {
                vertices: decode_option(vertices, dec_vertices)?,
                closed: decode_option(closed, |v| Ok(v == "1"))?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [position, height, value, layer, unknown] = parts.as_slice() else { return Err(format!("text diff: expected 5 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Text(DxfTextDiff {
                position: decode_option(position, dec_point3)?,
                height: decode_option(height, dec_f64)?,
                value: decode_option(value, dec_str)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [points, layer, unknown] = parts.as_slice() else { return Err(format!("solid diff: expected 3 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Solid(DxfSolidDiff { points: decode_option(points, dec_points4)?, layer: decode_option(layer, dec_str)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? }))
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [block_name, position, scale, rotation, layer, unknown] = parts.as_slice() else { return Err(format!("insert diff: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntityDiff::Insert(DxfInsertDiff {
                block_name: decode_option(block_name, dec_str)?,
                position: decode_option(position, dec_point3)?,
                scale: decode_option(scale, dec_point3)?,
                rotation: decode_option(rotation, dec_f64)?,
                layer: decode_option(layer, dec_str)?,
                unknown_group_codes: decode_option(unknown, dec_group_codes)?,
            }))
        }
        "O" => Ok(DxfEntityDiff::Other(DxfOtherDiff { group_codes: decode_option(inner, dec_group_codes)? })),
        other => Err(format!("entity diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff(d: &DxfBlockDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.name, |v| enc_str(v)), encode_option(&d.base_point, enc_point3), encode_option(&d.entities, enc_entities_diff), encode_option(&d.unknown_group_codes, |v| enc_group_codes(v)),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff(s: &str) -> Result<DxfBlockDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities, unknown] = parts.as_slice() else { return Err(format!("block diff: expected 4 fields, got {}", parts.len())) };
    Ok(DxfBlockDiff { name: decode_option(name, dec_str)?, base_point: decode_option(base_point, dec_point3)?, entities: decode_option(entities, dec_entities_diff)?, unknown_group_codes: decode_option(unknown, dec_group_codes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tables_diff(t: &DxfTablesDiff) -> String {
    format!("[{},{},{}]", encode_option(&t.layers, enc_layers_diff), encode_option(&t.styles, enc_styles_diff), encode_option(&t.linetypes, enc_linetypes_diff),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tables_diff(s: &str) -> Result<DxfTablesDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layers, styles, linetypes] = parts.as_slice() else { return Err(format!("tables diff: expected 3 fields, got {}", parts.len())) };
    Ok(DxfTablesDiff { layers: decode_option(layers, dec_layers_diff)?, styles: decode_option(styles, dec_styles_diff)?, linetypes: decode_option(linetypes, dec_linetypes_diff)? })
}

/// 🧮 Generic name-keyed `[removed];[modified];[added]` core — mirrors this file's own
/// `DxfNamedElem`/`named_between` generic core above, one level up (string grammar instead of
/// structural diff algebra). `modified` entries are `hexname:diffpayload`, `added` entries are
/// `index:fullitempayload` (recipe's own convention — the index is always a bare decimal
/// preceding the first colon, unambiguous since no payload anywhere in this grammar emits a
/// literal `:`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_name_triple<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed_s = removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified_s = modified.iter().map(|(n, d)| format!("{}:{}", enc_str(n), enc_diff(d))).collect::<Vec<_>>().join(",");
    let added_s = added.iter().map(|(i, t)| format!("{i}:{}", enc_item(t))).collect::<Vec<_>>().join(",");
    format!("[{removed_s}];[{modified_s}];[{added_s}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_name_triple<T, D>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<NamedDiffParts<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("name triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, rest) = entry.split_once(':').ok_or_else(|| format!("name triple modified: bad entry {entry:?}"))?;
            Ok((dec_str(name)?, dec_diff(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("name triple added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_item(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

/// 🧮 Generic index-keyed twin of [`enc_name_triple`]/[`dec_name_triple`] — used by
/// `entities`/`blocks` (both top-level and, for `entities`, nested inside a block diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_index_triple<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], enc_diff: impl Fn(&D) -> String, enc_item: impl Fn(&T) -> String) -> String {
    let removed_s = removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified_s = modified.iter().map(|(i, d)| format!("{i}:{}", enc_diff(d))).collect::<Vec<_>>().join(",");
    let added_s = added.iter().map(|(i, t)| format!("{i}:{}", enc_item(t))).collect::<Vec<_>>().join(",");
    format!("[{removed_s}];[{modified_s}];[{added_s}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_index_triple<T, D>(body: &str, dec_diff: impl Fn(&str) -> Result<D, String>, dec_item: impl Fn(&str) -> Result<T, String>) -> Result<IndexedDiffParts<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("index triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("index triple modified: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_diff(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("index triple added: bad entry {entry:?}"))?;
            Ok((parse_usize(idx)?, dec_item(rest)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_header_vars_diff(d: &DxfHeaderVarsDiff) -> String {
    let modified: Vec<(String, DxfHeaderVarDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfHeaderVar)> = d.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_header_var_diff, enc_header_var)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_header_vars_diff(s: &str) -> Result<DxfHeaderVarsDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_header_var_diff, dec_header_var)?;
    Ok(DxfHeaderVarsDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layers_diff(d: &DxfLayersDiff) -> String {
    let modified: Vec<(String, DxfLayerDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLayer)> = d.added.iter().map(|a| (a.index, a.layer.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_layer_diff, enc_layer)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layers_diff(s: &str) -> Result<DxfLayersDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_layer_diff, dec_layer)?;
    Ok(DxfLayersDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_styles_diff(d: &DxfStylesDiff) -> String {
    let modified: Vec<(String, DxfStyleDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfStyle)> = d.added.iter().map(|a| (a.index, a.style.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_style_diff, enc_style)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_styles_diff(s: &str) -> Result<DxfStylesDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_style_diff, dec_style)?;
    Ok(DxfStylesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_linetypes_diff(d: &DxfLinetypesDiff) -> String {
    let modified: Vec<(String, DxfLinetypeDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLinetype)> = d.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
    enc_name_triple(&d.removed, &modified, &added, enc_linetype_diff, enc_linetype)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_linetypes_diff(s: &str) -> Result<DxfLinetypesDiff, String> {
    let (removed, modified, added) = dec_name_triple(s, dec_linetype_diff, dec_linetype)?;
    Ok(DxfLinetypesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff(d: &DxfEntitiesDiff) -> String {
    let modified: Vec<(usize, DxfEntityDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfEntity)> = d.added.iter().map(|a| (a.index, a.entity.clone())).collect();
    enc_index_triple(&d.removed, &modified, &added, enc_entity_diff, enc_dxf_entity)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff(s: &str) -> Result<DxfEntitiesDiff, String> {
    let (removed, modified, added) = dec_index_triple(s, dec_entity_diff, dec_dxf_entity)?;
    Ok(DxfEntitiesDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff(d: &DxfBlocksDiff) -> String {
    let modified: Vec<(usize, DxfBlockDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfBlock)> = d.added.iter().map(|a| (a.index, a.block.clone())).collect();
    enc_index_triple(&d.removed, &modified, &added, enc_block_diff, enc_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff(s: &str) -> Result<DxfBlocksDiff, String> {
    let (removed, modified, added) = dec_index_triple(s, dec_block_diff, dec_block)?;
    Ok(DxfBlocksDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_dxf_diff(d: &DxfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.header_vars {
        tokens.push(format!("header-vars={}", enc_header_vars_diff(v)));
    }
    if let Some(v) = &d.tables {
        tokens.push(format!("tables={}", enc_tables_diff(v)));
    }
    if let Some(v) = &d.blocks {
        tokens.push(format!("blocks={}", enc_blocks_diff(v)));
    }
    if let Some(v) = &d.entities {
        tokens.push(format!("entities={}", enc_entities_diff(v)));
    }
    if let Some(v) = &d.other_tables {
        tokens.push(format!("other-tables={}", enc_list(v, enc_other_table)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_dxf_diff(line: &str) -> Result<DxfDiff, String> {
    let mut d = DxfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("header-vars=") {
            d.header_vars = Some(dec_header_vars_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("tables=") {
            d.tables = Some(dec_tables_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_blocks_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("entities=") {
            d.entities = Some(dec_entities_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("other-tables=") {
            d.other_tables = Some(dec_list(rest, dec_other_table)?);
        } else {
            return Err(format!("dxf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for DxfDiff {
fn print_diff(&self) -> String {
    print_dxf_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_dxf_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
