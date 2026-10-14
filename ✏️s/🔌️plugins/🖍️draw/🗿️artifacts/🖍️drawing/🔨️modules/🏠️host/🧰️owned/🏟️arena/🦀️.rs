//! 🏟️ Drawing host arena: the owned-field catalog, the mutation arena pool, the snapshot census and bounds, the native clone authorities, the layer locator and the container rebuild.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::DrawingImageAsset;
use semio_s_artifact_draw_drawing_model::FillStyle;
use semio_s_artifact_draw_drawing_model::StrokeStyle;
use semio_s_artifact_draw_drawing_model::PathSegment;
/// ♻️ Rebuilt source, removed layer, reverse arena and forward arena.
pub type DrawingRebuiltLayerOwners = (DrawingNativeLayers, Option<DrawingLayerNode>, Vec<DrawingLayerNode>, Vec<DrawingLayerNode>);

pub const DRAWING_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// 🏭️ Framework-owned retirement issuer for decoded Drawing snapshots.
pub type DrawingSnapshotRetirementFactory = semio_framework_value::retirement::OwnedValueRetirementFactory<DrawingSnapshot>;

/// 🏭️ Framework-owned retirement issuer for decoded Drawing mutations.
type DrawingMutationRetirementFactory = semio_framework_value::retirement::OwnedValueRetirementFactory<DrawingMutation>;

fn decode_drawing_snapshot_pack(bytes: &[u8]) -> Result<DrawingSnapshot, ()> {
    <DrawingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_drawing_mutation_pack(bytes: &[u8]) -> Result<DrawingMutation, ()> {
    DrawingMutation::decode_op(bytes).map_err(|_| ())
}

pub struct DrawingDecodedFieldRetirement<T: semio_framework_value::retirement::RetireOwned> {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<T>,
}

impl<T: semio_framework_value::retirement::RetireOwned> DrawingDecodedFieldRetirement<T> {
    pub fn try_new(value: T) -> Result<Self, (semio_framework_value::ValueError, T)> {
        semio_framework_value::retirement::controlled::ControlledRetirement::new(value).map(|owner| Self { owner })
    }

    pub fn demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let copy_bytes = self.owner.next_copy_byte_demand()?;
        Ok(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: self.owner.next_capacity_byte_demand(copy_bytes)?, release_bytes: self.owner.next_release_byte_demand()?, depth: self.owner.next_depth_demand()? })
    }

    pub fn next_grant(&self) -> Result<RetainedCloneGrant, semio_framework_value::ValueError> {
        let demand = self.demands()?;
        Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) })
    }

    pub fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> { self.owner.step(grant) }

    pub fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

macro_rules! drawing_owned_field_close_capacity {
    (@demands, $value:ty) => {
        fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes) }

        fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes) }

        fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes) }

        fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth) }
    };
    (ArtifactEnvelopeSnapshotFieldAuthority, $value:ty) => {
        drawing_owned_field_close_capacity!(@demands, $value);

        fn maximum_close_byte_demand(&self) -> usize {
            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority, $value:ty) => {
        drawing_owned_field_close_capacity!(@demands, $value);
    };
}

macro_rules! drawing_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::os_store::owned_schema_decode::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
                store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { offset, ..store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, self.path) }
            }

            fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if matches!(self.state, $state::Decode(_)) {
                    return Ok(semio_framework_value::RetirementDemand { release_bytes: DRAWING_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
                }
                let demand = match self.retirement.as_ref() {
                    Some(owner) => store::os_store::snapshot_read::artifact_retirement_box_demands(owner, maximum_copy_bytes),
                    None => store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value),
                };
                demand.map_err(|error| self.diagnostic(concat!("drawing-envelope.", $kind, "-retirement-demand-fault"), 0).with_native(error))
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::os_store::owned_schema_decode::OwnedSchemaToken,
                terminal: bool,
                source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { offset, ..store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, path) };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("drawing-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("drawing-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("drawing-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            drawing_owned_field_close_capacity!($authority_trait, $value);

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                let empty = RetainedCloneProgress::default();
                if self.terminal_is_empty() {
                    return Ok(RetainedCloneStep::Complete(empty));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let demand = self.close_demands(grant.maximum_copy_bytes)?;
                if grant.maximum_depth < demand.depth {
                    return Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-retirement-depth"), 0));
                }
                if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: DRAWING_OWNED_FIELD_BYTES, ..empty }));
                }
                let step = if self.retirement.is_some() {
                    store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant)
                } else if self.value.is_some() {
                    store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant)
                } else {
                    Ok(RetainedCloneStep::Complete(empty))
                }
                .map_err(|error| self.diagnostic(concat!("drawing-envelope.", $kind, "-retirement-fault"), 0).with_native(error))?;
                self.state = $state::Closing;
                if self.value.is_none() && self.retirement.is_none() {
                    self.state = $state::Complete;
                    return Ok(RetainedCloneStep::Complete(step.progress()));
                }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(
                    (matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()) || std::thread::panicking(),
                    concat!("Drawing ", $kind, " decode reached Drop before publication or bounded retirement")
                );
            }
        }
    };
}

drawing_owned_field_authority!(
    DrawingSnapshotDecodeState,
    DrawingSnapshotDecodeAuthority,
    DrawingSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_drawing_snapshot_pack,
    "snapshot"
);

drawing_owned_field_authority!(
    DrawingMutationDecodeState,
    DrawingMutationDecodeAuthority,
    DrawingMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_drawing_mutation_pack,
    "mutation"
);

