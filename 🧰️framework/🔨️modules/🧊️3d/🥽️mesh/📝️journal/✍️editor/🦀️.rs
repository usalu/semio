//! ✍️ Face-level editor of a [`MeshOverlay`](super::MeshOverlay): the half-edge rows a face edit semantically writes, and nothing else.
//!
//! 🧱️ A face is edited by naming its vertex loop. Adding a face appends its halfedges and links the twins of its directed edges to
//! the opposite halfedges the overlay shows. Replacing a face keeps every halfedge whose directed edge survives (twin and uv
//! untouched) and releases the others. A released halfedge is not rewritten: a later face that needs the same directed edge takes the
//! slot back with its twin intact, so an edge a gesture leaves in place costs no neighbour row. Slots nobody takes back are compacted
//! away by [`MeshOverlay::finish`], which moves the tail rows into the holes (every move rewrites exactly the rows that reference the
//! moved row) and truncates the arena.
//!
//! ✍️ A field setter states a row only when the gesture changes that field, so a gesture that leaves a vertex or a halfedge where it
//! is states nothing about it.

use semio_framework_3d_mesh_document::Vec3;
use super::{same_face, same_halfedge, same_vertex, FaceId, HalfEdge, Links, MeshFace, MeshKernelError, MeshOverlay, MeshResult, MeshVertex, VertexId};
use std::collections::BTreeSet;

//#region 🔖️Views
/// 🗑️ What [`MeshOverlay::finish`] does with vertices that no face references any more.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Orphans {
    /// 📦️ Leave every vertex in place.
    Keep,
    /// 🆕️ Remove the vertices the gesture's removed or replaced faces left without a face.
    Newly,
    /// ✂️ Remove every vertex no face references.
    All,
}

/// 🧭️ The unit Newell normal of a polygon: double-precision sums relative to the first corner.
pub(super) struct Normal {
    origin: Vec3,
    sum: [f64; 3],
}

impl Normal {
    pub(super) fn starting_at(origin: Vec3) -> Self {
        Self { origin, sum: [0.0; 3] }
    }

    pub(super) fn add(&mut self, a: Vec3, b: Vec3) {
        let a = [0, 1, 2].map(|i| a.0[i] as f64 - self.origin.0[i] as f64);
        let b = [0, 1, 2].map(|i| b.0[i] as f64 - self.origin.0[i] as f64);
        self.sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        self.sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        self.sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    pub(super) fn value(&self) -> Vec3 {
        let length = self.sum[0].hypot(self.sum[1]).hypot(self.sum[2]);
        if length == 0.0 {
            Vec3::ZERO
        } else {
            Vec3(self.sum.map(|v| (v / length) as f32)).normalize()
        }
    }
}

/// 🧭️ The unit normal of the polygon `corners` over `positions`.
pub(super) fn loop_normal(positions: &[[f32; 3]], corners: &[u32]) -> Vec3 {
    let mut normal = Normal::starting_at(Vec3(positions[corners[0] as usize]));
    for index in 0..corners.len() {
        normal.add(Vec3(positions[corners[index] as usize]), Vec3(positions[corners[(index + 1) % corners.len()] as usize]));
    }
    normal.value()
}

/// 🥽️ The faces the overlay shows as polygons: positions, wound vertex loops (flipped faces already reversed), shading and
/// unit normals. A removed face keeps its index with an empty loop.
pub(super) struct Soup {
    pub positions: Vec<[f32; 3]>,
    pub faces: Vec<Vec<u32>>,
    pub smooth: Vec<bool>,
    pub normals: Vec<Vec3>,
}
//#endregion 🔖️Views

impl Links {
    fn insert(&mut self, origin: u32, destination: u32, halfedge: u32) {
        self.edges.insert((origin, destination), halfedge);
        let outgoing = self.out.entry(origin).or_default();
        if !outgoing.contains(&halfedge) {
            outgoing.push(halfedge);
        }
    }

