//! 📦 STL/OBJ/GLB/mesh mesh import/export bridged to native B-Rep.
//!
//! Triangle soups interchange through `semio_framework_mesh_engine` codecs where available; solids
//! tessellate via [`crate::brep::queries::tessellation`] and import as
//! one planar face per triangle with shared vertices and edges, returning an open shell or a
//! closed solid. `export_mesh`/`import_mesh`/`export_solid_mesh`/`import_mesh_to_body` take their mesh
//! codec as a `MeshExporter`/`MeshImporter` parameter (ticket
//! `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 1): `semio_s_artifact_stdio_dwg` is a SEPARATE
//! artifact, and this kernel-layer file must not import another artifact directly — the caller
//! (`⚙️engine/🦀️.rs`, the contract façade that legitimately bridges artifacts) supplies the real
//! `DwgExporter`/`DwgImporter`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave DEDUP: this file's sole
//! production consumer was already `🧊️brep/🧬️schema/⚙️engine`'s own `use semio_framework_3d::
//! brep::mesh_io::{…}` (the "forward edge" this whole file is a leaf of), and its mesh calls were
//! the last framework-tier caller of the old `semio_framework::mesh_to_dwg_drawing`/`dwg_from_bytes`/
//! `dwg_to_bytes`/`dwg_drawing_to_mesh` re-exports (`🔺️mesh`, deleted this same wave). Moving this
//! file here — rather than repointing it at stdio's real `dwg` artifact from across a framework→
//! plugin edge, which would be a real crate cycle since `stdio → semio-framework-3d` already exists
//! for the algorithm forward-edge above — dissolves that edge instead: the mesh calls become
//! same-crate `semio_s_artifact_stdio_dwg::{…}`, and the framework-3d algorithm imports become the same
//! external `semio_framework_3d::engine::*` forward-edge pattern the parent `engine/component.rs`
//! used at the time. `MeshTransfer` moved again in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME
//! wave 1 (W1-A): the parent's own `engine::contract` module now owns it same-crate.

use crate::brep::operations::euler::{add_shell, add_solid};
use crate::brep::engine::MeshTransfer;
use crate::brep::queries::tessellation::tessellate_solid;
use crate::brep::representation::arena::SolidId;
use crate::brep::representation::error::KernelError;
use crate::brep::representation::tolerance::Tol;
use crate::brep::representation::topology::history::OpRecorder;
use crate::brep::representation::topology::{Body,EntityRef};
use crate::brep::representation::vector::{Pnt3, Vec3};
use semio_framework_mesh_engine::{mesh_from_obj, mesh_from_stl, mesh_to_obj, mesh_to_stl, GlbExporter, GlbImporter, MeshData, MeshExporter, MeshImporter};

// #region 🔖️Types

/// 📦 Indexed triangle soup in kernel world units.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriangleMesh {
    pub positions: Vec<Pnt3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}