struct DrawingRejectedConflictAuthority {
    pub terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for DrawingRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { offset: token.start, ..store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before("drawing-envelope.fresh-conflict-not-admitted", store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT) })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(usize::from(!self.terminal)) }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DrawingEnvelopeOwnedFieldCatalog;

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<DrawingSnapshot, DrawingMutation> for DrawingEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<DrawingSnapshot, DrawingMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<DrawingSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(DrawingSnapshotRetirementFactory::default()),
            std::sync::Arc::new(DrawingMutationRetirementFactory::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<DrawingSnapshot, DrawingMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<DrawingSnapshot>> {
        Box::new(DrawingSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<DrawingMutation>> {
        Box::new(DrawingMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(DrawingRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<DrawingMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(DrawingMutationRetirementFactory::default()))
    }
}

pub fn drawing_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<DrawingSnapshot, DrawingMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(DrawingEnvelopeOwnedFieldCatalog), std::sync::Arc::new(DrawingSnapshotRetirementFactory::default()), std::sync::Arc::new(DrawingMutationRetirementFactory::default()))
}

pub const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096;

pub const DRAWING_MAXIMUM_NESTED_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64;

pub const DRAWING_MUTATION_RETAINED_PAGE_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

pub const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16;

pub const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64;

const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4;

pub const DRAWING_DUPLICATE_ID_BYTES: usize = 80;

pub struct DrawingMutationArenaOwner {
    pub reverse: Vec<DrawingLayerNode>,
    pub output: Vec<DrawingLayerNode>,
    pub pages: Vec<String>,
    pub duplicate_id: String,
}

impl DrawingMutationArenaOwner {
    pub fn configured_totals() -> Result<(usize, usize), &'static str> {
        let items = DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            .checked_mul(2)
            .and_then(|items| items.checked_add(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY))
            .and_then(|items| items.checked_add(1))
            .ok_or("drawing-store.mutation-arena-item-overflow")?;
        let bytes = size_of::<Self>()
            .checked_add(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY.checked_mul(size_of::<DrawingLayerNode>()).ok_or("drawing-store.mutation-arena-byte-overflow")?)
            .and_then(|bytes| bytes.checked_add(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY.checked_mul(size_of::<DrawingLayerNode>())?))
            .and_then(|bytes| bytes.checked_add(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY.checked_mul(size_of::<String>())?))
            .and_then(|bytes| bytes.checked_add(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY.checked_mul(DRAWING_MUTATION_RETAINED_PAGE_BYTES)?))
            .and_then(|bytes| bytes.checked_add(DRAWING_DUPLICATE_ID_BYTES))
            .ok_or("drawing-store.mutation-arena-byte-overflow")?;
        Ok((items, bytes))
    }

    pub fn admitted_totals(&self) -> Result<(usize, usize), &'static str> {
        Self::retained_totals(&self.reverse, &self.output, &self.pages, &self.duplicate_id)
    }

    pub fn retained_totals(reverse: &Vec<DrawingLayerNode>, output: &Vec<DrawingLayerNode>, pages: &Vec<String>, duplicate_id: &String) -> Result<(usize, usize), &'static str> {
        let items = reverse.capacity().checked_add(output.capacity()).and_then(|items| items.checked_add(pages.capacity())).and_then(|items| items.checked_add(1)).ok_or("drawing-store.mutation-arena-item-overflow")?;
        let bytes = size_of::<Self>()
            .checked_add(reverse.capacity().checked_mul(size_of::<DrawingLayerNode>()).ok_or("drawing-store.mutation-arena-byte-overflow")?)
            .and_then(|bytes| bytes.checked_add(output.capacity().checked_mul(size_of::<DrawingLayerNode>())?))
            .and_then(|bytes| bytes.checked_add(pages.capacity().checked_mul(size_of::<String>())?))
            .and_then(|bytes| pages.iter().try_fold(bytes, |total, page| total.checked_add(page.capacity())))
            .and_then(|bytes| bytes.checked_add(duplicate_id.capacity()))
            .ok_or("drawing-store.mutation-arena-byte-overflow")?;
        Ok((items, bytes))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.reverse.is_empty()
            && self.reverse.capacity() >= DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.output.is_empty()
            && self.output.capacity() >= DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY
            && self.pages.len() == DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.capacity() >= DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
            && self.pages.iter().all(|page| page.is_empty() && page.capacity() >= DRAWING_MUTATION_RETAINED_PAGE_BYTES)
            && self.duplicate_id.is_empty()
            && self.duplicate_id.capacity() >= DRAWING_DUPLICATE_ID_BYTES
    }
}

struct DrawingMutationArenaOwnerBuilder {
    pub reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pub pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    pub duplicate_id: std::mem::ManuallyDrop<Option<String>>,
    pub rejected_string: std::mem::ManuallyDrop<Option<String>>,
    pub phase: usize,
    pub terminal: bool,
}

impl DrawingMutationArenaOwnerBuilder {
    pub fn new() -> Self {
        Self {
            reverse: std::mem::ManuallyDrop::new(None),
            output: std::mem::ManuallyDrop::new(None),
            pages: std::mem::ManuallyDrop::new(None),
            duplicate_id: std::mem::ManuallyDrop::new(None),
            rejected_string: std::mem::ManuallyDrop::new(None),
            phase: 0,
            terminal: false,
        }
    }

    pub fn from_owner(owner: DrawingMutationArenaOwner) -> Self {
        Self {
            reverse: std::mem::ManuallyDrop::new(Some(owner.reverse)),
            output: std::mem::ManuallyDrop::new(Some(owner.output)),
            pages: std::mem::ManuallyDrop::new(Some(owner.pages)),
            duplicate_id: std::mem::ManuallyDrop::new(Some(owner.duplicate_id)),
            rejected_string: std::mem::ManuallyDrop::new(None),
            phase: 20,
            terminal: false,
        }
    }

    pub fn inject(allocation: &mut usize, failure_at: Option<usize>) -> bool {
        let current = *allocation;
        *allocation += 1;
        failure_at == Some(current)
    }

    pub fn step(&mut self, allocation: &mut usize, failure_at: Option<usize>) -> Result<bool, &'static str> {
        if self.phase >= 20 {
            return Ok(true);
        }
        if Self::inject(allocation, failure_at) {
            return Err("drawing-store.mutation-arena-bootstrap-injected-allocation");
        }
        match self.phase {
            0 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-reverse-arena-admission");
                }
                *self.reverse = Some(value);
            }
            1 => {
                let mut value = Vec::new();
                if value.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-output-arena-admission");
                }
                *self.output = Some(value);
            }
            2 => {
                let mut pages = Vec::new();
                if pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY).is_err() {
                    return Err("drawing-store.mutation-overlay-arena-admission");
                }
                *self.pages = Some(pages);
            }
            phase if phase < 3 + DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY => {
                let mut page = String::new();
                if page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES).is_err() {
                    return Err("drawing-store.mutation-overlay-page-admission");
                }
                let Some(pages) = self.pages.as_mut() else {
                    *self.rejected_string = Some(page);
                    return Err("drawing-store.mutation-overlay-arena-missing");
                };
                if pages.len() >= pages.capacity() {
                    *self.rejected_string = Some(page);
                    return Err("drawing-store.mutation-overlay-arena-saturated");
                }
                pages.push(page);
            }
            19 => {
                let mut value = String::new();
                if value.try_reserve_exact(DRAWING_DUPLICATE_ID_BYTES).is_err() {
                    return Err("drawing-store.duplicate-id-owner-admission");
                }
                *self.duplicate_id = Some(value);
            }
            _ => return Err("drawing-store.mutation-arena-bootstrap-phase"),
        }
        self.phase += 1;
        Ok(self.phase == 20)
    }

    pub fn take(&mut self) -> Option<DrawingMutationArenaOwner> {
        if self.phase != 20 || self.terminal {
            return None;
        }
        if self.rejected_string.is_some() || self.reverse.is_none() || self.output.is_none() || self.pages.is_none() || self.duplicate_id.is_none() {
            return None;
        }
        let owner = DrawingMutationArenaOwner {
            reverse: self.reverse.take().expect("validated Drawing reverse bootstrap owner remains retained"),
            output: self.output.take().expect("validated Drawing output bootstrap owner remains retained"),
            pages: self.pages.take().expect("validated Drawing page bootstrap owner remains retained"),
            duplicate_id: self.duplicate_id.take().expect("validated Drawing duplicate bootstrap owner remains retained"),
        };
        self.terminal = true;
        Some(owner)
    }

    pub fn close_step(&mut self) -> RetainedCloneStep {
        let released = |released_bytes: usize| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() });
        if let Some(value) = self.rejected_string.take() {
            let released_bytes = value.capacity();
            drop(value);
            return released(released_bytes);
        }
        if let Some(value) = self.duplicate_id.take() {
            let released_bytes = value.capacity();
            drop(value);
            return released(released_bytes);
        }
        if let Some(pages) = self.pages.as_mut() {
            if let Some(value) = pages.pop() {
                let released_bytes = value.capacity();
                drop(value);
                return released(released_bytes);
            }
        }
        if let Some(value) = self.pages.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<String>());
            drop(value);
            return released(released_bytes);
        }
        if let Some(value) = self.output.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<DrawingLayerNode>());
            drop(value);
            return released(released_bytes);
        }
        if let Some(value) = self.reverse.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<DrawingLayerNode>());
            drop(value);
            return released(released_bytes);
        }
        self.terminal = true;
        RetainedCloneStep::Complete(RetainedCloneProgress::default())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.reverse.is_none() && self.output.is_none() && self.pages.is_none() && self.duplicate_id.is_none() && self.rejected_string.is_none()
    }
}

