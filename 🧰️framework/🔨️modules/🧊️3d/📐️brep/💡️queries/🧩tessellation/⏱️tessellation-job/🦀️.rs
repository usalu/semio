//! ⏱️ Tessellation job: the resumable tessellation phases, progress, steps and input, the topology cursor and the tessellation job with its stepping. See [`super`].

use super::*;
use semio_framework_3d_brep_axial_face_preview::{AxialFacePreview, AxialFacePreviewStep};
use super::analytic_component::{AnalyticComponent,PickDomain};
use semio_framework_mesh_engine::MeshData;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};

const PLANE_PREVIEW_CORNERS: [[f64; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
const PLANE_PREVIEW_BUFFERS: [usize; 4] = [12, 12, 6, 24];


// #region ⏱️ResumableJob
/// ⏱️ What a [`TessellationJob`] is currently doing. Phases run in declaration order; `Complete`
/// and `Cancelled` are terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TessellationPhase {
    /// 🧭️ Visits one original topology frontier per granted unit.
    #[default]
    CollectingTopology,
    /// 🖇️ Discretizing every distinct edge exactly once (one unit per edge).
    SamplingEdges,
    /// 🔺 Triangulating one face per unit against the shared edge samples.
    MeshingFaces,
    /// 🗃️ Emitting one edge's polyline segments and `EdgeGroup`/`EdgeInfo` metadata per unit.
    PackingEdges,
    Complete,
    Cancelled,
}

impl TessellationPhase {
    /// 🏷️ The stable wire tag every host/extension/UI layer names this phase by.
    pub fn tag(self) -> &'static str {
        match self {
            Self::CollectingTopology => "collectingTopology",
            Self::SamplingEdges => "samplingEdges",
            Self::MeshingFaces => "meshingFaces",
            Self::PackingEdges => "packingEdges",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
        }
    }
}

/// 📈 Monotone progress of one resumable tessellation: `units_done` never decreases and never
/// exceeds `units_total`, which grows while the original topology is discovered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TessellationProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub faces_done: usize,
    pub faces_total: usize,
    pub phase: TessellationPhase,
}

/// ⏱️ Outcome of one budgeted [`TessellationJob::step`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TessellationStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(TessellationProgress),
    /// ✅ Terminal: the mesh is ready, take it with [`TessellationJob::into_mesh`].
    Done(TessellationProgress),
    /// 🛑 Terminal: [`TessellationJob::cancel`] retired the job; no mesh is produced.
    Cancelled(TessellationProgress),
}

