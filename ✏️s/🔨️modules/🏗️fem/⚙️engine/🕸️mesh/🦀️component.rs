//! 🕸️ Meshing: 2D constrained Delaunay triangulation with holes (`PlanarDomain` → `TriMesh2`),
//! structured quad grids, quadratic promotion, and 3D extrusion (wedge/hex) with tet splitting.
//! `spade` (constrained Delaunay + Ruppert refinement) is the only external geometry dependency and
//! NEVER leaks through this module's public API — every public type here is a first-party plain-data
//! struct/enum of `f64`/`u32` so callers never need to import or know about `spade`.

use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};
use spade::handles::FixedVertexHandle;
use spade::{AngleLimit, ConstrainedDelaunayTriangulation, Point2, RefinementParameters, Triangulation};
use std::collections::HashMap;

// #region 🔖️PlanarDomain
/// 📐️ A planar region to mesh: an outer boundary loop and zero or more hole loops, each a closed
/// polygon (points NOT repeating the first point at the end), in consistent (either) winding order.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDomain {
    pub outer: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
}

/// 🕸️ A triangulated mesh: shared node positions plus triangles as index triples into `points`.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh2 {
    pub points: Vec<[f64; 2]>,
    pub tris: Vec<[u32; 3]>,
}

/// ⚙️ Refinement targets for [`triangulate`] — either left at `0.0` to disable that constraint.
#[derive(Clone, Copy, Debug)]
pub struct MeshOpts {
    pub max_edge: f64,
    pub min_angle_deg: f64,
}

/// ⚠️ Everything that can go wrong building a mesh.
#[derive(Debug, thiserror::Error)]
pub enum MeshError {
    #[error("domain has a degenerate outer boundary (fewer than 3 points)")]
    DegenerateDomain,
    #[error("triangulation failed: {0}")]
    TriangulationFailed(String),
}

type Cdt = ConstrainedDelaunayTriangulation<Point2<f64>>;

/// 🧵️ Inserts one closed loop's points as constrained CDT vertices, then constrains consecutive
/// pairs (wrapping around) so the loop's edges survive triangulation/refinement unbroken-in-shape.
fn insert_loop(cdt: &mut Cdt, loop_pts: &[[f64; 2]]) -> Result<(), MeshError> {
    let mut handles = Vec::with_capacity(loop_pts.len());
    for p in loop_pts {
        let handle = cdt.insert(Point2::new(p[0], p[1])).map_err(|e| MeshError::TriangulationFailed(format!("{e:?}")))?;
        handles.push(handle);
    }
    for i in 0..handles.len() {
        let a = handles[i];
        let b = handles[(i + 1) % handles.len()];
        if a != b {
            cdt.add_constraint(a, b);
        }
    }
    Ok(())
}

