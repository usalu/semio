//! 🕸️ Halfedge mesh operations: the primitives, transforms, edits and UV operations of the halfedge mesh. See [`super`].

use super::*;

//#region HalfedgeMesh
impl HalfedgeMesh {
    /// 🧩️ Transfer all seven kernel owners directly to an individually typed persisted model.
    pub fn into_owned_parts(self)->(Vec<MeshVertex>,Vec<HalfEdge>,Vec<MeshFace>,HistoryFoldSet<u32>,HistoryFoldIndex<String,MeshAttribute>,HistoryFoldIndex<String,semio_framework_value::DslValue>,HistoryFoldIndex<String,MeshTexture>){(self.vertices,self.halfedges,self.faces,self.uv_seams,self.attributes,self.materials,self.textures)}
    /// 🥽️ Editable literal topology enters the kernel without a serialized mesh document.
    pub fn from_owned_parts(vertices:Vec<MeshVertex>,halfedges:Vec<HalfEdge>,faces:Vec<MeshFace>,uv_seams:HistoryFoldSet<u32>,attributes:HistoryFoldIndex<String,MeshAttribute>,materials:HistoryFoldIndex<String,semio_framework_value::DslValue>,textures:HistoryFoldIndex<String,MeshTexture>)->Self{Self{vertices,halfedges,faces,uv_seams,attributes,materials,textures}}
    pub fn empty() -> Self {
        Self { vertices: Vec::new(), halfedges: Vec::new(), faces: Vec::new(), uv_seams: HistoryFoldSet::new(), attributes: HistoryFoldIndex::new(), materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new() }
    }

    /// 🏷️ Authored channels are read from the same mesh value consumed by retained jobs.
    pub fn attributes(&self) -> &HistoryFoldIndex<String,MeshAttribute> { &self.attributes }

    /// 🎨️ Material references and texture bytes belong to this same owned mesh payload.
    pub fn materials(&self)->&HistoryFoldIndex<String,semio_framework_value::DslValue> { &self.materials }
    /// 🖼️ Authored texture records are retained without external resource ownership.
    pub fn textures(&self)->&HistoryFoldIndex<String,MeshTexture> { &self.textures }

    /// 📦️ Admits material and texture records before dependent face channels.
    pub fn set_surface_assets(&mut self,materials:HistoryFoldIndex<String,semio_framework_value::DslValue>,textures:HistoryFoldIndex<String,MeshTexture>)->MeshResult<()> {
        semio_framework_mesh_engine::validate_mesh_surface_assets(&materials,&textures).map_err(MeshKernelError::InvalidInput)?;
        if self.attributes.values().filter(|attribute|attribute.semantic==MeshAttributeSemantic::Material).flat_map(|attribute|&attribute.values).any(|value|value.as_str().is_none_or(|name|!materials.contains_key(name))) { return Err(MeshKernelError::InvalidInput("undefined mesh face material".into())); }
        self.materials=materials;self.textures=textures;
        Ok(())
    }

