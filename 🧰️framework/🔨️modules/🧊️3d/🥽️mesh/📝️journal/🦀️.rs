//! 📝️ Write-recording view over a READ-ONLY half-edge mesh. A [`MeshOverlay`] borrows the base mesh, answers every read from it
//! (falling through to the rows the overlay has written) and records every write — vertex / halfedge / face rows, arena lengths,
//! uv seams — in a [`MeshJournal`], coalesced per row key in write order (the last write to a key wins). The journal IS the edit:
//! callers state exactly the rows a gesture wrote and never compare a before state with an after state.
//!
//! ✍️ Every topology gesture is a row emitter. It READS the overlay (the base plus the rows it has written so far), decides which
//! faces it replaces, removes or appends and which vertices it moves or appends, and states those edits through the face-level
//! editor ([`MeshOverlay::add_face`], [`MeshOverlay::replace_face`], [`MeshOverlay::kill_face`], [`MeshOverlay::push_vertex`],
//! [`MeshOverlay::move_vertex`]). The editor writes the half-edge rows of exactly those faces — new rows are appended, a replaced face
//! keeps every halfedge whose directed edge survives, a removed face's halfedges are recycled by the faces that need the same
//! directed edge and compacted away by moving the tail rows into the holes — so the journal names the rows the edit semantically
//! writes and nothing else. There is no assembled-mesh construction and no comparison against the base anywhere; a field setter
//! states a row only when the gesture changes that field.
//!
//! 🧭️ Derived data stays out of the rows: a gesture states no vertex normal (new vertices carry `None`); the artifact's central
//! applier re-derives normals from the applied primary rows.
//!
//! The base mesh is never mutated and never cloned into a working copy by this module.

use semio_framework_3d_mesh_document::{EdgeId, FaceId, HalfEdge, HalfedgeMesh, MeshAttribute, MeshFace, MeshKernelError, MeshResult, MeshTexture, MeshVertex, VertexId};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[path = "✍️editor/🦀️.rs"]
mod editor;
use editor::Orphans;

#[path = "🏗️gestures/🦀️.rs"]
mod gestures;

//#region 🔖️Journal
/// 🧾️ The rows written into one index-addressed arena: its new length when a write changed it and the written row at every index
/// the gesture wrote (rows past the base length are added rows).
#[derive(Clone, Debug)]
pub struct ArenaJournal<T> {
    pub len: Option<usize>,
    pub rows: BTreeMap<u32, T>,
}

impl<T> Default for ArenaJournal<T> {
    fn default() -> Self {
        Self { len: None, rows: BTreeMap::new() }
    }
}

impl<T> ArenaJournal<T> {
    /// 🫥️ Whether nothing was written.
    pub fn is_empty(&self) -> bool {
        self.len.is_none() && self.rows.is_empty()
    }

    /// ✍️ Records one row write; the last write of a key wins.
    fn put(&mut self, index: u32, row: T) {
        self.rows.insert(index, row);
    }

    /// 📏️ Records the arena's final length against the base length and drops the rows the new length cuts off.
    fn resize(&mut self, base_len: usize, len: usize) {
        self.len = (len != base_len).then_some(len);
        self.rows.retain(|index, _| (*index as usize) < len);
    }
}

/// 🧾️ Everything one gesture wrote into a mesh, in the kernel's own row types.
#[derive(Clone, Debug, Default)]
pub struct MeshJournal {
    pub vertices: ArenaJournal<MeshVertex>,
    pub halfedges: ArenaJournal<HalfEdge>,
    pub faces: ArenaJournal<MeshFace>,
    pub seams_added: BTreeSet<u32>,
    pub seams_removed: BTreeSet<u32>,
    pub attributes: ArenaJournal<(String, MeshAttribute)>,
    pub materials: ArenaJournal<(String, DslValue)>,
    pub textures: ArenaJournal<(String, MeshTexture)>,
}

