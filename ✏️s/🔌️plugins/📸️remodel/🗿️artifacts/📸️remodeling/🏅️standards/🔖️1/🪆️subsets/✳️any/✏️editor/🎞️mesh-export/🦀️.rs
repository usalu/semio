//! 🎞️ Original snapshot-backed, resumable mesh media export.
#[path = "🗃️source/🦀️.rs"]
mod source;

use semio_framework::{InteractiveJobClassification, MediaClass, MediaForm, MediaType, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
use semio_framework_job::{admit_original_construction, InteractiveJob, InteractiveJobCloseStep, InteractiveJobStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, OriginalConstructionDemand, OriginalJobReturnClaim, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_job_publication::RetainedJobPublication;
use semio_framework_mesh_engine::io::binary::stream::{GlbCursor, GlbSource};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_ordered::RetirementStep;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_plugin::{ArtifactApp, ArtifactMediaExportJobRequest, ArtifactOwnedToolJobFactory, ArtifactReservedJob, ArtifactReservedToolJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane};
use semio_plugin::app::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactSnapshotCloseLease};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{RemodelingDurableArtifact, RemodelingMeshChild, RemodelingOrderedDurableArtifactStore};
use source::{HandleCountCursor, MeshIndex, OriginalMeshLookup};
use std::{mem::{size_of, ManuallyDrop}, sync::Arc};

pub trait MeshExportSnapshot {
    fn mesh_export_handle(&self) -> &RemodelingMeshChild;
    fn mesh_export_store(&self) -> &RemodelingOrderedDurableArtifactStore;
}

const PORT: &str = "mesh:out";
const TOOL: &str = "export-media:mesh:out";
const SCHEMA: &str = "glb";
const MIME: &str = "model/gltf-binary";
type Outputs = (ArtifactMediaExportCredit, Option<ArtifactOutputChunks>, ArtifactMediaExportCompletion);
type Metadata = (String, String, String);

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }
fn allocate<T>(value: T) -> Result<Box<T>, ValueError> { let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(std::alloc::Layout::new::<T>()) }.cast::<T>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::AllocationFailed, "mesh export allocation refused"))?; Ok(unsafe { pointer.as_ptr().write(value); Box::from_raw(pointer.as_ptr()) }) }
fn demand(copy_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, depth: 1, ..Default::default() } }
fn construction(value: RetirementDemand) -> OriginalConstructionDemand { OriginalConstructionDemand { items: 1, copy_bytes: value.copy_bytes, capacity_bytes: value.capacity_bytes, release_bytes: value.release_bytes, depth: value.depth } }

pub struct MeshExportJob<A: ArtifactApp> where A::Snapshot: MeshExportSnapshot {
    snapshot: Option<Arc<A::Snapshot>>,
    lease: Option<ArtifactSnapshotCloseLease<A::Snapshot>>,
    metadata: ManuallyDrop<Option<Metadata>>,
    metadata_close: ManuallyDrop<Option<ControlledRetirement<Metadata>>>,
    outputs: ManuallyDrop<Option<Outputs>>,
    outputs_close: ManuallyDrop<Option<ControlledRetirement<Outputs>>>,
    pending: ManuallyDrop<Option<Result<ArtifactMediaExportResult, semio_framework::MediaError>>>,
    pending_close: ManuallyDrop<Option<ControlledRetirement<Result<ArtifactMediaExportResult, semio_framework::MediaError>>>>,
    parser: HandleCountCursor,
    declared: Option<u64>,
    lookup: Option<OriginalMeshLookup>,
    orphan: ManuallyDrop<Option<RemodelingDurableArtifact>>,
    orphan_close: ManuallyDrop<Option<ControlledRetirement<RemodelingDurableArtifact>>>,
    index: Option<Box<MeshIndex>>,
    cursor: Option<Box<GlbCursor>>,
    normals: Option<Vec<[f32; 3]>>,
    page: Option<Vec<u8>>,
    source_identity: (semio_framework_job::OperationId, semio_framework_job::Generation),
    binding: Option<OriginalSourceBinding>,
    publication: RetainedJobPublication,
    produced: u64,
    turns: u64,
    published: bool,
    encoded: bool,
    publish_progress: bool,
    checkpoint_active: bool,
    checkpoint_closing: bool,
    closing: bool,
}

