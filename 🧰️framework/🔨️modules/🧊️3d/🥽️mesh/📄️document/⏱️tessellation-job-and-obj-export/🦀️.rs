//! ⏱️ Tessellation job and OBJ export: the mesh tessellation job with its steps and retirement, the halfedge mesh tessellation entry points and the OBJ export. See [`super`].

use super::*;

//#region Export
/// ⏱️ Retained corner, ear-test and preview emission cursor; each unit visits at most one corner.
pub struct MeshTessellationJob {
    pub(super) mesh: HalfedgeMesh,
    pub(super) output: Option<MeshTransfer>,
    pub(super) face: usize,
    pub(super) phase: u8,
    pub(super) cursor: usize,
    pub(super) start: u32,
    pub(super) next_he: u32,
    pub(super) hes: Vec<u32>,
    pub(super) points: Vec<Vec3>,
    pub(super) normal_sum: [f64; 3],
    pub(super) normal: Vec3,
    basis: (Vec3f64, Vec3f64),
    pub(super) projected: Vec<(f64, f64)>,
    pub(super) scale: f64,
    pub(super) area: f64,
    pub(super) links: Vec<(usize, usize)>,
    pub(super) head: usize,
    pub(super) remaining: usize,
    pub(super) ear: usize,
    pub(super) probe: usize,
    pub(super) candidates: usize,
    pub(super) base: u32,
    pub(super) edge_seen: HistoryFoldSet<(u32, u32)>,
    pub(super) done: usize,
    pub(super) cancelled: bool,
    pub(super) maximum_preview_bytes: usize,
    pub(super) selected_faces: Option<HistoryFoldSet<u32>>,
    pub(super) corner_ids:Vec<u32>,
    attribute_name:Option<String>,
    pub(super) attribute_indices:Vec<u32>,
    pub(super) attribute_cursor:usize,
    pub(super) metadata_done:bool,
    pub(super) buffer_reservation:u8,
    pub(super) buffer_receipt:tessellation_buffer_reservation::MeshBufferReceipt,
    pub(super) metadata_cursor:usize,
    pub(super) semantic_slots:[Option<usize>;3],
    pub(super) metadata_receipt:tessellation_metadata_capture::MeshMetadataReceipt,
    pub(super) corner_receipt:tessellation_corner_emission::MeshCornerReceipt,
    pub(super) edge_pending:Option<usize>,
    pub(super) edge_receipt:tessellation_edge_emission::MeshEdgeReceipt,
    pub(super) triangle_receipt:tessellation_triangle_emission::MeshTriangleReceipt,
    pub(super) attribute_slot:usize,
    pub(super) attribute_stage:u8,
    pub(super) attribute_pending:Option<(String,MeshAttribute)>,
    pub(super) attribute_original_indices:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Option<Vec<u32>>>>,
    pub(super) attribute_receipt:tessellation_attribute_projection::MeshAttributeReceipt,
    pub(super) face_receipt:tessellation_face_preparation::MeshFaceReceipt,
    pub(super) triangle_pending:Option<tessellation_normal_work::TriangleFrame>,
    pub(super) normal_metadata_phase:u8,
    pub(super) normal_output_bytes:usize,
    pub(super) normal_receipt:tessellation_normal_work::MeshNormalReceipt,
}

/// 🧵️ Tessellation yields retain ownership; completed buffers transfer once.
pub enum MeshTessellationStep { Working(MeshModelingProgress), Done(MeshTransfer), Cancelled(MeshModelingProgress) }