    fn remove(&mut self, origin: u32, destination: u32, halfedge: u32) {
        if self.edges.get(&(origin, destination)) == Some(&halfedge) {
            self.edges.remove(&(origin, destination));
        }
        if let Some(outgoing) = self.out.get_mut(&origin) {
            outgoing.retain(|held| *held != halfedge);
        }
    }
}

impl<'a> MeshOverlay<'a> {
    //#region 🔖️Reads
    fn row(&self, halfedge: u32) -> MeshResult<&HalfEdge> {
        self.halfedge(halfedge).ok_or(MeshKernelError::InvalidHandle)
    }

    fn destination(&self, halfedge: u32) -> MeshResult<u32> {
        Ok(self.row(self.row(halfedge)?.next)?.vertex)
    }

    /// 🔷️ Whether `face` exists and was not removed by this gesture.
    pub(super) fn live_face(&self, face: u32) -> bool {
        (face as usize) < self.face_len() && !self.dead_faces.contains(&face)
    }

    /// 🔗️ The halfedges of `face` in `next` order from the face's first halfedge.
    pub(super) fn face_ring(&self, face: u32) -> MeshResult<Vec<u32>> {
        let start = self.face(FaceId(face)).ok_or(MeshKernelError::InvalidHandle)?.halfedge;
        let mut ring = Vec::new();
        let mut halfedge = start;
        loop {
            ring.push(halfedge);
            halfedge = self.row(halfedge)?.next;
            if halfedge == start {
                return Ok(ring);
            }
            if ring.len() > self.halfedge_len() {
                return Err(MeshKernelError::NonManifold);
            }
        }
    }

    /// 🔷️ The wound vertex loop of `face` (reversed for a flipped face).
    pub(super) fn face_loop(&self, face: u32) -> MeshResult<Vec<u32>> {
        let mut corners = self.face_ring(face)?.into_iter().map(|halfedge| self.row(halfedge).map(|row| row.vertex)).collect::<MeshResult<Vec<u32>>>()?;
        if self.face(FaceId(face)).is_some_and(|row| row.flipped) {
            corners.reverse();
        }
        Ok(corners)
    }

    /// 🥽️ Every vertex position and every live face as a wound loop with its normal and shading.
    pub(super) fn soup(&self) -> MeshResult<Soup> {
        let positions = (0..self.vertex_len() as u32).map(|vertex| self.vertex(VertexId(vertex)).map(|row| row.position).ok_or(MeshKernelError::InvalidHandle)).collect::<MeshResult<Vec<[f32; 3]>>>()?;
        let mut soup = Soup { positions, faces: Vec::new(), smooth: Vec::new(), normals: Vec::new() };
        for face in 0..self.face_len() as u32 {
            if !self.live_face(face) {
                soup.faces.push(Vec::new());
                soup.smooth.push(false);
                soup.normals.push(Vec3::ZERO);
                continue;
            }
            let corners = self.face_loop(face)?;
            if corners.iter().any(|vertex| *vertex as usize >= soup.positions.len()) {
                return Err(MeshKernelError::InvalidHandle);
            }
            soup.normals.push(loop_normal(&soup.positions, &corners));
            soup.smooth.push(self.face(FaceId(face)).is_some_and(|row| row.smooth));
            soup.faces.push(corners);
        }
        Ok(soup)
    }

    /// 🔗️ The directed-edge index of the live faces, built on first use.
    fn links_mut(&mut self) -> MeshResult<&mut Links> {
        if self.links.is_none() {
            let mut links = Links::default();
            for face in 0..self.face_len() as u32 {
                if !self.live_face(face) {
                    continue;
                }
                for halfedge in self.face_ring(face)? {
                    links.insert(self.row(halfedge)?.vertex, self.destination(halfedge)?, halfedge);
                }
            }
            self.links = Some(links);
        }
        self.links.as_mut().ok_or(MeshKernelError::InvalidHandle)
    }

    fn check_loop(&self, corners: &[u32]) -> MeshResult<()> {
        if corners.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        if corners.iter().any(|vertex| *vertex as usize >= self.vertex_len()) {
            return Err(MeshKernelError::InvalidHandle);
        }
        Ok(())
    }
    //#endregion 🔖️Reads