impl<A: ArtifactApp> MeshExportJob<A> where A::Snapshot: MeshExportSnapshot {
    pub fn admit_original(request: &mut Option<ArtifactMediaExportJobRequest<A>>, body: &mut Option<Box<dyn ArtifactReservedJob + Send>>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        if cx.should_yield() || cx.is_cancelled() { return Ok(false); }
        if body.is_none() {
            let request = request.as_ref().ok_or_else(|| invalid("mesh export request is absent"))?;
            if request.port != PORT || request.tool_id != TOOL { return Err(invalid("mesh export received a foreign port").into()); }
            let source_identity = request.source_identity;
            let bytes = size_of::<Self>();
            return admit_original_construction(cx, body, construction(RetirementDemand { capacity_bytes: bytes, ..demand(bytes) }), |_| {
                let result = (|| {
                    let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(std::alloc::Layout::new::<Self>()) }.cast::<Self>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::AllocationFailed, "mesh export body allocation refused"))?;
                    let owner = Self { snapshot: None, lease: None, metadata: ManuallyDrop::new(None), metadata_close: ManuallyDrop::new(None), outputs: ManuallyDrop::new(None), outputs_close: ManuallyDrop::new(None), pending: ManuallyDrop::new(None), pending_close: ManuallyDrop::new(None), parser: HandleCountCursor::new(), declared: None, lookup: None, orphan: ManuallyDrop::new(None), orphan_close: ManuallyDrop::new(None), index: None, cursor: None, normals: None, page: None, source_identity, binding: None, publication: RetainedJobPublication::new(), produced: 0, turns: 0, published: false, encoded: false, publish_progress: false, checkpoint_active: false, checkpoint_closing: false, closing: false };
                    let boxed: Box<dyn ArtifactReservedJob + Send> = unsafe { pointer.as_ptr().write(owner); Box::from_raw(pointer.as_ptr()) };
                    Ok((boxed, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default() }))
                })();
                RetainedWorkOutcome { result, work: 1 }
            }).map(|_| false);
        }
        let owner = (&mut **body.as_mut().unwrap() as &mut dyn std::any::Any).downcast_mut::<Self>().ok_or_else(|| invalid("mesh export requires its exact original body"))?;
        if owner.snapshot.is_some() { return if request.is_none() { Ok(true) } else { Err(invalid("mesh export source cannot be overwritten").into()) }; }
        let bytes = size_of::<ArtifactMediaExportJobRequest<A>>();
        let grant = cx.retained_grant();
        if !semio_plugin::app::plugin_grant_funds(demand(bytes), grant) || grant.maximum_items == 0 { return Ok(false); }
        let original = request.take().ok_or_else(|| invalid("mesh export request is absent"))?;
        owner.snapshot = Some(original.snapshot);
        owner.lease = Some(original.snapshot_close);
        *owner.metadata = Some((original.tool_id, original.parent_document_id, original.port));
        *owner.outputs = Some((original.output_credit, Some(original.output_chunks), original.completion));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(false)
    }

    fn advance_source(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let snapshot = self.snapshot.as_deref().ok_or_else(|| invalid("mesh export lost its original snapshot"))?;
        let handle = snapshot.mesh_export_handle();
        if cx.fuel_remaining() < 64 { return Ok(false); }
        let (empty, boxed, compared) = source::constant_mesh(&handle.child_id);
        cx.consume_fuel(compared);
        let placeholder = empty || boxed;
        if !placeholder && self.declared.is_none() {
            let grant = cx.retained_grant();
            let extra = size_of::<Option<u64>>();
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < 64 + extra || grant.maximum_depth == 0 || cx.fuel_remaining() == 0 { return Ok(false); }
            let turn = match self.parser.advance(handle, 1, grant.maximum_copy_bytes - extra, grant.maximum_depth, cx.fuel_remaining()) {
                Ok(turn) => turn,
                Err(error) => { cx.consume_fuel(error.work); return Err(invalid(error.reason).into()); }
            };
            let mut copied = turn.copied_bytes;
            if turn.complete { self.declared = turn.value; copied += extra; }
            cx.consume_retained(RetainedCloneProgress { copied_items: usize::from(turn.work != 0), copied_bytes: copied, ..Default::default() })?;
            cx.consume_fuel(turn.work);
            return Ok(false);
        }
        if !placeholder && self.lookup.is_none() {
            let store = snapshot.mesh_export_store();
            let grant = cx.retained_grant();
            let quote = OriginalMeshLookup::birth_demand(store);
            admit_original_construction(cx, &mut self.lookup, construction(quote), |_| OriginalMeshLookup::admit(store, handle, grant, 1))?;
            return Ok(false);
        }
        if let Some(lookup) = self.lookup.as_mut() {
            if !lookup.is_complete() {
                let grant = cx.retained_grant();
                let quote = lookup.next_demand(handle)?;
                if !semio_plugin::app::plugin_grant_funds(quote, grant) || grant.maximum_items == 0 || cx.fuel_remaining() == 0 { return Ok(false); }
                semio_framework_job::receive_original_retained_work(cx, lookup.advance(handle, grant, 1))?;
                return Ok(false);
            }
            if lookup.result().is_none() { return Err(invalid("mesh content was not found in the original snapshot").into()); }
        }
        if self.index.is_none() {
            let bytes = size_of::<MeshIndex>();
            admit_original_construction(cx, &mut self.index, construction(RetirementDemand { capacity_bytes: bytes, ..demand(bytes) }), |_| {
                RetainedWorkOutcome { result: allocate(MeshIndex::new()).map(|index| (index, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default() })), work: 1 }
            })?;
            return Ok(false);
        }
        let index = self.index.as_mut().unwrap();
        if !index.is_ready() {
            let grant = cx.retained_grant();
            let original = self.lookup.as_ref().and_then(OriginalMeshLookup::result);
            let turn = match index.advance(handle, self.declared.unwrap_or(0), original, grant.maximum_items, grant.maximum_copy_bytes, grant.maximum_capacity_bytes, grant.maximum_depth, cx.fuel_remaining()) {
                Ok(turn) => turn,
                Err(error) => { cx.consume_fuel(error.work); return Err(invalid(error.reason).into()); }
            };
            cx.consume_retained(RetainedCloneProgress { copied_items: usize::from(turn.work != 0), copied_bytes: turn.copied_bytes, retained_capacity_bytes: turn.retained_capacity_bytes, ..Default::default() })?;
            cx.consume_fuel(turn.work);
            return Ok(false);
        }
        Ok(true)
    }

    fn advance_output(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.pending.is_some() || self.published {
            self.outputs.as_ref().ok_or_else(|| invalid("mesh export lost its outputs"))?.2.complete_original(&mut self.pending, &mut self.published, cx)?;
            return Ok(());
        }
        if self.encoded {
            if self.page.as_ref().is_some_and(|page| !page.is_empty()) {
                let outputs = self.outputs.as_ref().ok_or_else(|| invalid("mesh export lost its outputs"))?;
                outputs.1.as_ref().ok_or_else(|| invalid("mesh export lost its chunks"))?.push_original(&mut self.page, Some(&outputs.0), cx)?;
                return Ok(());
            }
            let capacity = SCHEMA.len() + MIME.len();
            let bytes = size_of::<ArtifactMediaExportResult>() + capacity;
            let outputs = &mut self.outputs;
            admit_original_construction(cx, &mut self.pending, construction(RetirementDemand { capacity_bytes: capacity, ..demand(bytes) }), |_| {
                let result = (|| {
                    let original = outputs.as_mut().ok_or_else(|| invalid("mesh export lost its outputs"))?;
                    original.1.as_ref().ok_or_else(|| invalid("mesh export lost its chunks"))?.seal().map_err(|_| invalid("mesh output seal refused"))?;
                    original.0.credit_original(SCHEMA.len())?;
                    let chunks = original.1.take().unwrap();
                    Ok((Ok(ArtifactMediaExportResult { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, schema: SCHEMA.into(), mime_type: MIME.into(), chunks }), RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: capacity, ..Default::default() }))
                })();
                RetainedWorkOutcome { result, work: 1 }
            })?;
            return Ok(());
        }
        let index = self.index.as_ref().ok_or_else(|| invalid("mesh export lost its index"))?;
        let source = index.source(self.lookup.as_ref().and_then(OriginalMeshLookup::result));
        if !source.valid_layout() { return Err(invalid("mesh export lost its indexed original source").into()); }
        let normal_count = if source.has_normals() { 0 } else { source.vertices() };
        if self.normals.is_none() {
            let capacity = normal_count.checked_mul(size_of::<[f32; 3]>()).ok_or_else(|| invalid("mesh normal capacity overflow"))?;
            admit_original_construction(cx, &mut self.normals, construction(RetirementDemand { capacity_bytes: capacity, ..demand(size_of::<Vec<[f32; 3]>>()) }), |_| {
                let result = (|| {
                    let values = if normal_count == 0 { Vec::new() } else {
                        let layout = std::alloc::Layout::array::<[f32; 3]>(normal_count).map_err(|_| invalid("mesh normal capacity overflow"))?;
                        let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<[f32; 3]>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::AllocationFailed, "mesh normal allocation refused"))?;
                        unsafe { Vec::from_raw_parts(pointer.as_ptr(), 0, normal_count) }
                    };
                    Ok((values, RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<[f32; 3]>>(), retained_capacity_bytes: capacity, ..Default::default() }))
                })();
                RetainedWorkOutcome { result, work: 1 }
            })?;
            return Ok(());
        }
        if self.normals.as_ref().unwrap().len() < normal_count {
            let quote = demand(size_of::<[f32; 3]>() + size_of::<usize>());
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) || cx.fuel_remaining() == 0 { return Ok(()); }
            self.normals.as_mut().unwrap().push([0.0; 3]);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(());
        }
        if self.cursor.is_none() {
            let bytes = size_of::<GlbCursor>();
            if cx.fuel_remaining() < bytes as u64 { return Ok(()); }
            admit_original_construction(cx, &mut self.cursor, construction(RetirementDemand { capacity_bytes: bytes, ..demand(bytes) }), |_| {
                let result = GlbCursor::new(&source).map_err(invalid).and_then(allocate).map(|cursor| (cursor, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default() }));
                RetainedWorkOutcome { result, work: bytes as u64 }
            })?;
            return Ok(());
        }
        if self.page.as_ref().is_some_and(|page| page.len() == ArtifactOutputChunks::CHUNK_BYTES) {
            let outputs = self.outputs.as_ref().ok_or_else(|| invalid("mesh export lost its outputs"))?;
            outputs.1.as_ref().ok_or_else(|| invalid("mesh export lost its chunks"))?.push_original(&mut self.page, Some(&outputs.0), cx)?;
            return Ok(());
        }
        if self.page.is_none() {
            let bytes = ArtifactOutputChunks::CHUNK_BYTES;
            admit_original_construction(cx, &mut self.page, construction(RetirementDemand { capacity_bytes: bytes, ..demand(size_of::<Vec<u8>>()) }), |_| {
                let result = (|| { let layout = std::alloc::Layout::array::<u8>(bytes).map_err(|_| invalid("mesh output page layout refused"))?; let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }).ok_or_else(|| ValueError::literal(ValueRefusalKind::AllocationFailed, "mesh output page allocation refused"))?; let page = unsafe { Vec::from_raw_parts(pointer.as_ptr(), 0, bytes) }; let capacity = bytes; Ok((page, RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<u8>>(), retained_capacity_bytes: capacity, ..Default::default() })) })();
                RetainedWorkOutcome { result, work: 1 }
            })?;
            return Ok(());
        }
        let cursor = self.cursor.as_ref().unwrap();
        let extra = 2 + size_of::<usize>() + size_of::<u64>();
        let quote = cursor.next_copy_demand().checked_add(extra).ok_or_else(|| invalid("mesh output copy quote overflow"))?;
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < quote || grant.maximum_depth == 0 || cx.fuel_remaining() < cursor.next_work_demand(&source) { return Ok(()); }
        let mut byte = [0];
        let turn = match self.cursor.as_mut().unwrap().advance(&source, self.normals.as_mut().unwrap(), &mut byte, 1, grant.maximum_copy_bytes - extra, grant.maximum_depth, cx.fuel_remaining()) {
            Ok(turn) => turn,
            Err(error) => { cx.consume_fuel(error.work); return Err(invalid(error.reason).into()); }
        };
        let mut copied = turn.copied_bytes;
        if turn.written == 1 { self.page.as_mut().unwrap().push(byte[0]); self.produced += 1; copied += extra - 1; }
        if turn.complete { self.encoded = true; copied += 1; }
        cx.consume_retained(RetainedCloneProgress { copied_items: usize::from(turn.work != 0), copied_bytes: copied, ..Default::default() })?;
        cx.consume_fuel(turn.work);
        Ok(())
    }

    fn step_turn<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.closing || cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into); }
        if self.published { return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into); }
        if cx.should_yield() { return Ok(None); }
        if self.binding.is_none() {
            let quote = demand(size_of::<Option<OriginalSourceBinding>>());
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) || cx.fuel_remaining() == 0 { return Ok(None); }
            self.binding = Some(OriginalSourceBinding::borrow(cx));
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(None);
        }
        if !self.binding.unwrap().matches(cx) || self.source_identity != (cx.operation(), cx.generation()) { return Err(invalid("mesh export lost its original operation source").into()); }
        if self.checkpoint_active {
            if self.checkpoint_closing {
                if !self.publication.terminal_is_empty() {
                    let _ = semio_framework_job::close_original_quoted_retained_step(cx, self.publication.retirement_demands()?, self.publication.retirement_work()?, |grant| self.publication.close_step(grant))?;
                    return Ok(None);
                }
                let quote = demand(2 * size_of::<bool>());
                let grant = cx.retained_grant();
                if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) || cx.fuel_remaining() == 0 { return Ok(None); }
                self.checkpoint_active = false; self.checkpoint_closing = false;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(None);
            }
            if self.publication.is_delivered() {
                let quote = demand(size_of::<bool>());
                let grant = cx.retained_grant();
                if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) || cx.fuel_remaining() == 0 { return Ok(None); }
                self.checkpoint_closing = true;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(None);
            }
            return self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint { applied_progress: self.produced }, b"{\"stage\":\"mesh-export\"}", cx);
        }
        if self.publish_progress {
            let quote = demand(2 * size_of::<bool>());
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) || cx.fuel_remaining() == 0 { return Ok(None); }
            self.publish_progress = false; self.checkpoint_active = true;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(None);
        }
        cx.set_stage("mesh-export");
        while !cx.should_yield() {
            let fuel = cx.fuel_remaining();
            if self.advance_source(cx)? { self.advance_output(cx)?; }
            if self.published || cx.fuel_remaining() == fuel { break; }
            let quote = demand(size_of::<u64>() + size_of::<bool>());
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) || cx.fuel_remaining() == 0 { break; }
            self.turns = self.turns.saturating_add(1);
            self.publish_progress = self.turns.is_multiple_of(256);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            if self.publish_progress { break; }
        }
        Ok(None)
    }

    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() { return self.publication.retirement_demands(); }
        if self.publish_progress || self.checkpoint_active || self.checkpoint_closing { return Ok(demand(3 * size_of::<bool>())); }
        if self.cursor.is_some() { return Ok(RetirementDemand { release_bytes: size_of::<GlbCursor>(), ..demand(size_of::<Option<Box<GlbCursor>>>()) }); }
        if let Some(index) = self.index.as_ref() {
            if !index.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: index.next_release_demand(), ..demand(size_of::<Vec<u32>>()) }); }
            return Ok(RetirementDemand { release_bytes: size_of::<MeshIndex>(), ..demand(size_of::<Option<Box<MeshIndex>>>()) });
        }
        if let Some(values) = self.normals.as_ref() { return Ok(RetirementDemand { release_bytes: values.capacity() * size_of::<[f32; 3]>(), ..demand(size_of::<Option<Vec<[f32; 3]>>>()) }); }
        if let Some(page) = self.page.as_ref() { return Ok(RetirementDemand { release_bytes: page.capacity(), ..demand(size_of::<Option<Vec<u8>>>()) }); }
        if self.orphan.is_some() || self.orphan_close.is_some() { return semio_plugin::app::plugin_typed_owner_demand(&self.orphan, &self.orphan_close, body); }
        if let Some(lookup) = self.lookup.as_ref() {
            if lookup.terminal_is_empty() { return Ok(demand(size_of::<Option<OriginalMeshLookup>>())); }
            return Ok(RetirementDemand { copy_bytes: lookup.next_close_copy_demand().max(size_of::<RemodelingDurableArtifact>()), release_bytes: lookup.next_close_release_demand()?, depth: lookup.next_close_depth_demand().max(1), ..Default::default() });
        }
        if self.pending.is_some() || self.pending_close.is_some() { return semio_plugin::app::plugin_typed_owner_demand(&self.pending, &self.pending_close, body); }
        if self.outputs.is_some() || self.outputs_close.is_some() { return semio_plugin::app::plugin_typed_owner_demand(&self.outputs, &self.outputs_close, body); }
        if self.snapshot.is_some() { return Ok(demand(size_of::<Arc<A::Snapshot>>())); }
        if let Some(lease) = self.lease.as_ref() { return Ok(if lease.terminal_is_empty() { demand(size_of::<Option<ArtifactSnapshotCloseLease<A::Snapshot>>>()) } else { lease.retirement_demands() }); }
        if self.metadata.is_some() || self.metadata_close.is_some() { return semio_plugin::app::plugin_typed_owner_demand(&self.metadata, &self.metadata_close, body); }
        Ok(Default::default())
    }

    fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        if !self.publication.terminal_is_empty() { return self.publication.retirement_work(); }
        if self.publish_progress || self.checkpoint_active || self.checkpoint_closing || self.cursor.is_some() || self.index.is_some() || self.normals.is_some() || self.page.is_some() { return Ok(1); }
        if self.orphan.is_some() || self.orphan_close.is_some() { return semio_plugin::app::plugin_typed_owner_work_demand(&self.orphan, &self.orphan_close, body); }
        if self.lookup.is_some() { return Ok(1); }
        if self.pending.is_some() || self.pending_close.is_some() { return semio_plugin::app::plugin_typed_owner_work_demand(&self.pending, &self.pending_close, body); }
        if self.outputs.is_some() || self.outputs_close.is_some() { return semio_plugin::app::plugin_typed_owner_work_demand(&self.outputs, &self.outputs_close, body); }
        if self.snapshot.is_some() || self.lease.is_some() { return Ok(1); }
        if self.metadata.is_some() || self.metadata_close.is_some() { return semio_plugin::app::plugin_typed_owner_work_demand(&self.metadata, &self.metadata_close, body); }
        Ok(0)
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant, fuel: u64) -> semio_framework_value::OriginalRetirementOutcome {
        let mut work = 0;
        let result = self.close_unit(grant, fuel, &mut work);
        RetainedWorkOutcome { result, work }
    }

    fn close_unit(&mut self, grant: RetainedCloneGrant, fuel: u64, work: &mut u64) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        let quote = self.close_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || !semio_plugin::app::plugin_grant_funds(quote, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let item = RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() };
        if !self.publication.terminal_is_empty() { let turn = self.publication.close_step(grant); *work += turn.work; return turn.result.map_err(Into::into); }
        if self.publish_progress || self.checkpoint_active || self.checkpoint_closing { self.publish_progress = false; self.checkpoint_active = false; self.checkpoint_closing = false; *work = 1; return Ok(RetainedCloneStep::Progress(item)); }
        if self.cursor.is_some() { self.cursor.take(); *work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { released_bytes: quote.release_bytes, ..item })); }
        if let Some(index) = self.index.as_mut() {
            if !index.terminal_is_empty() {
                let turn = index.close_step(grant.maximum_items, grant.maximum_copy_bytes, grant.maximum_release_bytes, grant.maximum_depth, fuel).map_err(|error| invalid(error.reason))?;
                *work += turn.work;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(turn.work != 0), copied_bytes: turn.copied_bytes, released_bytes: turn.released_bytes, ..Default::default() }));
            }
            self.index.take(); *work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { released_bytes: quote.release_bytes, ..item }));
        }
        if self.normals.is_some() { self.normals.take(); *work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { released_bytes: quote.release_bytes, ..item })); }
        if self.page.is_some() { self.page.take(); *work = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { released_bytes: quote.release_bytes, ..item })); }
        if self.orphan.is_some() || self.orphan_close.is_some() { let turn = semio_plugin::app::plugin_typed_owner_original(&mut self.orphan, &mut self.orphan_close, grant); *work += turn.work; return turn.result; }
        if let Some(lookup) = self.lookup.as_mut() {
            if lookup.terminal_is_empty() { self.lookup.take(); *work = 1; return Ok(RetainedCloneStep::Progress(item)); }
            let (step, spent) = lookup.close_step(grant); *work += spent;
            return match step {
                RetirementStep::Blocked => Ok(RetainedCloneStep::Progress(Default::default())),
                RetirementStep::Progress { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() })),
                RetirementStep::ProcessedBytes(_) => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })),
                RetirementStep::OwnedValue(value) => { *self.orphan = Some(value); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<RemodelingDurableArtifact>(), ..Default::default() })) },
                RetirementStep::Failure(error) => Err(error.into()),
                RetirementStep::Complete => Ok(RetainedCloneStep::Progress(Default::default())),
            };
        }
        if self.pending.is_some() || self.pending_close.is_some() { let turn = semio_plugin::app::plugin_typed_owner_original(&mut self.pending, &mut self.pending_close, grant); *work += turn.work; return turn.result; }
        if self.outputs.is_some() || self.outputs_close.is_some() { let turn = semio_plugin::app::plugin_typed_owner_original(&mut self.outputs, &mut self.outputs_close, grant); *work += turn.work; return turn.result; }
        if let Some(snapshot) = self.snapshot.as_ref() {
            if !self.lease.as_ref().is_some_and(|lease| lease.can_release(snapshot)) { *work = 1; return Err(invalid("mesh export lost its original snapshot close supplier").into()); }
            self.snapshot.take(); *work = 1; return Ok(RetainedCloneStep::Progress(item));
        }
        if let Some(lease) = self.lease.as_mut() {
            if lease.terminal_is_empty() { self.lease.take(); *work = 1; return Ok(RetainedCloneStep::Progress(item)); }
            *work = 1; return lease.close_step(grant).map_err(Into::into);
        }
        if self.metadata.is_some() || self.metadata_close.is_some() { let turn = semio_plugin::app::plugin_typed_owner_original(&mut self.metadata, &mut self.metadata_close, grant); *work += turn.work; return turn.result; }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl<A: ArtifactApp> InteractiveJob for MeshExportJob<A> where A::Snapshot: MeshExportSnapshot {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>, _claim: &mut OriginalJobReturnClaim<'_>) -> InteractiveJobStep<'a> { InteractiveJobStep::from_callback(self.step_turn(cx)) }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> { match descriptor.kind() { JobOutcomeKind::Cancelled => descriptor.cancelled(), JobOutcomeKind::Complete if self.published => descriptor.complete(None, None), JobOutcomeKind::Yield => descriptor.yielded(), _ => self.publication.borrow_outcome(descriptor) } }
    fn begin_close(&mut self) { self.closing = true; if let Some(index) = self.index.as_mut() { index.begin_close(); } if let Some(lookup) = self.lookup.as_mut() { lookup.begin_close(); } }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> { self.begin_close(); let body = cx.retained_grant().maximum_copy_bytes; let quote = self.close_demand(body)?; let work = self.close_work_demand(body)?; let fuel = cx.fuel_remaining(); let step = semio_framework_job::close_original_quoted_retained_step(cx, quote, work, |grant| self.close_turn(grant, fuel))?; Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step }) }
    fn terminal_is_empty(&self) -> bool { self.cursor.is_none() && self.index.is_none() && self.normals.is_none() && self.page.is_none() && self.lookup.is_none() && self.orphan.is_none() && self.orphan_close.is_none() && self.snapshot.is_none() && self.lease.is_none() && self.outputs.is_none() && self.outputs_close.is_none() && self.metadata.is_none() && self.metadata_close.is_none() && self.pending.is_none() && self.pending_close.is_none() && self.publication.terminal_is_empty() && !self.publish_progress && !self.checkpoint_active && !self.checkpoint_closing }
}
impl<A: ArtifactApp> ArtifactReservedJob for MeshExportJob<A> where A::Snapshot: MeshExportSnapshot {}
impl<A: ArtifactApp> Drop for MeshExportJob<A> where A::Snapshot: MeshExportSnapshot { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "mesh export requires original paid close"); } }

