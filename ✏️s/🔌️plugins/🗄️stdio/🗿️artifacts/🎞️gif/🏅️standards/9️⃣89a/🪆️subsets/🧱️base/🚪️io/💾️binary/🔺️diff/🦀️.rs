//! gif rep for stdio.gif 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v89a::subsets::any::schema::diff::*;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText, GifRgb, GifSnapshot};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

/// 🧪️ P2-FG2: real binary value codecs for `GifDiff` (89a)'s nested types — mirrors the text
/// codecs above field-for-field, using `dsl::ByteWriter`/`dsl::ByteReader` (the same real
/// LEB128-varint/length-prefixed framework primitives png's own upgraded `PngDiff` binary frame
/// uses, `📷️png/…/🔺️diff/🦀️.rs`'s `RealBinaryPrimitives`/`RealBinaryDiffFrame`
/// regions, and 87a's own sibling upgrade — `dsl`/`store`/`protocol` all alias the same kernel
/// crate root, reachable with no `use` needed beyond the absolute path).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_rgb(w: &mut dsl::ByteWriter, c: &GifRgb) {
    w.write_u8(c.r);
    w.write_u8(c.g);
    w.write_u8(c.b);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_rgb(r: &mut dsl::ByteReader<'_>) -> Result<GifRgb, dsl::PackRefusal> {
    Ok(GifRgb { r: r.read_u8()?, g: r.read_u8()?, b: r.read_u8()? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_color_table(w: &mut dsl::ByteWriter, t: &GifColorTable) {
    w.write_u8(if t.sorted { 1 } else { 0 });
    write_bin_vec(w, &t.colors, write_bin_rgb);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_color_table(r: &mut dsl::ByteReader<'_>) -> Result<GifColorTable, dsl::PackRefusal> {
    let sorted = r.read_u8()? != 0;
    let colors = read_bin_vec(r, read_bin_rgb)?;
    Ok(GifColorTable { sorted, colors })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_blob(r: &mut dsl::ByteReader<'_>) -> Result<Vec<u8>, dsl::PackRefusal> {
    let len = r.read_varint_u64()? as usize;
    Ok(r.read_bytes(len)?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackRefusal> {
    let bytes = read_bin_blob(r)?;
    String::from_utf8(bytes).map_err(|e| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gif89a binary utf8 string", offset: 0, detail: e.to_string() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_disposal(w: &mut dsl::ByteWriter, d: GifDisposal) {
    w.write_u8(d.to_bits());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_disposal(r: &mut dsl::ByteReader<'_>) -> Result<GifDisposal, dsl::PackRefusal> {
    Ok(GifDisposal::from_bits(r.read_u8()?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_plain_text(r: &mut dsl::ByteReader<'_>) -> Result<GifPlainText, dsl::PackRefusal> {
    Ok(GifPlainText {
        left: r.read_u32_le()?,
        top: r.read_u32_le()?,
        width: r.read_u32_le()?,
        height: r.read_u32_le()?,
        cell_width: r.read_u8()?,
        cell_height: r.read_u8()?,
        fg_color_index: r.read_u8()?,
        bg_color_index: r.read_u8()?,
        text: read_bin_str(r)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_frame(w: &mut dsl::ByteWriter, f: &GifFrame) {
    w.write_u32_le(f.left);
    w.write_u32_le(f.top);
    w.write_u32_le(f.width);
    w.write_u32_le(f.height);
    w.write_u8(if f.interlace { 1 } else { 0 });
    write_bin_option(w, &f.lct, write_bin_color_table);
    write_bin_blob(w, &f.indices);
    w.write_u16_le(f.delay_cs);
    write_bin_disposal(w, f.disposal);
    write_bin_option(w, &f.transparent_index, |w, v| w.write_u8(*v));
    w.write_u8(if f.user_input { 1 } else { 0 });
    write_bin_option(w, &f.plain_text, write_bin_plain_text);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_frame(r: &mut dsl::ByteReader<'_>) -> Result<GifFrame, dsl::PackRefusal> {
    Ok(GifFrame {
        left: r.read_u32_le()?,
        top: r.read_u32_le()?,
        width: r.read_u32_le()?,
        height: r.read_u32_le()?,
        interlace: r.read_u8()? != 0,
        lct: read_bin_option(r, read_bin_color_table)?,
        indices: read_bin_blob(r)?,
        delay_cs: r.read_u16_le()?,
        disposal: read_bin_disposal(r)?,
        transparent_index: read_bin_option(r, |r| r.read_u8())?,
        user_input: r.read_u8()? != 0,
        plain_text: read_bin_option(r, read_bin_plain_text)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_app_extension(w: &mut dsl::ByteWriter, e: &GifAppExtension) {
    w.write_bytes(&e.identifier);
    w.write_bytes(&e.auth_code);
    write_bin_blob(w, &e.data);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_app_extension(r: &mut dsl::ByteReader<'_>) -> Result<GifAppExtension, dsl::PackRefusal> {
    let identifier: [u8; 8] = r.read_bytes(8)?.try_into().map_err(|_| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gif89a app extension identifier", offset: 0, detail: "expected 8 bytes".into() })?;
    let auth_code: [u8; 3] = r.read_bytes(3)?.try_into().map_err(|_| dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gif89a app extension auth_code", offset: 0, detail: "expected 3 bytes".into() })?;
    Ok(GifAppExtension { identifier, auth_code, data: read_bin_blob(r)? })
}

/// 🧩 2-way presence flag (`0`=None, `1`=Some) — shared by every plain `Option<T>` field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_option<T>(w: &mut dsl::ByteWriter, v: &Option<T>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(val) => {
            w.write_u8(1);
            write_value(w, val);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_option<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<T>, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(read_value(r)?)),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gif89a binary option tag", offset: 0, detail: format!("unknown tag {other}") }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_vec<T>(w: &mut dsl::ByteWriter, items: &[T], write_item: impl Fn(&mut dsl::ByteWriter, &T)) {
    w.write_varint_u64(items.len() as u64);
    for item in items {
        write_item(w, item);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_vec<T>(r: &mut dsl::ByteReader<'_>, mut read_item: impl FnMut(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Vec<T>, dsl::PackRefusal> {
    let n = r.read_varint_u64()? as usize;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_item(r)?);
    }
    Ok(out)
}

/// 🧩 3-way flag (`0`=unchanged, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) for every
/// TRI-STATE `Option<Option<T>>` field — same shape as png's own doc comment (this avoids
/// chaining two `if`-guarded conditional fields at the protocol-description level; the Rust
/// codec here has no such limitation but keeps the same 3-way-flag SHAPE for parity).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_tri_flag<T>(w: &mut dsl::ByteWriter, v: &Option<Option<T>>, write_value: impl FnOnce(&mut dsl::ByteWriter, &T)) {
    match v {
        None => w.write_u8(0),
        Some(None) => w.write_u8(1),
        Some(Some(val)) => {
            w.write_u8(2);
            write_value(w, val);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_tri_flag<T>(r: &mut dsl::ByteReader<'_>, read_value: impl FnOnce(&mut dsl::ByteReader<'_>) -> Result<T, dsl::PackRefusal>) -> Result<Option<Option<T>>, dsl::PackRefusal> {
    match r.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(read_value(r)?))),
        other => Err(dsl::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "gif89a diff tri-flag", offset: 0, detail: format!("unknown flag {other}") }),
    }
}

/// 🧪️ P2-FG2: real binary encodings for the three collection-triple diff types (`frames`,
/// `comments`, `app_extensions`) — each produces one opaque `Vec<u8>` blob matching
/// `../💾️binary/📡️.protocol.semio`'s `Array(u8, Field(<name>_len))` fields exactly
/// (the blob's OWN internal removed/modified/added shape isn't further protocol-walkable, see
/// that file's own doc comment); the Rust codec here IS genuinely, fully structured (real
/// varint counts, real per-item recursive encoding), never text-as-bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_frame_diff(w: &mut dsl::ByteWriter, d: &GifFrameDiff) {
    write_bin_option(w, &d.left, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.top, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.width, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.height, |w, v| w.write_u32_le(*v));
    write_bin_option(w, &d.interlace, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri_flag(w, &d.lct, write_bin_color_table);
    write_bin_option(w, &d.indices, |w, v| write_bin_blob(w, v));
    write_bin_option(w, &d.delay_cs, |w, v| w.write_u16_le(*v));
    write_bin_option(w, &d.disposal, |w, v| write_bin_disposal(w, *v));
    write_bin_tri_flag(w, &d.transparent_index, |w, v| w.write_u8(*v));
    write_bin_option(w, &d.user_input, |w, v| w.write_u8(if *v { 1 } else { 0 }));
    write_bin_tri_flag(w, &d.plain_text, write_bin_plain_text);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_frame_diff(r: &mut dsl::ByteReader<'_>) -> Result<GifFrameDiff, dsl::PackRefusal> {
    Ok(GifFrameDiff {
        left: read_bin_option(r, |r| r.read_u32_le())?,
        top: read_bin_option(r, |r| r.read_u32_le())?,
        width: read_bin_option(r, |r| r.read_u32_le())?,
        height: read_bin_option(r, |r| r.read_u32_le())?,
        interlace: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        lct: read_bin_tri_flag(r, read_bin_color_table)?,
        indices: read_bin_option(r, read_bin_blob)?,
        delay_cs: read_bin_option(r, |r| r.read_u16_le())?,
        disposal: read_bin_option(r, read_bin_disposal)?,
        transparent_index: read_bin_tri_flag(r, |r| r.read_u8())?,
        user_input: read_bin_option(r, |r| Ok(r.read_u8()? != 0))?,
        plain_text: read_bin_tri_flag(r, read_bin_plain_text)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frames_diff_bin(d: &GifFramesDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifFrameModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_frame_diff(w, &m.diff);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &GifFrameAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_frame(w, &a.frame);
    });
    w.into_bytes()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frames_diff_bin(bytes: &[u8]) -> Result<GifFramesDiff, dsl::PackRefusal> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let diff = read_bin_frame_diff(r)?;
        Ok(GifFrameModified { index, diff })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let frame = read_bin_frame(r)?;
        Ok(GifFrameAdded { index, frame })
    })?;
    Ok(GifFramesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comments_diff_bin(d: &GifCommentsDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifCommentModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_str(w, &m.text);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &GifCommentAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_str(w, &a.text);
    });
    w.into_bytes()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comments_diff_bin(bytes: &[u8]) -> Result<GifCommentsDiff, dsl::PackRefusal> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let text = read_bin_str(r)?;
        Ok(GifCommentModified { index, text })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let text = read_bin_str(r)?;
        Ok(GifCommentAdded { index, text })
    })?;
    Ok(GifCommentsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_app_extensions_diff_bin(d: &GifAppExtensionsDiff) -> Vec<u8> {
    let mut w = dsl::ByteWriter::new();
    write_bin_vec(&mut w, &d.removed, |w, v: &usize| w.write_varint_u64(*v as u64));
    write_bin_vec(&mut w, &d.modified, |w, m: &GifAppExtensionModified| {
        w.write_varint_u64(m.index as u64);
        write_bin_app_extension(w, &m.extension);
    });
    write_bin_vec(&mut w, &d.added, |w, a: &GifAppExtensionAdded| {
        w.write_varint_u64(a.index as u64);
        write_bin_app_extension(w, &a.extension);
    });
    w.into_bytes()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_app_extensions_diff_bin(bytes: &[u8]) -> Result<GifAppExtensionsDiff, dsl::PackRefusal> {
    let mut r = dsl::ByteReader::new(bytes);
    let removed = read_bin_vec(&mut r, |r| Ok(r.read_varint_u64()? as usize))?;
    let modified = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let extension = read_bin_app_extension(r)?;
        Ok(GifAppExtensionModified { index, extension })
    })?;
    let added = read_bin_vec(&mut r, |r| {
        let index = r.read_varint_u64()? as usize;
        let extension = read_bin_app_extension(r)?;
        Ok(GifAppExtensionAdded { index, extension })
    })?;
    Ok(GifAppExtensionsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_blob(w: &mut dsl::ByteWriter, bytes: &[u8]) {
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    write_bin_blob(w, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_plain_text(w: &mut dsl::ByteWriter, p: &GifPlainText) {
    w.write_u32_le(p.left);
    w.write_u32_le(p.top);
    w.write_u32_le(p.width);
    w.write_u32_le(p.height);
    w.write_u8(p.cell_width);
    w.write_u8(p.cell_height);
    w.write_u8(p.fg_color_index);
    w.write_u8(p.bg_color_index);
    write_bin_str(w, &p.text);
}

impl protocol::DiffBinary for GifDiff {
/// ⚡️ P2-FG2: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
/// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut
/// per the P2-W0 census; the FG1 wave's own closer report flagged leaving this un-upgraded
/// as a real defect to not repeat). Matches `../💾️binary/📡️.protocol.semio`'s real
/// flag-per-field layout exactly, field for field, in struct order (2-way flag for plain
/// `Option<T>` fields, 3-way flag for the tri-state `gct`/`loop_count` fields).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut w = dsl::ByteWriter::new();
    write_bin_option(&mut w, &self.width, |w, v| w.write_u32_le(*v));
    write_bin_option(&mut w, &self.height, |w, v| w.write_u32_le(*v));
    write_bin_tri_flag(&mut w, &self.gct, |w, v| {
        let mut inner = dsl::ByteWriter::new();
        write_bin_color_table(&mut inner, v);
        write_bin_blob(w, &inner.into_bytes());
    });
    write_bin_option(&mut w, &self.background_color_index, |w, v| w.write_u8(*v));
    write_bin_option(&mut w, &self.pixel_aspect_ratio, |w, v| w.write_u8(*v));
    write_bin_tri_flag(&mut w, &self.loop_count, |w, v| w.write_u16_le(*v));
    write_bin_option(&mut w, &self.frames, |w, v| write_bin_blob(w, &enc_frames_diff_bin(v)));
    write_bin_option(&mut w, &self.comments, |w, v| write_bin_blob(w, &enc_comments_diff_bin(v)));
    write_bin_option(&mut w, &self.app_extensions, |w, v| write_bin_blob(w, &enc_app_extensions_diff_bin(v)));
    Ok(w.into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let width = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(|error| diff_pack_err(&error))?;
    let height = read_bin_option(&mut r, |r| r.read_u32_le()).map_err(|error| diff_pack_err(&error))?;
    let gct = read_bin_tri_flag(&mut r, |r| {
        let blob = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&blob);
        read_bin_color_table(&mut inner)
    })
    .map_err(|error| diff_pack_err(&error))?;
    let background_color_index = read_bin_option(&mut r, |r| r.read_u8()).map_err(|error| diff_pack_err(&error))?;
    let pixel_aspect_ratio = read_bin_option(&mut r, |r| r.read_u8()).map_err(|error| diff_pack_err(&error))?;
    let loop_count = read_bin_tri_flag(&mut r, |r| r.read_u16_le()).map_err(|error| diff_pack_err(&error))?;
    let frames = read_bin_option(&mut r, |r| dec_frames_diff_bin(&read_bin_blob(r)?)).map_err(|error| diff_pack_err(&error))?;
    let comments = read_bin_option(&mut r, |r| dec_comments_diff_bin(&read_bin_blob(r)?)).map_err(|error| diff_pack_err(&error))?;
    let app_extensions = read_bin_option(&mut r, |r| dec_app_extensions_diff_bin(&read_bin_blob(r)?)).map_err(|error| diff_pack_err(&error))?;
    Ok(GifDiff { width, height, gct, background_color_index, pixel_aspect_ratio, loop_count, frames, comments, app_extensions })
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v89a::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

}
pub use diff_codec::*;