/// 📦 Bounded private triangle import and exact owned-entity retirement in the mesh-I/O owner.
pub struct MeshImportCursor {
    pub(super) active: bool,
    positions: Vec<f32>,
    positions64: Vec<Pnt3>,
    normals64: Vec<Vec3>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    phase: u8,
    cursor: usize,
    volume: f64,
    faces: Vec<crate::brep::representation::arena::FaceId>,
    shell: Option<crate::brep::representation::arena::ShellId>,
    solid: Option<SolidId>,
    recorder: OpRecorder,
    cancelled: bool,
    fault: Option<KernelError>,
    units: usize,
    retired: bool,
    retirement: crate::brep::engine::retirement::PayloadRetirement,
    released_buffers: bool,
    triangles: usize,
    provenance_credit: Option<usize>,
    vertex_map: std::collections::BTreeMap<[u64;3],crate::brep::representation::arena::VertexId>,
    edge_map: std::collections::BTreeMap<([u64;3],[u64;3]),(crate::brep::representation::arena::EdgeId,usize,bool)>,
    boundary_edges: usize,
}
struct ImportIndexRetirement {
    vertices: std::collections::BTreeMap<[u64;3],crate::brep::representation::arena::VertexId>,
    edges: std::collections::BTreeMap<([u64;3],[u64;3]),(crate::brep::representation::arena::EdgeId,usize,bool)>,
}
impl crate::brep::engine::retirement::RetirementFrontier for ImportIndexRetirement {
    fn advance(&mut self,_:&mut crate::brep::engine::retirement::PayloadRetirement)->bool {if self.vertices.pop_first().is_none() {self.edges.pop_first();}self.vertices.is_empty() && self.edges.is_empty()}
}
/// 📦 The current result of advancing the existing mesh import mutation.
pub enum MeshImportStep { Working, Cancelled, Done(crate::brep::representation::topology::EntityRef) }
impl MeshImportCursor {
    /// 🚦️ Checks constant-sized admission facts before a retained owner transfers its buffers.
    pub fn validate_admission_counts(position_scalars:usize,index_count:usize,tolerance:f64)->Result<(),KernelError> {
        if !tolerance.is_finite() || tolerance<=0.0 || position_scalars%3!=0 || index_count%3!=0 || index_count<3 || position_scalars>1_800_000 || index_count>300_000 {return Err(KernelError::InvalidInput("invalid or oversized triangle import".into()));}
        Ok(())
    }
    /// 📦 Admits owned typed buffers before creating any body entities.
    pub fn new(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>, tolerance: f64) -> Result<Self,KernelError> {
        Self::validate_admission_counts(positions.len(),indices.len(),tolerance)?;
        let triangles=indices.len()/3;
        Ok(Self { active:false,positions,positions64:Vec::new(),normals64:Vec::new(),normals,indices,phase:0,cursor:0,volume:0.0,faces:Vec::new(),shell:None,solid:None,recorder:OpRecorder::new(),cancelled:false,fault:None,units:0,retired:false,retirement:Default::default(),released_buffers:false,triangles,provenance_credit:None,vertex_map:Default::default(),edge_map:Default::default(),boundary_edges:0 })
    }
    /// 📦 Admits the existing owned f64 triangle representation without narrowing coordinates.
    pub fn from_triangle_mesh(mesh:TriangleMesh,tolerance:f64)->Result<Self,KernelError> {
        if mesh.positions.len()>600_000 {return Err(KernelError::InvalidInput("oversized triangle import".into()));}
        let TriangleMesh {positions,normals,indices}=mesh;let mut cursor=Self::new(Vec::new(),Vec::new(),indices,tolerance)?;cursor.positions64=positions;cursor.normals64=normals;Ok(cursor)
    }
    /// 📈 Returns the exact retained mutation phase and completed bounded units.
    pub fn progress(&self)->(usize,usize,&'static str) {
        let phase=if self.cancelled || self.fault.is_some() { "mesh-to-brep-retire" } else {match self.phase {0=>"mesh-to-brep-admit",1=>"mesh-to-brep-triangles",2=>"mesh-to-brep-shell",3=>"mesh-to-brep-solid",_=>"mesh-to-brep-ready"}};
        (self.units,self.units.max(self.triangles*2+4),phase)
    }
    /// 🛑 Requests rollback without dropping or publishing this private mutation.
    pub fn cancel(&mut self) { self.cancelled=true; }
    /// 🧹️ Reports cancellation only after every owned body entity has been retired.
    pub fn retirement_complete(&self)->bool {self.retired}
    fn points(&self)->Result<[Pnt3;3],KernelError> {
        let mut points=[Pnt3::new(0.0,0.0,0.0);3];
        for (point,index) in points.iter_mut().zip(&self.indices[self.cursor..self.cursor+3]) {
            if self.positions64.is_empty() {
                let start=*index as usize*3;let value=self.positions.get(start..start+3).ok_or_else(||KernelError::InvalidInput("triangle index out of range".into()))?;
                *point=Pnt3::new(value[0]as f64,value[1]as f64,value[2]as f64);
            } else {*point=*self.positions64.get(*index as usize).ok_or_else(||KernelError::InvalidInput("triangle index out of range".into()))?;}
            if !point.x.is_finite() || !point.y.is_finite() || !point.z.is_finite() {return Err(KernelError::InvalidInput("nonfinite triangle vertex".into()));}
        }
        Ok(points)
    }
    /// ⏱️ Advances at most the admitted budget, including cancellation and fault rollback.
    pub fn step(&mut self,body:&mut Body,budget:usize)->Result<MeshImportStep,KernelError> {self.close_step(body,budget,4096)}
    /// 🎟️ Advances the same import cursor with explicit payload-byte credit.
    pub fn close_step(&mut self,body:&mut Body,budget:usize,bytes:usize)->Result<MeshImportStep,KernelError> {
        if bytes==0 {return Ok(MeshImportStep::Working);}
        for _ in 0..budget {
            if self.cancelled || self.fault.is_some() {
                self.release_buffers();let empty=self.recorder.retire_step(body,1,&mut self.retirement);
                if empty {self.retirement.close_step(1,bytes);}
                self.units=self.units.saturating_add(1);
                if empty && self.retirement.terminal_is_empty() { self.retired=true;if let Some(error)=self.fault.take() {return Err(error);} return Ok(MeshImportStep::Cancelled); }
                continue;
            }
            let result=self.advance(body,bytes);
            self.units=self.units.saturating_add(1);
            match result {
                Ok(Some(solid))=>return Ok(MeshImportStep::Done(solid)),
                Ok(None)=>{},
                Err(error)=>self.fault=Some(error),
            }
        }
        Ok(MeshImportStep::Working)
    }
    fn release_buffers(&mut self) {
        if self.released_buffers {return;}self.released_buffers=true;
        self.retirement.pod(std::mem::take(&mut self.positions));self.retirement.pod(std::mem::take(&mut self.positions64));self.retirement.pod(std::mem::take(&mut self.normals64));self.retirement.pod(std::mem::take(&mut self.normals));self.retirement.pod(std::mem::take(&mut self.indices));self.retirement.pod(std::mem::take(&mut self.faces));self.retirement.frontier(ImportIndexRetirement {vertices:std::mem::take(&mut self.vertex_map),edges:std::mem::take(&mut self.edge_map)});
    }
    fn advance(&mut self,body:&mut Body,bytes:usize)->Result<Option<crate::brep::representation::topology::EntityRef>,KernelError> {
        use crate::brep::representation::topology::EntityRef;
        match self.phase {
            0=>{
                if self.cursor==self.indices.len() {self.cursor=0;self.phase=1;return Ok(None);}
                let [p0,p1,p2]=self.points()?;
                self.volume+=p0.to_vec().dot(p1.to_vec().cross(p2.to_vec()));
                self.cursor+=3;
            }
            1=>{
                if self.cursor==self.indices.len() {if self.faces.is_empty() {return Err(KernelError::Operation("no valid triangles in mesh".into()));}self.phase=2;return Ok(None);}
                let [p0,mut p1,mut p2]=self.points()?;
                let normal=(p1-p0).cross(p2-p0);
                if p0!=p1 && p1!=p2 && p0!=p2 {
                    let has_normals=if self.positions64.is_empty() {self.normals.len()>=self.positions.len()}else {self.normals64.len()>=self.positions64.len()};
                    let reverse=if has_normals {let index=self.indices[self.cursor]as usize;let n=if self.positions64.is_empty() {let n=&self.normals[index*3..index*3+3];Vec3::new(n[0]as f64,n[1]as f64,n[2]as f64)}else {self.normals64[index]};if !n.x.is_finite() || !n.y.is_finite() || !n.z.is_finite() {return Err(KernelError::InvalidInput("nonfinite triangle normal".into()));}normal.dot(n)<0.0}else {self.volume<0.0};
                    if reverse {std::mem::swap(&mut p1,&mut p2);}
                    let points=[p0,p1,p2];let normal=(p1-p0).cross(p2-p0).normalized().ok_or_else(||KernelError::InvalidInput("points are collinear".into()))?;
                    let keys=points.map(|point|[point.x,point.y,point.z].map(|value|if value==0.0 {0}else {value.to_bits()}));
                    for i in 0..3 {let a=keys[i];let b=keys[(i+1)%3];let key=(a.min(b),a.max(b));if let Some((_,uses,direction))=self.edge_map.get(&key) {let forward=a==key.0;if *uses>=2 || (*uses==1 && *direction==forward) {return Err(KernelError::InvalidInput("triangle import requires manifold coherent winding".into()));}}}
                    let mut local=OpRecorder::new();let mut vertices=Vec::with_capacity(3);let mut members=Vec::with_capacity(3);
                    for i in 0..3 {let id=if let Some(id)=self.vertex_map.get(&keys[i]) {*id}else {let id=crate::brep::operations::euler::make_vertex(body,points[i],Tol::DEFAULT,&mut local);self.vertex_map.insert(keys[i],id);id};vertices.push(id);}
                    for i in 0..3 {let j=(i+1)%3;let key=(keys[i].min(keys[j]),keys[i].max(keys[j]));let forward=keys[i]==key.0;let edge=if let Some((edge,uses,_))=self.edge_map.get_mut(&key) {*uses+=1;self.boundary_edges-=1;*edge}else {let (a,b)=if forward {(i,j)}else {(j,i)};let edge=crate::brep::operations::primitives::line_edge(body,points[a],points[b],vertices[a],vertices[b],Tol::DEFAULT,&mut local);self.edge_map.insert(key,(edge,1,forward));self.boundary_edges+=1;edge};members.push((edge,forward));}
                    let wire=crate::brep::operations::primitives::Wire {members,vertices,closed:true};let face=crate::brep::operations::primitives::make_planar_face_from_wire(body,&wire,p0,normal,&mut local)?;
                    local.own_triangle(body,face);self.recorder.append_disjoint(local);self.faces.push(face);
                }
                self.cursor+=3;
            }
            2=>{let mut local=OpRecorder::new();let shell=add_shell(body,std::mem::take(&mut self.faces),&mut local);local.own_entity(EntityRef::Shell(shell));self.recorder.append_disjoint(local);self.shell=Some(shell);self.phase=if self.boundary_edges==0 {3}else {4};}
            3=>{let mut local=OpRecorder::new();let solid=add_solid(body,self.shell.expect("retained shell"),Vec::new(),&mut local);local.own_entity(EntityRef::Solid(solid));self.recorder.append_disjoint(local);self.solid=Some(solid);self.phase=4;}
            4=>{self.release_buffers();self.retirement.close_step(1,bytes);if self.retirement.terminal_is_empty() {self.phase=5;self.provenance_credit=Some(self.recorder.retirement_bytes());}}
            5=>{let credit=self.provenance_credit.as_mut().expect("retained provenance credit");*credit=credit.saturating_sub(bytes);if *credit==0 {self.phase=6;}}
            6=>{self.phase=7;return Ok(Some(if let Some(solid)=self.solid.take() {EntityRef::Solid(solid)}else {EntityRef::Shell(self.shell.take().expect("retained shell"))}));}
            _=>return Err(KernelError::InvalidInput("triangle import already published".into())),
        }
        Ok(None)
    }
}

