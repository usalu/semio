//! 🧊️ Incremental GLB writing over an explicitly bounded borrowed mesh source.
use std::fmt::Write;
use std::mem::size_of;

/// 🔎️ A stable mesh view whose individual reads have bounded cost.
pub trait GlbSource {
    fn read_work(&self) -> u64;
    fn valid_layout(&self) -> bool;
    fn vertices(&self) -> usize;
    fn indices(&self) -> usize;
    fn has_normals(&self) -> bool;
    fn position(&self, vertex: usize) -> (Option<[f32; 3]>, u64);
    fn normal(&self, vertex: usize) -> (Option<[f32; 3]>, u64);
    fn index(&self, index: usize) -> (Option<u32>, u64);
}

impl GlbSource for crate::MeshData {
    fn read_work(&self) -> u64 { 1 }
    fn valid_layout(&self) -> bool { self.positions.len() % 3 == 0 }
    fn vertices(&self) -> usize { self.positions.len() / 3 }
    fn indices(&self) -> usize { self.indices.len() }
    fn has_normals(&self) -> bool { self.normals.len() == self.positions.len() }
    fn position(&self, vertex: usize) -> (Option<[f32; 3]>, u64) { (vertex.checked_mul(3).and_then(|start| self.positions.get(start..start.checked_add(3)?)).and_then(|value| value.try_into().ok()), 1) }
    fn normal(&self, vertex: usize) -> (Option<[f32; 3]>, u64) { (vertex.checked_mul(3).and_then(|start| self.normals.get(start..start.checked_add(3)?)).and_then(|value| value.try_into().ok()), 1) }
    fn index(&self, index: usize) -> (Option<u32>, u64) { (self.indices.get(index).copied(), 1) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MeshData;

    #[test]
    fn bounded_turns_match_independent_gltf_with_and_without_normals() {
        for supplied in [false, true] {
            let mut mesh = MeshData { positions: vec![0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0], indices: vec![0,1,2], ..Default::default() };
            if supplied { mesh.compute_normals(); }
            let mut expected = mesh.clone(); if !supplied { expected.compute_normals(); }
            let mut cursor = GlbCursor::new(&mesh).unwrap(); let mut bytes = Vec::new(); let mut scratch = vec![[0.0; 3]; mesh.vertices()];
            loop {
                let mut byte = [0];
                let denied = cursor.advance(&mesh, &mut scratch, &mut byte, 0, usize::MAX, 1, u64::MAX).unwrap();
                assert_eq!((denied.written, denied.copied_bytes, denied.work), (0,0,0));
                let denied = cursor.advance(&mesh, &mut scratch, &mut byte, 1, 0, 1, u64::MAX).unwrap();
                assert_eq!((denied.written, denied.copied_bytes, denied.work), (0,0,0));
                let denied = cursor.advance(&mesh, &mut scratch, &mut byte, 1, usize::MAX, 0, u64::MAX).unwrap();
                assert_eq!((denied.written, denied.copied_bytes, denied.work), (0,0,0));
                let denied = cursor.advance(&mesh, &mut scratch, &mut byte, 1, usize::MAX, 1, 0).unwrap();
                assert_eq!((denied.written, denied.copied_bytes, denied.work), (0,0,0));
                let demand = cursor.next_copy_demand();
                let work = cursor.next_work_demand(&mesh);
                let turn = cursor.advance(&mesh, &mut scratch, &mut byte, 1, demand, 1, work).unwrap();
                assert!(turn.work <= work); if turn.written == 1 { bytes.push(byte[0]); }
                if turn.complete { break; }
            }
            assert_eq!(bytes, super::super::mesh_to_glb(&mesh).unwrap());
            let glb = gltf::Gltf::from_slice(&bytes).unwrap(); let primitive = glb.meshes().next().unwrap().primitives().next().unwrap();
            let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(glb.blob.as_deref().unwrap()));
            assert_eq!(reader.read_positions().unwrap().flatten().collect::<Vec<_>>(), expected.positions);
            assert_eq!(reader.read_normals().unwrap().flatten().collect::<Vec<_>>(), expected.normals);
            assert_eq!(reader.read_indices().unwrap().into_u32().collect::<Vec<_>>(), expected.indices);
        }
    }