/// ⏱️ A tessellation split into budgetable units so a host can run it inside an interactive step
/// ceiling across many turns, report progress, and cancel it cleanly.
///
/// One unit is one topology cursor transition, edge discretization, or face triangulation. The
/// crack-free, edge-first algorithm shares each boundary sample across its adjacent faces. A face is
/// therefore the coarsest unit; a pathological single face still costs one whole unit, which is
/// why callers pick a budget rather than a wall-clock deadline.
/// 🧭️ Borrows the original topology target for one tessellation turn.
#[derive(Clone,Copy)]
pub enum TessellationInput<'a> { Curve(&'a Curve3), Plane(&'a Frame3), Vertex(VertexId), Edge(EdgeId), Solid(SolidId), Shell(ShellId), Face(FaceId), Wire(&'a [(EdgeId,bool)]), Solids(&'a [SolidId]) }

#[derive(Clone, Copy, Default)]
struct TopologyCursor { root:usize, pub(super) solid:Option<SolidId>,shell_index:usize,pub(super) shell:Option<ShellId>,face_index:usize,pub(super) face:Option<FaceId>,loop_index:usize,pub(super) ring:Option<(CoedgeId,CoedgeId,usize)>,pub(super) done:usize }


#[derive(Clone,Copy)]
struct SplinePreview {
    first: usize,
    next: usize,
    end: usize,
    stage: u8,
    segments: usize,
    subdivisions: usize,
    segment: usize,
    range: (f64,f64),
}

semio_framework_value::artifact_retire_leaf!(SplinePreview);

pub struct TessellationJob {
    pub(super) topology:TopologyCursor,
    pub(super) deflection: f64,
    pub(super) edge_order: Vec<EdgeId>,
    pub(super) edge_cache: EdgeSampleCache,
    pub(super) vertices: HistoryFoldIndex<VertexId, ()>,
    standalone_vertex: Option<VertexId>,
    vertex_units_done: usize,
    curve_segments: usize,
    plane_preview: bool,
    conic_face: bool,
    conic_picks: Option<super::conic_components::ConicComponents>,
    analytic_component:Option<AnalyticComponent>,
    spline: Option<SplinePreview>,
    surface_patch: Option<super::surface_preview::SurfacePatchPreview>,
    axial_face: Option<AxialFacePreview>,
    planar: Option<super::planar_preview::PlanarPreview>,
    wire_preview: Option<super::wire_preview::WirePreview>,
    funded_preview_units: usize,
    pub(super) faces: Vec<FaceId>,
    edge_cursor: usize,
    face_cursor: usize,
    pack_cursor: usize,
    pub(super) transfer: MeshTransfer,
    pub(super) report: TessellationReport,
    pub(super) phase: TessellationPhase,
}

semio_framework_value::artifact_retire_leaf!(TopologyCursor, TessellationReport, TessellationPhase);

semio_framework_value::artifact_retire_struct!(TessellationJob { topology, deflection, edge_order, edge_cache, vertices, standalone_vertex, vertex_units_done, curve_segments, plane_preview, conic_face, conic_picks, analytic_component, spline, surface_patch, axial_face, planar, wire_preview, funded_preview_units, faces, edge_cursor, face_cursor, pack_cursor, transfer, report, phase });

impl TessellationJob {
    /// 🧹️ Moves the whole original job inline without allocation or payload extraction.
    pub fn detach_retirement(self)->semio_framework_value::retirement::controlled::ControlledRetirement<Self> {
        semio_framework_value::retirement::controlled::ControlledRetirement::new(self).unwrap_or_else(|_|panic!("original tessellation owner requires typed retirement"))
    }
    /// 📍️ A resumable preview of one original topology vertex without sampled edges or faces.
    pub fn for_vertex(body: &Body, vertex: VertexId, deflection: f64) -> Result<Self, KernelError> {
        if body.vertices.get(vertex).is_none() { return Err(KernelError::MissingEntity(vertex.to_string())); }
        let mut job = Self::seeded(deflection, Vec::new(), Vec::new());
        job.standalone_vertex = Some(vertex);
        job.phase=TessellationPhase::PackingEdges;
        Ok(job)
    }

    /// 🧭️ Admits a cold owner without visiting or copying topology.
    pub fn new(deflection:f64)->Self {Self::seeded(deflection,Vec::new(),Vec::new())}
    /// 🧵️ Admits a borrowed edge or ordered wire for original analytic sampling.
    pub fn for_wire_preview(source: WirePreviewSource<'_>, revision: u64, deflection: f64) -> Self {
        let mut job = Self::new(deflection);
        job.wire_preview = Some(super::wire_preview::WirePreview::new(source, revision));
        job
    }

    pub fn is_wire_preview(&self) -> bool { self.wire_preview.is_some() }

    /// 📏️ Quotes the exact original wire read, buffer, or sample.
    pub fn wire_preview_demand(&self, body: &Body, source: WirePreviewSource<'_>, revision: u64) -> Result<RetirementDemand, ValueError> {
        if self.is_terminal() { return Ok(RetirementDemand::default()); }
        self.wire_preview.as_ref().ok_or_else(|| ValueError::invariant("wire preview owner is absent"))?.demand(body, source, revision)
    }

    /// ⏱️ Samples one funded native edge turn or falls back before allocating.
    pub fn step_wire_preview(&mut self, body: &Body, source: WirePreviewSource<'_>, revision: u64, grant: RetainedCloneGrant)
        -> Result<(Option<semio_framework_mesh_engine::MeshData>, RetainedCloneProgress), ValueError> {
        use super::wire_preview::WirePreviewStep;
        if self.is_terminal() { return Ok((None, Default::default())); }
        let preview = self.wire_preview.as_mut().ok_or_else(|| ValueError::invariant("wire preview owner is absent"))?;
        let (step, receipt) = preview.step(body, source, revision, self.deflection, grant)?;
        self.funded_preview_units += receipt.copied_items;
        match step {
            WirePreviewStep::Working => {
                self.phase = TessellationPhase::SamplingEdges;
                Ok((None, receipt))
            }
            WirePreviewStep::Unsupported => {
                self.wire_preview = None;
                self.phase = TessellationPhase::CollectingTopology;
                Ok((None, receipt))
            }
            WirePreviewStep::Ready(mesh) => {
                self.phase = TessellationPhase::Complete;
                Ok((Some(mesh), receipt))
            }
        }
    }

    /// 🪟️ Admits a planar topology preview and quotes each boundary visit and mesh event.
    pub fn for_planar_shape(root: PlanarPreviewRoot, revision: u64, deflection: f64) -> Self {
        let mut job=Self::new(deflection);
        job.planar=Some(super::planar_preview::PlanarPreview::new(root,revision,job.deflection));
        job
    }

    pub fn is_planar_preview(&self) -> bool { self.planar.is_some() }

    /// 🪪️ Quotes the next native topology or mesh access from the original body.
    pub fn planar_preview_demand(&self,body:&Body,members:Option<&[SolidId]>)->Result<RetirementDemand,ValueError>{
        self.planar.as_ref().ok_or_else(||ValueError::invariant("planar preview cursor is absent"))?.demand(body,members)
    }

    /// 🧱️ Advances one planar preview event against the unchanged native source.
    pub fn step_planar_funded(&mut self, body:&Body, root:PlanarPreviewRoot, members:Option<&[SolidId]>, revision:u64, grant:RetainedCloneGrant)
        -> Result<(Option<semio_framework_mesh_engine::MeshData>, RetainedCloneProgress), ValueError> {
        if self.is_terminal() { return Ok((None,Default::default())); }
        let preview=self.planar.as_mut().ok_or_else(||ValueError::invariant("planar preview cursor is absent"))?;
        let (event,receipt)=preview.advance(body,root,members,revision,grant)?;
        self.funded_preview_units+=receipt.copied_items;
        match event {
            super::planar_preview::PlanarPreviewStep::Pending=>{
                if preview.is_meshing() { self.phase=TessellationPhase::MeshingFaces; }
                Ok((None,receipt))
            }
            super::planar_preview::PlanarPreviewStep::Unsupported=>{
                self.planar=None;
                Ok((None,receipt))
            }
            super::planar_preview::PlanarPreviewStep::Ready(mesh)=>{
                self.phase=TessellationPhase::Complete;
                Ok((Some(mesh),receipt))
            }
        }
    }

    /// 🏄️ Admits a standalone analytic or rational surface without allocating its preview until funded.
    pub fn for_surface_patch(surface:&Surface,label:u64,deflection:f64)->Result<Self,ValueError>{
        let mut job=Self::new(deflection);
        job.surface_patch=Some(super::surface_preview::SurfacePatchPreview::new(surface,label,deflection)?);
        job.analytic_component=Some(AnalyticComponent::new(label,PickDomain::Face));
        Ok(job)
    }

    /// 🎲️ Samples one error bound, allocation, or triangle on the admitted original surface.
    pub fn step_surface_patch_funded(&mut self,surface:&Surface,label:u64,grant:RetainedCloneGrant)
        ->Result<(Option<MeshData>,RetainedCloneProgress),ValueError>{
        self.step_with_component(label,grant,|job,grant|{
            let patch=job.surface_patch.as_mut().ok_or_else(||ValueError::invariant("surface preview cursor is absent"))?;
            let (mesh,receipt)=patch.advance(surface,label,grant)?;
            job.funded_preview_units+=receipt.copied_items;
            if patch.is_meshing(){job.phase=TessellationPhase::MeshingFaces;}
            Ok((mesh,receipt))
        })
    }

    /// 🪟️ Admits a rectangular axial trim for validation against the original native face.
    pub fn for_axial_face_preview(face: FaceId, revision: u64, deflection: f64) -> Self {
        let mut job = Self::new(deflection);
        job.axial_face = Some(AxialFacePreview::new(face, revision, job.deflection));
        job
    }

    pub fn is_axial_face_preview(&self) -> bool { self.axial_face.is_some() }

    pub fn axial_face_preview_demand(&self, body: &Body, face: FaceId, revision: u64) -> Result<RetirementDemand, ValueError> {
        if self.is_terminal() { return Ok(RetirementDemand::default()); }
        self.axial_face.as_ref().ok_or_else(|| ValueError::invariant("axial face preview owner is absent"))?.demand(body, face, revision)
    }

    /// 🕹️ Advances one validated UV trim, triangle, topology pick segment or allocation.
    pub fn step_axial_face_preview(&mut self, body: &Body, face: FaceId, revision: u64, grant: RetainedCloneGrant)
        -> Result<(Option<semio_framework_mesh_engine::MeshData>, RetainedCloneProgress), ValueError> {
        if self.is_terminal() { return Ok((None, Default::default())); }
        let preview = self.axial_face.as_mut().ok_or_else(|| ValueError::invariant("axial face preview owner is absent"))?;
        let (step, receipt) = preview.step(body, face, revision, grant)?;
        self.funded_preview_units += receipt.copied_items;
        match step {
            AxialFacePreviewStep::Working => {
                if preview.is_meshing() { self.phase = TessellationPhase::MeshingFaces; }
                Ok((None, receipt))
            }
            AxialFacePreviewStep::Unsupported => {
                self.axial_face = None;
                self.phase = TessellationPhase::CollectingTopology;
                Ok((None, receipt))
            }
            AxialFacePreviewStep::Ready(mesh) => {
                self.phase = TessellationPhase::Complete;
                Ok((Some(mesh), receipt))
            }
        }
    }

    /// 🖊️ Admits standalone curve segments with the original shape's pick reference.
    pub fn for_curve_component(curve:&Curve3,label:u64,deflection:f64)->Result<Self,KernelError>{
        let mut job=Self::for_analytic_curve(curve,deflection)?;
        job.analytic_component=Some(AnalyticComponent::new(label,PickDomain::Edge));
        Ok(job)
    }

    /// 🗺️ Admits the finite original plane datum with its face pick reference.
    pub fn for_plane_component(deflection:f64,label:u64)->Self{
        let mut job=Self::for_analytic_plane(deflection);
        job.analytic_component=Some(AnalyticComponent::new(label,PickDomain::Face));
        job
    }

    /// 🖇️ Advances one original curve sampling or component publication unit.
    pub fn step_curve_component(&mut self,curve:&Curve3,label:u64,grant:RetainedCloneGrant)
        ->Result<(Option<MeshData>,RetainedCloneProgress),ValueError>{
        self.step_with_component(label,grant,|job,grant|{
            let (edges,receipt)=job.step_analytic_funded(curve,grant)?;
            Ok((edges.map(|edge_positions|MeshData{edge_positions,..Default::default()}),receipt))
        })
    }

    /// 📄️ Advances the original plane buffer or its bounded component map.
    pub fn step_plane_component(&mut self,frame:&Frame3,label:u64,grant:RetainedCloneGrant)
        ->Result<(Option<MeshData>,RetainedCloneProgress),ValueError>{
        self.step_with_component(label,grant,|job,grant|{
            let (mesh,receipt)=job.step_analytic_plane_funded(frame,grant)?;
            Ok((mesh.map(|mesh|MeshData{positions:mesh.position,normals:mesh.normal,indices:mesh.index,
                edge_positions:mesh.edges,..Default::default()}),receipt))
        })
    }

    fn step_with_component(&mut self,label:u64,grant:RetainedCloneGrant,
        step:impl FnOnce(&mut Self,RetainedCloneGrant)->Result<(Option<MeshData>,RetainedCloneProgress),ValueError>)
        ->Result<(Option<MeshData>,RetainedCloneProgress),ValueError>{
        if self.is_terminal(){return Ok((None,Default::default()));}
        let absent=||ValueError::invariant("analytic preview component cursor is absent");
        let component=self.analytic_component.as_mut().ok_or_else(absent)?;
        component.check_label(label)?;
        if component.demand().is_some(){
            let (mesh,receipt)=component.advance(grant)?;
            self.funded_preview_units+=receipt.copied_items;
            if mesh.is_some(){self.phase=TessellationPhase::Complete;}
            return Ok((mesh,receipt));
        }
        let (mesh,receipt)=step(self,grant)?;
        if let Some(mesh)=mesh{
            self.analytic_component.as_mut().ok_or_else(absent)?.hold(mesh);
            self.phase=TessellationPhase::PackingEdges;
        }
        Ok((None,receipt))
    }

    fn seeded(deflection:f64,edge_order:Vec<EdgeId>,faces:Vec<FaceId>)->Self {
        Self {topology:TopologyCursor::default(),deflection:deflection.max(1e-9),edge_cache:EdgeSampleCache::new(),vertices:Default::default(),standalone_vertex:None,vertex_units_done:0,curve_segments:0,plane_preview:false,conic_face:false,conic_picks:None,analytic_component:None,spline:None,surface_patch:None,axial_face:None,planar:None,wire_preview:None,funded_preview_units:0,edge_order,faces,edge_cursor:0,face_cursor:0,pack_cursor:0,transfer:Default::default(),report:Default::default(),phase:TessellationPhase::CollectingTopology}
    }
    fn collect_edge(&mut self,edge:EdgeId) {
        if !self.edge_cache.contains_key(&edge) {self.edge_cache.insert(edge, Vec::new());self.edge_order.push(edge);}
    }
    fn collect_topology(&mut self,body:&Body,input:TessellationInput<'_>)->Result<bool,KernelError> {
        if let Some((first,current,visited))=self.topology.ring {
            if visited>=body.coedges.len() {return Err(KernelError::InvalidInput("tessellation coedge cycle does not close".into()));}
            let coedge=body.coedges.get(current).ok_or_else(||KernelError::MissingEntity(current.to_string()))?;
            self.collect_edge(coedge.edge);
            self.topology.ring=if coedge.next==first {None}else {Some((first,coedge.next,visited+1))};
            return Ok(false);
        }
        if let Some(id)=self.topology.face {
            let face=body.faces.get(id).ok_or_else(||KernelError::MissingEntity(id.to_string()))?;
            let loop_id=if self.topology.loop_index==0 {face.outer}else {face.inners.get(self.topology.loop_index-1).copied()};
            self.topology.loop_index+=1;
            if let Some(id)=loop_id {let ring=body.loops.get(id).ok_or_else(||KernelError::MissingEntity(id.to_string()))?;self.topology.ring=Some((ring.first,ring.first,0));}
            else if self.topology.loop_index>face.inners.len()+1 {self.topology.face=None;}
            return Ok(false);
        }
        if let Some(id)=self.topology.shell {
            let shell=body.shells.get(id).ok_or_else(||KernelError::MissingEntity(id.to_string()))?;
            if let Some(face)=shell.faces.get(self.topology.face_index).copied() {self.faces.push(face);self.topology.face=Some(face);self.topology.loop_index=0;self.topology.face_index+=1;}
            else {self.topology.shell=None;}
            return Ok(false);
        }
        if let Some(id)=self.topology.solid {
            let solid=body.solids.get(id).ok_or_else(||KernelError::MissingEntity(id.to_string()))?;
            let shell=if self.topology.shell_index==0 {Some(solid.outer)}else {solid.inners.get(self.topology.shell_index-1).copied()};
            self.topology.shell_index+=1;
            if let Some(shell)=shell {self.topology.shell=Some(shell);self.topology.face_index=0;}else {self.topology.solid=None;}
            return Ok(false);
        }
        match input {
            TessellationInput::Wire(edges)=>{if let Some((edge,_))=edges.get(self.topology.root) {if !self.edge_cache.contains_key(edge) {self.edge_cache.insert(*edge, Vec::new());}self.edge_order.push(*edge);self.topology.root+=1;return Ok(false);}}
            TessellationInput::Solids(solids)=>{if let Some(solid)=solids.get(self.topology.root) {self.topology.solid=Some(*solid);self.topology.shell_index=0;self.topology.root+=1;return Ok(false);}}
            _ if self.topology.root==0=>{self.topology.root=1;match input {
                TessellationInput::Vertex(vertex)=>{self.standalone_vertex=Some(vertex);}
                TessellationInput::Edge(edge)=>{self.collect_edge(edge);}
                TessellationInput::Solid(solid)=>{self.topology.solid=Some(solid);}
                TessellationInput::Shell(shell)=>{self.topology.shell=Some(shell);}
                TessellationInput::Face(face)=>{self.faces.push(face);self.topology.face=Some(face);}
                _=>unreachable!(),
            }return Ok(false);}
            _=>{}
        }
        if matches!(input,TessellationInput::Solid(_)|TessellationInput::Shell(_)|TessellationInput::Solids(_))&&self.faces.is_empty() {return Err(KernelError::InvalidInput("tessellation target has no faces".into()));}
        Ok(true)
    }

    /// 📈 This job's progress right now — safe to read between steps and after termination.
    pub fn progress(&self) -> TessellationProgress {
        TessellationProgress {
            units_done: self.units_done(),
            units_total: self.units_total(),
            faces_done: self.face_cursor,
            faces_total: self.faces.len(),
            phase: self.phase,
        }
    }

    fn units_done(&self)->usize{
        self.conic_picks.as_ref().map_or(0,|picks|picks.storage_steps())+self.funded_preview_units+self.topology.done+self.edge_cursor+self.face_cursor+self.pack_cursor+self.vertex_units_done
    }

    fn units_total(&self) -> usize {
        if self.analytic_component.is_some(){return self.units_done()+usize::from(!self.is_terminal());}
        if self.planar.is_some() || self.wire_preview.is_some() || self.surface_patch.is_some() || self.axial_face.is_some() { return self.funded_preview_units + usize::from(!self.is_terminal()); }
        if self.plane_preview { return PLANE_PREVIEW_BUFFERS.len(); }
        if self.conic_face { return 5+self.curve_segments+if self.conic_picks.is_some(){super::conic_components::ConicComponents::STORAGE_STEPS}else{0}; }
        if let Some(spline)=self.spline {return 1+2*(spline.end-spline.first)+spline.segments;}
        self.funded_preview_units + self.curve_segments + self.topology.done + usize::from(self.phase==TessellationPhase::CollectingTopology) + self.edge_order.len() + self.faces.len() + self.edge_order.len() + self.vertex_units_done + usize::from(self.standalone_vertex.is_some())
    }

    /// 🛑 Retires this job at the next observable boundary — the current phase becomes `Cancelled`
    /// immediately; original buffers remain owned until granted retirement.
    pub fn cancel(&mut self) {
        if matches!(self.phase, TessellationPhase::Complete) {
            return;
        }
        self.phase = TessellationPhase::Cancelled;

    }

    /// ✅ True once the job reached a terminal phase.
    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, TessellationPhase::Complete | TessellationPhase::Cancelled)
    }

    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe that
    /// performs no work.
    pub fn step(&mut self, body: &Body,input:TessellationInput<'_>, budget: usize) -> Result<TessellationStep, KernelError> {
        match self.phase {
            TessellationPhase::Complete => return Ok(TessellationStep::Done(self.progress())),
            TessellationPhase::Cancelled => return Ok(TessellationStep::Cancelled(self.progress())),
            _ => {}
        }
        match input {
            TessellationInput::Curve(curve) => return self.step_curve(curve, budget),
            TessellationInput::Plane(frame) => {
                self.plane_preview = true;
                for _ in 0..budget.min(PLANE_PREVIEW_BUFFERS.len()) {
                    if self.phase == TessellationPhase::Complete { break; }
                    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 128, maximum_capacity_bytes: 128, maximum_release_bytes: 0, maximum_depth: 1 };
                    self.advance_plane(frame, grant).map_err(|_| KernelError::InvalidInput("plane preview failed".into()))?;
                }
                let progress = self.progress();
                return Ok(if self.phase == TessellationPhase::Complete { TessellationStep::Done(progress) } else { TessellationStep::Working(progress) });
            }
            _ => {}
        }
        let mut spent = 0;
        while spent < budget {
            match self.phase {
                TessellationPhase::CollectingTopology=>{
                    let complete=self.collect_topology(body,input)?;self.topology.done+=1;
                    if complete {self.phase=TessellationPhase::SamplingEdges;}
                }
                TessellationPhase::SamplingEdges => {
                    if self.edge_cursor >= self.edge_order.len() {
                        self.phase = if self.faces.is_empty() { TessellationPhase::PackingEdges } else { TessellationPhase::MeshingFaces };
                        continue;
                    }
                    let edge = self.edge_order[self.edge_cursor];
                    if self.edge_cache.get(&edge).is_none_or(Vec::is_empty) {self.edge_cache.insert(edge,sample_edge_points(body,edge,self.deflection)?);}
                    self.edge_cursor += 1;
                }
                TessellationPhase::MeshingFaces => {
                    if self.face_cursor >= self.faces.len() {
                        self.phase = TessellationPhase::PackingEdges;
                        continue;
                    }
                    let face = self.faces[self.face_cursor];
                    append_face_mesh(&mut self.transfer, &mut self.report, body, face, self.deflection, &self.edge_cache)?;
                    self.face_cursor += 1;
                }
                TessellationPhase::PackingEdges => {
                    if let Some(vertex) = self.standalone_vertex {
                        self.pack_vertex(body, vertex)?;
                        self.standalone_vertex = None;
                        self.vertex_units_done += 1;
                    } else {
                        if self.pack_cursor >= self.edge_order.len() {
                            self.phase = TessellationPhase::Complete;
                            return Ok(TessellationStep::Done(self.progress()));
                        }
                        let edge = self.edge_order[self.pack_cursor];
                        self.pack_edge(body, edge)?;
                        self.pack_cursor += 1;
                    }
                }
                TessellationPhase::Complete => return Ok(TessellationStep::Done(self.progress())),
                TessellationPhase::Cancelled => return Ok(TessellationStep::Cancelled(self.progress())),
            }
            spent += 1;
        }
        if self.phase!=TessellationPhase::CollectingTopology && self.edge_cursor >= self.edge_order.len() && self.face_cursor >= self.faces.len() && self.pack_cursor >= self.edge_order.len() && self.standalone_vertex.is_none() {
            self.phase = TessellationPhase::Complete;
            return Ok(TessellationStep::Done(self.progress()));
        }
        Ok(TessellationStep::Working(self.progress()))
    }

    fn analytic_segment_count(&self, curve: &Curve3) -> Result<usize, KernelError> {
        let count = match curve {
            Curve3::Line { .. } => 1,
            Curve3::Circle { radius, .. } =>
                edge_sampling::segments_for_chord_deviation(radius.abs(), std::f64::consts::TAU, self.deflection, DEFAULT_ANGULAR_TOL),
            Curve3::Ellipse { major_radius, minor_radius, .. } =>
                edge_sampling::segments_for_chord_deviation(major_radius.abs().max(minor_radius.abs()), std::f64::consts::TAU, self.deflection, DEFAULT_ANGULAR_TOL),
            Curve3::Nurbs { .. } => return Err(KernelError::InvalidInput("standalone curve requires analytic sampling".into())),
        };
        if count > 200_000 { return Err(KernelError::InvalidInput("curve deflection exceeds the segment limit".into())); }
        Ok(count)
    }

    /// 🎟️ Admits an analytic curve without allocating its polyline until the original caller funds it.
    pub fn for_analytic_curve(curve: &Curve3, deflection: f64) -> Result<Self, KernelError> {
        let mut job = Self::new(deflection);
        job.admit_standalone_curve(curve)?;
        job.phase = TessellationPhase::SamplingEdges;
        Ok(job)
    }

    /// 🗺️ Admits the finite datum preview of an unbounded analytic plane.
    pub fn for_analytic_plane(deflection: f64) -> Self {
        let mut job = Self::new(deflection);
        job.plane_preview = true;
        job.phase = TessellationPhase::MeshingFaces;
        job
    }


    /// 🪷️ Admits a full periodic planar trim as a bounded triangle fan.
    pub fn for_conic_face(curve: &Curve3, deflection: f64) -> Result<Self, KernelError> {
        if !matches!(curve,Curve3::Circle{..}|Curve3::Ellipse{..}) {
            return Err(KernelError::InvalidInput("conic faces require a complete circle or ellipse".into()));
        }
        let mut job = Self::for_analytic_curve(curve,deflection)?;
        job.curve_segments = job.curve_segments.max(3);
        job.conic_face = true;
        job.phase = TessellationPhase::MeshingFaces;
        Ok(job)
    }

    /// 🎯️ Attaches the original face, edge, and seam vertex labels to a conic disk preview.
    pub fn for_conic_face_components(curve:&Curve3,deflection:f64,revision:u64,labels:[u64;3])->Result<Self,KernelError>{
        let mut job=Self::for_conic_face(curve,deflection)?;
        job.conic_picks=Some(super::conic_components::ConicComponents::new(revision,labels));
        Ok(job)
    }

    fn conic_components_demand(&self)->RetirementDemand{
        let mut demand=self.conic_face_demand();
        if let Some(picks)=&self.conic_picks {
            if self.is_terminal(){ return demand; }
            if let Some(storage)=picks.demand(self.curve_segments){return storage;}
            demand.copy_bytes+=super::conic_components::ConicComponents::sample_bytes(self.pack_cursor);
        }
        demand
    }

    /// 🧲️ Emits one funded conic fan event and its matching native component picks.
    pub fn step_conic_face_components(&mut self,curve:&Curve3,normal:[f64;3],flip:bool,revision:u64,grant:RetainedCloneGrant)
        ->Result<(Option<semio_framework_mesh_engine::MeshData>,RetainedCloneProgress),ValueError>{
        let demand=self.conic_components_demand();
        let terminal=self.is_terminal();
        let picks=self.conic_picks.as_mut().ok_or_else(||ValueError::invariant("conic component cursor is absent"))?;
        picks.check_revision(revision)?;
        if terminal||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok((None,Default::default()));}
        if picks.demand(self.curve_segments).is_some(){
            let receipt=picks.advance_storage(self.curve_segments,demand)?;
            if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"conic component reservation exceeded its grant").with_retained_progress(receipt));}
            return Ok((None,receipt));
        }
        let cursor=self.pack_cursor;
        let (mesh,mut receipt)=self.step_conic_face_funded(curve,normal,flip,grant)?;
        let picks=self.conic_picks.as_mut().expect("conic component cursor");
        picks.append_pick(cursor);
        receipt.copied_bytes+=super::conic_components::ConicComponents::sample_bytes(cursor);
        Ok((mesh.map(|mesh|picks.finish(mesh)),receipt))
    }

    fn conic_face_demand(&self) -> RetirementDemand {
        if self.is_terminal() { return Default::default(); }
        let n = self.curve_segments;
        let (count,width) = match self.pack_cursor {
            0 | 1 => (3*(n+1),0),
            2 => (3*n,0),
            3 => (6*n,0),
            4 => (0,6),
            _ => (0,15),
        };
        RetirementDemand { capacity_bytes:count*size_of::<f32>(),copy_bytes:width*size_of::<f32>(),depth:1,..Default::default() }
    }

    /// ⏱️ Retains one original buffer or one fan triangle per granted turn.
    pub fn step_conic_face_funded(&mut self, curve:&Curve3, normal:[f64;3], flip:bool, grant:RetainedCloneGrant) -> Result<(Option<MeshTransfer>,RetainedCloneProgress),ValueError> {
        let demand = self.conic_face_demand();
        if self.is_terminal() || !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand) { return Ok((None,Default::default())); }
        let frame = match curve {
            Curve3::Circle{frame,..} | Curve3::Ellipse{frame,..} => frame,
            _ => return Err(ValueError::invariant("original conic face source changed")),
        };
        fn reserve<T>(buffer:&mut Vec<T>, bytes:usize) -> Result<usize,ValueError> {
            buffer.try_reserve_exact(bytes/size_of::<T>()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed,"conic face preview backing was refused"))?;
            Ok(buffer.capacity()*size_of::<T>())
        }
        fn position(point:Pnt3) -> Result<[f32;3],ValueError> {
            let vertex=[point.x as f32,point.y as f32,point.z as f32];
            if vertex.iter().all(|value| value.is_finite()) { Ok(vertex) }
            else { Err(ValueError::literal(ValueRefusalKind::InvalidValue,"conic face exceeds the preview coordinate range")) }
        }
        let mut receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let n=self.curve_segments;
        let normal=normal.map(|value| value as f32);
        match self.pack_cursor {
            0 => receipt.retained_capacity_bytes=reserve(&mut self.transfer.position,demand.capacity_bytes)?,
            1 => receipt.retained_capacity_bytes=reserve(&mut self.transfer.normal,demand.capacity_bytes)?,
            2 => receipt.retained_capacity_bytes=reserve(&mut self.transfer.index,demand.capacity_bytes)?,
            3 => receipt.retained_capacity_bytes=reserve(&mut self.transfer.edges,demand.capacity_bytes)?,
            4 => {
                self.transfer.position.extend(position(frame.origin)?);
                self.transfer.normal.extend(normal);
            }
            cursor => {
                let i=cursor-5;
                let next=(i+1)%n;
                let sample=|index:usize| position(curve.eval(std::f64::consts::TAU*index as f64/n as f64));
                let a=sample(i)?;
                let b=sample(next)?;
                self.transfer.position.extend(a);
                self.transfer.normal.extend(normal);
                let ends=[1+i as u32,1+next as u32];
                let ends=if flip {[ends[1],ends[0]]}else{ends};
                self.transfer.index.extend([0,ends[0],ends[1]]);
                self.transfer.edges.extend(a);
                self.transfer.edges.extend(b);
            }
        }
        if !receipt.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"conic face preview reservation exceeded its grant").with_retained_progress(receipt)); }
        self.pack_cursor+=1;
        if self.pack_cursor==n+5 { self.phase=TessellationPhase::Complete; }
        Ok((self.take_mesh().map(|(mesh,_)|mesh),receipt))
    }

    fn plane_demand(&self) -> RetirementDemand {
        let Some(&items) = PLANE_PREVIEW_BUFFERS.get(self.pack_cursor) else { return RetirementDemand::default(); };
        let bytes = items * size_of::<u32>();
        RetirementDemand { copy_bytes: bytes, capacity_bytes: bytes, depth: 1, ..Default::default() }
    }

    /// 🧮️ Builds one funded preview buffer and transfers the original mesh when complete.
    pub fn step_analytic_plane_funded(&mut self, frame: &Frame3, grant: RetainedCloneGrant) -> Result<(Option<MeshTransfer>, RetainedCloneProgress), ValueError> {
        if !self.plane_preview {
            return Err(ValueError::invariant("plane preview was not admitted"));
        }
        let receipt = self.advance_plane(frame, grant)?;
        Ok((self.take_mesh().map(|(mesh, _)| mesh), receipt))
    }

    fn advance_plane(&mut self, frame: &Frame3, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if matches!(self.phase, TessellationPhase::Complete | TessellationPhase::Cancelled) {
            return Ok(RetainedCloneProgress::default());
        }
        let demand = self.plane_demand();
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_copy_bytes < demand.copy_bytes {
            return Ok(RetainedCloneProgress::default());
        }
        let receipt = match self.pack_cursor {
            0 => write_plane_buffer(&mut self.transfer.position, plane_points(frame)?.as_flattened(), grant)?,
            1 => {
                let n = frame.x.cross(frame.y).normalized().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "plane preview has no finite normal"))?;
                let normals = [[n.x as f32, n.y as f32, n.z as f32]; 4];
                write_plane_buffer(&mut self.transfer.normal, normals.as_flattened(), grant)?
            }
            2 => write_plane_buffer(&mut self.transfer.index, &[0, 1, 2, 0, 2, 3], grant)?,
            3 => {
                let [a, b, c, d] = plane_points(frame)?;
                let outline = [a, b, b, c, c, d, d, a];
                write_plane_buffer(&mut self.transfer.edges, outline.as_flattened(), grant)?
            }
            _ => return Err(ValueError::invariant("plane preview has no remaining owner")),
        };
        self.pack_cursor += 1;
        self.phase = if self.pack_cursor == PLANE_PREVIEW_BUFFERS.len() { TessellationPhase::Complete }
            else if self.pack_cursor + 1 == PLANE_PREVIEW_BUFFERS.len() { TessellationPhase::PackingEdges }
            else { TessellationPhase::MeshingFaces };
        Ok(receipt)
    }

    /// 📏️ Quotes one admitted analytic preview allocation or buffer write.
    pub fn analytic_demand(&self) -> Option<semio_framework_value::RetirementDemand> {
        if let Some(demand)=self.analytic_component.as_ref().and_then(AnalyticComponent::demand){return Some(demand);}
        if let Some(preview)=&self.surface_patch {return Some(preview.demand());}
        if self.plane_preview { return Some(self.plane_demand()); }
        if self.conic_face { return Some(self.conic_components_demand()); }
        if self.spline.is_some() {return Some(self.spline_preview_demand());}
        if self.curve_segments == 0 { return None; }
        let capacity = self.curve_segments * 6;
        let reserve = self.transfer.edges.capacity() < capacity;
        Some(semio_framework_value::RetirementDemand {
            capacity_bytes: if reserve { capacity * size_of::<f32>() } else { 0 },
            copy_bytes: if reserve { 0 } else { 6 * size_of::<f32>() },
            depth: 1,
            ..Default::default()
        })
    }

    /// 🧵 Writes one admitted segment and transfers the completed edge buffer to its caller.
    pub fn step_analytic_funded(&mut self, curve: &Curve3, grant: semio_framework_value::RetainedCloneGrant)
        -> Result<(Option<Vec<f32>>, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
        if self.spline.is_some() {
            let progress=self.advance_spline(curve,grant)?;
            let output=matches!(self.phase,TessellationPhase::Complete).then(||std::mem::take(&mut self.transfer.edges));
            return Ok((output,progress));
        }
        let demand = self.analytic_demand().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "analytic curve cursor is absent"))?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_copy_bytes < demand.copy_bytes {
            return Ok((None, RetainedCloneProgress::default()));
        }
        let mut receipt = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if demand.capacity_bytes != 0 {
            self.transfer.edges.try_reserve_exact(self.curve_segments * 6).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "curve edge buffer reservation failed"))?;
            receipt.retained_capacity_bytes = self.transfer.edges.capacity() * size_of::<f32>();
            if !receipt.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "curve reservation exceeded its grant").with_retained_progress(receipt)); }
            self.topology.done += 1;
            return Ok((None, receipt));
        }
        let status = self.step_curve(curve, 1).map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "analytic curve segment failed"))?;
        receipt.copied_bytes = demand.copy_bytes;
        let edges = matches!(status, TessellationStep::Done(_)).then(|| std::mem::take(&mut self.transfer.edges));
        Ok((edges, receipt))
    }


    fn admit_standalone_curve(&mut self,curve:&Curve3) -> Result<(),KernelError> {
        if matches!(curve,Curve3::Nurbs{..}) {
            let (first,end)=curve.original_spline_span_range().ok_or_else(||KernelError::InvalidInput("invalid or unsupported standalone spline knot vector".into()))?;
            self.spline=Some(SplinePreview{first,next:first,end,stage:0,segments:0,subdivisions:0,segment:0,range:(0.0,0.0)});
        } else {self.curve_segments=self.analytic_segment_count(curve)?;}
        Ok(())
    }

    fn spline_preview_demand(&self) -> RetirementDemand {
        let Some(spline)=self.spline else{return Default::default()};
        if self.is_terminal() {return Default::default();}
        RetirementDemand {
            depth:1,
            capacity_bytes:if spline.stage==1 {spline.segments*6*size_of::<f32>()}else{0},
            copy_bytes:if spline.stage==3 {6*size_of::<f32>()}else{0},
            ..Default::default()
        }
    }

    fn advance_spline(&mut self,curve:&Curve3,grant:RetainedCloneGrant) -> Result<RetainedCloneProgress,ValueError> {
        let demand=self.spline_preview_demand();
        if self.is_terminal() || !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand) {return Ok(Default::default());}
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"spline preview requires finite positive weights and bounded subdivision");
        let spline=self.spline.as_mut().ok_or_else(invalid)?;
        if curve.original_spline_span_range()!=Some((spline.first,spline.end)){return Err(invalid());}
        let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match spline.stage {
            0 => {
                let (_,_,count)=curve.original_spline_subdivision(spline.next,self.deflection).ok_or_else(invalid)?;
                spline.segments=spline.segments.checked_add(count).filter(|count|*count<=200_000).ok_or_else(invalid)?;
                spline.next+=1;
                if spline.next==spline.end {
                    if spline.segments==0 {return Err(invalid());}
                    spline.stage=1;
                }
            }
            1 => {
                self.transfer.edges.try_reserve_exact(spline.segments*6)
                    .map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"spline edge buffer allocation failed"))?;
                receipt.retained_capacity_bytes=self.transfer.edges.capacity()*size_of::<f32>();
                spline.next=spline.first;
                spline.stage=2;
            }
            2 => {
                let (lo,hi,count)=curve.original_spline_subdivision(spline.next,self.deflection).ok_or_else(invalid)?;
                spline.range=(lo,hi);
                spline.subdivisions=count;
                spline.segment=0;
                if count!=0 {spline.stage=3;}
                else {
                    spline.next+=1;
                    if spline.next==spline.end {self.phase=TessellationPhase::Complete;}
                }
            }
            _ => {
                let (lo,hi)=spline.range;
                let t=|i| if i==spline.subdivisions {hi} else {lo+(hi-lo)*i as f64/spline.subdivisions as f64};
                let a=curve.original_spline_span_point(spline.next,t(spline.segment)).ok_or_else(invalid)?;
                let b=curve.original_spline_span_point(spline.next,t(spline.segment+1)).ok_or_else(invalid)?;
                let values=[a.x,a.y,a.z,b.x,b.y,b.z].map(|v|v as f32);
                if values.iter().any(|v|!v.is_finite()) || self.transfer.edges.len()+values.len()>spline.segments*6{ return Err(invalid());}
                self.transfer.edges.extend(values);
                spline.segment+=1;
                if spline.segment==spline.subdivisions {
                    spline.next+=1;
                    if spline.next==spline.end {self.phase=TessellationPhase::Complete;}
                    spline.stage=2;
                }
            }
        }
        if !receipt.fits(grant) {return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"spline edge backing exceeded its grant").with_retained_progress(receipt));}
        self.pack_cursor+=1;
        Ok(receipt)
    }

    fn step_spline_cold(&mut self,curve:&Curve3,budget:usize) -> Result<TessellationStep,KernelError> {
        for _ in 0..budget {
            if self.is_terminal(){break;}
            let d=self.spline_preview_demand();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:d.capacity_bytes,maximum_copy_bytes:d.copy_bytes,maximum_release_bytes:0,maximum_depth:d.depth};
            self.advance_spline(curve,grant).map_err(|error|KernelError::InvalidInput(error.to_string()))?;
        }
        if self.is_terminal(){Ok(TessellationStep::Done(self.progress()))}
        else {Ok(TessellationStep::Working(self.progress()))}
    }

    fn step_curve(&mut self, curve: &Curve3, mut budget: usize) -> Result<TessellationStep, KernelError> {
        if self.phase == TessellationPhase::CollectingTopology && budget != 0 {
            self.admit_standalone_curve(curve)?;
            self.topology.done = 1;
            self.phase = TessellationPhase::SamplingEdges;
            budget -= 1;
        }
        if self.spline.is_some() {return self.step_spline_cold(curve,budget);}
        let closed = curve.is_periodic();
        let extent = if closed { std::f64::consts::TAU } else { 1.0 };
        while budget != 0 && self.pack_cursor < self.curve_segments {
            let parameter = |index| if closed && index == self.curve_segments { 0.0 }
                else { extent * index as f64 / self.curve_segments as f64 };
            let segment = [curve.eval(parameter(self.pack_cursor)), curve.eval(parameter(self.pack_cursor + 1))];
            push_polyline_segments(&mut self.transfer.edges, &segment);
            self.pack_cursor += 1;
            budget -= 1;
        }
        if self.curve_segments != 0 && self.pack_cursor == self.curve_segments {
            self.phase = TessellationPhase::Complete;
            Ok(TessellationStep::Done(self.progress()))
        } else {
            Ok(TessellationStep::Working(self.progress()))
        }
    }

    /// 🗃️ One edge's polyline segments and analytic identity for every topology target.
    fn pack_edge(&mut self, body: &Body, edge_id: EdgeId) -> Result<(), KernelError> {
        let Some(points) = self.edge_cache.get(&edge_id) else { return Ok(()) };
        let start = (self.transfer.edges.len() / 6) as u32;
        let pts: Vec<Pnt3> = points.iter().map(|&(_, p)| p).collect();
        let length = polyline_length(points);
        push_polyline_segments(&mut self.transfer.edges, &pts);
        let Some(edge) = body.edges.get(edge_id) else { return Ok(()) };
        let count = pts.len().saturating_sub(1) as u32;
        let label = edge.label.0.to_string();
        let curve_kind = body.curves3.get(edge.curve).map_or(CurveKind::Line, curve_kind_of);
        self.transfer.edge_groups.push(EdgeGroup { start, count, entity_id: label.clone() });
        self.transfer.edge_infos.push(EdgeInfo { entity_id: label, curve_kind, length });
        for id in [edge.v0, edge.v1] { self.pack_vertex(body, id)?; }
        Ok(())
    }

    fn pack_vertex(&mut self, body: &Body, id: VertexId) -> Result<(), KernelError> {
        if self.vertices.contains_key(&id) { return Ok(()); }
        let vertex = body.vertices.get(id).ok_or_else(|| KernelError::MissingEntity(id.to_string()))?;
        let start = u32::try_from(self.transfer.points.len() / 3).map_err(|_| KernelError::InvalidInput("preview vertex capacity exceeded".into()))?;
        self.transfer.points.extend_from_slice(&[vertex.position.x as f32, vertex.position.y as f32, vertex.position.z as f32]);
        self.transfer.vertex_groups.push(VertexGroup { start, count: 1, entity_id: vertex.label.0.to_string() });
        self.vertices.insert(id, ());
        Ok(())
    }

    /// 📤 Takes the finished mesh — `None` unless the job reached `Complete`.
    pub fn into_mesh(mut self)->Option<(MeshTransfer,TessellationReport)> {
        let output=self.take_mesh();let mut owner=self.detach_retirement();let mut wallet=semio_framework_value::AuthoringWallet::new(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10},1_000_000_000_000);
        let mut stall=0;
        while !owner.terminal_is_empty() {
            let(body,demand)={let first=owner.next_demand(0).expect("original tessellation selected demand");(first.copy_bytes,owner.next_demand(first.copy_bytes).expect("original tessellation selected demand"))};
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let Some((turn,_))=wallet.run_settled(grant,1<<20,|effect|owner.step(body,effect)).expect("original cold tessellation retirement")else{panic!("original cold tessellation retirement quote refused")};let step=turn.result.expect("original cold tessellation retirement step");wallet.post(step.progress()).expect("original cold tessellation retirement stock");
            if step.progress()==Default::default(){stall+=1;assert!(stall<64,"cold tessellation retirement stalled without progress: grant={grant:?}");}else{stall=0;}
        }
        output
    }
    /// 📤 Moves completed output while retaining the original job's retirement payloads.
    pub fn take_mesh(&mut self)->Option<(MeshTransfer,TessellationReport)> {
        matches!(self.phase,TessellationPhase::Complete).then(||(std::mem::take(&mut self.transfer),self.report))
    }

    /// 🏁 Drives every remaining unit in one unbudgeted call — the one-shot façade every legacy-free
    /// caller of [`tessellate_solid`]/[`tessellate_face`]/[`tessellate_wire`] goes through.
    pub fn run_to_completion(mut self, body: &Body,input:TessellationInput<'_>) -> Result<(MeshTransfer, TessellationReport), KernelError> {
        loop {
            match self.step(body,input, usize::MAX)? {
                TessellationStep::Done(_) => break,
                TessellationStep::Cancelled(_) => return Err(KernelError::InvalidInput("tessellation cancelled".to_string())),
                TessellationStep::Working(_) => continue,
            }
        }
        self.into_mesh().map(|(mesh, report)| (mesh, report)).ok_or_else(|| KernelError::InvalidInput("tessellation produced no mesh".to_string()))
    }
}
// #endregion ⏱️ResumableJob


fn plane_points(frame: &Frame3) -> Result<[[f32; 3]; 4], ValueError> {
    let points = PLANE_PREVIEW_CORNERS.map(|[u, v]| {
        let p = frame.to_world(Pnt3::new(u, v, 0.0));
        [p.x as f32, p.y as f32, p.z as f32]
    });
    if points.iter().flatten().any(|axis| !axis.is_finite()) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "plane preview corners must be representable"));
    }
    Ok(points)
}

fn write_plane_buffer<T: Copy>(buffer: &mut Vec<T>, data: &[T], grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    buffer.try_reserve_exact(data.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "plane preview buffer reservation failed"))?;
    let receipt = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: buffer.capacity() * size_of::<T>(), ..Default::default() };
    if !receipt.fits(grant) {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "plane preview buffer exceeded its grant").with_retained_progress(receipt));
    }
    buffer.extend_from_slice(data);
    Ok(RetainedCloneProgress { copied_bytes: std::mem::size_of_val(data), ..receipt })
}