// #endregion 🔖️Types

// #region 🔖️Convert

/// 📦 Converts a tessellation [`MeshTransfer`] into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn triangle_mesh_from_transfer(transfer: &MeshTransfer) -> TriangleMesh {
    let mut positions = Vec::with_capacity(transfer.position.len() / 3);
    for chunk in transfer.position.as_chunks::<3>().0 {
        positions.push(Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    let mut normals = Vec::with_capacity(transfer.normal.len() / 3);
    for chunk in transfer.normal.as_chunks::<3>().0 {
        normals.push(Vec3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    TriangleMesh { positions, normals, indices: transfer.index.clone() }
}

/// 📦 Converts a [`TriangleMesh`] into framework-core [`MeshData`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_to_mesh_data(mesh: &TriangleMesh) -> MeshData {
    let mut positions = Vec::with_capacity(mesh.positions.len() * 3);
    for p in &mesh.positions {
        positions.extend_from_slice(&[p.x as f32, p.y as f32, p.z as f32]);
    }
    let mut normals = Vec::with_capacity(mesh.normals.len() * 3);
    for n in &mesh.normals {
        normals.extend_from_slice(&[n.x as f32, n.y as f32, n.z as f32]);
    }
    MeshData { positions, normals, indices: mesh.indices.clone(), ..MeshData::default() }
}

/// 📦 Converts [`MeshData`] into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_from_mesh_data(data: &MeshData) -> TriangleMesh {
    let mut positions = Vec::with_capacity(data.vertex_count());
    for chunk in data.positions.as_chunks::<3>().0 {
        positions.push(Pnt3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
    }
    let mut normals = Vec::with_capacity(positions.len());
    if data.normals.len() == data.positions.len() {
        for chunk in data.normals.as_chunks::<3>().0 {
            normals.push(Vec3::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64));
        }
    }
    TriangleMesh { positions, normals, indices: data.indices.clone() }
}

// #endregion 🔖️Convert

// #region 🔖️Api

/// 📦 Tessellates `solid` and encodes binary STL. ASCII STL export lives in the `s.stdio.stl/ascii`
/// artifact dialect (`SemioMeshToStl` + `encode_stl_ascii`), not here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_stl(body: &Body, solid: SolidId, deflection: f64) -> Result<Vec<u8>, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_stl(&triangle_mesh_from_transfer(&transfer))
}

