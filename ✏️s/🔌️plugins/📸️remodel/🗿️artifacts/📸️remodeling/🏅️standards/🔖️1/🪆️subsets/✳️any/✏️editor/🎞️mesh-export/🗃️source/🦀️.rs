//! 🗃️ Incrementally indexes original bounded Remodel mesh leaves for borrowed GLB reads.
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{remodeling_durable_chunk, RemodelingDurableArtifact, RemodelingMeshChild, RemodelingOrderedDurableArtifactStore};
use semio_framework_mesh_engine::io::binary::stream::GlbSource;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_ordered::{LookupCursor, RetirementStep};
use std::mem::size_of;

const MAX_SCALARS: usize = 512 * 3;
const MAX_BYTES: usize = 87_552 + 30;
const BOX_ID: &str = "remodeling-mesh-constant-box";
const CONTENT_PREFIX: &[u8] = b"remodeling-mesh-content:";

pub(super) fn constant_mesh(id: &str) -> (bool, bool, u64) {
    fn equal(actual: &[u8], expected: &[u8], work: &mut u64) -> bool {
        *work += 1;
        if actual.len() != expected.len() { return false; }
        for (left, right) in actual.iter().zip(expected) { *work += 1; if left != right { return false; } }
        true
    }
    let mut work = 0;
    let empty = equal(id.as_bytes(), b"remodeling-mesh-constant-empty", &mut work);
    let boxed = equal(id.as_bytes(), BOX_ID.as_bytes(), &mut work);
    (empty, boxed, work)
}

/// 🔎️ Original ordered lookup retains the snapshot root and borrows its exact child-id bytes.
pub struct OriginalMeshLookup { cursor: LookupCursor<RemodelingDurableArtifact> }
impl OriginalMeshLookup {
    pub fn birth_demand(store: &RemodelingOrderedDurableArtifactStore) -> RetirementDemand { store.lookup_borrowed_constructor_demands() }
    pub fn admit(store: &RemodelingOrderedDurableArtifactStore, handle: &RemodelingMeshChild, grant: RetainedCloneGrant, fuel: u64) -> RetainedWorkOutcome<(Self, RetainedCloneProgress)> {
        if fuel == 0 { return RetainedWorkOutcome { result: Err(ValueError::literal(ValueRefusalKind::WorkLimit, "mesh lookup requires an original work grant")), work: 0 }; }
        let turn = store.admit_lookup_borrowed_original(&handle.child_id, grant);
        RetainedWorkOutcome { result: turn.result.map(|(cursor, progress)| (Self { cursor }, progress)), work: turn.work }
    }
    pub fn next_demand(&self, handle: &RemodelingMeshChild) -> Result<RetirementDemand, ValueError> { self.cursor.next_borrowed_original_demands(&handle.child_id) }
    pub fn advance(&mut self, handle: &RemodelingMeshChild, grant: RetainedCloneGrant, fuel: u64) -> RetainedWorkOutcome {
        if fuel == 0 { return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 }; }
        self.cursor.advance_borrowed_original(&handle.child_id, grant)
    }
    pub fn is_complete(&self) -> bool { self.cursor.is_complete() }
    pub fn result(&self) -> Option<&RemodelingDurableArtifact> { self.cursor.result() }
    pub fn begin_close(&mut self) { self.cursor.begin_close(); }
    pub fn next_close_copy_demand(&self) -> usize { self.cursor.next_close_copy_byte_demand() }
    pub fn next_close_release_demand(&self) -> Result<usize, ValueError> { self.cursor.next_close_byte_demand() }
    pub fn next_close_depth_demand(&self) -> usize { self.cursor.next_close_depth_demand() }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> (RetirementStep<RemodelingDurableArtifact>, u64) { let step = self.cursor.close_step(grant); (step, self.cursor.last_close_work()) }
    pub fn terminal_is_empty(&self) -> bool { self.cursor.terminal_is_empty() }
}