/// 🎯️ Ray-casting point-in-polygon test (standard even-odd rule; polygon need not be convex).
fn point_in_polygon(point: [f64; 2], polygon: &[[f64; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = (polygon[i][0], polygon[i][1]);
        let (xj, yj) = (polygon[j][0], polygon[j][1]);
        let crosses = (yi > point[1]) != (yj > point[1]);
        if crosses && point[0] < (xj - xi) * (point[1] - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// 🕸️ Constrained Delaunay triangulation of `domain` honoring the outer boundary and hole boundaries
/// as constrained edges, with holes excluded from the output, optionally refined per `opts`.
///
/// Refinement (Ruppert's algorithm via `spade::RefinementParameters`) targets a minimum triangle
/// angle (`opts.min_angle_deg`, disabled when `<= 0.0`) and/or a maximum triangle area derived from
/// `opts.max_edge` via the equilateral-triangle area formula `sqrt(3)/4 * max_edge^2` (disabled when
/// `<= 0.0`). Triangle inside/outside classification happens AFTER refinement, by a local
/// point-in-polygon centroid test — `spade`'s own outer-face exclusion is not relied upon, since a
/// domain with holes has several disjoint constrained loops that classification must handle directly.
pub fn triangulate(domain: &PlanarDomain, opts: &MeshOpts) -> Result<TriMesh2, MeshError> {
    if domain.outer.len() < 3 {
        return Err(MeshError::DegenerateDomain);
    }

    let mut cdt: Cdt = ConstrainedDelaunayTriangulation::new();
    insert_loop(&mut cdt, &domain.outer)?;
    for hole in &domain.holes {
        if hole.len() < 3 {
            return Err(MeshError::DegenerateDomain);
        }
        insert_loop(&mut cdt, hole)?;
    }

    if opts.max_edge > 0.0 || opts.min_angle_deg > 0.0 {
        let mut params = RefinementParameters::<f64>::new();
        if opts.min_angle_deg > 0.0 {
            params = params.with_angle_limit(AngleLimit::from_deg(opts.min_angle_deg));
        }
        if opts.max_edge > 0.0 {
            let max_area = (3f64.sqrt() / 4.0) * opts.max_edge * opts.max_edge;
            params = params.with_max_allowed_area(max_area);
        }
        cdt.refine(params);
    }

    let mut point_index: HashMap<(u64, u64), u32> = HashMap::new();
    let mut points: Vec<[f64; 2]> = Vec::new();
    let mut tris: Vec<[u32; 3]> = Vec::new();

    for face in cdt.inner_faces() {
        let verts = face.vertices();
        let positions: [[f64; 2]; 3] = std::array::from_fn(|i| {
            let p = verts[i].position();
            [p.x, p.y]
        });
        let centroid = [(positions[0][0] + positions[1][0] + positions[2][0]) / 3.0, (positions[0][1] + positions[1][1] + positions[2][1]) / 3.0];

        let mut outside = !point_in_polygon(centroid, &domain.outer);
        if !outside {
            outside = domain.holes.iter().any(|hole| point_in_polygon(centroid, hole));
        }
        if outside {
            continue;
        }

        let mut idxs = [0u32; 3];
        for k in 0..3 {
            let key = (positions[k][0].to_bits(), positions[k][1].to_bits());
            let idx = *point_index.entry(key).or_insert_with(|| {
                points.push(positions[k]);
                (points.len() - 1) as u32
            });
            idxs[k] = idx;
        }
        tris.push(idxs);
    }

    Ok(TriMesh2 { points, tris })
}
// #endregion 🔖️PlanarDomain

// #region 🧵️IncrementalMeshJob
/// 🎚️ The fidelity of a [`MeshJobPreview`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshQualityTier {
    Coarse,
    Refined,
    Final,
}

/// 🗺️ A replaceable, non-authoritative mesh overlay published while [`MeshJob`] is running.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshJobPreview {
    pub sequence: u64,
    pub tier: MeshQualityTier,
    pub refinement_steps: usize,
    pub mesh: TriMesh2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MeshJobStage {
    Validate,
    InsertBoundary,
    ConstrainBoundary,
    Classify,
    Refine,
    Finalize,
    Complete,
}

const MESH_JOB_UNIT_BATCH: usize = 8;

/// 🧵️ Persistent constrained-mesh state machine. Boundary insertion, constraint creation, face
/// classification and refinement are cursor-resumable. Refinement grants the current triangulator at
/// most one additional vertex per call; the public job and deterministic payload remain unchanged when
/// the owned Bowyer-Watson implementation replaces that internal seam.
pub struct MeshJob {
    operation: Operation,
    domain: PlanarDomain,
    options: MeshOpts,
    cdt: Cdt,
    handles: Vec<Vec<FixedVertexHandle>>,
    stage: MeshJobStage,
    loop_cursor: usize,
    point_cursor: usize,
    edge_cursor: usize,
    face_cursor: usize,
    after_classify: MeshJobStage,
    preview_tier: MeshQualityTier,
    mesh: TriMesh2,
    point_index: HashMap<(u64, u64), u32>,
    refinement_steps: usize,
    max_refinement_steps: usize,
}

impl MeshJob {
    /// 🌱️ Creates a deterministic mesh operation from an immutable domain snapshot.
    pub fn new(domain: PlanarDomain, options: MeshOpts, operation: Operation) -> Self {
        let loop_count = 1 + domain.holes.len();
        let input_points = domain.outer.len() + domain.holes.iter().map(Vec::len).sum::<usize>();
        Self {
            operation,
            domain,
            options,
            cdt: ConstrainedDelaunayTriangulation::new(),
            handles: vec![Vec::new(); loop_count],
            stage: MeshJobStage::Validate,
            loop_cursor: 0,
            point_cursor: 0,
            edge_cursor: 0,
            face_cursor: 0,
            after_classify: MeshJobStage::Finalize,
            preview_tier: MeshQualityTier::Coarse,
            mesh: TriMesh2 { points: Vec::new(), tris: Vec::new() },
            point_index: HashMap::new(),
            refinement_steps: 0,
            max_refinement_steps: input_points.saturating_mul(10).max(1),
        }
    }

    /// 🗺️ The latest complete replaceable overlay; authoritative callers commit only the final outcome.
    pub fn preview(&self) -> MeshJobPreview {
        MeshJobPreview { sequence: self.operation.preview_sequence, tier: self.preview_tier, refinement_steps: self.refinement_steps, mesh: self.mesh.clone() }
    }

    fn loop_points(&self, index: usize) -> &[[f64; 2]] {
        if index == 0 {
            &self.domain.outer
        } else {
            &self.domain.holes[index - 1]
        }
    }

    fn needs_refinement(&self) -> bool {
        self.options.max_edge > 0.0 || self.options.min_angle_deg > 0.0
    }

    fn begin_classification(&mut self, tier: MeshQualityTier, after: MeshJobStage) {
        self.mesh.points.clear();
        self.mesh.tris.clear();
        self.point_index.clear();
        self.face_cursor = 0;
        self.preview_tier = tier;
        self.after_classify = after;
        self.stage = MeshJobStage::Classify;
    }

    fn refinement_parameters(&self) -> RefinementParameters<f64> {
        let mut parameters = RefinementParameters::new().with_max_additional_vertices(1);
        if self.options.min_angle_deg > 0.0 {
            parameters = parameters.with_angle_limit(AngleLimit::from_deg(self.options.min_angle_deg));
        }
        if self.options.max_edge > 0.0 {
            parameters = parameters.with_max_allowed_area((3f64.sqrt() / 4.0) * self.options.max_edge * self.options.max_edge);
        }
        parameters
    }

    fn append_face(&mut self, positions: [[f64; 2]; 3]) {
        let centroid = [(positions[0][0] + positions[1][0] + positions[2][0]) / 3.0, (positions[0][1] + positions[1][1] + positions[2][1]) / 3.0];
        if !point_in_polygon(centroid, &self.domain.outer) || self.domain.holes.iter().any(|hole| point_in_polygon(centroid, hole)) {
            return;
        }
        let mut indices = [0; 3];
        for (slot, position) in positions.into_iter().enumerate() {
            let key = (position[0].to_bits(), position[1].to_bits());
            indices[slot] = *self.point_index.entry(key).or_insert_with(|| {
                self.mesh.points.push(position);
                (self.mesh.points.len() - 1) as u32
            });
        }
        self.mesh.tris.push(indices);
    }

    fn encode_preview(&mut self, context: &mut StepContext<'_>) -> Vec<u8> {
        let sequence = context.next_preview_sequence();
        self.operation.preview_sequence = sequence + 1;
        self.encode_mesh(sequence, false)
    }

    fn encode_mesh(&self, sequence: u64, complete: bool) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(42 + self.mesh.points.len() * 16 + self.mesh.tris.len() * 12);
        bytes.extend_from_slice(b"FEMMESH1");
        bytes.push(match self.preview_tier {
            MeshQualityTier::Coarse => 0,
            MeshQualityTier::Refined => 1,
            MeshQualityTier::Final => 2,
        });
        bytes.push(u8::from(complete));
        bytes.extend_from_slice(&sequence.to_le_bytes());
        bytes.extend_from_slice(&(self.refinement_steps as u64).to_le_bytes());
        bytes.extend_from_slice(&(self.mesh.points.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&(self.mesh.tris.len() as u64).to_le_bytes());
        for point in &self.mesh.points {
            bytes.extend_from_slice(&point[0].to_bits().to_le_bytes());
            bytes.extend_from_slice(&point[1].to_bits().to_le_bytes());
        }
        for triangle in &self.mesh.tris {
            for index in triangle {
                bytes.extend_from_slice(&index.to_le_bytes());
            }
        }
        bytes
    }

    fn fail(message: impl Into<Vec<u8>>) -> StepOutcome {
        StepOutcome::Fault(JobFault { detail: message.into() })
    }
}

impl InteractiveJob for MeshJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Self::fail(b"stale-mesh-operation".to_vec());
        }
        context.set_stage(match self.stage {
            MeshJobStage::Validate => "validate-references",
            MeshJobStage::InsertBoundary => "insert-boundary",
            MeshJobStage::ConstrainBoundary => "constrain-boundary",
            MeshJobStage::Classify => "classify-elements",
            MeshJobStage::Refine => "refine-quality",
            MeshJobStage::Finalize => "finalize-mesh",
            MeshJobStage::Complete => "complete",
        });
        match self.stage {
            MeshJobStage::Validate => {
                if self.domain.outer.len() < 3 || self.domain.holes.iter().any(|hole| hole.len() < 3) {
                    return Self::fail(b"degenerate-planar-domain".to_vec());
                }
                self.stage = MeshJobStage::InsertBoundary;
                context.consume_fuel(1);
                StepOutcome::Yield
            }
            MeshJobStage::InsertBoundary => {
                let mut units = 0;
                while self.loop_cursor < self.handles.len() && units < MESH_JOB_UNIT_BATCH && !context.should_yield() {
                    let point_count = self.loop_points(self.loop_cursor).len();
                    if self.point_cursor == point_count {
                        self.loop_cursor += 1;
                        self.point_cursor = 0;
                        continue;
                    }
                    let point = self.loop_points(self.loop_cursor)[self.point_cursor];
                    let handle = match self.cdt.insert(Point2::new(point[0], point[1])) {
                        Ok(handle) => handle,
                        Err(error) => return Self::fail(format!("triangulation insertion failed: {error:?}").into_bytes()),
                    };
                    self.handles[self.loop_cursor].push(handle);
                    self.point_cursor += 1;
                    units += 1;
                    context.consume_fuel(1);
                    if context.is_cancelled() {
                        return StepOutcome::Cancelled;
                    }
                }
                if self.loop_cursor == self.handles.len() {
                    self.loop_cursor = 0;
                    self.edge_cursor = 0;
                    self.stage = MeshJobStage::ConstrainBoundary;
                }
                StepOutcome::Yield
            }
            MeshJobStage::ConstrainBoundary => {
                let mut units = 0;
                while self.loop_cursor < self.handles.len() && units < MESH_JOB_UNIT_BATCH && !context.should_yield() {
                    let handles = &self.handles[self.loop_cursor];
                    if self.edge_cursor == handles.len() {
                        self.loop_cursor += 1;
                        self.edge_cursor = 0;
                        continue;
                    }
                    let a = handles[self.edge_cursor];
                    let b = handles[(self.edge_cursor + 1) % handles.len()];
                    if a != b {
                        self.cdt.add_constraint(a, b);
                    }
                    self.edge_cursor += 1;
                    units += 1;
                    context.consume_fuel(1);
                }
                if self.loop_cursor == self.handles.len() {
                    let after = if self.needs_refinement() { MeshJobStage::Refine } else { MeshJobStage::Finalize };
                    self.begin_classification(MeshQualityTier::Coarse, after);
                }
                StepOutcome::Yield
            }
            MeshJobStage::Classify => {
                let face_count = self.cdt.num_inner_faces();
                let mut positions = Vec::new();
                for handle in self.cdt.fixed_inner_faces().skip(self.face_cursor).take(MESH_JOB_UNIT_BATCH) {
                    let vertices = self.cdt.face(handle).vertices();
                    positions.push(std::array::from_fn(|index| {
                        let point = vertices[index].position();
                        [point.x, point.y]
                    }));
                }
                for face in positions {
                    self.append_face(face);
                    self.face_cursor += 1;
                    context.consume_fuel(1);
                    if context.is_cancelled() {
                        return StepOutcome::Cancelled;
                    }
                    if context.should_yield() {
                        break;
                    }
                }
                if self.face_cursor >= face_count {
                    self.stage = self.after_classify;
                    return StepOutcome::PreviewReady(self.encode_preview(context));
                }
                StepOutcome::Yield
            }
            MeshJobStage::Refine => {
                if context.should_yield() {
                    return StepOutcome::Yield;
                }
                let before = self.cdt.num_vertices();
                let parameters = self.refinement_parameters();
                let result = self.cdt.refine(parameters);
                let after = self.cdt.num_vertices();
                self.refinement_steps += after.saturating_sub(before);
                context.consume_fuel(1);
                let final_pass = result.refinement_complete || after == before || self.refinement_steps >= self.max_refinement_steps;
                if final_pass {
                    self.begin_classification(MeshQualityTier::Final, MeshJobStage::Finalize);
                } else if self.refinement_steps % 8 == 0 {
                    self.begin_classification(MeshQualityTier::Refined, MeshJobStage::Refine);
                }
                StepOutcome::Yield
            }
            MeshJobStage::Finalize => {
                self.preview_tier = MeshQualityTier::Final;
                self.stage = MeshJobStage::Complete;
                let state = self.encode_mesh(self.operation.preview_sequence, true);
                StepOutcome::CheckpointReady(Checkpoint { applied_progress: self.mesh.tris.len() as u64, state })
            }
            MeshJobStage::Complete => {
                let output = self.encode_mesh(self.operation.preview_sequence, true);
                StepOutcome::Complete(CommitCandidate { state: output.clone(), output })
            }
        }
    }
}
// #endregion 🧵️IncrementalMeshJob

