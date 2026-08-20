//! 🔷️ Half-edge mesh kernel for low-poly editing. **Host authority:** `HalfedgeMesh` is a value
//! document/engine payload — not a process-global mesh store.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//#region Types

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub const ZERO: Self = Self([0.0, 0.0, 0.0]);

    pub async fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub async fn x(self) -> f32 {
        self.0[0]
    }
    pub async fn y(self) -> f32 {
        self.0[1]
    }
    pub async fn z(self) -> f32 {
        self.0[2]
    }

    #[allow(clippy::should_implement_trait, reason = "renaming ripples through lowpoly/core, lowpoly/plugin, remodel/plugin (outside this crate); add/sub read better than +/- across this file's dense vector algebra")]
    pub async fn add(self, o: Self) -> Self {
        Self([self.x().await + o.x().await, self.y().await + o.y().await, self.z().await + o.z().await])
    }

    #[allow(clippy::should_implement_trait, reason = "renaming ripples through lowpoly/core, lowpoly/plugin, remodel/plugin (outside this crate); add/sub read better than +/- across this file's dense vector algebra")]
    pub async fn sub(self, o: Self) -> Self {
        Self([self.x().await - o.x().await, self.y().await - o.y().await, self.z().await - o.z().await])
    }

    pub async fn scale(self, s: f32) -> Self {
        Self([self.x().await * s, self.y().await * s, self.z().await * s])
    }

    pub async fn dot(self, o: Self) -> f32 {
        self.x().await * o.x().await + self.y().await * o.y().await + self.z().await * o.z().await
    }

    pub async fn cross(self, o: Self) -> Self {
        Self([
            self.y().await * o.z().await - self.z().await * o.y().await,
            self.z().await * o.x().await - self.x().await * o.z().await,
            self.x().await * o.y().await - self.y().await * o.x().await,
        ])
    }

    pub async fn length(self) -> f32 {
        self.dot(self).await.sqrt()
    }

    pub async fn normalize(self) -> Self {
        let l = self.length().await;
        if l < 1e-8 {
            return Self::ZERO;
        }
        self.scale(1.0 / l).await
    }

    pub async fn lerp(self, o: Self, t: f32) -> Self {
        self.add(o.sub(self).await.scale(t).await).await
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VertexId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HalfEdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FaceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeldMode {
    Center,
    First,
    ByDistance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorAxis {
    X,
    Y,
    Z,
}

//#region ⚠️ Errors
/// ⚠️ Half-edge mesh kernel operation failure.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum MeshKernelError {
    #[error("invalid handle")]
    InvalidHandle,
    #[error("mesh is non-manifold")]
    NonManifold,
    #[error("degenerate operation")]
    DegenerateOperation,
    #[error("empty selection")]
    EmptySelection,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
//#endregion ⚠️ Errors

pub type MeshResult<T> = Result<T, MeshKernelError>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: Option<[f32; 3]>,
    halfedge: Option<u32>,
}

// 🚫️async: E4 fn-pointer slot — serde's `#[serde(default = "...")]` calls this by path as a plain
// `fn() -> T`; an `async fn` item's pointer type is unnameable there.
fn default_uv() -> [f32; 2] {
    [0.0, 0.0]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HalfEdge {
    vertex: u32,
    twin: Option<u32>,
    next: u32,
    face: Option<u32>,
    #[serde(default = "default_uv")]
    uv: [f32; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MeshFace {
    halfedge: u32,
    smooth: bool,
    #[serde(default)]
    flipped: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshTransfer {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub edge_positions: Vec<f32>,
    #[serde(default)]
    pub face_ids: Vec<u32>,
    #[serde(default)]
    pub vertex_ids: Vec<u32>,
    #[serde(default)]
    pub edge_ids: Vec<u32>,
    #[serde(default)]
    pub uvs: Vec<f32>,
    #[serde(default)]
    pub edge_uvs: Vec<f32>,
    #[serde(default)]
    pub edge_is_seam: Vec<u8>,
}

//#endregion Types

//#region HalfedgeMesh

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HalfedgeMesh {
    vertices: Vec<MeshVertex>,
    halfedges: Vec<HalfEdge>,
    faces: Vec<MeshFace>,
    #[serde(default)]
    uv_seams: HashSet<u32>,
}

impl HalfedgeMesh {
    pub async fn empty() -> Self {
        Self { vertices: Vec::new(), halfedges: Vec::new(), faces: Vec::new(), uv_seams: HashSet::new() }
    }

    pub async fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub async fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub async fn edge_count(&self) -> usize {
        self.halfedges.len() / 2
    }

    pub async fn vertex_position(&self, id: VertexId) -> MeshResult<Vec3> {
        self.vertices.get(id.0 as usize).map(|v| Vec3(v.position)).ok_or(MeshKernelError::InvalidHandle)
    }

    pub async fn set_vertex_position(&mut self, id: VertexId, pos: Vec3) -> MeshResult<()> {
        let v = self.vertices.get_mut(id.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        v.position = pos.0;
        Ok(())
    }

    pub async fn face_vertex_ids(&self, face: FaceId) -> MeshResult<Vec<VertexId>> {
        let f = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let mut out = Vec::new();
        let start = f.halfedge;
        let mut he = start;
        loop {
            let e = &self.halfedges[he as usize];
            out.push(VertexId(e.vertex));
            he = e.next;
            if he == start {
                break;
            }
        }
        if f.flipped {
            out.reverse();
        }
        Ok(out)
    }

    pub async fn face_normal(&self, face: FaceId) -> MeshResult<Vec3> {
        let verts = self.face_vertex_ids(face).await?;
        if verts.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let mut positions: Vec<Vec3> = Vec::with_capacity(verts.len());
        for v in &verts {
            positions.push(self.vertex_position(*v).await?);
        }
        Ok(newell_normal(&positions).await.normalize().await)
    }

    pub async fn edge_endpoints(&self, edge: EdgeId) -> MeshResult<(VertexId, VertexId)> {
        let he = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let v0 = VertexId(he.vertex);
        let next = &self.halfedges[he.next as usize];
        let v1 = VertexId(next.vertex);
        Ok((v0, v1))
    }

    pub async fn face_halfedge_ids(&self, face: FaceId) -> MeshResult<Vec<u32>> {
        let f = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let mut out = Vec::new();
        let start = f.halfedge;
        let mut he = start;
        loop {
            out.push(he);
            he = self.halfedges[he as usize].next;
            if he == start {
                break;
            }
        }
        Ok(out)
    }

    pub async fn flip_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        for face in faces {
            let entry = self.faces.get_mut(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            entry.flipped = !entry.flipped;
        }
        self.recompute_normals().await
    }

    pub async fn from_indexed_triangles(positions: &[f32], indices: &[u32]) -> MeshResult<Self> {
        if !positions.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("positions length must be a multiple of 3".into()));
        }
        if !indices.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("indices length must be a multiple of 3".into()));
        }
        let verts: Vec<[f32; 3]> = positions.as_chunks::<3>().0.to_vec();
        let faces: Vec<Vec<u32>> = indices.as_chunks::<3>().0.iter().map(|tri| tri.to_vec()).collect();
        Self::from_faces(&verts, &faces).await
    }

    /// Builds a halfedge mesh from a CAD solid tessellation that carries one B-Rep face id per triangle.
    ///
    /// Each B-Rep face is reconstructed independently: coplanar triangles of a simply-connected face merge
    /// into one n-gon; faces with holes keep their triangulation so openings are not filled. Call
    /// [`Self::weld_coincident_vertices`] afterwards so independently-tessellated seam vertices become shared.
    pub async fn from_indexed_triangles_by_face_id(positions: &[f32], indices: &[u32], face_ids: &[u32]) -> MeshResult<Self> {
        if face_ids.is_empty() {
            return Self::from_indexed_triangles(positions, indices).await;
        }
        if !positions.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("positions length must be a multiple of 3".into()));
        }
        if !indices.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("indices length must be a multiple of 3".into()));
        }
        let tri_count = indices.len() / 3;
        if face_ids.len() != tri_count {
            return Err(MeshKernelError::InvalidInput(format!("face_ids length {} must equal triangle count {}", face_ids.len(), tri_count)));
        }
        let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
        for (triangle_index, &face_id) in face_ids.iter().enumerate() {
            let base = triangle_index * 3;
            let group = groups.entry(face_id).or_default();
            group.extend_from_slice(&indices[base..base + 3]);
        }
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for group_indices in groups.values() {
            let mut face_mesh = Self::from_indexed_triangles(positions, group_indices).await?;
            let _ = face_mesh.merge_coplanar_faces().await?;
            let (_, group_faces) = face_mesh.polygon_soup().await;
            faces.extend(group_faces);
        }
        let verts: Vec<[f32; 3]> = positions.as_chunks::<3>().0.to_vec();
        Self::from_faces(&verts, &faces).await
    }

    /// Builds a halfedge mesh from CAD face wire loops that share a global vertex buffer.
    ///
    /// Each entry is `(outer, holes)`. Faces without holes become one n-gon; faces with holes are
    /// triangulated via keyhole bridging so openings stay empty (never filled by a single outer n-gon).
    pub async fn from_face_loops(positions: &[[f32; 3]], face_loops: &[(Vec<u32>, Vec<Vec<u32>>)]) -> MeshResult<Self> {
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for (outer, holes) in face_loops {
            if outer.len() < 3 {
                continue;
            }
            if holes.is_empty() {
                faces.push(outer.clone());
                continue;
            }
            for tri in triangulate_indexed_polygon_with_holes(positions, outer, holes).await {
                faces.push(tri.to_vec());
            }
        }
        Self::from_faces(positions, &faces).await
    }

    pub async fn from_faces(positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<Self> {
        let mut mesh = Self::empty().await;
        for p in positions {
            mesh.vertices.push(MeshVertex { position: *p, normal: None, halfedge: None });
        }
        let mut edge_map: HashMap<(u32, u32), u32> = HashMap::new();
        for face_verts in faces {
            if face_verts.len() < 3 {
                return Err(MeshKernelError::DegenerateOperation);
            }
            let face_id = mesh.faces.len() as u32;
            let mut face_hes = Vec::new();
            for i in 0..face_verts.len() {
                let v0 = face_verts[i];
                let v1 = face_verts[(i + 1) % face_verts.len()];
                if v0 as usize >= mesh.vertices.len() || v1 as usize >= mesh.vertices.len() {
                    return Err(MeshKernelError::InvalidInput("face index out of range".into()));
                }
                let he_id = mesh.halfedges.len() as u32;
                mesh.halfedges.push(HalfEdge { vertex: v0, twin: None, next: 0, face: Some(face_id), uv: [0.0, 0.0] });
                face_hes.push(he_id);
                if let Some(&twin_id) = edge_map.get(&(v1, v0)) {
                    mesh.halfedges[he_id as usize].twin = Some(twin_id);
                    mesh.halfedges[twin_id as usize].twin = Some(he_id);
                }
                edge_map.insert((v0, v1), he_id);
            }
            for i in 0..face_hes.len() {
                let next = face_hes[(i + 1) % face_hes.len()];
                mesh.halfedges[face_hes[i] as usize].next = next;
            }
            let start_he = face_hes[0];
            mesh.faces.push(MeshFace { halfedge: start_he, smooth: false, flipped: false });
            let v0 = mesh.halfedges[start_he as usize].vertex;
            mesh.vertices[v0 as usize].halfedge = Some(start_he);
        }
        mesh.recompute_normals().await?;
        Ok(mesh)
    }

    async fn add_vertex(&mut self, pos: [f32; 3]) -> VertexId {
        let id = self.vertices.len() as u32;
        self.vertices.push(MeshVertex { position: pos, normal: None, halfedge: None });
        VertexId(id)
    }

    async fn rebuild_from_polygon_soup(&mut self, positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<()> {
        *self = Self::from_faces(positions, faces).await?;
        Ok(())
    }

    async fn polygon_soup(&self) -> (Vec<[f32; 3]>, Vec<Vec<u32>>) {
        let positions: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.position).collect();
        let mut faces = Vec::new();
        for fi in 0..self.faces.len() {
            if let Ok(verts) = self.face_vertex_ids(FaceId(fi as u32)).await {
                faces.push(verts.into_iter().map(|v| v.0).collect());
            }
        }
        (positions, faces)
    }
}

//#endregion HalfedgeMesh

//#region Primitives

impl HalfedgeMesh {
    pub async fn box_prim(width: f32, height: f32, depth: f32) -> MeshResult<Self> {
        let hw = width * 0.5;
        let hh = height * 0.5;
        let hd = depth * 0.5;
        let positions = [[-hw, -hh, -hd], [hw, -hh, -hd], [hw, hh, -hd], [-hw, hh, -hd], [-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]];
        let faces = vec![vec![0, 1, 2, 3], vec![4, 7, 6, 5], vec![0, 4, 5, 1], vec![1, 5, 6, 2], vec![2, 6, 7, 3], vec![3, 7, 4, 0]];
        Self::from_faces(&positions, &faces).await
    }