/// 📦 Decodes STL bytes into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_stl_to_body(body: &mut Body, data: &[u8], tolerance: f64) -> Result<EntityRef, KernelError> {
    import_triangle_mesh_to_body(body, &import_stl(data)?, tolerance)
}

/// 📦 Tessellates `solid` and encodes OBJ text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_obj(body: &Body, solid: SolidId, deflection: f64) -> Result<String, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    Ok(export_obj(&triangle_mesh_from_transfer(&transfer)))
}

/// 📦 Decodes OBJ text into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_obj_to_body(body: &mut Body, text: &str, tolerance: f64) -> Result<EntityRef, KernelError> {
    import_triangle_mesh_to_body(body, &import_obj(text)?, tolerance)
}

/// 📦 Tessellates `solid` and encodes GLB bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_glb(body: &Body, solid: SolidId, deflection: f64) -> Result<Vec<u8>, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_glb(&triangle_mesh_from_transfer(&transfer))
}

/// 📦 Decodes GLB bytes into `body` as a single solid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_glb_to_body(body: &mut Body, data: &[u8], tolerance: f64) -> Result<EntityRef, KernelError> {
    import_triangle_mesh_to_body(body, &import_glb(data)?, tolerance)
}

/// 📦 Tessellates `solid` and encodes mesh mesh bytes via `exporter` — the mesh codec itself
/// (`semio_s_artifact_stdio_dwg`) is a separate artifact this kernel-layer file must not import
/// directly; the caller (`⚙️engine/🦀️.rs`, the contract façade that legitimately bridges
/// artifacts) supplies it as a [`MeshExporter`], the same pattern `export_glb`/`import_glb` already
/// use for `GlbExporter`/`GlbImporter`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_solid_mesh(body: &Body, solid: SolidId, deflection: f64, exporter: &(impl MeshExporter + ?Sized)) -> Result<Vec<u8>, KernelError> {
    let transfer = tessellate_solid(body, solid, deflection)?;
    export_mesh(&triangle_mesh_from_transfer(&transfer), exporter)
}