    //#region 🔖️Setters
    fn edit_vertex(&mut self, index: u32, edit: impl FnOnce(&mut MeshVertex)) {
        let Some(held) = self.vertex(VertexId(index)) else {
            return;
        };
        let mut row = held.clone();
        edit(&mut row);
        if !same_vertex(held, &row) {
            self.journal.vertices.put(index, row);
        }
    }

    fn edit_halfedge(&mut self, index: u32, edit: impl FnOnce(&mut HalfEdge)) {
        let Some(held) = self.halfedge(index) else {
            return;
        };
        let mut row = held.clone();
        edit(&mut row);
        if !same_halfedge(held, &row) {
            self.journal.halfedges.put(index, row);
        }
    }

    fn edit_face(&mut self, index: u32, edit: impl FnOnce(&mut MeshFace)) {
        let Some(held) = self.face(FaceId(index)) else {
            return;
        };
        let mut row = held.clone();
        edit(&mut row);
        if !same_face(held, &row) {
            self.journal.faces.put(index, row);
        }
    }

    /// 📍️ Appends a vertex row (no normal: the central applier derives it) and returns its index.
    pub(super) fn push_vertex(&mut self, position: [f32; 3]) -> u32 {
        let index = self.vertex_len() as u32;
        self.journal.vertices.put(index, MeshVertex { position, normal: None, halfedge: None });
        self.journal.vertices.resize(self.base.vertices.len(), index as usize + 1);
        index
    }

    /// 📍️ Moves a vertex; a vertex that is already there states no row.
    pub(super) fn move_vertex(&mut self, vertex: u32, position: [f32; 3]) -> MeshResult<()> {
        if vertex as usize >= self.vertex_len() {
            return Err(MeshKernelError::InvalidHandle);
        }
        self.edit_vertex(vertex, |row| row.position = position);
        Ok(())
    }

    /// 🔗️ Sets the uv of a halfedge; a corner that already holds it states no row.
    pub(super) fn set_corner_uv(&mut self, halfedge: u32, uv: [f32; 2]) {
        self.edit_halfedge(halfedge, |row| row.uv = uv);
    }

    fn append_halfedge(&mut self, row: HalfEdge) -> u32 {
        let index = self.halfedge_len() as u32;
        self.journal.halfedges.put(index, row);
        self.journal.halfedges.resize(self.base.halfedges.len(), index as usize + 1);
        index
    }
    //#endregion 🔖️Setters

    //#region 🔖️Faces
    fn release_halfedge(&mut self, halfedge: u32) -> MeshResult<()> {
        let origin = self.row(halfedge)?.vertex;
        let destination = self.destination(halfedge)?;
        self.links_mut()?.remove(origin, destination, halfedge);
        self.released.insert((origin, destination), halfedge);
        self.dead_halfedges.insert(halfedge);
        Ok(())
    }

