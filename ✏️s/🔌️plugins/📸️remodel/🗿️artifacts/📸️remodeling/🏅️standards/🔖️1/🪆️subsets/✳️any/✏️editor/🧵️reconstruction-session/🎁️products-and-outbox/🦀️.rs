//! 🎁️ Products and outbox of the reconstruction session: the content leaves, the resumable product preparation and the outbox that publishes the outcome in turns. See [`super`].

use super::*;

//#region 🔖️Products
/// 🧱️ Raw leaves of one durable content entry under construction, content-addressed once complete.
#[derive(semio_framework_value::RetireOwned)]
struct ContentLeaves {
    pub(super) kind: RemodelingContentKind,
    pub(super) leaves: Vec<Vec<u8>>,
    pub(super) digest: RemodelingContentDigest,
    pub(super) bytes: usize,
}

impl ContentLeaves {
    pub(super) fn new(kind: RemodelingContentKind) -> Self {
        Self { kind, leaves: Vec::new(), digest: RemodelingContentDigest::default(), bytes: 0 }
    }

    pub(super) fn push(&mut self, leaf: Vec<u8>) {
        self.digest.record(&leaf);
        self.bytes += leaf.len();
        self.leaves.push(leaf);
    }

    /// 📦️ One `append-content` op per leaf (a leaf is the largest unit one tick page carries), and the id.
    fn into_ops(self, mime: Option<String>, width: u32, height: u32) -> (String, u64, Vec<RemodelingMutation>) {
        let content_id = self.digest.content_id(self.kind);
        let count = self.leaves.len() as u64;
        let ops = self
            .leaves
            .into_iter()
            .enumerate()
            .map(|(index, leaf)| append_content(AppendContent { content_id: content_id.clone(), kind: self.kind, mime: mime.clone(), width, height, first: index as u64, chunks: vec![ByteBuffer(leaf)] }))
            .collect();
        (content_id, count, ops)
    }
}

