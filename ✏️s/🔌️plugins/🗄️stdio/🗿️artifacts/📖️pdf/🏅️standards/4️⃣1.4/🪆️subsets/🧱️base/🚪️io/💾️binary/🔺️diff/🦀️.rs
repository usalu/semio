//! pdf rep for stdio.pdf 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_4::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, text: &str) {
    store::pack_rt::write_varint_u64(out, text.len() as u64);
    out.extend_from_slice(text.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let length = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    String::from_utf8(reader.read_bytes(length).map_err(|error| error.to_string())?.to_vec()).map_err(|error| error.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page_bin(page: &PageDoc, out: &mut Vec<u8>) {
    out.extend_from_slice(&page.width.to_le_bytes());
    out.extend_from_slice(&page.height.to_le_bytes());
    write_str_lp(out, &page.text);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page_bin(reader: &mut store::ByteReader<'_>) -> Result<PageDoc, String> {
    let width = reader.read_f64_le().map_err(|error| error.to_string())?;
    let height = reader.read_f64_le().map_err(|error| error.to_string())?;
    Ok(PageDoc { width, height, text: read_str_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page_diff_bin(diff: &PdfPageDiff, out: &mut Vec<u8>) {
    out.push(u8::from(diff.width.is_some()));
    if let Some(value) = diff.width {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.push(u8::from(diff.height.is_some()));
    if let Some(value) = diff.height {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.push(u8::from(diff.text.is_some()));
    if let Some(value) = &diff.text {
        write_str_lp(out, value);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPageDiff, String> {
    let mut diff = PdfPageDiff::default();
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.width = Some(reader.read_f64_le().map_err(|error| error.to_string())?);
    }
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.height = Some(reader.read_f64_le().map_err(|error| error.to_string())?);
    }
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.text = Some(read_str_lp(reader)?);
    }
    Ok(diff)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pages_diff_bin(diff: &PdfPagesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for item in &diff.modified {
        store::pack_rt::write_varint_u64(out, item.index as u64);
        enc_page_diff_bin(&item.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for item in &diff.added {
        store::pack_rt::write_varint_u64(out, item.index as u64);
        enc_page_bin(&item.page, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pages_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPagesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|error| error.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
        modified.push(PdfPageModified { index, diff: dec_page_diff_bin(reader)? });
    }
    let added_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
        added.push(PdfPageAdded { index, page: dec_page_bin(reader)? });
    }
    Ok(PdfPagesDiff { removed, modified, added })
}

impl protocol::DiffBinary for PdfDiff {
/// 🧪️ Real binary frame (`format u8 | flags u8 | [pages]`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// varint-counted, length-prefixed, genuinely structured, never `print_diff().into_bytes()`.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let flags: u8 = u8::from(self.pages.is_some());
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(pages) = &self.pages {
        enc_pages_diff_bin(pages, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let format = reader.read_u8().map_err(|error| malformed("diff format", 0, error.to_string()))?;
    if format != store::pack_rt::OP_BINARY_FORMAT {
        return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
    }
    let flags = reader.read_u8().map_err(|error| malformed("diff flags", 1, error.to_string()))?;
    if flags & !0b0000_0001 != 0 {
        return Err(malformed("diff flags", 1, format!("unknown flag bits {:#010b}", flags & !0b0000_0001)));
    }
    let pages = if flags & 0b0000_0001 != 0 { Some(dec_pages_diff_bin(&mut reader).map_err(|error| malformed("diff pages", reader.position(), error))?) } else { None };
    if reader.remaining() != 0 {
        return Err(malformed("diff trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
    }
    Ok(PdfDiff { pages })
}
}
}
pub use diff_codec::*;