    /// ♻️ Writes the rows of the loop `corners` as face `face`: `slots` holds the halfedges the caller keeps in place, every other
    /// corner takes back a released halfedge of the same directed edge or appends a new one; twins link to the opposite halfedges.
    fn place_loop(&mut self, face: u32, corners: &[u32], mut slots: Vec<Option<u32>>) -> MeshResult<Vec<u32>> {
        let count = corners.len();
        let kept: Vec<bool> = slots.iter().map(Option::is_some).collect();
        let mut taken_back = vec![false; count];
        for index in 0..count {
            if slots[index].is_none() {
                let key = (corners[index], corners[(index + 1) % count]);
                if let Some(halfedge) = self.released.remove(&key) {
                    if self.dead_halfedges.remove(&halfedge) {
                        slots[index] = Some(halfedge);
                        taken_back[index] = true;
                    }
                }
            }
        }
        for index in 0..count {
            if slots[index].is_none() {
                slots[index] = Some(self.append_halfedge(HalfEdge { vertex: corners[index], twin: None, next: 0, face: Some(face), uv: [0.0, 0.0] }));
            }
        }
        let slots: Vec<u32> = slots.into_iter().flatten().collect();
        for index in 0..count {
            let next = slots[(index + 1) % count];
            if kept[index] || taken_back[index] {
                let twin = self.row(slots[index])?.twin;
                let stale = twin.is_some_and(|twin| self.dead_halfedges.contains(&twin) || self.halfedge(twin).is_none_or(|other| other.twin != Some(slots[index])));
                self.edit_halfedge(slots[index], |row| {
                    row.next = next;
                    row.face = Some(face);
                    if stale {
                        row.twin = None;
                    }
                });
            } else {
                self.journal.halfedges.put(slots[index], HalfEdge { vertex: corners[index], twin: None, next, face: Some(face), uv: [0.0, 0.0] });
            }
        }
        for index in 0..count {
            let destination = corners[(index + 1) % count];
            self.links_mut()?.insert(corners[index], destination, slots[index]);
            if !kept[index] {
                self.placed.insert(slots[index]);
            }
        }
        for index in 0..count {
            if kept[index] || self.row(slots[index])?.twin.is_some() {
                continue;
            }
            let (origin, destination) = (corners[index], corners[(index + 1) % count]);
            let opposite = self.links_mut()?.edges.get(&(destination, origin)).copied();
            let Some(other) = opposite else {
                continue;
            };
            if other == slots[index] || self.dead_halfedges.contains(&other) {
                continue;
            }
            let other_twin = self.row(other)?.twin;
            if other_twin.is_none_or(|held| self.dead_halfedges.contains(&held)) {
                self.edit_halfedge(slots[index], |row| row.twin = Some(other));
                self.edit_halfedge(other, |row| row.twin = Some(slots[index]));
            }
        }
        Ok(slots)
    }

    /// ➕️ Appends the face with the wound vertex loop `corners` and returns its index.
    pub(super) fn add_face(&mut self, corners: &[u32], smooth: bool) -> MeshResult<u32> {
        self.check_loop(corners)?;
        let face = self.face_len() as u32;
        let slots = self.place_loop(face, corners, vec![None; corners.len()])?;
        self.journal.faces.put(face, MeshFace { halfedge: slots[0], smooth, flipped: false });
        self.journal.faces.resize(self.base.faces.len(), face as usize + 1);
        Ok(face)
    }

    /// 🔁️ Gives `face` the wound vertex loop `corners`: halfedges whose directed edge survives stay as they are, the others are
    /// released for the faces that need them. The face is stored unflipped.
    pub(super) fn replace_face(&mut self, face: u32, corners: &[u32]) -> MeshResult<()> {
        self.check_loop(corners)?;
        if !self.live_face(face) {
            return Err(MeshKernelError::InvalidHandle);
        }
        let mut old: Vec<(u32, u32, u32)> = Vec::new();
        for halfedge in self.face_ring(face)? {
            let origin = self.row(halfedge)?.vertex;
            old.push((origin, self.destination(halfedge)?, halfedge));
            self.orphans.insert(origin);
        }
        let count = corners.len();
        let mut claimed = vec![false; old.len()];
        let mut slots: Vec<Option<u32>> = vec![None; count];
        for index in 0..count {
            let key = (corners[index], corners[(index + 1) % count]);
            if let Some(at) = (0..old.len()).find(|at| !claimed[*at] && (old[*at].0, old[*at].1) == key) {
                claimed[at] = true;
                slots[index] = Some(old[at].2);
            }
        }
        for (at, entry) in old.iter().enumerate() {
            if !claimed[at] {
                self.release_halfedge(entry.2)?;
            }
        }
        let slots = self.place_loop(face, corners, slots)?;
        let first = self.face(FaceId(face)).ok_or(MeshKernelError::InvalidHandle)?.halfedge;
        let keeps_first = slots.contains(&first);
        self.edit_face(face, |row| {
            row.flipped = false;
            if !keeps_first {
                row.halfedge = slots[0];
            }
        });
        Ok(())
    }