    pub async fn plane_prim(width: f32, depth: f32) -> MeshResult<Self> {
        let hw = width * 0.5;
        let hd = depth * 0.5;
        Self::from_faces(&[[-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]], &[vec![0, 1, 2, 3]]).await
    }

    pub async fn cylinder_prim(radius: f32, height: f32, segments: u32) -> MeshResult<Self> {
        let segs = segments.max(3);
        let mut positions = Vec::new();
        let hh = height * 0.5;
        for i in 0..segs {
            let a = (i as f32 / segs as f32) * std::f32::consts::TAU;
            positions.push([radius * a.cos(), -hh, radius * a.sin()]);
        }
        for i in 0..segs {
            let a = (i as f32 / segs as f32) * std::f32::consts::TAU;
            positions.push([radius * a.cos(), hh, radius * a.sin()]);
        }
        positions.push([0.0, -hh, 0.0]);
        positions.push([0.0, hh, 0.0]);
        let bottom = segs;
        let top = segs + 1;
        let mut faces = Vec::new();
        for i in 0..segs {
            let i0 = i;
            let i1 = (i + 1) % segs;
            let b0 = i0;
            let b1 = i1;
            let t0 = segs + i0;
            let t1 = segs + i1;
            faces.push(vec![b0, b1, t1, t0]);
            faces.push(vec![bottom, b1, b0]);
            faces.push(vec![top, t0, t1]);
        }
        Self::from_faces(&positions, &faces).await
    }

    pub async fn cone_prim(radius: f32, height: f32, segments: u32) -> MeshResult<Self> {
        let segs = segments.max(3);
        let mut positions = Vec::new();
        let hh = height * 0.5;
        for i in 0..segs {
            let a = (i as f32 / segs as f32) * std::f32::consts::TAU;
            positions.push([radius * a.cos(), -hh, radius * a.sin()]);
        }
        positions.push([0.0, hh, 0.0]);
        positions.push([0.0, -hh, 0.0]);
        let apex = segs;
        let base = segs + 1;
        let mut faces = Vec::new();
        for i in 0..segs {
            let i0 = i;
            let i1 = (i + 1) % segs;
            faces.push(vec![i0, i1, apex]);
            faces.push(vec![base, i1, i0]);
        }
        Self::from_faces(&positions, &faces).await
    }

    pub async fn ico_sphere_prim(radius: f32, subdivisions: u32) -> MeshResult<Self> {
        let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
        let mut positions = vec![[-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0]];
        for p in &mut positions {
            let v = Vec3(*p).normalize().await.scale(radius);
            *p = v.await.0;
        }
        let mut faces = vec![
            vec![0, 11, 5],
            vec![0, 5, 1],
            vec![0, 1, 7],
            vec![0, 7, 10],
            vec![0, 10, 11],
            vec![1, 5, 9],
            vec![5, 11, 4],
            vec![11, 10, 2],
            vec![10, 7, 6],
            vec![7, 1, 8],
            vec![3, 9, 4],
            vec![3, 4, 2],
            vec![3, 2, 6],
            vec![3, 6, 8],
            vec![3, 8, 9],
            vec![4, 9, 5],
            vec![2, 4, 11],
            vec![6, 2, 10],
            vec![8, 6, 7],
            vec![9, 8, 1],
        ];
        let subs = subdivisions.min(3);
        for _ in 0..subs {
            let mut new_faces = Vec::new();
            let mut midpoint_cache: HashMap<(u32, u32), u32> = HashMap::new();
            for face in &faces {
                let mut mids = Vec::new();
                for i in 0..face.len() {
                    let a = face[i];
                    let b = face[(i + 1) % face.len()];
                    let key = if a < b { (a, b) } else { (b, a) };
                    let mid = if let Some(&existing) = midpoint_cache.get(&key) {
                        existing
                    } else {
                        let pa = Vec3(positions[a as usize]);
                        let pb = Vec3(positions[b as usize]);
                        let m = pa.lerp(pb, 0.5).await.normalize().await.scale(radius).await;
                        let id = positions.len() as u32;
                        positions.push(m.0);
                        midpoint_cache.insert(key, id);
                        id
                    };
                    mids.push(mid);
                }
                for i in 0..face.len() {
                    let v = face[i];
                    let m0 = mids[i];
                    let m1 = mids[(i + face.len() - 1) % face.len()];
                    new_faces.push(vec![v, m0, m1]);
                }
            }
            faces = new_faces;
        }
        Self::from_faces(&positions, &faces).await
    }
}

//#endregion Primitives

//#region Transform

impl HalfedgeMesh {
    pub async fn translate(&mut self, delta: Vec3) -> MeshResult<()> {
        for v in &mut self.vertices {
            v.position = Vec3(v.position).add(delta).await.0;
        }
        Ok(())
    }

    pub async fn rotate(&mut self, axis: Vec3, angle_rad: f32) -> MeshResult<()> {
        let ax = axis.normalize().await;
        let (x, y, z) = (ax.x().await, ax.y().await, ax.z().await);
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        let t = 1.0 - c;
        for v in &mut self.vertices {
            let p = Vec3(v.position);
            let (px, py, pz) = (p.x().await, p.y().await, p.z().await);
            let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
            let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
            let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
            v.position = [rx, ry, rz];
        }
        self.recompute_normals().await
    }

    pub async fn scale(&mut self, factor: Vec3) -> MeshResult<()> {
        let (fx, fy, fz) = (factor.x().await, factor.y().await, factor.z().await);
        for v in &mut self.vertices {
            v.position = [v.position[0] * fx, v.position[1] * fy, v.position[2] * fz];
        }
        self.recompute_normals().await
    }

    pub async fn move_vertices(&mut self, verts: &[VertexId], delta: Vec3) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            v.position = Vec3(v.position).add(delta).await.0;
        }
        self.recompute_normals().await
    }

    pub async fn rotate_vertices(&mut self, verts: &[VertexId], axis: Vec3, angle_rad: f32, pivot: Vec3) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let ax = axis.normalize().await;
        let (x, y, z) = (ax.x().await, ax.y().await, ax.z().await);
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        let t = 1.0 - c;
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            let p = Vec3(v.position).sub(pivot).await;
            let (px, py, pz) = (p.x().await, p.y().await, p.z().await);
            let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
            let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
            let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
            v.position = pivot.add(Vec3::new(rx, ry, rz).await).await.0;
        }
        self.recompute_normals().await
    }

    pub async fn scale_vertices(&mut self, verts: &[VertexId], factor: Vec3, pivot: Vec3) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (fx, fy, fz) = (factor.x().await, factor.y().await, factor.z().await);
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            let p = Vec3(v.position).sub(pivot).await;
            let (px, py, pz) = (p.x().await, p.y().await, p.z().await);
            v.position = pivot.add(Vec3::new(px * fx, py * fy, pz * fz).await).await.0;
        }
        self.recompute_normals().await
    }

    pub async fn move_vertices_proportional(&mut self, verts: &[VertexId], delta: Vec3, pivot: Vec3, radius: f32) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let r = radius.max(1e-6);
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            let pos = Vec3(v.position);
            let dist = pos.sub(pivot).await.length().await;
            let falloff = (1.0 - (dist / r).min(1.0)).max(0.0);
            v.position = pos.add(delta.scale(falloff).await).await.0;
        }
        self.recompute_normals().await
    }

    pub async fn snap_vertices_to_grid(&mut self, verts: &[VertexId], grid: f32) -> MeshResult<()> {
        if grid <= 0.0 {
            return Err(MeshKernelError::InvalidInput("grid must be positive".into()));
        }
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            v.position = [(v.position[0] / grid).round() * grid, (v.position[1] / grid).round() * grid, (v.position[2] / grid).round() * grid];
        }
        Ok(())
    }
}

//#endregion Transform

//#region Edit