impl Drop for DrawingMutationArenaOwnerBuilder {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation arena owner builder reached Drop before exact construction handoff or retirement");
    }
}

struct DrawingMutationArenaPoolSlot {
    pub reverse: Option<Vec<DrawingLayerNode>>,
    pub output: Option<Vec<DrawingLayerNode>>,
    pub pages: Option<Vec<String>>,
    pub duplicate_id: Option<String>,
    pub generation: u64,
    pub leased: bool,
}

impl DrawingMutationArenaPoolSlot {
    pub fn new(owner: DrawingMutationArenaOwner) -> Self {
        Self { reverse: Some(owner.reverse), output: Some(owner.output), pages: Some(owner.pages), duplicate_id: Some(owner.duplicate_id), generation: 0, leased: false }
    }

    pub fn is_available(&self) -> bool {
        !self.leased && self.reverse.is_some() && self.output.is_some() && self.pages.is_some() && self.duplicate_id.is_some()
    }

    pub fn take(&mut self, generation: u64) -> Option<DrawingMutationArenaOwner> {
        if !self.is_available() {
            return None;
        }
        self.generation = generation;
        self.leased = true;
        Some(DrawingMutationArenaOwner {
            reverse: self.reverse.take().expect("available Drawing pool slot retains reverse owner"),
            output: self.output.take().expect("available Drawing pool slot retains output owner"),
            pages: self.pages.take().expect("available Drawing pool slot retains page owner"),
            duplicate_id: self.duplicate_id.take().expect("available Drawing pool slot retains duplicate owner"),
        })
    }
}

struct DrawingMutationArenaPoolState {
    pub slots: [DrawingMutationArenaPoolSlot; DRAWING_MUTATION_ARENA_POOL_CAPACITY],
}

pub struct DrawingMutationArenaPool {
    pub state: std::sync::Mutex<DrawingMutationArenaPoolState>,
    pub admitted_items: usize,
    pub admitted_bytes: usize,
}

struct DrawingMutationArenaPoolBootstrap {
    pub owners: std::mem::ManuallyDrop<[Option<DrawingMutationArenaOwner>; DRAWING_MUTATION_ARENA_POOL_CAPACITY]>,
    pub active: std::mem::ManuallyDrop<Option<DrawingMutationArenaOwnerBuilder>>,
    pub owner: usize,
    pub allocation: usize,
    pub failure_at: Option<usize>,
    pub failure_after_owner: Option<usize>,
    pub maximum_items: usize,
    pub maximum_bytes: usize,
    pub admitted_items: usize,
    pub admitted_bytes: usize,
    pub ready: bool,
    pub fault: Option<&'static str>,
    pub terminal: bool,
}

impl std::fmt::Debug for DrawingMutationArenaPoolBootstrap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DrawingMutationArenaPoolBootstrap").field("owner", &self.owner).field("allocation", &self.allocation).field("fault", &self.fault).finish()
    }
}

impl DrawingMutationArenaPoolBootstrap {
    pub fn new(failure_at: Option<usize>, failure_after_owner: Option<usize>, maximum_items: usize, maximum_bytes: usize) -> Self {
        Self {
            owners: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            active: std::mem::ManuallyDrop::new(None),
            owner: 0,
            allocation: 0,
            failure_at,
            failure_after_owner,
            maximum_items,
            maximum_bytes,
            admitted_items: 0,
            admitted_bytes: 0,
            ready: false,
            fault: None,
            terminal: false,
        }
    }

    pub fn production(admission: DrawingMutationArenaBootstrapAdmission) -> Self {
        Self::new(None, None, admission.maximum_items, admission.maximum_bytes)
    }