pub struct MeshExportFactory<A: ArtifactApp> where A::Snapshot: MeshExportSnapshot { keys: [ToolFactoryKey; 1], owner: std::marker::PhantomData<fn() -> A> }
impl<A: ArtifactApp> MeshExportFactory<A> where A::Snapshot: MeshExportSnapshot {
    pub fn new(controller: &str) -> Self { Self { keys: [ToolFactoryKey::new(controller, TOOL)], owner: std::marker::PhantomData } }
}
impl<A: ArtifactApp> ToolJobFactory for MeshExportFactory<A> where A::Snapshot: MeshExportSnapshot {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(4096, 4096, 4096, ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, 2000, 64, 1) }
    fn admission_demands(&self, _operation: &semio_framework_job::Operation, _payload: &Self::Payload, input: Option<&semio_framework::action_bus::RetainedToolWireInput>, checkpoint: Option<&semio_framework::action_bus::RetainedToolWireInput>) -> Result<semio_framework::action_bus::ToolJobAdmissionDemand, ValueError> {
        if input.is_some() || checkpoint.is_some() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original mesh export does not accept replacement input or checkpoint")); }
        Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())
    }
    fn admit_job(&mut self, context: &mut StepContext<'_>, operation: &semio_framework_job::Operation, payload: &mut Option<Box<Self::Payload>>, input: &mut Option<semio_framework::action_bus::RetainedToolWireInput>, checkpoint: &mut Option<semio_framework::action_bus::RetainedToolWireInput>, output: &mut Option<Box<Self::Job>>) -> Result<bool, OriginalTurnFailure> {
        let demand = self.admission_demands(operation, payload.as_deref().ok_or_else(|| invalid("original mesh export payload is absent"))?, input.as_ref(), checkpoint.as_ref())?;
        semio_framework::action_bus::admit_original_tool_job_controlled(context, operation, payload, input, checkpoint, output, demand, |_, payload, _, _| {
            payload.as_mut().expect("retained original mesh job").bind_operation(*operation)?;
            let job = payload.take().expect("admitted original mesh job");
            Ok((job, RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        })
    }
}
impl<A: ArtifactApp> ArtifactOwnedToolJobFactory for MeshExportFactory<A> where A::Snapshot: MeshExportSnapshot {
    type Owner = A;
    const TOOL_IDS: &'static [&'static str] = &[TOOL];
    const DOCUMENT_SCHEMA: &'static str = A::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TOOL, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}