// #region 🔖️QuadMesh2
/// 🔲️ A structured quad mesh: shared node positions plus quads as index quadruples into `points`.
#[derive(Clone, Debug, PartialEq)]
pub struct QuadMesh2 {
    pub points: Vec<[f64; 2]>,
    pub quads: Vec<[u32; 4]>,
}

/// 🔲️ An `nx` x `ny` structured grid of quads over an axis-aligned rectangle `[x0,x1] x [y0,y1]`,
/// row-major point numbering, each quad wound `[bottom-left, bottom-right, top-right, top-left]`.
pub fn quad_grid(x0: f64, y0: f64, x1: f64, y1: f64, nx: usize, ny: usize) -> QuadMesh2 {
    let mut points = Vec::with_capacity((nx + 1) * (ny + 1));
    for j in 0..=ny {
        for i in 0..=nx {
            let x = x0 + (x1 - x0) * (i as f64) / (nx as f64);
            let y = y0 + (y1 - y0) * (j as f64) / (ny as f64);
            points.push([x, y]);
        }
    }
    let index = |i: usize, j: usize| (j * (nx + 1) + i) as u32;
    let mut quads = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            quads.push([index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
        }
    }
    QuadMesh2 { points, quads }
}
// #endregion 🔖️QuadMesh2

// #region 🔖️Quadratic
/// 🔺️ A quadratic-promoted triangle mesh: shared node positions (originals first, then appended
/// mid-edge points) plus 6-node triangles.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh2Quadratic {
    pub points: Vec<[f64; 2]>,
    pub tris6: Vec<[u32; 6]>,
}