/// ⚠️ A refusal and the original work already performed by the turn.
#[derive(Debug)]
pub struct SourceFailure { pub reason: &'static str, pub work: u64 }

/// 🧾️ One paid handle parsing turn.
pub struct HandleTurn { pub value: Option<u64>, pub complete: bool, pub copied_bytes: usize, pub work: u64 }

/// 🔢️ Incrementally parses the existing content-count spelling without bounding its text length.
pub struct HandleCountCursor { identity: Option<usize>, at: usize, value: u64, digits: usize, complete: bool }
impl HandleCountCursor {
    pub fn new() -> Self { Self { identity: None, at: 0, value: 0, digits: 0, complete: false } }
    pub fn advance(&mut self, handle: &RemodelingMeshChild, items: usize, copy_bytes: usize, depth: usize, fuel: u64) -> Result<HandleTurn, SourceFailure> {
        if items == 0 || copy_bytes < 64 || depth == 0 || fuel == 0 { return Ok(HandleTurn { value: self.complete.then_some(self.value), complete: self.complete, copied_bytes: 0, work: 0 }); }
        let identity = handle as *const RemodelingMeshChild as usize;
        if self.identity.is_some_and(|original| original != identity) { return Err(SourceFailure { reason: "mesh handle source changed", work: 1 }); }
        if self.complete { return Ok(HandleTurn { value: Some(self.value), complete: true, copied_bytes: 0, work: 1 }); }
        let text = handle.target.artifact_id.as_bytes();
        if self.at == text.len() {
            if self.at < CONTENT_PREFIX.len() || self.digits == 0 { return Err(SourceFailure { reason: "invalid mesh content count", work: 1 }); }
            self.complete = true;
            return Ok(HandleTurn { value: Some(self.value), complete: true, copied_bytes: 1, work: 1 });
        }
        let byte = text[self.at];
        let mut next = self.value;
        let mut digit = false;
        if self.at < CONTENT_PREFIX.len() {
            if byte != CONTENT_PREFIX[self.at] { return Err(SourceFailure { reason: "invalid mesh content prefix", work: 1 }); }
        } else if self.at == CONTENT_PREFIX.len() && byte == b'+' {
        } else if byte.is_ascii_digit() {
            next = self.value.checked_mul(10).and_then(|value| value.checked_add(u64::from(byte - b'0'))).ok_or(SourceFailure { reason: "mesh content count overflow", work: 1 })?;
            digit = true;
        } else { return Err(SourceFailure { reason: "invalid mesh content count", work: 1 }); }
        let mut copied = size_of::<usize>();
        if self.identity.is_none() { self.identity = Some(identity); copied += size_of::<Option<usize>>(); }
        if digit { self.value = next; self.digits += 1; copied += size_of::<u64>() + size_of::<usize>(); }
        self.at += 1;
        Ok(HandleTurn { value: None, complete: false, copied_bytes: copied, work: 1 })
    }
}

/// 🧾️ One funded source-index turn.
pub struct IndexTurn { pub complete: bool, pub copied_bytes: usize, pub retained_capacity_bytes: usize, pub work: u64 }

/// ♻️ One paid scalar-index retirement turn.
pub struct IndexCloseTurn { pub complete: bool, pub copied_bytes: usize, pub released_bytes: usize, pub work: u64 }

/// 🧭️ Stores only scalar locations; the original snapshot remains the sole owner of mesh bytes.
pub struct MeshIndex {
    positions: Vec<u32>, normals: Vec<u32>, indices: Vec<u32>,
    counts: [usize; 12], leaf: usize, offset: usize, total: usize, previous: Option<u8>,
    utf8_remaining: u8, utf8_min: u8, utf8_max: u8, box_mesh: bool, empty_mesh: bool, ready: bool,
    handle_identity: Option<usize>, artifact_identity: Option<usize>, closing: bool,
}

impl MeshIndex {
    /// 🌱️ Constructs empty location storage without reading mesh content.
    pub fn new() -> Self { Self { positions: Vec::new(), normals: Vec::new(), indices: Vec::new(), counts: [0; 12], leaf: 0, offset: 0, total: 0, previous: None, utf8_remaining: 0, utf8_min: 0x80, utf8_max: 0xbf, box_mesh: false, empty_mesh: false, ready: false, handle_identity: None, artifact_identity: None, closing: false } }

    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn is_ready(&self) -> bool { self.ready && !self.closing }
    pub fn terminal_is_empty(&self) -> bool { self.positions.capacity() == 0 && self.normals.capacity() == 0 && self.indices.capacity() == 0 }
    pub fn next_release_demand(&self) -> usize { [&self.positions, &self.normals, &self.indices].into_iter().find(|values| values.capacity() != 0).map_or(0, |values| values.capacity() * size_of::<u32>()) }
    pub fn close_step(&mut self, items: usize, copy_bytes: usize, release_bytes: usize, depth: usize, fuel: u64) -> Result<IndexCloseTurn, SourceFailure> {
        if !self.closing { return Err(SourceFailure { reason: "mesh index is not closing", work: 0 }); }
        let release = self.next_release_demand();
        if release == 0 { return Ok(IndexCloseTurn { complete: true, copied_bytes: 0, released_bytes: 0, work: 0 }); }
        if items == 0 || copy_bytes < size_of::<Vec<u32>>() || release_bytes < release || depth == 0 || fuel == 0 { return Ok(IndexCloseTurn { complete: false, copied_bytes: 0, released_bytes: 0, work: 0 }); }
        let values = if self.positions.capacity() != 0 { &mut self.positions } else if self.normals.capacity() != 0 { &mut self.normals } else { &mut self.indices };
        *values = Vec::new();
        Ok(IndexCloseTurn { complete: self.terminal_is_empty(), copied_bytes: size_of::<Vec<u32>>(), released_bytes: release, work: 1 })
    }