    pub fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        Err(fault)
    }

    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() {
            return Ok(false);
        }
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.ready {
            return Ok(true);
        }
        if self.owner == DRAWING_MUTATION_ARENA_POOL_CAPACITY {
            if self.admitted_items > self.maximum_items || self.admitted_bytes > self.maximum_bytes {
                return self.fail("drawing-store.mutation-arena-pool-capacity");
            }
            self.ready = true;
            return Ok(true);
        }
        if self.active.is_none() {
            *self.active = Some(DrawingMutationArenaOwnerBuilder::new());
            return Ok(false);
        }
        let complete = match self.active.as_mut().expect("Drawing arena owner builder remains retained").step(&mut self.allocation, self.failure_at) {
            Ok(complete) => complete,
            Err(error) => return self.fail(error),
        };
        if !complete {
            return Ok(false);
        }
        let mut builder = self.active.take().expect("completed Drawing arena owner builder remains retained");
        let Some(owner) = builder.take() else {
            *self.active = Some(builder);
            return self.fail("drawing-store.mutation-arena-owner-false-terminal");
        };
        drop(builder);
        if !owner.terminal_is_empty() {
            self.owners[self.owner] = Some(owner);
            return self.fail("drawing-store.mutation-arena-pool-initial-owner");
        }
        let totals = owner.admitted_totals();
        let owner_index = self.owner;
        self.owners[owner_index] = Some(owner);
        self.owner += 1;
        if self.failure_after_owner == Some(owner_index) {
            return self.fail("drawing-store.mutation-arena-bootstrap-injected-owner");
        }
        let (items, bytes) = match totals {
            Ok(totals) => totals,
            Err(error) => return self.fail(error),
        };
        self.admitted_items = match self.admitted_items.checked_add(items) {
            Some(total) => total,
            None => return self.fail("drawing-store.mutation-arena-pool-item-overflow"),
        };
        self.admitted_bytes = match self.admitted_bytes.checked_add(bytes) {
            Some(total) => total,
            None => return self.fail("drawing-store.mutation-arena-pool-byte-overflow"),
        };
        Ok(false)
    }

    pub fn take_pool(&mut self) -> Option<std::sync::Arc<DrawingMutationArenaPool>> {
        if !self.ready || self.terminal {
            return None;
        }
        let owners = std::mem::replace(&mut *self.owners, std::array::from_fn(|_| None));
        let slots = owners.map(|owner| DrawingMutationArenaPoolSlot::new(owner.expect("validated Drawing arena bootstrap retains every owner")));
        self.terminal = true;
        Some(std::sync::Arc::new(DrawingMutationArenaPool { state: std::sync::Mutex::new(DrawingMutationArenaPoolState { slots }), admitted_items: self.admitted_items, admitted_bytes: self.admitted_bytes }))
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> RetainedCloneStep {
        if cx.should_yield() {
            return RetainedCloneStep::Progress(RetainedCloneProgress::default());
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step() {
                RetainedCloneStep::Complete(_) if active.terminal_is_empty() => {
                    drop(self.active.take());
                    RetainedCloneStep::Progress(RetainedCloneProgress::default())
                }
                step => RetainedCloneStep::Progress(step.progress()),
            };
        }
        if self.owner > 0 {
            self.owner -= 1;
            let owner = self.owners[self.owner].take().expect("Drawing bootstrap retirement cursor locates the preceding retained owner");
            *self.active = Some(DrawingMutationArenaOwnerBuilder::from_owner(owner));
            return RetainedCloneStep::Progress(RetainedCloneProgress::default());
        }
        self.terminal = true;
        RetainedCloneStep::Complete(RetainedCloneProgress::default())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.owners.iter().all(Option::is_none)
    }
}

impl Drop for DrawingMutationArenaPoolBootstrap {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation arena pool bootstrap reached Drop before exact handoff or fault retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingMutationArenaBootstrapAdmission {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

impl DrawingMutationArenaBootstrapAdmission {
    pub fn fixed() -> Result<Self, &'static str> {
        let (owner_items, owner_bytes) = DrawingMutationArenaOwner::configured_totals()?;
        Ok(Self {
            maximum_items: owner_items.checked_mul(DRAWING_MUTATION_ARENA_POOL_CAPACITY).ok_or("drawing-store.mutation-arena-bootstrap-item-claim")?,
            maximum_bytes: owner_bytes.checked_mul(DRAWING_MUTATION_ARENA_POOL_CAPACITY).ok_or("drawing-store.mutation-arena-bootstrap-byte-claim")?,
        })
    }
}

enum DrawingMutationArenaProcessState {
    Inert,
    Building(DrawingMutationArenaPoolBootstrap),
    Ready(std::sync::Arc<DrawingMutationArenaPool>),
    Retiring(DrawingMutationArenaPoolBootstrap),
    Fault(&'static str),
}

static DRAWING_MUTATION_ARENA_POOL: std::sync::OnceLock<std::sync::Mutex<DrawingMutationArenaProcessState>> = std::sync::OnceLock::new();

static DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaPoolAvailability {
    Ready,
    NotReady,
    Contended,
    Fault(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaBorrowError {
    NotReady,
    Contended,
    Fault(&'static str),
    Invalid(&'static str),
}

impl DrawingMutationArenaBorrowError {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotReady => "drawing-store.mutation-arena-bootstrap-not-ready",
            Self::Contended => "drawing-store.mutation-arena-pool-contended",
            Self::Fault(fault) | Self::Invalid(fault) => fault,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingMutationArenaBootstrapStep {
    Pending { advanced_items: u64 },
    Blocked,
    Ready,
    Cancelled,
    Fault(&'static str),
}

pub struct DrawingMutationArenaBootstrapJob {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub admission: DrawingMutationArenaBootstrapAdmission,
    pub terminal: bool,
}

impl DrawingMutationArenaBootstrapJob {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        request_drawing_mutation_arena_pool();
        Ok(Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission::fixed()?, terminal: false })
    }

    pub fn inactive(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission { maximum_items: 0, maximum_bytes: 0 }, terminal: true }
    }

    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
        if self.terminal {
            return DrawingMutationArenaBootstrapStep::Ready;
        }
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.terminal = true;
            return DrawingMutationArenaBootstrapStep::Fault("drawing-store.mutation-arena-bootstrap-stale-authority");
        }
        if cx.should_yield() {
            return DrawingMutationArenaBootstrapStep::Blocked;
        }
        let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
        let Ok(mut state) = state.try_lock() else {
            return DrawingMutationArenaBootstrapStep::Blocked;
        };
        self.step_locked(&mut state, cx)
    }

    pub fn step_locked(&mut self, state: &mut DrawingMutationArenaProcessState, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
        cx.set_stage("drawing-arena-bootstrap");
        if cx.is_cancelled() {
            match &*state {
                DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Ready(_) => {
                    self.terminal = true;
                    return DrawingMutationArenaBootstrapStep::Cancelled;
                }
                DrawingMutationArenaProcessState::Building(_) => {
                    let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                    let DrawingMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Drawing bootstrap cancellation preserves its exact building owner") };
                    bootstrap.fault = Some("drawing-store.mutation-arena-bootstrap-cancelled");
                    *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                    cx.consume_fuel(1);
                    return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                }
                DrawingMutationArenaProcessState::Retiring(_) => {}
                DrawingMutationArenaProcessState::Fault(fault) => {
                    self.terminal = true;
                    return DrawingMutationArenaBootstrapStep::Fault(fault);
                }
            }
        }
        let transition = match &mut *state {
            DrawingMutationArenaProcessState::Inert => {
                if !DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    return DrawingMutationArenaBootstrapStep::Blocked;
                }
                *state = DrawingMutationArenaProcessState::Building(DrawingMutationArenaPoolBootstrap::production(self.admission));
                cx.consume_fuel(1);
                return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
            }
            DrawingMutationArenaProcessState::Building(bootstrap) => match bootstrap.step(cx) {
                Ok(true) => DrawingMutationArenaProcessTransition::Publish,
                Ok(false) => DrawingMutationArenaProcessTransition::None,
                Err(_) => DrawingMutationArenaProcessTransition::Retire,
            },
            DrawingMutationArenaProcessState::Ready(_) => {
                self.terminal = true;
                return DrawingMutationArenaBootstrapStep::Ready;
            }
            DrawingMutationArenaProcessState::Fault(error) => {
                self.terminal = true;
                return DrawingMutationArenaBootstrapStep::Fault(error);
            }
            DrawingMutationArenaProcessState::Retiring(bootstrap) => {
                let fault = bootstrap.fault.unwrap_or("drawing-store.mutation-arena-bootstrap-fault");
                if matches!(bootstrap.close_step(cx), RetainedCloneStep::Complete(_)) && bootstrap.terminal_is_empty() {
                    DrawingMutationArenaProcessTransition::Fault(fault)
                } else {
                    DrawingMutationArenaProcessTransition::None
                }
            }
        };
        cx.consume_fuel(1);
        match transition {
            DrawingMutationArenaProcessTransition::None => DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 },
            DrawingMutationArenaProcessTransition::Publish => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                let DrawingMutationArenaProcessState::Building(mut bootstrap) = previous else { unreachable!("Drawing arena publish transition preserves the building owner") };
                let Some(pool) = bootstrap.take_pool() else {
                    *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                    return DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 };
                };
                drop(bootstrap);
                *state = DrawingMutationArenaProcessState::Ready(pool);
                self.terminal = true;
                DrawingMutationArenaBootstrapStep::Ready
            }
            DrawingMutationArenaProcessTransition::Retire => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault("drawing-store.mutation-arena-bootstrap-transition"));
                let DrawingMutationArenaProcessState::Building(bootstrap) = previous else { unreachable!("Drawing arena fault transition preserves the building owner") };
                *state = DrawingMutationArenaProcessState::Retiring(bootstrap);
                DrawingMutationArenaBootstrapStep::Pending { advanced_items: 1 }
            }
            DrawingMutationArenaProcessTransition::Fault(fault) => {
                let previous = std::mem::replace(&mut *state, DrawingMutationArenaProcessState::Fault(fault));
                let DrawingMutationArenaProcessState::Retiring(bootstrap) = previous else { unreachable!("Drawing arena terminal fault transition preserves the retirement owner") };
                drop(bootstrap);
                self.terminal = true;
                if fault == "drawing-store.mutation-arena-bootstrap-cancelled" {
                    DrawingMutationArenaBootstrapStep::Cancelled
                } else {
                    DrawingMutationArenaBootstrapStep::Fault(fault)
                }
            }
        }
    }
}

