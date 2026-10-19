//! 🧲️ Vertex merging, edge dissolution and mirroring as row emitters: faces that change are replaced in place, faces that vanish are
//! removed, mirrored faces and vertices are appended.

use super::{EdgeId, MeshKernelError, MeshOverlay, MeshResult, MirrorAxis, Orphans, Vec3, VertexId, WeldMode};
use std::collections::{BTreeMap, BTreeSet};

fn reject() -> MeshKernelError {
    MeshKernelError::InvalidInput("selected dissolution cannot preserve the boundary contract".into())
}

impl MeshOverlay<'_> {
    /// 🧲️ Welds the named vertices onto one of them (the first, their centre, or per distance cluster); faces keep the collapsed loop
    /// and vanish when fewer than three corners remain; see [`HalfedgeMesh::merge_vertices`].
    pub fn merge_vertices(&mut self, vertices: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("vertex merge")?;
        if vertices.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        if mode == WeldMode::ByDistance && (!threshold.is_finite() || threshold < 0.0) {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let soup = self.soup()?;
        let mut selected: Vec<u32> = Vec::new();
        let mut seen: BTreeSet<u32> = BTreeSet::new();
        for vertex in vertices {
            if vertex.0 as usize >= soup.positions.len() {
                return Err(MeshKernelError::InvalidHandle);
            }
            if seen.insert(vertex.0) {
                selected.push(vertex.0);
            }
        }
        if selected.len() < 2 {
            return Err(MeshKernelError::EmptySelection);
        }
        let mut roots: Vec<usize> = (0..selected.len()).map(|index| if mode == WeldMode::ByDistance { index } else { 0 }).collect();
        if mode == WeldMode::ByDistance {
            for i in 1..selected.len() {
                for j in 0..i {
                    let distance = Vec3(soup.positions[selected[i] as usize]).sub(Vec3(soup.positions[selected[j] as usize])).length();
                    if distance <= threshold {
                        let (mut a, mut b) = (i, j);
                        while roots[a] != a {
                            a = roots[a];
                        }
                        while roots[b] != b {
                            b = roots[b];
                        }
                        roots[a.max(b)] = a.min(b);
                    }
                }
            }
        }
        let mut remap: Vec<u32> = (0..soup.positions.len() as u32).collect();
        let mut centre = [0.0_f64; 3];
        for cursor in 0..selected.len() {
            let mut root = cursor;
            while roots[root] != root {
                root = roots[root];
            }
            remap[selected[cursor] as usize] = selected[root];
            if mode == WeldMode::Center {
                for axis in 0..3 {
                    centre[axis] += soup.positions[selected[cursor] as usize][axis] as f64 / selected.len() as f64;
                }
            }
        }
        if mode == WeldMode::Center {
            self.move_vertex(selected[0], centre.map(|coordinate| coordinate as f32))?;
        }
        for (face, corners) in soup.faces.iter().enumerate() {
            if corners.is_empty() || corners.iter().all(|vertex| remap[*vertex as usize] == *vertex) {
                continue;
            }
            let mut collapsed: Vec<u32> = Vec::with_capacity(corners.len());
            for vertex in corners {
                let id = remap[*vertex as usize];
                if collapsed.last() != Some(&id) {
                    collapsed.push(id);
                }
            }
            if collapsed.first() == collapsed.last() {
                collapsed.pop();
            }
            let distinct: BTreeSet<u32> = collapsed.iter().copied().collect();
            if distinct.len() != collapsed.len() {
                return Err(MeshKernelError::NonManifold);
            }
            if collapsed.len() >= 3 {
                self.replace_face(face as u32, &collapsed)?;
            } else {
                self.kill_face(face as u32)?;
            }
        }
        self.finish(Orphans::Newly)
    }

    /// 💧️ Dissolves the named edges: the two faces on both sides of an edge become one face (the lower index keeps the merged loop, the
    /// higher one is removed); an edge the two faces share with a second edge is refused, an edge on the boundary is degenerate; see
    /// [`HalfedgeMesh::dissolve_edges`].
    pub fn dissolve_edges(&mut self, edges: &[EdgeId]) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("edge dissolution")?;
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let soup = self.soup()?;
        let mut pending: BTreeSet<(u32, u32)> = BTreeSet::new();
        for edge in edges {
            let (a, b) = self.base.edge_endpoints(*edge)?;
            pending.insert((a.0.min(b.0), a.0.max(b.0)));
        }
        let mut faces = soup.faces.clone();
        let mut incidence: BTreeMap<(u32, u32), (usize, usize)> = BTreeMap::new();
        for (face, corners) in faces.iter().enumerate() {
            for index in 0..corners.len() {
                if incidence.insert((corners[index], corners[(index + 1) % corners.len()]), (face, index)).is_some() {
                    return Err(MeshKernelError::NonManifold);
                }
            }
        }
        let mut merged_into: BTreeSet<usize> = BTreeSet::new();
        let mut removed: BTreeSet<usize> = BTreeSet::new();
        while let Some((u, v)) = pending.pop_first() {
            let (Some(&(a, ai)), Some(&(b, bi))) = (incidence.get(&(u, v)), incidence.get(&(v, u))) else {
                return Err(MeshKernelError::DegenerateOperation);
            };
            if a == b {
                continue;
            }
            let shared = (0..faces[a].len()).filter(|index| incidence.get(&(faces[a][(index + 1) % faces[a].len()], faces[a][*index])).is_some_and(|(other, _)| *other == b)).count();
            if shared > 1 {
                return Err(reject());
            }
            let (alen, blen) = (faces[a].len(), faces[b].len());
            let mut merged: Vec<u32> = Vec::with_capacity(alen + blen - 2);
            let mut distinct: BTreeSet<u32> = BTreeSet::new();
            for cursor in 0..alen + blen - 2 {
                let vertex = if cursor == 0 {
                    faces[a][ai]
                } else if cursor < blen - 1 {
                    faces[b][(bi + cursor + 1) % blen]
                } else {
                    faces[a][(ai + cursor - (blen - 2)) % alen]
                };
                if !distinct.insert(vertex) {
                    return Err(reject());
                }
                merged.push(vertex);
            }
            for face in [a, b] {
                let corners = &faces[face];
                for index in 0..corners.len() {
                    incidence.remove(&(corners[index], corners[(index + 1) % corners.len()]));
                }
            }
            let (keep, gone) = (a.min(b), a.max(b));
            for index in 0..merged.len() {
                if incidence.insert((merged[index], merged[(index + 1) % merged.len()]), (keep, index)).is_some() {
                    return Err(MeshKernelError::NonManifold);
                }
            }
            faces[keep] = merged;
            faces[gone] = Vec::new();
            merged_into.insert(keep);
            removed.insert(gone);
        }
        for face in &removed {
            self.kill_face(*face as u32)?;
        }
        for face in &merged_into {
            if !removed.contains(face) {
                self.replace_face(*face as u32, &faces[*face])?;
            }
        }
        self.finish(Orphans::Newly)
    }

    /// 🪞️ Reflects the geometry across the axis plane and welds the seam: vertices within half the threshold of the plane snap onto it
    /// and are shared with their reflection, every face gains a reflected twin of reversed winding and faces lying in the plane
    /// vanish; see [`HalfedgeMesh::mirror`].
    pub fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("mirror")?;
        if !weld_threshold.is_finite() || weld_threshold < 0.0 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let axis = match axis {
            MirrorAxis::X => 0,
            MirrorAxis::Y => 1,
            MirrorAxis::Z => 2,
        };
        let tolerance = weld_threshold * 0.5;
        let soup = self.soup()?;
        let (mut positive, mut negative) = (false, false);
        for position in &soup.positions {
            positive |= position[axis] > tolerance;
            negative |= position[axis] < -tolerance;
            if positive && negative {
                return Err(MeshKernelError::InvalidInput("mirror geometry must lie on one side of its plane".into()));
            }
        }
        let on_plane: Vec<bool> = soup.positions.iter().map(|position| position[axis].abs() <= tolerance).collect();
        for (vertex, position) in soup.positions.iter().enumerate() {
            if on_plane[vertex] {
                let mut snapped = *position;
                snapped[axis] = 0.0;
                self.move_vertex(vertex as u32, snapped)?;
            }
        }
        let mut reflected: BTreeMap<u32, u32> = BTreeMap::new();
        for (face, corners) in soup.faces.iter().enumerate() {
            if !corners.is_empty() && corners.iter().all(|vertex| on_plane[*vertex as usize]) {
                self.kill_face(face as u32)?;
            }
        }
        for (face, corners) in soup.faces.iter().enumerate() {
            if corners.is_empty() || corners.iter().all(|vertex| on_plane[*vertex as usize]) {
                continue;
            }
            let mut mirrored: Vec<u32> = Vec::with_capacity(corners.len());
            for index in 0..corners.len() {
                let vertex = corners[corners.len() - 1 - index];
                if on_plane[vertex as usize] {
                    mirrored.push(vertex);
                } else {
                    let id = match reflected.get(&vertex) {
                        Some(id) => *id,
                        None => {
                            let mut position = soup.positions[vertex as usize];
                            position[axis] = -position[axis];
                            let id = self.push_vertex(position);
                            reflected.insert(vertex, id);
                            id
                        }
                    };
                    mirrored.push(id);
                }
            }
            self.add_face(&mirrored, soup.smooth[face])?;
        }
        self.finish(Orphans::Newly)
    }
}
