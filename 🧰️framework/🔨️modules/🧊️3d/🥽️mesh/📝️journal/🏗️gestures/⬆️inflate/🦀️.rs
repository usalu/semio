//! ⬆️ Extrusion and inset as row emitters: the selected faces get their new loop in place, the side or border faces are appended, the
//! displaced or inner vertices are appended.

use super::{select_faces, FaceId, MeshKernelError, MeshOverlay, MeshResult, Orphans, Vec3};
use std::collections::{BTreeMap, BTreeSet};

impl MeshOverlay<'_> {
    /// ⬆️ Extrudes the faces along their vertex-summed normal: every vertex of the selection gets a displaced twin, the selected faces
    /// move onto the twins and a side quad joins each boundary edge of the selection to its twin; see [`HalfedgeMesh::extrude_faces`].
    pub fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("extrusion")?;
        if !distance.is_finite() || distance == 0.0 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let soup = self.soup()?;
        let input = select_faces(&soup, faces, "duplicate selected face")?;
        let mut sums: BTreeMap<u32, Vec3> = BTreeMap::new();
        let mut boundary: BTreeMap<(u32, u32), (u32, u32, usize, u32)> = BTreeMap::new();
        let mut edges: BTreeSet<(u32, u32)> = BTreeSet::new();
        for id in &input {
            let corners = &soup.faces[*id as usize];
            let normal = soup.normals[*id as usize];
            for index in 0..corners.len() {
                let (a, b) = (corners[index], corners[(index + 1) % corners.len()]);
                sums.entry(a).and_modify(|sum| *sum = sum.add(normal)).or_insert(normal);
                let edge = boundary.entry((a.min(b), a.max(b))).or_insert((a, b, 0, *id));
                if edge.2 == 0 {
                    edges.insert((a, b));
                } else {
                    edges.remove(&(edge.0, edge.1));
                }
                edge.2 += 1;
            }
        }
        let mut points: Vec<(u32, [f32; 3])> = Vec::with_capacity(sums.len());
        for (vertex, sum) in &sums {
            let normal = sum.normalize();
            if normal.length() < 1e-6 {
                return Err(MeshKernelError::DegenerateOperation);
            }
            let point = Vec3(soup.positions[*vertex as usize]).add(normal.scale(distance)).0;
            if point.iter().any(|coordinate| !coordinate.is_finite()) {
                return Err(MeshKernelError::DegenerateOperation);
            }
            points.push((*vertex, point));
        }
        let mut displaced: BTreeMap<u32, u32> = BTreeMap::new();
        for (vertex, point) in points {
            displaced.insert(vertex, self.push_vertex(point));
        }
        for id in &input {
            let moved: Vec<u32> = soup.faces[*id as usize].iter().map(|vertex| displaced[vertex]).collect();
            self.replace_face(*id, &moved)?;
        }
        for (a, b) in &edges {
            let source = boundary[&(*a.min(b), *a.max(b))].3;
            self.add_face(&[*a, *b, displaced[b], displaced[a]], soup.smooth[source as usize])?;
        }
        self.finish(Orphans::Keep)
    }

    /// 🔲️ Insets the faces: every selected face shrinks onto an inner loop and a border quad joins each of its edges to the loop;
    /// see [`HalfedgeMesh::inset_faces`].
    pub fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("inset")?;
        if !amount.is_finite() || amount <= 0.0 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let soup = self.soup()?;
        let input = select_faces(&soup, faces, "duplicate selected face")?;
        let mut inner: Vec<(u32, Vec<[f32; 3]>)> = Vec::with_capacity(input.len());
        for id in &input {
            let corners = &soup.faces[*id as usize];
            let count = corners.len();
            let normal = soup.normals[*id as usize];
            let point = |index: usize| Vec3(soup.positions[corners[index] as usize]);
            let mut ring = Vec::with_capacity(count);
            for index in 0..count {
                let (previous, current, next) = (point((index + count - 1) % count), point(index), point((index + 1) % count));
                let a = normal.cross(current.sub(previous).normalize());
                let b = normal.cross(next.sub(current).normalize());
                let denominator = 1.0 + a.dot(b);
                if denominator <= 1e-6 {
                    return Err(MeshKernelError::DegenerateOperation);
                }
                let offset = a.add(b).scale(amount / denominator);
                if offset.length() >= current.sub(previous).length().min(next.sub(current).length()) * 0.5 {
                    return Err(MeshKernelError::InvalidInput("inset exceeds local edge clearance".into()));
                }
                let position = current.add(offset).0;
                if position.iter().any(|coordinate| !coordinate.is_finite()) {
                    return Err(MeshKernelError::DegenerateOperation);
                }
                ring.push(position);
            }
            inner.push((*id, ring));
        }
        for (id, ring) in inner {
            let ids: Vec<u32> = ring.into_iter().map(|position| self.push_vertex(position)).collect();
            let corners = soup.faces[id as usize].clone();
            let count = corners.len();
            self.replace_face(id, &ids)?;
            for index in 0..count {
                let next = (index + 1) % count;
                self.add_face(&[corners[index], corners[next], ids[next], ids[index]], soup.smooth[id as usize])?;
            }
        }
        self.finish(Orphans::Keep)
    }
}