impl MeshTessellationJob {
    pub fn new(mesh: HalfedgeMesh) -> Self {
        Self { mesh, output: Some(MeshTransfer { colors:Vec::new(),attributes:Default::default(),materials:Default::default(),textures:Default::default(),positions: Vec::new(), normals: Vec::new(), indices: Vec::new(), edge_positions: Vec::new(), face_ids: Vec::new(), vertex_ids: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), edge_uvs: Vec::new(), edge_is_seam: Vec::new() }), face: 0, phase: 0, cursor: 0, start: 0, next_he: 0, hes: Vec::new(), points: Vec::new(), normal_sum: [0.0; 3], normal: Vec3::ZERO, basis: ((0.0,0.0,0.0),(0.0,0.0,0.0)), projected: Vec::new(), scale: 0.0, area: 0.0, links: Vec::new(), head: 0, remaining: 0, ear: 0, probe: 0, candidates: 0, base: 0, edge_seen: HistoryFoldSet::new(), done: 0, cancelled: false, maximum_preview_bytes: usize::MAX, selected_faces: None,corner_ids:Vec::new(),attribute_name:None,attribute_indices:Vec::new(),attribute_cursor:0,metadata_done:false,buffer_reservation:0,buffer_receipt:Default::default(),metadata_cursor:0,semantic_slots:[None;3],metadata_receipt:Default::default(),corner_receipt:Default::default(),edge_pending:None,edge_receipt:Default::default(),triangle_receipt:Default::default(),attribute_slot:0,attribute_stage:0,attribute_pending:None,attribute_original_indices:None,attribute_receipt:Default::default(),face_receipt:Default::default(),triangle_pending:None,normal_metadata_phase:0,normal_output_bytes:0,normal_receipt:Default::default() }
    }
    pub fn with_preview_capacity(mesh: HalfedgeMesh, maximum_preview_bytes: usize) -> Self { let mut job = Self::new(mesh); job.maximum_preview_bytes = maximum_preview_bytes; job }
    pub fn selected(mesh: HalfedgeMesh, faces: HistoryFoldSet<u32>) -> Self { let mut job = Self::new(mesh); job.selected_faces = Some(faces); job }
    /// 🧹️ Transfers the existing tessellation payload to its shared typed retirement authority.
    pub fn retirement_birth_bytes(&self)->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<Self>()}
    pub fn into_retirement(self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,Self)> {
        semio_framework_value::retirement::admit_owned_retirement(self,grant)
    }
    pub fn source(&self) -> &HalfedgeMesh { &self.mesh }
    pub fn take_source(&mut self)->HalfedgeMesh { std::mem::replace(&mut self.mesh,HalfedgeMesh::empty()) }
    pub fn progress(&self) -> MeshModelingProgress { MeshModelingProgress { units_done: self.done, units_total: self.done.saturating_add(self.mesh.face_count().saturating_sub(self.face)), phase: if self.face==self.mesh.face_count() {"tessellate-attributes"} else {"tessellate"} } }
    pub fn cancel(&mut self) { self.cancelled = true; }
    pub(super) fn halfedge(&self, local: usize) -> u32 { self.hes[if self.mesh.faces[self.face].flipped { self.hes.len() - 1 - local } else { local }] }
    fn corner_uv(&self,id:u32)->[f64;2] {
        let edge=&self.mesh.halfedges[id as usize];
        self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv).and_then(|attribute|attribute.value_at(match attribute.domain {MeshAttributeDomain::Corner=>id as usize,MeshAttributeDomain::Face=>self.face,MeshAttributeDomain::Vertex=>edge.vertex as usize,MeshAttributeDomain::Edge=>id as usize})).and_then(semio_framework_value::DslValue::as_array).map_or(edge.uv.map(f64::from),|value|[0,1].map(|axis|value[axis].as_f64().unwrap()))
    }
    pub(super) fn corner(&mut self, local: usize) {
        let data=self.corner_data(local,[self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Normal),self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv),self.mesh.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Color)]);
        self.append_corner_data(data);
    }
    pub(super) fn triangle(&mut self, triangle: [usize; 3]) {
        let indices = if self.mesh.faces[self.face].smooth { triangle.map(|local| self.base + local as u32) } else {
            let base = self.output.as_ref().unwrap().positions.len() as u32 / 3;
            for local in triangle { self.corner(local); }
            [base, base + 1, base + 2]
        };
        self.append_triangle_indices(indices);
    }
    pub(super) fn advance(&mut self) -> MeshResult<()> {
        let n = self.hes.len();
        match self.phase {
            0 => {
                if self.selected_faces.as_ref().is_some_and(|faces| !faces.contains(&(self.face as u32))) { self.face += 1; return Ok(()); }
                self.start = self.mesh.faces[self.face].halfedge; self.next_he = self.start; self.cursor = 0; self.phase = 1;
            }
            1 => {
                self.hes.push(self.next_he); self.next_he = self.mesh.halfedges[self.next_he as usize].next;
                if self.hes.len() > self.mesh.halfedges.len() { return Err(MeshKernelError::InvalidHandle); }
                if self.next_he == self.start { self.phase = 2; self.cursor = 0; }
            }
            2 => {
                self.points.push(Vec3(self.mesh.vertices[self.mesh.halfedges[self.halfedge(self.cursor) as usize].vertex as usize].position));
                self.cursor += 1; if self.cursor == n { self.phase = 3; self.cursor = 0; }
            }
            3 => {
                let origin = self.points[0].0.map(f64::from);
                let a = self.points[self.cursor].0.map(f64::from); let b = self.points[(self.cursor + 1) % n].0.map(f64::from);
                let a = [a[0]-origin[0],a[1]-origin[1],a[2]-origin[2]]; let b = [b[0]-origin[0],b[1]-origin[1],b[2]-origin[2]];
                for (axis, (u,v)) in [(1,2),(2,0),(0,1)].into_iter().enumerate() { self.normal_sum[axis] += (a[u]-b[u])*(a[v]+b[v]); }
                self.cursor += 1;
                if self.cursor == n {
                    let length = self.normal_sum[0].hypot(self.normal_sum[1]).hypot(self.normal_sum[2]);
                    let normal = if length == 0.0 { Vec3::ZERO } else { Vec3(self.normal_sum.map(|value| (value/length) as f32)) };
                    self.basis = plane_basis(normalize3((normal.x() as f64,normal.y() as f64,normal.z() as f64)));
                    self.normal = normal.normalize();
                    self.phase = 4; self.cursor = 0;
                }
            }
            4 => {
                let origin = self.points[0]; let point = self.points[self.cursor]; let local = sub3((point.x() as f64,point.y() as f64,point.z() as f64),(origin.x() as f64,origin.y() as f64,origin.z() as f64));
                let projected = (dot3(local,self.basis.0),dot3(local,self.basis.1)); self.scale = self.scale.max(projected.0.abs()).max(projected.1.abs()); self.projected.push(projected);
                self.links.push(((self.cursor+n-1)%n,(self.cursor+1)%n)); self.cursor += 1;
                if self.cursor == n { self.phase = 5; self.cursor = 0; self.remaining = n; }
            }
            5 => {
                if self.scale != 0.0 { self.projected[self.cursor].0 /= self.scale; self.projected[self.cursor].1 /= self.scale; }
                self.cursor += 1; if self.cursor == n { self.phase = 6; self.cursor = 0; }
            }
            6 => {
                let a = self.projected[self.cursor]; let b = self.projected[(self.cursor+1)%n]; self.area += a.0*b.1-b.0*a.1;
                self.cursor += 1;
                if self.cursor == n { self.phase = 7; self.cursor = 0; self.base = self.output.as_ref().unwrap().positions.len() as u32 / 3; }
            }
            7 => {
                if self.mesh.faces[self.face].smooth { self.corner(self.cursor); self.cursor += 1; }
                if !self.mesh.faces[self.face].smooth || self.cursor == n { self.phase = if n == 3 || self.normal.length() < 1e-8 || self.scale == 0.0 || self.area.abs() < 1e-14 { 10 } else { 8 }; self.ear = self.head; self.cursor = self.links[self.head].1; }
            }
            8..=10 => {if let Some((finish,locals))=self.choose_triangle(){self.triangle(locals);self.finish_triangle(finish,locals);}}
            11 => {
                let he_id = self.hes[self.cursor]; let he = &self.mesh.halfedges[he_id as usize]; let next = &self.mesh.halfedges[he.next as usize];
                let key = (he.vertex.min(next.vertex),he.vertex.max(next.vertex));
                if self.edge_seen.insert(key) {
                    let (a,b)=(self.corner_uv(he_id),self.corner_uv(he.next));
                    let seam=self.mesh.uv_seams.contains(&he_id) || he.twin.is_some_and(|twin|self.mesh.uv_seams.contains(&twin) || a!=self.corner_uv(self.mesh.halfedges[twin as usize].next) || b!=self.corner_uv(twin));
                    let out = self.output.as_mut().unwrap(); out.edge_positions.extend_from_slice(&self.mesh.vertices[he.vertex as usize].position); out.edge_positions.extend_from_slice(&self.mesh.vertices[next.vertex as usize].position); out.edge_ids.push(he_id); out.edge_uvs.extend_from_slice(&a.map(|value|value as f32)); out.edge_uvs.extend_from_slice(&b.map(|value|value as f32)); out.edge_is_seam.push(u8::from(seam));
                }
                self.cursor += 1;
                if self.cursor == n {
                    self.face += 1; self.phase = 0; self.hes.clear(); self.points.clear(); self.projected.clear(); self.links.clear(); self.normal_sum = [0.0;3]; self.scale = 0.0; self.area = 0.0; self.head = 0; self.candidates = 0;
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn advance_attributes(&mut self)->MeshResult<()> {
        if self.selected_faces.is_some() {self.metadata_done=true;return Ok(());}
        let next=if let Some(name)=&self.attribute_name {self.mesh.attributes.range((std::ops::Bound::Included(name.clone()),std::ops::Bound::Unbounded)).next()}else {self.mesh.attributes.first_key_value()};
        let Some((name,attribute))=next else {let out=self.output.as_mut().unwrap();out.materials=std::mem::take(&mut self.mesh.materials);out.textures=std::mem::take(&mut self.mesh.textures);self.metadata_done=true;return Ok(());};
        if self.attribute_name.is_none() {self.attribute_name=Some(name.clone());}
        let out=self.output.as_ref().unwrap();
        let (domain,ids)=match attribute.domain {MeshAttributeDomain::Vertex=>(MeshAttributeDomain::Vertex,&out.vertex_ids),MeshAttributeDomain::Corner=>(MeshAttributeDomain::Vertex,&self.corner_ids),MeshAttributeDomain::Face=>(MeshAttributeDomain::Face,&out.face_ids),MeshAttributeDomain::Edge=>(MeshAttributeDomain::Edge,&out.edge_ids)};
        if self.attribute_cursor<ids.len() {let id=ids[self.attribute_cursor] as usize;let sample=attribute.indices.as_ref().map_or(id as u32,|indices|indices[id]);self.attribute_indices.push(sample);self.attribute_cursor+=1;}
        else {let name=self.attribute_name.take().unwrap();let mut attribute=self.mesh.attributes.remove(&name).unwrap();attribute.domain=domain;attribute.indices=Some(std::mem::take(&mut self.attribute_indices));self.output.as_mut().unwrap().attributes.insert(name,attribute);self.attribute_cursor=0;}
        Ok(())
    }
    pub(super) fn reject_ear(&mut self) {
        self.candidates += 1; self.ear = self.links[self.ear].1;
        self.phase = if self.candidates == self.remaining { self.cursor = self.links[self.head].1; 10 } else { 8 };
    }

}

impl HalfedgeMesh {
    /// 🪢️ One boundary link for retained traversal without collecting a polygon.
    pub fn face_boundary(&self, face: FaceId) -> MeshResult<(EdgeId, bool)> { let face = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?; Ok((EdgeId(face.halfedge), face.flipped)) }
    /// 🔗️ Constant-time corner identity and successor for a boundary cursor.
    pub fn boundary_corner(&self, edge: EdgeId) -> MeshResult<(VertexId, EdgeId)> { let edge = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?; Ok((VertexId(edge.vertex), EdgeId(edge.next))) }
    /// 🥽️ The indexed polygon source of this mesh with authored channels re-keyed to canonical corner and edge identities; reconstructing it with `polygon_source_job` yields an equal mesh.
    pub fn polygon_source(&self) -> MeshResult<semio_framework_mesh_engine::PolygonMeshSource> {
        let mut corner_ids = Vec::new();
        let mut edge_ids = Vec::new();
        for face in 0..self.face_count() {
            let (start, flipped) = self.face_boundary(FaceId(face as u32))?;
            let mut halfedges = Vec::new();
            let mut next = start;
            loop {
                halfedges.push(next.0);
                next = self.boundary_corner(next)?.1;
                if next == start {
                    break;
                }
            }
            let count = halfedges.len();
            for cursor in 0..count {
                let index = if flipped { count - 1 - cursor } else { cursor };
                corner_ids.push(halfedges[index]);
                edge_ids.push(halfedges[if flipped { (index + count - 1) % count } else { index }]);
            }
        }
        let mut attributes = self.attributes.clone();
        for attribute in attributes.slot_values_mut() {
            let ids = match attribute.domain {
                MeshAttributeDomain::Corner => Some(&corner_ids),
                MeshAttributeDomain::Edge => Some(&edge_ids),
                _ => None,
            };
            if let Some(ids) = ids {
                if attribute.indices.is_some() || ids.iter().enumerate().any(|(index, id)| index != *id as usize) {
                    attribute.indices = Some(ids.iter().map(|id| attribute.indices.as_ref().map_or(*id, |indices| indices[*id as usize])).collect());
                }
            }
        }
        Ok(semio_framework_mesh_engine::PolygonMeshSource { vertices: self.positions(), faces: self.polygons(), attributes, materials: self.materials.clone(), textures: self.textures.clone() })
    }

    pub fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut job=MeshTessellationJob::new(self.clone());
        loop {let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:128,maximum_capacity_bytes:job.next_normal_capacity_byte_demand(128).map_err(MeshKernelError::Retained)?,maximum_release_bytes:job.next_normal_release_byte_demand().map_err(MeshKernelError::Retained)?,maximum_depth:job.next_normal_depth_demand().map_err(MeshKernelError::Retained)?};match job.step(grant).map_err(MeshKernelError::Retained)?.0 {MeshTessellationStep::Done(transfer)=>return Ok(transfer),MeshTessellationStep::Working(_)=>{},MeshTessellationStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh tessellation cancelled".into()))}}
    }

    /// 🧵 Reads one authored UV sample through its canonical corner identity.
    pub fn corner_uv(&self,corner:EdgeId)->MeshResult<[f32;2]> {let edge=self.halfedges.get(corner.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;if let Some(attribute)=self.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv) {let id=match attribute.domain {MeshAttributeDomain::Vertex=>edge.vertex,MeshAttributeDomain::Face=>edge.face.ok_or(MeshKernelError::InvalidHandle)?,_=>corner.0};let value=attribute.value_at(id as usize).and_then(semio_framework_value::DslValue::as_array).ok_or(MeshKernelError::InvalidHandle)?;return Ok([value[0].as_f64().unwrap()as f32,value[1].as_f64().unwrap()as f32]);}Ok(edge.uv)}

    pub fn to_obj(&self) -> MeshResult<String> {
        let mut export = MeshObjExport::new();
        loop {
            if let Some(text) = export.step(self, 1)? {
                return Ok(text);
            }
        }
    }

    pub fn to_json(&self) -> MeshResult<String> {
        Ok(semio_framework_pack_json::to_json_string(self))
    }

    pub fn from_json(json: &str) -> MeshResult<Self> {
        semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }
}

/// 📜️ Incremental OBJ writer: vertices, texture coordinates and faces are written in units of 256 elements so a host can stay inside an interactive step ceiling.
pub struct MeshObjExport {
    pub(super) out: String,
    pub(super) phase: u8,
    pub(super) cursor: usize,
    has_uv: bool,
    units: usize,
}

impl Default for MeshObjExport {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshObjExport {
    pub fn new() -> Self {
        Self { out: String::from("# kernel_3d_mesh OBJ export\n"), phase: 0, cursor: 0, has_uv: false, units: 0 }
    }

    /// 📊 Completed units and the units the mesh needs in total (an upper bound until the texture scan finished).
    pub fn progress(&self, mesh: &HalfedgeMesh) -> (usize, usize) {
        let total = (mesh.vertices.len() + 2 * mesh.halfedges.len() + mesh.faces.len()).div_ceil(256) + 4;
        (self.units, total.max(self.units))
    }

    /// ⏱️ Writes at most `budget` units of 256 elements; the finished text once everything is written.
    pub fn step(&mut self, mesh: &HalfedgeMesh, budget: usize) -> MeshResult<Option<String>> {
        for _ in 0..budget.max(1) {
            self.units += 1;
            match self.phase {
                0 => {
                    let end = (self.cursor + 256).min(mesh.vertices.len());
                    for vertex in &mesh.vertices[self.cursor..end] {
                        self.out.push_str(&format!("v {} {} {}\n", vertex.position[0], vertex.position[1], vertex.position[2]));
                    }
                    self.cursor = end;
                    if self.cursor == mesh.vertices.len() {
                        self.phase = 1;
                        self.cursor = 0;
                    }
                }
                1 => {
                    let end = (self.cursor + 256).min(mesh.halfedges.len());
                    self.has_uv = mesh.halfedges[self.cursor..end].iter().any(|halfedge| halfedge.uv[0] != 0.0 || halfedge.uv[1] != 0.0);
                    if self.has_uv || end == mesh.halfedges.len() {
                        self.phase = if self.has_uv { 2 } else { 3 };
                        self.cursor = 0;
                    } else {
                        self.cursor = end;
                    }
                }
                2 => {
                    let end = (self.cursor + 256).min(mesh.halfedges.len());
                    for halfedge in &mesh.halfedges[self.cursor..end] {
                        self.out.push_str(&format!("vt {} {}\n", halfedge.uv[0], halfedge.uv[1]));
                    }
                    self.cursor = end;
                    if self.cursor == mesh.halfedges.len() {
                        self.phase = 3;
                        self.cursor = 0;
                    }
                }
                _ => {
                    let end = (self.cursor + 256).min(mesh.faces.len());
                    for face in self.cursor..end {
                        let mut halfedges = mesh.face_halfedge_ids(FaceId(face as u32))?;
                        if mesh.faces[face].flipped {
                            halfedges.reverse();
                        }
                        self.out.push('f');
                        for id in halfedges {
                            let vertex = mesh.halfedges[id as usize].vertex;
                            if self.has_uv {
                                self.out.push_str(&format!(" {}/{}", vertex + 1, id as usize + 1));
                            } else {
                                self.out.push_str(&format!(" {}", vertex + 1));
                            }
                        }
                        self.out.push('\n');
                    }
                    self.cursor = end;
                    if self.cursor == mesh.faces.len() {
                        return Ok(Some(std::mem::take(&mut self.out)));
                    }
                }
            }
        }
        Ok(None)
    }
}
//#endregion Export

impl semio_framework_value::retirement::RetireOwned for MeshTessellationJob {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.mesh),semio_framework_value::retirement::deferred(self.output),semio_framework_value::retirement::deferred(self.hes),semio_framework_value::retirement::deferred(self.points),semio_framework_value::retirement::deferred(self.projected),semio_framework_value::retirement::deferred(self.links),semio_framework_value::retirement::deferred(self.edge_seen),semio_framework_value::retirement::deferred(self.selected_faces),semio_framework_value::retirement::deferred(self.corner_ids),semio_framework_value::retirement::deferred(self.attribute_name),semio_framework_value::retirement::deferred(self.attribute_indices),semio_framework_value::retirement::deferred(self.attribute_pending),semio_framework_value::retirement::deferred(self.attribute_original_indices)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.mesh),semio_framework_value::retirement::deferred_birth_bytes_for(&self.output),semio_framework_value::retirement::deferred_birth_bytes_for(&self.hes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.points),semio_framework_value::retirement::deferred_birth_bytes_for(&self.projected),semio_framework_value::retirement::deferred_birth_bytes_for(&self.links),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_seen),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corner_ids),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_name),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_indices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_pending),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_original_indices)])}
    fn controlled_retirement_supported()->bool {true}
}
