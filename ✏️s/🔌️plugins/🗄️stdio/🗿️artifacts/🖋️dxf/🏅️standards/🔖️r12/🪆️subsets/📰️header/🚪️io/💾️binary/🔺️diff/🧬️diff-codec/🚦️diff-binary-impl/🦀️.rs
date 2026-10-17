//! 🚦️ Frame of the DXF diff codec: the binary diff implementation of the root diff. See [`super`].

use super::*;

impl protocol::DiffBinary for DxfDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | per-present-field payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
/// `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` is a 4-bit
/// presence mask (bit0=`header_vars`,bit1=`tables`,bit2=`blocks`,bit3=`entities`,bit4=`other_tables`) since
/// `DxfDiff` has FOUR independently optional top-level fields, unlike `stdio.json`'s single
/// `value` (one `has_value` byte there); each PRESENT field's own real recursive
/// collection-triple/tri-state binary payload follows, genuinely structured
/// (`#region 🔖️DiffBinaryCodecs`/`#region 🔖️CollectionTripleBinaryCodecs` above), never
/// text-as-bytes.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.header_vars.is_some() {
        flags |= 0b0001;
    }
    if self.tables.is_some() {
        flags |= 0b0010;
    }
    if self.blocks.is_some() {
        flags |= 0b0100;
    }
    if self.entities.is_some() {
        flags |= 0b1000;
    }
    if self.other_tables.is_some() {
        flags |= 0b1_0000;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = &self.header_vars {
        enc_header_vars_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.tables {
        enc_tables_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.blocks {
        enc_blocks_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.entities {
        enc_entities_diff_bin(v, &mut out);
    }
    if let Some(v) = &self.other_tables {
        store::pack_rt::write_varint_u64(&mut out, v.len() as u64);
        for table in v {
            enc_other_table_bin(table, &mut out);
        }
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let header_vars = if flags & 0b0001 != 0 { Some(dec_header_vars_diff_bin(&mut reader).map_err(|e| malformed("diff header_vars", reader.position(), e))?) } else { None };
    let tables = if flags & 0b0010 != 0 { Some(dec_tables_diff_bin(&mut reader).map_err(|e| malformed("diff tables", reader.position(), e))?) } else { None };
    let blocks = if flags & 0b0100 != 0 { Some(dec_blocks_diff_bin(&mut reader).map_err(|e| malformed("diff blocks", reader.position(), e))?) } else { None };
    let entities = if flags & 0b1000 != 0 { Some(dec_entities_diff_bin(&mut reader).map_err(|e| malformed("diff entities", reader.position(), e))?) } else { None };
    let other_tables = if flags & 0b1_0000 != 0 {
        let count = reader.read_varint_u64().map_err(|e| malformed("diff other_tables count", reader.position(), e.to_string()))?;
        let mut tables = Vec::new();
        for _ in 0..count {
            tables.push(dec_other_table_bin(&mut reader).map_err(|e| malformed("diff other_tables", reader.position(), e))?);
        }
        Some(tables)
    } else {
        None
    };
    Ok(DxfDiff { header_vars, tables, blocks, entities, other_tables })
}
}