    /// 📏️ Quotes the next original scalar-index allocation before touching its source.
    pub fn next_capacity_demand(&self) -> usize {
        if self.offset == 0 { return 0; }
        let values = match self.previous { Some(0) => &self.positions, Some(1) => &self.normals, Some(3) => &self.indices, _ => return 0 };
        if values.capacity() == 0 { MAX_SCALARS * size_of::<u32>() } else { 0 }
    }

    fn finish(&mut self) -> Result<(), &'static str> {
        let c = &self.counts; let vertices = c[0] / 3; let triangles = c[3] / 3;
        if c[0] == 0 || c[0] % 3 != 0 || c[3] == 0 || c[3] % 3 != 0 || vertices > 512 || triangles > 512 || !(c[1] == 0 || c[1] == c[0]) || !(c[2] == 0 || c[2] == vertices * 3 || c[2] == vertices * 4) || !(c[4] == 0 || c[4] == vertices * 2) || !(c[5] == 0 || c[5] == triangles) || !(c[6] == 0 || c[6] == vertices) || c[7] > 512 * 6 || c[8] > 512 * 3 || c[9] > 512 * 4 || c[10] > 512 * 3 || c[11] > 24_576 { return Err("invalid bounded mesh envelope"); }
        self.ready = true; Ok(())
    }

