//! 🧬️ Raster snapshot schema — artifact-lane fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `RasterSnapshot.assets` carries real `store::ArtifactChild<SemioImageSnapshot>` handles (composed
//! `s.stdio.semio.image` children, one per asset id — see `🗿️artifacts/🖨️raster/🦀️.rs`'s
//! `🧩️Composition` region), which `dsl::DslRecord`'s derive cannot represent (no `DslField` impl for
//! `ArtifactChild<S>`) — the same reason `✳️object`/`✳️kit` (stdio) and `💠️lowpoly`/`🗺️gismap` (this
//! ticket's own exemplars) hand-roll their own codecs rather than deriving. This file follows their
//! exact hex/bracket convention, never a hand-written slot list — `#[derive(ArtifactSchema)]` still
//! emits `field_states()` for the top-level facets. `child_slots()` is honestly EMPTY for `assets`:
//! the derive's `#[child(kind=...)]` mechanism (`🧬️schema/✨️derive/🦀️.rs`) only recognizes a
//! bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field directly on the struct, not an owned-map
//! value — kept as a `RasterOwnedMap<ArtifactChild<S>>` anyway (rather than reshaping to a `Vec`) to
//! preserve the SAME id-keyed addressing `image_key: Option<String>` already used pre-migration and
//! every existing `add-layer-asset`/`remove-layer-asset` mutation already assumes. This is the exact
//! same already-accepted shape of gap `💠️lowpoly`'s own `LowpolyObject.mesh` doc comment documents (a
//! nested/non-bare child slot the derive can't see): the type/mutation/persistence layer is fully
//! real, only the derive-generated SCHEMA INTROSPECTION table is incomplete for this one field.

use crate::artifacts::raster::{RasterAssetChild, RasterLayerMask, RasterLayerNode, RasterOwnedMap, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted raster document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    pub layers: Vec<RasterLayerNode>,
    #[state(artifact)]
    #[serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map")]
    #[value(default, skip_serializing_if = "RasterOwnedMap::is_empty")]
    #[serde(default, skip_serializing_if = "RasterOwnedMap::is_empty")]
    pub assets: RasterOwnedMap<RasterAssetChild>,
}

pub(crate) const RASTER_POPULATED_OUTPUT_ERROR: &str = "Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority";

impl RasterSnapshot {
    /// 🛡️ Admits only the constant-size empty snapshot shell to legacy whole-output codecs.
    pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str> {
        if self.layers.is_empty() && self.assets.is_empty() {
            Ok(())
        } else {
            Err(RASTER_POPULATED_OUTPUT_ERROR)
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️CodecPrimitives
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};

pub(crate) fn enc_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn dec_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option: bad shape {other:?}")),
    }
}
pub(crate) fn enc_opt_str(s: &Option<String>) -> String {
    enc_option(s, |v| enc_str(v))
}
pub(crate) fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    dec_option(s, dec_str)
}
pub(crate) fn enc_opt_u32(v: &Option<u32>) -> String {
    enc_option(v, |n| n.to_string())
}
pub(crate) fn dec_opt_u32(s: &str) -> Result<Option<u32>, String> {
    dec_option(s, |n| n.parse::<u32>().map_err(|e: std::num::ParseIntError| e.to_string()))
}

pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

/// 🪪️ `[<hex child_id>,<hex target-uri>]` — the two-string handle, real and complete, never content.
pub(crate) fn enc_child(c: &RasterAssetChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
pub(crate) fn dec_child(s: &str) -> Result<RasterAssetChild, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}

pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String {
    assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");
    "[]".to_string()
}
pub(crate) fn dec_asset_map(s: &str) -> Result<RasterOwnedMap<RasterAssetChild>, String> {
    let mut out = RasterOwnedMap::new();
    for entry in split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()) {
        let parts = split_top_level(strip_brackets(entry)?, ',');
        let [key, child] = parts.as_slice() else { return Err(format!("asset map entry: expected 2 fields, got {}", parts.len())) };
        out.insert(dec_str(key)?, dec_child(child)?).map_err(|rejected| rejected.reason.to_string())?;
    }
    Ok(out)
}

