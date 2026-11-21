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
        Self { mesh, output: Some(MeshTransfer { colors:Vec::new(),attributes:Default::default(),materials:Default::default(),textures:Default::default(),positions: Vec::new(), normals: Vec::new(), indices: Vec::new(), edge_positions: Vec::new(), face_ids: Vec::new(), vertex_ids: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), edge_uvs: Vec::new(), edge_is_seam: Vec::new() }), face: 0, phase: 0, cursor: 0, start: 0, next_he: 0, hes: Vec::new(), points: Vec::new(), normal_sum: [0.0; 3], normal: Vec3::ZERO, basis: ((0.0,0.0,0.0),(0.0,0.0,0.0)), projected: Vec::new(), scale: 0.0, area: 0.0, links: Vec::new(), head: 0, remaining: 0, ear: 0, probe: 0, candidates: 0, base: 0, edge_seen: HistoryFoldSet::new(), done: 0, cancelled: false, maximum_preview_bytes: usize::MAX, selected_faces: None,corner_ids:Vec::new(),attribute_indices:Vec::new(),attribute_cursor:0,metadata_done:false,buffer_reservation:0,buffer_receipt:Default::default(),metadata_cursor:0,semantic_slots:[None;3],metadata_receipt:Default::default(),corner_receipt:Default::default(),edge_pending:None,edge_receipt:Default::default(),triangle_receipt:Default::default(),attribute_slot:0,attribute_stage:0,attribute_pending:None,attribute_receipt:Default::default(),face_receipt:Default::default(),triangle_pending:None,normal_metadata_phase:0,normal_output_bytes:0,normal_receipt:Default::default() }
    }
    pub fn with_preview_capacity(mesh: HalfedgeMesh, maximum_preview_bytes: usize) -> Self { let mut job = Self::new(mesh); job.maximum_preview_bytes = maximum_preview_bytes; job }
    pub fn selected(mesh: HalfedgeMesh, faces: HistoryFoldSet<u32>) -> Self { let mut job = Self::new(mesh); job.selected_faces = Some(faces); job }
    /// 🧹️ Transfers the existing tessellation payload to its shared typed retirement authority.
    pub fn retirement_birth_bytes(&self)->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<Self>()}
    pub fn into_retirement(self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,Option<Self>)> {
        self.into_original_retirement(effect).result
    }
    /// 🧾️ Preserves the original tessellation admission result and actual work together.
    pub fn into_original_retirement(self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::RetainedCloneProgress),(semio_framework_value::ValueError,Option<Self>)>{
        let mut slot=Some(self);let turn=semio_framework_value::retirement::admit_owned_retirement(&mut slot,effect);
        let result=match turn.result{Ok(Some(admitted))=>Ok(admitted),Ok(None)=>Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"tessellation retirement quote was not admitted"),slot)),Err(error)=>Err((error,slot))};
        semio_framework_value::RetainedWorkOutcome{result,work:turn.work}
    }
    pub fn source(&self) -> &HalfedgeMesh { &self.mesh }
    pub fn take_source(&mut self)->HalfedgeMesh { std::mem::replace(&mut self.mesh,HalfedgeMesh::empty()) }
    pub fn progress(&self) -> MeshModelingProgress { MeshModelingProgress { units_done: self.done, units_total: self.done.saturating_add(self.mesh.face_count().saturating_sub(self.face)), phase: if self.face==self.mesh.face_count() {"tessellate-attributes"} else {"tessellate"} } }
    pub fn cancel(&mut self) { self.cancelled = true; }
    pub(super) fn halfedge(&self, local: usize) -> u32 { self.hes[if self.mesh.faces[self.face].flipped { self.hes.len() - 1 - local } else { local }] }
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
            _ => unreachable!(),
        }
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
        use semio_framework_value::{AuthoringWallet,RetainedCloneGrant};
        let mut capture=MeshPolygonSourceCapture::new();
        let mut wallet=AuthoringWallet::new(RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,
            maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},u128::MAX);
        loop {
            let next=capture.demand(self,256).map_err(MeshKernelError::Retained)?;
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:next.copy_bytes,maximum_capacity_bytes:next.capacity_bytes,
                maximum_release_bytes:next.release_bytes,maximum_depth:next.depth};
            let turn=wallet.run(grant,32,|effect|capture.step(self,effect)).map_err(MeshKernelError::Retained)?
                .ok_or(MeshKernelError::DegenerateOperation)?;
            let (step,receipt)=turn.output.map_err(MeshKernelError::Retained)?;
            wallet.post(receipt).map_err(MeshKernelError::Retained)?;
            if let MeshPolygonSourceStep::Done(source)=step{return Ok(source);}
        }
    }

    /// 🧴️ Reads one optional native surface sample through a canonical topology identity.
    pub fn polygon_surface_sample(&self, kind: semio_framework_mesh_engine::PolygonSurfaceKind, index: u32, source: u32) -> Option<semio_framework_mesh_engine::PolygonSurfaceSample> {
        use semio_framework_mesh_engine::{PolygonSurfaceKind::*, PolygonSurfaceSample};
        let (normal, uv) = match kind {
            SmoothFace if self.faces.get(source as usize)?.smooth => (None, None),
            VertexNormal => (Some(self.vertices.get(source as usize)?.normal?), None),
            CornerUv => {
                let uv = self.halfedges.get(source as usize)?.uv;
                if uv == [0.0; 2] { return None; }
                (None, Some(uv))
            }
            SeamEdge if self.is_uv_seam(EdgeId(source)) => (None, None),
            _ => return None,
        };
        Some(PolygonSurfaceSample { kind, index, normal, uv })
    }

    pub fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut job=MeshTessellationJob::new(self.clone());
        let size=self.halfedges.len().saturating_add(16);let bytes=size.saturating_mul(1<<12);let mut wallet=semio_framework_value::AuthoringWallet::new(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:size.saturating_mul(64),maximum_copy_bytes:bytes,maximum_capacity_bytes:bytes,maximum_release_bytes:bytes,maximum_depth:1<<10},(size as u128).saturating_mul(1<<20));
        loop {let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:128,maximum_capacity_bytes:job.next_normal_capacity_byte_demand(128).map_err(MeshKernelError::Retained)?,maximum_release_bytes:job.next_normal_release_byte_demand().map_err(MeshKernelError::Retained)?,maximum_depth:job.next_normal_depth_demand().map_err(MeshKernelError::Retained)?};let Some(done)=wallet.run(grant,1<<20,|effect|job.step(effect)).map_err(MeshKernelError::Retained)? else {return Err(MeshKernelError::InvalidInput("mesh tessellation quote refused".into()))};let(step,progress)=done.output.map_err(MeshKernelError::Retained)?;wallet.post(progress).map_err(MeshKernelError::Retained)?;match step {MeshTessellationStep::Done(transfer)=>return Ok(transfer),MeshTessellationStep::Working(_)=>{},MeshTessellationStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh tessellation cancelled".into()))}}
    }

    /// 🧵 Reads one authored UV sample through its canonical corner identity.
    pub fn corner_uv(&self,corner:EdgeId)->MeshResult<[f32;2]> {let edge=self.halfedges.get(corner.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;if let Some(attribute)=self.attributes.values().find(|attribute|attribute.semantic==MeshAttributeSemantic::Uv) {let id=match attribute.domain {MeshAttributeDomain::Vertex=>edge.vertex,MeshAttributeDomain::Face=>edge.face.ok_or(MeshKernelError::InvalidHandle)?,_=>corner.0};let value=attribute.value_at(id as usize).and_then(semio_framework_value::DslValue::as_array).ok_or(MeshKernelError::InvalidHandle)?;let (Some(u),Some(v))=(value.first().and_then(semio_framework_value::DslValue::as_f64),value.get(1).and_then(semio_framework_value::DslValue::as_f64)) else {return Err(MeshKernelError::InvalidHandle);};return Ok([u as f32,v as f32]);}Ok(edge.uv)}

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

/// 📜️ Incremental OBJ writer for polygon geometry and authored corner channels.
pub struct MeshObjExport {
    pub(super) out: String,
    pub(super) phase: u8,
    pub(super) cursor: usize,
    channels: MeshObjChannels,
    units: usize,
}

impl Default for MeshObjExport {
    fn default() -> Self { Self::new() }
}

impl MeshObjExport {
    pub fn new() -> Self {
        Self { out: String::from("# kernel_3d_mesh OBJ export\n"), phase: 0, cursor: 0, channels: Default::default(), units: 0 }
    }

    /// 📊 Completed units and an upper bound for this mesh.
    pub fn progress(&self, mesh: &HalfedgeMesh) -> (usize, usize) {
        let total=mesh.attributes().len()+mesh.halfedges.len().max(mesh.vertices.len())+(mesh.vertices.len()+2*mesh.halfedges.len()+mesh.faces.len()).div_ceil(256)+7;
        (self.units,total.max(self.units))
    }

    /// ⏱️ Writes at most `budget` units, returning the finished text when all records are written.
    pub fn step(&mut self, mesh: &HalfedgeMesh, budget: usize) -> MeshResult<Option<String>> {
        use std::fmt::Write;
        for _ in 0..budget.max(1) {
            self.units += 1;
            match self.phase {
                0 => {
                    let end=(self.cursor+256).min(mesh.vertices.len());
                    for vertex in &mesh.vertices[self.cursor..end] {
                        let [x,y,z]=vertex.position;
                        self.channels.scan_vertex_normal(vertex.normal);
                        writeln!(self.out,"v {x} {y} {z}").expect("writing OBJ to a string");
                    }
                    self.cursor=end;
                    if self.cursor==mesh.vertices.len() {self.phase=1;self.cursor=0;}
                }
                1 => {if self.channels.scan_attributes(mesh) {self.phase=2;}}
                2 => {
                    let end=(self.cursor+256).min(mesh.halfedges.len());
                    for corner in self.cursor..end {self.channels.scan_corner(mesh,EdgeId(corner as u32))?;}
                    self.cursor=end;
                    if self.cursor==mesh.halfedges.len() {
                        self.phase=if self.channels.has_uv(){3}else{4};
                        self.cursor=0;
                    }
                }
                3 => {
                    let end=(self.cursor+256).min(mesh.halfedges.len());
                    for corner in self.cursor..end {
                        let [u,v]=self.channels.texture(mesh,EdgeId(corner as u32))?;
                        writeln!(self.out,"vt {u} {v}").expect("writing OBJ to a string");
                    }
                    self.cursor=end;
                    if self.cursor==mesh.halfedges.len() {self.phase=4;self.cursor=0;}
                }
                4 if self.cursor<self.channels.normal_count(mesh) => {
                    let [x,y,z]=self.channels.normal(mesh,self.cursor)?;
                    writeln!(self.out,"vn {x} {y} {z}").expect("writing OBJ to a string");
                    self.cursor+=1;
                }
                4 => {self.phase=5;self.cursor=0;}
                _ => {
                    let end=(self.cursor+256).min(mesh.faces.len());
                    for face in self.cursor..end {
                        let mut halfedges=mesh.face_halfedge_ids(FaceId(face as u32))?;
                        if mesh.faces[face].flipped {halfedges.reverse();}
                        self.out.push('f');
                        for id in halfedges {self.channels.write_corner(mesh,&mut self.out,mesh.halfedges[id as usize].vertex,id)?;}
                        self.out.push('\n');
                    }
                    self.cursor=end;
                    if self.cursor==mesh.faces.len() {return Ok(Some(std::mem::take(&mut self.out)));}
                }
            }
        }
        Ok(None)
    }
}
//#endregion Export

impl semio_framework_value::retirement::RetireOwned for MeshTessellationJob {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(HalfedgeMesh,Option<MeshTransfer>,Vec<u32>,Vec<Vec3>,Vec<(f64, f64)>,Vec<(usize, usize)>,HistoryFoldSet<(u32, u32)>,Option<HistoryFoldSet<u32>>,Vec<u32>,Vec<u32>,Option<(String,MeshAttribute)>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.mesh,self.output,self.hes,self.points,self.projected,self.links,self.edge_seen,self.selected_faces,self.corner_ids,self.attribute_indices,self.attribute_pending)}
    fn controlled_retirement_supported()->bool {true}
}