    fn utf8(&mut self, byte: u8) -> Result<usize, &'static str> {
        if self.utf8_remaining > 0 { if byte < self.utf8_min || byte > self.utf8_max { return Err("invalid mesh texture UTF-8"); } self.utf8_remaining -= 1; self.utf8_min = 0x80; self.utf8_max = 0xbf; return Ok(3); }
        Ok(match byte { 0x00..=0x7f => 0, 0xc2..=0xdf => { self.utf8_remaining = 1; 1 }, 0xe0 => { self.utf8_remaining = 2; self.utf8_min = 0xa0; 2 }, 0xe1..=0xec | 0xee..=0xef => { self.utf8_remaining = 2; 1 }, 0xed => { self.utf8_remaining = 2; self.utf8_max = 0x9f; 2 }, 0xf0 => { self.utf8_remaining = 3; self.utf8_min = 0x90; 2 }, 0xf1..=0xf3 => { self.utf8_remaining = 3; 1 }, 0xf4 => { self.utf8_remaining = 3; self.utf8_max = 0x8f; 2 }, _ => return Err("invalid mesh texture UTF-8") })
    }

    /// ⏭️ Inspects one leaf header, scalar location or texture byte per turn.
    pub fn advance(&mut self, handle: &RemodelingMeshChild, declared: u64, original: Option<&RemodelingDurableArtifact>, items: usize, copy_bytes: usize, capacity_bytes: usize, depth: usize, fuel: u64) -> Result<IndexTurn, SourceFailure> {
        if items == 0 || copy_bytes < 128 || capacity_bytes < self.next_capacity_demand() || depth == 0 || fuel < 80 { return Ok(IndexTurn { complete: self.ready, copied_bytes: 0, retained_capacity_bytes: 0, work: 0 }); }
        if self.closing { return Err(SourceFailure { reason: "mesh index is closing", work: 1 }); }
        let mut work = 1;
        let handle_identity = handle as *const RemodelingMeshChild as usize;
        if self.handle_identity.is_some_and(|identity| identity != handle_identity) { return Err(SourceFailure { reason: "mesh handle source changed", work }); }
        if self.ready {
            if !self.box_mesh && !self.empty_mesh && !original.is_some_and(|artifact| self.artifact_identity == Some(artifact as *const RemodelingDurableArtifact as usize)) { return Err(SourceFailure { reason: "mesh artifact source changed", work }); }
            return Ok(IndexTurn { complete: true, copied_bytes: 0, retained_capacity_bytes: 0, work });
        }
        let (empty, boxed, compared) = constant_mesh(&handle.child_id); work += compared;
        if empty { self.handle_identity = Some(handle_identity); self.empty_mesh = true; self.ready = true; return Ok(IndexTurn { complete: true, copied_bytes: 2 + size_of::<Option<usize>>(), retained_capacity_bytes: 0, work }); }
        if boxed { self.handle_identity = Some(handle_identity); self.box_mesh = true; self.ready = true; return Ok(IndexTurn { complete: true, copied_bytes: 2 + size_of::<Option<usize>>(), retained_capacity_bytes: 0, work }); }
        work += 1;
        let artifact = original.filter(|value| value.kind == "mesh").ok_or(SourceFailure { reason: "mesh content is absent", work })?;
        work += 1;
        if declared != artifact.chunks.len() as u64 || declared == 0 { return Err(SourceFailure { reason: "mesh leaf count differs", work }); }
        let artifact_identity = artifact as *const RemodelingDurableArtifact as usize;
        if self.artifact_identity.is_some_and(|identity| identity != artifact_identity) { return Err(SourceFailure { reason: "mesh artifact source changed", work }); }
        let binding = usize::from(self.handle_identity.is_none()) * size_of::<Option<usize>>() + usize::from(self.artifact_identity.is_none()) * size_of::<Option<usize>>();
        work += 1;
        if self.leaf == artifact.chunks.len() { self.finish().map_err(|reason| SourceFailure { reason, work })?; return Ok(IndexTurn { complete: true, copied_bytes: binding + 1, retained_capacity_bytes: 0, work }); }
        let chunk = remodeling_durable_chunk(&artifact.chunks[self.leaf]).ok_or(SourceFailure { reason: "mesh leaf exceeds bound", work })?;
        work += 1;
        if self.offset == 0 {
            let (&tag, values) = chunk.split_first().ok_or(SourceFailure { reason: "empty mesh leaf", work })?;
            if tag > 11 || self.previous.is_some_and(|last| tag < last) { return Err(SourceFailure { reason: "invalid mesh field order", work }); }
            let total = self.total.checked_add(chunk.len()).filter(|total| *total <= MAX_BYTES).ok_or(SourceFailure { reason: "mesh content exceeds bound", work })?;
            if tag != 10 && tag != 11 && values.len() % 4 != 0 { return Err(SourceFailure { reason: "invalid mesh scalar length", work }); }
            let units = if tag == 10 || tag == 11 { values.len() } else { values.len() / 4 };
            let count = self.counts[tag as usize].checked_add(units).ok_or(SourceFailure { reason: "mesh field count overflow", work })?;
            let limit = match tag { 0 | 1 => MAX_SCALARS, 2 => 512 * 4, 3 => MAX_SCALARS, 4 => 512 * 2, 5 => 512, 6 => 512, 7 => 512 * 6, 8 | 10 => 512 * 3, 9 => 512 * 4, _ => 24_576 };
            if count > limit { return Err(SourceFailure { reason: "mesh field exceeds bound", work }); }
            if self.handle_identity.is_none() { self.handle_identity = Some(handle_identity); }
            if self.artifact_identity.is_none() { self.artifact_identity = Some(artifact_identity); }
            self.total = total; self.previous = Some(tag); self.offset = 1;
            let mut copied = binding + 2 * size_of::<usize>() + size_of::<Option<u8>>();
            if !matches!(tag, 0 | 1 | 3 | 11) { self.counts[tag as usize] = count; self.leaf += 1; self.offset = 0; copied += 3 * size_of::<usize>(); }
            return Ok(IndexTurn { complete: false, copied_bytes: copied, retained_capacity_bytes: 0, work });
        }
        let tag = chunk[0];
        if self.offset == chunk.len() { if tag == 11 && self.utf8_remaining != 0 { return Err(SourceFailure { reason: "incomplete mesh texture UTF-8", work }); } self.leaf += 1; self.offset = 0; return Ok(IndexTurn { complete: false, copied_bytes: binding + 2 * size_of::<usize>(), retained_capacity_bytes: 0, work }); }
        if tag == 11 { let copied = self.utf8(chunk[self.offset]).map_err(|reason| SourceFailure { reason, work })?; self.offset += 1; self.counts[11] += 1; return Ok(IndexTurn { complete: false, copied_bytes: binding + 2 * size_of::<usize>() + copied, retained_capacity_bytes: 0, work }); }
        let values = match tag { 0 => &mut self.positions, 1 => &mut self.normals, 3 => &mut self.indices, _ => return Err(SourceFailure { reason: "invalid indexed mesh field", work }) };
        if values.capacity() == 0 {
            let layout = std::alloc::Layout::array::<u32>(MAX_SCALARS).map_err(|_| SourceFailure { reason: "mesh index allocation overflow", work })?;
            let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<u32>()).ok_or(SourceFailure { reason: "mesh index allocation refused", work })?;
            *values = unsafe { Vec::from_raw_parts(pointer.as_ptr(), 0, MAX_SCALARS) };
            return Ok(IndexTurn { complete: false, copied_bytes: size_of::<Vec<u32>>(), retained_capacity_bytes: layout.size(), work });
        }
        let packed = u32::try_from(self.leaf).ok().and_then(|leaf| leaf.checked_shl(12)).and_then(|leaf| leaf.checked_add(self.offset as u32)).ok_or(SourceFailure { reason: "mesh leaf location overflow", work })?;
        values.push(packed);
        self.counts[tag as usize] += 1; self.offset += 4; Ok(IndexTurn { complete: false, copied_bytes: binding + 4 + 3 * size_of::<usize>(), retained_capacity_bytes: 0, work })
    }

    /// 🔎️ Borrows the exact immutable artifact supplied by the original metered lookup.
    pub fn source<'a>(&'a self, original: Option<&'a RemodelingDurableArtifact>) -> MeshSource<'a> { MeshSource { index: self, original } }
}