    #[test]
    fn empty_scene_matches_neutral_case_and_independent_gltf() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️empty/🔣️.json")).unwrap();
        assert!(fixture["positions"].as_array().unwrap().is_empty());
        assert!(fixture["indices"].as_array().unwrap().is_empty());
        let mesh = MeshData::default();
        let mut cursor = GlbCursor::new(&mesh).unwrap();
        let mut bytes = Vec::new();
        loop {
            let mut byte = [0];
            let demand = cursor.next_copy_demand(); let work = cursor.next_work_demand(&mesh);
            let turn = cursor.advance(&mesh, &mut [], &mut byte, 1, demand, 1, work).unwrap();
            if turn.written == 1 { bytes.push(byte[0]); }
            if turn.complete { break; }
        }
        let batch = super::super::mesh_to_glb(&mesh).unwrap();
        for value in [&bytes, &batch] {
            let parsed = gltf::Gltf::from_slice(value).unwrap();
            assert!(parsed.blob.is_none());
            assert_eq!(parsed.scenes().count(), 1);
            assert_eq!(parsed.meshes().count(), 0);
            let json_len = u32::from_le_bytes(value[12..16].try_into().unwrap()) as usize;
            let document: serde_json::Value = serde_json::from_slice(&value[20..20 + json_len]).unwrap();
            assert_eq!(document, fixture["document"]);
        }
    }

    #[test]
    fn rejects_trailing_position_components() {
        let mesh = MeshData { positions: vec![0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0,9.0], indices: vec![0,1,2], ..Default::default() };
        assert!(GlbCursor::new(&mesh).is_err());
        assert!(super::super::mesh_to_glb(&mesh).is_err());
    }

    #[test]
    fn generated_normals_remain_unit_length_for_large_finite_geometry() {
        let mesh = MeshData { positions: vec![0.0,0.0,0.0,1e20,0.0,0.0,0.0,1.0,0.0], indices: vec![0,1,2], ..Default::default() };
        let mut cursor = GlbCursor::new(&mesh).unwrap();
        let mut normals = vec![[0.0; 3]; 3];
        let mut bytes = Vec::new();
        loop {
            let mut byte = [0];
            let demand = cursor.next_copy_demand();
            let work = cursor.next_work_demand(&mesh);
            let turn = cursor.advance(&mesh, &mut normals, &mut byte, 1, demand, 1, work).unwrap();
            if turn.written == 1 { bytes.push(byte[0]); }
            if turn.complete { break; }
        }
        let glb = gltf::Gltf::from_slice(&bytes).unwrap();
        let primitive = glb.meshes().next().unwrap().primitives().next().unwrap();
        let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(glb.blob.as_deref().unwrap()));
        for normal in reader.read_normals().unwrap() { assert_eq!(normal, [0.0, 0.0, 1.0]); }
    }
}

/// 🧾️ One bounded cursor turn; a zero-length destination refuses the turn unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlbTurn { pub written: usize, pub copied_bytes: usize, pub complete: bool, pub work: u64 }

/// ⚠️ A refusal retaining the source work already observed by its turn.
#[derive(Debug)]
pub struct GlbFailure { pub reason: &'static str, pub work: u64 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage { Positions, Indices, Normals, ClearNormals, AccumulateNormals, NormalizeNormals, Header, PositionBytes, NormalBytes, IndexBytes, Done }

/// 🧭️ An allocation-free cursor; each funded turn inspects one vertex or triangle or writes one byte.
pub struct GlbCursor {
    vertices: usize, indices: usize, supplied_normals: bool, stage: Stage, at: usize,
    min: [f32; 3], max: [f32; 3], header: [u8; 2048], header_len: usize,
    word: [u8; 12], word_len: usize, word_at: usize,
}

struct FixedText<'a> { bytes: &'a mut [u8], len: usize }
impl Write for FixedText<'_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let end = self.len.checked_add(value.len()).ok_or(std::fmt::Error)?;
        let target = self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?;
        target.copy_from_slice(value.as_bytes()); self.len = end; Ok(())
    }
}

