//! 🧵️ The incremental `MeshJob`: stages, cursors, constraint recovery, classification and publication.

use super::*;

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
pub(super) enum MeshJobStage {
    Validate,
    ReservePreparation,
    ReservePreparationIndex,
    ReservePreparationConstraints,
    PreparationFailed,
    PrepareInput,
    CountInput,
    Initialize,
    InsertBoundary,
    ReserveEdgeAuthorities,
    IndexEdges,
    ConstrainBoundary,
    ReservePointIndex,
    ReserveMeshPoints,
    ReserveMeshTriangles,
    Classify,
    PublishPreview,
    Finalize,
    PublishCheckpoint,
    Complete,
    Published,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum ConstraintRecoveryStage {
    ReserveConstraintWorkspace,
    IndexTriangleEdge,
    SearchConstraintEdge,
    ClassifyIntersection,
    SelectDeterministicFlip,
    ValidateFlip,
    ApplyFlip,
    RetireFormerEdge,
    PublishConstraintProgress,
    ConstraintComplete,
}

#[derive(Clone, Copy)]
pub(super) struct IndexedConstraintEdge {
    pub(super) edge: Edge,
    pub(super) adjacent: [Option<usize>; 2],
    pub(super) active: bool,
    pub(super) fixed: bool,
}

#[derive(Clone, Copy)]
struct ConstraintFlipCandidate {
    pub(super) edge: Edge,
    pub(super) adjacent: [usize; 2],
    replacement: [[usize; 3]; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaceClassificationStage {
    Begin,
    OuterEdge,
    HoleEdge,
    PointLookup,
    PointInsert,
    TrianglePublish,
}

#[derive(Clone, Copy)]
struct FaceClassificationCursor {
    pub(super) stage: FaceClassificationStage,
    positions: [[f64; 2]; 3],
    centroid: [f64; 2],
    pub(super) hole: usize,
    pub(super) edge: usize,
    inside: bool,
    slot: usize,
    lookup: usize,
    indices: [u32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MeshPublicationKind {
    Preview,
    Checkpoint,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum MeshPayloadStage {
    Magic,
    Tier,
    Complete,
    Sequence,
    Refinement,
    PointCount,
    TriangleCount,
    PointCoordinate,
    TriangleIndex,
    CommitPage,
    Done,
}

#[derive(Clone, Copy, Debug)]
struct MeshPayloadCursor {
    pub(super) stage: MeshPayloadStage,
    pub(super) point: usize,
    pub(super) coordinate: usize,
    pub(super) triangle: usize,
    pub(super) index: usize,
}

/// 🧵️ Persistent constrained-mesh state machine. Bowyer-Watson insertion retains scan, cavity
/// compaction, and fan cursors; boundary recovery and face classification retain independent cursors.
/// Only the completed deterministic payload is authoritative.
pub struct MeshJob {
    pub(super) operation: Operation,
    pub(super) domain: MeshDomainOwner,
    pub(super) options: MeshOpts,
    pub(super) preparation: Option<MeshInputPreparation>,
    pub(super) prepared_input: Option<(Vec<[f64; 2]>, Vec<Edge>)>,
    pub(super) triangulation: Option<OwnedTriangulation>,
    pub(super) constraints: Vec<Edge>,
    pub(super) indexed_constraint_edges: Vec<IndexedConstraintEdge>,
    pub(super) edge_index_cursor: usize,
    pub(super) edge_index_local_cursor: usize,
    pub(super) edge_index_lookup_cursor: usize,
    pub(super) edge_index_vacancy: Option<usize>,
    pub(super) edge_index_candidate: Option<(Edge, usize)>,
    pub(super) edge_index_valid: bool,
    pub(super) stage: MeshJobStage,
    pub(super) constraint_cursor: usize,
    pub(super) constraint_stage: ConstraintRecoveryStage,
    pub(super) constraint_search_cursor: usize,
    constraint_adjacency_cursor: usize,
    constraint_candidate: Option<(Edge, usize)>,
    constraint_flip: Option<ConstraintFlipCandidate>,
    constraint_flip_count: usize,
    constraint_flip_limit: usize,
    pub(super) constraint_apply_cursor: usize,
    pub(super) constraint_retire_cursor: usize,
    pub(super) constraint_retire_adjacency_cursor: usize,
    constraint_reindex_cursor: usize,
    pub(super) face_cursor: usize,
    face_classification: Option<FaceClassificationCursor>,
    pub(super) input_count: usize,
    pub(super) input_count_hole: usize,
    pub(super) preview_tier: MeshQualityTier,
    pub(super) mesh: TriMesh2,
    pub(super) point_index: Vec<((u64, u64), u32)>,
    pub(super) refinement_steps: usize,
    pub(super) validation_hole_cursor: usize,
    pub(super) publication_writer: Option<RetainedJobPayloadWriter>,
    pub(super) publication_kind: Option<MeshPublicationKind>,
    pub(super) publication_cursor: MeshPayloadCursor,
    publication_sequence: u64,
    pub(super) lent: Option<MeshLent>,
    pub(super) lent_delivered: bool,
    pub(super) pending_run: Option<MeshRun>,
    pub(super) markers: MeshOutcomeMarkers,
    pub(super) close_lane: u8,
    pub(super) maximum_points: usize,
    pub(super) maximum_triangles: usize,
}

/// 🧭️ What one internal step decided before any outcome is lent to the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MeshRun {
    Yield,
    Cancelled,
    Fault,
    Preview,
    Checkpoint(u64),
    Complete,
    Published,
}

/// 🎁️ One finished publication payload, owned by the job until the caller acknowledged the outcome that lent it.
struct MeshLent {
    pub(super) payload: RetainedJobPayload,
}

/// 🏷️ Empty payloads lent where an outcome carries no mesh bytes.
struct MeshOutcomeMarkers {
    pub(super) state: RetainedJobPayload,
    pub(super) output: RetainedJobPayload,
    pub(super) fault: RetainedJobPayload,
}

impl Default for MeshOutcomeMarkers {
    fn default() -> Self {
        Self { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput), fault: RetainedJobPayload::empty(JobPayloadStream::Fault) }
    }
}

pub(super) fn mesh_covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

impl MeshJob {
    /// 🌱️ Creates a deterministic mesh operation from an immutable domain snapshot.
    pub fn new(domain: PlanarDomain, options: MeshOpts, operation: Operation) -> Self {
        Self::from_domain(MeshDomainOwner::Dynamic(domain), options, operation)
    }

    fn from_domain(domain: MeshDomainOwner, options: MeshOpts, operation: Operation) -> Self {
        Self {
            operation,
            domain,
            options,
            preparation: None,
            prepared_input: None,
            triangulation: None,
            constraints: Vec::new(),
            indexed_constraint_edges: Vec::new(),
            edge_index_cursor: 0,
            edge_index_local_cursor: 0,
            edge_index_lookup_cursor: 0,
            edge_index_vacancy: None,
            edge_index_candidate: None,
            edge_index_valid: false,
            stage: MeshJobStage::Validate,
            constraint_cursor: 0,
            constraint_stage: ConstraintRecoveryStage::ReserveConstraintWorkspace,
            constraint_search_cursor: 0,
            constraint_adjacency_cursor: 0,
            constraint_candidate: None,
            constraint_flip: None,
            constraint_flip_count: 0,
            constraint_flip_limit: 0,
            constraint_apply_cursor: 0,
            constraint_retire_cursor: 0,
            constraint_retire_adjacency_cursor: 0,
            constraint_reindex_cursor: 0,
            face_cursor: 0,
            face_classification: None,
            input_count: 0,
            input_count_hole: 0,
            preview_tier: MeshQualityTier::Coarse,
            mesh: TriMesh2 { points: Vec::new(), tris: Vec::new() },
            point_index: Vec::new(),
            refinement_steps: 0,
            validation_hole_cursor: 0,
            publication_writer: None,
            publication_kind: None,
            publication_cursor: MeshPayloadCursor { stage: MeshPayloadStage::Magic, point: 0, coordinate: 0, triangle: 0, index: 0 },
            publication_sequence: 0,
            lent: None,
            lent_delivered: false,
            pending_run: None,
            markers: MeshOutcomeMarkers::default(),
            close_lane: 0,
            maximum_points: usize::MAX,
            maximum_triangles: usize::MAX,
        }
    }

    /// 🔒️ Creates a mesh operation whose retained point/triangle owners cannot grow past
    /// the caller's already-admitted fixed process credits.
    pub fn new_bounded(domain: PlanarDomain, options: MeshOpts, operation: Operation, maximum_points: usize, maximum_triangles: usize) -> Self {
        let mut job = Self::new(domain, options, operation);
        job.maximum_points = maximum_points;
        job.maximum_triangles = maximum_triangles;
        job
    }

    /// 🧱 Creates a bounded mesh operation by transferring an already-admitted fixed domain owner.
    pub fn new_mounted_bounded(domain: MountedPlanarDomain, options: MeshOpts, operation: Operation, maximum_points: usize, maximum_triangles: usize) -> Self {
        let mut job = Self::from_domain(MeshDomainOwner::Mounted(domain), options, operation);
        job.maximum_points = maximum_points;
        job.maximum_triangles = maximum_triangles;
        job
    }

    /// 🔭 Borrows one completed point without transferring the mesh backing.
    pub fn completed_point(&self, index: usize) -> Option<[f64; 2]> {
        (self.stage == MeshJobStage::Published).then(|| self.mesh.points.get(index).copied()).flatten()
    }

    /// 🔭 Borrows one completed triangle without transferring the mesh backing.
    pub fn completed_triangle(&self, index: usize) -> Option<[u32; 3]> {
        (self.stage == MeshJobStage::Published).then(|| self.mesh.tris.get(index).copied()).flatten()
    }

    pub fn completed_counts(&self) -> Option<(usize, usize)> {
        (self.stage == MeshJobStage::Published).then_some((self.mesh.points.len(), self.mesh.tris.len()))
    }

    /// 🧹️ Retires one exact mesh/domain owner per governed close opportunity.
    pub(super) fn retire_owner_turn(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        loop {
            let released = match self.close_lane {
                0 => match &mut self.domain {
                    MeshDomainOwner::Dynamic(domain) => match close_vec_owner_step(&mut domain.outer, maximum_bytes) {
                        Ok(Some(step)) => step,
                        Err(()) => return (false, 0, 0),
                        Ok(None) => {
                            self.close_lane += 1;
                            continue;
                        }
                    },
                    MeshDomainOwner::Mounted(domain) => {
                        if domain.close_step() {
                            self.close_lane += 2;
                            continue;
                        }
                        (1, 0)
                    }
                },
                1 => {
                    let MeshDomainOwner::Dynamic(domain) = &mut self.domain else {
                        self.close_lane += 1;
                        continue;
                    };
                    if let Some(hole) = domain.holes.last_mut() {
                        match close_vec_owner_step(hole, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        domain.holes.pop();
                        (1, 0)
                    } else {
                        match close_vec_owner_step(&mut domain.holes, maximum_bytes) {
                            Ok(Some(step)) => step,
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {
                                self.close_lane += 1;
                                continue;
                            }
                        }
                    }
                }
                2 => {
                    if let Some(preparation) = self.preparation.as_mut() {
                        match close_vec_owner_step(&mut preparation.points, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut preparation.constraints, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut preparation.point_indices, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.preparation = None;
                        (1, 0)
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                3 => {
                    if let Some((points, edges)) = self.prepared_input.as_mut() {
                        match close_vec_owner_step(points, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(edges, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.prepared_input = None;
                        (1, 0)
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                4 => {
                    if let Some(triangulation) = self.triangulation.as_mut() {
                        if let Some(insertion) = triangulation.insertion.as_mut() {
                            match close_vec_owner_step(&mut insertion.bad, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut insertion.boundary, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut insertion.retained, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            triangulation.insertion = None;
                            (1, 0)
                        } else {
                            match close_vec_owner_step(&mut triangulation.points, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut triangulation.triangles, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            match close_vec_owner_step(&mut triangulation.insertion_order, maximum_bytes) {
                                Ok(Some(step)) => return (false, step.0, step.1),
                                Err(()) => return (false, 0, 0),
                                Ok(None) => {}
                            }
                            self.triangulation = None;
                            (1, 0)
                        }
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                5 => match close_vec_owner_step(&mut self.constraints, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                6 => match close_vec_owner_step(&mut self.mesh.points, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                7 => match close_vec_owner_step(&mut self.mesh.tris, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                8 => match close_vec_owner_step(&mut self.point_index, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                9 => match close_vec_owner_step(&mut self.indexed_constraint_edges, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                _ => return (true, 0, 0),
            };
            return (false, released.0, released.1);
        }
    }

    /// 🗺️ The latest complete replaceable overlay; authoritative callers commit only the final outcome.
    pub fn preview(&self) -> MeshJobPreview {
        MeshJobPreview { sequence: self.operation.preview_sequence, tier: self.preview_tier, refinement_steps: self.refinement_steps, mesh: self.mesh.clone() }
    }

    /// 🧵️ Moves the completed mesh into its retained model-construction child. A false
    /// terminal leaves the exact owner in this job.
    pub fn take_completed_mesh(&mut self) -> Option<TriMesh2> {
        (self.stage == MeshJobStage::Published).then(|| std::mem::replace(&mut self.mesh, TriMesh2 { points: Vec::new(), tris: Vec::new() }))
    }

    pub(super) fn begin_classification(&mut self, tier: MeshQualityTier) {
        self.face_cursor = 0;
        self.face_classification = None;
        self.preview_tier = tier;
        self.stage = MeshJobStage::Classify;
    }

    fn advance_polygon_edge(point: [f64; 2], polygon: &[[f64; 2]], edge: usize, inside: &mut bool) -> bool {
        if edge == polygon.len() {
            return true;
        }
        let current = polygon[edge];
        let previous = polygon[(edge + polygon.len() - 1) % polygon.len()];
        if ((current[1] > point[1]) != (previous[1] > point[1])) && point[0] < (previous[0] - current[0]) * (point[1] - current[1]) / (previous[1] - current[1]) + current[0] {
            *inside = !*inside;
        }
        false
    }

    pub(super) fn advance_face_classification(&mut self) -> Result<bool, ()> {
        if self.face_classification.is_none() {
            let triangulation = self.triangulation.as_ref().ok_or(())?;
            let Some(triangle) = triangulation.triangles.get(self.face_cursor) else { return Ok(true) };
            let positions = std::array::from_fn(|index| triangulation.points[triangle[index]]);
            let centroid = [(positions[0][0] + positions[1][0] + positions[2][0]) / 3.0, (positions[0][1] + positions[1][1] + positions[2][1]) / 3.0];
            self.face_classification = Some(FaceClassificationCursor { stage: FaceClassificationStage::Begin, positions, centroid, hole: 0, edge: 0, inside: false, slot: 0, lookup: 0, indices: [0; 3] });
            return Ok(false);
        }
        let cursor = self.face_classification.as_mut().ok_or(())?;
        match cursor.stage {
            FaceClassificationStage::Begin => cursor.stage = FaceClassificationStage::OuterEdge,
            FaceClassificationStage::OuterEdge => {
                if Self::advance_polygon_edge(cursor.centroid, self.domain.outer(), cursor.edge, &mut cursor.inside) {
                    if !cursor.inside {
                        self.face_classification = None;
                        self.face_cursor += 1;
                        return Ok(false);
                    }
                    cursor.edge = 0;
                    cursor.inside = false;
                    cursor.stage = FaceClassificationStage::HoleEdge;
                } else {
                    cursor.edge += 1;
                }
            }
            FaceClassificationStage::HoleEdge => {
                if let Some(hole) = self.domain.hole(cursor.hole) {
                    if Self::advance_polygon_edge(cursor.centroid, hole, cursor.edge, &mut cursor.inside) {
                        if cursor.inside {
                            self.face_classification = None;
                            self.face_cursor += 1;
                            return Ok(false);
                        }
                        cursor.hole += 1;
                        cursor.edge = 0;
                        cursor.inside = false;
                    } else {
                        cursor.edge += 1;
                    }
                } else {
                    cursor.stage = FaceClassificationStage::PointLookup;
                }
            }
            FaceClassificationStage::PointLookup => {
                let position = cursor.positions[cursor.slot];
                let key = (position[0].to_bits(), position[1].to_bits());
                if let Some((current, index)) = self.point_index.get(cursor.lookup) {
                    if *current == key {
                        cursor.indices[cursor.slot] = *index;
                        cursor.slot += 1;
                        cursor.lookup = 0;
                    } else {
                        cursor.lookup += 1;
                    }
                } else {
                    cursor.stage = FaceClassificationStage::PointInsert;
                }
                if cursor.slot == 3 {
                    cursor.stage = FaceClassificationStage::TrianglePublish;
                }
            }
            FaceClassificationStage::PointInsert => {
                if self.mesh.points.len() == self.maximum_points {
                    return Err(());
                }
                let position = cursor.positions[cursor.slot];
                let key = (position[0].to_bits(), position[1].to_bits());
                self.mesh.points.push(position);
                let index = (self.mesh.points.len() - 1) as u32;
                self.point_index.push((key, index));
                cursor.indices[cursor.slot] = index;
                cursor.slot += 1;
                cursor.lookup = 0;
                cursor.stage = if cursor.slot == 3 { FaceClassificationStage::TrianglePublish } else { FaceClassificationStage::PointLookup };
            }
            FaceClassificationStage::TrianglePublish => {
                if self.mesh.tris.len() == self.maximum_triangles {
                    return Err(());
                }
                self.mesh.tris.push(cursor.indices);
                self.face_classification = None;
                self.face_cursor += 1;
            }
        }
        Ok(false)
    }

    fn advance_mesh_payload(&mut self) -> Result<bool, JobPayloadAdmissionFault> {
        let writer = self.publication_writer.as_mut().ok_or(JobPayloadAdmissionFault::WriterSealed)?;
        let mut scalar = [0; 8];
        let length = match self.publication_cursor.stage {
            MeshPayloadStage::Magic => {
                scalar.copy_from_slice(b"FEMMESH1");
                Some(8)
            }
            MeshPayloadStage::Tier => {
                scalar[0] = match self.preview_tier {
                    MeshQualityTier::Coarse => 0,
                    MeshQualityTier::Refined => 1,
                    MeshQualityTier::Final => 2,
                };
                Some(1)
            }
            MeshPayloadStage::Complete => {
                scalar[0] = u8::from(self.publication_kind != Some(MeshPublicationKind::Preview));
                Some(1)
            }
            MeshPayloadStage::Sequence => {
                scalar = self.publication_sequence.to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::Refinement => {
                scalar = (self.refinement_steps as u64).to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::PointCount => {
                scalar = (self.mesh.points.len() as u64).to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::TriangleCount => {
                scalar = (self.mesh.tris.len() as u64).to_le_bytes();
                Some(8)
            }
            MeshPayloadStage::PointCoordinate => self.mesh.points.get(self.publication_cursor.point).map(|point| {
                scalar = point[self.publication_cursor.coordinate].to_bits().to_le_bytes();
                8
            }),
            MeshPayloadStage::TriangleIndex => self.mesh.tris.get(self.publication_cursor.triangle).map(|triangle| {
                scalar[..4].copy_from_slice(&triangle[self.publication_cursor.index].to_le_bytes());
                4
            }),
            MeshPayloadStage::CommitPage | MeshPayloadStage::Done => None,
        };
        if let Some(length) = length {
            if writer.staged_page_remaining() < length {
                writer.commit_staged_page()?;
                return Ok(false);
            }
            writer.write_staged(&scalar[..length])?;
        }
        self.publication_cursor.stage = match self.publication_cursor.stage {
            MeshPayloadStage::Magic => MeshPayloadStage::Tier,
            MeshPayloadStage::Tier => MeshPayloadStage::Complete,
            MeshPayloadStage::Complete => MeshPayloadStage::Sequence,
            MeshPayloadStage::Sequence => MeshPayloadStage::Refinement,
            MeshPayloadStage::Refinement => MeshPayloadStage::PointCount,
            MeshPayloadStage::PointCount => MeshPayloadStage::TriangleCount,
            MeshPayloadStage::TriangleCount if self.mesh.points.is_empty() => MeshPayloadStage::TriangleIndex,
            MeshPayloadStage::TriangleCount => MeshPayloadStage::PointCoordinate,
            MeshPayloadStage::PointCoordinate => {
                self.publication_cursor.coordinate += 1;
                if self.publication_cursor.coordinate == 2 {
                    self.publication_cursor.coordinate = 0;
                    self.publication_cursor.point += 1;
                }
                if self.publication_cursor.point == self.mesh.points.len() {
                    MeshPayloadStage::TriangleIndex
                } else {
                    MeshPayloadStage::PointCoordinate
                }
            }
            MeshPayloadStage::TriangleIndex => {
                if self.publication_cursor.triangle == self.mesh.tris.len() {
                    MeshPayloadStage::CommitPage
                } else {
                    self.publication_cursor.index += 1;
                    if self.publication_cursor.index == 3 {
                        self.publication_cursor.index = 0;
                        self.publication_cursor.triangle += 1;
                    }
                    if self.publication_cursor.triangle == self.mesh.tris.len() {
                        MeshPayloadStage::CommitPage
                    } else {
                        MeshPayloadStage::TriangleIndex
                    }
                }
            }
            MeshPayloadStage::CommitPage => {
                writer.commit_staged_page()?;
                MeshPayloadStage::Done
            }
            MeshPayloadStage::Done => MeshPayloadStage::Done,
        };
        Ok(self.publication_cursor.stage == MeshPayloadStage::Done)
    }

    pub(super) fn advance_publication(&mut self, context: &mut StepContext<'_>, kind: MeshPublicationKind) -> Result<bool, ()> {
        if self.publication_kind.is_none() {
            let sequence = if kind == MeshPublicationKind::Preview {
                let sequence = context.next_preview_sequence().map_err(|_| ())?;
                self.operation.preview_sequence = sequence + 1;
                sequence
            } else {
                self.operation.preview_sequence
            };
            self.publication_kind = Some(kind);
            self.publication_sequence = sequence;
            self.publication_cursor = MeshPayloadCursor { stage: MeshPayloadStage::Magic, point: 0, coordinate: 0, triangle: 0, index: 0 };
            return Ok(false);
        }
        if self.publication_kind != Some(kind) {
            return Err(());
        }
        if self.publication_writer.is_none() {
            let stream = match kind {
                MeshPublicationKind::Preview => JobPayloadStream::Preview,
                MeshPublicationKind::Checkpoint => JobPayloadStream::CheckpointState,
                MeshPublicationKind::Complete => JobPayloadStream::CommitOutput,
            };
            self.publication_writer = Some(RetainedJobPayloadWriter::new(stream));
            return Ok(false);
        }
        let writer = self.publication_writer.as_mut().ok_or(())?;
        if writer.staged_page_len().is_none() && self.publication_cursor.stage != MeshPayloadStage::Done {
            writer.begin_staged_page(context).map_err(|_| ())?;
            return Ok(false);
        }
        if !self.advance_mesh_payload().map_err(|_| ())? {
            return Ok(false);
        }
        let writer = self.publication_writer.take().ok_or(())?;
        let payload = writer.finish().map_err(|writer| {
            self.publication_writer = Some(writer);
        })?;
        self.publication_kind = None;
        self.lent = Some(MeshLent { payload });
        Ok(true)
    }

    pub(super) fn fail(_message: impl Into<Vec<u8>>) -> MeshRun {
        MeshRun::Fault
    }

    pub(super) fn begin_edge_index_candidate(&mut self, triangle: usize, local: usize) -> bool {
        let Some(indices) = self.triangulation.as_ref().and_then(|triangulation| triangulation.triangles.get(triangle)).copied() else {
            return false;
        };
        let edge = match local {
            0 => Edge::new(indices[0], indices[1]),
            1 => Edge::new(indices[1], indices[2]),
            _ => Edge::new(indices[2], indices[0]),
        };
        self.edge_index_candidate = Some((edge, triangle));
        self.edge_index_lookup_cursor = 0;
        self.edge_index_vacancy = None;
        true
    }

    pub(super) fn advance_edge_index_candidate(&mut self) -> Result<bool, ()> {
        let Some((edge, triangle)) = self.edge_index_candidate else {
            return Ok(true);
        };
        if self.edge_index_lookup_cursor < self.indexed_constraint_edges.len() {
            let index = self.edge_index_lookup_cursor;
            let slot = &mut self.indexed_constraint_edges[index];
            self.edge_index_lookup_cursor += 1;
            if !slot.active {
                self.edge_index_vacancy.get_or_insert(index);
            } else if slot.edge == edge {
                if slot.adjacent[0] != Some(triangle) && slot.adjacent[1] != Some(triangle) {
                    if slot.adjacent[0].is_none() {
                        slot.adjacent[0] = Some(triangle);
                    } else if slot.adjacent[1].is_none() {
                        slot.adjacent[1] = Some(triangle);
                    } else {
                        return Err(());
                    }
                }
                self.edge_index_candidate = None;
                return Ok(true);
            }
            return Ok(false);
        }
        let entry = IndexedConstraintEdge { edge, adjacent: [Some(triangle), None], active: true, fixed: false };
        if let Some(index) = self.edge_index_vacancy {
            self.indexed_constraint_edges[index] = entry;
        } else if self.indexed_constraint_edges.len() < self.indexed_constraint_edges.capacity() {
            self.indexed_constraint_edges.push(entry);
        } else {
            return Err(());
        }
        self.edge_index_candidate = None;
        Ok(true)
    }

    pub(super) fn advance_constraint_recovery(&mut self) -> Result<bool, MeshError> {
        let constraint = self.constraints[self.constraint_cursor];
        match self.constraint_stage {
            ConstraintRecoveryStage::ReserveConstraintWorkspace => {
                self.constraint_flip_limit = self.triangulation.as_ref().map_or(0, |triangulation| triangulation.triangles.len().saturating_mul(triangulation.triangles.len()).max(1));
                self.constraint_flip_count = 0;
                self.constraint_search_cursor = 0;
                self.constraint_stage = ConstraintRecoveryStage::SearchConstraintEdge;
            }
            ConstraintRecoveryStage::IndexTriangleEdge => {
                let adjacent = self.constraint_flip.map(|candidate| candidate.adjacent).ok_or_else(|| MeshError::TriangulationFailed("missing retained flip owner".into()))?;
                if self.edge_index_candidate.is_none() {
                    if self.constraint_reindex_cursor == 6 {
                        self.constraint_search_cursor = 0;
                        self.constraint_stage = ConstraintRecoveryStage::PublishConstraintProgress;
                        return Ok(false);
                    }
                    let triangle = adjacent[self.constraint_reindex_cursor / 3];
                    let local = self.constraint_reindex_cursor % 3;
                    if !self.begin_edge_index_candidate(triangle, local) {
                        return Err(MeshError::TriangulationFailed("missing flipped triangle".into()));
                    }
                } else if self.advance_edge_index_candidate().map_err(|_| MeshError::TriangulationFailed("constraint edge index capacity".into()))? {
                    self.constraint_reindex_cursor += 1;
                }
            }
            ConstraintRecoveryStage::SearchConstraintEdge => {
                if self.constraint_search_cursor >= self.indexed_constraint_edges.len() {
                    return Err(MeshError::TriangulationFailed(format!("could not recover boundary edge {}-{}", constraint.0, constraint.1)));
                }
                let index = self.constraint_search_cursor;
                let slot = self.indexed_constraint_edges[index];
                self.constraint_search_cursor += 1;
                if slot.active && slot.edge == constraint {
                    self.indexed_constraint_edges[index].fixed = true;
                    self.constraint_stage = ConstraintRecoveryStage::ConstraintComplete;
                } else if slot.active && slot.adjacent[1].is_some() && !slot.fixed && slot.edge.0 != constraint.0 && slot.edge.0 != constraint.1 && slot.edge.1 != constraint.0 && slot.edge.1 != constraint.1 {
                    self.constraint_candidate = Some((slot.edge, self.constraint_search_cursor - 1));
                    self.constraint_adjacency_cursor = 0;
                    self.constraint_stage = ConstraintRecoveryStage::ClassifyIntersection;
                }
            }
            ConstraintRecoveryStage::ClassifyIntersection => {
                let (edge, index) = self.constraint_candidate.ok_or_else(|| MeshError::TriangulationFailed("missing constraint candidate".into()))?;
                let points = &self.triangulation.as_ref().ok_or_else(|| MeshError::TriangulationFailed("missing triangulation".into()))?.points;
                if segments_cross(points[constraint.0], points[constraint.1], points[edge.0], points[edge.1]) {
                    self.constraint_adjacency_cursor = index;
                    self.constraint_stage = ConstraintRecoveryStage::SelectDeterministicFlip;
                } else {
                    self.constraint_candidate = None;
                    self.constraint_stage = ConstraintRecoveryStage::SearchConstraintEdge;
                }
            }
            ConstraintRecoveryStage::SelectDeterministicFlip => {
                let (edge, index) = self.constraint_candidate.ok_or_else(|| MeshError::TriangulationFailed("missing selected constraint edge".into()))?;
                let slot = self.indexed_constraint_edges[index];
                let adjacent = match slot.adjacent {
                    [Some(first), Some(second)] => [first, second],
                    _ => return Err(MeshError::TriangulationFailed("constraint edge adjacency".into())),
                };
                self.constraint_flip = Some(ConstraintFlipCandidate { edge, adjacent, replacement: [[0; 3]; 2] });
                self.constraint_stage = ConstraintRecoveryStage::ValidateFlip;
            }
            ConstraintRecoveryStage::ValidateFlip => {
                let mut candidate = self.constraint_flip.ok_or_else(|| MeshError::TriangulationFailed("missing flip candidate".into()))?;
                let triangulation = self.triangulation.as_ref().ok_or_else(|| MeshError::TriangulationFailed("missing triangulation".into()))?;
                let [a, b] = [candidate.edge.0, candidate.edge.1];
                let first = triangulation.triangles[candidate.adjacent[0]];
                let second = triangulation.triangles[candidate.adjacent[1]];
                let c = first.into_iter().find(|index| *index != a && *index != b).ok_or_else(|| MeshError::TriangulationFailed("first opposite vertex".into()))?;
                let d = second.into_iter().find(|index| *index != a && *index != b).ok_or_else(|| MeshError::TriangulationFailed("second opposite vertex".into()))?;
                if !segments_cross(triangulation.points[a], triangulation.points[b], triangulation.points[c], triangulation.points[d]) {
                    return Err(MeshError::TriangulationFailed("unflippable constraint".into()));
                }
                candidate.replacement[0] = ccw_triangle(&triangulation.points, [c, d, a]).ok_or_else(|| MeshError::TriangulationFailed("invalid first flip".into()))?;
                candidate.replacement[1] = ccw_triangle(&triangulation.points, [d, c, b]).ok_or_else(|| MeshError::TriangulationFailed("invalid second flip".into()))?;
                self.constraint_flip = Some(candidate);
                self.constraint_apply_cursor = 0;
                self.constraint_stage = ConstraintRecoveryStage::ApplyFlip;
            }
            ConstraintRecoveryStage::ApplyFlip => {
                let candidate = self.constraint_flip.ok_or_else(|| MeshError::TriangulationFailed("missing validated flip".into()))?;
                let index = candidate.adjacent[self.constraint_apply_cursor];
                self.triangulation.as_mut().ok_or_else(|| MeshError::TriangulationFailed("missing triangulation".into()))?.triangles[index] = candidate.replacement[self.constraint_apply_cursor];
                self.constraint_apply_cursor += 1;
                if self.constraint_apply_cursor == 2 {
                    self.constraint_flip_count += 1;
                    if self.constraint_flip_count > self.constraint_flip_limit {
                        return Err(MeshError::TriangulationFailed("constraint flip limit".into()));
                    }
                    self.constraint_retire_cursor = 0;
                    self.constraint_retire_adjacency_cursor = 0;
                    self.constraint_stage = ConstraintRecoveryStage::RetireFormerEdge;
                }
            }
            ConstraintRecoveryStage::RetireFormerEdge => {
                if self.constraint_retire_cursor < self.indexed_constraint_edges.len() {
                    let adjacent = self.constraint_flip.map(|candidate| candidate.adjacent).ok_or_else(|| MeshError::TriangulationFailed("missing retire authority".into()))?;
                    let slot = &mut self.indexed_constraint_edges[self.constraint_retire_cursor];
                    if self.constraint_retire_adjacency_cursor < 2 {
                        let owner = &mut slot.adjacent[self.constraint_retire_adjacency_cursor];
                        if owner.is_some_and(|triangle| adjacent[0] == triangle || adjacent[1] == triangle) {
                            *owner = None;
                        }
                        self.constraint_retire_adjacency_cursor += 1;
                    } else {
                        slot.active = slot.adjacent[0].is_some() || slot.adjacent[1].is_some();
                        self.constraint_retire_cursor += 1;
                        self.constraint_retire_adjacency_cursor = 0;
                    }
                } else {
                    self.constraint_reindex_cursor = 0;
                    self.edge_index_candidate = None;
                    self.constraint_stage = ConstraintRecoveryStage::IndexTriangleEdge;
                }
            }
            ConstraintRecoveryStage::PublishConstraintProgress => {
                self.constraint_candidate = None;
                self.constraint_search_cursor = 0;
                self.constraint_stage = ConstraintRecoveryStage::SearchConstraintEdge;
            }
            ConstraintRecoveryStage::ConstraintComplete => return Ok(true),
        }
        Ok(false)
    }
}
// #endregion 🧵️IncrementalMeshJob