    /// ✅️ Admits a channel atomically after checking its domain and interpolation shape.
    pub fn set_attribute(&mut self, name: String, attribute: MeshAttribute) -> MeshResult<()> {
        if !self.attributes.contains_key(&name) && self.attributes.len()>=64 {return Err(MeshKernelError::InvalidInput("mesh attribute declaration limit exceeded".into()));}
        semio_framework_mesh_engine::validate_mesh_attribute(&name,&attribute,self.vertex_count(),self.face_count(),self.halfedge_count(),&self.materials).map_err(MeshKernelError::InvalidInput)?;
        self.attributes.insert(name, attribute);
        Ok(())
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn halfedge_count(&self) -> usize {
        self.halfedges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.halfedges.iter().enumerate().filter(|(index, edge)| edge.twin.is_none_or(|twin| *index < twin as usize)).count()
    }

    pub fn vertex_position(&self, id: VertexId) -> MeshResult<Vec3> {
        self.vertices.get(id.0 as usize).map(|v| Vec3(v.position)).ok_or(MeshKernelError::InvalidHandle)
    }

    pub fn set_vertex_position(&mut self, id: VertexId, pos: Vec3) -> MeshResult<()> {
        let v = self.vertices.get_mut(id.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        v.position = pos.0;
        Ok(())
    }

    pub fn face_vertex_ids(&self, face: FaceId) -> MeshResult<Vec<VertexId>> {
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

    pub fn face_normal(&self, face: FaceId) -> MeshResult<Vec3> {
        let verts = self.face_vertex_ids(face)?;
        if verts.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let mut positions: Vec<Vec3> = Vec::with_capacity(verts.len());
        for v in &verts {
            positions.push(self.vertex_position(*v)?);
        }
        Ok(newell_normal(&positions).normalize())
    }

    /// 📍️ Every vertex position in id order.
    pub fn positions(&self) -> Vec<[f32; 3]> {
        self.vertices.iter().map(|vertex| vertex.position).collect()
    }

    /// 🔷️ Every face as its wound vertex-index loop in id order, flipped faces already reversed.
    pub fn polygons(&self) -> Vec<Vec<u32>> {
        (0..self.faces.len() as u32).filter_map(|face| self.face_vertex_ids(FaceId(face)).ok()).map(|loop_ids| loop_ids.into_iter().map(|vertex| vertex.0).collect()).collect()
    }

    /// 🔗️ One canonical id per undirected edge (the lower half-edge of a twin pair), in id order.
    pub fn edge_ids(&self) -> Vec<EdgeId> {
        self.halfedges.iter().enumerate().filter(|(index, edge)| edge.twin.is_none_or(|twin| *index < twin as usize)).map(|(index, _)| EdgeId(index as u32)).collect()
    }

    /// 🧭️ The stored unit normal of one vertex.
    pub fn vertex_normal(&self, id: VertexId) -> MeshResult<Vec3> {
        self.vertices.get(id.0 as usize).ok_or(MeshKernelError::InvalidHandle)?.normal.map(Vec3).ok_or(MeshKernelError::DegenerateOperation)
    }

    pub fn edge_endpoints(&self, edge: EdgeId) -> MeshResult<(VertexId, VertexId)> {
        let he = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let v0 = VertexId(he.vertex);
        let next = &self.halfedges[he.next as usize];
        let v1 = VertexId(next.vertex);
        Ok((v0, v1))
    }

    pub fn face_halfedge_ids(&self, face: FaceId) -> MeshResult<Vec<u32>> {
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

    pub fn from_indexed_triangles(positions: &[f32], indices: &[u32]) -> MeshResult<Self> {
        if !positions.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("positions length must be a multiple of 3".into()));
        }
        if !indices.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("indices length must be a multiple of 3".into()));
        }
        let verts: Vec<[f32; 3]> = positions.as_chunks::<3>().0.to_vec();
        let faces: Vec<Vec<u32>> = indices.as_chunks::<3>().0.iter().map(|tri| tri.to_vec()).collect();
        Self::from_faces(&verts, &faces)
    }

    /// 🔲️ Builds a halfedge mesh from CAD face wire loops that share a global vertex buffer.
    ///
    /// Each entry is `(outer, holes)`. Faces without holes become one n-gon; faces with holes are
    /// triangulated via keyhole bridging so openings stay empty (never filled by a single outer n-gon).
    pub fn from_face_loops(positions: &[[f32; 3]], face_loops: &[(Vec<u32>, Vec<Vec<u32>>)]) -> MeshResult<Self> {
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for (outer, holes) in face_loops {
            if outer.len() < 3 {
                continue;
            }
            if holes.is_empty() {
                faces.push(outer.clone());
                continue;
            }
            for tri in triangulate_indexed_polygon_with_holes(positions, outer, holes) {
                faces.push(tri.to_vec());
            }
        }
        Self::from_faces(positions, &faces)
    }