impl GlbCursor {
    /// 🌱️ Captures only dimensions; no source element is read or copied during construction.
    pub fn new(source: &impl GlbSource) -> Result<Self, &'static str> {
        let vertices = source.vertices(); let indices = source.indices();
        if !source.valid_layout() || (vertices == 0) != (indices == 0) || indices % 3 != 0 || vertices > u32::MAX as usize || indices > u32::MAX as usize { return Err("invalid GLB mesh dimensions"); }
        Ok(Self { vertices, indices, supplied_normals: source.has_normals(), stage: if vertices == 0 { Stage::Header } else { Stage::Positions }, at: 0, min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3], header: [0; 2048], header_len: 0, word: [0; 12], word_len: 0, word_at: 0 })
    }

    fn read<T>(source: &impl GlbSource, result: (Option<T>, u64), work: &mut u64) -> Result<T, &'static str> {
        *work = work.saturating_add(result.1);
        if result.1 > source.read_work() { return Err("GLB source exceeded its work quote"); }
        result.0.ok_or("missing GLB source value")
    }

    fn vertex(source: &impl GlbSource, index: usize, work: &mut u64) -> Result<[f32; 3], &'static str> {
        let value = Self::read(source, source.position(index), work)?;
        if !value.iter().all(|v| v.is_finite()) { return Err("non-finite GLB position"); } Ok(value)
    }

    fn build_header(&mut self) -> Result<(), &'static str> {
        if self.vertices == 0 {
            let json = super::EMPTY_SCENE_JSON; let json_len = (json.len() + 3) & !3; let end = 20 + json_len;
            self.header[..4].copy_from_slice(b"glTF"); self.header[4..8].copy_from_slice(&2u32.to_le_bytes()); self.header[8..12].copy_from_slice(&(end as u32).to_le_bytes());
            self.header[12..16].copy_from_slice(&(json_len as u32).to_le_bytes()); self.header[16..20].copy_from_slice(b"JSON"); self.header[20..20 + json.len()].copy_from_slice(json); self.header[20 + json.len()..end].fill(b' '); self.header_len = end; return Ok(());
        }
        let position_bytes = self.vertices.checked_mul(12).ok_or("GLB length overflow")?;
        let index_bytes = self.indices.checked_mul(4).ok_or("GLB length overflow")?;
        let binary = position_bytes.checked_mul(2).and_then(|n| n.checked_add(index_bytes)).ok_or("GLB length overflow")?;
        let mut header = [0u8; 2048];
        let mut text = FixedText { bytes: &mut header[20..], len: 0 };
        write!(text, "{{\"asset\":{{\"version\":\"2.0\"}},\"scene\":0,\"scenes\":[{{\"nodes\":[0]}}],\"nodes\":[{{\"mesh\":0}}],\"meshes\":[{{\"primitives\":[{{\"attributes\":{{\"POSITION\":0,\"NORMAL\":1}},\"indices\":2,\"mode\":4}}]}}],\"accessors\":[{{\"bufferView\":0,\"componentType\":5126,\"count\":{},\"type\":\"VEC3\",\"min\":[{},{},{}],\"max\":[{},{},{}]}},{{\"bufferView\":1,\"componentType\":5126,\"count\":{},\"type\":\"VEC3\"}},{{\"bufferView\":2,\"componentType\":5125,\"count\":{},\"type\":\"SCALAR\"}}],\"bufferViews\":[{{\"buffer\":0,\"byteOffset\":0,\"byteLength\":{}}},{{\"buffer\":0,\"byteOffset\":{},\"byteLength\":{}}},{{\"buffer\":0,\"byteOffset\":{},\"byteLength\":{}}}],\"buffers\":[{{\"byteLength\":{}}}]}}", self.vertices, self.min[0], self.min[1], self.min[2], self.max[0], self.max[1], self.max[2], self.vertices, self.indices, position_bytes, position_bytes, position_bytes, position_bytes * 2, index_bytes, binary).map_err(|_| "GLB header exceeds fixed bound")?;
        let json_len = (text.len + 3) & !3; drop(text);
        let end = 20usize.checked_add(json_len).ok_or("GLB length overflow")?;
        if end + 8 > header.len() { return Err("GLB header exceeds fixed bound"); }
        for byte in &mut header[20..end] { if *byte == 0 { *byte = b' '; } }
        let total = u32::try_from(end.checked_add(8).and_then(|n| n.checked_add(binary)).ok_or("GLB length overflow")?).map_err(|_| "GLB length overflow")?;
        header[..4].copy_from_slice(b"glTF"); header[4..8].copy_from_slice(&2u32.to_le_bytes()); header[8..12].copy_from_slice(&total.to_le_bytes());
        header[12..16].copy_from_slice(&(json_len as u32).to_le_bytes()); header[16..20].copy_from_slice(b"JSON");
        header[end..end + 4].copy_from_slice(&(binary as u32).to_le_bytes()); header[end + 4..end + 8].copy_from_slice(b"BIN\0");
        self.header_len = end + 8; self.header[..self.header_len].copy_from_slice(&header[..self.header_len]); Ok(())
    }

    fn load_vector(&mut self, value: [f32; 3]) { for (index, part) in value.iter().enumerate() { self.word[index * 4..index * 4 + 4].copy_from_slice(&part.to_le_bytes()); } self.word_len = 12; self.word_at = 0; }

    /// 📏️ Quotes the maximum copied bytes for the next turn.
    pub fn next_copy_demand(&self) -> usize { match self.stage { Stage::Header if self.header_len == 0 => self.header.len() + size_of::<usize>(), Stage::ClearNormals | Stage::AccumulateNormals | Stage::NormalizeNormals => 64, Stage::Positions => 40, Stage::PositionBytes | Stage::NormalBytes | Stage::IndexBytes => 40, Stage::Indices | Stage::Normals | Stage::Header => 16, Stage::Done => 0 } }

    /// ⛽️ Quotes the next turn's maximum primitive work.
    pub fn next_work_demand(&self, source: &impl GlbSource) -> u64 { let read = source.read_work(); match self.stage { Stage::Header if self.header_len == 0 => 4096, Stage::AccumulateNormals => read.saturating_mul(6).saturating_add(64), Stage::NormalizeNormals => 32, Stage::Positions | Stage::Normals => read.saturating_add(4), Stage::PositionBytes | Stage::NormalBytes if self.word_len == 0 => read.saturating_add(4), Stage::Indices => read.saturating_add(1), Stage::IndexBytes if self.word_len == 0 => read.saturating_add(2), Stage::Done => 0, _ => 1 } }

    /// ⏭️ Advances at most one source inspection or one output byte without allocating.
    pub fn advance(&mut self, source: &impl GlbSource, normals: &mut [[f32; 3]], output: &mut [u8], items: usize, copy_bytes: usize, depth: usize, fuel: u64) -> Result<GlbTurn, GlbFailure> {
        if output.is_empty() || items == 0 || depth == 0 || copy_bytes < self.next_copy_demand() || fuel < self.next_work_demand(source) || self.stage == Stage::Done { return Ok(GlbTurn { written: 0, copied_bytes: 0, complete: self.stage == Stage::Done, work: 0 }); }
        if !source.valid_layout() || source.vertices() != self.vertices || source.indices() != self.indices || source.has_normals() != self.supplied_normals || (!self.supplied_normals && normals.len() != self.vertices) { return Err(GlbFailure { reason: "GLB source dimensions or normal storage changed", work: 1 }); }
        let mut observed = 0; let mut work = match self.stage { Stage::Header if self.header_len == 0 => 0, Stage::AccumulateNormals => 64, Stage::NormalizeNormals => 32, Stage::Positions | Stage::Normals => 4, Stage::PositionBytes | Stage::NormalBytes if self.word_len == 0 => 4, Stage::IndexBytes if self.word_len == 0 => 2, _ => 1 }; let mut written = 0; let mut copied_bytes = 0;
        let result = (|| -> Result<(), &'static str> { match self.stage {
            Stage::Positions => { let p = Self::vertex(source, self.at, &mut observed)?; for axis in 0..3 { self.min[axis] = self.min[axis].min(p[axis]); self.max[axis] = self.max[axis].max(p[axis]); } copied_bytes = 24 + size_of::<usize>(); self.at += 1; if self.at == self.vertices { self.at = 0; self.stage = Stage::Indices; copied_bytes += 1; } }
            Stage::Indices => { if Self::read(source, source.index(self.at), &mut observed)? as usize >= self.vertices { return Err("invalid GLB index"); } self.at += 1; copied_bytes = size_of::<usize>(); if self.at == self.indices { self.at = 0; self.stage = if self.supplied_normals { Stage::Normals } else { Stage::ClearNormals }; copied_bytes += 1; } }
            Stage::Normals => { let n = Self::read(source, source.normal(self.at), &mut observed)?; if !n.iter().all(|v| v.is_finite()) { return Err("non-finite GLB normal"); } self.at += 1; copied_bytes = size_of::<usize>(); if self.at == self.vertices { self.at = 0; self.stage = Stage::Header; copied_bytes += 1; } }
            Stage::Header => { if self.header_len == 0 { self.build_header()?; copied_bytes = self.header_len + size_of::<usize>(); work = (self.header_len * 2) as u64; } else { output[0] = self.header[self.at]; written = 1; self.at += 1; copied_bytes = size_of::<usize>(); if self.at == self.header_len { self.at = 0; self.stage = if self.vertices == 0 { Stage::Done } else { Stage::PositionBytes }; copied_bytes += 1; } } }
            Stage::ClearNormals => { normals[self.at] = [0.0; 3]; self.at += 1; copied_bytes = 12 + size_of::<usize>(); if self.at == self.vertices { self.at = 0; self.stage = Stage::AccumulateNormals; copied_bytes += 1; } }
            Stage::AccumulateNormals => { let ids = [Self::read(source,source.index(self.at),&mut observed)?, Self::read(source,source.index(self.at + 1),&mut observed)?, Self::read(source,source.index(self.at + 2),&mut observed)?]; if ids.iter().any(|id| *id as usize >= self.vertices) { return Err("invalid GLB index"); } let a = Self::vertex(source, ids[0] as usize, &mut observed)?; let b = Self::vertex(source, ids[1] as usize, &mut observed)?; let c = Self::vertex(source, ids[2] as usize, &mut observed)?; let x = [b[0]-a[0],b[1]-a[1],b[2]-a[2]]; let y = [c[0]-a[0],c[1]-a[1],c[2]-a[2]]; let n = [x[1]*y[2]-x[2]*y[1],x[2]*y[0]-x[0]*y[2],x[0]*y[1]-x[1]*y[0]]; let mut values = ids.map(|id| normals[id as usize]); for index in 0..3 { for axis in 0..3 { values[index][axis] += n[axis]; } if !values[index].iter().all(|v| v.is_finite()) { return Err("non-finite generated GLB normal"); } for next in index+1..3 { if ids[next] == ids[index] { values[next] = values[index]; } } } for index in 0..3 { normals[ids[index] as usize] = values[index]; } self.at += 3; copied_bytes = 36 + size_of::<usize>(); if self.at == self.indices { self.at = 0; self.stage = Stage::NormalizeNormals; copied_bytes += 1; } }
            Stage::NormalizeNormals => { let value = normals[self.at]; if !value.iter().all(|v| v.is_finite()) { return Err("non-finite generated GLB normal"); } let scale = value.iter().fold(0.0f32, |maximum, v| maximum.max(v.abs())); let scaled = if scale > 0.0 { value.map(|v| v / scale) } else { value }; let length = scaled.iter().map(|v| v*v).sum::<f32>().sqrt(); normals[self.at] = if length > 0.0 && scale > 1e-8 / length { scaled.map(|v| v / length) } else { value }; self.at += 1; copied_bytes = 12 + size_of::<usize>(); if self.at == self.vertices { self.at = 0; self.stage = Stage::Header; copied_bytes += 1; } }
            Stage::PositionBytes => { if self.word_len == 0 { self.load_vector(Self::vertex(source, self.at, &mut observed)?); copied_bytes = 12 + 2 * size_of::<usize>(); } else { output[0] = self.word[self.word_at]; written = 1; self.word_at += 1; copied_bytes = size_of::<usize>(); if self.word_at == self.word_len { self.word_len = 0; self.at += 1; copied_bytes += 2 * size_of::<usize>(); if self.at == self.vertices { self.at = 0; self.stage = Stage::NormalBytes; copied_bytes += 1; } } } }
            Stage::NormalBytes => { if self.word_len == 0 { let value = if self.supplied_normals { Self::read(source,source.normal(self.at),&mut observed)? } else { normals[self.at] }; self.load_vector(value); copied_bytes = 12 + 2 * size_of::<usize>(); } else { output[0] = self.word[self.word_at]; written = 1; self.word_at += 1; copied_bytes = size_of::<usize>(); if self.word_at == self.word_len { self.word_len = 0; self.at += 1; copied_bytes += 2 * size_of::<usize>(); if self.at == self.vertices { self.at = 0; self.stage = Stage::IndexBytes; copied_bytes += 1; } } } }
            Stage::IndexBytes => { if self.word_len == 0 { self.word[..4].copy_from_slice(&Self::read(source,source.index(self.at),&mut observed)?.to_le_bytes()); self.word_len = 4; self.word_at = 0; copied_bytes = 4 + 2 * size_of::<usize>(); } else { output[0] = self.word[self.word_at]; written = 1; self.word_at += 1; copied_bytes = size_of::<usize>(); if self.word_at == self.word_len { self.word_len = 0; self.at += 1; copied_bytes += 2 * size_of::<usize>(); if self.at == self.indices { self.stage = Stage::Done; copied_bytes += 1; } } } }
            Stage::Done => unreachable!(),
        } Ok(()) })();
        work = work.saturating_add(observed);
        result.map_err(|reason| GlbFailure { reason, work })?;
        Ok(GlbTurn { written, copied_bytes: copied_bytes + written, complete: self.stage == Stage::Done, work })
    }
}