pub(crate) fn enc_transform(t: &RasterTransform) -> String {
    format!("[{},{},{},{},{}]", t.x, t.y, t.scale_x, t.scale_y, t.rotation)
}
pub(crate) fn dec_transform(s: &str) -> Result<RasterTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, sx, sy, rot] = parts.as_slice() else { return Err(format!("transform: expected 5 fields, got {}", parts.len())) };
    let f = |s: &str| s.trim().parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());
    Ok(RasterTransform { x: f(x)?, y: f(y)?, scale_x: f(sx)?, scale_y: f(sy)?, rotation: f(rot)? })
}

pub(crate) fn enc_mask(m: &RasterLayerMask) -> String {
    format!("[{},{},{},{},{}]", m.enabled, m.linked, m.invert, enc_opt_u32(&m.width), enc_opt_u32(&m.height))
}
pub(crate) fn dec_mask(s: &str) -> Result<RasterLayerMask, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [enabled, linked, invert, width, height] = parts.as_slice() else { return Err(format!("mask: expected 5 fields, got {}", parts.len())) };
    let b = |s: &str| s.trim().parse::<bool>().map_err(|e: std::str::ParseBoolError| e.to_string());
    Ok(RasterLayerMask { enabled: b(enabled)?, linked: b(linked)?, invert: b(invert)?, width: dec_opt_u32(width)?, height: dec_opt_u32(height)? })
}
pub(crate) fn enc_mask_opt(m: &Option<RasterLayerMask>) -> String {
    enc_option(m, enc_mask)
}
pub(crate) fn dec_mask_opt(s: &str) -> Result<Option<RasterLayerMask>, String> {
    dec_option(s, dec_mask)
}

/// 🧬️ Empty parameter-map shell for the legacy codec; populated output requires retained paging.
pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String {
    assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");
    "[]".to_string()
}
pub(crate) fn dec_params(s: &str) -> Result<RasterOwnedMap<dsl::DslValue>, String> {
    let mut out = RasterOwnedMap::new();
    for entry in split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()) {
        let parts = split_top_level(strip_brackets(entry)?, ',');
        let [key, value] = parts.as_slice() else { return Err(format!("params entry: expected 2 fields, got {}", parts.len())) };
        let bytes = hex_decode(value)?;
        let dsl_value: dsl::DslValue = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        out.insert(dec_str(key)?, dsl_value).map_err(|rejected| rejected.reason.to_string())?;
    }
    Ok(out)
}