/// 🔗️ Looks up (or creates, welding shared edges to exactly one mid-node) the mid-edge point index
/// for edge `(a,b)`, keyed by the sorted `(min,max)` index pair.
fn mid_index(a: u32, b: u32, points: &mut Vec<[f64; 2]>, edge_mid: &mut HashMap<(u32, u32), u32>) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(&idx) = edge_mid.get(&key) {
        return idx;
    }
    let pa = points[a as usize];
    let pb = points[b as usize];
    let idx = points.len() as u32;
    points.push([(pa[0] + pb[0]) * 0.5, (pa[1] + pb[1]) * 0.5]);
    edge_mid.insert(key, idx);
    idx
}

/// 🔺️ Promotes a linear `TriMesh2` to quadratic by inserting a mid-edge node per unique edge (shared
/// edges between adjacent triangles get exactly ONE mid-node, deduped by sorted `(min,max)` edge key).
/// Original points keep their indices unchanged; mid-edge points are appended after them. Each
/// triangle's 6 node indices follow `[n0,n1,n2, mid(n0,n1), mid(n1,n2), mid(n2,n0)]` — the standard
/// Tri6 convention (matches `elements2d.rs`'s `shape_tri6` node ordering, documented here since that
/// function may land concurrently with this module).
pub fn to_quadratic(mesh: &TriMesh2) -> TriMesh2Quadratic {
    let mut points = mesh.points.clone();
    let mut edge_mid: HashMap<(u32, u32), u32> = HashMap::new();
    let mut tris6 = Vec::with_capacity(mesh.tris.len());

    for tri in &mesh.tris {
        let [n0, n1, n2] = *tri;
        let m01 = mid_index(n0, n1, &mut points, &mut edge_mid);
        let m12 = mid_index(n1, n2, &mut points, &mut edge_mid);
        let m20 = mid_index(n2, n0, &mut points, &mut edge_mid);
        tris6.push([n0, n1, n2, m01, m12, m20]);
    }

    TriMesh2Quadratic { points, tris6 }
}
// #endregion 🔖️Quadratic

// #region 🔖️VolumeMesh
/// 🧱️ One volumetric cell — a linear wedge/hex prism or a tet, as index tuples into `VolumeMesh::points`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Wedge6([u32; 6]),
    Hex8([u32; 8]),
    Tet4([u32; 4]),
}

/// 🧱️ A volumetric mesh: shared node positions plus cells.
#[derive(Clone, Debug, PartialEq)]
pub struct VolumeMesh {
    pub points: Vec<[f64; 3]>,
    pub cells: Vec<Cell>,
}

/// 🧱️ Extrudes a flat `TriMesh2` (lying in the z=0 plane) along +z by `height`, split into `layers`
/// equal-height layers, producing one `Wedge6` per (triangle, layer) — node order
/// `[bottom0,bottom1,bottom2, top0,top1,top2]` (bottom face matches the triangle's own `[n0,n1,n2]`
/// winding, top face directly above).
pub fn extrude_tri_mesh(mesh: &TriMesh2, height: f64, layers: usize) -> VolumeMesh {
    let layers = layers.max(1);
    let n = mesh.points.len();
    let mut points = Vec::with_capacity(n * (layers + 1));
    for l in 0..=layers {
        let z = height * (l as f64) / (layers as f64);
        for p in &mesh.points {
            points.push([p[0], p[1], z]);
        }
    }
    let mut cells = Vec::with_capacity(mesh.tris.len() * layers);
    for l in 0..layers {
        let bottom_off = (l * n) as u32;
        let top_off = ((l + 1) * n) as u32;
        for tri in &mesh.tris {
            let [a, b, c] = *tri;
            cells.push(Cell::Wedge6([bottom_off + a, bottom_off + b, bottom_off + c, top_off + a, top_off + b, top_off + c]));
        }
    }
    VolumeMesh { points, cells }
}

/// 🧱️ Extrudes a flat `QuadMesh2` along +z by `height` into `layers` layers of `Hex8` cells — node
/// order `[bottom0,bottom1,bottom2,bottom3, top0,top1,top2,top3]` matching the quad's own winding.
pub fn extrude_quad_mesh(mesh: &QuadMesh2, height: f64, layers: usize) -> VolumeMesh {
    let layers = layers.max(1);
    let n = mesh.points.len();
    let mut points = Vec::with_capacity(n * (layers + 1));
    for l in 0..=layers {
        let z = height * (l as f64) / (layers as f64);
        for p in &mesh.points {
            points.push([p[0], p[1], z]);
        }
    }
    let mut cells = Vec::with_capacity(mesh.quads.len() * layers);
    for l in 0..layers {
        let bottom_off = (l * n) as u32;
        let top_off = ((l + 1) * n) as u32;
        for quad in &mesh.quads {
            let [a, b, c, d] = *quad;
            cells.push(Cell::Hex8([bottom_off + a, bottom_off + b, bottom_off + c, bottom_off + d, top_off + a, top_off + b, top_off + c, top_off + d]));
        }
    }
    VolumeMesh { points, cells }
}

/// ✂️ Splits a quad face `[a,b,c,d]` (in winding order, so `a`-`c` and `b`-`d` are the two diagonals)
/// into 2 triangles, choosing the diagonal FROM the corner with the smallest global point index. This
/// depends only on the face's own 4 global indices (not on cell/apex choice), so two cells sharing a
/// quad face always agree — the parity-consistency guarantee `split_to_tets` relies on.
fn split_quad_face(a: u32, b: u32, c: u32, d: u32) -> [[u32; 3]; 2] {
    let min = a.min(b).min(c).min(d);
    if min == a || min == c {
        [[a, b, c], [a, c, d]]
    } else {
        [[a, b, d], [b, c, d]]
    }
}

/// 🔺️ Fan-triangulates a convex cell's boundary from `apex` (one of the cell's own vertices): every
/// quad face is split via [`split_quad_face`], every triangular face passes through as-is, and every
/// resulting boundary triangle that does NOT already contain `apex` becomes a tet `(apex, t0, t1, t2)`
/// — the standard star/cone decomposition of a convex polyhedron, valid since convexity guarantees no
/// overlap/gaps. Faces touching `apex` need no explicit tet: their volume is degenerate (zero) from
/// `apex`'s own cone and is instead captured as internal faces of tets from adjacent, non-apex faces.
fn split_cell_to_tets(quad_faces: &[[u32; 4]], tri_faces: &[[u32; 3]], apex: u32) -> Vec<[u32; 4]> {
    let mut tets = Vec::new();
    for &[a, b, c, d] in quad_faces {
        for tri in split_quad_face(a, b, c, d) {
            if !tri.contains(&apex) {
                tets.push([apex, tri[0], tri[1], tri[2]]);
            }
        }
    }
    for &tri in tri_faces {
        if !tri.contains(&apex) {
            tets.push([apex, tri[0], tri[1], tri[2]]);
        }
    }
    tets
}

