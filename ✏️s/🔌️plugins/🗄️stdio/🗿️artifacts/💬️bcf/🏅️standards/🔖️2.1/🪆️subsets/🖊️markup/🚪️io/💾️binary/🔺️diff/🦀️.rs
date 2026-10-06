//! bcf rep for stdio.bcf 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_1::subsets::any::schema::diff::*;
use crate::schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility};
use crate::BcfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_opt_str(out: &mut Vec<u8>, opt: &Option<String>) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        write_str_lp(out, v);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    Ok(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None })
}

/// 🌳️ Full-item (non-diff) binary codecs, mirrored one-for-one against `../🔖️ValueCodecs`'s text
/// forms above. `pub(crate)` so `../🧬️mutations/🦀️.rs` reuses these rather than
/// re-deriving its own copies (same intra-artifact reuse pattern the text codecs already use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3_bin(p: &BcfPoint3, out: &mut Vec<u8>) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfPoint3, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    let z = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(BcfPoint3 { x, y, z })
}

/// 🌳️ `0`=Perspective / `1`=Orthogonal -- binary twin of `enc_camera`/`dec_camera`'s `P`/`O` tags.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_camera_bin(c: &BcfCamera, out: &mut Vec<u8>) {
    match c {
        BcfCamera::Perspective { view_point, direction, up_vector, field_of_view } => {
            out.push(0);
            enc_point3_bin(view_point, out);
            enc_point3_bin(direction, out);
            enc_point3_bin(up_vector, out);
            out.extend_from_slice(&field_of_view.to_le_bytes());
        }
        BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => {
            out.push(1);
            enc_point3_bin(view_point, out);
            enc_point3_bin(direction, out);
            enc_point3_bin(up_vector, out);
            out.extend_from_slice(&view_to_world_scale.to_le_bytes());
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_camera_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfCamera, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(BcfCamera::Perspective { view_point: dec_point3_bin(reader)?, direction: dec_point3_bin(reader)?, up_vector: dec_point3_bin(reader)?, field_of_view: reader.read_f64_le().map_err(|e| e.to_string())? }),
        1 => Ok(BcfCamera::Orthogonal { view_point: dec_point3_bin(reader)?, direction: dec_point3_bin(reader)?, up_vector: dec_point3_bin(reader)?, view_to_world_scale: reader.read_f64_le().map_err(|e| e.to_string())? }),
        other => Err(format!("camera binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str_list_bin(items: &[String], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for s in items {
        write_str_lp(out, s);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<String>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(read_str_lp(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_visibility_bin(v: &BcfVisibility, out: &mut Vec<u8>) {
    out.push(v.default_visibility as u8);
    enc_str_list_bin(&v.exceptions, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_visibility_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfVisibility, String> {
    let default_visibility = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let exceptions = dec_str_list_bin(reader)?;
    Ok(BcfVisibility { default_visibility, exceptions })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_coloring_bin(c: &BcfColoring, out: &mut Vec<u8>) {
    write_str_lp(out, &c.color);
    enc_str_list_bin(&c.components, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_coloring_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfColoring, String> {
    let color = read_str_lp(reader)?;
    let components = dec_str_list_bin(reader)?;
    Ok(BcfColoring { color, components })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_components_bin(c: &BcfComponents, out: &mut Vec<u8>) {
    enc_str_list_bin(&c.selection, out);
    enc_visibility_bin(&c.visibility, out);
    store::pack_rt::write_varint_u64(out, c.coloring.len() as u64);
    for entry in &c.coloring {
        enc_coloring_bin(entry, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_components_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfComponents, String> {
    let selection = dec_str_list_bin(reader)?;
    let visibility = dec_visibility_bin(reader)?;
    let coloring_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut coloring = Vec::with_capacity(coloring_count as usize);
    for _ in 0..coloring_count {
        coloring.push(dec_coloring_bin(reader)?);
    }
    Ok(BcfComponents { selection, visibility, coloring })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment_bin(c: &BcfComment, out: &mut Vec<u8>) {
    write_str_lp(out, &c.guid);
    write_str_lp(out, &c.date);
    write_str_lp(out, &c.author);
    write_str_lp(out, &c.text);
    write_opt_str(out, &c.viewpoint_ref);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfComment, String> {
    let guid = read_str_lp(reader)?;
    let date = read_str_lp(reader)?;
    let author = read_str_lp(reader)?;
    let text = read_str_lp(reader)?;
    let viewpoint_ref = read_opt_str(reader)?;
    Ok(BcfComment { guid, date, author, text, viewpoint_ref })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint_bin(v: &BcfViewpoint, out: &mut Vec<u8>) {
    write_str_lp(out, &v.guid);
    out.push(if v.camera.is_some() { 1 } else { 0 });
    if let Some(camera) = &v.camera {
        enc_camera_bin(camera, out);
    }
    out.push(if v.components.is_some() { 1 } else { 0 });
    if let Some(components) = &v.components {
        enc_components_bin(components, out);
    }
    out.push(if v.snapshot.is_some() { 1 } else { 0 });
    if let Some(snapshot) = &v.snapshot {
        write_bytes_lp(out, snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfViewpoint, String> {
    let guid = read_str_lp(reader)?;
    let camera = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_camera_bin(reader)?) } else { None };
    let components = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_components_bin(reader)?) } else { None };
    let snapshot = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(BcfViewpoint { guid, camera, components, snapshot })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic_bin(t: &BcfTopic, out: &mut Vec<u8>) {
    write_str_lp(out, &t.guid);
    write_str_lp(out, &t.title);
    write_str_lp(out, &t.description);
    write_str_lp(out, &t.status);
    write_str_lp(out, &t.priority);
    enc_str_list_bin(&t.labels, out);
    write_str_lp(out, &t.creation_date);
    write_str_lp(out, &t.creation_author);
    store::pack_rt::write_varint_u64(out, t.comments.len() as u64);
    for c in &t.comments {
        enc_comment_bin(c, out);
    }
    store::pack_rt::write_varint_u64(out, t.viewpoints.len() as u64);
    for v in &t.viewpoints {
        enc_viewpoint_bin(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfTopic, String> {
    let guid = read_str_lp(reader)?;
    let title = read_str_lp(reader)?;
    let description = read_str_lp(reader)?;
    let status = read_str_lp(reader)?;
    let priority = read_str_lp(reader)?;
    let labels = dec_str_list_bin(reader)?;
    let creation_date = read_str_lp(reader)?;
    let creation_author = read_str_lp(reader)?;
    let comment_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut comments = Vec::with_capacity(comment_count as usize);
    for _ in 0..comment_count {
        comments.push(dec_comment_bin(reader)?);
    }
    let viewpoint_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut viewpoints = Vec::with_capacity(viewpoint_count as usize);
    for _ in 0..viewpoint_count {
        viewpoints.push(dec_viewpoint_bin(reader)?);
    }
    Ok(BcfTopic { guid, title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part_bin(p: &BcfRawPart, out: &mut Vec<u8>) {
    write_str_lp(out, &p.name);
    write_bytes_lp(out, &p.data);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfRawPart, String> {
    let name = read_str_lp(reader)?;
    let data = read_bytes_lp(reader)?;
    Ok(BcfRawPart { name, data })
}

/// 🏷️ Binary twin of `enc_named_triple`/`dec_named_triple` -- three varint-counted sections
/// (removed keys / modified key+diff pairs / added whole items), generic over `K`/`D`/`T`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_triple_bin<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K, &mut Vec<u8>), enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, triple.removed.len() as u64);
    for k in &triple.removed {
        enc_k(k, out);
    }
    store::pack_rt::write_varint_u64(out, triple.modified.len() as u64);
    for m in &triple.modified {
        enc_k(&m.key, out);
        enc_d(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, triple.added.len() as u64);
    for t in &triple.added {
        enc_t(t, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_triple_bin<K, D, T>(
    reader: &mut store::ByteReader<'_>,
    dec_k: impl Fn(&mut store::ByteReader<'_>) -> Result<K, String>,
    dec_d: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_t: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<NamedTripleDiff<K, D, T>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(dec_k(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = dec_k(reader)?;
        let diff = dec_d(reader)?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        added.push(dec_t(reader)?);
    }
    Ok(NamedTripleDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment_diff_bin(d: &BcfCommentDiff, out: &mut Vec<u8>) {
    write_opt_str(out, &d.date);
    write_opt_str(out, &d.author);
    write_opt_str(out, &d.text);
    out.push(if d.viewpoint_ref.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.viewpoint_ref {
        write_opt_str(out, inner);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfCommentDiff, String> {
    let date = read_opt_str(reader)?;
    let author = read_opt_str(reader)?;
    let text = read_opt_str(reader)?;
    let viewpoint_ref = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_opt_str(reader)?) } else { None };
    Ok(BcfCommentDiff { date, author, text, viewpoint_ref })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint_diff_bin(d: &BcfViewpointDiff, out: &mut Vec<u8>) {
    out.push(if d.camera.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.camera {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(camera) = inner {
            enc_camera_bin(camera, out);
        }
    }
    out.push(if d.components.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.components {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(components) = inner {
            enc_components_bin(components, out);
        }
    }
    out.push(if d.snapshot.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.snapshot {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(snapshot) = inner {
            write_bytes_lp(out, snapshot);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfViewpointDiff, String> {
    let camera = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_camera_bin(reader)?) } else { None }) } else { None };
    let components = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_components_bin(reader)?) } else { None }) } else { None };
    let snapshot = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None }) } else { None };
    Ok(BcfViewpointDiff { camera, components, snapshot })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part_diff_bin(d: &BcfPartDiff, out: &mut Vec<u8>) {
    out.push(if d.data.is_some() { 1 } else { 0 });
    if let Some(v) = &d.data {
        write_bytes_lp(out, v);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfPartDiff, String> {
    let data = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(BcfPartDiff { data })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comments_diff_bin(d: &BcfCommentsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_comment_diff_bin, enc_comment_bin, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comments_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfCommentsDiff, String> {
    dec_named_triple_bin(reader, read_str_lp, dec_comment_diff_bin, dec_comment_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoints_diff_bin(d: &BcfViewpointsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_viewpoint_diff_bin, enc_viewpoint_bin, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoints_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfViewpointsDiff, String> {
    dec_named_triple_bin(reader, read_str_lp, dec_viewpoint_diff_bin, dec_viewpoint_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic_diff_bin(d: &BcfTopicDiff, out: &mut Vec<u8>) {
    write_opt_str(out, &d.title);
    write_opt_str(out, &d.description);
    write_opt_str(out, &d.status);
    write_opt_str(out, &d.priority);
    out.push(if d.labels.is_some() { 1 } else { 0 });
    if let Some(v) = &d.labels {
        enc_str_list_bin(v, out);
    }
    write_opt_str(out, &d.creation_date);
    write_opt_str(out, &d.creation_author);
    out.push(if d.comments.is_some() { 1 } else { 0 });
    if let Some(v) = &d.comments {
        enc_comments_diff_bin(v, out);
    }
    out.push(if d.viewpoints.is_some() { 1 } else { 0 });
    if let Some(v) = &d.viewpoints {
        enc_viewpoints_diff_bin(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfTopicDiff, String> {
    let title = read_opt_str(reader)?;
    let description = read_opt_str(reader)?;
    let status = read_opt_str(reader)?;
    let priority = read_opt_str(reader)?;
    let labels = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_str_list_bin(reader)?) } else { None };
    let creation_date = read_opt_str(reader)?;
    let creation_author = read_opt_str(reader)?;
    let comments = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_comments_diff_bin(reader)?) } else { None };
    let viewpoints = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_viewpoints_diff_bin(reader)?) } else { None };
    Ok(BcfTopicDiff { title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topics_diff_bin(d: &BcfTopicsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_topic_diff_bin, enc_topic_bin, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topics_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfTopicsDiff, String> {
    dec_named_triple_bin(reader, read_str_lp, dec_topic_diff_bin, dec_topic_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_parts_diff_bin(d: &BcfPartsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_part_diff_bin, enc_part_bin, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_parts_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfPartsDiff, String> {
    dec_named_triple_bin(reader, read_str_lp, dec_part_diff_bin, dec_part_bin)
}

impl protocol::DiffBinary for BcfDiff {
/// 🧪️ FG-wave: REAL binary frame (`format u8 | flags u8 | [version][topics][parts]`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (per this ticket's
/// own `📖️grammar-recipe.md` §4/§6 census, 100% of stdio's `DiffCodec` impls were still on
/// that shortcut before this pilot ladder). `flags` bit0/bit1/bit2 mark
/// `version`/`topics`/`parts` presence; each present field's own binary payload follows in
/// that fixed order (see `🔖️BinaryCodecs` above).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.version.is_some() {
        flags |= 0b001;
    }
    if self.topics.is_some() {
        flags |= 0b010;
    }
    if self.parts.is_some() {
        flags |= 0b100;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = &self.version {
        write_str_lp(&mut out, v);
    }
    if let Some(t) = &self.topics {
        enc_topics_diff_bin(t, &mut out);
    }
    if let Some(p) = &self.parts {
        enc_parts_diff_bin(p, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let version = if flags & 0b001 != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("diff version", reader.position(), e))?) } else { None };
    let topics = if flags & 0b010 != 0 { Some(dec_topics_diff_bin(&mut reader).map_err(|e| malformed("diff topics", reader.position(), e))?) } else { None };
    let parts = if flags & 0b100 != 0 { Some(dec_parts_diff_bin(&mut reader).map_err(|e| malformed("diff parts", reader.position(), e))?) } else { None };
    Ok(BcfDiff { version, topics, parts })
}
}
}
pub use diff_codec::*;