impl DrawingMutationArenaPool {
    #[cfg(test)]
    pub fn try_new() -> Result<std::sync::Arc<Self>, DrawingMutationArenaPoolBootstrap> {
        let mut bootstrap = DrawingMutationArenaPoolBootstrap::production(DrawingMutationArenaBootstrapAdmission::fixed().expect("fixed Drawing arena bootstrap claim"));
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..=DRAWING_MUTATION_ARENA_POOL_CAPACITY * 24 {
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(7_901),
                semio_framework_job::Generation(79),
                semio_framework_job::StepBudget::new(1, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            match bootstrap.step(&mut context) {
                Ok(true) => return Ok(bootstrap.take_pool().expect("completed Drawing arena bootstrap publishes exact pool")),
                Ok(false) => {}
                Err(_) => return Err(bootstrap),
            }
        }
        bootstrap.fault = Some("drawing-store.mutation-arena-bootstrap-turn-capacity");
        Err(bootstrap)
    }
}

enum DrawingMutationArenaProcessTransition {
    None,
    Publish,
    Retire,
    Fault(&'static str),
}

pub fn request_drawing_mutation_arena_pool() -> DrawingMutationArenaPoolAvailability {
    DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true, std::sync::atomic::Ordering::Release);
    let state = DRAWING_MUTATION_ARENA_POOL.get_or_init(|| std::sync::Mutex::new(DrawingMutationArenaProcessState::Inert));
    let Ok(state) = state.try_lock() else {
        return DrawingMutationArenaPoolAvailability::Contended;
    };
    match &*state {
        DrawingMutationArenaProcessState::Ready(_) => DrawingMutationArenaPoolAvailability::Ready,
        DrawingMutationArenaProcessState::Fault(fault) => DrawingMutationArenaPoolAvailability::Fault(fault),
        DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Building(_) | DrawingMutationArenaProcessState::Retiring(_) => DrawingMutationArenaPoolAvailability::NotReady,
    }
}

pub fn drawing_mutation_arena_pool_fault() -> Option<&'static str> {
    let state = DRAWING_MUTATION_ARENA_POOL.get()?;
    let state = state.try_lock().ok()?;
    match &*state {
        DrawingMutationArenaProcessState::Fault(fault) => Some(*fault),
        DrawingMutationArenaProcessState::Inert | DrawingMutationArenaProcessState::Building(_) | DrawingMutationArenaProcessState::Ready(_) | DrawingMutationArenaProcessState::Retiring(_) => None,
    }
}

pub fn borrow_drawing_mutation_arena_from(pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), &'static str> {
    if pool.admitted_items == 0 || pool.admitted_bytes == 0 {
        return Err("drawing-store.mutation-arena-pool-unadmitted");
    }
    let mut state = pool.state.try_lock().map_err(|_| "drawing-store.mutation-arena-pool-contended")?;
    let slot = state.slots.iter().position(DrawingMutationArenaPoolSlot::is_available).ok_or("drawing-store.mutation-arena-pool-saturated")?;
    let generation = state.slots[slot].generation.checked_add(1).ok_or("drawing-store.mutation-arena-generation-exhausted")?;
    let owner = state.slots[slot].take(generation).ok_or("drawing-store.mutation-arena-owner-missing")?;
    drop(state);
    Ok((pool, slot, generation, owner))
}

pub fn borrow_drawing_mutation_arena() -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), DrawingMutationArenaBorrowError> {
    match request_drawing_mutation_arena_pool() {
        DrawingMutationArenaPoolAvailability::Ready => {}
        DrawingMutationArenaPoolAvailability::NotReady => return Err(DrawingMutationArenaBorrowError::NotReady),
        DrawingMutationArenaPoolAvailability::Contended => return Err(DrawingMutationArenaBorrowError::Contended),
        DrawingMutationArenaPoolAvailability::Fault(fault) => return Err(DrawingMutationArenaBorrowError::Fault(fault)),
    }
    let state = DRAWING_MUTATION_ARENA_POOL.get().ok_or(DrawingMutationArenaBorrowError::Invalid("drawing-store.mutation-arena-pool-uninitialized"))?;
    let state = state.try_lock().map_err(|_| DrawingMutationArenaBorrowError::Contended)?;
    let DrawingMutationArenaProcessState::Ready(pool) = &*state else { return Err(DrawingMutationArenaBorrowError::NotReady) };
    let pool = pool.clone();
    drop(state);
    borrow_drawing_mutation_arena_from(pool).map_err(DrawingMutationArenaBorrowError::Invalid)
}