/// 🔪️ Splits every `Wedge6`/`Hex8` cell into `Tet4` cells (`Wedge6` → 3 tets, `Hex8` → 6 tets), using
/// the minimum-global-node-index apex + face-diagonal rule so adjacent cells split their SHARED quad
/// faces identically (see [`split_quad_face`]). `Tet4` cells in the input pass through unchanged.
pub fn split_to_tets(mesh: &VolumeMesh) -> VolumeMesh {
    let mut cells = Vec::with_capacity(mesh.cells.len());
    for cell in &mesh.cells {
        match cell {
            Cell::Tet4(t) => cells.push(Cell::Tet4(*t)),
            Cell::Wedge6(w) => {
                let [n0, n1, n2, n3, n4, n5] = *w;
                let apex = w.iter().copied().min().unwrap();
                let quads = [[n0, n1, n4, n3], [n1, n2, n5, n4], [n2, n0, n3, n5]];
                let tris = [[n0, n1, n2], [n3, n4, n5]];
                for tet in split_cell_to_tets(&quads, &tris, apex) {
                    cells.push(Cell::Tet4(tet));
                }
            }
            Cell::Hex8(h) => {
                let [n0, n1, n2, n3, n4, n5, n6, n7] = *h;
                let apex = h.iter().copied().min().unwrap();
                let quads = [[n0, n1, n2, n3], [n4, n5, n6, n7], [n0, n1, n5, n4], [n1, n2, n6, n5], [n2, n3, n7, n6], [n3, n0, n4, n7]];
                for tet in split_cell_to_tets(&quads, &[], apex) {
                    cells.push(Cell::Tet4(tet));
                }
            }
        }
    }
    VolumeMesh { points: mesh.points.clone(), cells }
}

/// 🧭️ The average of `mesh.points` at `idxs` — shared by `boundary_faces`'s per-tet and per-face
/// centroid computations.
fn point_centroid(mesh: &VolumeMesh, idxs: &[u32]) -> [f64; 3] {
    let mut c = [0.0; 3];
    for &i in idxs {
        let p = mesh.points[i as usize];
        for k in 0..3 {
            c[k] += p[k];
        }
    }
    let n = idxs.len() as f64;
    [c[0] / n, c[1] / n, c[2] / n]
}

/// 🧱️ Every triangular face belonging to EXACTLY ONE `Tet4` cell — the mesh's outer surface (call
/// [`split_to_tets`] first if `mesh` still has `Wedge6`/`Hex8` cells; those contribute no faces here).
/// Each returned triangle is independently wound so its `cross(edge0,edge1)` normal points AWAY from
/// its own tet's centroid (outward) — determined per-tet via a centroid side-test, so the result
/// doesn't depend on any input node-order convention. Used by `fem_3d`'s solid mesh preview/rendering.
pub fn boundary_faces(mesh: &VolumeMesh) -> Vec<[u32; 3]> {
    let mut counts: HashMap<[u32; 3], usize> = HashMap::new();
    let mut oriented: HashMap<[u32; 3], [u32; 3]> = HashMap::new();

    for cell in &mesh.cells {
        let Cell::Tet4(t) = cell else { continue };
        let [n0, n1, n2, n3] = *t;
        let tet_centroid = point_centroid(mesh, &[n0, n1, n2, n3]);

        for face in [[n0, n1, n2], [n0, n1, n3], [n0, n2, n3], [n1, n2, n3]] {
            let mut key = face;
            key.sort_unstable();
            *counts.entry(key).or_insert(0) += 1;

            let p = |i: u32| mesh.points[i as usize];
            let (a, b, c) = (p(face[0]), p(face[1]), p(face[2]));
            let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let normal = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            let face_centroid = point_centroid(mesh, &face);
            let to_tet = [tet_centroid[0] - face_centroid[0], tet_centroid[1] - face_centroid[1], tet_centroid[2] - face_centroid[2]];
            let dot = normal[0] * to_tet[0] + normal[1] * to_tet[1] + normal[2] * to_tet[2];
            let outward = if dot > 0.0 { [face[0], face[2], face[1]] } else { face };
            oriented.insert(key, outward);
        }
    }

    counts.into_iter().filter(|(_, count)| *count == 1).map(|(key, _)| oriented[&key]).collect()
}
// #endregion 🔖️VolumeMesh

// #region 🔖️Quality
/// 📊️ Cheap mesh sanity report — interior angle bounds (2D) and inverted-cell detection (3D).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QualityReport {
    pub min_angle_deg: f64,
    pub max_angle_deg: f64,
    pub min_jacobian_sign_positive: bool,
    pub element_count: usize,
}

/// 📐️ Interior angle at `p`, between the edges to `prev` and `next`, in degrees.
fn angle_at(prev: [f64; 2], p: [f64; 2], next: [f64; 2]) -> f64 {
    let v1 = [prev[0] - p[0], prev[1] - p[1]];
    let v2 = [next[0] - p[0], next[1] - p[1]];
    let dot = v1[0] * v2[0] + v1[1] * v2[1];
    let n1 = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
    let n2 = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
    let cos_a = (dot / (n1 * n2)).clamp(-1.0, 1.0);
    cos_a.acos().to_degrees()
}

/// 📊️ Min/max interior angle across all triangles; `min_jacobian_sign_positive` mirrors the 2D
/// analogue of the 3D check — true iff every triangle's signed area (shoelace, `[n0,n1,n2]` order) is
/// positive, i.e. consistently wound.
pub fn tri_mesh_quality(mesh: &TriMesh2) -> QualityReport {
    let mut min_angle = f64::INFINITY;
    let mut max_angle = f64::NEG_INFINITY;
    let mut all_positive = true;
    for tri in &mesh.tris {
        let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
        for i in 0..3 {
            let a = angle_at(p[(i + 2) % 3], p[i], p[(i + 1) % 3]);
            min_angle = min_angle.min(a);
            max_angle = max_angle.max(a);
        }
        let signed_area = 0.5 * ((p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[2][0] - p[0][0]) * (p[1][1] - p[0][1]));
        if signed_area <= 0.0 {
            all_positive = false;
        }
    }
    if mesh.tris.is_empty() {
        min_angle = 0.0;
        max_angle = 0.0;
    }
    QualityReport { min_angle_deg: min_angle, max_angle_deg: max_angle, min_jacobian_sign_positive: all_positive, element_count: mesh.tris.len() }
}