impl MeshJournal {
    /// 🫥️ Whether the gesture wrote nothing.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() && self.halfedges.is_empty() && self.faces.is_empty() && self.seams_added.is_empty() && self.seams_removed.is_empty() && self.attributes.is_empty() && self.materials.is_empty() && self.textures.is_empty()
    }
}

fn same_vertex(left: &MeshVertex, right: &MeshVertex) -> bool {
    left.position.map(f32::to_bits) == right.position.map(f32::to_bits) && left.normal.map(|normal| normal.map(f32::to_bits)) == right.normal.map(|normal| normal.map(f32::to_bits)) && left.halfedge == right.halfedge
}

fn same_halfedge(left: &HalfEdge, right: &HalfEdge) -> bool {
    left.vertex == right.vertex && left.twin == right.twin && left.next == right.next && left.face == right.face && left.uv.map(f32::to_bits) == right.uv.map(f32::to_bits)
}

fn same_face(left: &MeshFace, right: &MeshFace) -> bool {
    left.halfedge == right.halfedge && left.smooth == right.smooth && left.flipped == right.flipped
}
//#endregion 🔖️Journal

//#region 🔖️Overlay
/// 🔗️ The directed-edge index of the faces the overlay shows: the halfedge of every `(origin, destination)` pair and the outgoing
/// halfedges of every vertex. It is a read cache of the overlay, maintained by the editor as it writes rows.
#[derive(Default)]
struct Links {
    edges: HashMap<(u32, u32), u32>,
    out: HashMap<u32, Vec<u32>>,
}

/// 👁️ A write-recording view over a read-only base mesh.
pub struct MeshOverlay<'a> {
    base: &'a HalfedgeMesh,
    journal: MeshJournal,
    links: Option<Links>,
    dead_faces: BTreeSet<u32>,
    dead_halfedges: BTreeSet<u32>,
    released: HashMap<(u32, u32), u32>,
    orphans: BTreeSet<u32>,
    vertex_moves: Vec<(u32, u32)>,
    placed: BTreeSet<u32>,
}

impl<'a> MeshOverlay<'a> {
    /// 🆕️ An overlay that has written nothing over `base`.
    pub fn new(base: &'a HalfedgeMesh) -> Self {
        Self { base, journal: MeshJournal::default(), links: None, dead_faces: BTreeSet::new(), dead_halfedges: BTreeSet::new(), released: HashMap::new(), orphans: BTreeSet::new(), vertex_moves: Vec::new(), placed: BTreeSet::new() }
    }

