//! 💾️ Binary representation of `stdio.dwg` (snapshot): the pack codec and the DWG wire vocabulary (sentinels,
//! CRC, header decrypt, bit reader, version stamp) that the byte reader (`io::import`) and writer (`io::export`) share.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use crate::standards::v_ac1024::subsets::any::schema::STDIO_DWG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use std::fmt;
use semio_framework_value::{ValueError, ValueRefusalKind};

impl pack::snapshot::ArtifactPack for DwgSnapshot {
    /// 🪶️ Publishes this owner's complete authored relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for DwgSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::serialization::ArtifactSqliteSnapshotHost>::sqlite_codec())
    }
}
}

//#region 🧶️Wire
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_version_sentinel(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() < 6 {
        return Err("DWG too short for AC10xx header".into());
    }
    let head = &bytes[0..6];
    if head[0] != b'A' || head[1] != b'C' || !head[2].is_ascii_digit() || !head[3].is_ascii_digit() {
        return Err("missing AC10xx DWG version sentinel".into());
    }
    if !head[4].is_ascii_digit() || !head[5].is_ascii_digit() {
        return Err("invalid AC10xx version digits".into());
    }
    Ok(String::from_utf8_lossy(head).into_owned())
}
/// 🗓️🌐 Reads `maint_version` (offset 0x12) and `codepage` (offset 0x13-0x14 LE) from the plain
/// file-header preamble shared by every AC1015+ DWG file, per LibreDWG's own
/// `header.spec` field order (`zero_one_or_three@0x0B`, `thumbnail_address@0x0D`,
/// `dwg_version@0x11`, `maint_version@0x12`, `codepage@0x13`). Truncated headers are rejected.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_version_header_fields(bytes: &[u8]) -> Result<(u8, u16), String> {
    let maintenance_version = *bytes.get(0x12).ok_or("DWG header is too short for maintenance version")?;
    let codepage = bytes.get(0x13..0x15).ok_or("DWG header is too short for codepage")?;
    Ok((maintenance_version, u16::from_le_bytes([codepage[0], codepage[1]])))
}
/// 🕸️ Evaluation graph nodes, edges, and adjacency indexes.
pub type DwgEvaluationGraphIndexes = (Vec<[i32; 4]>, Vec<[i32; 5]>, Vec<(usize, usize)>);
//#region 🔖️R2004FileHeaderDecrypt
/// 🔓 R2004+ file header "decryption" -- not real security, a fixed LCG-generated one-time pad
/// (the classic Borland/MSVC `rand()` constants: `seed = seed*0x343fd + 0x269ec3`, upper 16 bits
/// of the running seed XORed per byte). Symmetric -- the same function both encrypts and decrypts.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decrypt_r2004_header(src: &[u8]) -> Vec<u8> {
    let mut rseed: u32 = 1;
    src.iter()
        .map(|&b| {
            rseed = rseed.wrapping_mul(0x343fd).wrapping_add(0x269ec3);
            b ^ ((rseed >> 0x10) & 0xFF) as u8
        })
        .collect()
}
pub const R2004_HEADER_LEN: usize = 0x6c;
//#endregion 🔖️R2004FileHeaderDecrypt
//#region 🔖️DwgStructuralCodec
//#region DwgBits
#[derive(Clone)]
pub struct DwgBitReader<'a> {
    pub bytes: &'a [u8],
    pub byte_pos: usize,
    bit: u8,
}
impl<'a> DwgBitReader<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, byte_pos: 0, bit: 0 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn at_bit(bytes: &'a [u8], bit_pos: usize) -> Result<Self, String> {
        if bit_pos > bytes.len().saturating_mul(8) {
            return Err("dwg bitstream position exceeds payload".to_string());
        }
        Ok(Self { bytes, byte_pos: bit_pos / 8, bit: (bit_pos % 8) as u8 })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bit(&mut self) -> Result<bool, String> {
        if self.byte_pos >= self.bytes.len() {
            return Err("dwg bitstream underflow".to_string());
        }
        let value = (self.bytes[self.byte_pos] >> (7 - self.bit)) & 1 != 0;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.byte_pos += 1;
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bits(&mut self, count: u8) -> Result<u64, String> {
        let mut value = 0u64;
        for _ in 0..count {
            value = (value << 1) | self.read_bit()? as u64;
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn skip_bits(&mut self, count: usize) -> Result<(), String> {
        for _ in 0..count {
            self.read_bit()?;
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn bit_position(&self) -> usize {
        self.byte_pos.saturating_mul(8).saturating_add(self.bit as usize)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_b(&mut self) -> Result<bool, String> {
        self.read_bit()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bb(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(2)? as u8)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_3b(&mut self) -> Result<u8, String> {
        let mut value = 0u8;
        for _ in 0..3 {
            let bit = self.read_b()?;
            value = (value << 1) | u8::from(bit);
            if !bit {
                break;
            }
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_rc(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(8)? as u8)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_rs(&mut self) -> Result<u16, String> {
        let lo = self.read_rc()? as u16;
        let hi = self.read_rc()? as u16;
        Ok(lo | (hi << 8))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_rl(&mut self) -> Result<u32, String> {
        let lo = self.read_rs()? as u32;
        let hi = self.read_rs()? as u32;
        Ok(lo | (hi << 16))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_rll(&mut self) -> Result<u64, String> {
        let lo = self.read_rl()? as u64;
        let hi = self.read_rl()? as u64;
        Ok(lo | (hi << 32))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_rd(&mut self) -> Result<f64, String> {
        let lo = self.read_rl()? as u64;
        let hi = self.read_rl()? as u64;
        Ok(f64::from_bits(lo | (hi << 32)))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bs(&mut self) -> Result<u16, String> {
        match self.read_bb()? {
            0 => self.read_rs(),
            1 => Ok(self.read_rc()? as u16),
            2 => Ok(0),
            _ => Ok(256),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bl(&mut self) -> Result<u32, String> {
        match self.read_bb()? {
            0 => self.read_rl(),
            1 => Ok(self.read_rc()? as u32),
            2 => Ok(0),
            _ => Err("invalid BL flag".to_string()),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bll(&mut self) -> Result<u64, String> {
        let byte_count = usize::from(self.read_3b()?);
        let mut value = 0u64;
        for shift in 0..byte_count {
            value |= u64::from(self.read_rc()?) << (shift * 8);
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bd(&mut self) -> Result<f64, String> {
        let position = self.bit_position();
        match self.read_bb()? {
            0 => self.read_rd(),
            1 => Ok(1.0),
            2 => Ok(0.0),
            _ => {
                let mut probe = self.clone();
                let next = probe.read_bits(16).unwrap_or(0);
                Err(format!("invalid BD flag at bit {position}, next={next:016b}"))
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_dd(&mut self, default: f64) -> Result<f64, String> {
        let mut bytes = default.to_le_bytes();
        match self.read_bb()? {
            0 => Ok(default),
            1 => {
                for byte in bytes.iter_mut().take(4) {
                    *byte = self.read_rc()?;
                }
                Ok(f64::from_le_bytes(bytes))
            }
            2 => {
                bytes[4] = self.read_rc()?;
                bytes[5] = self.read_rc()?;
                for byte in bytes.iter_mut().take(4) {
                    *byte = self.read_rc()?;
                }
                Ok(f64::from_le_bytes(bytes))
            }
            _ => self.read_rd(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bt(&mut self) -> Result<f64, String> {
        if self.read_b()? {
            Ok(0.0)
        } else {
            self.read_bd()
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_2rd(&mut self) -> Result<[f64; 2], String> {
        Ok([self.read_rd()?, self.read_rd()?])
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_3bd(&mut self) -> Result<[f64; 3], String> {
        Ok([self.read_bd()?, self.read_bd()?, self.read_bd()?])
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_be(&mut self) -> Result<[f64; 3], String> {
        if self.read_b()? {
            Ok([0.0, 0.0, 1.0])
        } else {
            self.read_3bd()
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_tv(&mut self) -> Result<String, String> {
        let length = self.read_bs()? as usize;
        let bytes = (0..length).map(|_| self.read_rc()).collect::<Result<Vec<_>, _>>()?;
        Ok(bytes.iter().take_while(|byte| **byte != 0).map(|byte| char::from(*byte)).collect())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_tu(&mut self) -> Result<String, String> {
        let position = self.bit_position();
        let length = self.read_bs()? as usize;
        let mut units = Vec::with_capacity(length);
        for _ in 0..length {
            units.push(self.read_rs().map_err(|error| format!("{error} in TU at bit {position}, length {length}"))?);
        }
        String::from_utf16(&units).map_err(|error| format!("invalid DWG UTF-16 string: {error}"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_ms(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let chunk = self.read_rs()?;
            value |= ((chunk & 0x7FFF) as u32) << shift;
            shift += 15;
            if chunk & 0x8000 == 0 {
                break;
            }
        }
        Ok(value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_umc(&mut self) -> Result<u64, String> {
        let mut value = 0u64;
        for shift in (0..56).step_by(7) {
            let byte = self.read_rc()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err("dwg unsigned modular-char overflow".into())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_bot(&mut self) -> Result<u16, String> {
        match self.read_bb()? {
            0 => Ok(self.read_rc()?.into()),
            1 => Ok(u16::from(self.read_rc()?) + 0x1f0),
            _ => self.read_rs(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn read_handle(&mut self) -> Result<(u8, u64), String> {
        let head = self.read_rc()?;
        let code = head >> 4;
        let len = head & 0x0F;
        let mut value = 0u64;
        for _ in 0..len {
            value = (value << 8) | self.read_rc()? as u64;
        }
        Ok((code, value))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn pad_to_byte(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte_pos += 1;
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_crc16(seed: u16, data: &[u8]) -> u16 {
    let mut crc = seed;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
//#endregion DwgBits
//#region DwgWrite
pub const DWG_SENTINEL_CLASSES_BEGIN: [u8; 16] = [0x8D, 0xA1, 0xC4, 0xB8, 0xC4, 0xA9, 0xF8, 0xC5, 0xC0, 0xDC, 0xF4, 0x5F, 0xE7, 0xCF, 0xB6, 0x8A];
//#endregion DwgWrite
//#region DwgRead
pub const DWG_PREVIEW_BEGIN: [u8; 16] = [0x1f, 0x25, 0x6d, 0x07, 0xd4, 0x36, 0x28, 0x28, 0x9d, 0x57, 0xca, 0x3f, 0x9d, 0x44, 0x10, 0x2b];
pub const DWG_PREVIEW_END: [u8; 16] = [0xe0, 0xda, 0x92, 0xf8, 0x2b, 0xc9, 0xd7, 0xd7, 0x62, 0xa8, 0x35, 0xc0, 0x62, 0xbb, 0xef, 0xd4];
pub const DWG_HEADER_BEGIN: [u8; 16] = [0xcf, 0x7b, 0x1f, 0x23, 0xfd, 0xde, 0x38, 0xa9, 0x5f, 0x7c, 0x68, 0xb8, 0x4e, 0x6d, 0x33, 0x5f];
pub const DWG_HEADER_END: [u8; 16] = [0x30, 0x84, 0xe0, 0xdc, 0x02, 0x21, 0xc7, 0x56, 0xa0, 0x83, 0x97, 0x47, 0xb1, 0x92, 0xcc, 0xa0];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn evaluation_graph_indexes(graph: &crate::standards::v_ac1024::subsets::any::schema::snapshot::DwgEvaluationGraph) -> Result<DwgEvaluationGraphIndexes, String> {
    let mut node_indexes = std::collections::BTreeMap::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        if node.id == 0 || node.expression_handle == 0 || node_indexes.insert(node.id, index).is_some() {
            return Err("evaluation graph contains an invalid or duplicate node".into());
        }
    }
    let mut endpoints = Vec::with_capacity(graph.edges.len());
    let mut incoming = vec![Vec::<usize>::new(); graph.nodes.len()];
    let mut outgoing = vec![Vec::<usize>::new(); graph.nodes.len()];
    for (index, edge) in graph.edges.iter().enumerate() {
        if edge.reference_count == 0 || edge.invertible || edge.suppressed {
            return Err("evaluation graph edge state is unsupported".into());
        }
        let from = *node_indexes.get(&edge.from_node_id).ok_or("evaluation graph edge source is missing")?;
        let to = *node_indexes.get(&edge.to_node_id).ok_or("evaluation graph edge target is missing")?;
        outgoing[from].push(index);
        incoming[to].push(index);
        endpoints.push((from, to));
    }
    let mut degree = incoming.iter().map(Vec::len).collect::<Vec<_>>();
    let mut queue = std::collections::VecDeque::from_iter(degree.iter().enumerate().filter_map(|(index, degree)| (*degree == 0).then_some(index)));
    let mut visited = 0usize;
    while let Some(node) = queue.pop_front() {
        visited += 1;
        for &edge in &outgoing[node] {
            let target = endpoints[edge].1;
            degree[target] -= 1;
            if degree[target] == 0 {
                queue.push_back(target);
            }
        }
    }
    if visited != graph.nodes.len() {
        return Err("evaluation graph must be acyclic".into());
    }
    let node_relations = (0..graph.nodes.len())
        .map(|index| {
            [incoming[index].first().map_or(-1, |value| *value as i32), incoming[index].last().map_or(-1, |value| *value as i32), outgoing[index].first().map_or(-1, |value| *value as i32), outgoing[index].last().map_or(-1, |value| *value as i32)]
        })
        .collect();
    let mut edge_relations = vec![[-1i32; 5]; graph.edges.len()];
    for list in incoming.iter().chain(outgoing.iter()) {
        for (position, edge) in list.iter().copied().enumerate() {
            let pair = if position == 0 { -1 } else { list[position - 1] as i32 };
            let next = list.get(position + 1).map_or(-1, |value| *value as i32);
            if incoming.iter().any(|candidate| std::ptr::eq(candidate, list)) {
                edge_relations[edge][0] = pair;
                edge_relations[edge][1] = next;
            } else {
                edge_relations[edge][2] = pair;
                edge_relations[edge][3] = next;
            }
        }
    }
    Ok((node_relations, edge_relations, endpoints))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn constraint_node_id(node: &crate::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNode) -> i32 {
    use crate::standards::v_ac1024::subsets::any::schema::snapshot::DwgConstraintNode;
    match node {
        DwgConstraintNode::ConstrainedImplicitPoint(value) => value.geometry.node.id,
        DwgConstraintNode::PointCurveConstraint(value)
        | DwgConstraintNode::PointCoincidenceConstraint(value)
        | DwgConstraintNode::PerpendicularConstraint(value)
        | DwgConstraintNode::ParallelConstraint(value)
        | DwgConstraintNode::MidPointConstraint(value)
        | DwgConstraintNode::EqualLengthConstraint(value)
        | DwgConstraintNode::ColinearConstraint(value)
        | DwgConstraintNode::FixedConstraint(value) => value.node.id,
        DwgConstraintNode::ConstrainedBoundedLine(value) => value.geometry.node.id,
        DwgConstraintNode::DistanceConstraint(value) => value.explicit.geometric.node.id,
        DwgConstraintNode::HorizontalConstraint(value) | DwgConstraintNode::VerticalConstraint(value) => value.geometric.node.id,
        DwgConstraintNode::ConstrainedDatumLine(value) => value.geometry.node.id,
    }
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
//#endregion 🧶️Wire