/// 🌳️ Recursive layer-tree codec — one tag char (`p`/`g`/`a`) prefixed directly onto the bracketed
/// field list (`split_top_level`'s depth tracking only keys off `[`/`]`, so the leading tag never
/// confuses top-level list splitting).
pub(crate) fn enc_layer(layer: &RasterLayerNode) -> String {
    match layer {
        RasterLayerNode::Pixel { id, name, visible, opacity, blend_mode, transform, mask, width, height, image_key } => {
            format!("p[{},{},{},{},{},{},{},{},{},{}]", enc_str(id), enc_str(name), visible, opacity, enc_str(blend_mode), enc_transform(transform), enc_mask_opt(mask), enc_opt_u32(width), enc_opt_u32(height), enc_opt_str(image_key),)
        }
        RasterLayerNode::Group { id, name, visible, opacity, blend_mode, transform, mask, children } => {
            format!("g[{},{},{},{},{},{},{},{}]", enc_str(id), enc_str(name), visible, opacity, enc_str(blend_mode), enc_transform(transform), enc_mask_opt(mask), enc_layer_list(children),)
        }
        RasterLayerNode::Adjustment { id, name, visible, opacity, blend_mode, transform, adjustment_kind, params } => {
            format!("a[{},{},{},{},{},{},{},{}]", enc_str(id), enc_str(name), visible, opacity, enc_str(blend_mode), enc_transform(transform), enc_str(adjustment_kind), enc_params(params),)
        }
    }
}
pub(crate) fn dec_layer(s: &str) -> Result<RasterLayerNode, String> {
    if s.is_empty() {
        return Err("layer: empty".into());
    }
    let (tag, rest) = s.split_at(1);
    let parts = split_top_level(strip_brackets(rest)?, ',');
    match tag {
        "p" => {
            let [id, name, visible, opacity, blend_mode, transform, mask, width, height, image_key] = parts.as_slice() else {
                return Err(format!("pixel layer: expected 10 fields, got {}", parts.len()));
            };
            Ok(RasterLayerNode::Pixel {
                id: dec_str(id)?,
                name: dec_str(name)?,
                visible: visible.trim().parse().map_err(|e: std::str::ParseBoolError| e.to_string())?,
                opacity: opacity.trim().parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
                blend_mode: dec_str(blend_mode)?,
                transform: dec_transform(transform)?,
                mask: dec_mask_opt(mask)?,
                width: dec_opt_u32(width)?,
                height: dec_opt_u32(height)?,
                image_key: dec_opt_str(image_key)?,
            })
        }
        "g" => {
            let [id, name, visible, opacity, blend_mode, transform, mask, children] = parts.as_slice() else {
                return Err(format!("group layer: expected 8 fields, got {}", parts.len()));
            };
            Ok(RasterLayerNode::Group {
                id: dec_str(id)?,
                name: dec_str(name)?,
                visible: visible.trim().parse().map_err(|e: std::str::ParseBoolError| e.to_string())?,
                opacity: opacity.trim().parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
                blend_mode: dec_str(blend_mode)?,
                transform: dec_transform(transform)?,
                mask: dec_mask_opt(mask)?,
                children: dec_layer_list(children)?,
            })
        }
        "a" => {
            let [id, name, visible, opacity, blend_mode, transform, adjustment_kind, params] = parts.as_slice() else {
                return Err(format!("adjustment layer: expected 8 fields, got {}", parts.len()));
            };
            Ok(RasterLayerNode::Adjustment {
                id: dec_str(id)?,
                name: dec_str(name)?,
                visible: visible.trim().parse().map_err(|e: std::str::ParseBoolError| e.to_string())?,
                opacity: opacity.trim().parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
                blend_mode: dec_str(blend_mode)?,
                transform: dec_transform(transform)?,
                adjustment_kind: dec_str(adjustment_kind)?,
                params: dec_params(params)?,
            })
        }
        other => Err(format!("layer: unknown tag {other:?}")),
    }
}
pub(crate) fn enc_layer_list(list: &[RasterLayerNode]) -> String {
    assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");
    "[]".to_string()
}
pub(crate) fn dec_layer_list(s: &str) -> Result<Vec<RasterLayerNode>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_layer).collect()
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives
fn print_raster_snapshot_body(s: &RasterSnapshot) -> String {
    s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);
    format!("schema={}\nid={}\ntitle={}\nlayers={}\nassets={}", enc_str(&s.schema), enc_str(&s.id), enc_opt_str(&s.title), enc_layer_list(&s.layers), enc_asset_map(&s.assets),)
}
fn parse_raster_snapshot_body(body: &str) -> Result<RasterSnapshot, String> {
    let mut schema = None;
    let mut id = None;
    let mut title = None;
    let mut layers = Vec::new();
    let mut assets = RasterOwnedMap::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("id=") {
            id = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("title=") {
            title = dec_opt_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("layers=") {
            layers = dec_layer_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("assets=") {
            assets = dec_asset_map(rest)?;
        } else {
            return Err(format!("raster snapshot: unknown line {line:?}"));
        }
    }
    Ok(RasterSnapshot { schema: schema.ok_or_else(|| "raster snapshot: missing schema line".to_string())?, id: id.ok_or_else(|| "raster snapshot: missing id line".to_string())?, title, layers, assets })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

fn write_opt<T>(out: &mut Vec<u8>, v: &Option<T>, write: impl Fn(&mut Vec<u8>, &T)) {
    match v {
        Some(x) => {
            out.push(1);
            write(out, x);
        }
        None => out.push(0),
    }
}
fn read_opt<T>(reader: &mut store::ByteReader<'_>, read: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read(reader)?)),
        other => Err(format!("option: bad presence tag {other}")),
    }
}
fn write_opt_str(out: &mut Vec<u8>, v: &Option<String>) {
    write_opt(out, v, |out, s| write_str_lp(out, s));
}
fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    read_opt(reader, read_str_lp)
}
fn write_opt_u32(out: &mut Vec<u8>, v: &Option<u32>) {
    write_opt(out, v, |out, n| out.extend_from_slice(&n.to_le_bytes()));
}
fn read_opt_u32(reader: &mut store::ByteReader<'_>) -> Result<Option<u32>, String> {
    read_opt(reader, |r| Ok(u32::from_le_bytes(r.read_bytes(4).map_err(|e| e.to_string())?.try_into().map_err(|_| "u32: short read".to_string())?)))
}

fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child(out: &mut Vec<u8>, c: &RasterAssetChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child(reader: &mut store::ByteReader<'_>) -> Result<RasterAssetChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>) {
    assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");
    store::pack_rt::write_varint_u64(out, 0);
}
fn read_asset_map(reader: &mut store::ByteReader<'_>) -> Result<RasterOwnedMap<RasterAssetChild>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = RasterOwnedMap::new();
    for _ in 0..count {
        let k = read_str_lp(reader)?;
        let v = read_child(reader)?;
        out.insert(k, v).map_err(|rejected| rejected.reason.to_string())?;
    }
    Ok(out)
}

fn write_transform(out: &mut Vec<u8>, t: &RasterTransform) {
    for v in [t.x, t.y, t.scale_x, t.scale_y, t.rotation] {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<RasterTransform, String> {
    let mut next = || -> Result<f64, String> { Ok(f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "transform: short read".to_string())?)) };
    Ok(RasterTransform { x: next()?, y: next()?, scale_x: next()?, scale_y: next()?, rotation: next()? })
}

fn write_mask(out: &mut Vec<u8>, m: &RasterLayerMask) {
    out.push(m.enabled as u8);
    out.push(m.linked as u8);
    out.push(m.invert as u8);
    write_opt_u32(out, &m.width);
    write_opt_u32(out, &m.height);
}
fn read_mask(reader: &mut store::ByteReader<'_>) -> Result<RasterLayerMask, String> {
    let enabled = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let linked = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let invert = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let width = read_opt_u32(reader)?;
    let height = read_opt_u32(reader)?;
    Ok(RasterLayerMask { enabled, linked, invert, width, height })
}
fn write_mask_opt(out: &mut Vec<u8>, m: &Option<RasterLayerMask>) {
    write_opt(out, m, write_mask);
}
fn read_mask_opt(reader: &mut store::ByteReader<'_>) -> Result<Option<RasterLayerMask>, String> {
    read_opt(reader, read_mask)
}

fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>) {
    assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");
    store::pack_rt::write_varint_u64(out, 0);
}
fn read_params(reader: &mut store::ByteReader<'_>) -> Result<RasterOwnedMap<dsl::DslValue>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = RasterOwnedMap::new();
    for _ in 0..count {
        let k = read_str_lp(reader)?;
        let bytes = read_bytes_lp(reader)?;
        let v: dsl::DslValue = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        out.insert(k, v).map_err(|rejected| rejected.reason.to_string())?;
    }
    Ok(out)
}

fn write_layer(out: &mut Vec<u8>, layer: &RasterLayerNode) {
    match layer {
        RasterLayerNode::Pixel { id, name, visible, opacity, blend_mode, transform, mask, width, height, image_key } => {
            out.push(0);
            write_str_lp(out, id);
            write_str_lp(out, name);
            out.push(*visible as u8);
            out.extend_from_slice(&opacity.to_le_bytes());
            write_str_lp(out, blend_mode);
            write_transform(out, transform);
            write_mask_opt(out, mask);
            write_opt_u32(out, width);
            write_opt_u32(out, height);
            write_opt_str(out, image_key);
        }
        RasterLayerNode::Group { id, name, visible, opacity, blend_mode, transform, mask, children } => {
            out.push(1);
            write_str_lp(out, id);
            write_str_lp(out, name);
            out.push(*visible as u8);
            out.extend_from_slice(&opacity.to_le_bytes());
            write_str_lp(out, blend_mode);
            write_transform(out, transform);
            write_mask_opt(out, mask);
            write_layer_list(out, children);
        }
        RasterLayerNode::Adjustment { id, name, visible, opacity, blend_mode, transform, adjustment_kind, params } => {
            out.push(2);
            write_str_lp(out, id);
            write_str_lp(out, name);
            out.push(*visible as u8);
            out.extend_from_slice(&opacity.to_le_bytes());
            write_str_lp(out, blend_mode);
            write_transform(out, transform);
            write_str_lp(out, adjustment_kind);
            write_params(out, params);
        }
    }
}
fn read_layer(reader: &mut store::ByteReader<'_>) -> Result<RasterLayerNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let id = read_str_lp(reader)?;
    let name = read_str_lp(reader)?;
    let visible = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let opacity = f32::from_le_bytes(reader.read_bytes(4).map_err(|e| e.to_string())?.try_into().map_err(|_| "opacity: short read".to_string())?);
    let blend_mode = read_str_lp(reader)?;
    let transform = read_transform(reader)?;
    match tag {
        0 => {
            let mask = read_mask_opt(reader)?;
            let width = read_opt_u32(reader)?;
            let height = read_opt_u32(reader)?;
            let image_key = read_opt_str(reader)?;
            Ok(RasterLayerNode::Pixel { id, name, visible, opacity, blend_mode, transform, mask, width, height, image_key })
        }
        1 => {
            let mask = read_mask_opt(reader)?;
            let children = read_layer_list(reader)?;
            Ok(RasterLayerNode::Group { id, name, visible, opacity, blend_mode, transform, mask, children })
        }
        2 => {
            let adjustment_kind = read_str_lp(reader)?;
            let params = read_params(reader)?;
            Ok(RasterLayerNode::Adjustment { id, name, visible, opacity, blend_mode, transform, adjustment_kind, params })
        }
        other => Err(format!("layer: unknown tag {other}")),
    }
}
fn write_layer_list(out: &mut Vec<u8>, list: &[RasterLayerNode]) {
    assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");
    store::pack_rt::write_varint_u64(out, 0);
}
fn read_layer_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<RasterLayerNode>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_layer(reader)).collect()
}

fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.id);
    write_opt_str(&mut out, &s.title);
    write_layer_list(&mut out, &s.layers);
    write_asset_map(&mut out, &s.assets);
    out
}
fn decode_raster_snapshot_binary(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let id = read_str_lp(&mut reader)?;
    let title = read_opt_str(&mut reader)?;
    let layers = read_layer_list(&mut reader)?;
    let assets = read_asset_map(&mut reader)?;
    Ok(RasterSnapshot { schema, id, title, layers, assets })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Empty-shell ArtifactDsl/ArtifactPack bridge; populated snapshots require retained paging.
impl store::ArtifactDsl for RasterSnapshot {
    const EXTENSION: &'static str = "raster";
    fn envelope_id() -> &'static str {
        "raster.raster"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_raster_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);
        let body = print_raster_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for RasterSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;
        let raw = encode_raster_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_raster_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Defaults
impl Default for RasterSnapshot {
    fn default() -> Self {
        Self { schema: RASTER_DOCUMENT_SCHEMA.into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() }
    }
}
//#endregion 🔖️Defaults