#[derive(Clone, Copy)]
pub struct DrawingTraversalFrame {
    pub phase: u8,
    pub child: usize,
    pub string: usize,
}

impl DrawingTraversalFrame {
    pub const EMPTY: Self = Self { phase: 0, child: 0, string: 0 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingSnapshotOwnerTotals {
    pub source_items: usize,
    pub source_bytes: usize,
    pub candidate_items: usize,
    pub candidate_bytes: usize,
    pub maximum_container: usize,
}

#[derive(Clone, Copy)]
struct DrawingOwnerCreditSlot {
    pub source_items: u32,
    pub source_bytes: u32,
    pub derived_items: u32,
    pub derived_bytes: u32,
}

impl DrawingOwnerCreditSlot {
    pub const EMPTY: Self = Self { source_items: 0, source_bytes: 0, derived_items: 0, derived_bytes: 0 };
}

pub struct DrawingFixedOwnerCensus {
    pub slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS],
    pub length: usize,
}

impl DrawingFixedOwnerCensus {
    pub fn new() -> Self {
        Self { slots: [DrawingOwnerCreditSlot::EMPTY; DRAWING_MAXIMUM_NESTED_ITEMS], length: 0 }
    }

    pub fn admit(&mut self, source_items: usize, source_bytes: usize, derived_items: usize, derived_bytes: usize) -> Result<DrawingOwnerCreditSlot, &'static str> {
        let target = self.slots.get_mut(self.length).ok_or("drawing-store.owner-census-slot-capacity")?;
        *target = DrawingOwnerCreditSlot {
            source_items: source_items.try_into().map_err(|_| "drawing-store.owner-census-item-width")?,
            source_bytes: source_bytes.try_into().map_err(|_| "drawing-store.owner-census-byte-width")?,
            derived_items: derived_items.try_into().map_err(|_| "drawing-store.owner-census-item-width")?,
            derived_bytes: derived_bytes.try_into().map_err(|_| "drawing-store.owner-census-byte-width")?,
        };
        self.length += 1;
        Ok(*target)
    }
}

struct DrawingAssetBoundsCursor {
    pub index: usize,
    pub owner: Option<usize>,
}

impl DrawingAssetBoundsCursor {
    pub fn new() -> Self { Self { index: 0, owner: None } }

    pub fn next<'a>(&mut self, assets: &'a semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>) -> Result<Option<(&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>, &'a DrawingImageAsset)>, &'static str> {
        let owner = assets as *const _ as usize;
        if self.owner.is_some_and(|expected| expected != owner) { return Err("drawing-store.preflight-asset-owner-changed"); }
        self.owner = Some(owner);
        Ok(assets.entry_at(self.index))
    }

    pub fn advance(&mut self) -> Result<(), &'static str> {
        self.index = self.index.checked_add(1).ok_or("drawing-store.preflight-asset-index-overflow")?;
        Ok(())
    }
}

pub struct DrawingSnapshotBoundsAuthority {
    pub root: usize,
    pub asset_cursor: DrawingAssetBoundsCursor,
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub items: usize,
    pub bytes: usize,
    pub candidate_items: usize,
    pub candidate_bytes: usize,
    pub maximum_container: usize,
    pub owner_census: DrawingFixedOwnerCensus,
    pub direct: native_text_footprint::DrawingLayerFootprintCursor,
    pub record: native_text_footprint::DrawingRecordFootprintCursor,
    pub root_complete: bool,
    pub layers_complete: bool,
    pub terminal: bool,
}

impl DrawingSnapshotBoundsAuthority {
    pub fn new() -> Self {
        Self {
            root: 0,
            asset_cursor: DrawingAssetBoundsCursor::new(),
            depth: 0,
            path: [0; DRAWING_MAXIMUM_LAYER_DEPTH],
            frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH],
            items: 0,
            bytes: 0,
            candidate_items: 0,
            candidate_bytes: 0,
            maximum_container: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            direct: native_text_footprint::DrawingLayerFootprintCursor::default(),
            record: native_text_footprint::DrawingRecordFootprintCursor::default(),
            root_complete: false,
            layers_complete: false,
            terminal: false,
        }
    }

    pub fn layer_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut value = root;
        for index in path {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    pub fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        let credit = self.owner_census.admit(items, bytes, candidate_items, candidate_bytes)?;
        self.items = self.items.checked_add(credit.source_items as usize).ok_or("drawing-store.preflight-item-overflow")?;
        self.bytes = self.bytes.checked_add(credit.source_bytes as usize).ok_or("drawing-store.preflight-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(credit.derived_items as usize).ok_or("drawing-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(credit.derived_bytes as usize).ok_or("drawing-store.preflight-candidate-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS || self.candidate_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.preflight-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES || self.candidate_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.preflight-byte-capacity");
        }
        Ok(())
    }

    pub fn step(&mut self, source: &DrawingSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.layers_complete {
            self.maximum_container = self.maximum_container.max(source.layers.len());
            let Some(root) = source.layers.get(self.root) else {
                self.layers_complete = true;
                cx.consume_fuel(1);
                return Ok(false);
            };
            let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.preflight-path")?;
            let frame = self.frames[self.depth];
            if frame.phase == 0 {
                let complete = self.direct.step(layer, 1)?;
                cx.consume_fuel(1);
                if !complete { return Ok(false); }
                let totals = self.direct.totals().ok_or("drawing-store.preflight-layer-incomplete")?;
                self.add(totals.items, totals.backing_bytes, 0, 0)?;
                self.direct.close_step(1);
                self.direct = native_text_footprint::DrawingLayerFootprintCursor::default();
                self.frames[self.depth].phase = 1;
                return Ok(false);
            }
            if let DrawingLayerNode::Group(value) = layer {
                self.maximum_container = self.maximum_container.max(value.children.len());
                if frame.child < value.children.len() {
                    if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                        return Err("drawing-store.preflight-depth-capacity");
                    }
                    self.path[self.depth] = frame.child;
                    self.frames[self.depth].child += 1;
                    self.depth += 1;
                    self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
            }
            if self.depth == 0 {
                self.root += 1;
                self.frames[0] = DrawingTraversalFrame::EMPTY;
            } else {
                self.depth -= 1;
            }
            cx.consume_fuel(1);
            return Ok(false);
        }
        if !self.root_complete {
            let complete = self.record.step(native_text_footprint::DrawingRecordFootprintSource::Snapshot(source), 1)?;
            cx.consume_fuel(1);
            if !complete { return Ok(false); }
            let totals = self.record.totals().ok_or("drawing-store.preflight-root-incomplete")?;
            self.add(totals.items, totals.backing_bytes.checked_add(size_of::<DrawingSnapshot>()).ok_or("drawing-store.preflight-byte-overflow")?, 0, 0)?;
            self.record.close_step(1);
            self.record = native_text_footprint::DrawingRecordFootprintCursor::default();
            self.root_complete = true;
            return Ok(false);
        }
        let Some((key, value)) = self.asset_cursor.next(&source.assets)? else {
            self.terminal = true;
            return Ok(true);
        };
        let complete = self.record.step(native_text_footprint::DrawingRecordFootprintSource::Asset(key, value), 1)?;
        cx.consume_fuel(1);
        if !complete { return Ok(false); }
        let totals = self.record.totals().ok_or("drawing-store.preflight-asset-incomplete")?;
        self.add(totals.items, totals.backing_bytes, 0, 0)?;
        self.record.close_step(1);
        self.record = native_text_footprint::DrawingRecordFootprintCursor::default();
        self.asset_cursor.advance()?;
        Ok(false)
    }

    pub fn totals(&self) -> Option<DrawingSnapshotOwnerTotals> {
        self.terminal.then_some(DrawingSnapshotOwnerTotals { source_items: self.items, source_bytes: self.bytes, candidate_items: self.candidate_items, candidate_bytes: self.candidate_bytes, maximum_container: self.maximum_container })
    }
}