    pub fn from_faces(positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<Self> {
        let mut mesh = Self::empty();
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
        mesh.recompute_normals()?;
        Ok(mesh)
    }

    pub fn add_vertex(&mut self, pos: [f32; 3]) -> VertexId {
        let id = self.vertices.len() as u32;
        self.vertices.push(MeshVertex { position: pos, normal: None, halfedge: None });
        VertexId(id)
    }

    fn rebuild_from_polygon_soup(&mut self, positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<()> {
        self.require_topology_source_remap("topology reconstruction")?;
        let mut mesh=Self::from_faces(positions,faces)?;mesh.materials=std::mem::take(&mut self.materials);mesh.textures=std::mem::take(&mut self.textures);*self=mesh;
        Ok(())
    }

    pub fn polygon_soup(&self) -> (Vec<[f32; 3]>, Vec<Vec<u32>>) {
        let positions: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.position).collect();
        let mut faces = Vec::new();
        for fi in 0..self.faces.len() {
            if let Ok(verts) = self.face_vertex_ids(FaceId(fi as u32)) {
                faces.push(verts.into_iter().map(|v| v.0).collect());
            }
        }
        (positions, faces)
    }
}
//#endregion HalfedgeMesh

//#region Transform
impl HalfedgeMesh {

    pub fn move_vertices(&mut self, verts: &[VertexId], delta: Vec3) -> MeshResult<()> {
        if delta.0.iter().any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
        self.map_vertices(verts, None, |point| std::array::from_fn(|axis| point[axis] + delta.0[axis] as f64))
    }

    pub fn rotate_vertices(&mut self, verts: &[VertexId], axis: Vec3, angle_rad: f32, pivot: Vec3) -> MeshResult<()> {
        if axis.0.iter().chain(pivot.0.iter()).any(|value| !value.is_finite()) || !angle_rad.is_finite() { return Err(MeshKernelError::DegenerateOperation); }
        let axis = axis.0.map(f64::from);
        let length = axis[0].hypot(axis[1]).hypot(axis[2]);
        if length == 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let [x, y, z] = axis.map(|value| value / length);
        let pivot = pivot.0.map(f64::from);
        let c = (angle_rad as f64).cos();
        let s = (angle_rad as f64).sin();
        let t = 1.0 - c;
        self.map_vertices(verts, Some([[t*x*x+c,t*x*y-s*z,t*x*z+s*y],[t*x*y+s*z,t*y*y+c,t*y*z-s*x],[t*x*z-s*y,t*y*z+s*x,t*z*z+c]]), |point| {
            let [px, py, pz] = std::array::from_fn::<_, 3, _>(|axis| point[axis] - pivot[axis]);
            let rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz;
            let ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz;
            let rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz;
            [pivot[0] + rx, pivot[1] + ry, pivot[2] + rz]
        })
    }

    pub fn scale_vertices(&mut self, verts: &[VertexId], factor: Vec3, pivot: Vec3) -> MeshResult<()> {
        if factor.0.iter().chain(pivot.0.iter()).any(|value| !value.is_finite()) || factor.0.contains(&0.0) { return Err(MeshKernelError::DegenerateOperation); }
        self.map_vertices(verts, Some([[1.0/factor.x() as f64,0.0,0.0],[0.0,1.0/factor.y() as f64,0.0],[0.0,0.0,1.0/factor.z() as f64]]), |point| std::array::from_fn(|axis| pivot.0[axis] as f64 + (point[axis] - pivot.0[axis] as f64) * factor.0[axis] as f64))
    }