    /// ➖️ Removes `face`: its halfedges are released and the face is compacted away by [`Self::finish`].
    pub(super) fn kill_face(&mut self, face: u32) -> MeshResult<()> {
        if !self.live_face(face) {
            return Err(MeshKernelError::InvalidHandle);
        }
        let ring = self.face_ring(face)?;
        for halfedge in &ring {
            let origin = self.row(*halfedge)?.vertex;
            self.orphans.insert(origin);
        }
        for halfedge in ring {
            self.release_halfedge(halfedge)?;
        }
        self.dead_faces.insert(face);
        Ok(())
    }
    //#endregion 🔖️Faces

    //#region 🔖️Compaction
    fn move_halfedge(&mut self, last: u32, hole: u32) -> MeshResult<()> {
        let row = self.row(last)?.clone();
        let destination = self.destination(last)?;
        self.journal.halfedges.put(hole, row.clone());
        if let Some(face) = row.face {
            let start = self.face(FaceId(face)).ok_or(MeshKernelError::InvalidHandle)?.halfedge;
            let mut previous = start;
            let mut guard = 0;
            while self.row(previous)?.next != last {
                previous = self.row(previous)?.next;
                guard += 1;
                if guard > self.halfedge_len() {
                    return Err(MeshKernelError::NonManifold);
                }
            }
            self.edit_halfedge(previous, |held| held.next = hole);
            if start == last {
                self.edit_face(face, |held| held.halfedge = hole);
            }
        }
        if let Some(twin) = row.twin {
            self.edit_halfedge(twin, |held| held.twin = Some(hole));
        }
        if self.vertex(VertexId(row.vertex)).is_some_and(|held| held.halfedge == Some(last)) {
            self.edit_vertex(row.vertex, |held| held.halfedge = Some(hole));
        }
        let links = self.links_mut()?;
        links.remove(row.vertex, destination, last);
        links.insert(row.vertex, destination, hole);
        Ok(())
    }

    fn move_face(&mut self, last: u32, hole: u32) -> MeshResult<()> {
        let row = self.face(FaceId(last)).ok_or(MeshKernelError::InvalidHandle)?.clone();
        let ring = self.face_ring(last)?;
        self.journal.faces.put(hole, row);
        for halfedge in ring {
            self.edit_halfedge(halfedge, |held| held.face = Some(hole));
        }
        Ok(())
    }