/// 📦 Decodes mesh mesh bytes into `body` as a single solid via `importer` — see [`export_solid_mesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_mesh_to_body(body: &mut Body, data: &[u8], tolerance: f64, importer: &(impl MeshImporter + ?Sized)) -> Result<EntityRef, KernelError> {
    import_triangle_mesh_to_body(body, &import_mesh(data, importer)?, tolerance)
}

/// 📦 Encodes a [`TriangleMesh`] as binary STL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_stl(mesh: &TriangleMesh) -> Result<Vec<u8>, KernelError> {
    if mesh.indices.len() < 3 {
        return Err(KernelError::InvalidInput("mesh has no triangles".into()));
    }
    Ok(mesh_to_stl(&mesh_to_mesh_data(mesh)))
}

/// 📦 Decodes STL bytes (auto-detects binary vs ASCII).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_stl(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    if is_ascii_stl(data) {
        read_ascii_stl(data)
    } else {
        mesh_from_stl(data).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
    }
}

/// 📦 Encodes OBJ text from a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_obj(mesh: &TriangleMesh) -> String {
    mesh_to_obj(&mesh_to_mesh_data(mesh), "mesh")
}

/// 📦 Parses OBJ text into a [`TriangleMesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_obj(text: &str) -> Result<TriangleMesh, KernelError> {
    mesh_from_obj(text).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
}