impl Drop for MeshIndex {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "mesh index requires paid retirement"); }
}

/// 🧊️ Read-only scalar projection over an indexed immutable snapshot.
pub struct MeshSource<'a> { index: &'a MeshIndex, original: Option<&'a RemodelingDurableArtifact> }

impl MeshSource<'_> {
    fn artifact(&self, work: &mut u64) -> Option<&RemodelingDurableArtifact> {
        *work += 1;
        self.original
    }
    fn scalar(artifact: &RemodelingDurableArtifact, location: u32, work: &mut u64) -> Option<u32> {
        *work += 1;
        let leaf = remodeling_durable_chunk(artifact.chunks.get((location >> 12) as usize)?)?;
        let at = (location & 0xfff) as usize;
        Some(u32::from_le_bytes(leaf.get(at..at + 4)?.try_into().ok()?))
    }
    fn vector(&self, locations: &[u32], vertex: usize, work: &mut u64) -> Option<[f32; 3]> {
        *work += 1;
        let start = vertex.checked_mul(3)?; let values = locations.get(start..start.checked_add(3)?)?;
        let artifact = self.artifact(work)?;
        Some([f32::from_bits(Self::scalar(artifact, values[0], work)?), f32::from_bits(Self::scalar(artifact, values[1], work)?), f32::from_bits(Self::scalar(artifact, values[2], work)?)])
    }
    fn box_position(vertex: usize) -> Option<[f32; 3]> {
        let faces = [([-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]),([0.5,-0.5,-0.5],[-0.5,-0.5,-0.5],[-0.5,0.5,-0.5],[0.5,0.5,-0.5]),([-0.5,0.5,0.5],[0.5,0.5,0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5]),([-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,-0.5,0.5],[-0.5,-0.5,0.5]),([0.5,-0.5,0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[0.5,0.5,0.5]),([-0.5,-0.5,-0.5],[-0.5,-0.5,0.5],[-0.5,0.5,0.5],[-0.5,0.5,-0.5])];
        let face = faces.get(vertex / 6)?; let corners = [face.0,face.1,face.2,face.3]; Some(corners[[0,1,2,0,2,3][vertex % 6]])
    }
}