pub struct DrawingNativeCloneAuthority<T: RetainedClone> {
    pub cursor: T::Cursor,
    pub value: std::mem::ManuallyDrop<Option<T>>,
    pub retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>>,
    pub phase: u8,
    pub terminal: bool,
}

impl<T: RetainedClone> DrawingNativeCloneAuthority<T> {
    pub fn new() -> Self {
        Self {
            cursor: T::retained_clone_cursor(),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            terminal: false,
        }
    }

    /// 🧬️ Clones from a sealed original source ref; every turn is funded by the job's retained wallet.
    pub fn step(&mut self, source: RetainedCloneRef<'_, T>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal { return Ok(true); }
        let grant = cx.retained_grant();
        match self.phase {
            0 => {
                let step = self.cursor.advance(source, grant).map_err(|_| "drawing-store.layer-native-clone")?;
                cx.consume_retained(step.progress()).map_err(|_| "drawing-store.layer-native-clone-grant")?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 1; }
            }
            1 => {
                let bytes = size_of::<T>();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes { return Ok(false); }
                *self.value = self.cursor.take();
                if self.value.is_none() { return Err("drawing-store.layer-clone-false-terminal"); }
                self.cursor.begin_close();
                self.phase = 2;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }).map_err(|_| "drawing-store.layer-clone-take-grant")?;
                cx.consume_fuel(bytes.max(1) as u64);
            }
            2 => {
                let step = self.cursor.close_step(grant).map_err(|_| "drawing-store.layer-native-clone-close")?;
                cx.consume_retained(step.progress()).map_err(|_| "drawing-store.layer-native-clone-close-grant")?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) && self.cursor.terminal_is_empty() { self.terminal = true; }
            }
            _ => return Err("drawing-store.layer-clone-phase"),
        }
        Ok(self.terminal)
    }

    pub fn take(&mut self) -> Option<T> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.cursor.terminal_is_empty() && self.value.is_none() && self.retirement.is_none()
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::ValueError>{
        if !self.cursor.terminal_is_empty(){self.cursor.begin_close();return self.cursor.close_step(grant);}
        if self.value.is_some(){
            if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<T>(){return Ok(RetainedCloneStep::Progress(Default::default()));}
            let value=self.value.take().unwrap();match semio_framework_value::retirement::controlled::ControlledRetirement::new(value){Ok(owner)=>*self.retirement=Some(owner),Err((error,value))=>{*self.value=Some(value);return Err(error);}}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}));
        }
        if let Some(owner)=self.retirement.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.retirement.take();}return Ok(step);}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub fn close_demands(&self,copy:usize)->Result<RetainedCloneGrant,semio_framework_value::ValueError>{
        if !self.cursor.terminal_is_empty(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.cursor.next_close_copy_byte_demand()?,maximum_capacity_bytes:self.cursor.next_close_capacity_byte_demand(copy)?,maximum_release_bytes:self.cursor.next_close_release_byte_demand()?,maximum_depth:self.cursor.next_close_depth_demand()?});}
        if self.value.is_some(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1});}
        if let Some(owner)=self.retirement.as_ref(){return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand()?,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy)?,maximum_release_bytes:owner.next_release_byte_demand()?,maximum_depth:owner.next_depth_demand()?});}
        Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0})
    }
}

impl<T: RetainedClone> Drop for DrawingNativeCloneAuthority<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing layer clone reached Drop before exact handoff or cursor retirement");
    }
}

pub type DrawingLayerCloneAuthority = DrawingNativeCloneAuthority<DrawingLayerNode>;

pub type DrawingFillCloneAuthority = DrawingNativeCloneAuthority<FillStyle>;

pub type DrawingStrokeCloneAuthority = DrawingNativeCloneAuthority<StrokeStyle>;

pub type DrawingNativeLayers = semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}

pub type DrawingNativeText = semio_framework_value::paged::PagedUtf8<{usize::MAX}

pub type DrawingNativeSegments = semio_framework_value::list::PagedList<PathSegment, {usize::MAX}

pub type DrawingTextCloneAuthority = DrawingNativeCloneAuthority<DrawingNativeText>;

pub type DrawingSegmentsCloneAuthority = DrawingNativeCloneAuthority<DrawingNativeSegments>;