    /// 🧹️ Ends a gesture's edit: the neighbours of halfedges nobody took back lose their twin, the tail rows move into the holes the
    /// removed halfedges and faces left and the arenas shrink, vertex links the edit invalidated are repointed and the vertices
    /// `orphans` names are compacted away the same way.
    pub(super) fn finish(&mut self, orphans: Orphans) -> MeshResult<()> {
        let dead: Vec<u32> = self.dead_halfedges.iter().copied().collect();
        let mut invalidated: BTreeSet<u32> = BTreeSet::new();
        let mut unpaired: BTreeSet<u32> = std::mem::take(&mut self.placed);
        for halfedge in &dead {
            let row = self.row(*halfedge)?.clone();
            if let Some(twin) = row.twin {
                if (twin as usize) < self.halfedge_len() && !self.dead_halfedges.contains(&twin) && self.row(twin)?.twin == Some(*halfedge) {
                    self.edit_halfedge(twin, |held| held.twin = None);
                    unpaired.insert(twin);
                }
            }
            if self.vertex(VertexId(row.vertex)).is_some_and(|held| held.halfedge == Some(*halfedge)) {
                self.edit_vertex(row.vertex, |held| held.halfedge = None);
                invalidated.insert(row.vertex);
            }
        }
        for halfedge in unpaired.iter().copied().collect::<Vec<u32>>() {
            if self.dead_halfedges.contains(&halfedge) || halfedge as usize >= self.halfedge_len() || self.row(halfedge)?.twin.is_some() {
                continue;
            }
            let (origin, destination) = (self.row(halfedge)?.vertex, self.destination(halfedge)?);
            let opposite = self.links_mut()?.edges.get(&(destination, origin)).copied();
            let Some(other) = opposite else {
                continue;
            };
            if other != halfedge && unpaired.contains(&other) && !self.dead_halfedges.contains(&other) && self.row(other)?.twin.is_none() {
                self.edit_halfedge(halfedge, |held| held.twin = Some(other));
                self.edit_halfedge(other, |held| held.twin = Some(halfedge));
            }
        }
        if !dead.is_empty() {
            let length = self.halfedge_len();
            let keep = length - dead.len();
            let holes: Vec<u32> = dead.iter().copied().filter(|hole| (*hole as usize) < keep).collect();
            let movers: Vec<u32> = (keep as u32..length as u32).rev().filter(|mover| !self.dead_halfedges.contains(mover)).collect();
            for (hole, last) in holes.iter().zip(&movers) {
                self.move_halfedge(*last, *hole)?;
            }
            self.dead_halfedges.clear();
            self.released.clear();
            self.resize_halfedges(keep);
        }
        let dead_faces: Vec<u32> = self.dead_faces.iter().copied().collect();
        if !dead_faces.is_empty() {
            let length = self.face_len();
            let keep = length - dead_faces.len();
            let holes: Vec<u32> = dead_faces.iter().copied().filter(|hole| (*hole as usize) < keep).collect();
            let movers: Vec<u32> = (keep as u32..length as u32).rev().filter(|mover| !self.dead_faces.contains(mover)).collect();
            for (hole, last) in holes.iter().zip(&movers) {
                self.move_face(*last, *hole)?;
            }
            self.dead_faces.clear();
            self.resize_faces(keep);
        }
        let mut outgoing: Vec<Vec<u32>> = vec![Vec::new(); self.vertex_len()];
        for halfedge in 0..self.halfedge_len() as u32 {
            let origin = self.row(halfedge)?.vertex as usize;
            outgoing.get_mut(origin).ok_or(MeshKernelError::InvalidHandle)?.push(halfedge);
        }
        let appended = self.base.vertices.len() as u32;
        let repair: Vec<u32> = invalidated.into_iter().chain(appended..self.vertex_len() as u32).collect();
        for vertex in repair {
            if (vertex as usize) < outgoing.len() && self.vertex(VertexId(vertex)).is_some_and(|held| held.halfedge.is_none()) {
                if let Some(first) = outgoing[vertex as usize].first().copied() {
                    self.edit_vertex(vertex, |held| held.halfedge = Some(first));
                }
            }
        }
        let doomed: Vec<u32> = match orphans {
            Orphans::Keep => Vec::new(),
            Orphans::Newly => self.orphans.iter().copied().filter(|vertex| (*vertex as usize) < outgoing.len() && outgoing[*vertex as usize].is_empty()).collect(),
            Orphans::All => (0..outgoing.len() as u32).filter(|vertex| outgoing[*vertex as usize].is_empty()).collect(),
        };
        if !doomed.is_empty() {
            let length = self.vertex_len();
            let keep = length - doomed.len();
            let gone: BTreeSet<u32> = doomed.iter().copied().collect();
            let holes: Vec<u32> = doomed.iter().copied().filter(|hole| (*hole as usize) < keep).collect();
            let movers: Vec<u32> = (keep as u32..length as u32).rev().filter(|mover| !gone.contains(mover)).collect();
            for (hole, last) in holes.iter().zip(&movers) {
                let row = self.vertex(VertexId(*last)).ok_or(MeshKernelError::InvalidHandle)?.clone();
                self.journal.vertices.put(*hole, row);
                for halfedge in outgoing[*last as usize].clone() {
                    self.edit_halfedge(halfedge, |held| held.vertex = *hole);
                }
                self.vertex_moves.push((*hole, *last));
            }
            self.resize_vertices(keep);
        }
        self.links = None;
        self.orphans.clear();
        Ok(())
    }
    //#endregion 🔖️Compaction
}