    fn map_vertices(&mut self, verts: &[VertexId], normal_matrix:Option<[[f64;3];3]>, transform: impl Fn([f64; 3]) -> [f64; 3]) -> MeshResult<()> {
        if verts.is_empty() { return Err(MeshKernelError::EmptySelection); }
        let mut ids = verts.iter().map(|vertex| vertex.0 as usize).collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        let positions = ids.iter().map(|&id| {
            let point = self.vertices.get(id).ok_or(MeshKernelError::InvalidHandle)?.position;
            let result = transform(point.map(f64::from)).map(|value| value as f32);
            if result.iter().any(|value| !value.is_finite()) { return Err(MeshKernelError::DegenerateOperation); }
            Ok(result)
        }).collect::<MeshResult<Vec<_>>>()?;
        let mut normal_channels=std::collections::BTreeMap::new();
        if let Some(matrix)=normal_matrix {
            for (name,attribute) in &self.attributes {
                if attribute.semantic!=MeshAttributeSemantic::Normal {continue;}
                let mut values=Vec::new();let mut indices=Vec::new();let mut samples=HashMap::new();
                for domain in 0..attribute.domain_len() {
                    let vertex=if attribute.domain==MeshAttributeDomain::Corner {self.halfedges[domain].vertex as usize}else {domain};
                    let affected=ids.binary_search(&vertex).is_ok();let source=attribute.indices.as_ref().map_or(domain as u32,|indices|indices[domain]);
                    let sample=if let Some(sample)=samples.get(&(source,affected)) {*sample}else {
                        let original=attribute.values[source as usize].as_array().unwrap();let mut normal=[0,1,2].map(|axis|original[axis].as_f64().unwrap());
                        if affected {normal=[0,1,2].map(|axis|matrix[axis][0]*normal[0]+matrix[axis][1]*normal[1]+matrix[axis][2]*normal[2]);let length=normal[0].hypot(normal[1]).hypot(normal[2]);normal=normal.map(|value|value/length);if normal.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::DegenerateOperation);}}
                        let sample=values.len() as u32;values.push(semio_framework_value::DslValue::Array(normal.into_iter().map(semio_framework_value::DslValue::float).collect()));samples.insert((source,affected),sample);sample
                    };indices.push(sample);
                }
                normal_channels.insert(name.clone(),MeshAttribute {domain:attribute.domain,semantic:attribute.semantic,interpolation:attribute.interpolation,values,indices:Some(indices)});
            }
        }
        for (id, position) in ids.into_iter().zip(positions) { self.vertices[id].position = position; }
        self.attributes.extend(normal_channels);
        self.recompute_normals()
    }

}
//#endregion Transform

//#region Edit
impl HalfedgeMesh {

    fn require_topology_source_remap(&self,operation:&str)->MeshResult<()> {if let Some((name,_))=self.attributes.first_key_value() {return Err(MeshKernelError::InvalidInput(format!("mesh attribute '{name}' requires declared source remapping for {operation}")));}if !self.uv_seams.is_empty() {return Err(MeshKernelError::InvalidInput(format!("mesh UV seams require declared source remapping for {operation}")));}Ok(())}

    /// 🧹️ Compacts away vertices no longer referenced by any face, so `vertex_count()` reflects the mesh's
    /// actual remaining complexity after operations (like retained decimation) that remap vertex ids away
    /// without themselves shrinking the position buffer.
    fn drop_unreferenced_vertices(&mut self) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup();
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
        self.rebuild_from_polygon_soup(&compacted, &new_faces)
    }

    pub fn set_shading(&mut self, faces: &[FaceId], smooth: bool) -> MeshResult<()> {
        *self = self.set_shading_job(faces, smooth)?.finish_with_progress()?;
        Ok(())
    }

    pub fn recompute_normals(&mut self) -> MeshResult<()> {
        let mut flat = vec![None; self.vertex_count()];
        let mut sums = vec![Vec3::ZERO; self.vertex_count()];
        self.accumulate_vertex_normals(0..self.faces.len(), &mut flat, &mut sums)?;
        self.apply_vertex_normals(0..self.vertices.len(), &flat, &sums);
        Ok(())
    }

}
//#endregion Edit

//#region Uv
impl HalfedgeMesh {
    /// 🪡️ Marks or clears the seams on the edges (an edge and its twin are one seam); handles that name no edge are ignored.
    pub fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool) {
        for &edge in edges {
            if (edge.0 as usize) < self.halfedges.len() {
                self.set_seam_pair(edge, seam);
            }
        }
    }

    pub fn is_uv_seam(&self, edge: EdgeId) -> bool {
        (edge.0 as usize) < self.halfedges.len() && self.is_seam_pair(edge.0)
    }

    /// 🗺️ Unwraps the surface along its seams into packed texture coordinates; see [`Self::unwrap_uv_job`].
    pub fn unwrap_uv(&mut self) -> MeshResult<()> {
        *self = self.unwrap_uv_job()?.finish_with_progress()?;
        Ok(())
    }
}
//#endregion Uv