/// 🧊️ Field-tagged mesh leaves: `[field, values…]` per ≤ 4 KiB leaf, fields in ascending order.
fn mesh_leaves(mesh: &semio_framework::MeshData) -> ContentLeaves {
    let mut content = ContentLeaves::new(RemodelingContentKind::Mesh);
    let value_bytes = (REMODELING_DURABLE_CHUNK_RAW_BYTES - 1) / 4 * 4;
    let fields: [(u8, Vec<u8>); 12] = [
        (0, mesh.positions.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (1, mesh.normals.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (2, mesh.colors.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (3, mesh.indices.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (4, mesh.uvs.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (5, mesh.face_ids.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (6, mesh.vertex_ids.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (7, mesh.edge_positions.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (8, mesh.edge_ids.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (9, mesh.edge_uvs.iter().flat_map(|value| value.to_le_bytes()).collect()),
        (10, mesh.edge_is_seam.clone()),
        (11, mesh.paint_texture_base64.clone().unwrap_or_default().into_bytes()),
    ];
    for (field, bytes) in fields {
        let step = if field >= 10 { REMODELING_DURABLE_CHUNK_RAW_BYTES - 1 } else { value_bytes };
        for window in bytes.chunks(step) {
            let mut leaf = Vec::with_capacity(window.len() + 1);
            leaf.push(field);
            leaf.extend_from_slice(window);
            content.push(leaf);
        }
    }
    content
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum ProductPhase {
    Sparse,
    Quality,
    Mesh,
    Geo,
    Dsm,
    Dtm,
    Commit,
    Done,
}

/// 🏁️ What one bounded product unit yielded.
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum ProductYield {
    Working,
    Op(RemodelingMutation),
    Step(ReconstructionRunReason, Vec<u64>),
    Failed(ReconstructionRunReason),
    Done,
}

/// 🏭️ The product preparation after the engine finished: sparse leaves, QC, mesh leaves, DSM/DTM rasters,
/// then the one `commit-reconstruction` op.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct ProductPreparation {
    pub(super) phase: ProductPhase,
    camera_cursor: usize,
    point_cursor: usize,
    points_total: usize,
    quality_cursor: usize,
    quality_squared_error_sum: f64,
    quality_observations: usize,
    quality_points: BTreeSet<usize>,
    trajectory: Vec<CameraPosePreview>,
    sparse_leaves: ContentLeaves,
    sparse: Option<SparseCloud>,
    watertight: Option<WatertightReportSnapshot>,
    qc: Option<QcReportSnapshot>,
    pub(super) mesh: Option<Box<RemodelingMesh>>,
    geo_preparation: Option<TerminalGeoPreparation>,
    pending_dtm: Option<remodeling_geo::Raster>,
    raster: Option<(RasterPngPreparation, ContentLeaves, &'static str)>,
    pub(super) assets: Vec<ReconstructionAssetCommit>,
    pub(super) geo: Option<GeoProducts>,
    pending: VecDeque<ProductYield>,
    pub(super) gcp_count: usize,
}

impl ProductPreparation {
    pub(super) fn new(gcp_count: usize) -> Self {
        Self {
            phase: ProductPhase::Sparse,
            camera_cursor: 0,
            point_cursor: 0,
            points_total: 0,
            quality_cursor: 0,
            quality_squared_error_sum: 0.0,
            quality_observations: 0,
            quality_points: BTreeSet::new(),
            trajectory: Vec::new(),
            sparse_leaves: ContentLeaves::new(RemodelingContentKind::Sparse),
            sparse: None,
            watertight: None,
            qc: None,
            mesh: None,
            geo_preparation: None,
            pending_dtm: None,
            raster: None,
            assets: Vec::new(),
            geo: None,
            pending: VecDeque::new(),
            gcp_count,
        }
    }

    fn queue_content(&mut self, content: ContentLeaves, mime: Option<String>, width: u32, height: u32) -> (String, u64) {
        let (leaves, bytes) = (content.leaves.len() as u64, content.bytes as u64);
        let (content_id, count, ops) = content.into_ops(mime, width, height);
        self.pending.push_back(ProductYield::Step(ReconstructionRunReason::ContentPrepared, vec![leaves, bytes]));
        self.pending.extend(ops.into_iter().map(ProductYield::Op));
        (content_id, count)
    }

    pub(super) fn advance(&mut self, engine: &mut ReconstructionEngine) -> ProductYield {
        if let Some(pending) = self.pending.pop_front() {
            return pending;
        }
        match self.phase {
            ProductPhase::Sparse => {
                let chunk = engine.terminal_sparse_chunk(self.camera_cursor, self.point_cursor, TERMINAL_CAMERA_WORK, TERMINAL_POINT_WORK);
                for (offset, pose) in chunk.camera_poses.iter().enumerate() {
                    self.trajectory.push(camera_pose_preview((self.camera_cursor + offset) as u32, pose));
                }
                self.points_total += chunk.packed_points.len() / 3;
                let room = (RemodelingContentKind::Sparse.max_bytes() - self.sparse_leaves.bytes) / 12 * 3;
                let kept = &chunk.packed_points[..chunk.packed_points.len().min(room)];
                if !kept.is_empty() {
                    self.sparse_leaves.push(kept.iter().flat_map(|value| value.to_le_bytes()).collect());
                }
                self.camera_cursor = chunk.next_camera;
                self.point_cursor = chunk.next_point;
                if chunk.complete {
                    self.phase = ProductPhase::Quality;
                    let kept_points = self.sparse_leaves.bytes / 12;
                    if kept_points < self.points_total {
                        self.pending.push_back(ProductYield::Step(ReconstructionRunReason::SparseCapped, vec![kept_points as u64, self.points_total as u64]));
                    }
                    let content = std::mem::replace(&mut self.sparse_leaves, ContentLeaves::new(RemodelingContentKind::Sparse));
                    if !content.leaves.is_empty() {
                        let (content_id, count) = self.queue_content(content, None, 0, 0);
                        self.sparse = Some(SparseCloud { points: Float32Buffer::Content { content_id: content_id.clone(), chunk_count: count }, colors: None });
                    }
                }
                ProductYield::Working
            }
            ProductPhase::Quality => {
                let chunk = engine.terminal_quality_chunk(self.quality_cursor, TERMINAL_QUALITY_WORK);
                self.quality_cursor = chunk.next_observation;
                self.quality_squared_error_sum += chunk.squared_error_sum;
                self.quality_observations += chunk.observation_count;
                self.quality_points.extend(chunk.point_indices);
                if chunk.complete {
                    self.watertight = engine.terminal_watertight_report().as_ref().map(watertight_snapshot);
                    let accepted = engine.frame_source().accepted_count();
                    let mut warnings = Vec::new();
                    if self.watertight.as_ref().is_some_and(|report| !report.is_watertight) {
                        warnings.push("Mesh is not watertight.".into());
                    }
                    if self.gcp_count > 0 {
                        warnings.push("Ground control points are set but checkpoint RMSE is not yet computed.".into());
                    }
                    self.qc = Some(QcReportSnapshot {
                        reprojection_rms_px: if self.quality_observations == 0 { 0.0 } else { (self.quality_squared_error_sum / self.quality_observations as f64).sqrt() },
                        gcp_checkpoint_rmse: None,
                        watertight: self.watertight.clone(),
                        mean_track_length: if self.quality_points.is_empty() { 0.0 } else { self.quality_observations as f32 / self.quality_points.len() as f32 },
                        registered_frame_ratio: if accepted == 0 { 0.0 } else { self.trajectory.len() as f32 / accepted as f32 },
                        dense_coverage_ratio: 0.0,
                        warnings,
                    });
                    self.phase = ProductPhase::Mesh;
                }
                ProductYield::Working
            }
            ProductPhase::Mesh => {
                self.phase = ProductPhase::Geo;
                let Some(mesh) = engine.take_mesh().filter(|mesh| !mesh.indices.is_empty()) else { return ProductYield::Working };
                let (vertices, triangles) = ((mesh.positions.len() / 3) as u64, (mesh.indices.len() / 3) as u64);
                if !mesh_is_within_resolution_envelope(&mesh) {
                    return ProductYield::Step(ReconstructionRunReason::MeshOutsideEnvelope, vec![vertices, triangles]);
                }
                let (content_id, count) = self.queue_content(mesh_leaves(&mesh), None, 0, 0);
                self.mesh = Some(Box::new(RemodelingMesh { mesh: remodeling_mesh_content_handle(&content_id, count), source: MeshSource::Reconstructed, texture_asset_id: None, watertight: self.watertight.clone() }));
                ProductYield::Working
            }
            ProductPhase::Geo => {
                match self.geo_preparation.as_mut() {
                    None => {
                        self.geo_preparation = engine.begin_terminal_geo();
                        if self.geo_preparation.is_none() {
                            self.phase = ProductPhase::Commit;
                        }
                    }
                    Some(preparation) => {
                        if engine.advance_terminal_geo(preparation, TERMINAL_GEO_WORK) {
                            let products = self.geo_preparation.take().and_then(ReconstructionEngine::finish_terminal_geo);
                            match products {
                                Some(products) => {
                                    self.pending_dtm = Some(products.dtm);
                                    self.raster = Some((RasterPngPreparation::new(products.dsm), ContentLeaves::new(RemodelingContentKind::Image), REMODELING_RECONSTRUCTION_DSM_ASSET_ID));
                                    self.phase = ProductPhase::Dsm;
                                }
                                None => self.phase = ProductPhase::Commit,
                            }
                        }
                    }
                }
                ProductYield::Working
            }
            ProductPhase::Dsm | ProductPhase::Dtm => {
                let Some((encoder, content, _)) = self.raster.as_mut() else { return ProductYield::Failed(ReconstructionRunReason::RasterFailed) };
                match encoder.advance(RASTER_CELL_WORK) {
                    RasterPngProgress::Working => ProductYield::Working,
                    RasterPngProgress::Chunk(bytes) => {
                        content.push(bytes);
                        ProductYield::Working
                    }
                    RasterPngProgress::Failed => ProductYield::Failed(ReconstructionRunReason::RasterFailed),
                    RasterPngProgress::Complete => {
                        let (encoder, content, asset_id) = self.raster.take().expect("completed raster");
                        let (content_id, _) = self.queue_content(content, Some("image/png".into()), encoder.width(), encoder.height());
                        self.assets.push(ReconstructionAssetCommit { id: asset_id.into(), content_id: Some(content_id) });
                        if self.phase == ProductPhase::Dsm {
                            let dtm = self.pending_dtm.take().expect("DTM retained through DSM encoding");
                            self.raster = Some((RasterPngPreparation::new(dtm), ContentLeaves::new(RemodelingContentKind::Image), REMODELING_RECONSTRUCTION_DTM_ASSET_ID));
                            self.phase = ProductPhase::Dtm;
                        } else {
                            self.geo = Some(GeoProducts { dsm_asset_id: Some(REMODELING_RECONSTRUCTION_DSM_ASSET_ID.into()), dtm_asset_id: Some(REMODELING_RECONSTRUCTION_DTM_ASSET_ID.into()), ortho_asset_id: None });
                            self.phase = ProductPhase::Commit;
                        }
                        ProductYield::Working
                    }
                }
            }
            ProductPhase::Commit => {
                self.phase = ProductPhase::Done;
                let trajectory = (!self.trajectory.is_empty()).then(|| CameraTrajectory { poses: std::mem::take(&mut self.trajectory) });
                if self.sparse.is_none() && self.mesh.is_none() {
                    // 🌱️ A run without geometry has nothing to publish: no edit, no placeholder
                    // replaced. That covers a run that registered no camera (the trace says
                    // `seedPairFailed`) and one whose seed pair solved but kept no triangulated
                    // point (two views 50° apart): a bare trajectory is no reconstruction.
                    return ProductYield::Done;
                }
                ProductYield::Op(commit_reconstruction(CommitReconstruction { sparse: self.sparse.take(), trajectory, mesh: self.mesh.take(), geo: self.geo.take(), qc: self.qc.take(), assets: std::mem::take(&mut self.assets) }))
            }
            ProductPhase::Done => ProductYield::Done,
        }
    }
}
//#endregion 🔖️Products

//#region 📤️Outbox
/// 🧭️ Where one publication stands between the job and its driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OutboxPhase {
    Idle,
    Building,
    Delivered,
    Retiring,
}

/// 📤️ One original payload source and the publication that lends it; the source never moves while the loan lives.
#[derive(Default)]
pub(super) struct Outbox {
    pub(super) publication: RetainedJobPublication,
    pub(super) source: Vec<u8>,
    pub(super) kind: Option<JobPublicationKind>,
    retiring: bool,
}

impl Outbox {
    pub(super) fn begin(&mut self, kind: JobPublicationKind, source: Vec<u8>) {
        self.source = source;
        self.kind = Some(kind);
    }

    pub(super) fn phase(&self, cx: &StepContext<'_>) -> Result<OutboxPhase, ValueError> {
        let Some(kind) = self.kind else { return Ok(OutboxPhase::Idle) };
        if self.retiring {
            return Ok(OutboxPhase::Retiring);
        }
        let demand = self.publication.advance_demands(kind, &self.source, cx)?;
        let delivered = self.publication.published().is_some() && demand.copy_bytes == 0 && demand.capacity_bytes == 0 && demand.release_bytes == 0 && demand.depth == 0;
        Ok(if delivered { OutboxPhase::Delivered } else { OutboxPhase::Building })
    }

    pub(super) fn advance<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let kind = self.kind.expect("a building outbox owns its publication kind");
        self.publication.advance_from_source(kind, &self.source, cx)
    }

    pub(super) fn retire_step(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        self.retiring = true;
        match self.publication.close_step(cx.retained_grant())? {
            RetainedCloneStep::Progress(progress) => cx.consume_retained(progress),
            RetainedCloneStep::Complete(progress) => {
                cx.consume_retained(progress)?;
                self.kind = None;
                self.retiring = false;
                self.source = Vec::new();
                Ok(())
            }
        }
    }

    pub(super) fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        self.publication.borrow_outcome(descriptor)
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.kind.is_none() && self.publication.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        self.retiring = true;
        let step = self.publication.close_step(grant)?;
        if self.publication.terminal_is_empty() {
            self.kind = None;
            self.retiring = false;
            self.source = Vec::new();
        }
        Ok(step)
    }

    pub(super) fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        self.publication.retirement_demands()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.kind.is_none() && self.publication.terminal_is_empty()
    }
}

pub(super) fn borrow_remodel_outcome<'a>(outbox: &'a Outbox, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
    match descriptor.kind() {
        JobOutcomeKind::Yield => descriptor.yielded(),
        JobOutcomeKind::Cancelled => descriptor.cancelled(),
        JobOutcomeKind::Complete => descriptor.complete(None, None),
        JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => outbox.borrow_outcome(descriptor),
    }
}

pub(super) fn close_outbox_turn(outbox: &mut Outbox, grant: RetainedCloneGrant) -> Option<InteractiveJobCloseStep> {
    if outbox.terminal_is_empty() {
        return None;
    }
    Some(match outbox.close_step(grant) {
        Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
        Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
    })
}
//#endregion 📤️Outbox