impl DrawingNativeCloneAuthority<DrawingLayerNode> {
    pub fn target_at_mut<'a>(root: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
        if let Some((head, tail)) = path.split_first() {
            let DrawingLayerNode::Group(group) = root else { return None };
            return Self::target_at_mut(group.children.get_mut(*head)?, tail);
        }
        Some(root)
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingCloneWorkTotals {
    pub items: usize,
    pub bytes: usize,
}

pub struct DrawingLayerCloneWorkAuthority {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub items: usize,
    pub bytes: usize,
    pub direct: native_text_footprint::DrawingLayerFootprintCursor,
    pub terminal: bool,
}

impl DrawingLayerCloneWorkAuthority {
    pub fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], items: 1, bytes: size_of::<DrawingLayerNode>(), direct: native_text_footprint::DrawingLayerFootprintCursor::default(), terminal: false }
    }

    pub fn add(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("drawing-store.mutation-clone-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("drawing-store.mutation-clone-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-clone-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-clone-byte-capacity");
        }
        Ok(())
    }

    pub fn vector<T, const N: usize>(value: &semio_framework_value::list::PagedList<T, N>) -> Result<(usize, usize), &'static str> {
        Ok((1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-clone-item-overflow")?, value.allocated_bytes()))
    }

    pub fn step(&mut self, root: &DrawingLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let layer = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.mutation-clone-work-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            let complete = self.direct.step(layer, 1)?;
            cx.consume_fuel(1);
            if !complete { return Ok(false); }
            let totals = self.direct.totals().ok_or("drawing-store.mutation-clone-work-incomplete")?;
            self.add(totals.items, totals.backing_bytes)?;
            self.direct.close_step(1);
            self.direct = native_text_footprint::DrawingLayerFootprintCursor::default();
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        if let DrawingLayerNode::Group(value) = layer {
            if frame.child < value.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                    return Err("drawing-store.mutation-clone-work-depth-capacity");
                }
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);
                return Ok(false);
            }
        }
        if self.depth == 0 {
            self.terminal = true;
            Ok(true)
        } else {
            self.depth -= 1;
            cx.consume_fuel(1);
            Ok(false)
        }
    }

    pub fn totals(&self) -> Option<DrawingCloneWorkTotals> {
        self.terminal.then_some(DrawingCloneWorkTotals { items: self.items, bytes: self.bytes })
    }
}

pub fn drawing_fill_clone_work_totals(value: &FillStyle) -> Result<DrawingCloneWorkTotals, &'static str> {
    let (items, bytes) = match value {
        FillStyle::Solid { .. } => (0, 0),
        FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => DrawingLayerCloneWorkAuthority::vector(stops)?,
    };
    Ok(DrawingCloneWorkTotals { items, bytes })
}

pub fn drawing_stroke_clone_work_totals(value: &StrokeStyle) -> Result<DrawingCloneWorkTotals, &'static str> {
    let mut items = 0usize;
    let mut bytes = 0usize;
    if let Some(dash) = value.dash.as_ref() {
        let (dash_items, dash_bytes) = DrawingLayerCloneWorkAuthority::vector(dash)?;
        items = items.checked_add(dash_items).ok_or("drawing-store.mutation-clone-item-overflow")?;
        bytes = bytes.checked_add(dash_bytes).ok_or("drawing-store.mutation-clone-byte-overflow")?;
    }
    Ok(DrawingCloneWorkTotals { items, bytes })
}

pub fn clone_drawing_string(source: &str) -> Result<String, &'static str> {
    if source.len() > DRAWING_OWNED_FIELD_BYTES {
        return Err("drawing-store.initializer-field-too-large");
    }
    let mut value = String::new();
    value.try_reserve_exact(source.len()).map_err(|_| "drawing-store.initializer-string-admission")?;
    value.push_str(source);
    Ok(value)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq,semio_framework_value::RetireOwned)]
pub struct DrawingLayerAddress {
    pub length: usize,
    pub indices: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
}

impl DrawingLayerAddress {
    pub fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    pub fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

pub struct DrawingLayerLocator {
    pub asset: bool,
    pub root: usize,
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub found: Option<DrawingLayerAddress>,
    pub equality: native_text_footprint::DrawingTextEqualityCursor,
    pub terminal: bool,
}

impl DrawingLayerLocator {
    pub fn new() -> Self {
        Self { asset:false, root: 0, depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], found: None, equality: native_text_footprint::DrawingTextEqualityCursor::default(), terminal: false }
    }
    pub fn asset_reference()->Self{let mut cursor=Self::new();cursor.asset=true;cursor}

    pub fn node_at(snapshot: &DrawingSnapshot, address: DrawingLayerAddress) -> Option<&DrawingLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    pub fn node_at_mut(snapshot: &mut DrawingSnapshot, address: DrawingLayerAddress) -> Option<&mut DrawingLayerNode> {
        fn descend<'a>(value: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let DrawingLayerNode::Group(group) = value else { return None };
            descend(group.children.get_mut(*head)?, tail)
        }
        let value = snapshot.layers.get_mut(address.indices[0])?;
        descend(value, &address.indices[1..address.length])
    }

    pub fn container_mut(snapshot: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Option<&mut DrawingNativeLayers> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                DrawingLayerNode::Group(group) => Some(&mut group.children),
                _ => None,
            },
        }
    }

    pub fn step(&mut self, snapshot: &DrawingSnapshot, target: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.mutation-locator-path")?;
        if self.frames[self.depth].phase == 0 {
            let key=if self.asset{match node{DrawingLayerNode::Image(image)=>Some(&image.image_key),DrawingLayerNode::Trace(trace)=>Some(&trace.source_key),_=>None}}else{Some(semio_s_artifact_draw_drawing_model::layer_id(node))};
            let Some(key)=key else{self.frames[self.depth].phase=1;cx.consume_fuel(1);return Ok(false);};
            let compared = self.equality.step(key, target, 1, DRAWING_OWNED_FIELD_BYTES)?;
            cx.consume_fuel(compared.compared_bytes.max(1) as u64);
            if !compared.complete { return Ok(false); }
            let equal = self.equality.result().ok_or("drawing-store.mutation-locator-equality-incomplete")?;
            self.equality.close_step(1);
            self.equality = native_text_footprint::DrawingTextEqualityCursor::default();
            self.frames[self.depth].phase = 1;
            if equal {
                let mut indices = [0; DRAWING_MAXIMUM_LAYER_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(DrawingLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            cx.consume_fuel(1);
            return Ok(self.terminal);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                    return Err("drawing-store.mutation-locator-depth");
                }
                self.frames[self.depth].child += 1;
                self.path[self.depth] = child;
                self.depth += 1;
                self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);
                return Ok(false);
            }
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = DrawingTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        cx.consume_fuel(1);
        Ok(false)
    }

    pub fn found(&self) -> Option<DrawingLayerAddress> {
        self.found
    }
}

pub fn initial_snapshot_clone_grant(turn: usize) -> RetainedCloneGrant {
    match turn % 3 {
        0 => RetainedCloneGrant::one_capacity_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
        1 => RetainedCloneGrant::one_payload_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
        _ => RetainedCloneGrant::one_release_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
    }
}

//#region 🪢️TaxonomyMounts
#[path = "../🏗️initialization/📚️catalog/🦀️.rs"]
pub mod initialization_catalog_close;
#[path = "../♻️retirement/🦀️.rs"]
pub mod native_retirement;
#[path = "../📐️footprint/🦀️.rs"]
pub mod native_text_footprint;
//#endregion 🪢️TaxonomyMounts