    /// 🔭️ The read-only base mesh.
    pub fn base(&self) -> &'a HalfedgeMesh {
        self.base
    }

    /// 🧾️ The rows written so far.
    pub fn journal(&self) -> &MeshJournal {
        &self.journal
    }

    /// 🧾️ The rows the gesture wrote.
    pub fn into_journal(self) -> MeshJournal {
        self.journal
    }

    //#region 🔖️Reads
    /// 📏️ The visible length of the vertex arena.
    pub fn vertex_len(&self) -> usize {
        self.journal.vertices.len.unwrap_or(self.base.vertices.len())
    }

    /// 📏️ The visible length of the halfedge arena.
    pub fn halfedge_len(&self) -> usize {
        self.journal.halfedges.len.unwrap_or(self.base.halfedges.len())
    }

    /// 📏️ The visible length of the face arena.
    pub fn face_len(&self) -> usize {
        self.journal.faces.len.unwrap_or(self.base.faces.len())
    }

    /// 📍️ The vertex row as the overlay sees it: the written row, else the base row, `None` past the visible length.
    pub fn vertex(&self, id: VertexId) -> Option<&MeshVertex> {
        if (id.0 as usize) >= self.vertex_len() {
            return None;
        }
        self.journal.vertices.rows.get(&id.0).or_else(|| self.base.vertices.get(id.0 as usize))
    }

    /// 🔗️ The halfedge row as the overlay sees it.
    pub fn halfedge(&self, index: u32) -> Option<&HalfEdge> {
        if (index as usize) >= self.halfedge_len() {
            return None;
        }
        self.journal.halfedges.rows.get(&index).or_else(|| self.base.halfedges.get(index as usize))
    }

    /// 🔷️ The face row as the overlay sees it.
    pub fn face(&self, id: FaceId) -> Option<&MeshFace> {
        if (id.0 as usize) >= self.face_len() {
            return None;
        }
        self.journal.faces.rows.get(&id.0).or_else(|| self.base.faces.get(id.0 as usize))
    }

    //#endregion 🔖️Reads

    //#region 🔖️Writes
    /// ✍️ Writes one vertex row.
    pub fn write_vertex(&mut self, index: u32, row: MeshVertex) {
        self.journal.vertices.put(index, row);
    }

    /// ✍️ Writes one halfedge row.
    pub fn write_halfedge(&mut self, index: u32, row: HalfEdge) {
        self.journal.halfedges.put(index, row);
    }

    /// ✍️ Writes one face row.
    pub fn write_face(&mut self, index: u32, row: MeshFace) {
        self.journal.faces.put(index, row);
    }

    /// 📏️ Sets the final length of the vertex arena.
    pub fn resize_vertices(&mut self, len: usize) {
        self.journal.vertices.resize(self.base.vertices.len(), len);
    }

    /// 📏️ Sets the final length of the halfedge arena.
    pub fn resize_halfedges(&mut self, len: usize) {
        self.journal.halfedges.resize(self.base.halfedges.len(), len);
    }

    /// 📏️ Sets the final length of the face arena.
    pub fn resize_faces(&mut self, len: usize) {
        self.journal.faces.resize(self.base.faces.len(), len);
    }

    fn write_seam(&mut self, seam: u32, marked: bool) {
        let held = self.base.uv_seams.contains(&seam);
        if marked {
            self.journal.seams_removed.remove(&seam);
            if !held {
                self.journal.seams_added.insert(seam);
            }
        } else {
            self.journal.seams_added.remove(&seam);
            if held {
                self.journal.seams_removed.insert(seam);
            }
        }
    }

    /// 🪡️ Marks or clears the seam on each named edge and its twin as one pair (an edge that names no halfedge is ignored): the
    /// local row edit behind `mark_uv_seam`, written straight into the journal.
    pub fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool) {
        for edge in edges {
            let Some(twin) = self.base.halfedges.get(edge.0 as usize).map(|halfedge| halfedge.twin) else {
                continue;
            };
            if seam {
                self.write_seam(twin.map_or(edge.0, |twin| twin.min(edge.0)), true);
            } else {
                self.write_seam(edge.0, false);
                if let Some(twin) = twin {
                    self.write_seam(twin, false);
                }
            }
        }
    }
    //#endregion 🔖️Writes

    //#region 🔖️Guards
    fn untouched(&self) -> MeshResult<()> {
        if self.journal.is_empty() {
            Ok(())
        } else {
            Err(MeshKernelError::InvalidInput("a topology gesture runs on an untouched overlay".into()))
        }
    }

    /// 🧱️ Topology gestures renumber corners, so authored channels and uv seams (which address them) need a declared source remap.
    fn require_plain(&self, operation: &str) -> MeshResult<()> {
        if let Some((name, _)) = self.base.attributes.first_key_value() {
            return Err(MeshKernelError::InvalidInput(format!("mesh attribute '{name}' requires declared source remapping for {operation}")));
        }
        if !self.base.uv_seams.is_empty() {
            return Err(MeshKernelError::InvalidInput(format!("mesh UV seams require declared source remapping for {operation}")));
        }
        Ok(())
    }
    //#endregion 🔖️Guards
}
//#endregion 🔖️Overlay

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