impl HalfedgeMesh {
    pub async fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let mut new_faces = Vec::new();
        for &fid in faces {
            let normal = self.face_normal(fid).await?;
            let verts = self.face_vertex_ids(fid).await?;
            let mut new_verts = Vec::new();
            for v in &verts {
                let pos = self.vertex_position(*v).await?;
                let nv = self.add_vertex(pos.add(normal.scale(distance).await).await.0);
                new_verts.push(nv.await.0);
            }
            let old_ids: Vec<u32> = verts.iter().map(|v| v.0).collect();
            new_faces.push(old_ids.clone());
            new_faces.push(new_verts.clone());
            let n = old_ids.len();
            for i in 0..n {
                let o0 = old_ids[i];
                let o1 = old_ids[(i + 1) % n];
                let n0 = new_verts[i];
                let n1 = new_verts[(i + 1) % n];
                new_faces.push(vec![o0, o1, n1, n0]);
            }
        }
        let (positions, mut face_list) = self.polygon_soup().await;
        for f in new_faces {
            face_list.push(f);
        }
        self.rebuild_from_polygon_soup(&positions, &face_list).await?;
        Ok(())
    }

    pub async fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (mut positions, mut face_list) = self.polygon_soup().await;
        for &fid in faces {
            let verts = self.face_vertex_ids(fid).await?;
            let mut centroid = Vec3::ZERO;
            for v in &verts {
                centroid = centroid.add(self.vertex_position(*v).await?).await;
            }
            centroid = centroid.scale(1.0 / verts.len() as f32).await;
            let normal = self.face_normal(fid).await?;
            let indices: Vec<usize> = verts.iter().map(|v| v.0 as usize).collect();
            for &vi in &indices {
                let pos = Vec3(positions[vi]);
                let to_center = centroid.sub(pos).await;
                let to_center_dot_n = to_center.dot(normal).await;
                let inset_dir = to_center.sub(normal.scale(to_center_dot_n).await).await.normalize().await;
                let new_pos = pos.add(inset_dir.scale(amount).await).await;
                positions[vi] = new_pos.0;
            }
            if amount.abs() > 1e-6 {
                let mut inner = Vec::new();
                for v in &verts {
                    let pos = Vec3(positions[v.0 as usize]);
                    let to_center = centroid.sub(pos).await;
                    let to_center_dot_n = to_center.dot(normal).await;
                    let inset_dir = to_center.sub(normal.scale(to_center_dot_n).await).await.normalize().await;
                    let inner_pos = pos.add(inset_dir.scale(amount * 0.5).await).await;
                    let id = positions.len() as u32;
                    positions.push(inner_pos.0);
                    inner.push(id);
                }
                face_list.push(inner);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list).await
    }

    pub async fn bevel_edges(&mut self, edges: &[EdgeId], amount: f32, _segments: u32) -> MeshResult<()> {
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (mut positions, face_list) = self.polygon_soup().await;
        for &eid in edges {
            let (v0, v1) = self.edge_endpoints(eid).await?;
            let p0 = self.vertex_position(v0).await?;
            let p1 = self.vertex_position(v1).await?;
            let dir = p1.sub(p0).await.normalize().await;
            let mid = p0.lerp(p1, 0.5).await;
            let offset = dir.cross(Vec3::new(0.0, 1.0, 0.0).await).await.normalize().await.scale(amount).await;
            let nv0 = positions.len() as u32;
            positions.push(mid.add(offset).await.0);
            let nv1 = positions.len() as u32;
            positions.push(mid.sub(offset).await.0);
            let _ = (nv0, nv1, v0, v1);
        }
        self.rebuild_from_polygon_soup(&positions, &face_list).await
    }

    pub async fn loop_cut(&mut self, _edges: &[EdgeId], cuts: u32) -> MeshResult<()> {
        if cuts == 0 {
            return Err(MeshKernelError::InvalidInput("cuts must be > 0".into()));
        }
        let (mut positions, mut face_list) = self.polygon_soup().await;
        let face_count = face_list.len();
        for fi in 0..face_count {
            let face = face_list[fi].clone();
            if face.len() < 3 {
                continue;
            }
            let mut sub_faces = Vec::new();
            for step in 1..=cuts {
                let t = step as f32 / (cuts + 1) as f32;
                let mut ring = Vec::new();
                for i in 0..face.len() {
                    let a = Vec3(positions[face[i] as usize]);
                    let b = Vec3(positions[face[(i + 1) % face.len()] as usize]);
                    let p = a.lerp(b, t);
                    let id = positions.len() as u32;
                    positions.push(p.await.0);
                    ring.push(id);
                }
                sub_faces.push(ring);
            }
            for ring in sub_faces {
                face_list.push(ring);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list).await
    }

    pub async fn knife_cut(&mut self, face: FaceId, cut_a: Vec3, cut_b: Vec3) -> MeshResult<()> {
        let verts = self.face_vertex_ids(face).await?;
        if verts.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let (mut positions, mut face_list) = self.polygon_soup().await;
        let cut_dir = cut_b.sub(cut_a).await.normalize().await;
        let cut_plane_normal = cut_dir.cross(Vec3::new(0.0, 1.0, 0.0).await).await.normalize().await;
        let mut hits: Vec<(usize, f32, [f32; 3])> = Vec::new();
        for i in 0..verts.len() {
            let v0 = self.vertex_position(verts[i]).await?;
            let v1 = self.vertex_position(verts[(i + 1) % verts.len()]).await?;
            if let Some((t, p)) = segment_plane_intersect(v0, v1, cut_a, cut_plane_normal).await {
                if t > 0.0 && t < 1.0 {
                    hits.push((i, t, p.0));
                }
            }
        }
        if hits.len() >= 2 {
            let id0 = positions.len() as u32;
            positions.push(hits[0].2);
            let id1 = positions.len() as u32;
            positions.push(hits[1].2);
            let face_idx = face.0 as usize;
            if face_idx < face_list.len() {
                let f0 = face_list[face_idx][0];
                let f1 = face_list[face_idx][1];
                face_list.push(vec![f0, id0, id1]);
                face_list.push(vec![id0, f1, id1]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list).await
    }

    pub async fn merge_vertices(&mut self, verts: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()> {
        if verts.len() < 2 {
            return Err(MeshKernelError::EmptySelection);
        }
        let target_pos = match mode {
            WeldMode::First => self.vertex_position(verts[0]).await?,
            WeldMode::Center => {
                let mut c = Vec3::ZERO;
                for v in verts {
                    c = c.add(self.vertex_position(*v).await?).await;
                }
                c.scale(1.0 / verts.len() as f32).await
            }
            WeldMode::ByDistance => self.vertex_position(verts[0]).await?,
        };
        let (positions, face_list) = self.polygon_soup().await;
        let keep = verts[0].0;
        let mut remap: HashMap<u32, u32> = HashMap::new();
        for v in verts {
            if mode == WeldMode::ByDistance {
                let pos = self.vertex_position(*v).await?;
                if pos.sub(target_pos).await.length().await <= threshold {
                    remap.insert(v.0, keep);
                }
            } else {
                remap.insert(v.0, keep);
            }
        }
        let mut new_positions = positions;
        new_positions[keep as usize] = target_pos.0;
        let new_faces: Vec<Vec<u32>> = face_list
            .into_iter()
            .map(|f| f.into_iter().map(|vi| *remap.get(&vi).unwrap_or(&vi)).collect::<Vec<_>>())
            .filter(|f| {
                let mut unique = f.clone();
                unique.sort();
                unique.dedup();
                unique.len() >= 3
            })
            .collect();
        self.rebuild_from_polygon_soup(&new_positions, &new_faces).await
    }

    pub async fn dissolve_edges(&mut self, edges: &[EdgeId]) -> MeshResult<()> {
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (positions, mut face_list) = self.polygon_soup().await;
        for &eid in edges {
            let Ok((v0, v1)) = self.edge_endpoints(eid).await else {
                continue;
            };
            let mut a_idx = None;
            let mut b_idx = None;
            for (fi, face) in face_list.iter().enumerate() {
                if find_edge_position(face, v0.0, v1.0).await.is_some() {
                    a_idx = Some(fi);
                }
                if find_edge_position(face, v1.0, v0.0).await.is_some() {
                    b_idx = Some(fi);
                }
            }
            if let (Some(ai), Some(bi)) = (a_idx, b_idx) {
                if ai == bi {
                    continue;
                }
                if let Some(merged) = merge_face_loops(&face_list[ai], &face_list[bi]).await {
                    let (keep, remove) = if ai < bi { (ai, bi) } else { (bi, ai) };
                    face_list[keep] = merged;
                    face_list.remove(remove);
                }
            }
        }
        let cleaned = collinear_cleanup(&positions, &face_list).await;
        self.rebuild_from_polygon_soup(&positions, &cleaned).await
    }

    pub async fn dissolve_vertices(&mut self, verts: &[VertexId]) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let remove: HashMap<u32, bool> = verts.iter().map(|v| (v.0, true)).collect();
        let (positions, face_list) = self.polygon_soup().await;
        let new_faces: Vec<Vec<u32>> = face_list.into_iter().filter(|f| !f.iter().any(|vi| remove.contains_key(vi))).collect();
        self.rebuild_from_polygon_soup(&positions, &new_faces).await
    }

    pub async fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (mut positions, face_list) = self.polygon_soup().await;
        let selected: HashMap<u32, bool> = faces.iter().map(|f| (f.0, true)).collect();
        let mut new_faces = Vec::new();
        for (fi, face) in face_list.iter().enumerate() {
            if !selected.contains_key(&(fi as u32)) {
                new_faces.push(face.clone());
                continue;
            }
            if face.len() < 3 {
                continue;
            }
            let mut centroid = Vec3::ZERO;
            for &vi in face {
                centroid = centroid.add(Vec3(positions[vi as usize])).await;
            }
            centroid = centroid.scale(1.0 / face.len() as f32).await;
            let ci = positions.len() as u32;
            positions.push(centroid.0);
            let mut mids = Vec::new();
            for i in 0..face.len() {
                let a = Vec3(positions[face[i] as usize]);
                let b = Vec3(positions[face[(i + 1) % face.len()] as usize]);
                let mid = a.lerp(b, 0.5);
                let mi = positions.len() as u32;
                positions.push(mid.await.0);
                mids.push(mi);
            }
            for i in 0..face.len() {
                let v = face[i];
                let m0 = mids[i];
                new_faces.push(vec![v, m0, ci]);
                new_faces.push(vec![m0, mids[(i + 1) % face.len()], ci]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &new_faces).await
    }

    pub async fn triangulate(&mut self) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup().await;
        let mut new_faces = Vec::new();
        for face in face_list {
            if face.len() <= 3 {
                new_faces.push(face);
                continue;
            }
            let face_positions: Vec<Vec3> = face.iter().map(|&vi| Vec3(positions[vi as usize])).collect();
            for tri in triangulate_polygon(&face_positions).await {
                new_faces.push(vec![face[tri[0]], face[tri[1]], face[tri[2]]]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &new_faces).await
    }

    /// Merges every pair of adjacent faces whose normals are parallel and whose union is planar (within kernel
    /// tolerances) into a single n-gon, then drops resulting straight-pass-through (collinear) vertices. Returns
    /// the number of merges performed.
    pub async fn merge_coplanar_faces(&mut self) -> MeshResult<usize> {
        let (positions, mut face_list) = self.polygon_soup().await;
        let mut merge_count = 0usize;
        loop {
            let mut merged_this_round = false;
            'search: for ai in 0..face_list.len() {
                let a_len = face_list[ai].len();
                for i in 0..a_len {
                    let u = face_list[ai][i];
                    let v = face_list[ai][(i + 1) % a_len];
                    let mut bi_found = None;
                    for (bi, f) in face_list.iter().enumerate() {
                        if bi != ai && find_edge_position(f, v, u).await.is_some() {
                            bi_found = Some(bi);
                            break;
                        }
                    }
                    let Some(bi) = bi_found else { continue };
                    if !faces_coplanar(&positions, &face_list[ai], &face_list[bi]).await {
                        continue;
                    }
                    if let Some(merged) = merge_face_loops(&face_list[ai], &face_list[bi]).await {
                        let (keep, remove) = if ai < bi { (ai, bi) } else { (bi, ai) };
                        face_list[keep] = merged;
                        face_list.remove(remove);
                        merge_count += 1;
                        merged_this_round = true;
                        break 'search;
                    }
                }
            }
            if !merged_this_round {
                break;
            }
        }
        let cleaned = collinear_cleanup(&positions, &face_list).await;
        self.rebuild_from_polygon_soup(&positions, &cleaned).await?;
        Ok(merge_count)
    }

    /// Unifies every group of vertices at (nearly) the same position — as commonly produced by importers
    /// that tessellate adjacent source faces independently, leaving duplicate, non-shared vertex ids along
    /// shared boundaries — into a single vertex id per position, so the halfedge topology (twins, boundary
    /// detection) reflects the true geometric connectivity. Returns the number of vertices removed.
    pub async fn weld_coincident_vertices(&mut self, precision: f32) -> MeshResult<usize> {
        let (positions, face_list) = self.polygon_soup().await;
        let scale = 1.0 / precision.max(1e-9);
        let mut groups: HashMap<(i64, i64, i64), u32> = HashMap::new();
        let mut compacted_positions: Vec<[f32; 3]> = Vec::new();
        let mut remap: Vec<u32> = Vec::with_capacity(positions.len());
        for p in &positions {
            let key = ((p[0] as f64 * scale as f64).round() as i64, (p[1] as f64 * scale as f64).round() as i64, (p[2] as f64 * scale as f64).round() as i64);
            let canonical = *groups.entry(key).or_insert_with(|| {
                compacted_positions.push(*p);
                (compacted_positions.len() - 1) as u32
            });
            remap.push(canonical);
        }
        let removed = positions.len() - compacted_positions.len();
        if removed == 0 {
            return Ok(0);
        }
        let new_faces: Vec<Vec<u32>> = face_list
            .into_iter()
            .map(|f| f.into_iter().map(|vi| remap[vi as usize]).collect::<Vec<u32>>())
            .filter(|f| {
                let mut unique = f.clone();
                unique.sort();
                unique.dedup();
                unique.len() >= 3
            })
            .collect();
        self.rebuild_from_polygon_soup(&compacted_positions, &new_faces).await?;
        Ok(removed)
    }

    /// Flips faces so every undirected edge is traversed in opposite directions by its two incident faces.
    /// CAD imports often leave inconsistently oriented face wires; without this pass, halfedge twins are
    /// missing even though the undirected mesh is closed. Returns the number of faces flipped.
    pub async fn orient_faces_consistently(&mut self) -> MeshResult<usize> {
        let (positions, mut face_list) = self.polygon_soup().await;
        if face_list.is_empty() {
            return Ok(0);
        }
        let mut edge_faces: HashMap<(u32, u32), Vec<(usize, bool)>> = HashMap::new();
        for (fi, face) in face_list.iter().enumerate() {
            let n = face.len();
            for i in 0..n {
                let a = face[i];
                let b = face[(i + 1) % n];
                let key = if a < b { (a, b) } else { (b, a) };
                let forward = a < b;
                edge_faces.entry(key).or_default().push((fi, forward));
            }
        }
        let mut adjacency: Vec<Vec<(usize, bool)>> = vec![Vec::new(); face_list.len()];
        for owners in edge_faces.values() {
            if owners.len() != 2 {
                continue;
            }
            let (a, a_forward) = owners[0];
            let (b, b_forward) = owners[1];
            // Same directed sense on a shared undirected edge ⇒ neighbor needs a relative flip.
            let needs_relative_flip = a_forward == b_forward;
            adjacency[a].push((b, needs_relative_flip));
            adjacency[b].push((a, needs_relative_flip));
        }
        let mut oriented = vec![false; face_list.len()];
        let mut flip = vec![false; face_list.len()];
        let mut flips = 0usize;
        for start in 0..face_list.len() {
            if oriented[start] {
                continue;
            }
            let mut stack = vec![start];
            oriented[start] = true;
            while let Some(fi) = stack.pop() {
                for &(neighbor, needs_relative_flip) in &adjacency[fi] {
                    let neighbor_flip = flip[fi] ^ needs_relative_flip;
                    if !oriented[neighbor] {
                        oriented[neighbor] = true;
                        flip[neighbor] = neighbor_flip;
                        if neighbor_flip {
                            flips += 1;
                        }
                        stack.push(neighbor);
                    }
                }
            }
        }
        for (fi, face) in face_list.iter_mut().enumerate() {
            if flip[fi] {
                face.reverse();
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list).await?;
        Ok(flips)
    }

    /// Finds every closed boundary loop (a chain of edges with no opposite face on the other side) in the
    /// current halfedge topology and caps each with a new n-gon face, so the mesh becomes watertight. Call
    /// `weld_coincident_vertices` first if the mesh may contain importer-duplicated boundary vertices, or
    /// this will also "cap" seams that are actually already shared with a differently-indexed neighbor.
    /// Returns the number of holes filled.
    pub async fn fill_holes(&mut self) -> MeshResult<usize> {
        // Proper half-edge boundary walk: from a boundary half-edge (twin=None), the next boundary
        // half-edge of the SAME hole loop is found by rotating around its destination vertex via
        // next/twin jumps until another twin-less half-edge is hit. This correctly disambiguates separate
        // holes that happen to share a corner vertex (a vertex-only "next" map cannot).
        let he_count = self.halfedges.len() as u32;
        let mut visited: HashSet<u32> = HashSet::new();
        let mut new_loops: Vec<Vec<u32>> = Vec::new();
        for start in 0..he_count {
            if self.halfedges[start as usize].twin.is_some() || visited.contains(&start) {
                continue;
            }
            let mut loop_he_ids = vec![start];
            visited.insert(start);
            let mut current = start;
            let mut closed = false;
            loop {
                let mut probe = self.halfedges[current as usize].next;
                let mut guard = 0usize;
                let next_boundary = loop {
                    let Some(twin) = self.halfedges[probe as usize].twin else {
                        break Some(probe);
                    };
                    probe = self.halfedges[twin as usize].next;
                    guard += 1;
                    if guard > self.halfedges.len() + 4 {
                        break None;
                    }
                };
                let Some(next_he) = next_boundary else { break };
                if next_he == start {
                    closed = true;
                    break;
                }
                if visited.contains(&next_he) {
                    break;
                }
                visited.insert(next_he);
                loop_he_ids.push(next_he);
                current = next_he;
            }
            if closed && loop_he_ids.len() >= 3 {
                new_loops.push(loop_he_ids.iter().map(|&he| self.halfedges[he as usize].vertex).collect());
            }
        }
        if new_loops.is_empty() {
            return Ok(0);
        }
        let filled = new_loops.len();
        let (positions, face_list) = self.polygon_soup().await;
        let mut all_faces = face_list;
        for mut loop_verts in new_loops {
            // Boundary loops are traced in the "hole" direction (following existing faces' own winding
            // around the missing area); the cap face must have the opposite winding to be consistent.
            loop_verts.reverse();
            all_faces.push(loop_verts);
        }
        self.rebuild_from_polygon_soup(&positions, &all_faces).await?;
        Ok(filled)
    }

    pub async fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup().await;
        let mut all_positions = positions.clone();
        let offset = all_positions.len() as u32;
        for p in &positions {
            let mut np = *p;
            match axis {
                MirrorAxis::X => np[0] = -np[0],
                MirrorAxis::Y => np[1] = -np[1],
                MirrorAxis::Z => np[2] = -np[2],
            }
            all_positions.push(np);
        }
        let mut all_faces = face_list.clone();
        for face in &face_list {
            let mut mirrored: Vec<u32> = face.iter().map(|&vi| vi + offset).collect();
            mirrored.reverse();
            all_faces.push(mirrored);
        }
        self.rebuild_from_polygon_soup(&all_positions, &all_faces).await?;
        let vert_count = self.vertex_count().await;
        let mut to_merge: Vec<VertexId> = Vec::new();
        for i in 0..vert_count {
            for j in (i + 1)..vert_count {
                let pi = self.vertex_position(VertexId(i as u32)).await?;
                let pj = self.vertex_position(VertexId(j as u32)).await?;
                if pi.sub(pj).await.length().await <= weld_threshold {
                    to_merge.push(VertexId(i as u32));
                    to_merge.push(VertexId(j as u32));
                    if to_merge.len() >= 2 {
                        let batch = to_merge.clone();
                        to_merge.clear();
                        let _ = self.merge_vertices(&batch, WeldMode::Center, weld_threshold);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn decimate(&mut self, target_ratio: f32) -> MeshResult<()> {
        let ratio = target_ratio.clamp(0.1, 1.0);
        let target_verts = ((self.vertex_count().await as f32) * ratio).ceil() as usize;
        if target_verts >= self.vertex_count().await {
            return Ok(());
        }
        // 🩹️ Merging remaps a vertex id but never shrinks `self.vertices`, so `self.vertex_count()` itself
        // never drops; track the live (merged-away-aware) count locally for the loop guard instead, or this
        // never converges on `target_verts` and instead collapses edges until the mesh is empty.
        let mut live_verts = self.vertex_count().await;
        while live_verts > target_verts && self.edge_count().await > 0 {
            let mut shortest: Option<(EdgeId, f32)> = None;
            for he_id in 0..self.halfedges.len() {
                if he_id % 2 != 0 {
                    continue;
                }
                if let Ok((v0, v1)) = self.edge_endpoints(EdgeId(he_id as u32)).await {
                    let len = self.vertex_position(v0).await?.sub(self.vertex_position(v1).await?).await.length().await;
                    if shortest.map(|(_, l)| len < l).unwrap_or(true) {
                        shortest = Some((EdgeId(he_id as u32), len));
                    }
                }
            }
            let Some((edge, _)) = shortest else { break };
            let (v0, v1) = self.edge_endpoints(edge).await?;
            let p0 = self.vertex_position(v0).await?;
            let p1 = self.vertex_position(v1).await?;
            let mid = p0.lerp(p1, 0.5);
            let (positions, face_list) = self.polygon_soup().await;
            let mut remap: HashMap<u32, u32> = HashMap::new();
            remap.insert(v1.0, v0.0);
            let mut new_positions = positions;
            new_positions[v0.0 as usize] = mid.await.0;
            let new_faces: Vec<Vec<u32>> = face_list
                .into_iter()
                .map(|f| f.into_iter().map(|vi| *remap.get(&vi).unwrap_or(&vi)).collect())
                .filter(|f: &Vec<u32>| {
                    let mut u = f.clone();
                    u.sort();
                    u.dedup();
                    u.len() >= 3
                })
                .collect();
            self.rebuild_from_polygon_soup(&new_positions, &new_faces).await?;
            live_verts -= 1;
        }
        self.drop_unreferenced_vertices().await
    }

    /// 🧹️ Compacts away vertices no longer referenced by any face, so `vertex_count()` reflects the mesh's
    /// actual remaining complexity after operations (like [`Self::decimate`]) that remap vertex ids away
    /// without themselves shrinking the position buffer.
    async fn drop_unreferenced_vertices(&mut self) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup().await;
        let mut used = vec![false; positions.len()];
        for face in &face_list {
            for &vi in face {
                used[vi as usize] = true;
            }
        }
        if used.iter().all(|&u| u) {
            return Ok(());
        }
        let mut remap = vec![0u32; positions.len()];
        let mut compacted = Vec::new();
        for (i, &keep) in used.iter().enumerate() {
            if keep {
                remap[i] = compacted.len() as u32;
                compacted.push(positions[i]);
            }
        }
        let new_faces: Vec<Vec<u32>> = face_list.into_iter().map(|f| f.into_iter().map(|vi| remap[vi as usize]).collect()).collect();
        self.rebuild_from_polygon_soup(&compacted, &new_faces).await
    }

    pub async fn set_shading(&mut self, faces: &[FaceId], smooth: bool) -> MeshResult<()> {
        for &fid in faces {
            let f = self.faces.get_mut(fid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            f.smooth = smooth;
        }
        Ok(())
    }

    pub async fn recompute_normals(&mut self) -> MeshResult<()> {
        for v in &mut self.vertices {
            v.normal = None;
        }
        for fi in 0..self.faces.len() {
            let face_normal = self.face_normal(FaceId(fi as u32)).await?;
            let verts = self.face_vertex_ids(FaceId(fi as u32)).await?;
            let smooth = self.faces[fi].smooth;
            for v in verts {
                let vert = &mut self.vertices[v.0 as usize];
                if smooth {
                    let n = vert.normal.map(Vec3).unwrap_or(Vec3::ZERO).add(face_normal);
                    vert.normal = Some(n.await.normalize().await.0);
                } else {
                    vert.normal = Some(face_normal.0);
                }
            }
        }
        Ok(())
    }
}

async fn segment_plane_intersect(a: Vec3, b: Vec3, plane_point: Vec3, plane_normal: Vec3) -> Option<(f32, Vec3)> {
    let ab = b.sub(a).await;
    let denom = plane_normal.dot(ab).await;
    if denom.abs() < 1e-8 {
        return None;
    }
    let numer = plane_normal.dot(plane_point.sub(a).await).await;
    let t = numer / denom;
    if t < 0.0 || t > 1.0 {
        return None;
    }
    Some((t, a.add(ab.scale(t).await).await))
}

//#endregion Edit

//#region Uv

async fn cot_angle(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let ab = b.sub(a).await;
    let ac = c.sub(a).await;
    let cross_len = ab.cross(ac).await.length().await;
    if cross_len < 1e-8 {
        return 0.0;
    }
    ab.dot(ac).await / cross_len
}

async fn solve_lscm_1d(n: usize, triplets: &[(usize, usize, f64)], pin_a: usize, pin_b: usize, val_a: f64, val_b: f64) -> Vec<f64> {
    let free: Vec<usize> = (0..n).filter(|&i| i != pin_a && i != pin_b).collect();
    let m = free.len();
    if m == 0 {
        let mut out = vec![0.0; n];
        out[pin_a] = val_a;
        out[pin_b] = val_b;
        return out;
    }
    let mut a = vec![0.0f64; m * m];
    let mut b = vec![0.0f64; m];
    let idx = |v: usize| -> Option<usize> {
        if v == pin_a || v == pin_b {
            None
        } else {
            free.iter().position(|&x| x == v)
        }
    };
    for &(i, j, w) in triplets {
        if i == j {
            if let Some(ii) = idx(i) {
                a[ii * m + ii] += w;
            }
        } else {
            if let Some(ii) = idx(i) {
                if let Some(jj) = idx(j) {
                    a[ii * m + jj] -= w;
                } else if j == pin_a {
                    b[ii] += w * val_a;
                } else if j == pin_b {
                    b[ii] += w * val_b;
                }
            }
            if let Some(jj) = idx(j) {
                if let Some(ii) = idx(i) {
                    a[jj * m + ii] -= w;
                } else if i == pin_a {
                    b[jj] += w * val_a;
                } else if i == pin_b {
                    b[jj] += w * val_b;
                }
            }
        }
    }
    for row in 0..m {
        let mut pivot = row;
        for r in (row + 1)..m {
            if a[r * m + row].abs() > a[pivot * m + row].abs() {
                pivot = r;
            }
        }
        if a[pivot * m + row].abs() < 1e-12 {
            continue;
        }
        if pivot != row {
            for c in 0..m {
                a.swap(row * m + c, pivot * m + c);
            }
            b.swap(row, pivot);
        }
        let div = a[row * m + row];
        for c in row..m {
            a[row * m + c] /= div;
        }
        b[row] /= div;
        for r in 0..m {
            if r == row {
                continue;
            }
            let factor = a[r * m + row];
            if factor.abs() < 1e-12 {
                continue;
            }
            for c in row..m {
                a[r * m + c] -= factor * a[row * m + c];
            }
            b[r] -= factor * b[row];
        }
    }
    let mut out = vec![0.0; n];
    out[pin_a] = val_a;
    out[pin_b] = val_b;
    for (fi, &vi) in free.iter().enumerate() {
        out[vi] = b[fi];
    }
    out
}

impl HalfedgeMesh {
    pub async fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool) {
        for &edge in edges {
            self.uv_seams.insert(edge.0);
            if !seam {
                self.uv_seams.remove(&edge.0);
            }
        }
    }

    pub async fn is_uv_seam(&self, edge: EdgeId) -> bool {
        self.uv_seams.contains(&edge.0)
    }

    async fn uv_island_faces(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.faces.len()];
        let mut islands = Vec::new();
        for start in 0..self.faces.len() {
            if visited[start] {
                continue;
            }
            let mut stack = vec![start];
            let mut island = Vec::new();
            visited[start] = true;
            while let Some(fi) = stack.pop() {
                island.push(fi);
                let hes = self.face_halfedge_ids(FaceId(fi as u32)).await.unwrap_or_default();
                for he_id in hes {
                    let he = &self.halfedges[he_id as usize];
                    if self.uv_seams.contains(&he_id) {
                        continue;
                    }
                    if let Some(twin_id) = he.twin {
                        let twin = &self.halfedges[twin_id as usize];
                        if let Some(adj) = twin.face {
                            let adj = adj as usize;
                            if !visited[adj] {
                                visited[adj] = true;
                                stack.push(adj);
                            }
                        }
                    }
                }
            }
            if !island.is_empty() {
                islands.push(island);
            }
        }
        islands
    }

    async fn solve_island_uv(&self, island_faces: &[usize]) -> HashMap<u32, [f32; 2]> {
        let mut vert_set: HashSet<u32> = HashSet::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for &fi in island_faces {
            let verts = self.face_vertex_ids(FaceId(fi as u32)).await.unwrap_or_default();
            if verts.len() < 3 {
                continue;
            }
            let face_positions: Vec<Vec3> = verts.iter().map(|v| Vec3(self.vertices[v.0 as usize].position)).collect();
            for tri in triangulate_polygon(&face_positions).await {
                triangles.push([verts[tri[0]].0, verts[tri[1]].0, verts[tri[2]].0]);
            }
            for v in verts {
                vert_set.insert(v.0);
            }
        }
        let verts: Vec<u32> = vert_set.into_iter().collect();
        let n = verts.len();
        if n < 3 {
            return HashMap::new();
        }
        let index: HashMap<u32, usize> = verts.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let pos = |vid: u32| Vec3(self.vertices[vid as usize].position);
        let mut triplets: Vec<(usize, usize, f64)> = Vec::new();
        for tri in &triangles {
            let (a, b, c) = (tri[0], tri[1], tri[2]);
            let ia = index[&a];
            let ib = index[&b];
            let ic = index[&c];
            let pa = pos(a);
            let pb = pos(b);
            let pc = pos(c);
            let cot_a = cot_angle(pb, pa, pc).await as f64;
            let cot_b = cot_angle(pa, pb, pc).await as f64;
            let cot_c = cot_angle(pa, pc, pb).await as f64;
            let pairs = [(ia, ib, cot_c), (ib, ia, cot_c), (ib, ic, cot_a), (ic, ib, cot_a), (ia, ic, cot_b), (ic, ia, cot_b)];
            for (i, j, w) in pairs {
                if w.abs() < 1e-12 {
                    continue;
                }
                triplets.push((i, i, w));
                triplets.push((i, j, -w));
            }
        }
        let pin_a = 0;
        let mut pin_b = 1;
        let mut max_dist = 0.0f32;
        let p0 = pos(verts[pin_a]);
        for (i, &v) in verts.iter().enumerate().skip(1) {
            let d = p0.sub(pos(v)).await.length().await;
            if d > max_dist {
                max_dist = d;
                pin_b = i;
            }
        }
        let u = solve_lscm_1d(n, &triplets, pin_a, pin_b, 0.0, 1.0).await;
        let v = solve_lscm_1d(n, &triplets, pin_a, pin_b, 0.0, 0.0).await;
        verts.into_iter().enumerate().map(|(i, vid)| (vid, [u[i] as f32, v[i] as f32])).collect()
    }

    async fn pack_island_uvs(&self, islands: &[Vec<usize>]) -> HashMap<u32, [f32; 2]> {
        let mut packed = HashMap::new();
        let mut shelf_y = 0.0f32;
        let mut shelf_height = 0.0f32;
        let mut shelf_x = 0.0f32;
        const PAD: f32 = 0.01;
        for island in islands {
            let local = self.solve_island_uv(island).await;
            if local.is_empty() {
                continue;
            }
            let mut min_u = f32::INFINITY;
            let mut min_v = f32::INFINITY;
            let mut max_u = f32::NEG_INFINITY;
            let mut max_v = f32::NEG_INFINITY;
            for uv in local.values() {
                min_u = min_u.min(uv[0]);
                min_v = min_v.min(uv[1]);
                max_u = max_u.max(uv[0]);
                max_v = max_v.max(uv[1]);
            }
            let w = (max_u - min_u).max(1e-4);
            let h = (max_v - min_v).max(1e-4);
            if shelf_x + w + PAD > 1.0 {
                shelf_x = 0.0;
                shelf_y += shelf_height + PAD;
                shelf_height = 0.0;
            }
            shelf_height = shelf_height.max(h);
            let scale = (w.max(h)).min(0.45);
            for (vid, uv) in local {
                let nu = shelf_x + (uv[0] - min_u) / w * scale;
                let nv = shelf_y + (uv[1] - min_v) / h * scale;
                packed.insert(vid, [nu, nv]);
            }
            shelf_x += scale + PAD;
        }
        packed
    }

    pub async fn unwrap_uv(&mut self) -> MeshResult<()> {
        let islands = self.uv_island_faces().await;
        let packed = self.pack_island_uvs(&islands).await;
        for fi in 0..self.faces.len() {
            let hes = self.face_halfedge_ids(FaceId(fi as u32)).await?;
            for he_id in hes {
                let he = &mut self.halfedges[he_id as usize];
                if let Some(uv) = packed.get(&he.vertex) {
                    he.uv = *uv;
                }
            }
        }
        Ok(())
    }
}

//#endregion Uv

//#region Polygon

/// Newell's method: robust face normal for arbitrary (including non-planar/concave/collinear-first-corner) loops.
async fn newell_normal(positions: &[Vec3]) -> Vec3 {
    let n = positions.len();
    let mut nx = 0.0f64;
    let mut ny = 0.0f64;
    let mut nz = 0.0f64;
    for i in 0..n {
        let a = positions[i];
        let b = positions[(i + 1) % n];
        let (ax, ay, az) = (a.x().await as f64, a.y().await as f64, a.z().await as f64);
        let (bx, by, bz) = (b.x().await as f64, b.y().await as f64, b.z().await as f64);
        nx += (ay - by) * (az + bz);
        ny += (az - bz) * (ax + bx);
        nz += (ax - bx) * (ay + by);
    }
    Vec3::new(nx as f32, ny as f32, nz as f32).await
}

type Vec3f64 = (f64, f64, f64);

async fn sub3(a: Vec3f64, b: Vec3f64) -> Vec3f64 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

async fn dot3(a: Vec3f64, b: Vec3f64) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

async fn cross3(a: Vec3f64, b: Vec3f64) -> Vec3f64 {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}

async fn length3(a: Vec3f64) -> f64 {
    dot3(a, a).await.sqrt()
}

async fn normalize3(a: Vec3f64) -> Vec3f64 {
    let l = length3(a).await;
    if l < 1e-12 {
        return (0.0, 0.0, 0.0);
    }
    (a.0 / l, a.1 / l, a.2 / l)
}

/// Least-parallel-world-axis projection basis for a plane with the given unit normal.
async fn plane_basis(normal: Vec3f64) -> (Vec3f64, Vec3f64) {
    let (nx, ny, nz) = normal;
    let reference: Vec3f64 = if nx.abs() < ny.abs() && nx.abs() < nz.abs() {
        (1.0, 0.0, 0.0)
    } else if ny.abs() < nz.abs() {
        (0.0, 1.0, 0.0)
    } else {
        (0.0, 0.0, 1.0)
    };
    let axis_u = normalize3(cross3(normal, reference).await).await;
    let axis_v = cross3(normal, axis_u).await;
    (axis_u, axis_v)
}

async fn cross2(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

async fn point_in_triangle(p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let d1 = cross2(a, b, p).await;
    let d2 = cross2(b, c, p).await;
    let d3 = cross2(c, a, p).await;
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

/// Triangulates a planar polygon that may contain holes. Holes are bridged into the outer loop with a
/// keyhole (doubled bridge edge) then ear-clipped, so the result covers only the solid region.
async fn triangulate_indexed_polygon_with_holes(positions: &[[f32; 3]], outer: &[u32], holes: &[Vec<u32>]) -> Vec<[u32; 3]> {
    if holes.is_empty() {
        let pts: Vec<Vec3> = outer.iter().map(|&i| Vec3(positions[i as usize])).collect();
        return triangulate_polygon(&pts).await.into_iter().map(|[a, b, c]| [outer[a], outer[b], outer[c]]).collect();
    }
    let mut combined: Vec<u32> = outer.to_vec();
    let mut remaining: Vec<Vec<u32>> = holes.to_vec();
    while let Some((hole_index, outer_pos, hole_pos)) = find_closest_bridge(&combined, &remaining, positions).await {
        let hole = remaining.remove(hole_index);
        let mut spliced = Vec::with_capacity(combined.len() + hole.len() + 2);
        spliced.extend_from_slice(&combined[..=outer_pos]);
        spliced.push(hole[hole_pos]);
        for k in 1..hole.len() {
            spliced.push(hole[(hole_pos + k) % hole.len()]);
        }
        spliced.push(hole[hole_pos]);
        spliced.push(combined[outer_pos]);
        spliced.extend_from_slice(&combined[outer_pos + 1..]);
        combined = spliced;
    }
    let pts: Vec<Vec3> = combined.iter().map(|&i| Vec3(positions[i as usize])).collect();
    triangulate_polygon(&pts).await.into_iter().map(|[a, b, c]| [combined[a], combined[b], combined[c]]).collect()
}

async fn find_closest_bridge(outer: &[u32], holes: &[Vec<u32>], positions: &[[f32; 3]]) -> Option<(usize, usize, usize)> {
    let mut best: Option<(f32, usize, usize, usize)> = None;
    for (hi, hole) in holes.iter().enumerate() {
        for (oi, &ov) in outer.iter().enumerate() {
            let operation = Vec3(positions[ov as usize]);
            for (hpi, &hv) in hole.iter().enumerate() {
                let hp = Vec3(positions[hv as usize]);
                let d = operation.sub(hp).await.length().await;
                if best.map(|(bd, _, _, _)| d < bd).unwrap_or(true) {
                    best = Some((d, hi, oi, hpi));
                }
            }
        }
    }
    best.map(|(_, hi, oi, hpi)| (hi, oi, hpi))
}

/// Deterministic ear-clipping triangulation of a simple polygon (convex or concave), given ordered 3D corner
/// positions. Falls back to a fan (previous behavior) whenever the polygon is degenerate (zero-area / collinear)
/// or clipping stalls, so it never returns fewer than `n - 2` triangles or panics.
async fn triangulate_polygon(positions: &[Vec3]) -> Vec<[usize; 3]> {
    let n = positions.len();
    if n < 3 {
        return Vec::new();
    }
    if n == 3 {
        return vec![[0, 1, 2]];
    }
    let fan = |from: usize| -> Vec<[usize; 3]> {
        let _ = from;
        (1..n - 1).map(|i| [0, i, i + 1]).collect()
    };
    let mut points_f64: Vec<Vec3f64> = Vec::with_capacity(positions.len());
    for p in positions {
        points_f64.push((p.x().await as f64, p.y().await as f64, p.z().await as f64));
    }
    let normal = newell_normal(positions).await;
    if normal.length().await < 1e-8 {
        return fan(0);
    }
    let normal_f64 = normalize3((normal.x().await as f64, normal.y().await as f64, normal.z().await as f64)).await;
    let (axis_u, axis_v) = plane_basis(normal_f64).await;
    let origin = points_f64[0];
    let mut projected: Vec<(f64, f64)> = Vec::with_capacity(points_f64.len());
    for &p in &points_f64 {
        let local = sub3(p, origin).await;
        projected.push((dot3(local, axis_u).await, dot3(local, axis_v).await));
    }
    let mut signed_area2 = 0.0f64;
    for i in 0..n {
        let a = projected[i];
        let b = projected[(i + 1) % n];
        signed_area2 += a.0 * b.1 - b.0 * a.1;
    }
    if signed_area2.abs() < 1e-14 {
        return fan(0);
    }
    let ccw = signed_area2 > 0.0;

    let mut indices: Vec<usize> = (0..n).collect();
    let mut triangles = Vec::with_capacity(n - 2);
    let mut guard = 0usize;
    let guard_limit = n * n + 16;
    while indices.len() > 3 {
        guard += 1;
        if guard > guard_limit {
            for i in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[i], indices[i + 1]]);
            }
            return triangles;
        }
        let m = indices.len();
        let mut ear_found = false;
        for i in 0..m {
            let prev = indices[(i + m - 1) % m];
            let curr = indices[i];
            let next = indices[(i + 1) % m];
            let a = projected[prev];
            let b = projected[curr];
            let c = projected[next];
            let cross = cross2(a, b, c).await;
            let is_convex = if ccw { cross > 1e-14 } else { cross < -1e-14 };
            if !is_convex {
                continue;
            }
            let mut contains_other = false;
            for &k in &indices {
                if k == prev || k == curr || k == next {
                    continue;
                }
                if point_in_triangle(projected[k], a, b, c).await {
                    contains_other = true;
                    break;
                }
            }
            if contains_other {
                continue;
            }
            triangles.push([prev, curr, next]);
            indices.remove(i);
            ear_found = true;
            break;
        }
        if !ear_found {
            for i in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[i], indices[i + 1]]);
            }
            return triangles;
        }
    }
    triangles.push([indices[0], indices[1], indices[2]]);
    triangles
}

async fn find_edge_position(loop_verts: &[u32], from: u32, to: u32) -> Option<usize> {
    let n = loop_verts.len();
    (0..n).find(|&i| loop_verts[i] == from && loop_verts[(i + 1) % n] == to)
}

/// Merges two face loops that share exactly one boundary edge (in opposite winding, as guaranteed by a
/// consistently-oriented manifold) into a single n-gon loop. Returns `None` if the faces do not share exactly
/// one edge, or if splicing would produce a loop with a repeated vertex (non-simple / holed result).
async fn merge_face_loops(a: &[u32], b: &[u32]) -> Option<Vec<u32>> {
    let n = a.len();
    let m = b.len();
    if n < 3 || m < 3 {
        return None;
    }
    let mut shared_a_pos = None;
    for i in 0..n {
        let u = a[i];
        let v = a[(i + 1) % n];
        if find_edge_position(b, v, u).await.is_some() {
            shared_a_pos = Some(i);
            break;
        }
    }
    let a_pos = shared_a_pos?;
    let u = a[a_pos];
    let v = a[(a_pos + 1) % n];
    let b_pos = find_edge_position(b, v, u).await?;
    for i in 0..n {
        if i == a_pos {
            continue;
        }
        let uu = a[i];
        let vv = a[(i + 1) % n];
        if find_edge_position(b, vv, uu).await.is_some() {
            return None;
        }
    }
    let a_rot: Vec<u32> = (0..n).map(|k| a[(a_pos + k) % n]).collect();
    let b_rot: Vec<u32> = (0..m).map(|k| b[(b_pos + k) % m]).collect();
    let mut merged = Vec::with_capacity(n + m - 2);
    merged.push(a_rot[0]);
    merged.extend_from_slice(&b_rot[2..]);
    merged.extend_from_slice(&a_rot[1..]);
    let mut seen = HashSet::new();
    for &vid in &merged {
        if !seen.insert(vid) {
            return None;
        }
    }
    Some(merged)
}

const COPLANAR_NORMAL_DOT_MIN: f32 = 1.0 - 1e-4;
const COPLANAR_DISTANCE_REL_TOL: f32 = 1e-4;

async fn faces_coplanar(positions: &[[f32; 3]], a: &[u32], b: &[u32]) -> bool {
    let pos = |vi: u32| Vec3(positions[vi as usize]);
    let a_pts: Vec<Vec3> = a.iter().map(|&vi| pos(vi)).collect();
    let b_pts: Vec<Vec3> = b.iter().map(|&vi| pos(vi)).collect();
    let na = newell_normal(&a_pts).await;
    let la = na.length().await;
    if la < 1e-10 {
        return false;
    }
    let na_n = na.scale(1.0 / la).await;
    let nb = newell_normal(&b_pts).await;
    let lb = nb.length().await;
    if lb < 1e-10 {
        return false;
    }
    let nb_n = nb.scale(1.0 / lb).await;
    if na_n.dot(nb_n).await < COPLANAR_NORMAL_DOT_MIN {
        return false;
    }
    let origin = a_pts[0];
    let mut min = a_pts[0];
    let mut max = a_pts[0];
    for &p in a_pts.iter().chain(b_pts.iter()) {
        min = Vec3::new(min.x().await.min(p.x().await), min.y().await.min(p.y().await), min.z().await.min(p.z().await)).await;
        max = Vec3::new(max.x().await.max(p.x().await), max.y().await.max(p.y().await), max.z().await.max(p.z().await)).await;
    }
    let diag = max.sub(min).await.length().await;
    let tol = (COPLANAR_DISTANCE_REL_TOL * diag).max(1e-6);
    for &p in b_pts.iter() {
        if p.sub(origin).await.dot(na_n).await.abs() > tol {
            return false;
        }
    }
    true
}

/// Drops vertices that are a straight (~180°) pass-through in *every* face loop that references them, i.e. whose
/// loop-neighbors are identical across all incident faces. Turns merged coplanar-face borders into clean n-gon
/// corners instead of chains of collinear vertices left over from the original triangulation.
async fn collinear_cleanup(positions: &[[f32; 3]], face_list: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let pos = |vi: u32| Vec3(positions[vi as usize]);
    let mut neighbor_pairs: HashMap<u32, HashSet<(u32, u32)>> = HashMap::new();
    for face in face_list {
        let n = face.len();
        for i in 0..n {
            let prev = face[(i + n - 1) % n];
            let curr = face[i];
            let next = face[(i + 1) % n];
            neighbor_pairs.entry(curr).or_default().insert((prev, next));
        }
    }
    let mut removable: HashSet<u32> = HashSet::new();
    for (&vid, pairs) in &neighbor_pairs {
        if pairs.len() != 1 {
            continue;
        }
        let Some(&(prev, next)) = pairs.iter().next() else { continue };
        if prev == next || prev == vid || next == vid {
            continue;
        }
        let d1 = pos(vid).sub(pos(prev)).await;
        let d2 = pos(next).sub(pos(vid)).await;
        if d1.length().await < 1e-9 || d2.length().await < 1e-9 {
            continue;
        }
        if d1.normalize().await.dot(d2.normalize().await).await > 1.0 - 1e-4 {
            removable.insert(vid);
        }
    }
    face_list.iter().map(|face| face.iter().copied().filter(|v| !removable.contains(v)).collect::<Vec<u32>>()).filter(|face: &Vec<u32>| face.len() >= 3).collect()
}

//#endregion Polygon

//#region Export

impl HalfedgeMesh {
    pub async fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut edge_positions = Vec::new();
        let mut face_ids = Vec::new();
        let mut vertex_ids = Vec::new();
        let mut edge_ids = Vec::new();
        let mut uvs = Vec::new();
        let mut edge_uvs = Vec::new();
        let mut edge_is_seam = Vec::new();
        let mut edge_seen: HashMap<(u32, u32), bool> = HashMap::new();

        for fi in 0..self.faces.len() {
            let face = &self.faces[fi];
            let smooth = face.smooth;
            let topology_hes = self.face_halfedge_ids(FaceId(fi as u32)).await?;
            let topology_verts: Vec<VertexId> = topology_hes.iter().map(|halfedge| VertexId(self.halfedges[*halfedge as usize].vertex)).collect();
            let mut hes = topology_hes.clone();
            if face.flipped {
                hes.reverse();
            }
            let verts = self.face_vertex_ids(FaceId(fi as u32)).await?;
            if verts.len() < 3 {
                continue;
            }
            let face_normal = self.face_normal(FaceId(fi as u32)).await?;
            let base = positions.len() as u32 / 3;

            let push_corner = |he_id: u32, positions: &mut Vec<f32>, normals: &mut Vec<f32>, vertex_ids: &mut Vec<u32>, uvs: &mut Vec<f32>, normal: Vec3| {
                let he = &self.halfedges[he_id as usize];
                let vert = &self.vertices[he.vertex as usize];
                let n = if smooth { vert.normal.map(Vec3).unwrap_or(normal) } else { normal };
                positions.extend_from_slice(&vert.position);
                normals.extend_from_slice(&n.0);
                vertex_ids.push(he.vertex);
                uvs.push(he.uv[0]);
                uvs.push(he.uv[1]);
            };

            let face_positions: Vec<Vec3> = verts.iter().map(|v| Vec3(self.vertices[v.0 as usize].position)).collect();
            let triangles = triangulate_polygon(&face_positions).await;

            if smooth {
                for &he_id in &hes {
                    push_corner(he_id, &mut positions, &mut normals, &mut vertex_ids, &mut uvs, face_normal);
                }
                for tri in &triangles {
                    indices.push(base + tri[0] as u32);
                    indices.push(base + tri[1] as u32);
                    indices.push(base + tri[2] as u32);
                    face_ids.push(fi as u32);
                }
            } else {
                for tri in &triangles {
                    for &local in tri {
                        push_corner(hes[local], &mut positions, &mut normals, &mut vertex_ids, &mut uvs, face_normal);
                    }
                    let tri_base = (positions.len() / 3 - 3) as u32;
                    indices.push(tri_base);
                    indices.push(tri_base + 1);
                    indices.push(tri_base + 2);
                    face_ids.push(fi as u32);
                }
            }

            for i in 0..topology_verts.len() {
                let v0 = topology_verts[i].0;
                let v1 = topology_verts[(i + 1) % topology_verts.len()].0;
                let key = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                if edge_seen.contains_key(&key) {
                    continue;
                }
                edge_seen.insert(key, true);
                let p0 = self.vertices[v0 as usize].position;
                let p1 = self.vertices[v1 as usize].position;
                edge_positions.extend_from_slice(&p0);
                edge_positions.extend_from_slice(&p1);
                let he_id = topology_hes[i];
                let he = &self.halfedges[he_id as usize];
                let he_next = &self.halfedges[he.next as usize];
                edge_ids.push(he_id);
                edge_uvs.push(he.uv[0]);
                edge_uvs.push(he.uv[1]);
                edge_uvs.push(he_next.uv[0]);
                edge_uvs.push(he_next.uv[1]);
                edge_is_seam.push(if self.uv_seams.contains(&he_id) { 1 } else { 0 });
            }
        }

        Ok(MeshTransfer { positions, normals, indices, edge_positions, face_ids, vertex_ids, edge_ids, uvs, edge_uvs, edge_is_seam })
    }

    pub async fn to_obj(&self) -> MeshResult<String> {
        let mut out = String::from("# kernel_3d_mesh OBJ export\n");
        for v in &self.vertices {
            out.push_str(&format!("v {} {} {}\n", v.position[0], v.position[1], v.position[2]));
        }
        let mut vt_written = false;
        for he in &self.halfedges {
            if he.uv[0] != 0.0 || he.uv[1] != 0.0 {
                vt_written = true;
                break;
            }
        }
        if vt_written {
            for he in &self.halfedges {
                out.push_str(&format!("vt {} {}\n", he.uv[0], he.uv[1]));
            }
        }
        for fi in 0..self.faces.len() {
            let mut hes = self.face_halfedge_ids(FaceId(fi as u32)).await?;
            if self.faces[fi].flipped {
                hes.reverse();
            }
            out.push('f');
            for he_id in hes {
                let he = &self.halfedges[he_id as usize];
                if vt_written {
                    out.push_str(&format!(" {}/{}", he.vertex + 1, he_id as usize + 1));
                } else {
                    out.push_str(&format!(" {}", he.vertex + 1));
                }
            }
            out.push('\n');
        }
        Ok(out)
    }

    pub async fn to_json(&self) -> MeshResult<String> {
        serde_json::to_string(self).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }

    pub async fn from_json(json: &str) -> MeshResult<Self> {
        serde_json::from_str(json).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }
}

//#endregion Export

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn box_prim_has_six_faces() {
        let mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        assert_eq!(mesh.face_count().await, 6);
        assert_eq!(mesh.vertex_count().await, 8);
    }

    #[semio_framework_async_macros::async_test]
    async fn plane_prim_single_face() {
        let mesh = HalfedgeMesh::plane_prim(4.0, 4.0).await.unwrap();
        assert_eq!(mesh.face_count().await, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn translate_moves_vertices() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.translate(Vec3::new(1.0, 0.0, 0.0).await).await.unwrap();
        let p = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((p.x().await - 0.5).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_indexed_triangles_builds_triangle_faces() {
        let positions = vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, //
        ];
        let indices = vec![0, 1, 2];
        let mesh = HalfedgeMesh::from_indexed_triangles(&positions, &indices).await.unwrap();
        assert_eq!(mesh.vertex_count().await, 3);
        assert_eq!(mesh.face_count().await, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_indexed_triangles_by_face_id_merges_per_brep_face_without_filling_holes() {
        // Two quads on z=0 sharing no edge (a slab with a gap — like a face pair that must not be bridged),
        // plus a third vertical face that should stay separate. Face ids: 1 covers both coplanar quads'
        // triangles as two separate B-Rep faces (10 and 11), so the gap is never capped.
        let positions = vec![
            0.0, 0.0, 0.0, // 0
            1.0, 0.0, 0.0, // 1
            1.0, 1.0, 0.0, // 2
            0.0, 1.0, 0.0, // 3
            2.0, 0.0, 0.0, // 4
            3.0, 0.0, 0.0, // 5
            3.0, 1.0, 0.0, // 6
            2.0, 1.0, 0.0, // 7
            0.0, 0.0, 1.0, // 8
            1.0, 0.0, 1.0, // 9
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, // face 10 — left quad
            4, 5, 6, 4, 6, 7, // face 11 — right quad (gap between x=1 and x=2)
            0, 1, 9, 0, 9, 8, // face 12 — vertical
        ];
        let face_ids = vec![10, 10, 11, 11, 12, 12];
        let mut mesh = HalfedgeMesh::from_indexed_triangles_by_face_id(&positions, &indices, &face_ids).await.unwrap();
        assert_eq!(mesh.face_count().await, 3, "each B-Rep face becomes one n-gon; gap must not be filled");
        mesh.weld_coincident_vertices(1e-6).await.unwrap();
        let open = {
            let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
            for fi in 0..mesh.face_count().await {
                let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
                let n = verts.len();
                for i in 0..n {
                    *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
                }
            }
            directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
        };
        assert!(open > 0, "gap between the two quads must remain an open boundary, not a filled face");
    }

    #[semio_framework_async_macros::async_test]
    async fn orient_faces_consistently_fixes_same_winding_neighbors() {
        // Two quads sharing edge 1-2, both wound CCW in XY — shared edge has the same directed sense.
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
        let faces = vec![vec![0, 1, 2, 3], vec![1, 2, 5, 4]];
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).await.unwrap();
        let open_before = {
            let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
            for fi in 0..mesh.face_count().await {
                let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
                let n = verts.len();
                for i in 0..n {
                    *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
                }
            }
            directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
        };
        assert!(open_before > 0);
        let flips = mesh.orient_faces_consistently().await.unwrap();
        assert!(flips >= 1);
        let open_after = {
            let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
            for fi in 0..mesh.face_count().await {
                let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
                let n = verts.len();
                for i in 0..n {
                    *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
                }
            }
            directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
        };
        assert_eq!(open_after, 6, "only the outer boundary of the 2-quad strip should remain open");
    }

    #[semio_framework_async_macros::async_test]
    async fn triangulate_produces_triangles_only() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.triangulate().await.unwrap();
        for fi in 0..mesh.face_count().await {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
            assert_eq!(verts.len(), 3);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn extrude_increases_face_count() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let before = mesh.face_count().await;
        mesh.extrude_faces(&[FaceId(0)], 0.5).await.unwrap();
        assert!(mesh.face_count().await > before);
    }

    #[semio_framework_async_macros::async_test]
    async fn tessellate_has_positions_and_indices() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let transfer = mesh.tessellate().await.unwrap();
        assert!(!transfer.positions.is_empty());
        assert!(!transfer.indices.is_empty());
        assert!(!transfer.edge_positions.is_empty());
        assert_eq!(transfer.face_ids.len(), transfer.indices.len() / 3);
        assert_eq!(transfer.vertex_ids.len(), transfer.positions.len() / 3);
        assert_eq!(transfer.edge_ids.len() * 2, transfer.edge_positions.len() / 3);
        assert_eq!(transfer.uvs.len(), transfer.positions.len() / 3 * 2);
        assert_eq!(transfer.edge_uvs.len(), transfer.edge_ids.len() * 4);
        assert_eq!(transfer.edge_is_seam.len(), transfer.edge_ids.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn flip_faces_reverses_only_requested_normals() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let before = mesh.face_normal(FaceId(0)).await.unwrap();
        let edge_ids = mesh.tessellate().await.unwrap().edge_ids;
        mesh.flip_faces(&[FaceId(0)]).await.unwrap();
        let after = mesh.face_normal(FaceId(0)).await.unwrap();
        assert!(before.dot(after).await < -0.99);
        assert_eq!(mesh.tessellate().await.unwrap().edge_ids, edge_ids);
    }

    #[semio_framework_async_macros::async_test]
    async fn unwrap_uv_produces_bounded_coordinates() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.unwrap_uv().await.unwrap();
        let transfer = mesh.tessellate().await.unwrap();
        assert!(!transfer.uvs.is_empty());
        for chunk in transfer.uvs.chunks(2) {
            assert!(chunk[0].is_finite());
            assert!(chunk[1].is_finite());
            assert!(chunk[0] >= -0.01 && chunk[0] <= 1.01);
            assert!(chunk[1] >= -0.01 && chunk[1] <= 1.01);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn obj_export_contains_vertices() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let obj = mesh.to_obj().await.unwrap();
        assert!(obj.contains("v "));
        assert!(obj.contains("f "));
    }

    #[semio_framework_async_macros::async_test]
    async fn ico_sphere_has_faces() {
        let mesh = HalfedgeMesh::ico_sphere_prim(1.0, 1).await.unwrap();
        assert!(mesh.face_count().await > 20);
    }

    #[semio_framework_async_macros::async_test]
    async fn decimate_reduces_vertices() {
        let mut mesh = HalfedgeMesh::ico_sphere_prim(1.0, 2).await.unwrap();
        let before = mesh.vertex_count().await;
        mesh.decimate(0.5).await.unwrap();
        assert!(mesh.vertex_count().await <= before);
    }

    #[semio_framework_async_macros::async_test]
    async fn json_roundtrip() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let json = mesh.to_json().await.unwrap();
        let restored = HalfedgeMesh::from_json(&json).await.unwrap();
        assert_eq!(restored.vertex_count().await, mesh.vertex_count().await);
    }

    #[semio_framework_async_macros::async_test]
    async fn newell_normal_handles_collinear_first_corner() {
        // First three points (0,0,0)-(1,0,0)-(2,0,0) are collinear: the old first-triangle-cross
        // method degenerates to a zero vector here, Newell's method must not.
        let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
        let mesh = HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 2, 3]]).await.unwrap();
        let normal = mesh.face_normal(FaceId(0)).await.unwrap();
        assert!(normal.length().await > 0.99 && normal.length().await < 1.01);
        assert!(normal.dot(Vec3::new(0.0, 0.0, 1.0).await).await.abs() > 0.99);
    }

    #[semio_framework_async_macros::async_test]
    async fn ear_clipping_triangulates_concave_l_polygon() {
        // Concave L-shaped hexagon.
        let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
        let mut positions: Vec<Vec3> = Vec::with_capacity(corners.len());
        for &(x, y) in &corners {
            positions.push(Vec3::new(x, y, 0.0).await);
        }
        let triangles = triangulate_polygon(&positions).await;
        assert_eq!(triangles.len(), corners.len() - 2);
        let shoelace = |pts: &[(f64, f64)]| -> f64 {
            let n = pts.len();
            let mut sum = 0.0;
            for i in 0..n {
                let (x0, y0) = pts[i];
                let (x1, y1) = pts[(i + 1) % n];
                sum += x0 * y1 - x1 * y0;
            }
            sum.abs() * 0.5
        };
        let polygon_area = shoelace(&corners.iter().map(|&(x, y)| (x as f64, y as f64)).collect::<Vec<_>>());
        let mut triangle_area_sum = 0.0f64;
        for tri in &triangles {
            let pts: Vec<(f64, f64)> = tri.iter().map(|&i| (corners[i].0 as f64, corners[i].1 as f64)).collect();
            triangle_area_sum += shoelace(&pts);
            // Nondegenerate.
            assert!(shoelace(&pts) > 1e-6);
        }
        assert!((triangle_area_sum - polygon_area).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_coplanar_faces_reassembles_triangulated_cube_into_quads() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.triangulate().await.unwrap();
        assert_eq!(mesh.face_count().await, 12);
        let merges = mesh.merge_coplanar_faces().await.unwrap();
        assert_eq!(merges, 6);
        assert_eq!(mesh.face_count().await, 6);
        for fi in 0..mesh.face_count().await {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
            assert_eq!(verts.len(), 4);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn dissolve_edges_merges_two_triangles_into_a_quad() {
        let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
        let faces = vec![vec![0, 1, 2], vec![0, 2, 3]];
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).await.unwrap();
        assert_eq!(mesh.face_count().await, 2);
        // Halfedge index 2 is face 0's edge 2->0, the shared diagonal.
        mesh.dissolve_edges(&[EdgeId(2)]).await.unwrap();
        assert_eq!(mesh.face_count().await, 1);
        let verts = mesh.face_vertex_ids(FaceId(0)).await.unwrap();
        assert_eq!(verts.len(), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_and_cleanup_collapses_seam_vertices_of_a_contiguous_strip() {
        // Three coplanar quads in a row sharing seam edges; the seam vertices are used by exactly
        // two faces each and lie on straight boundary lines, so they must be dropped after merge.
        let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0], [3.0, 0.0, 0.0], [3.0, 1.0, 0.0]];
        let faces = vec![vec![0, 1, 2, 3], vec![1, 4, 5, 2], vec![4, 6, 7, 5]];
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).await.unwrap();
        assert_eq!(mesh.face_count().await, 3);
        let merges = mesh.merge_coplanar_faces().await.unwrap();
        assert_eq!(merges, 2);
        assert_eq!(mesh.face_count().await, 1);
        let verts = mesh.face_vertex_ids(FaceId(0)).await.unwrap();
        assert_eq!(verts.len(), 4, "seam vertices 1,2,4,5 should have been dropped as collinear");
    }

    #[semio_framework_async_macros::async_test]
    async fn tessellate_concave_face_tags_all_triangles_with_one_face_id() {
        let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
        let positions: Vec<[f32; 3]> = corners.iter().map(|&(x, y)| [x, y, 0.0]).collect();
        let mesh = HalfedgeMesh::from_faces(&positions, &[(0..6).collect()]).await.unwrap();
        let transfer = mesh.tessellate().await.unwrap();
        assert_eq!(transfer.face_ids.len(), corners.len() - 2);
        assert!(transfer.face_ids.iter().all(|&id| id == 0));
        assert_eq!(transfer.indices.len(), (corners.len() - 2) * 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn weld_coincident_vertices_unifies_independently_tessellated_seam() {
        // Two quads sharing an edge but built with DUPLICATE (non-shared) vertices at that seam, as an
        // importer would produce when tessellating adjacent source faces independently.
        let positions = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0], // seam vertex, quad A's copy
            [1.0, 1.0, 0.0], // seam vertex, quad A's copy
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0], // seam vertex, quad B's copy (duplicate position, different id)
            [1.0, 1.0, 0.0], // seam vertex, quad B's copy (duplicate position, different id)
            [2.0, 0.0, 0.0],
            [2.0, 1.0, 0.0],
        ];
        let faces = vec![vec![0, 1, 2, 3], vec![4, 6, 7, 5]];
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).await.unwrap();
        assert_eq!(mesh.vertex_count().await, 8);
        let removed = mesh.weld_coincident_vertices(1e-4).await.unwrap();
        assert_eq!(removed, 2);
        assert_eq!(mesh.vertex_count().await, 6);
        let merges = mesh.merge_coplanar_faces().await.unwrap();
        assert_eq!(merges, 1, "welding should have made the seam mergeable");
        assert_eq!(mesh.face_count().await, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_holes_caps_a_missing_box_face() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        // Remove one face by rebuilding without it.
        let (positions, mut face_list) = mesh.polygon_soup().await;
        face_list.remove(0);
        assert_eq!(face_list.len(), 5);
        mesh = HalfedgeMesh::from_faces(&positions, &face_list).await.unwrap();
        let filled = mesh.fill_holes().await.unwrap();
        assert_eq!(filled, 1);
        assert_eq!(mesh.face_count().await, 6);
        // Verify the resulting mesh is watertight: every directed boundary edge (position-keyed) now has
        // an opposite-winding counterpart.
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count().await {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        for &(a, b) in directed.keys() {
            assert!(directed.contains_key(&(b, a)), "edge {a}->{b} has no opposite counterpart after fill_holes");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_holes_disambiguates_two_holes_sharing_one_vertex() {
        // 3x3 grid of vertices forming a 2x2 grid of quads; keep only the two DIAGONAL quads, so the two
        // missing (diagonally opposite) quads are separate holes that touch at exactly the shared center
        // vertex (index 4). A vertex-only "next" map cannot disambiguate this; proper halfedge rotation can.
        let mut positions = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                positions.push([i as f32, j as f32, 0.0]);
            }
        }
        let idx = |i: usize, j: usize| (i * 3 + j) as u32;
        let q00 = vec![idx(0, 0), idx(1, 0), idx(1, 1), idx(0, 1)];
        let q11 = vec![idx(1, 1), idx(2, 1), idx(2, 2), idx(1, 2)];
        let mut mesh = HalfedgeMesh::from_faces(&positions, &[q00, q11]).await.unwrap();
        assert_eq!(mesh.face_count().await, 2);
        let filled = mesh.fill_holes().await.unwrap();
        assert_eq!(filled, 2, "expected both diagonal holes to be found and capped separately");
        assert_eq!(mesh.face_count().await, 4);
        for fi in 0..mesh.face_count().await {
            assert_eq!(mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap().len(), 4);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn vec3_dot_cross_length_lerp() {
        let a = Vec3::new(1.0, 0.0, 0.0).await;
        let b = Vec3::new(0.0, 1.0, 0.0).await;
        assert!(a.dot(b).await.abs() < 1e-6);
        assert!((a.cross(b).await.z().await - 1.0).abs() < 1e-6);
        assert!((Vec3::new(3.0, 4.0, 0.0).await.length().await - 5.0).abs() < 1e-6);
        let mid = a.lerp(b, 0.5).await;
        assert!((mid.x().await - 0.5).abs() < 1e-6 && (mid.y().await - 0.5).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn vec3_normalize_zero_vector_returns_zero() {
        assert_eq!(Vec3::ZERO.normalize().await, Vec3::ZERO);
        let tiny = Vec3::new(1e-9, 0.0, 0.0).await;
        assert_eq!(tiny.normalize().await, Vec3::ZERO);
    }

    #[semio_framework_async_macros::async_test]
    async fn vertex_position_invalid_handle_returns_err() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.vertex_position(VertexId(999)).await, Err(MeshKernelError::InvalidHandle));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_vertex_position_updates_and_rejects_invalid_handle() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.set_vertex_position(VertexId(0), Vec3::new(9.0, 9.0, 9.0).await).await.unwrap();
        assert_eq!(mesh.vertex_position(VertexId(0)).await.unwrap(), Vec3::new(9.0, 9.0, 9.0).await);
        assert_eq!(mesh.set_vertex_position(VertexId(999), Vec3::ZERO).await, Err(MeshKernelError::InvalidHandle));
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_endpoints_returns_ordered_vertices() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let (v0, v1) = mesh.edge_endpoints(EdgeId(0)).await.unwrap();
        assert_ne!(v0, v1);
        assert_eq!(mesh.edge_endpoints(EdgeId(9999)).await, Err(MeshKernelError::InvalidHandle));
    }

    #[semio_framework_async_macros::async_test]
    async fn face_vertex_ids_invalid_handle_returns_err() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.face_vertex_ids(FaceId(999)).await, Err(MeshKernelError::InvalidHandle));
    }

    #[semio_framework_async_macros::async_test]
    async fn flip_faces_rejects_empty_selection_and_invalid_handle() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.flip_faces(&[]).await, Err(MeshKernelError::EmptySelection));
        assert_eq!(mesh.flip_faces(&[FaceId(999)]).await, Err(MeshKernelError::InvalidHandle));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_indexed_triangles_rejects_malformed_lengths() {
        assert!(matches!(HalfedgeMesh::from_indexed_triangles(&[0.0, 0.0], &[0, 1, 2]).await, Err(MeshKernelError::InvalidInput(_))));
        assert!(matches!(HalfedgeMesh::from_indexed_triangles(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], &[0, 1]).await, Err(MeshKernelError::InvalidInput(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_indexed_triangles_by_face_id_falls_back_when_empty() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let indices = vec![0, 1, 2];
        let mesh = HalfedgeMesh::from_indexed_triangles_by_face_id(&positions, &indices, &[]).await.unwrap();
        assert_eq!(mesh.face_count().await, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_indexed_triangles_by_face_id_rejects_length_mismatch() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let indices = vec![0, 1, 2];
        let err = HalfedgeMesh::from_indexed_triangles_by_face_id(&positions, &indices, &[10, 11]).await.unwrap_err();
        assert!(matches!(err, MeshKernelError::InvalidInput(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_faces_rejects_degenerate_and_out_of_range() {
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert!(matches!(HalfedgeMesh::from_faces(&positions, &[vec![0, 1]]).await, Err(MeshKernelError::DegenerateOperation)));
        assert!(matches!(HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 99]]).await, Err(MeshKernelError::InvalidInput(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_face_loops_bridges_hole_and_skips_degenerate_outer() {
        let mut positions = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 4.0, 0.0], [0.0, 4.0, 0.0]];
        positions.extend_from_slice(&[[1.0, 1.0, 0.0], [3.0, 1.0, 0.0], [3.0, 3.0, 0.0], [1.0, 3.0, 0.0]]);
        let outer = vec![0, 1, 2, 3];
        let hole = vec![4, 5, 6, 7];
        let face_loops = vec![(outer, vec![hole]), (vec![0, 1], vec![])];
        let mesh = HalfedgeMesh::from_face_loops(&positions, &face_loops).await.unwrap();
        assert_eq!(mesh.face_count().await, 8, "outer-with-hole bridges into a 10-vertex simple polygon (n-2 triangles); degenerate loop must be skipped");
        for fi in 0..mesh.face_count().await {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).await.unwrap();
            assert_eq!(verts.len(), 3);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_prim_has_expected_topology() {
        let mesh = HalfedgeMesh::cylinder_prim(1.0, 2.0, 8).await.unwrap();
        assert_eq!(mesh.face_count().await, 8 * 3);
        assert_eq!(mesh.vertex_count().await, 8 * 2 + 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn cone_prim_has_expected_topology() {
        let mesh = HalfedgeMesh::cone_prim(1.0, 2.0, 6).await.unwrap();
        assert_eq!(mesh.face_count().await, 6 * 2);
        assert_eq!(mesh.vertex_count().await, 6 + 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn rotate_mesh_rotates_vertices_about_axis() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        let before = mesh.vertex_position(VertexId(0)).await.unwrap();
        mesh.rotate(Vec3::new(0.0, 0.0, 1.0).await, std::f32::consts::FRAC_PI_2).await.unwrap();
        let after = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((before.x().await - after.y().await).abs() < 1e-4);
    }

    #[semio_framework_async_macros::async_test]
    async fn scale_mesh_scales_vertices() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        mesh.scale(Vec3::new(2.0, 1.0, 1.0).await).await.unwrap();
        let p = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((p.x().await - (-2.0)).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_vertices_rejects_empty_and_moves_selected() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.move_vertices(&[], Vec3::new(1.0, 0.0, 0.0).await).await, Err(MeshKernelError::EmptySelection));
        let before = mesh.vertex_position(VertexId(0)).await.unwrap();
        mesh.move_vertices(&[VertexId(0)], Vec3::new(1.0, 0.0, 0.0).await).await.unwrap();
        let after = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((after.x().await - before.x().await - 1.0).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn rotate_vertices_rejects_empty_and_rotates_around_pivot() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        assert_eq!(mesh.rotate_vertices(&[], Vec3::new(0.0, 0.0, 1.0).await, 1.0, Vec3::ZERO).await, Err(MeshKernelError::EmptySelection));
        // Vertex 0 starts at (-1,-1,-1); rotating 90° about Z around the origin maps (x,y) -> (-y,x).
        mesh.rotate_vertices(&[VertexId(0)], Vec3::new(0.0, 0.0, 1.0).await, std::f32::consts::FRAC_PI_2, Vec3::ZERO).await.unwrap();
        let p = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((p.x().await - 1.0).abs() < 1e-4, "got x={}", p.x().await);
        assert!((p.y().await - (-1.0)).abs() < 1e-4, "got y={}", p.y().await);
        assert!((p.z().await - (-1.0)).abs() < 1e-4, "got z={}", p.z().await);
    }

    #[semio_framework_async_macros::async_test]
    async fn scale_vertices_rejects_empty_and_scales_around_pivot() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        assert_eq!(mesh.scale_vertices(&[], Vec3::new(2.0, 2.0, 2.0).await, Vec3::ZERO).await, Err(MeshKernelError::EmptySelection));
        let before = mesh.vertex_position(VertexId(0)).await.unwrap();
        mesh.scale_vertices(&[VertexId(0)], Vec3::new(2.0, 1.0, 1.0).await, Vec3::ZERO).await.unwrap();
        let after = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((after.x().await - before.x().await * 2.0).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_vertices_proportional_rejects_empty_and_applies_falloff() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        assert_eq!(mesh.move_vertices_proportional(&[], Vec3::new(1.0, 0.0, 0.0).await, Vec3::ZERO, 1.0).await, Err(MeshKernelError::EmptySelection));
        let all: Vec<VertexId> = (0..mesh.vertex_count().await as u32).map(VertexId).collect();
        let before = mesh.vertex_position(VertexId(0)).await.unwrap();
        mesh.move_vertices_proportional(&all, Vec3::new(1.0, 0.0, 0.0).await, before, 0.001).await.unwrap();
        let moved = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((moved.x().await - before.x().await - 1.0).abs() < 1e-4, "vertex at the pivot itself should get full falloff");
    }

    #[semio_framework_async_macros::async_test]
    async fn snap_vertices_to_grid_rejects_non_positive_and_snaps() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.snap_vertices_to_grid(&[VertexId(0)], 0.0).await, Err(MeshKernelError::InvalidInput("grid must be positive".into())));
        mesh.set_vertex_position(VertexId(0), Vec3::new(0.44, 0.0, 0.0).await).await.unwrap();
        mesh.snap_vertices_to_grid(&[VertexId(0)], 0.5).await.unwrap();
        let p = mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((p.x().await - 0.5).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn inset_faces_rejects_empty_and_adds_inner_face() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.inset_faces(&[], 0.1).await, Err(MeshKernelError::EmptySelection));
        let before = mesh.face_count().await;
        mesh.inset_faces(&[FaceId(0)], 0.1).await.unwrap();
        assert!(mesh.face_count().await > before);
    }

    #[semio_framework_async_macros::async_test]
    async fn bevel_edges_rejects_empty_and_runs_on_selection() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.bevel_edges(&[], 0.1, 1).await, Err(MeshKernelError::EmptySelection));
        let before_verts = mesh.vertex_count().await;
        mesh.bevel_edges(&[EdgeId(0)], 0.1, 1).await.unwrap();
        assert_eq!(mesh.vertex_count().await, before_verts + 2, "bevel_edges appends two offset points per edge");
    }

    #[semio_framework_async_macros::async_test]
    async fn loop_cut_rejects_zero_cuts_and_adds_rings() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.loop_cut(&[], 0).await, Err(MeshKernelError::InvalidInput("cuts must be > 0".into())));
        let before = mesh.face_count().await;
        mesh.loop_cut(&[], 1).await.unwrap();
        assert!(mesh.face_count().await > before);
    }

    #[semio_framework_async_macros::async_test]
    async fn knife_cut_on_quad_face_adds_split_triangles() {
        // Quad lies in the XZ plane (y=0); the cut plane (x from cut_dir, z=1 from cut_a/cut_b) crosses
        // both z-varying edges of the quad transversally, so knife_cut must find two hits and add faces.
        let positions = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 0.0, 2.0], [0.0, 0.0, 2.0]];
        let mut mesh = HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 2, 3]]).await.unwrap();
        let before = mesh.face_count().await;
        mesh.knife_cut(FaceId(0), Vec3::new(0.0, -1.0, 1.0).await, Vec3::new(1.0, -1.0, 1.0).await).await.unwrap();
        assert!(mesh.face_count().await > before, "two valid plane hits on the quad must add new split faces");
    }

    #[semio_framework_async_macros::async_test]
    async fn knife_cut_rejects_invalid_face_handle() {
        let mut mesh = HalfedgeMesh::empty().await;
        assert_eq!(mesh.knife_cut(FaceId(0), Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0).await).await, Err(MeshKernelError::InvalidHandle));
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_vertices_rejects_too_few_and_merges_first_and_center_modes() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        assert_eq!(mesh.merge_vertices(&[VertexId(0)], WeldMode::First, 0.0).await, Err(MeshKernelError::EmptySelection));

        let mut first_mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        let p0 = first_mesh.vertex_position(VertexId(0)).await.unwrap();
        first_mesh.merge_vertices(&[VertexId(0), VertexId(1)], WeldMode::First, 0.0).await.unwrap();
        assert_eq!(first_mesh.vertex_position(VertexId(0)).await.unwrap(), p0);

        let mut center_mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        let a = center_mesh.vertex_position(VertexId(0)).await.unwrap();
        let b = center_mesh.vertex_position(VertexId(1)).await.unwrap();
        center_mesh.merge_vertices(&[VertexId(0), VertexId(1)], WeldMode::Center, 0.0).await.unwrap();
        let merged = center_mesh.vertex_position(VertexId(0)).await.unwrap();
        assert!((merged.x().await - a.lerp(b, 0.5).await.x().await).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_vertices_by_distance_only_merges_within_threshold() {
        let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).await.unwrap();
        let before_verts = mesh.vertex_count().await;
        mesh.merge_vertices(&[VertexId(0), VertexId(1)], WeldMode::ByDistance, 0.01).await.unwrap();
        assert_eq!(mesh.vertex_count().await, before_verts, "vertices farther apart than threshold must not be remapped");
    }

    #[semio_framework_async_macros::async_test]
    async fn dissolve_vertices_rejects_empty_and_removes_incident_faces() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.dissolve_vertices(&[]).await, Err(MeshKernelError::EmptySelection));
        let before = mesh.face_count().await;
        mesh.dissolve_vertices(&[VertexId(0)]).await.unwrap();
        assert!(mesh.face_count().await < before);
    }

    #[semio_framework_async_macros::async_test]
    async fn subdivide_faces_rejects_empty_and_quadruples_selected_face() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.subdivide_faces(&[]).await, Err(MeshKernelError::EmptySelection));
        let before = mesh.face_count().await;
        mesh.subdivide_faces(&[FaceId(0)]).await.unwrap();
        assert_eq!(mesh.face_count().await, before - 1 + 8, "a quad face fans 4 edge-midpoint pairs to the centroid into 8 triangles");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_shading_rejects_invalid_handle_and_marks_smooth() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        assert_eq!(mesh.set_shading(&[FaceId(999)], true).await, Err(MeshKernelError::InvalidHandle));
        mesh.set_shading(&[FaceId(0)], true).await.unwrap();
        mesh.recompute_normals().await.unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn mirror_doubles_geometry_and_welds_seam() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let before_faces = mesh.face_count().await;
        mesh.mirror(MirrorAxis::X, 1e-4).await.unwrap();
        assert_eq!(mesh.face_count().await, before_faces * 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn mark_uv_seam_toggles_and_is_uv_seam_reports_state() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let mut mesh = mesh;
        assert!(!mesh.is_uv_seam(EdgeId(0)).await);
        mesh.mark_uv_seam(&[EdgeId(0)], true).await;
        assert!(mesh.is_uv_seam(EdgeId(0)).await);
        mesh.mark_uv_seam(&[EdgeId(0)], false).await;
        assert!(!mesh.is_uv_seam(EdgeId(0)).await);
    }

    #[semio_framework_async_macros::async_test]
    async fn unwrap_uv_splits_islands_across_seam() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.mark_uv_seam(&[EdgeId(0), EdgeId(2), EdgeId(4), EdgeId(6), EdgeId(8), EdgeId(10)], true).await;
        mesh.unwrap_uv().await.unwrap();
        let transfer = mesh.tessellate().await.unwrap();
        assert!(!transfer.uvs.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn decimate_no_op_when_ratio_at_max_and_clamps_below_min() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        let before = mesh.vertex_count().await;
        mesh.decimate(1.0).await.unwrap();
        assert_eq!(mesh.vertex_count().await, before);

        let mut sphere = HalfedgeMesh::ico_sphere_prim(1.0, 2).await.unwrap();
        let before_sphere = sphere.vertex_count().await;
        sphere.decimate(0.0).await.unwrap();
        assert!(sphere.vertex_count().await < before_sphere, "ratio below 0.1 must clamp to 0.1, not become a no-op");
    }

    #[semio_framework_async_macros::async_test]
    async fn decimate_converges_near_target_ratio_without_emptying_mesh() {
        let mut mesh = HalfedgeMesh::ico_sphere_prim(1.0, 2).await.unwrap();
        let before = mesh.vertex_count().await;
        mesh.decimate(0.5).await.unwrap();
        let target = ((before as f32) * 0.5).ceil() as usize;
        assert!(mesh.vertex_count().await <= target + 1, "decimate must converge on roughly the requested vertex count, not merge forever");
        assert!(mesh.face_count().await > 0, "a 50% decimation must not leave zero faces");
    }

    #[semio_framework_async_macros::async_test]
    async fn to_obj_includes_uv_coordinates_when_present() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).await.unwrap();
        mesh.unwrap_uv().await.unwrap();
        let obj = mesh.to_obj().await.unwrap();
        assert!(obj.contains("vt "));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_json_rejects_invalid_input() {
        let err = HalfedgeMesh::from_json("not json").await.unwrap_err();
        assert!(matches!(err, MeshKernelError::InvalidInput(_)));
    }
}

//#endregion Tests