/// 🧮️ Signed tet volume via the scalar triple product of edge vectors from `p0`.
fn tet_signed_volume(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> f64 {
    let a = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let b = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let c = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
    let cross = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    (cross[0] * c[0] + cross[1] * c[1] + cross[2] * c[2]) / 6.0
}

/// 🧮️ Signed volume of one cell, via a FIXED (purely local-index-based, not global-min-based like
/// [`split_to_tets`]) tet decomposition — so the sign faithfully reflects whether the cell's own
/// documented local node order (`extrude_tri_mesh`/`extrude_quad_mesh`'s convention) is right-handed,
/// independent of the cell's global point indices. Verified by hand against a unit right prism and a
/// unit cube (both give the expected positive volume for correctly-ordered nodes).
fn cell_signed_volume(points: &[[f64; 3]], cell: &Cell) -> f64 {
    let p = |i: u32| points[i as usize];
    match cell {
        Cell::Tet4([a, b, c, d]) => tet_signed_volume(p(*a), p(*b), p(*c), p(*d)),
        Cell::Wedge6(n) => tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[3])) + tet_signed_volume(p(n[1]), p(n[2]), p(n[3]), p(n[4])) + tet_signed_volume(p(n[2]), p(n[3]), p(n[4]), p(n[5])),
        Cell::Hex8(n) => {
            tet_signed_volume(p(n[0]), p(n[4]), p(n[5]), p(n[6]))
                + tet_signed_volume(p(n[0]), p(n[4]), p(n[6]), p(n[7]))
                + tet_signed_volume(p(n[0]), p(n[1]), p(n[2]), p(n[6]))
                + tet_signed_volume(p(n[0]), p(n[1]), p(n[6]), p(n[5]))
                + tet_signed_volume(p(n[0]), p(n[2]), p(n[3]), p(n[7]))
                + tet_signed_volume(p(n[0]), p(n[2]), p(n[7]), p(n[6]))
        }
    }
}

