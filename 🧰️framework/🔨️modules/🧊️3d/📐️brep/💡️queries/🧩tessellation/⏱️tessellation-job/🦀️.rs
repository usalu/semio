//! ⏱️ Tessellation job: the resumable tessellation phases, progress, steps and input, the topology cursor and the tessellation job with its stepping. See [`super`].

use super::*;

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
pub enum TessellationInput<'a> { Vertex(VertexId), Solid(SolidId), Shell(ShellId), Face(FaceId), Wire(&'a [(EdgeId,bool)]), Solids(&'a [SolidId]) }

#[derive(Clone, Copy, Default)]
struct TopologyCursor { root:usize, pub(super) solid:Option<SolidId>,shell_index:usize,pub(super) shell:Option<ShellId>,face_index:usize,pub(super) face:Option<FaceId>,loop_index:usize,pub(super) ring:Option<(CoedgeId,CoedgeId,usize)>,pub(super) done:usize }

pub struct TessellationJob {
    pub(super) topology:TopologyCursor,
    pub(super) deflection: f64,
    pub(super) edge_order: Vec<EdgeId>,
    pub(super) edge_cache: EdgeSampleCache,
    pub(super) vertices: HistoryFoldIndex<VertexId, ()>,
    standalone_vertex: Option<VertexId>,
    vertex_units_done: usize,
    pub(super) faces: Vec<FaceId>,
    edge_cursor: usize,
    face_cursor: usize,
    pack_cursor: usize,
    pub(super) transfer: MeshTransfer,
    pub(super) report: TessellationReport,
    pub(super) phase: TessellationPhase,
}

semio_framework_value::artifact_retire_leaf!(TopologyCursor, TessellationReport, TessellationPhase);

semio_framework_value::artifact_retire_struct!(TessellationJob { topology, deflection, edge_order, edge_cache, vertices, standalone_vertex, vertex_units_done, faces, edge_cursor, face_cursor, pack_cursor, transfer, report, phase });

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
    fn seeded(deflection:f64,edge_order:Vec<EdgeId>,faces:Vec<FaceId>)->Self {
        Self {topology:TopologyCursor::default(),deflection:deflection.max(1e-9),edge_cache:EdgeSampleCache::new(),vertices:Default::default(),standalone_vertex:None,vertex_units_done:0,edge_order,faces,edge_cursor:0,face_cursor:0,pack_cursor:0,transfer:Default::default(),report:Default::default(),phase:TessellationPhase::CollectingTopology}
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
            units_done: self.topology.done + self.edge_cursor + self.face_cursor + self.pack_cursor + self.vertex_units_done,
            units_total: self.units_total(),
            faces_done: self.face_cursor,
            faces_total: self.faces.len(),
            phase: self.phase,
        }
    }

    fn units_total(&self) -> usize {
        self.topology.done + usize::from(self.phase==TessellationPhase::CollectingTopology) + self.edge_order.len() + self.faces.len() + self.edge_order.len() + self.vertex_units_done + usize::from(self.standalone_vertex.is_some())
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
        let output=self.take_mesh();let mut owner=self.detach_retirement();
        while !owner.terminal_is_empty() {
            let copy=owner.next_copy_byte_demand().expect("original tessellation copy demand");
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).expect("original tessellation capacity demand"),maximum_release_bytes:owner.next_release_byte_demand().expect("original tessellation release demand"),maximum_depth:owner.next_depth_demand().expect("original tessellation depth demand")};
            owner.step(grant).expect("original cold tessellation retirement");
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