/// 📦 Encodes GLB from a [`TriangleMesh`] using [`GlbExporter`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_glb(mesh: &TriangleMesh) -> Result<Vec<u8>, KernelError> {
    let data = mesh_to_mesh_data(mesh);
    GlbExporter.export(&data).map_err(KernelError::Operation)
}

/// 📦 Decodes GLB into a [`TriangleMesh`] using [`GlbImporter`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_glb(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    GlbImporter.import(data).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
}

/// 📦 Encodes mesh mesh bytes from a [`TriangleMesh`] via `exporter` — see [`export_solid_mesh`] for
/// why the mesh codec itself is caller-supplied rather than imported here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn export_mesh(mesh: &TriangleMesh, exporter: &(impl MeshExporter + ?Sized)) -> Result<Vec<u8>, KernelError> {
    let data = mesh_to_mesh_data(mesh);
    exporter.export(&data).map_err(KernelError::Operation)
}

/// 📦 Decodes mesh bytes into a [`TriangleMesh`] via `importer` — see [`export_solid_mesh`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_mesh(data: &[u8], importer: &(impl MeshImporter + ?Sized)) -> Result<TriangleMesh, KernelError> {
    importer.import(data).map(|data| mesh_from_mesh_data(&data)).map_err(KernelError::Operation)
}

/// 📦 Imports a triangle soup as a single solid shell (one planar face per triangle).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn import_triangle_mesh_to_body(body: &mut Body, mesh: &TriangleMesh, tolerance: f64) -> Result<EntityRef, KernelError> {
    let mut cursor=MeshImportCursor::from_triangle_mesh(mesh.clone(),tolerance)?;
    loop {match cursor.step(body,4096)? {MeshImportStep::Done(entity)=>return Ok(entity),MeshImportStep::Working=>{},MeshImportStep::Cancelled=>return Err(KernelError::Operation("triangle import cancelled".into()))}}
}

// #endregion 🔖️Api

// #region 🔖️StlAscii

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_ascii_stl(data: &[u8]) -> bool {
    if data.len() < 84 {
        return data.len() >= 5 && data[..5].eq_ignore_ascii_case(b"solid");
    }
    if !data[..5].eq_ignore_ascii_case(b"solid") {
        return false;
    }
    let tri_count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]);
    let expected = 84u64 + u64::from(tri_count) * 50;
    expected != data.len() as u64
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_ascii_stl(data: &[u8]) -> Result<TriangleMesh, KernelError> {
    let text = std::str::from_utf8(data).map_err(|e| KernelError::Operation(format!("ascii stl utf-8: {e}")))?;
    let mut mesh = TriangleMesh::default();
    let mut current_normal = Vec3::Z;
    let mut vertex_count = 0u32;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("facet normal") {
            current_normal = parse_vec3_token(rest)?;
        } else if let Some(rest) = trimmed.strip_prefix("vertex") {
            mesh.positions.push(parse_point3_token(rest)?);
            mesh.normals.push(current_normal);
            mesh.indices.push(vertex_count);
            vertex_count += 1;
        }
    }
    if mesh.positions.is_empty() {
        return Err(KernelError::Operation("ascii stl has no vertices".into()));
    }
    Ok(mesh)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_vec3_token(s: &str) -> Result<Vec3, KernelError> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(KernelError::Operation(format!("expected 3 floats, got '{s}'")));
    }
    Ok(Vec3::new(parse_f64_token(parts[0])?, parse_f64_token(parts[1])?, parse_f64_token(parts[2])?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_point3_token(s: &str) -> Result<Pnt3, KernelError> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(KernelError::Operation(format!("expected 3 floats, got '{s}'")));
    }
    Ok(Pnt3::new(parse_f64_token(parts[0])?, parse_f64_token(parts[1])?, parse_f64_token(parts[2])?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64_token(s: &str) -> Result<f64, KernelError> {
    s.parse::<f64>().map_err(|e| KernelError::Operation(format!("invalid float '{s}': {e}")))
}

// #endregion 🔖️StlAscii

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

impl TriangleMesh {
    #[cfg(test)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

// #endregion 🔖️Tests