impl GlbSource for MeshSource<'_> {
    fn read_work(&self) -> u64 { 5 }
    fn valid_layout(&self) -> bool { !self.index.closing && self.index.ready && (self.index.box_mesh || self.index.empty_mesh || self.original.is_some_and(|artifact| self.index.artifact_identity == Some(artifact as *const RemodelingDurableArtifact as usize))) }
    fn vertices(&self) -> usize { if self.index.box_mesh { 36 } else { self.index.counts[0] / 3 } }
    fn indices(&self) -> usize { if self.index.box_mesh { 36 } else { self.index.counts[3] } }
    fn has_normals(&self) -> bool { !self.index.box_mesh && self.index.counts[1] == self.index.counts[0] }
    fn position(&self, vertex: usize) -> (Option<[f32; 3]>, u64) { if self.index.box_mesh { return (Self::box_position(vertex), 1); } let mut work = 0; let value = self.vector(&self.index.positions[..self.index.counts[0]], vertex, &mut work); (value, work) }
    fn normal(&self, vertex: usize) -> (Option<[f32; 3]>, u64) { let mut work = 0; let value = self.vector(&self.index.normals[..self.index.counts[1]], vertex, &mut work); (value, work) }
    fn index(&self, index: usize) -> (Option<u32>, u64) { if self.index.box_mesh { return ((index < 36).then_some(index as u32), 1); } let mut work = 1; let value = self.index.indices[..self.index.counts[3]].get(index).and_then(|location| self.artifact(&mut work).and_then(|artifact| Self::scalar(artifact, *location, &mut work))); (value, work) }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
