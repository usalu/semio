//! 🧮️ Gestures that rewrite rows where they are: winding flips, grid snapping, uv unwrapping and face deletion.

use super::{FaceId, MeshKernelError, MeshOverlay, MeshResult, Orphans, VertexId};
use std::collections::{BTreeMap, BTreeSet};

impl MeshOverlay<'_> {
    /// 🔃️ Reverses the winding of the faces: each face is replaced by its reversed loop (a face named an even number of times keeps
    /// its winding); see [`HalfedgeMesh::flip_faces`].
    pub fn flip_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("face flipping")?;
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let soup = self.soup()?;
        let mut flipped: BTreeMap<u32, bool> = BTreeMap::new();
        for face in faces {
            if soup.faces.get(face.0 as usize).is_none_or(|corners| corners.is_empty()) {
                return Err(MeshKernelError::InvalidHandle);
            }
            let held = flipped.entry(face.0).or_insert(false);
            *held = !*held;
        }
        for (face, reversed) in flipped {
            if reversed {
                let mut corners = soup.faces[face as usize].clone();
                corners.reverse();
                self.replace_face(face, &corners)?;
            }
        }
        self.finish(Orphans::Keep)
    }

    /// 🧲️ Rounds the named vertices to the nearest grid point; a vertex already on the grid states no row; see
    /// [`HalfedgeMesh::snap_vertices_to_grid`].
    pub fn snap_vertices_to_grid(&mut self, vertices: &[VertexId], grid: f32) -> MeshResult<()> {
        self.untouched()?;
        if !grid.is_finite() || grid <= 0.0 {
            return Err(MeshKernelError::InvalidInput("grid must be finite and positive".into()));
        }
        if vertices.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let selected: BTreeSet<u32> = vertices.iter().map(|vertex| vertex.0).collect();
        let mut snapped: Vec<(u32, [f32; 3])> = Vec::with_capacity(selected.len());
        for vertex in selected {
            let held = self.vertex(VertexId(vertex)).ok_or(MeshKernelError::InvalidHandle)?.position;
            let position = held.map(|coordinate| ((coordinate as f64 / grid as f64).round() * grid as f64) as f32);
            if position.iter().any(|coordinate| !coordinate.is_finite()) {
                return Err(MeshKernelError::DegenerateOperation);
            }
            snapped.push((vertex, position));
        }
        for (vertex, position) in snapped {
            self.move_vertex(vertex, position)?;
        }
        Ok(())
    }

    /// 🗺️ Writes the unwrapped texture coordinate of every corner: the kernel's conformal unwrap computes the coordinates from the
    /// mesh's seams and the gesture states each corner's uv (a corner that keeps its coordinate states no row); see
    /// [`HalfedgeMesh::unwrap_uv`].
    pub fn unwrap_uv(&mut self) -> MeshResult<()> {
        self.untouched()?;
        let unwrapped = self.base.unwrap_uv_job()?.finish_with_progress()?;
        let (_, halfedges, ..) = unwrapped.into_owned_parts();
        for (index, corner) in halfedges.iter().enumerate() {
            self.set_corner_uv(index as u32, corner.uv);
        }
        Ok(())
    }

    /// 🗑️ Removes the faces and every vertex no face references any more (the other corners keep their rows and uv); an id that names
    /// no face is ignored; see [`HalfedgeMesh::delete_faces`].
    pub fn delete_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("face deletion")?;
        let doomed: BTreeSet<u32> = faces.iter().map(|face| face.0).filter(|face| self.live_face(*face)).collect();
        if doomed.is_empty() {
            return Ok(());
        }
        for face in doomed {
            self.kill_face(face)?;
        }
        self.finish(Orphans::All)
    }
}