/// 📊️ `min_jacobian_sign_positive` is true iff every cell's signed volume is positive — a negative
/// signed volume flags inverted/degenerate connectivity. Angle bounds are a 2D-only concept and are
/// left at `0.0` here.
pub fn volume_mesh_quality(mesh: &VolumeMesh) -> QualityReport {
    let all_positive = mesh.cells.iter().all(|cell| cell_signed_volume(&mesh.points, cell) > 0.0);
    QualityReport { min_angle_deg: 0.0, max_angle_deg: 0.0, min_jacobian_sign_positive: all_positive, element_count: mesh.cells.len() }
}
// #endregion 🔖️Quality

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::{root_cancel_token, Generation, OperationId, RevisionId, StepBudget};
    use std::collections::HashSet;
    use std::time::{Duration, Instant};

    fn mesh_operation() -> Operation {
        Operation::new(OperationId(700), RevisionId(11), Generation(3), 19)
    }

    fn drive_mesh_job(mut job: MeshJob) -> (Vec<u8>, usize, Duration) {
        fn now() -> u64 {
            0
        }
        let cancel = root_cancel_token();
        let mut sequence = 0;
        let mut previews = 0;
        let mut worst = Duration::ZERO;
        for _ in 0..10_000 {
            let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(64, 10), cancel.clone(), now, &mut sequence);
            let started = Instant::now();
            let outcome = job.step(&mut context);
            worst = worst.max(started.elapsed());
            match outcome {
                StepOutcome::PreviewReady(_) => previews += 1,
                StepOutcome::Complete(candidate) => return (candidate.output, previews, worst),
                StepOutcome::Yield | StepOutcome::CheckpointReady(_) => {}
                other => panic!("mesh job failed: {other:?}"),
            }
        }
        panic!("mesh job did not complete")
    }

    fn shoelace_area(points: &[[f64; 2]]) -> f64 {
        let mut sum = 0.0;
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            sum += a[0] * b[1] - b[0] * a[1];
        }
        (sum * 0.5).abs()
    }

    fn tri_area(mesh: &TriMesh2, tri: &[u32; 3]) -> f64 {
        shoelace_area(&[mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]])
    }

    fn total_area(mesh: &TriMesh2) -> f64 {
        mesh.tris.iter().map(|t| tri_area(mesh, t)).sum()
    }

    fn no_refine() -> MeshOpts {
        MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }
    }

    fn square(side: f64) -> Vec<[f64; 2]> {
        vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]]
    }

    #[semio_framework_async_macros::async_test]
    async fn triangulate_square_area_matches_input() {
        let outer = square(10.0);
        let expected = shoelace_area(&outer);
        let domain = PlanarDomain { outer, holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        assert!(!mesh.tris.is_empty());
        assert!((total_area(&mesh) - expected).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn triangulate_respects_hole_area() {
        let outer = square(10.0);
        let hole = vec![[3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]];
        let domain = PlanarDomain { outer, holes: vec![hole.clone()] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let expected = 100.0 - 16.0;
        assert!((total_area(&mesh) - expected).abs() < 1e-6, "area={}", total_area(&mesh));
        for tri in &mesh.tris {
            let p0 = mesh.points[tri[0] as usize];
            let p1 = mesh.points[tri[1] as usize];
            let p2 = mesh.points[tri[2] as usize];
            let centroid = [(p0[0] + p1[0] + p2[0]) / 3.0, (p0[1] + p1[1] + p2[1]) / 3.0];
            assert!(!point_in_polygon(centroid, &hole));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn triangulate_honors_constrained_boundary_edges() {
        // L-shape: non-convex outer boundary.
        let outer = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [5.0, 5.0], [5.0, 10.0], [0.0, 10.0]];
        let domain = PlanarDomain { outer: outer.clone(), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");

        let key = |p: [f64; 2]| (p[0].to_bits(), p[1].to_bits());
        let mut edge_set: HashSet<((u64, u64), (u64, u64))> = HashSet::new();
        for tri in &mesh.tris {
            let p = [mesh.points[tri[0] as usize], mesh.points[tri[1] as usize], mesh.points[tri[2] as usize]];
            for i in 0..3 {
                let a = key(p[i]);
                let b = key(p[(i + 1) % 3]);
                let edge = if a <= b { (a, b) } else { (b, a) };
                edge_set.insert(edge);
            }
        }

        for i in 0..outer.len() {
            let a = key(outer[i]);
            let b = key(outer[(i + 1) % outer.len()]);
            let edge = if a <= b { (a, b) } else { (b, a) };
            assert!(edge_set.contains(&edge), "boundary edge {i} missing from triangulation");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn refined_mesh_respects_min_angle() {
        // A long thin rectangle: all INPUT corners are 90 degrees (refinable), but the single
        // diagonal edge spade's initial CDT picks to fill it naturally produces slivers absent
        // refinement — Ruppert refinement can freely add Steiner points to fix that, unlike a sharp
        // INPUT corner angle (which no amount of edge splitting can widen).
        let outer = vec![[0.0, 0.0], [20.0, 0.0], [20.0, 1.0], [0.0, 1.0]];
        let domain = PlanarDomain { outer, holes: vec![] };
        let opts = MeshOpts { max_edge: 1.0, min_angle_deg: 25.0 };
        let mesh = triangulate(&domain, &opts).expect("triangulates");
        let quality = tri_mesh_quality(&mesh);
        let epsilon = 2.0; // Ruppert refinement guarantees are best-effort/asymptotic, not exact.
        assert!(quality.min_angle_deg >= opts.min_angle_deg - epsilon, "min_angle={}", quality.min_angle_deg);
    }

    #[semio_framework_async_macros::async_test]
    async fn quad_grid_has_expected_topology() {
        let mesh = quad_grid(0.0, 0.0, 3.0, 2.0, 3, 2);
        assert_eq!(mesh.quads.len(), 6);
        assert_eq!(mesh.points.len(), 12);
        assert_eq!(mesh.points[0], [0.0, 0.0]);
        assert_eq!(mesh.points[3], [3.0, 0.0]);
        assert_eq!(mesh.points[11], [3.0, 2.0]);
        assert_eq!(mesh.quads[0], [0, 1, 5, 4]);
        assert_eq!(mesh.quads[5], [6, 7, 11, 10]);
    }

    #[semio_framework_async_macros::async_test]
    async fn to_quadratic_welds_shared_edges() {
        let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        assert!(mesh.tris.len() >= 2);

        let mut unique_edges: HashSet<(u32, u32)> = HashSet::new();
        for tri in &mesh.tris {
            for &(a, b) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                unique_edges.insert(if a < b { (a, b) } else { (b, a) });
            }
        }

        let quadratic = to_quadratic(&mesh);
        let new_points = quadratic.points.len() - mesh.points.len();
        assert_eq!(new_points, unique_edges.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn extrude_tri_mesh_volume_matches_area_times_height() {
        let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let area = total_area(&mesh);
        let height = 3.0;
        let volume_mesh = extrude_tri_mesh(&mesh, height, 2);
        let tets = split_to_tets(&volume_mesh);
        let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
        assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
    }

    #[semio_framework_async_macros::async_test]
    async fn extrude_quad_mesh_volume_matches_area_times_height() {
        let mesh = quad_grid(0.0, 0.0, 4.0, 3.0, 4, 3);
        let area = 12.0;
        let height = 2.5;
        let volume_mesh = extrude_quad_mesh(&mesh, height, 3);
        let tets = split_to_tets(&volume_mesh);
        let total: f64 = tets.cells.iter().map(|c| cell_signed_volume(&tets.points, c).abs()).sum();
        assert!((total - area * height).abs() < 1e-6, "total={} expected={}", total, area * height);
    }

    #[semio_framework_async_macros::async_test]
    async fn split_to_tets_preserves_volume() {
        let domain = PlanarDomain { outer: square(4.0), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let wedge_mesh = extrude_tri_mesh(&mesh, 2.0, 2);
        let pre_wedge: f64 = wedge_mesh.cells.iter().map(|c| cell_signed_volume(&wedge_mesh.points, c).abs()).sum();
        let post_wedge = split_to_tets(&wedge_mesh);
        let post_wedge_total: f64 = post_wedge.cells.iter().map(|c| cell_signed_volume(&post_wedge.points, c).abs()).sum();
        assert!((pre_wedge - post_wedge_total).abs() < 1e-9);

        let quad_mesh = quad_grid(0.0, 0.0, 4.0, 4.0, 2, 2);
        let hex_mesh = extrude_quad_mesh(&quad_mesh, 2.0, 2);
        let pre_hex: f64 = hex_mesh.cells.iter().map(|c| cell_signed_volume(&hex_mesh.points, c).abs()).sum();
        let post_hex = split_to_tets(&hex_mesh);
        let post_hex_total: f64 = post_hex.cells.iter().map(|c| cell_signed_volume(&post_hex.points, c).abs()).sum();
        assert!((pre_hex - post_hex_total).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn split_to_tets_shared_faces_are_parity_consistent() {
        // Two Hex8 cells sharing the quad face [1,2,6,5] (cell A's +x face / cell B's -x face).
        let points = vec![
            [0.0, 0.0, 0.0], // 0
            [1.0, 0.0, 0.0], // 1
            [1.0, 1.0, 0.0], // 2
            [0.0, 1.0, 0.0], // 3
            [0.0, 0.0, 1.0], // 4
            [1.0, 0.0, 1.0], // 5
            [1.0, 1.0, 1.0], // 6
            [0.0, 1.0, 1.0], // 7
            [2.0, 0.0, 0.0], // 8
            [2.0, 1.0, 0.0], // 9
            [2.0, 0.0, 1.0], // 10
            [2.0, 1.0, 1.0], // 11
        ];
        let cell_a = Cell::Hex8([0, 1, 2, 3, 4, 5, 6, 7]);
        let cell_b = Cell::Hex8([1, 8, 9, 2, 5, 10, 11, 6]);
        let shared_face: HashSet<u32> = [1, 2, 6, 5].into_iter().collect();

        let mesh_a = VolumeMesh { points: points.clone(), cells: vec![cell_a] };
        let mesh_b = VolumeMesh { points: points.clone(), cells: vec![cell_b] };
        let tets_a = split_to_tets(&mesh_a);
        let tets_b = split_to_tets(&mesh_b);

        let face_triangles = |vm: &VolumeMesh| -> HashSet<[u32; 3]> {
            let mut out = HashSet::new();
            for cell in &vm.cells {
                if let Cell::Tet4(t) = cell {
                    let faces = [[t[0], t[1], t[2]], [t[0], t[1], t[3]], [t[0], t[2], t[3]], [t[1], t[2], t[3]]];
                    for mut f in faces {
                        if f.iter().all(|v| shared_face.contains(v)) {
                            f.sort_unstable();
                            out.insert(f);
                        }
                    }
                }
            }
            out
        };

        let from_a = face_triangles(&tets_a);
        let from_b = face_triangles(&tets_b);
        assert_eq!(from_a.len(), 2, "expected the shared quad face split into 2 triangles from cell A");
        assert_eq!(from_a, from_b, "shared face must split identically from both cells");
    }

    #[semio_framework_async_macros::async_test]
    async fn volume_mesh_quality_detects_inverted_cell() {
        let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let good = VolumeMesh { points: points.clone(), cells: vec![Cell::Tet4([0, 1, 2, 3])] };
        assert!(volume_mesh_quality(&good).min_jacobian_sign_positive);

        // Swap two nodes to invert the signed volume.
        let inverted = VolumeMesh { points, cells: vec![Cell::Tet4([1, 0, 2, 3])] };
        assert!(!volume_mesh_quality(&inverted).min_jacobian_sign_positive);
    }

    /// 🧱️ A `side`x`side` square extruded `height` tall, 1 layer, split to tets — `boundary_faces`'s
    /// total triangle area must equal the analytic box surface `2*side² + 4*side*height` (top + bottom
    /// + 4 sides), which also confirms every internal (shared, appears-twice) face was excluded.
    #[semio_framework_async_macros::async_test]
    async fn boundary_faces_area_matches_extruded_box_surface() {
        let side = 4.0;
        let height = 3.0;
        let domain = PlanarDomain { outer: square(side), holes: vec![] };
        let mesh = triangulate(&domain, &no_refine()).expect("triangulates");
        let volume_mesh = extrude_tri_mesh(&mesh, height, 1);
        let tets = split_to_tets(&volume_mesh);

        let faces = boundary_faces(&tets);
        let tri_area = |f: &[u32; 3]| -> f64 {
            let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
            let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let cross = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
        };
        let total_area: f64 = faces.iter().map(tri_area).sum();
        let expected = 2.0 * side * side + 4.0 * side * height;
        assert!((total_area - expected).abs() < 1e-6, "total={total_area} expected={expected}");

        // Every boundary face must be wound so its normal points away from its own tet's centroid —
        // spot-checked here on the bottom face (z=0, outward normal must have negative z).
        for f in &faces {
            if tets.points[f[0] as usize][2] < 1e-9 && tets.points[f[1] as usize][2] < 1e-9 && tets.points[f[2] as usize][2] < 1e-9 {
                let (a, b, c) = (tets.points[f[0] as usize], tets.points[f[1] as usize], tets.points[f[2] as usize]);
                let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
                let normal_z = e0[0] * e1[1] - e0[1] * e1[0];
                assert!(normal_z < 0.0, "bottom face normal should point outward (-z), got normal_z={normal_z}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn triangulate_rejects_degenerate_outer_boundary() {
        let domain = PlanarDomain { outer: vec![[0.0, 0.0], [1.0, 0.0]], holes: vec![] };
        match triangulate(&domain, &no_refine()) {
            Err(MeshError::DegenerateDomain) => {}
            other => panic!("expected DegenerateDomain, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn triangulate_rejects_degenerate_hole() {
        let domain = PlanarDomain { outer: square(10.0), holes: vec![vec![[3.0, 3.0], [4.0, 4.0]]] };
        match triangulate(&domain, &no_refine()) {
            Err(MeshError::DegenerateDomain) => {}
            other => panic!("expected DegenerateDomain, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn point_in_polygon_returns_false_for_degenerate_polygon() {
        assert!(!point_in_polygon([0.0, 0.0], &[]));
        assert!(!point_in_polygon([0.0, 0.0], &[[0.0, 0.0], [1.0, 0.0]]));
    }

    /// 📊️ `tri_mesh_quality` flags a clockwise-wound (negative signed area) triangle via
    /// `min_jacobian_sign_positive`, and reports `0.0` angle bounds for an empty mesh instead of the
    /// unhelpful `f64::INFINITY`/`NEG_INFINITY` an empty min/max fold would otherwise leave behind.
    #[semio_framework_async_macros::async_test]
    async fn tri_mesh_quality_detects_inverted_winding_and_handles_empty_mesh() {
        let ccw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 1, 2]] };
        assert!(tri_mesh_quality(&ccw).min_jacobian_sign_positive);

        let cw = TriMesh2 { points: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], tris: vec![[0, 2, 1]] };
        assert!(!tri_mesh_quality(&cw).min_jacobian_sign_positive);

        let empty = TriMesh2 { points: vec![], tris: vec![] };
        let quality = tri_mesh_quality(&empty);
        assert_eq!(quality.min_angle_deg, 0.0);
        assert_eq!(quality.max_angle_deg, 0.0);
        assert_eq!(quality.element_count, 0);
    }

    /// 🔺️ A `Cell::Tet4` already present in the input `VolumeMesh` passes through `split_to_tets`
    /// completely unchanged — the only cell kind besides `Wedge6`/`Hex8` `split_to_tets` accepts.
    #[semio_framework_async_macros::async_test]
    async fn split_to_tets_passes_through_existing_tet4_cells() {
        let points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let tet = Cell::Tet4([0, 1, 2, 3]);
        let mesh = VolumeMesh { points, cells: vec![tet] };
        let result = split_to_tets(&mesh);
        assert_eq!(result.cells.len(), 1);
        match result.cells[0] {
            Cell::Tet4(nodes) => assert_eq!(nodes, [0, 1, 2, 3]),
            _ => panic!("expected the Tet4 cell to pass through unchanged"),
        }
    }

    #[test]
    fn mesh_job_is_previewing_deterministic_and_step_bounded() {
        let domain = PlanarDomain { outer: square(8.0), holes: vec![square(2.0).into_iter().map(|point| [point[0] + 3.0, point[1] + 3.0]).collect()] };
        let options = MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 };
        let first = drive_mesh_job(MeshJob::new(domain.clone(), options, mesh_operation()));
        let second = drive_mesh_job(MeshJob::new(domain, options, mesh_operation()));
        assert_eq!(first.0, second.0);
        assert!(first.1 > 0);
        assert!(first.2 < Duration::from_millis(8), "worst mesh job step was {:?}", first.2);
    }

    #[test]
    fn mesh_job_observes_cancellation_before_mutating() {
        fn now() -> u64 {
            0
        }
        let operation = mesh_operation();
        let mut job = MeshJob::new(PlanarDomain { outer: square(2.0), holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, operation);
        let cancel = root_cancel_token();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(64, 10), cancel, now, &mut sequence);
        assert_eq!(job.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(job.stage, MeshJobStage::Validate);
        assert_eq!(job.cdt.num_vertices(), 0);
    }

    #[test]
    fn mesh_job_large_boundary_never_runs_to_completion_in_one_step() {
        let boundary = (0..1_024)
            .map(|index| {
                let angle = index as f64 * std::f64::consts::TAU / 1_024.0;
                [angle.cos() * 100.0, angle.sin() * 100.0]
            })
            .collect();
        let (_, previews, worst) = drive_mesh_job(MeshJob::new(PlanarDomain { outer: boundary, holes: vec![] }, MeshOpts { max_edge: 0.0, min_angle_deg: 0.0 }, mesh_operation()));
        assert!(previews > 0);
        assert!(worst < Duration::from_millis(8), "worst large-boundary mesh job step was {worst:?}");
    }
}
// #endregion 🔖️Tests
