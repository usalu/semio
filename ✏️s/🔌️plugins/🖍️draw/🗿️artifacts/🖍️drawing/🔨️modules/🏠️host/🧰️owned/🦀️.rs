//! 🧰️ Drawing owned envelope decoder, recursive retirement, and retained store initializer.

/// ♻️ Rebuilt source, removed layer, reverse arena and forward arena.
type DrawingRebuiltLayerOwners = (DrawingNativeLayers, Option<DrawingLayerNode>, Vec<DrawingLayerNode>, Vec<DrawingLayerNode>);

use crate::op::DrawingMutation;
use crate::{DrawingAttributes, DrawingImageAsset, DrawingLayerBase, DrawingLayerNode, DrawingSnapshot, FillStyle, GradientStop, PathSegment, StrokeStyle};
use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneBorrowAuthority, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

#[path = "📐️footprint/🦀️.rs"]
mod native_text_footprint;

#[path = "♻️retirement/🦀️.rs"]
mod native_retirement;
use native_retirement::{DrawingOwnedRetirement, DrawingRetirementOwner};

#[path = "🏗️initialization/📚️catalog/🦀️.rs"]
mod initialization_catalog_close;
use initialization_catalog_close::{close_initialization_catalog, next_initialization_catalog_close_byte_demand};

//#region 🔖️OwnedSprCatalog
const DRAWING_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DrawingSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<DrawingSnapshot> for DrawingSnapshotRetirementFactory {
    fn retire_owned(&self, value: DrawingSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        semio_framework_value::retirement::owned_retirement(value)
    }
}

impl store::SnapshotRetirementFactory<DrawingSnapshot> for DrawingSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<DrawingSnapshot>) -> usize { semio_framework_value::retirement::shared_retirement_birth_bytes::<DrawingSnapshot>() }

    fn retire(&self, snapshot: std::sync::Arc<DrawingSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        semio_framework_value::retirement::shared_lease_retirement(snapshot)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DrawingMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<DrawingMutation> for DrawingMutationRetirementFactory {
    fn retire_owned(&self, value: DrawingMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        semio_framework_value::retirement::owned_retirement(value)
    }
}

fn decode_drawing_snapshot_pack(bytes: &[u8]) -> Result<DrawingSnapshot, ()> {
    <DrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_drawing_mutation_pack(bytes: &[u8]) -> Result<DrawingMutation, ()> {
    DrawingMutation::decode_op(bytes).map_err(|_| ())
}

struct DrawingDecodedFieldRetirement<T: semio_framework_value::retirement::RetireOwned> {
    owner: semio_framework_value::retirement::controlled::ControlledRetirement<T>,
}

impl<T: semio_framework_value::retirement::RetireOwned> DrawingDecodedFieldRetirement<T> {
    fn try_new(value: T) -> Result<Self, (semio_framework_value::ValueError, T)> {
        semio_framework_value::retirement::controlled::ControlledRetirement::new(value).map(|owner| Self { owner })
    }

    fn next_close_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        let grant = self.next_grant()?;
        grant.maximum_copy_bytes.checked_add(grant.maximum_capacity_bytes).and_then(|bytes| bytes.checked_add(grant.maximum_release_bytes)).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Drawing decoded field retirement demand overflow"))
    }

    fn next_grant(&self) -> Result<RetainedCloneGrant, semio_framework_value::ValueError> {
        let copy = self.owner.next_copy_byte_demand();
        let capacity = self.owner.next_capacity_byte_demand(copy)?;
        let release = self.owner.next_release_byte_demand()?;
        Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: usize::MAX })
    }

    fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let grant = self.next_grant()?;
        if grant.maximum_copy_bytes.checked_add(grant.maximum_capacity_bytes).and_then(|bytes| bytes.checked_add(grant.maximum_release_bytes)).is_none_or(|required| required > maximum_bytes) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.owner.step(grant)
    }

    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }

    fn step_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> { self.owner.step(grant) }
}

macro_rules! drawing_owned_field_close_capacity {
    (@demand, $value:ty) => {
        fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
            self.retirement.as_ref().map_or_else(|| Ok(usize::from(self.value.is_some()) * size_of::<$value>()), |retirement| retirement.next_close_byte_demand().map_err(|_| self.diagnostic("drawing-envelope.retirement-demand-fault", 0)))
        }
    };
    (ArtifactEnvelopeSnapshotFieldAuthority, $value:ty) => {
        drawing_owned_field_close_capacity!(@demand, $value);

        fn maximum_close_byte_demand(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority, $value:ty) => {
        drawing_owned_field_close_capacity!(@demand, $value);
    };
}

macro_rules! drawing_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<DrawingDecodedFieldRetirement<$value>>>,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
                store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::OwnedSchemaToken,
                terminal: bool,
                source: &store::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("drawing-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("drawing-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("drawing-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("drawing-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            drawing_owned_field_close_capacity!($authority_trait, $value);

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
                if maximum_items == 0 || maximum_bytes == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                if self.retirement.is_none() {
                    if self.value.is_some() && size_of::<$value>() > maximum_bytes { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
                    if let Some(value) = self.value.take() {
                        match DrawingDecodedFieldRetirement::try_new(value) {
                            Ok(retirement) => *self.retirement = Some(retirement),
                            Err((_, value)) => { *self.value = Some(value);return Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-retirement-unsupported"), 0)); }
                        }
                        self.state = $state::Closing;
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    }
                    self.state = $state::Complete;
                    return Ok(store::SnapshotRetirementStep::Complete);
                }
                let path = self.path;
                let retirement = self.retirement.as_mut().expect("Drawing packed field retirement remains retained");
                match retirement.step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: concat!("drawing-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path })? {
                    RetainedCloneStep::Complete(_) if retirement.terminal_is_empty() => {
                        drop(self.retirement.take());
                        self.state = $state::Complete;
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                    RetainedCloneStep::Complete(_) => Err(self.diagnostic(concat!("drawing-envelope.", $kind, "-retirement-false-terminal"), 0)),
                    RetainedCloneStep::Progress(progress) => Ok(store::SnapshotRetirementStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes }),
                }
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
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for DrawingRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "drawing-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub struct DrawingEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<DrawingSnapshot, DrawingMutation> for DrawingEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<DrawingSnapshot, DrawingMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<DrawingSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(DrawingSnapshotRetirementFactory),
            std::sync::Arc::new(DrawingMutationRetirementFactory),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<DrawingSnapshot, DrawingMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<DrawingSnapshot>> {
        Box::new(DrawingSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<DrawingMutation>> {
        Box::new(DrawingMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(DrawingRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<DrawingMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(DrawingMutationRetirementFactory))
    }
}

pub fn drawing_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<DrawingSnapshot, DrawingMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(DrawingEnvelopeOwnedFieldCatalog), std::sync::Arc::new(DrawingSnapshotRetirementFactory), std::sync::Arc::new(DrawingMutationRetirementFactory))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096;
const DRAWING_MAXIMUM_NESTED_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64;
const DRAWING_MUTATION_RETAINED_PAGE_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16;
const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64;
const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4;
const DRAWING_DUPLICATE_ID_BYTES: usize = 80;

struct DrawingMutationArenaOwner {
    reverse: Vec<DrawingLayerNode>,
    output: Vec<DrawingLayerNode>,
    pages: Vec<String>,
    duplicate_id: String,
}

impl DrawingMutationArenaOwner {
    fn configured_totals() -> Result<(usize, usize), &'static str> {
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

    fn admitted_totals(&self) -> Result<(usize, usize), &'static str> {
        Self::retained_totals(&self.reverse, &self.output, &self.pages, &self.duplicate_id)
    }

    fn retained_totals(reverse: &Vec<DrawingLayerNode>, output: &Vec<DrawingLayerNode>, pages: &Vec<String>, duplicate_id: &String) -> Result<(usize, usize), &'static str> {
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

    fn terminal_is_empty(&self) -> bool {
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
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    duplicate_id: std::mem::ManuallyDrop<Option<String>>,
    rejected_string: std::mem::ManuallyDrop<Option<String>>,
    phase: usize,
    terminal: bool,
}

impl DrawingMutationArenaOwnerBuilder {
    fn new() -> Self {
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

    fn from_owner(owner: DrawingMutationArenaOwner) -> Self {
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

    fn inject(allocation: &mut usize, failure_at: Option<usize>) -> bool {
        let current = *allocation;
        *allocation += 1;
        failure_at == Some(current)
    }

    fn step(&mut self, allocation: &mut usize, failure_at: Option<usize>) -> Result<bool, &'static str> {
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

    fn take(&mut self) -> Option<DrawingMutationArenaOwner> {
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

    fn close_step(&mut self) -> store::SnapshotRetirementStep {
        if let Some(value) = self.rejected_string.take() {
            let released_bytes = value.capacity();
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.duplicate_id.take() {
            let released_bytes = value.capacity();
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(pages) = self.pages.as_mut() {
            if let Some(value) = pages.pop() {
                let released_bytes = value.capacity();
                drop(value);
                return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
            }
        }
        if let Some(value) = self.pages.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<String>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.output.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<DrawingLayerNode>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(value) = self.reverse.take() {
            let released_bytes = value.capacity().saturating_mul(size_of::<DrawingLayerNode>());
            drop(value);
            return store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes };
        }
        self.terminal = true;
        store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.reverse.is_none() && self.output.is_none() && self.pages.is_none() && self.duplicate_id.is_none() && self.rejected_string.is_none()
    }
}

impl Drop for DrawingMutationArenaOwnerBuilder {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation arena owner builder reached Drop before exact construction handoff or retirement");
    }
}

struct DrawingMutationArenaPoolSlot {
    reverse: Option<Vec<DrawingLayerNode>>,
    output: Option<Vec<DrawingLayerNode>>,
    pages: Option<Vec<String>>,
    duplicate_id: Option<String>,
    generation: u64,
    leased: bool,
}

impl DrawingMutationArenaPoolSlot {
    fn new(owner: DrawingMutationArenaOwner) -> Self {
        Self { reverse: Some(owner.reverse), output: Some(owner.output), pages: Some(owner.pages), duplicate_id: Some(owner.duplicate_id), generation: 0, leased: false }
    }

    fn is_available(&self) -> bool {
        !self.leased && self.reverse.is_some() && self.output.is_some() && self.pages.is_some() && self.duplicate_id.is_some()
    }

    fn take(&mut self, generation: u64) -> Option<DrawingMutationArenaOwner> {
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
    slots: [DrawingMutationArenaPoolSlot; DRAWING_MUTATION_ARENA_POOL_CAPACITY],
}

struct DrawingMutationArenaPool {
    state: std::sync::Mutex<DrawingMutationArenaPoolState>,
    admitted_items: usize,
    admitted_bytes: usize,
}

struct DrawingMutationArenaPoolBootstrap {
    owners: std::mem::ManuallyDrop<[Option<DrawingMutationArenaOwner>; DRAWING_MUTATION_ARENA_POOL_CAPACITY]>,
    active: std::mem::ManuallyDrop<Option<DrawingMutationArenaOwnerBuilder>>,
    owner: usize,
    allocation: usize,
    failure_at: Option<usize>,
    failure_after_owner: Option<usize>,
    maximum_items: usize,
    maximum_bytes: usize,
    admitted_items: usize,
    admitted_bytes: usize,
    ready: bool,
    fault: Option<&'static str>,
    terminal: bool,
}

impl std::fmt::Debug for DrawingMutationArenaPoolBootstrap {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DrawingMutationArenaPoolBootstrap").field("owner", &self.owner).field("allocation", &self.allocation).field("fault", &self.fault).finish()
    }
}

impl DrawingMutationArenaPoolBootstrap {
    fn new(failure_at: Option<usize>, failure_after_owner: Option<usize>, maximum_items: usize, maximum_bytes: usize) -> Self {
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

    fn production(admission: DrawingMutationArenaBootstrapAdmission) -> Self {
        Self::new(None, None, admission.maximum_items, admission.maximum_bytes)
    }

    fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        Err(fault)
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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

    fn take_pool(&mut self) -> Option<std::sync::Arc<DrawingMutationArenaPool>> {
        if !self.ready || self.terminal {
            return None;
        }
        let owners = std::mem::replace(&mut *self.owners, std::array::from_fn(|_| None));
        let slots = owners.map(|owner| DrawingMutationArenaPoolSlot::new(owner.expect("validated Drawing arena bootstrap retains every owner")));
        self.terminal = true;
        Some(std::sync::Arc::new(DrawingMutationArenaPool { state: std::sync::Mutex::new(DrawingMutationArenaPoolState { slots }), admitted_items: self.admitted_items, admitted_bytes: self.admitted_bytes }))
    }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> store::SnapshotRetirementStep {
        if cx.should_yield() {
            return store::SnapshotRetirementStep::Blocked;
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step() {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                step => step,
            };
        }
        if self.owner > 0 {
            self.owner -= 1;
            let owner = self.owners[self.owner].take().expect("Drawing bootstrap retirement cursor locates the preceding retained owner");
            *self.active = Some(DrawingMutationArenaOwnerBuilder::from_owner(owner));
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.terminal = true;
        store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
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
    maximum_items: usize,
    maximum_bytes: usize,
}

impl DrawingMutationArenaBootstrapAdmission {
    fn fixed() -> Result<Self, &'static str> {
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
enum DrawingMutationArenaBorrowError {
    NotReady,
    Contended,
    Fault(&'static str),
    Invalid(&'static str),
}

impl DrawingMutationArenaBorrowError {
    fn as_str(self) -> &'static str {
        match self {
            Self::NotReady => "drawing-store.mutation-arena-bootstrap-not-ready",
            Self::Contended => "drawing-store.mutation-arena-pool-contended",
            Self::Fault(fault) | Self::Invalid(fault) => fault,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingMutationArenaBootstrapStep {
    Pending { advanced_items: u64 },
    Blocked,
    Ready,
    Cancelled,
    Fault(&'static str),
}

struct DrawingMutationArenaBootstrapJob {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    admission: DrawingMutationArenaBootstrapAdmission,
    terminal: bool,
}

impl DrawingMutationArenaBootstrapJob {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, &'static str> {
        request_drawing_mutation_arena_pool();
        Ok(Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission::fixed()?, terminal: false })
    }

    fn inactive(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self { operation, generation, admission: DrawingMutationArenaBootstrapAdmission { maximum_items: 0, maximum_bytes: 0 }, terminal: true }
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
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

    fn step_locked(&mut self, state: &mut DrawingMutationArenaProcessState, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep {
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
                if matches!(bootstrap.close_step(cx), store::SnapshotRetirementStep::Complete) && bootstrap.terminal_is_empty() {
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
    fn try_new() -> Result<std::sync::Arc<Self>, DrawingMutationArenaPoolBootstrap> {
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

fn borrow_drawing_mutation_arena_from(pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), &'static str> {
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

fn borrow_drawing_mutation_arena() -> Result<(std::sync::Arc<DrawingMutationArenaPool>, usize, u64, DrawingMutationArenaOwner), DrawingMutationArenaBorrowError> {
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
struct DrawingTraversalFrame {
    phase: u8,
    child: usize,
    string: usize,
}

impl DrawingTraversalFrame {
    const EMPTY: Self = Self { phase: 0, child: 0, string: 0 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingSnapshotOwnerTotals {
    source_items: usize,
    source_bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    maximum_container: usize,
}

#[derive(Clone, Copy)]
struct DrawingOwnerCreditSlot {
    source_items: u32,
    source_bytes: u32,
    derived_items: u32,
    derived_bytes: u32,
}

impl DrawingOwnerCreditSlot {
    const EMPTY: Self = Self { source_items: 0, source_bytes: 0, derived_items: 0, derived_bytes: 0 };
}

struct DrawingFixedOwnerCensus {
    slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS],
    length: usize,
}

impl DrawingFixedOwnerCensus {
    fn new() -> Self {
        Self { slots: [DrawingOwnerCreditSlot::EMPTY; DRAWING_MAXIMUM_NESTED_ITEMS], length: 0 }
    }

    fn admit(&mut self, source_items: usize, source_bytes: usize, derived_items: usize, derived_bytes: usize) -> Result<DrawingOwnerCreditSlot, &'static str> {
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
    index: usize,
    owner: Option<usize>,
}

impl DrawingAssetBoundsCursor {
    fn new() -> Self { Self { index: 0, owner: None } }

    fn next<'a>(&mut self, assets: &'a semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>) -> Result<Option<(&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>, &'a DrawingImageAsset)>, &'static str> {
        let owner = assets as *const _ as usize;
        if self.owner.is_some_and(|expected| expected != owner) { return Err("drawing-store.preflight-asset-owner-changed"); }
        self.owner = Some(owner);
        Ok(assets.entry_at(self.index))
    }

    fn advance(&mut self) -> Result<(), &'static str> {
        self.index = self.index.checked_add(1).ok_or("drawing-store.preflight-asset-index-overflow")?;
        Ok(())
    }
}

struct DrawingSnapshotBoundsAuthority {
    root: usize,
    asset_cursor: DrawingAssetBoundsCursor,
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    items: usize,
    bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
    maximum_container: usize,
    owner_census: DrawingFixedOwnerCensus,
    direct: native_text_footprint::DrawingLayerFootprintCursor,
    record: native_text_footprint::DrawingRecordFootprintCursor,
    root_complete: bool,
    layers_complete: bool,
    terminal: bool,
}

impl DrawingSnapshotBoundsAuthority {
    fn new() -> Self {
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

    fn layer_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut value = root;
        for index in path {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
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

    fn step(&mut self, source: &DrawingSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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

    fn totals(&self) -> Option<DrawingSnapshotOwnerTotals> {
        self.terminal.then_some(DrawingSnapshotOwnerTotals { source_items: self.items, source_bytes: self.bytes, candidate_items: self.candidate_items, candidate_bytes: self.candidate_bytes, maximum_container: self.maximum_container })
    }
}

struct DrawingNativeCloneAuthority<T: RetainedClone> {
    cursor: T::Cursor,
    authority: RetainedCloneBorrowAuthority,
    value: std::mem::ManuallyDrop<Option<T>>,
    retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>>,
    turn: usize,
    phase: u8,
    terminal: bool,
}

impl<T: RetainedClone> DrawingNativeCloneAuthority<T> {
    fn new() -> Self {
        Self {
            cursor: T::retained_clone_cursor(),
            authority: RetainedCloneBorrowAuthority::new(()),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            turn: 0,
            phase: 0,
            terminal: false,
        }
    }

    fn grant(&mut self, bytes: usize) -> RetainedCloneGrant {
        let grant = match self.turn % 3 {
            0 => RetainedCloneGrant::one_capacity_turn(bytes, usize::MAX),
            1 => RetainedCloneGrant::one_payload_turn(bytes, usize::MAX),
            _ => RetainedCloneGrant::one_release_turn(bytes, usize::MAX),
        };
        self.turn = self.turn.wrapping_add(1);
        grant
    }

    fn step(&mut self, source: &T, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal { return Ok(true); }
        let grant = self.grant(DRAWING_OWNED_FIELD_BYTES);
        match self.phase {
            0 => {
                let step = self.cursor.advance(self.authority.borrow(source), grant).map_err(|_| "drawing-store.layer-native-clone")?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = 1; }
            }
            1 => {
                let bytes = size_of::<T>();
                if grant.maximum_copy_bytes < bytes { return Ok(false); }
                *self.value = self.cursor.take();
                if self.value.is_none() { return Err("drawing-store.layer-clone-false-terminal"); }
                self.cursor.begin_close();
                self.phase = 2;
                cx.consume_fuel(bytes.max(1) as u64);
            }
            2 => {
                let step = self.cursor.close_granted(grant).map_err(|_| "drawing-store.layer-native-clone-close")?;
                cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                if matches!(step, RetainedCloneStep::Complete(_)) && self.cursor.terminal_is_empty() { self.terminal = true; }
            }
            _ => return Err("drawing-store.layer-clone-phase"),
        }
        Ok(self.terminal)
    }

    fn take(&mut self) -> Option<T> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        let grant = self.grant(maximum_bytes);
        if !self.cursor.terminal_is_empty() {
            self.cursor.begin_close();
            let step = self.cursor.close_granted(grant)?;
            let progress = step.progress();
            return Ok(store::SnapshotRetirementStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes });
        }
        if let Some(value) = self.value.as_ref() {
            let bytes = size_of::<T>();
            if grant.maximum_copy_bytes < bytes { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            let value = self.value.take().unwrap();
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(value) {
                Ok(retirement) => *self.retirement = Some(retirement),
                Err((error, value)) => { *self.value = Some(value); return Err(error); }
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.step(grant)?;
            let progress = step.progress();
            if matches!(step, RetainedCloneStep::Complete(_)) && retirement.terminal_is_empty() { drop(self.retirement.take()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.cursor.terminal_is_empty() && self.value.is_none() && self.retirement.is_none()
    }
}

impl<T: RetainedClone> Drop for DrawingNativeCloneAuthority<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing layer clone reached Drop before exact handoff or cursor retirement");
    }
}

type DrawingLayerCloneAuthority = DrawingNativeCloneAuthority<DrawingLayerNode>;
type DrawingFillCloneAuthority = DrawingNativeCloneAuthority<FillStyle>;
type DrawingStrokeCloneAuthority = DrawingNativeCloneAuthority<StrokeStyle>;
type DrawingNativeLayers = semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>;
type DrawingNativeText = semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
type DrawingNativeSegments = semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>;
type DrawingTextCloneAuthority = DrawingNativeCloneAuthority<DrawingNativeText>;
type DrawingSegmentsCloneAuthority = DrawingNativeCloneAuthority<DrawingNativeSegments>;

impl DrawingNativeCloneAuthority<DrawingLayerNode> {
    fn target_at_mut<'a>(root: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
        if let Some((head, tail)) = path.split_first() {
            let DrawingLayerNode::Group(group) = root else { return None };
            return Self::target_at_mut(group.children.get_mut(*head)?, tail);
        }
        Some(root)
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingCloneWorkTotals {
    items: usize,
    bytes: usize,
}

struct DrawingLayerCloneWorkAuthority {
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    items: usize,
    bytes: usize,
    direct: native_text_footprint::DrawingLayerFootprintCursor,
    terminal: bool,
}

impl DrawingLayerCloneWorkAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], items: 1, bytes: size_of::<DrawingLayerNode>(), direct: native_text_footprint::DrawingLayerFootprintCursor::default(), terminal: false }
    }

    fn add(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
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

    fn vector<T, const N: usize>(value: &semio_framework_value::list::PagedList<T, N>) -> Result<(usize, usize), &'static str> {
        Ok((1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-clone-item-overflow")?, value.allocated_bytes()))
    }

    fn step(&mut self, root: &DrawingLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
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

    fn totals(&self) -> Option<DrawingCloneWorkTotals> {
        self.terminal.then_some(DrawingCloneWorkTotals { items: self.items, bytes: self.bytes })
    }
}

fn drawing_fill_clone_work_totals(value: &FillStyle) -> Result<DrawingCloneWorkTotals, &'static str> {
    let (items, bytes) = match value {
        FillStyle::Solid { .. } => (0, 0),
        FillStyle::LinearGradient { stops, .. } | FillStyle::RadialGradient { stops, .. } => DrawingLayerCloneWorkAuthority::vector(stops)?,
    };
    Ok(DrawingCloneWorkTotals { items, bytes })
}

fn drawing_stroke_clone_work_totals(value: &StrokeStyle) -> Result<DrawingCloneWorkTotals, &'static str> {
    let mut items = 0usize;
    let mut bytes = 0usize;
    if let Some(dash) = value.dash.as_ref() {
        let (dash_items, dash_bytes) = DrawingLayerCloneWorkAuthority::vector(dash)?;
        items = items.checked_add(dash_items).ok_or("drawing-store.mutation-clone-item-overflow")?;
        bytes = bytes.checked_add(dash_bytes).ok_or("drawing-store.mutation-clone-byte-overflow")?;
    }
    Ok(DrawingCloneWorkTotals { items, bytes })
}

fn clone_drawing_string(source: &str) -> Result<String, &'static str> {
    if source.len() > DRAWING_OWNED_FIELD_BYTES {
        return Err("drawing-store.initializer-field-too-large");
    }
    let mut value = String::new();
    value.try_reserve_exact(source.len()).map_err(|_| "drawing-store.initializer-string-admission")?;
    value.push_str(source);
    Ok(value)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingLayerAddress {
    length: usize,
    indices: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
}

impl DrawingLayerAddress {
    fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

struct DrawingLayerLocator {
    root: usize,
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    found: Option<DrawingLayerAddress>,
    equality: native_text_footprint::DrawingTextEqualityCursor,
    terminal: bool,
}

impl DrawingLayerLocator {
    fn new() -> Self {
        Self { root: 0, depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], found: None, equality: native_text_footprint::DrawingTextEqualityCursor::default(), terminal: false }
    }

    fn node_at(snapshot: &DrawingSnapshot, address: DrawingLayerAddress) -> Option<&DrawingLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    fn node_at_mut(snapshot: &mut DrawingSnapshot, address: DrawingLayerAddress) -> Option<&mut DrawingLayerNode> {
        fn descend<'a>(value: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let DrawingLayerNode::Group(group) = value else { return None };
            descend(group.children.get_mut(*head)?, tail)
        }
        let value = snapshot.layers.get_mut(address.indices[0])?;
        descend(value, &address.indices[1..address.length])
    }

    fn container_mut(snapshot: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Option<&mut DrawingNativeLayers> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                DrawingLayerNode::Group(group) => Some(&mut group.children),
                _ => None,
            },
        }
    }

    fn step(&mut self, snapshot: &DrawingSnapshot, target: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.mutation-locator-path")?;
        if self.frames[self.depth].phase == 0 {
            let compared = self.equality.step(crate::schema::layer_id(node), target, 1, DRAWING_OWNED_FIELD_BYTES)?;
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

    fn found(&self) -> Option<DrawingLayerAddress> {
        self.found
    }
}

const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY: usize = DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY * 4 + 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingContainerRebuildMove {
    Empty,
    SourceToReverse,
    PendingToOutput,
    ReverseToOutput,
    ReverseToRemoved,
    OutputToReverse,
    ReverseToSource,
}

struct DrawingContainerRebuildAuthority {
    source: std::mem::ManuallyDrop<Option<DrawingNativeLayers>>,
    reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    pending: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    removed: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    moves: [DrawingContainerRebuildMove; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
    move_count: usize,
    rollback_cursor: Option<usize>,
    remove_index: Option<usize>,
    insert_index: Option<usize>,
    original_index: usize,
    phase: u8,
    terminal: bool,
}

struct DrawingContainerRebuildRejected {
    source: DrawingNativeLayers,
    pending: Option<DrawingLayerNode>,
    reverse: Vec<DrawingLayerNode>,
    output: Vec<DrawingLayerNode>,
}

impl DrawingContainerRebuildAuthority {
    fn new(
        source: DrawingNativeLayers,
        remove_index: Option<usize>,
        insert_index: Option<usize>,
        pending: Option<DrawingLayerNode>,
        reverse: Vec<DrawingLayerNode>,
        output: Vec<DrawingLayerNode>,
        workset: DrawingMutationWorksetPlan,
    ) -> Result<Self, DrawingContainerRebuildRejected> {
        let extra = usize::from(pending.is_some());
        let Some(output_capacity) = source.len().saturating_sub(usize::from(remove_index.is_some())).checked_add(extra) else {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        };
        if output_capacity > DRAWING_MAXIMUM_NESTED_ITEMS
            || source.len().saturating_add(output_capacity) > workset.container_slots
            || source.len() > workset.maximum_container.saturating_add(1)
            || output_capacity > workset.maximum_container.saturating_add(1)
            || source.capacity() < output_capacity
            || reverse.capacity() < source.len().max(output_capacity)
            || output.capacity() < output_capacity
            || !reverse.is_empty()
            || !output.is_empty()
        {
            return Err(DrawingContainerRebuildRejected { source, pending, reverse, output });
        }
        Ok(Self {
            source: std::mem::ManuallyDrop::new(Some(source)),
            reverse: std::mem::ManuallyDrop::new(Some(reverse)),
            output: std::mem::ManuallyDrop::new(Some(output)),
            pending: std::mem::ManuallyDrop::new(pending),
            removed: std::mem::ManuallyDrop::new(None),
            moves: [DrawingContainerRebuildMove::Empty; DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY],
            move_count: 0,
            rollback_cursor: None,
            remove_index,
            insert_index,
            original_index: 0,
            phase: 0,
            terminal: false,
        })
    }

    fn reserve_move(&self) -> Result<(), &'static str> {
        self.moves.get(self.move_count).map(|_| ()).ok_or("drawing-store.container-move-capacity")
    }

    fn record_reserved_move(&mut self, value: DrawingContainerRebuildMove) {
        self.moves[self.move_count] = value;
        self.move_count += 1;
    }

    fn advance(&mut self) -> Result<(bool, u64), &'static str> {
        if self.terminal {
            return Ok((true, 0));
        }
        if self.rollback_cursor.is_some() {
            return Err("drawing-store.container-advance-after-rollback");
        }
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-owner-missing");
        }
        if self.phase == 0 {
            if !self.source.as_ref().expect("validated Drawing source remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.source.as_mut().expect("validated Drawing source remains retained").pop().expect("nonempty Drawing source yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::SourceToReverse);
                return Ok((false, 1));
            }
            self.phase = 1;
            return Ok((false, 0));
        }
        if self.phase == 1 {
            if self.pending.is_some() && self.insert_index.is_some_and(|index| index.min(self.reverse.as_ref().map_or(0, Vec::len) + self.original_index) == self.output.as_ref().map_or(0, Vec::len)) {
                self.reserve_move()?;
                self.output.as_mut().expect("validated Drawing output remains retained").push(self.pending.take().expect("Drawing insertion owner remains retained"));
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
                if self.remove_index == Some(self.original_index) && self.removed.is_some() {
                    return Err("drawing-store.container-duplicate-removal");
                }
                self.reserve_move()?;
                let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
                if self.remove_index == Some(self.original_index) {
                    *self.removed = Some(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToRemoved);
                } else {
                    self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                    self.record_reserved_move(DrawingContainerRebuildMove::ReverseToOutput);
                }
                self.original_index += 1;
                return Ok((false, 1));
            }
            if self.pending.is_some() {
                self.reserve_move()?;
                let value = self.pending.take().expect("validated Drawing pending owner remains retained");
                self.output.as_mut().expect("validated Drawing output remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::PendingToOutput);
                return Ok((false, 1));
            }
            self.phase = 2;
            return Ok((false, 0));
        }
        if self.phase == 2 {
            if !self.output.as_ref().expect("validated Drawing output remains retained").is_empty() {
                self.reserve_move()?;
                let value = self.output.as_mut().expect("validated Drawing output remains retained").pop().expect("nonempty Drawing output yields one owner");
                self.reverse.as_mut().expect("validated Drawing reverse remains retained").push(value);
                self.record_reserved_move(DrawingContainerRebuildMove::OutputToReverse);
                return Ok((false, 1));
            }
            self.phase = 3;
            return Ok((false, 0));
        }
        if !self.reverse.as_ref().expect("validated Drawing reverse remains retained").is_empty() {
            self.reserve_move()?;
            let value = self.reverse.as_mut().expect("validated Drawing reverse remains retained").pop().expect("nonempty Drawing reverse yields one owner");
            self.source.as_mut().expect("validated Drawing source remains retained").push_reserved(value).unwrap_or_else(|_| panic!("admitted native Drawing source slot remains reserved"));
            self.record_reserved_move(DrawingContainerRebuildMove::ReverseToSource);
            return Ok((false, 1));
        }
        self.terminal = true;
        Ok((true, 0))
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let (complete, fuel) = self.advance()?;
        if fuel > 0 {
            cx.consume_fuel(fuel);
        }
        Ok(complete)
    }

    fn close_forward_step(&mut self) -> Result<bool, &'static str> {
        self.advance().map(|(complete, _)| complete)
    }

    fn take(&mut self) -> Option<DrawingRebuiltLayerOwners> {
        self.terminal.then(|| {
            (
                self.source.take().expect("Drawing rebuilt source container remains retained"),
                self.removed.take(),
                self.reverse.take().expect("Drawing emptied reverse arena remains retained"),
                self.output.take().expect("Drawing emptied output arena remains retained"),
            )
        })
    }

    fn rollback_step(&mut self) -> Result<bool, &'static str> {
        if self.source.is_none() || self.reverse.is_none() || self.output.is_none() {
            return Err("drawing-store.container-rollback-owner-missing");
        }
        let cursor = *self.rollback_cursor.get_or_insert(self.move_count);
        if cursor == 0 {
            self.phase = 0;
            self.original_index = 0;
            return Ok(true);
        }
        let index = cursor - 1;
        match self.moves[index] {
            DrawingContainerRebuildMove::SourceToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.source.as_mut().ok_or("drawing-store.container-source")?.push_reserved(value).unwrap_or_else(|_| panic!("native Drawing rollback source slot remains reserved"));
            }
            DrawingContainerRebuildMove::PendingToOutput => {
                if self.pending.is_some() {
                    return Err("drawing-store.container-rollback-pending");
                }
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                *self.pending = Some(value);
            }
            DrawingContainerRebuildMove::ReverseToOutput => {
                let value = self.output.as_mut().ok_or("drawing-store.container-output")?.pop().ok_or("drawing-store.container-rollback-output")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToRemoved => {
                let value = self.removed.take().ok_or("drawing-store.container-rollback-removed")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::OutputToReverse => {
                let value = self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.pop().ok_or("drawing-store.container-rollback-reverse")?;
                self.output.as_mut().ok_or("drawing-store.container-output")?.push(value);
            }
            DrawingContainerRebuildMove::ReverseToSource => {
                let value = self.source.as_mut().ok_or("drawing-store.container-source")?.pop().ok_or("drawing-store.container-rollback-source")?;
                self.reverse.as_mut().ok_or("drawing-store.container-reverse")?.push(value);
            }
            DrawingContainerRebuildMove::Empty => return Err("drawing-store.container-rollback-empty-move"),
        }
        self.moves[index] = DrawingContainerRebuildMove::Empty;
        self.rollback_cursor = Some(index);
        Ok(false)
    }

    fn rollback_complete(&self) -> bool {
        self.rollback_cursor == Some(0)
    }

    #[cfg(test)]
    fn recorded_move_in_phase(&self, phase: u8) -> bool {
        let Some(index) = self.move_count.checked_sub(1) else { return false };
        self.phase == phase
            && match phase {
                0 => self.moves[index] == DrawingContainerRebuildMove::SourceToReverse,
                1 => matches!(self.moves[index], DrawingContainerRebuildMove::PendingToOutput | DrawingContainerRebuildMove::ReverseToOutput | DrawingContainerRebuildMove::ReverseToRemoved),
                2 => self.moves[index] == DrawingContainerRebuildMove::OutputToReverse,
                3 => self.moves[index] == DrawingContainerRebuildMove::ReverseToSource,
                _ => false,
            }
    }

    fn finish_handoff(&mut self) -> Result<(), &'static str> {
        if !(self.terminal || self.rollback_complete()) || self.source.is_some() || self.reverse.is_some() || self.output.is_some() || self.pending.is_some() || self.removed.is_some() {
            return Err("drawing-store.container-rollback-handoff-incomplete");
        }
        self.terminal = true;
        Ok(())
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.source.is_none() && self.reverse.is_none() && self.output.is_none() && self.pending.is_none() && self.removed.is_none()
    }
}

impl Drop for DrawingContainerRebuildAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing container rebuild reached Drop before exact handoff or cursor retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingSemanticDigestTotals {
    semantic_items: usize,
    semantic_bytes: usize,
    maximum_field_bytes: usize,
    source_owner_items: usize,
    source_owner_bytes: usize,
    derived_owner_items: usize,
    derived_owner_bytes: usize,
}

struct DrawingSemanticDigestCredit {
    items: usize,
    bytes: usize,
    maximum_field_bytes: usize,
    source_owner_items: usize,
    source_owner_bytes: usize,
    derived_owner_items: usize,
    derived_owner_bytes: usize,
    owner_census: DrawingFixedOwnerCensus,
    semantic: Option<semio_framework_hash::Sha256>,
    text: Option<native_text_footprint::DrawingTextDigestCursor>,
}

impl Default for DrawingSemanticDigestCredit {
    fn default() -> Self {
        Self {
            items: 0,
            bytes: 0,
            maximum_field_bytes: 0,
            source_owner_items: 1,
            source_owner_bytes: size_of::<DrawingMutation>(),
            derived_owner_items: 0,
            derived_owner_bytes: 0,
            owner_census: DrawingFixedOwnerCensus::new(),
            semantic: Some(semio_framework_hash::Sha256::new()),
            text: None,
        }
    }
}

impl DrawingSemanticDigestCredit {
    fn add_source_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(items, bytes, 0, 0)?;
        self.source_owner_items = self.source_owner_items.checked_add(items).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        self.source_owner_bytes = self.source_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        if self.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if self.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        Ok(())
    }

    fn add_derived_owner(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.owner_census.admit(0, 0, items, bytes)?;
        self.derived_owner_items = self.derived_owner_items.checked_add(items).ok_or("drawing-store.mutation-derived-owner-item-overflow")?;
        self.derived_owner_bytes = self.derived_owner_bytes.checked_add(bytes).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        if self.derived_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-derived-owner-item-capacity");
        }
        if self.derived_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-derived-owner-byte-capacity");
        }
        Ok(())
    }

    fn source_vec<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        let items = 1usize.checked_add(value.capacity()).ok_or("drawing-store.mutation-source-owner-item-overflow")?;
        let bytes = value.allocated_bytes().checked_add(size_of_val(value)).ok_or("drawing-store.mutation-source-owner-byte-overflow")?;
        self.add_source_owner(items, bytes)
    }

    fn derived_vec<T, const N: usize>(&mut self, value: &semio_framework_value::list::PagedList<T, N>) -> Result<(), &'static str> {
        let bytes = value.len().checked_mul(size_of::<T>()).ok_or("drawing-store.mutation-derived-owner-byte-overflow")?;
        let pages = bytes.checked_add(DRAWING_MUTATION_RETAINED_PAGE_BYTES - 1).ok_or("drawing-store.mutation-derived-owner-byte-overflow")? / DRAWING_MUTATION_RETAINED_PAGE_BYTES;
        self.add_derived_owner(pages.max(1), 0)
    }

    fn observe_owned_string(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &DrawingNativeText, cloned: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let cursor = self.text.get_or_insert_with(native_text_footprint::DrawingTextDigestCursor::default);
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        let step = cursor.step(value, tag, 1, DRAWING_OWNED_FIELD_BYTES, |bytes| { semantic.update(bytes); digest.observe(bytes); })?;
        cx.consume_fuel(1);
        if !step.complete { return Ok(false); }
        let totals = cursor.totals().ok_or("drawing-store.text-digest-false-terminal")?;
        cursor.close_step(1);
        self.text = None;
        self.add_source_owner(1usize.checked_add(totals.chunks).ok_or("drawing-store.mutation-source-owner-item-overflow")?, size_of_val(value).checked_add(totals.backing_bytes).ok_or("drawing-store.mutation-source-owner-byte-overflow")?)?;
        if cloned { self.add_derived_owner(totals.chunks.max(1), 0)?; }
        self.maximum_field_bytes = self.maximum_field_bytes.max(value.len());
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        Ok(true)
    }

    fn observe(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        if value.len() > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        self.maximum_field_bytes = self.maximum_field_bytes.max(value.len());
        self.items = self.items.checked_add(1).ok_or("drawing-store.mutation-item-overflow")?;
        self.bytes = self.bytes.checked_add(11).and_then(|bytes| bytes.checked_add(value.len())).ok_or("drawing-store.mutation-byte-overflow")?;
        if self.items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-item-capacity");
        }
        if self.bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-byte-capacity");
        }
        let prefix = [0xd8];
        let tag = tag.to_be_bytes();
        let length = (value.len() as u64).to_be_bytes();
        let semantic = self.semantic.as_mut().ok_or("drawing-store.mutation-digest-sealed")?;
        semantic.update(&prefix);
        semantic.update(&tag);
        semantic.update(&length);
        semantic.update(value);
        digest.observe(&prefix);
        digest.observe(&tag);
        digest.observe(&length);
        digest.observe(value);
        cx.consume_fuel(1);
        Ok(())
    }

    fn scalar_f64(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: f64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &value.to_bits().to_be_bytes(), cx)
    }

    fn scalar_usize(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, tag: u16, value: usize, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, tag, &(value as u64).to_be_bytes(), cx)
    }

    fn seal(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        self.observe(digest, 99, &[], cx)?;
        let semantic = self.semantic.take().ok_or("drawing-store.mutation-digest-sealed")?.finalize();
        digest.observe(b"drawing.semantic.sha256");
        digest.observe(&semantic);
        Ok(())
    }

    fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.semantic.is_none().then_some(DrawingSemanticDigestTotals {
            semantic_items: self.items,
            semantic_bytes: self.bytes,
            maximum_field_bytes: self.maximum_field_bytes,
            source_owner_items: self.source_owner_items,
            source_owner_bytes: self.source_owner_bytes,
            derived_owner_items: self.derived_owner_items,
            derived_owner_bytes: self.derived_owner_bytes,
        })
    }
}

struct DrawingFillDigestAuthority {
    phase: u8,
    index: usize,
    field: u8,
    terminal: bool,
}

impl DrawingFillDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, index: 0, field: 0, terminal: false }
    }

    fn step(&mut self, value: Option<&FillStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 200, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-fill-missing")?;
        if self.phase == 1 {
            let variant = match value {
                FillStyle::Solid { .. } => 1,
                FillStyle::LinearGradient { .. } => 2,
                FillStyle::RadialGradient { .. } => 3,
            };
            credit.observe(digest, 201, &[variant], cx)?;
            self.phase = 2;
            return Ok(false);
        }
        match value {
            FillStyle::Solid { color } => {
                let Some(value) = color.get((self.phase - 2) as usize) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 202 + u16::from(self.phase - 2), *value, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 6;
            }
            FillStyle::LinearGradient { x1, y1, x2, y2, stops } => {
                if self.phase <= 5 {
                    let fields = [*x1, *y1, *x2, *y2];
                    credit.scalar_f64(digest, 210 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 6 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 214, stops.len(), cx)?;
                    self.phase = 7;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-linear-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 215, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 215 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
            FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops } => {
                if self.phase <= 4 {
                    let fields = [*center_x, *center_y, *r];
                    credit.scalar_f64(digest, 220 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                } else if self.phase == 5 {
                    credit.source_vec(stops)?;
                    credit.derived_vec(stops)?;
                    credit.scalar_usize(digest, 223, stops.len(), cx)?;
                    self.phase = 6;
                    self.terminal = stops.is_empty();
                } else {
                    let stop = stops.get(self.index).ok_or("drawing-store.digest-radial-stop")?;
                    if self.field == 0 {
                        credit.scalar_f64(digest, 224, stop.offset, cx)?;
                    } else {
                        credit.scalar_f64(digest, 224 + u16::from(self.field), stop.color[(self.field - 1) as usize], cx)?;
                    }
                    self.field += 1;
                    if self.field == 5 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == stops.len();
                    }
                }
            }
        }
        Ok(self.terminal)
    }
}

struct DrawingStrokeDigestAuthority {
    phase: u8,
    index: usize,
    terminal: bool,
}

impl DrawingStrokeDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, index: 0, terminal: false }
    }

    fn step(&mut self, value: Option<&StrokeStyle>, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            credit.observe(digest, 240, &[u8::from(value.is_some())], cx)?;
            self.phase = 1;
            if value.is_none() {
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        let value = value.ok_or("drawing-store.digest-stroke-missing")?;
        match self.phase {
            1..=4 => credit.scalar_f64(digest, 240 + u16::from(self.phase), value.color[(self.phase - 1) as usize], cx)?,
            5 => credit.scalar_f64(digest, 245, value.width, cx)?,
            6 => credit.observe(digest, 246, value.cap.as_str().as_bytes(), cx)?,
            7 => credit.observe(digest, 247, value.join.as_str().as_bytes(), cx)?,
            8 => credit.observe(digest, 248, &[u8::from(value.dash.is_some())], cx)?,
            9 => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                credit.source_vec(dash)?;
                credit.derived_vec(dash)?;
                credit.scalar_usize(digest, 249, dash.len(), cx)?;
                self.terminal = dash.is_empty();
            }
            _ => {
                let dash = value.dash.as_ref().ok_or("drawing-store.digest-dash-missing")?;
                let item = *dash.get(self.index).ok_or("drawing-store.digest-dash-index")?;
                credit.scalar_f64(digest, 250, item, cx)?;
                self.index += 1;
                self.terminal = self.index == dash.len();
            }
        }
        if !self.terminal {
            self.phase = match self.phase {
                8 if value.dash.is_none() => {
                    self.terminal = true;
                    8
                }
                phase => phase + 1,
            };
        }
        Ok(self.terminal)
    }
}

struct DrawingPathSegmentDigestAuthority {
    phase: u8,
    terminal: bool,
}

impl DrawingPathSegmentDigestAuthority {
    fn new() -> Self {
        Self { phase: 0, terminal: false }
    }

    fn step(&mut self, value: &PathSegment, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            let variant = match value {
                PathSegment::Move { .. } => 1,
                PathSegment::Line { .. } => 2,
                PathSegment::Quad { .. } => 3,
                PathSegment::Cubic { .. } => 4,
                PathSegment::Arc { .. } => 5,
                PathSegment::Close => 6,
            };
            credit.observe(digest, 300, &[variant], cx)?;
            self.phase = 1;
            self.terminal = matches!(value, PathSegment::Close);
            return Ok(self.terminal);
        }
        let index = (self.phase - 1) as usize;
        match value {
            PathSegment::Move { to } | PathSegment::Line { to } => {
                let Some(field) = to.get(index) else {
                    self.terminal = true;
                    return Ok(true);
                };
                credit.scalar_f64(digest, 301 + index as u16, *field, cx)?;
                self.phase += 1;
                self.terminal = self.phase == 3;
            }
            PathSegment::Quad { ctrl, to } => {
                let fields = [ctrl[0], ctrl[1], to[0], to[1]];
                credit.scalar_f64(digest, 303 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                let fields = [ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]];
                credit.scalar_f64(digest, 307 + index as u16, fields[index], cx)?;
                self.phase += 1;
                self.terminal = self.phase == 7;
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                if index < 3 {
                    credit.scalar_f64(digest, 313 + index as u16, [*rx, *ry, *rotation][index], cx)?;
                } else if index < 5 {
                    credit.observe(digest, 313 + index as u16, &[u8::from([*large_arc, *sweep][index - 3])], cx)?;
                } else {
                    credit.scalar_f64(digest, 313 + index as u16, to[index - 5], cx)?;
                }
                self.phase += 1;
                self.terminal = self.phase == 8;
            }
            PathSegment::Close => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

#[derive(Clone, Copy)]
struct DrawingOptionalDigestField {
    tag: u16,
    present: bool,
    next: u8,
    absent: u8,
}

struct DrawingLayerVariantDigestAuthority {
    phase: u8,
    index: usize,
    field: u8,
    segment: Option<DrawingPathSegmentDigestAuthority>,
    terminal: bool,
}

impl DrawingLayerVariantDigestAuthority {
    fn new() -> Self {
        Self { phase: 1, index: 0, field: 0, segment: None, terminal: false }
    }

    fn option(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, field: DrawingOptionalDigestField, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), &'static str> {
        let DrawingOptionalDigestField { tag, present, next, absent } = field;
        credit.observe(digest, tag, &[u8::from(present)], cx)?;
        self.phase = if present { next } else { absent };
        Ok(())
    }

    fn step(&mut self, layer: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match layer {
            DrawingLayerNode::Shape(value) => match self.phase {
                1 => {
                    if !credit.observe_owned_string(digest, 340, &value.shape_kind, true, cx)? { return Ok(false); }
                    self.phase = 2;
                }
                2 => self.option(digest, credit, DrawingOptionalDigestField { tag: 341, present: value.rect.is_some(), next: 3, absent: 7 }, cx)?,
                3..=6 => {
                    let rect = value.rect.as_ref().ok_or("drawing-store.digest-rect-missing")?;
                    credit.scalar_f64(digest, 342 + u16::from(self.phase - 3), [rect.x, rect.y, rect.width, rect.height][(self.phase - 3) as usize], cx)?;
                    self.phase += 1;
                }
                7 => self.option(digest, credit, DrawingOptionalDigestField { tag: 346, present: value.ellipse.is_some(), next: 8, absent: 12 }, cx)?,
                8..=11 => {
                    let ellipse = value.ellipse.as_ref().ok_or("drawing-store.digest-ellipse-missing")?;
                    credit.scalar_f64(digest, 347 + u16::from(self.phase - 8), [ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry][(self.phase - 8) as usize], cx)?;
                    self.phase += 1;
                }
                12 => self.option(digest, credit, DrawingOptionalDigestField { tag: 351, present: value.circle.is_some(), next: 13, absent: 16 }, cx)?,
                13..=15 => {
                    let circle = value.circle.as_ref().ok_or("drawing-store.digest-circle-missing")?;
                    credit.scalar_f64(digest, 352 + u16::from(self.phase - 13), [circle.cx, circle.cy, circle.r][(self.phase - 13) as usize], cx)?;
                    self.phase += 1;
                }
                16 => self.option(digest, credit, DrawingOptionalDigestField { tag: 355, present: value.line.is_some(), next: 17, absent: 21 }, cx)?,
                17..=20 => {
                    let line = value.line.as_ref().ok_or("drawing-store.digest-line-missing")?;
                    credit.scalar_f64(digest, 356 + u16::from(self.phase - 17), [line.x1, line.y1, line.x2, line.y2][(self.phase - 17) as usize], cx)?;
                    self.phase += 1;
                }
                21 => self.option(digest, credit, DrawingOptionalDigestField { tag: 360, present: value.polygon.is_some(), next: 22, absent: 24 }, cx)?,
                22 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    credit.source_vec(points)?;
                    credit.derived_vec(points)?;
                    credit.scalar_usize(digest, 361, points.len(), cx)?;
                    self.phase = 23;
                    self.terminal = points.is_empty();
                }
                23 => {
                    let points = &value.polygon.as_ref().ok_or("drawing-store.digest-polygon-missing")?.points;
                    let point = points.get(self.index).ok_or("drawing-store.digest-point-index")?;
                    credit.scalar_f64(digest, 362 + u16::from(self.field), point[self.field as usize], cx)?;
                    self.field += 1;
                    if self.field == 2 {
                        self.field = 0;
                        self.index += 1;
                        self.terminal = self.index == points.len();
                    }
                }
                _ => self.terminal = true,
            },
            DrawingLayerNode::Path(value) => match self.phase {
                1 => {
                    credit.source_vec(&value.segments)?;
                    credit.derived_vec(&value.segments)?;
                    credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 2;
                    self.terminal = value.segments.is_empty();
                }
                _ => {
                    let segment = value.segments.get(self.index).ok_or("drawing-store.digest-segment-index")?;
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, credit, cx)? {
                        self.segment = None;
                        self.index += 1;
                        self.terminal = self.index == value.segments.len();
                    }
                }
            },
            DrawingLayerNode::Text(value) => {
                match self.phase {
                    1 => credit.scalar_f64(digest, 380, value.x, cx)?,
                    2 => credit.scalar_f64(digest, 381, value.y, cx)?,
                    3 => { if !credit.observe_owned_string(digest, 382, &value.content, true, cx)? { return Ok(false); } },
                    4 => credit.scalar_f64(digest, 383, value.size, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 5;
            }
            DrawingLayerNode::Image(value) => {
                match self.phase {
                    1 => { if !credit.observe_owned_string(digest, 390, &value.image_key, true, cx)? { return Ok(false); } },
                    2 => credit.scalar_f64(digest, 391, value.width, cx)?,
                    3 => credit.scalar_f64(digest, 392, value.height, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
            DrawingLayerNode::Group(value) => {
                if self.phase==1 {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest,400,value.children.len(),cx)?;
                    self.phase=2;
                } else {credit.observe(digest,401,&[u8::from(value.isolation)],cx)?;self.terminal=true;}
            }
            DrawingLayerNode::Boolean(value) => match self.phase {
                1 => {
                    if !credit.observe_owned_string(digest, 410, &value.operation, true, cx)? { return Ok(false); }
                    self.phase = 2;
                }
                2 => {
                    credit.source_vec(&value.children)?;
                    credit.derived_vec(&value.children)?;
                    credit.scalar_usize(digest, 411, value.children.len(), cx)?;
                    self.phase = 3;
                    self.terminal = value.children.is_empty();
                }
                _ => {
                    let item = value.children.get(self.index).ok_or("drawing-store.digest-boolean-child")?;
                    if !credit.observe_owned_string(digest, 412, item, true, cx)? { return Ok(false); }
                    self.index += 1;
                    self.terminal = self.index == value.children.len();
                }
            },
            DrawingLayerNode::Trace(value) => {
                match self.phase {
                    1 => { if !credit.observe_owned_string(digest, 420, &value.source_key, true, cx)? { return Ok(false); } },
                    2 => credit.scalar_f64(digest, 421, value.params.threshold, cx)?,
                    3 => credit.scalar_f64(digest, 422, value.params.simplify_epsilon, cx)?,
                    _ => {
                        self.terminal = true;
                        return Ok(true);
                    }
                }
                self.phase += 1;
                self.terminal = self.phase == 4;
            }
        }
        Ok(self.terminal)
    }
}

struct DrawingLayerDigestAuthority {
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    fill: Option<DrawingFillDigestAuthority>,
    stroke: Option<DrawingStrokeDigestAuthority>,
    variant: Option<DrawingLayerVariantDigestAuthority>,
    terminal: bool,
}

impl DrawingLayerDigestAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], fill: None, stroke: None, variant: None, terminal: false }
    }

    fn step(&mut self, root: &DrawingLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, credit: &mut DrawingSemanticDigestCredit, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.digest-layer-path")?;
        let base = crate::schema::layer_base(node);
        let phase = self.frames[self.depth].phase;
        match phase {
            0 => {
                credit.add_source_owner(1, size_of::<DrawingLayerNode>())?;
                credit.add_derived_owner(1, size_of::<DrawingLayerNode>())?;
                let variant = match node {
                    DrawingLayerNode::Shape(_) => 1,
                    DrawingLayerNode::Path(_) => 2,
                    DrawingLayerNode::Text(_) => 3,
                    DrawingLayerNode::Image(_) => 4,
                    DrawingLayerNode::Group(_) => 5,
                    DrawingLayerNode::Boolean(_) => 6,
                    DrawingLayerNode::Trace(_) => 7,
                };
                credit.observe(digest, 100, &[variant], cx)?;
                self.frames[self.depth].phase = 1;
            }
            1 => {
                if !credit.observe_owned_string(digest, 101, &base.id, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 2;
            }
            2 => {
                if !credit.observe_owned_string(digest, 102, &base.name, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 3;
            }
            3 => {
                credit.observe(digest, 103, &[u8::from(base.visible)], cx)?;
                self.frames[self.depth].phase = 4;
            }
            4 => {
                credit.observe(digest, 104, &[u8::from(base.locked)], cx)?;
                self.frames[self.depth].phase = 5;
            }
            5 => {
                credit.scalar_f64(digest, 105, base.opacity, cx)?;
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if !credit.observe_owned_string(digest, 106, &base.blend_mode, true, cx)? { return Ok(false); }
                self.frames[self.depth].phase = 7;
            }
            7..=12 => {
                let fields = [base.transform.x, base.transform.y, base.transform.scale_x, base.transform.scale_y, base.transform.rotation, base.transform.shear];
                credit.scalar_f64(digest, 107 + u16::from(phase - 7), fields[(phase - 7) as usize], cx)?;
                self.frames[self.depth].phase += 1;
            }
            13 => {
                let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                if fill.step(base.attributes.fill.as_ref(), digest, credit, cx)? {
                    self.fill = None;
                    self.frames[self.depth].phase = 14;
                }
            }
            14 => {
                let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                if stroke.step(base.attributes.stroke.as_ref(), digest, credit, cx)? {
                    self.stroke = None;
                    self.frames[self.depth].phase = 15;
                }
            }
            15 => {
                credit.observe(digest, 113, &[u8::from(base.attributes.fill_rule == crate::FillRule::Nonzero)], cx)?;
                self.frames[self.depth].phase = 16;
            }
            16 => {
                let variant = self.variant.get_or_insert_with(DrawingLayerVariantDigestAuthority::new);
                if variant.step(node, digest, credit, cx)? {
                    self.variant = None;
                    self.frames[self.depth].phase = 17;
                }
            }
            17 => {
                if let DrawingLayerNode::Group(group) = node {
                    let child = self.frames[self.depth].child;
                    if child < group.children.len() {
                        if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                            return Err("drawing-store.digest-layer-depth");
                        }
                        self.frames[self.depth].child += 1;
                        self.path[self.depth] = child;
                        self.depth += 1;
                        self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 18;
                return Ok(false);
            }
            _ => {
                credit.observe(digest, 199, &[], cx)?;
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    fn close(&mut self) {
        self.fill = None;
        self.stroke = None;
        self.variant = None;
        self.terminal = true;
    }
}

struct DrawingMutationDigestAuthority {
    layer: Option<DrawingLayerDigestAuthority>,
    fill: Option<DrawingFillDigestAuthority>,
    stroke: Option<DrawingStrokeDigestAuthority>,
    segment: Option<DrawingPathSegmentDigestAuthority>,
    segment_index: usize,
    credit: DrawingSemanticDigestCredit,
    phase: u8,
    terminal: bool,
}

impl DrawingMutationDigestAuthority {
    fn new() -> Self {
        Self { layer: None, fill: None, stroke: None, segment: None, segment_index: 0, credit: DrawingSemanticDigestCredit::default(), phase: 0, terminal: false }
    }

    fn variant(mutation: &DrawingMutation) -> u8 {
        match mutation {
            DrawingMutation::SetLayerVisible(_) => 1,
            DrawingMutation::SetLayerLocked(_) => 2,
            DrawingMutation::SetLayerOpacity(_) => 3,
            DrawingMutation::SetLayerBlendMode(_) => 4,
            DrawingMutation::RenameLayer(_) => 5,
            DrawingMutation::UpdateLayerTransform(_) => 6,
            DrawingMutation::ReplaceLayerFill(_) => 7,
            DrawingMutation::ReplaceLayerStroke(_) => 8,
            DrawingMutation::SetLayerBooleanOperation(_) => 9,
            DrawingMutation::UpdateLayerTraceParams(_) => 10,
            DrawingMutation::CreateLayer(_) => 11,
            DrawingMutation::DuplicateLayer(_) => 12,
            DrawingMutation::DeleteLayer(_) => 13,
            DrawingMutation::ReorderLayer(_) => 14,
            DrawingMutation::UpdatePathGeometry(_) => 15,
            DrawingMutation::UpdateText(_) => 16,
            DrawingMutation::SetLayerFillRule(_) => 17,
            DrawingMutation::SetGroupIsolation(_) => 18,
            DrawingMutation::DragLayers(_) => 19,
            DrawingMutation::RotateLayers(_) => 20,
            DrawingMutation::ScaleLayers(_) => 21,
            DrawingMutation::DragPathPoints(_) => 22,
        }
    }

    fn finish(&mut self, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        self.credit.seal(digest, cx)?;
        self.terminal = true;
        Ok(true)
    }

    /// 🧮️ The scalar parameters of a selection transform leaf in declaration order, and how many there are.
    fn selection_scalars(mutation: &DrawingMutation) -> ([f64; 4], usize) {
        match mutation {
            DrawingMutation::DragLayers(value) => ([value.dx, value.dy, 0.0, 0.0], 2),
            DrawingMutation::RotateLayers(value) => ([value.pivot_x, value.pivot_y, value.angle, 0.0], 3),
            DrawingMutation::ScaleLayers(value) => ([value.pivot_x, value.pivot_y, value.scale_x, value.scale_y], 4),
            DrawingMutation::DragPathPoints(value) => ([value.dx, value.dy, 0.0, 0.0], 2),
            _ => ([0.0; 4], 0),
        }
    }

    fn step(&mut self, mutation: &DrawingMutation, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.phase == 0 {
            self.credit.observe(digest, 1, &[Self::variant(mutation)], cx)?;
            self.phase = 1;
            return Ok(false);
        }
        if self.phase == 1 {
            match mutation {
                DrawingMutation::CreateLayer(value) => {
                    self.credit.add_source_owner(1, size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.add_derived_owner(1, size_of::<Box<DrawingLayerNode>>())?;
                    self.credit.observe(digest, 2, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 2 } else { 3 };
                }
                DrawingMutation::DragLayers(crate::mutations::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(crate::mutations::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(crate::mutations::ScaleLayers { targets, .. }) => {
                    self.credit.source_vec(targets)?;
                    self.credit.scalar_usize(digest, 2, targets.len(), cx)?;
                    self.phase = 2;
                }
                DrawingMutation::DragPathPoints(value) => {
                    self.credit.source_vec(&value.targets)?;
                    self.credit.scalar_usize(digest, 2, value.targets.len(), cx)?;
                    self.phase = 2;
                }
                _ => {
                    let target = DrawingMutationCandidateAuthority::target_owner(mutation).ok_or("drawing-store.mutation-target-owner")?;
                    if !self.credit.observe_owned_string(digest, 2, target, false, cx)? { return Ok(false); }
                    self.phase = 2;
                }
            }
            return Ok(false);
        }
        match mutation {
            DrawingMutation::DragLayers(crate::mutations::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(crate::mutations::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(crate::mutations::ScaleLayers { targets, .. }) => {
                if let Some(target) = targets.get(self.segment_index) {
                    if !self.credit.observe_owned_string(digest, 3, target, false, cx)? { return Ok(false); }
                    self.segment_index += 1;
                    return Ok(false);
                }
                let (scalars, count) = Self::selection_scalars(mutation);
                let offset = usize::from(self.phase - 2);
                if offset < count {
                    self.credit.scalar_f64(digest, 4 + offset as u16, scalars[offset], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::DragPathPoints(value) => {
                if let Some(target) = value.targets.get(self.segment_index) {
                    if !self.credit.observe_owned_string(digest, 3, &target.layer_id, false, cx)? { return Ok(false); }
                    let point = match target.point {
                        crate::schema::geometry::editing::PathPoint::Anchor => 0u64,
                        crate::schema::geometry::editing::PathPoint::Control1 => 1,
                        crate::schema::geometry::editing::PathPoint::Control2 => 2,
                    };
                    self.credit.scalar_usize(digest, 4, (target.index as u64).checked_mul(3).and_then(|index| index.checked_add(point)).ok_or("drawing-store.mutation-point-index-overflow")? as usize, cx)?;
                    self.segment_index += 1;
                    return Ok(false);
                }
                let (scalars, count) = Self::selection_scalars(mutation);
                let offset = usize::from(self.phase - 2);
                if offset < count {
                    self.credit.scalar_f64(digest, 5 + offset as u16, scalars[offset], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdatePathGeometry(value) => match self.phase {
                2 => {
                    self.credit.source_vec(&value.segments)?;
                    self.credit.derived_vec(&value.segments)?;
                    self.credit.scalar_usize(digest, 370, value.segments.len(), cx)?;
                    self.phase = 3;
                    Ok(false)
                }
                _ => {
                    let Some(segment) = value.segments.get(self.segment_index) else { return self.finish(digest, cx) };
                    if !crate::schema::valid_path_segment(segment) { return Err("drawing-store.path-invalid-segment"); }
                    let cursor = self.segment.get_or_insert_with(DrawingPathSegmentDigestAuthority::new);
                    if cursor.step(segment, digest, &mut self.credit, cx)? { self.segment = None; self.segment_index += 1; }
                    Ok(false)
                }
            },
            DrawingMutation::SetLayerVisible(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.visible)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerLocked(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest, 3, &[u8::from(value.locked)], cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerFillRule(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest,3,&[u8::from(value.fill_rule==crate::FillRule::Nonzero)],cx)?;
                    self.phase=3;Ok(false)
                } else {self.finish(digest,cx)}
            }
            DrawingMutation::SetGroupIsolation(value) => {
                if self.phase == 2 {
                    self.credit.observe(digest,3,&[u8::from(value.isolation)],cx)?;
                    self.phase=3;Ok(false)
                } else {self.finish(digest,cx)}
            }
            DrawingMutation::SetLayerOpacity(value) => {
                if self.phase == 2 {
                    self.credit.scalar_f64(digest, 3, value.opacity, cx)?;
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBlendMode(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.blend_mode, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateText(value) => match self.phase {
                2 => { if !self.credit.observe_owned_string(digest, 3, &value.content, true, cx)? { return Ok(false); } self.phase = 3; Ok(false) }
                3 => { self.credit.scalar_f64(digest, 4, value.size, cx)?; self.phase = 4; Ok(false) }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::RenameLayer(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.new_name, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTransform(value) => {
                if self.phase <= 7 {
                    let fields = [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation, value.transform.shear];
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), fields[(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerFill(value) => {
                if self.phase == 2 {
                    let fill = self.fill.get_or_insert_with(DrawingFillDigestAuthority::new);
                    if fill.step(value.fill.as_ref(), digest, &mut self.credit, cx)? {
                        self.fill = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::ReplaceLayerStroke(value) => {
                if self.phase == 2 {
                    let stroke = self.stroke.get_or_insert_with(DrawingStrokeDigestAuthority::new);
                    if stroke.step(value.stroke.as_ref(), digest, &mut self.credit, cx)? {
                        self.stroke = None;
        self.segment = None;
                        self.phase = 3;
                    }
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::SetLayerBooleanOperation(value) => {
                if self.phase == 2 {
                    if !self.credit.observe_owned_string(digest, 3, &value.boolean_operation, true, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::UpdateLayerTraceParams(value) => {
                if self.phase <= 3 {
                    self.credit.scalar_f64(digest, 3 + u16::from(self.phase - 2), [value.params.threshold, value.params.simplify_epsilon][(self.phase - 2) as usize], cx)?;
                    self.phase += 1;
                    Ok(false)
                } else {
                    self.finish(digest, cx)
                }
            }
            DrawingMutation::CreateLayer(value) => match self.phase {
                2 => {
                    if !self.credit.observe_owned_string(digest, 3, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)? { return Ok(false); }
                    self.phase = 3;
                    Ok(false)
                }
                3 => {
                    self.credit.observe(digest, 4, &[u8::from(value.index.is_some())], cx)?;
                    self.phase = if value.index.is_some() { 4 } else { 5 };
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index.ok_or("drawing-store.digest-index-missing")?, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                5 => {
                    let layer = self.layer.get_or_insert_with(DrawingLayerDigestAuthority::new);
                    if layer.step(&value.layer, digest, &mut self.credit, cx)? {
                        self.layer = None;
                        self.phase = 6;
                    }
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
            DrawingMutation::DuplicateLayer(_) | DrawingMutation::DeleteLayer(_) => self.finish(digest, cx),
            DrawingMutation::ReorderLayer(value) => match self.phase {
                2 => {
                    self.credit.observe(digest, 3, &[u8::from(value.parent_id.is_some())], cx)?;
                    self.phase = if value.parent_id.is_some() { 3 } else { 4 };
                    Ok(false)
                }
                3 => {
                    if !self.credit.observe_owned_string(digest, 4, value.parent_id.as_ref().ok_or("drawing-store.digest-parent-missing")?, false, cx)? { return Ok(false); }
                    self.phase = 4;
                    Ok(false)
                }
                4 => {
                    self.credit.scalar_usize(digest, 5, value.index, cx)?;
                    self.phase = 5;
                    Ok(false)
                }
                _ => self.finish(digest, cx),
            },
        }
    }

    fn totals(&self) -> Option<DrawingSemanticDigestTotals> {
        self.terminal.then(|| self.credit.totals()).flatten()
    }

    fn close_step(&mut self, _maximum_bytes: usize) -> store::SnapshotRetirementStep {
        if let Some(layer) = self.layer.as_mut() {
            layer.close();
        }
        self.layer = None;
        self.fill = None;
        self.stroke = None;
        self.segment = None;
        self.terminal = true;
        store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.layer.is_none() && self.fill.is_none() && self.stroke.is_none() && self.segment.is_none()
    }
}

impl Drop for DrawingMutationDigestAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation digest reached Drop before exact terminal close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DrawingMutationWorksetPlan {
    source_items: usize,
    source_bytes: usize,
    mutation_items: usize,
    mutation_bytes: usize,
    arena_items: usize,
    arena_bytes: usize,
    authority_items: usize,
    authority_bytes: usize,
    clone_items: usize,
    clone_bytes: usize,
    maximum_container: usize,
    container_slots: usize,
}

impl DrawingMutationWorksetPlan {
    fn checked_total(values: &[usize], fault: &'static str) -> Result<usize, &'static str> {
        values.iter().try_fold(0usize, |total, value| total.checked_add(*value).ok_or(fault))
    }

    fn admit(
        source: DrawingSnapshotOwnerTotals,
        mutation: DrawingSemanticDigestTotals,
        arena_items: usize,
        arena_bytes: usize,
        container_slots: usize,
    ) -> Result<Self, &'static str> {
        if source.source_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.preflight-item-capacity");
        }
        if source.source_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.preflight-byte-capacity");
        }
        if mutation.source_owner_items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-source-owner-item-capacity");
        }
        if mutation.source_owner_bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-source-owner-byte-capacity");
        }
        if mutation.maximum_field_bytes > DRAWING_OWNED_FIELD_BYTES {
            return Err("drawing-store.mutation-field-capacity");
        }
        Ok(Self {
            source_items: source.source_items,
            source_bytes: source.source_bytes,
            mutation_items: mutation.semantic_items,
            mutation_bytes: mutation.maximum_field_bytes,
            arena_items,
            arena_bytes,
            authority_items: 1,
            authority_bytes: size_of::<DrawingMutationCandidateAuthority>(),
            clone_items: 0,
            clone_bytes: 0,
            maximum_container: source.maximum_container,
            container_slots,
        })
    }

    fn admit_clone(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        if items > DRAWING_MAXIMUM_NESTED_ITEMS {
            return Err("drawing-store.mutation-clone-item-capacity");
        }
        if bytes > DRAWING_MAXIMUM_NESTED_BYTES {
            return Err("drawing-store.mutation-clone-byte-capacity");
        }
        self.clone_items = items;
        self.clone_bytes = bytes;
        self.workset_items()?;
        self.workset_bytes()?;
        Ok(())
    }

    fn workset_items(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.arena_items, self.authority_items, self.clone_items], "drawing-store.mutation-workset-item-overflow")
    }

    fn workset_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.arena_bytes, self.authority_bytes, self.clone_bytes], "drawing-store.mutation-workset-byte-overflow")
    }

    fn simultaneous_bytes(&self) -> Result<usize, &'static str> {
        Self::checked_total(&[self.source_bytes, self.mutation_bytes, self.workset_bytes()?], "drawing-store.mutation-simultaneous-byte-overflow")
    }

    fn architectural_maximum_bytes(&self) -> Result<usize, &'static str> {
        DRAWING_MAXIMUM_NESTED_BYTES
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(DRAWING_OWNED_FIELD_BYTES))
            .and_then(|bytes| bytes.checked_add(self.arena_bytes))
            .and_then(|bytes| bytes.checked_add(self.authority_bytes))
            .ok_or("drawing-store.mutation-architectural-byte-overflow")
    }
}

struct DrawingDuplicateReferenceSearch {
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    owners: Option<(usize, usize)>,
    equality: native_text_footprint::DrawingTextEqualityCursor,
    result: Option<bool>,
}

impl DrawingDuplicateReferenceSearch {
    fn new() -> Self { Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], owners: None, equality: Default::default(), result: None } }

    fn node_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut node = root;
        for index in path { let DrawingLayerNode::Group(group) = node else { return None };node = group.children.get(*index)?; }
        Some(node)
    }

    fn step(&mut self, root: &DrawingLayerNode, target: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let owners = (root as *const _ as usize, target as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.duplicate-reference-source-changed"); }
        self.owners = Some(owners);
        if self.result.is_some() { return Ok(true); }
        let node = Self::node_at(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-reference-path")?;
        if self.frames[self.depth].phase == 0 {
            let step = self.equality.step(crate::schema::layer_id(node), target, 1, DRAWING_OWNED_FIELD_BYTES)?;
            cx.consume_fuel(step.compared_bytes.max(1) as u64);
            if !step.complete { return Ok(false); }
            let equal = self.equality.result().ok_or("drawing-store.duplicate-reference-equality")?;
            self.equality.close_step(1);self.equality = Default::default();
            if equal { self.result = Some(true);return Ok(true); }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH { return Err("drawing-store.duplicate-reference-depth-capacity"); }
                self.frames[self.depth].child += 1;self.path[self.depth] = child;self.depth += 1;self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);return Ok(false);
            }
        }
        if self.depth == 0 { self.result = Some(false);Ok(true) } else { self.depth -= 1;cx.consume_fuel(1);Ok(false) }
    }

    fn close(&mut self) { self.equality.close_step(1);self.owners = None;self.result = None; }
}

struct DrawingDuplicateRewriteAuthority {
    depth: usize,
    path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    owners: Option<(usize, usize)>,
    identity: native_text_footprint::DrawingDuplicateIdentityCursor,
    append: semio_framework_value::paged::PagedUtf8AppendCursor,
    append_active: bool,
    native_id: std::mem::ManuallyDrop<DrawingNativeText>,
    displaced: Option<DrawingDecodedFieldRetirement<DrawingNativeText>>,
    search: Option<DrawingDuplicateReferenceSearch>,
    reference: usize,
    turn: usize,
    mode: u8,
    pending_id: std::mem::ManuallyDrop<Option<String>>,
    pending_name: std::mem::ManuallyDrop<Option<String>>,
    terminal: bool,
}

impl DrawingDuplicateRewriteAuthority {
    fn new(mut pending_id: String, mut pending_name: String) -> Self {
        pending_id.clear();pending_name.clear();
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], owners: None, identity: Default::default(), append: Default::default(), append_active: false, native_id: std::mem::ManuallyDrop::new(Default::default()), displaced: None, search: None, reference: 0, turn: 0, mode: 0, pending_id: std::mem::ManuallyDrop::new(Some(pending_id)), pending_name: std::mem::ManuallyDrop::new(Some(pending_name)), terminal: false }
    }

    fn close_append(&mut self, grant: RetainedCloneGrant) -> Result<bool, &'static str> {
        if !self.append_active { return Ok(true); }
        self.append.begin_close();
        let step = self.append.close_granted(grant).map_err(|_| "drawing-store.duplicate-append-close")?;
        if matches!(step, RetainedCloneStep::Complete(_)) && self.append.terminal_is_empty() { self.append = Default::default();self.append_active = false;return Ok(true); }
        Ok(false)
    }

    fn retire_displaced_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if let Some(displaced) = self.displaced.as_mut() {
            let step = displaced.step_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && displaced.terminal_is_empty() { self.displaced = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DrawingNativeText>(), ..Default::default() };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let value = std::mem::take(&mut *self.native_id);
        match DrawingDecodedFieldRetirement::try_new(value) {
            Ok(retirement) => self.displaced = Some(retirement),
            Err((error, value)) => { *self.native_id = value;return Err(error); }
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    fn step(&mut self, root: &mut DrawingLayerNode, original: &DrawingLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal { return Ok(true); }
        let owners = (root as *const _ as usize, original as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.duplicate-root-changed"); }
        self.owners = Some(owners);
        let grant = initial_snapshot_clone_grant(self.turn);self.turn = self.turn.wrapping_add(1);
        let source = DrawingDuplicateReferenceSearch::node_at(original, &self.path[..self.depth]).ok_or("drawing-store.duplicate-source-path")?;
        let node = DrawingLayerCloneAuthority::target_at_mut(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-path")?;
        let phase = self.frames[self.depth].phase;
        if self.mode == 0 && phase < 7 {
            match phase {
                0 => {
                    let step = self.identity.step(crate::schema::layer_id(source), " copy", 1, DRAWING_OWNED_FIELD_BYTES)?;
                    cx.consume_fuel(step.observed_bytes.max(1) as u64);
                    if step.complete { self.frames[self.depth].phase = 1; }
                }
                1 => {
                    self.append_active = true;
                    let id = self.identity.identity().ok_or("drawing-store.duplicate-identity-missing")?;
                    let step = self.append.advance(id, &mut self.native_id, grant).map_err(|_| "drawing-store.duplicate-identity-append")?;
                    cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                    if matches!(step, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 2; }
                }
                2 => { if self.close_append(grant)? { self.frames[self.depth].phase = 3; }cx.consume_fuel(1); }
                3 => {
                    std::mem::swap(&mut crate::schema::layer_base_mut(node).id, &mut self.native_id);
                    self.identity.close_step(1);self.identity = Default::default();self.frames[self.depth].phase = 4;
                    cx.consume_fuel(size_of::<DrawingNativeText>() as u64);
                }
                4 => { if matches!(self.retire_displaced_granted(grant).map_err(|_| "drawing-store.duplicate-text-close")?, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 5; }cx.consume_fuel(1); }
                5 if self.depth == 0 => {
                    self.append_active = true;
                    let step = self.append.advance(" copy", &mut crate::schema::layer_base_mut(node).name, grant).map_err(|_| "drawing-store.duplicate-name-append")?;
                    cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                    if matches!(step, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 6; }
                }
                5 => { self.frames[self.depth].phase = 7;cx.consume_fuel(1); }
                6 => { if self.close_append(grant)? { self.frames[self.depth].phase = 7; }cx.consume_fuel(1); }
                _ => unreachable!(),
            }
            return Ok(false);
        }
        if self.mode == 1 {
            if let (DrawingLayerNode::Boolean(source), DrawingLayerNode::Boolean(node)) = (source, &mut *node) {
                if self.reference < source.children.len() {
                    let target = source.children.get(self.reference).ok_or("drawing-store.duplicate-reference-target")?;
                    match phase {
                        0 => {
                            let search = self.search.get_or_insert_with(DrawingDuplicateReferenceSearch::new);
                            if !search.step(original, target, cx)? { return Ok(false); }
                            let internal = search.result.ok_or("drawing-store.duplicate-reference-result")?;
                            search.close();self.search = None;
                            if internal { self.frames[self.depth].phase = 1; } else { self.reference += 1; }
                        }
                        1 => {
                            let step = self.identity.step(target, " copy", 1, DRAWING_OWNED_FIELD_BYTES)?;
                            cx.consume_fuel(step.observed_bytes.max(1) as u64);
                            if step.complete { self.frames[self.depth].phase = 2; }
                        }
                        2 => {
                            self.append_active = true;
                            let id = self.identity.identity().ok_or("drawing-store.duplicate-reference-identity")?;
                            let step = self.append.advance(id, &mut self.native_id, grant).map_err(|_| "drawing-store.duplicate-reference-append")?;
                            cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                            if matches!(step, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 3; }
                        }
                        3 => { if self.close_append(grant)? { self.frames[self.depth].phase = 4; }cx.consume_fuel(1); }
                        4 => {
                            let child = node.children.get_mut(self.reference).ok_or("drawing-store.duplicate-reference-destination")?;
                            std::mem::swap(child, &mut self.native_id);self.identity.close_step(1);self.identity = Default::default();self.frames[self.depth].phase = 5;
                            cx.consume_fuel(size_of::<DrawingNativeText>() as u64);
                        }
                        5 => { if matches!(self.retire_displaced_granted(grant).map_err(|_| "drawing-store.duplicate-text-close")?, RetainedCloneStep::Complete(_)) { self.reference += 1;self.frames[self.depth].phase = 0; }cx.consume_fuel(1); }
                        _ => return Err("drawing-store.duplicate-reference-phase"),
                    }
                    return Ok(false);
                }
            }
        }
        if let DrawingLayerNode::Group(group) = source {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH { return Err("drawing-store.duplicate-depth-capacity"); }
                self.frames[self.depth].child += 1;self.path[self.depth] = child;self.depth += 1;self.frames[self.depth] = DrawingTraversalFrame::EMPTY;self.reference = 0;
                cx.consume_fuel(1);return Ok(false);
            }
        }
        if self.depth == 0 {
            if self.mode == 0 { self.mode = 1;self.frames = [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH];self.path = [0; DRAWING_MAXIMUM_LAYER_DEPTH];self.reference = 0;cx.consume_fuel(1);return Ok(false); }
            self.terminal = true;self.owners = None;Ok(true)
        } else { self.depth -= 1;self.reference = 0;cx.consume_fuel(1);Ok(false) }
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.terminal { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.identity.terminal_is_empty() { self.identity.close_step(1);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if let Some(search) = self.search.as_mut() { search.close();self.search = None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if self.append_active {
            self.append.begin_close();
            let step = self.append.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && self.append.terminal_is_empty() { self.append = Default::default();self.append_active = false; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = self.retire_displaced_granted(grant)?;
        if !matches!(step, RetainedCloneStep::Complete(_)) { return Ok(step); }
        self.owners = None;self.terminal = true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn close_step(&mut self, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_bytes == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        let mut grant = initial_snapshot_clone_grant(self.turn);self.turn = self.turn.wrapping_add(1);
        grant.maximum_copy_bytes = grant.maximum_copy_bytes.min(maximum_bytes);
        grant.maximum_capacity_bytes = grant.maximum_capacity_bytes.min(maximum_bytes);
        grant.maximum_release_bytes = grant.maximum_release_bytes.min(maximum_bytes);
        let step = self.close_granted(grant)?;
        Ok(match step { RetainedCloneStep::Complete(_) => store::SnapshotRetirementStep::Complete, RetainedCloneStep::Progress(progress) => store::SnapshotRetirementStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes } })
    }

    fn take_owners(&mut self) -> Option<(String, String)> {
        if !self.terminal || self.append_active || self.displaced.is_some() || self.search.is_some() || !self.identity.terminal_is_empty() { return None; }
        let mut id = self.pending_id.take()?;let mut name = self.pending_name.take()?;id.clear();name.clear();Some((id, name))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.pending_id.is_none() && self.pending_name.is_none() && self.owners.is_none() && !self.append_active && self.displaced.is_none() && self.search.is_none() && self.identity.terminal_is_empty() && self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty()
    }
}

impl Drop for DrawingDuplicateRewriteAuthority {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing duplicate rewrite reached Drop before staged id/name retirement"); }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingMutationCandidatePhase {
    PreflightSource,
    PreflightMutation,
    LocateCloneSource,
    PrepareOwnedValue,
    PlanOwnedValue,
    BindOverlay,
    LocatePrimary,
    LocateSecondary,
    Apply,
    RebuildSource,
    LocateDestination,
    RebuildDestination,
    Complete,
    Retire,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingContainerRebuildRole {
    Source,
    Destination,
    CloseSourceUndo,
}

#[derive(Clone, Copy)]
struct DrawingContainerSourceUndo {
    parent: Option<DrawingLayerAddress>,
    index: usize,
}

struct DrawingMutationOverlayPatch {
    source_owner: usize,
    committed: bool,
}

impl DrawingMutationOverlayPatch {
    fn bind(source: &DrawingSnapshot) -> Self {
        Self { source_owner: std::ptr::from_ref(source) as usize, committed: false }
    }

    fn validate(&self, source: &DrawingSnapshot) -> Result<(), &'static str> {
        if self.source_owner != std::ptr::from_ref(source) as usize {
            return Err("drawing-store.mutation-overlay-owner-changed");
        }
        Ok(())
    }

    fn commit(&mut self, source: &DrawingSnapshot) -> Result<(), &'static str> {
        self.validate(source)?;
        self.committed = true;
        Ok(())
    }
}

struct DrawingMutationCandidateAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    arena_pool: Option<std::sync::Arc<DrawingMutationArenaPool>>,
    arena_slot: usize,
    arena_generation: u64,
    arena_return_phase: u8,
    preflight_source: Option<DrawingSnapshotBoundsAuthority>,
    preflight_mutation: std::mem::ManuallyDrop<Option<DrawingMutationDigestAuthority>>,
    preflight_digest: Option<store::ArtifactStoreInitializationDigest>,
    workset: Option<DrawingMutationWorksetPlan>,
    overlay: Option<DrawingMutationOverlayPatch>,
    locator: Option<DrawingLayerLocator>,
    primary: Option<DrawingLayerAddress>,
    secondary: Option<DrawingLayerAddress>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<DrawingLayerCloneAuthority>>>,
    clone_work: Option<DrawingLayerCloneWorkAuthority>,
    fill_clone: std::mem::ManuallyDrop<Option<DrawingFillCloneAuthority>>,
    stroke_clone: std::mem::ManuallyDrop<Option<DrawingStrokeCloneAuthority>>,
    segments_clone: std::mem::ManuallyDrop<Option<DrawingSegmentsCloneAuthority>>,
    text_clone: std::mem::ManuallyDrop<Option<DrawingTextCloneAuthority>>,
    clone_text_work: native_text_footprint::DrawingTextFootprintCursor,
    duplicate_rewrite: Option<DrawingDuplicateRewriteAuthority>,
    duplicate_id_owner: std::mem::ManuallyDrop<Option<String>>,
    rebuild: std::mem::ManuallyDrop<Option<DrawingContainerRebuildAuthority>>,
    rebuild_target: Option<Option<DrawingLayerAddress>>,
    rebuild_role: Option<DrawingContainerRebuildRole>,
    rebuild_close_phase: u8,
    source_undo: Option<DrawingContainerSourceUndo>,
    container_reverse: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    container_output: std::mem::ManuallyDrop<Option<Vec<DrawingLayerNode>>>,
    overlay_pages: std::mem::ManuallyDrop<Option<Vec<String>>>,
    pending_layer: std::mem::ManuallyDrop<Option<DrawingLayerNode>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: DrawingMutationCandidatePhase,
    terminal: bool,
    fault: Option<&'static str>,
}

impl DrawingMutationCandidateAuthority {
    /// 🧮️ Borrows one slot of the process arena pool. `NotReady` (the pool is still bootstrapping under
    /// maintenance) and `Contended` (another thread holds the pool for one bounded borrow) are
    /// transient: the caller yields and borrows again on a later step.
    fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<Self, DrawingMutationArenaBorrowError> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_drawing_mutation_arena()?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    #[cfg(test)]
    fn try_new_from_pool(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pool: std::sync::Arc<DrawingMutationArenaPool>) -> Result<Self, &'static str> {
        let (arena_pool, arena_slot, arena_generation, owner) = borrow_drawing_mutation_arena_from(pool)?;
        Ok(Self::from_arena(operation, generation, arena_pool, arena_slot, arena_generation, owner))
    }

    fn from_arena(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, arena_pool: std::sync::Arc<DrawingMutationArenaPool>, arena_slot: usize, arena_generation: u64, owner: DrawingMutationArenaOwner) -> Self {
        Self {
            operation,
            generation,
            arena_pool: Some(arena_pool),
            arena_slot,
            arena_generation,
            arena_return_phase: 0,
            preflight_source: Some(DrawingSnapshotBoundsAuthority::new()),
            preflight_mutation: std::mem::ManuallyDrop::new(Some(DrawingMutationDigestAuthority::new())),
            preflight_digest: Some(store::ArtifactStoreInitializationDigest::new(b"drawing.mutation-preflight")),
            workset: None,
            overlay: None,
            locator: None,
            primary: None,
            secondary: None,
            layer_clone: std::mem::ManuallyDrop::new(None),
            clone_work: None,
            fill_clone: std::mem::ManuallyDrop::new(None),
            stroke_clone: std::mem::ManuallyDrop::new(None),
            segments_clone: std::mem::ManuallyDrop::new(None),
            text_clone: std::mem::ManuallyDrop::new(None),
            clone_text_work: native_text_footprint::DrawingTextFootprintCursor::default(),
            duplicate_rewrite: None,
            duplicate_id_owner: std::mem::ManuallyDrop::new(Some(owner.duplicate_id)),
            rebuild: std::mem::ManuallyDrop::new(None),
            rebuild_target: None,
            rebuild_role: None,
            rebuild_close_phase: 0,
            source_undo: None,
            container_reverse: std::mem::ManuallyDrop::new(Some(owner.reverse)),
            container_output: std::mem::ManuallyDrop::new(Some(owner.output)),
            overlay_pages: std::mem::ManuallyDrop::new(Some(owner.pages)),
            pending_layer: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: DrawingMutationCandidatePhase::PreflightSource,
            terminal: false,
            fault: None,
        }
    }

    fn target_owner(mutation: &DrawingMutation) -> Option<&DrawingNativeText> {
        match mutation {
            DrawingMutation::SetLayerVisible(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerLocked(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerOpacity(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerFillRule(value) => Some(&value.layer_id),
            DrawingMutation::SetGroupIsolation(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerBlendMode(value) => Some(&value.layer_id),
            DrawingMutation::RenameLayer(value) => Some(&value.layer_id),
            DrawingMutation::UpdateLayerTransform(value) => Some(&value.layer_id),
            DrawingMutation::ReplaceLayerFill(value) => Some(&value.layer_id),
            DrawingMutation::ReplaceLayerStroke(value) => Some(&value.layer_id),
            DrawingMutation::SetLayerBooleanOperation(value) => Some(&value.layer_id),
            DrawingMutation::UpdateLayerTraceParams(value) => Some(&value.layer_id),
            DrawingMutation::CreateLayer(value) => Some(crate::schema::layer_id(&value.layer)),
            DrawingMutation::DuplicateLayer(value) => Some(&value.layer_id),
            DrawingMutation::DeleteLayer(value) => Some(&value.layer_id),
            DrawingMutation::ReorderLayer(value) => Some(&value.layer_id),
            DrawingMutation::UpdatePathGeometry(value) => Some(&value.layer_id),
            DrawingMutation::UpdateText(value) => Some(&value.layer_id),
            DrawingMutation::DragLayers(crate::mutations::DragLayers { targets, .. }) | DrawingMutation::RotateLayers(crate::mutations::RotateLayers { targets, .. }) | DrawingMutation::ScaleLayers(crate::mutations::ScaleLayers { targets, .. }) => targets.first(),
            DrawingMutation::DragPathPoints(value) => value.targets.first().map(|target| &target.layer_id),
        }
    }

    /// 🧭️ Applies one relative selection transform through its own diff — the leaf resolves every addressed layer against
    /// its parent chain on `source`, and each patched transform or path geometry lands in place; the replaced geometry is
    /// retired through the candidate's retirement owner.
    fn apply_selection_transform(&mut self, source: &mut DrawingSnapshot, mutation: &DrawingMutation) -> Result<(), &'static str> {
        let outcome = <DrawingMutation as Mutation<DrawingSnapshot>>::diff(mutation, source);
        if !outcome.is_applicable(protocol::MergePolicy::default()) {
            return Err("drawing-store.selection-transform-rejected");
        }
        let mut retired: semio_framework_value::list::PagedList<semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>, {usize::MAX}> = Default::default();
        for entry in outcome.diff().layers.iter().flat_map(|delta| delta.patched.iter()) {
            let mut fault = None;
            let found = crate::schema::update_layer_in_tree(&mut source.layers, &entry.id, &mut |layer| {
                if let Some(transform) = entry.patch.transform.as_ref() {
                    crate::schema::layer_base_mut(layer).transform = transform.clone();
                }
                if let Some(segments) = entry.patch.path_segments.as_ref() {
                    match layer {
                        DrawingLayerNode::Path(path) => retired.push(std::mem::replace(&mut path.segments, segments.clone())),
                        _ => fault = Some("drawing-store.path-target-kind"),
                    }
                }
            });
            if let Some(fault) = fault {
                return Err(fault);
            }
            if !found {
                return Err("drawing-store.mutation-target-lost");
            }
        }
        if !retired.is_empty() {
            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::SegmentCollections(retired))));
        }
        Ok(())
    }

    fn parent(mutation: &DrawingMutation) -> Option<&DrawingNativeText> {
        match mutation {
            DrawingMutation::CreateLayer(value) => value.parent_id.as_ref(),
            DrawingMutation::ReorderLayer(value) => value.parent_id.as_ref(),
            _ => None,
        }
    }

    fn fail(&mut self, fault: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(fault);
        self.phase = DrawingMutationCandidatePhase::Fault;
        Err(fault)
    }

    fn return_arena_owner(&mut self) -> Result<Option<bool>, &'static str> {
        let Some(pool) = self.arena_pool.as_ref() else {
            return Ok(Some(true));
        };
        let Ok(mut state) = pool.state.try_lock() else {
            return Ok(None);
        };
        let slot = state.slots.get_mut(self.arena_slot).ok_or("drawing-store.mutation-arena-slot")?;
        if !slot.leased || slot.generation != self.arena_generation {
            return Err("drawing-store.mutation-arena-stale-generation");
        }
        match self.arena_return_phase {
            0 => {
                let reverse = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?;
                if !reverse.is_empty() || reverse.capacity() < DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY || slot.reverse.is_some() {
                    return Err("drawing-store.mutation-reverse-arena-not-terminal");
                }
                slot.reverse = Some(self.container_reverse.take().expect("validated Drawing reverse owner remains retained"));
            }
            1 => {
                let output = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?;
                if !output.is_empty() || output.capacity() < DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY || slot.output.is_some() {
                    return Err("drawing-store.mutation-output-arena-not-terminal");
                }
                slot.output = Some(self.container_output.take().expect("validated Drawing output owner remains retained"));
            }
            2 => {
                let pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                if pages.len() != DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.capacity() < DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY
                    || pages.iter().any(|page| !page.is_empty() || page.capacity() < DRAWING_MUTATION_RETAINED_PAGE_BYTES)
                    || slot.pages.is_some()
                {
                    return Err("drawing-store.mutation-overlay-arena-not-terminal");
                }
                slot.pages = Some(self.overlay_pages.take().expect("validated Drawing page owner remains retained"));
            }
            3 => {
                let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("drawing-store.mutation-duplicate-id-owner-missing")?;
                if !duplicate_id.is_empty() || duplicate_id.capacity() < DRAWING_DUPLICATE_ID_BYTES || slot.duplicate_id.is_some() {
                    return Err("drawing-store.mutation-duplicate-id-owner-not-terminal");
                }
                slot.duplicate_id = Some(self.duplicate_id_owner.take().expect("validated Drawing duplicate owner remains retained"));
                slot.leased = false;
            }
            _ => return Err("drawing-store.mutation-arena-return-phase"),
        }
        self.arena_return_phase += 1;
        let complete = self.arena_return_phase == 4;
        if complete && !slot.is_available() {
            return Err("drawing-store.mutation-arena-return-false-terminal");
        }
        drop(state);
        if complete {
            self.arena_pool = None;
        }
        Ok(Some(complete))
    }

    fn prepare_native_text(&mut self, source: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.text_clone.is_none() {
            *self.text_clone = Some(DrawingTextCloneAuthority::new());
            cx.consume_fuel(1);
            return Ok(false);
        }
        let clone = self.text_clone.as_mut().ok_or("drawing-store.text-clone-missing")?;
        if !clone.step(source, cx)? { return Ok(false); }
        let value = clone.value.as_ref().ok_or("drawing-store.text-clone-value")?;
        let complete = self.clone_text_work.step(value, 1)?;
        cx.consume_fuel(1);
        if !complete { return Ok(false); }
        let totals = self.clone_text_work.totals().ok_or("drawing-store.text-clone-work-incomplete")?;
        self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.chunks.checked_add(1).ok_or("drawing-store.text-clone-item-overflow")?, totals.backing_bytes)?;
        self.clone_text_work.close_step(1);
        Ok(true)
    }

    fn adopt_native_text(&mut self, target: &mut DrawingNativeText) -> Result<(), &'static str> {
        let replacement = self.text_clone.as_mut().ok_or("drawing-store.text-clone-missing")?.take().ok_or("drawing-store.text-false-terminal")?;
        let old = std::mem::replace(target, replacement);
        *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::String(old))));
        drop(self.text_clone.take());
        Ok(())
    }

    fn start_rebuild(&mut self, source: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>, remove_index: Option<usize>, insert_index: Option<usize>, role: DrawingContainerRebuildRole) -> Result<(), &'static str> {
        let workset = self.workset.ok_or("drawing-store.mutation-workset-missing")?;
        if self.container_reverse.is_none() {
            return Err("drawing-store.mutation-reverse-arena-missing");
        }
        if self.container_output.is_none() {
            return Err("drawing-store.mutation-output-arena-missing");
        }
        let container = DrawingLayerLocator::container_mut(source, parent).ok_or("drawing-store.mutation-container-missing")?;
        let source = std::mem::take(container);
        let pending = self.pending_layer.take();
        let reverse = self.container_reverse.take().expect("validated Drawing reverse arena remains retained");
        let output = self.container_output.take().expect("validated Drawing output arena remains retained");
        match DrawingContainerRebuildAuthority::new(source, remove_index, insert_index, pending, reverse, output, workset) {
            Ok(rebuild) => {
                *self.rebuild = Some(rebuild);
                self.rebuild_target = Some(parent);
                self.rebuild_role = Some(role);
                self.rebuild_close_phase = 0;
                Ok(())
            }
            Err(rejected) => {
                *container = rejected.source;
                *self.pending_layer = rejected.pending;
                *self.container_reverse = Some(rejected.reverse);
                *self.container_output = Some(rejected.output);
                Err("drawing-store.mutation-container-admission")
            }
        }
    }

    fn finish_rebuild(&mut self, source: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Result<Option<DrawingLayerNode>, &'static str> {
        if DrawingLayerLocator::container_mut(source, parent).is_none() {
            return Err("drawing-store.mutation-container-lost");
        }
        let (rebuilt_source, removed, reverse_arena, output_arena) = self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.take().ok_or("drawing-store.mutation-rebuild-false-terminal")?;
        *DrawingLayerLocator::container_mut(source, parent).expect("validated Drawing mutation container remains available") = rebuilt_source;
        *self.container_reverse = Some(reverse_arena);
        *self.container_output = Some(output_arena);
        let mut rebuild = self.rebuild.take().expect("Drawing completed rebuild remains exact");
        rebuild.terminal = true;
        drop(rebuild);
        self.rebuild_target = None;
        self.rebuild_role = None;
        self.rebuild_close_phase = 0;
        Ok(removed)
    }

    fn step(&mut self, source: &mut DrawingSnapshot, mutation: &DrawingMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if (cx.operation() != self.operation || cx.generation() != self.generation) && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return self.fail("drawing-store.mutation-candidate-stale-authority");
        }
        if cx.is_cancelled() && !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            self.fault = Some("drawing-store.mutation-candidate-cancelled");
            self.phase = DrawingMutationCandidatePhase::Retire;
            return Err("drawing-store.mutation-candidate-cancelled");
        }
        if let Some(overlay) = self.overlay.as_ref() {
            overlay.validate(source)?;
        }
        match self.phase {
            DrawingMutationCandidatePhase::PreflightSource => {
                let bounds = self.preflight_source.as_mut().ok_or("drawing-store.mutation-source-preflight-missing")?;
                if bounds.step(source, cx)? {
                    self.phase = DrawingMutationCandidatePhase::PreflightMutation;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::PreflightMutation => {
                let digest = self.preflight_mutation.as_mut().ok_or("drawing-store.mutation-preflight-missing")?;
                if !digest.step(mutation, self.preflight_digest.as_mut().ok_or("drawing-store.mutation-preflight-digest")?, cx)? {
                    return Ok(false);
                }
                let source_credit = self.preflight_source.as_ref().and_then(DrawingSnapshotBoundsAuthority::totals).ok_or("drawing-store.mutation-source-preflight-incomplete")?;
                let mutation_credit = digest.totals().ok_or("drawing-store.mutation-preflight-incomplete")?;
                let reverse_slots = self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?.capacity();
                let output_slots = self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?.capacity();
                let overlay_pages = self.overlay_pages.as_ref().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                let overlay_slots = overlay_pages.capacity();
                let required_overlay_slots = usize::from(matches!(
                    mutation,
                    DrawingMutation::DuplicateLayer(_)
                ));
                if required_overlay_slots > overlay_slots {
                    return Err("drawing-store.mutation-overlay-slot-capacity");
                }
                let duplicate_id = self.duplicate_id_owner.as_ref().ok_or("drawing-store.mutation-duplicate-id-arena-missing")?;
                let (arena_items, arena_bytes) = DrawingMutationArenaOwner::retained_totals(
                    self.container_reverse.as_ref().ok_or("drawing-store.mutation-reverse-arena-missing")?,
                    self.container_output.as_ref().ok_or("drawing-store.mutation-output-arena-missing")?,
                    overlay_pages,
                    duplicate_id,
                )?;
                let container_slots = reverse_slots.checked_add(output_slots).ok_or("drawing-store.mutation-container-credit-overflow")?;
                self.workset = Some(DrawingMutationWorksetPlan::admit(source_credit, mutation_credit, arena_items, arena_bytes, container_slots)?);
                drop(self.preflight_mutation.take());
                self.preflight_source = None;
                self.preflight_digest = None;
                if matches!(mutation, DrawingMutation::DuplicateLayer(_)) {
                    self.locator = Some(DrawingLayerLocator::new());
                    self.phase = DrawingMutationCandidatePhase::LocateCloneSource;
                } else if matches!(mutation, DrawingMutation::CreateLayer(_) | DrawingMutation::ReplaceLayerFill(_) | DrawingMutation::ReplaceLayerStroke(_) | DrawingMutation::UpdatePathGeometry(_) | DrawingMutation::UpdateText(_) | DrawingMutation::SetLayerBlendMode(_) | DrawingMutation::RenameLayer(_) | DrawingMutation::SetLayerBooleanOperation(_)) {
                    self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                } else {
                    self.phase = DrawingMutationCandidatePhase::BindOverlay;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateCloneSource => {
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-clone-locator-missing")?;
                if !locator.step(source, Self::target_owner(mutation).ok_or("drawing-store.mutation-target-missing")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if self.primary.is_none() {
                    return Err("drawing-store.mutation-target-missing");
                }
                self.phase = DrawingMutationCandidatePhase::PrepareOwnedValue;
                Ok(false)
            }
            DrawingMutationCandidatePhase::BindOverlay => {
                self.clone_work = None;
                self.overlay = Some(DrawingMutationOverlayPatch::bind(source));
                if matches!(mutation, DrawingMutation::DuplicateLayer(_) | DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_)) {
                    self.phase = DrawingMutationCandidatePhase::Apply;
                } else {
                    self.locator = Some(DrawingLayerLocator::new());
                    self.phase = DrawingMutationCandidatePhase::LocatePrimary;
                }
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocatePrimary => {
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-locator-missing")?;
                if !locator.step(source, Self::target_owner(mutation).ok_or("drawing-store.mutation-target-missing")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found();
                self.locator = None;
                if matches!(mutation, DrawingMutation::CreateLayer(_)) {
                    if self.primary.is_some() {
                        return Err("drawing-store.mutation-duplicate-layer");
                    }
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawingLayerLocator::new());
                        self.phase = DrawingMutationCandidatePhase::LocateSecondary;
                    } else {
                        self.phase = DrawingMutationCandidatePhase::Apply;
                    }
                } else if self.primary.is_none() {
                    return Err("drawing-store.mutation-target-missing");
                } else {
                    self.phase = DrawingMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateSecondary => {
                let target = Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?;
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-parent-locator")?;
                if !locator.step(source, target, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::Apply;
                Ok(false)
            }
            DrawingMutationCandidatePhase::PrepareOwnedValue => {
                match mutation {
                    DrawingMutation::CreateLayer(value) => {
                        if self.layer_clone.is_none() {
                            *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new()));
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        let clone = self.layer_clone.as_mut().expect("Drawing create layer clone remains retained");
                        if !clone.step(&value.layer, cx)? {
                            return Ok(false);
                        }
                        *self.pending_layer = clone.take();
                        drop(self.layer_clone.take());
                        self.clone_work = Some(DrawingLayerCloneWorkAuthority::new());
                    }
                    DrawingMutation::DuplicateLayer(_) => {
                        let duplicate_source = DrawingLayerLocator::node_at(source, self.primary.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-duplicate-source")?;
                        if self.pending_layer.is_none() {
                            if self.layer_clone.is_none() {
                                *self.layer_clone = Some(Box::new(DrawingLayerCloneAuthority::new()));
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            let clone = self.layer_clone.as_mut().expect("Drawing duplicate layer clone remains retained");
                            if !clone.step(duplicate_source, cx)? {
                                return Ok(false);
                            }
                            *self.pending_layer = clone.take();
                            drop(self.layer_clone.take());
                            let name_owner = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?.pop().ok_or("drawing-store.duplicate-name-owner-missing")?;
                            self.duplicate_rewrite = Some(DrawingDuplicateRewriteAuthority::new(self.duplicate_id_owner.take().ok_or("drawing-store.duplicate-id-owner-missing")?, name_owner));
                            return Ok(false);
                        }
                        if !self.duplicate_rewrite.as_mut().ok_or("drawing-store.duplicate-rewrite-missing")?.step(self.pending_layer.as_mut().ok_or("drawing-store.duplicate-owner-missing")?, duplicate_source, cx)? {
                            return Ok(false);
                        }
                        let pages = self.overlay_pages.as_mut().ok_or("drawing-store.mutation-overlay-arena-missing")?;
                        if pages.len() >= pages.capacity() {
                            return Err("drawing-store.duplicate-name-owner-return-saturated");
                        }
                        let (id_owner, name_owner) = self.duplicate_rewrite.as_mut().and_then(DrawingDuplicateRewriteAuthority::take_owners).ok_or("drawing-store.duplicate-owner-false-terminal")?;
                        *self.duplicate_id_owner = Some(id_owner);
                        pages.push(name_owner);
                        drop(self.duplicate_rewrite.take());
                        self.clone_work = Some(DrawingLayerCloneWorkAuthority::new());
                    }
                    DrawingMutation::UpdateText(value) => {
                        if !value.size.is_finite() || value.size <= 0.0 { return Err("drawing-store.text-size-invalid"); }
                        if !self.prepare_native_text(&value.content, cx)? { return Ok(false); }
                    }
                    DrawingMutation::SetLayerBlendMode(value) => { if !self.prepare_native_text(&value.blend_mode, cx)? { return Ok(false); } }
                    DrawingMutation::RenameLayer(value) => { if !self.prepare_native_text(&value.new_name, cx)? { return Ok(false); } }
                    DrawingMutation::SetLayerBooleanOperation(value) => { if !self.prepare_native_text(&value.boolean_operation, cx)? { return Ok(false); } }
                    DrawingMutation::UpdatePathGeometry(value) => {
                        if self.segments_clone.is_none() {
                            *self.segments_clone = Some(DrawingSegmentsCloneAuthority::new());
                            cx.consume_fuel(1);
                            return Ok(false);
                        }
                        let clone = self.segments_clone.as_mut().ok_or("drawing-store.path-clone-missing")?;
                        if !clone.step(&value.segments, cx)? { return Ok(false); }
                        let (items, bytes) = DrawingLayerCloneWorkAuthority::vector(clone.value.as_ref().ok_or("drawing-store.path-clone-value")?)?;
                        self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(items, bytes)?;
                    }
                    DrawingMutation::ReplaceLayerFill(value) => {
                        if let Some(source) = value.fill.as_ref() {
                            if self.fill_clone.is_none() {
                                *self.fill_clone = Some(DrawingFillCloneAuthority::new());
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.fill_clone.as_mut().expect("Drawing fill clone remains retained").step(source, cx)? {
                                return Ok(false);
                            }
                            let cloned = self.fill_clone.as_ref().and_then(|clone| clone.value.as_ref()).ok_or("drawing-store.fill-clone-work-missing")?;
                            let totals = drawing_fill_clone_work_totals(cloned)?;
                            self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                        }
                    }
                    DrawingMutation::ReplaceLayerStroke(value) => {
                        if let Some(source) = value.stroke.as_ref() {
                            if self.stroke_clone.is_none() {
                                *self.stroke_clone = Some(DrawingStrokeCloneAuthority::new());
                                cx.consume_fuel(1);
                                return Ok(false);
                            }
                            if !self.stroke_clone.as_mut().expect("Drawing stroke clone remains retained").step(source, cx)? {
                                return Ok(false);
                            }
                            let cloned = self.stroke_clone.as_ref().and_then(|clone| clone.value.as_ref()).ok_or("drawing-store.stroke-clone-work-missing")?;
                            let totals = drawing_stroke_clone_work_totals(cloned)?;
                            self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                        }
                    }
                    _ => {}
                }
                self.phase = if self.clone_work.is_some() { DrawingMutationCandidatePhase::PlanOwnedValue } else { DrawingMutationCandidatePhase::BindOverlay };
                Ok(false)
            }
            DrawingMutationCandidatePhase::PlanOwnedValue => {
                let pending = self.pending_layer.as_ref().ok_or("drawing-store.mutation-clone-work-owner-missing")?;
                let work = self.clone_work.as_mut().ok_or("drawing-store.mutation-clone-work-missing")?;
                if !work.step(pending, cx)? {
                    return Ok(false);
                }
                let totals = work.totals().ok_or("drawing-store.mutation-clone-work-false-terminal")?;
                self.workset.as_mut().ok_or("drawing-store.mutation-workset-missing")?.admit_clone(totals.items, totals.bytes)?;
                self.preflight_digest = None;
                self.phase = DrawingMutationCandidatePhase::BindOverlay;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Apply => {
                match mutation {
                    DrawingMutation::CreateLayer(value) => {
                        let parent = self.secondary;
                        let index = match value.index {
                            Some(index) => index,
                            None => DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len()),
                        };
                        self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawingMutation::DuplicateLayer(_) => {
                        let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), None, Some(address.index() + 1), DrawingContainerRebuildRole::Destination)?;
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                        return Ok(false);
                    }
                    DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                        let address = self.primary.ok_or("drawing-store.mutation-primary-missing")?;
                        self.start_rebuild(source, address.parent(), Some(address.index()), None, DrawingContainerRebuildRole::Source)?;
                        self.source_undo = Some(DrawingContainerSourceUndo { parent: address.parent(), index: address.index() });
                        self.phase = DrawingMutationCandidatePhase::RebuildSource;
                        return Ok(false);
                    }
                    DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_) => {
                        self.apply_selection_transform(source, mutation)?;
                        self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                        self.phase = DrawingMutationCandidatePhase::Complete;
                        cx.consume_fuel(1);
                        return Ok(false);
                    }
                    _ => {}
                }
                let address = self.primary;
                match mutation {
                    DrawingMutation::SetLayerVisible(value) => {
                        crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).visible = value.visible;
                    }
                    DrawingMutation::SetLayerLocked(value) => {
                        crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).locked = value.locked;
                    }
                    DrawingMutation::SetGroupIsolation(value) => {
                        let DrawingLayerNode::Group(group)=DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {return Err("drawing-store.mutation-isolation-target-not-group");};
                        group.isolation=value.isolation;
                    }
                    DrawingMutation::SetLayerFillRule(value) => {
                        crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source,address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill_rule=value.fill_rule;
                    }
                    DrawingMutation::SetLayerOpacity(value) if value.opacity.is_finite() => {
                        crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).opacity = value.opacity;
                    }
                    DrawingMutation::SetLayerOpacity(_) => return Err("drawing-store.mutation-opacity-invalid"),
                    DrawingMutation::SetLayerBlendMode(value) => {
                        if !crate::DRAWING_BLEND_MODES.iter().any(|allowed| value.blend_mode.eq_str(allowed)) { return Err("drawing-store.mutation-blend-mode-invalid"); }
                        self.adopt_native_text(&mut crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).blend_mode)?;
                    }
                    DrawingMutation::UpdateText(value) => {
                        if !value.size.is_finite() || value.size <= 0.0 { return Err("drawing-store.text-size-invalid"); }
                        let DrawingLayerNode::Text(text) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else { return Err("drawing-store.text-target-kind"); };
                        self.adopt_native_text(&mut text.content)?;
                        text.size = value.size;
                    }
                    DrawingMutation::RenameLayer(_) => {
                        self.adopt_native_text(&mut crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).name)?;
                    }
                    DrawingMutation::UpdateLayerTransform(value)
                        if [value.transform.x, value.transform.y, value.transform.scale_x, value.transform.scale_y, value.transform.rotation, value.transform.shear].iter().all(|field| field.is_finite()) =>
                    {
                        crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).transform =
                            crate::DrawingTransform { x: value.transform.x, y: value.transform.y, scale_x: value.transform.scale_x, scale_y: value.transform.scale_y, rotation: value.transform.rotation, shear: value.transform.shear };
                    }
                    DrawingMutation::UpdateLayerTransform(_) => return Err("drawing-store.mutation-transform-invalid"),
                    DrawingMutation::UpdatePathGeometry(_) => {
                        let DrawingLayerNode::Path(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else { return Err("drawing-store.path-target-kind"); };
                        let replacement = self.segments_clone.as_mut().ok_or("drawing-store.path-clone-missing")?.take().ok_or("drawing-store.path-false-terminal")?;
                        let old = std::mem::replace(&mut target.segments, replacement);
                        *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Segments(old))));
                        drop(self.segments_clone.take());
                    }
                    DrawingMutation::ReplaceLayerFill(value) => {
                        let replacement = match value.fill.as_ref() {
                            Some(_) => Some(self.fill_clone.as_mut().ok_or("drawing-store.fill-clone-missing")?.take().ok_or("drawing-store.fill-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.fill,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Fill(old))));
                        }
                        if let Some(mut clone) = self.fill_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawingMutation::ReplaceLayerStroke(value) => {
                        let replacement = match value.stroke.as_ref() {
                            Some(_) => Some(self.stroke_clone.as_mut().ok_or("drawing-store.stroke-clone-missing")?.take().ok_or("drawing-store.stroke-false-terminal")?),
                            None => None,
                        };
                        let old = std::mem::replace(
                            &mut crate::schema::layer_base_mut(DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")?).attributes.stroke,
                            replacement,
                        );
                        if let Some(old) = old {
                            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Stroke(old))));
                        }
                        if let Some(mut clone) = self.stroke_clone.take() {
                            clone.terminal = true;
                            drop(clone);
                        }
                    }
                    DrawingMutation::SetLayerBooleanOperation(value) => {
                        let DrawingLayerNode::Boolean(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                            return Err("drawing-store.mutation-boolean-target");
                        };
                        self.adopt_native_text(&mut target.operation)?;
                    }
                    DrawingMutation::UpdateLayerTraceParams(value) if value.params.threshold.is_finite() && value.params.simplify_epsilon.is_finite() => {
                        let DrawingLayerNode::Trace(target) = DrawingLayerLocator::node_at_mut(source, address.ok_or("drawing-store.mutation-primary-missing")?).ok_or("drawing-store.mutation-target-lost")? else {
                            return Err("drawing-store.mutation-trace-target");
                        };
                        target.params = crate::DrawingTraceParams { threshold: value.params.threshold, simplify_epsilon: value.params.simplify_epsilon };
                    }
                    DrawingMutation::UpdateLayerTraceParams(_) => return Err("drawing-store.mutation-trace-invalid"),
                    DrawingMutation::CreateLayer(_) | DrawingMutation::DuplicateLayer(_) | DrawingMutation::DeleteLayer(_) | DrawingMutation::ReorderLayer(_) => {
                        unreachable!("structural Drawing mutations start retained rebuild before scalar mutation")
                    }
                    DrawingMutation::DragLayers(_) | DrawingMutation::RotateLayers(_) | DrawingMutation::ScaleLayers(_) | DrawingMutation::DragPathPoints(_) => {
                        unreachable!("selection transforms apply their resolved patches before scalar mutation")
                    }
                }
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.phase = DrawingMutationCandidatePhase::Complete;
                cx.consume_fuel(1);
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildSource => {
                if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent();
                let removed = self.finish_rebuild(source, parent)?.ok_or("drawing-store.mutation-removal-missing")?;
                *self.pending_layer = Some(removed);
                if matches!(mutation, DrawingMutation::DeleteLayer(_)) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                    self.source_undo = None;
                    self.phase = DrawingMutationCandidatePhase::Complete;
                } else {
                    self.secondary = None;
                    if Self::parent(mutation).is_some() {
                        self.locator = Some(DrawingLayerLocator::new());
                        self.phase = DrawingMutationCandidatePhase::LocateDestination;
                    } else {
                        self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                    }
                }
                Ok(false)
            }
            DrawingMutationCandidatePhase::LocateDestination => {
                let locator = self.locator.as_mut().ok_or("drawing-store.mutation-destination-locator")?;
                if !locator.step(source, Self::parent(mutation).ok_or("drawing-store.mutation-parent-missing")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found();
                self.locator = None;
                let Some(address) = self.secondary else { return Err("drawing-store.mutation-parent-not-found") };
                if !matches!(DrawingLayerLocator::node_at(source, address), Some(DrawingLayerNode::Group(_))) {
                    return Err("drawing-store.mutation-parent-not-group");
                }
                self.phase = DrawingMutationCandidatePhase::RebuildDestination;
                Ok(false)
            }
            DrawingMutationCandidatePhase::RebuildDestination => {
                if self.rebuild.is_none() {
                    let parent = match mutation {
                        DrawingMutation::CreateLayer(_) => self.secondary,
                        DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                        DrawingMutation::ReorderLayer(_) => self.secondary,
                        _ => return Err("drawing-store.mutation-destination-variant"),
                    };
                    let index = match mutation {
                        DrawingMutation::CreateLayer(value) => value.index.unwrap_or_else(|| DrawingLayerLocator::container_mut(source, parent).map_or(0, |values| values.len())),
                        DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.index() + 1,
                        DrawingMutation::ReorderLayer(value) => value.index,
                        _ => 0,
                    };
                    self.start_rebuild(source, parent, None, Some(index), DrawingContainerRebuildRole::Destination)?;
                }
                if !self.rebuild.as_mut().ok_or("drawing-store.mutation-rebuild-missing")?.step(cx)? {
                    return Ok(false);
                }
                let parent = match mutation {
                    DrawingMutation::CreateLayer(_) | DrawingMutation::ReorderLayer(_) => self.secondary,
                    DrawingMutation::DuplicateLayer(_) => self.primary.ok_or("drawing-store.mutation-primary-missing")?.parent(),
                    _ => None,
                };
                if self.finish_rebuild(source, parent)?.is_some() {
                    return Err("drawing-store.mutation-unexpected-removal");
                }
                self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                self.source_undo = None;
                self.phase = DrawingMutationCandidatePhase::Complete;
                Ok(false)
            }
            DrawingMutationCandidatePhase::Complete => {
                if let Some(value) = self.pending_layer.take() {
                    *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Layer(value))));
                    return Ok(false);
                }
                if let Some(retirement) = self.retirement.as_mut() {
                    return match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES).map_err(|_| "drawing-store.mutation-retirement")? {
                        store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                            drop(self.retirement.take());
                            Ok(false)
                        }
                        store::SnapshotRetirementStep::Complete => self.fail("drawing-store.mutation-retirement-false-terminal"),
                        _ => Ok(false),
                    };
                }
                if self.return_arena_owner()? != Some(true) {
                    return Ok(false);
                }
                if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
                    self.overlay.as_mut().ok_or("drawing-store.mutation-overlay-missing")?.commit(source)?;
                }
                self.terminal = true;
                Ok(true)
            }
            DrawingMutationCandidatePhase::Retire | DrawingMutationCandidatePhase::Fault => Err(self.fault.unwrap_or("drawing-store.mutation-candidate-fault")),
        }
    }

    fn take(&mut self) -> Option<()> {
        if !self.terminal {
            return None;
        }
        self.preflight_source = None;
        self.preflight_digest = None;
        self.workset = None;
        self.clone_work = None;
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            return None;
        }
        self.overlay = None;
        Some(())
    }

    fn pump_rebuild_close(&mut self, source: &mut DrawingSnapshot) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        let role = self.rebuild_role.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rebuild role missing"))?;
        let target = self.rebuild_target.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rebuild target missing"))?;
        let rebuild = self.rebuild.as_mut().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rebuild missing"))?;
        let ready = if self.rebuild_close_phase > 0 {
            true
        } else if role == DrawingContainerRebuildRole::CloseSourceUndo {
            rebuild.close_forward_step().map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?
        } else {
            rebuild.rollback_step().map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?
        };
        if !ready {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        match self.rebuild_close_phase {
            0 => {
                let container = DrawingLayerLocator::container_mut(source, target).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback container missing"))?;
                if !container.is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback destination was not empty"));
                }
                *container = rebuild.source.take().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback source missing"))?;
            }
            1 => {
                if rebuild.pending.is_some() && self.pending_layer.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback pending owner collision"));
                }
                *self.pending_layer = rebuild.pending.take();
            }
            2 => {
                if self.container_reverse.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback reverse owner collision"));
                }
                *self.container_reverse = Some(rebuild.reverse.take().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback reverse owner missing"))?);
            }
            3 => {
                if self.container_output.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback output owner collision"));
                }
                *self.container_output = Some(rebuild.output.take().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback output owner missing"))?);
            }
            4 => {
                if rebuild.removed.is_some() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback retained an unexpected removed owner"));
                }
                rebuild.finish_handoff().map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?;
                drop(self.rebuild.take());
                self.rebuild_target = None;
                self.rebuild_role = None;
                self.rebuild_close_phase = 0;
                if role != DrawingContainerRebuildRole::Destination {
                    self.source_undo = None;
                }
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation rollback handoff phase invalid")),
        }
        self.rebuild_close_phase += 1;
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn close_step(&mut self, mut source: Option<&mut DrawingSnapshot>, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        self.clone_text_work.close_step(1);
        if let Some(preflight) = self.preflight_mutation.as_mut() {
            return match preflight.close_step(maximum_bytes) {
                store::SnapshotRetirementStep::Complete if preflight.terminal_is_empty() => {
                    drop(self.preflight_mutation.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation preflight reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation candidate retirement reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(rewrite) = self.duplicate_rewrite.as_mut() {
            return match rewrite.close_step(maximum_bytes)? {
                store::SnapshotRetirementStep::Complete => {
                    let pages = self.overlay_pages.as_mut().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing duplicate name arena missing"))?;
                    if pages.len() >= pages.capacity() {
                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing duplicate name arena return saturated"));
                    }
                    let (id_owner, name_owner) = rewrite.take_owners().ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing duplicate rewrite reported false terminal"))?;
                    *self.duplicate_id_owner = Some(id_owner);
                    pages.push(name_owner);
                    drop(self.duplicate_rewrite.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                step => Ok(step),
            };
        }
        if self.rebuild.is_some() {
            let Some(source) = source.as_deref_mut() else { return Ok(store::SnapshotRetirementStep::Blocked) };
            return self.pump_rebuild_close(source);
        }
        if !self.overlay.as_ref().is_some_and(|overlay| overlay.committed) {
            if let Some(undo) = self.source_undo {
                let Some(source) = source else { return Ok(store::SnapshotRetirementStep::Blocked) };
                self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo).map_err(|e|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,e))?;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
        }
        self.clone_work = None;
        if let Some(layer) = self.layer_clone.as_mut() {
            return match layer.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if layer.terminal_is_empty() => {
                    drop(self.layer_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation layer clone reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(fill) = self.fill_clone.as_mut() {
            return match fill.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if fill.terminal_is_empty() => {
                    drop(self.fill_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation fill clone reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(text) = self.text_clone.as_mut() {
            return match text.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if text.terminal_is_empty() => {
                    drop(self.text_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Text clone reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(segments) = self.segments_clone.as_mut() {
            return match segments.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if segments.terminal_is_empty() => {
                    drop(self.segments_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Path clone reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(stroke) = self.stroke_clone.as_mut() {
            return match stroke.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if stroke.terminal_is_empty() => {
                    drop(self.stroke_clone.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation stroke clone reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(value) = self.pending_layer.take() {
            *self.retirement = Some(Box::new(DrawingOwnedRetirement::new(DrawingRetirementOwner::Layer(value))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.return_arena_owner() {
            Ok(Some(true)) => {}
            Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(None) => return Ok(store::SnapshotRetirementStep::Blocked),
            Err(error) => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,error)),
        }
        self.preflight_source = None;
        self.preflight_digest = None;
        self.workset = None;
        self.clone_work = None;
        self.overlay = None;
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.clone_text_work.terminal_is_empty()
            && self.preflight_source.is_none()
            && self.preflight_mutation.is_none()
            && self.preflight_digest.is_none()
            && self.workset.is_none()
            && self.arena_pool.is_none()
            && self.arena_return_phase == 4
            && self.overlay.is_none()
            && self.layer_clone.is_none()
            && self.clone_work.is_none()
            && self.fill_clone.is_none()
            && self.stroke_clone.is_none()
            && self.segments_clone.is_none()
            && self.text_clone.is_none()
            && self.duplicate_rewrite.is_none()
            && self.duplicate_id_owner.is_none()
            && self.rebuild.is_none()
            && self.rebuild_target.is_none()
            && self.rebuild_role.is_none()
            && self.rebuild_close_phase == 0
            && self.source_undo.is_none()
            && self.container_reverse.is_none()
            && self.container_output.is_none()
            && self.overlay_pages.is_none()
            && self.pending_layer.is_none()
            && self.retirement.is_none()
    }
}

impl Drop for DrawingMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing mutation candidate reached Drop before atomic handoff or cursor retirement");
    }
}

pub fn drawing_document_store_owners() -> store::DocumentStoreOwners<DrawingSnapshot, DrawingMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(DrawingSnapshotRetirementFactory),
        std::sync::Arc::new(DrawingSnapshotRetirementFactory),
        std::sync::Arc::new(DrawingMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<DrawingSnapshot, DrawingMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingStoreInitializationPhase {
    InitializeArena,
    BindGenesis,
    ValidateEnvelope,
    ValidateEditId { edit: usize },
    ValidateEditMeta { edit: usize, meta: usize },
    ValidateEdit { index: usize },
    StartInitialClone,
    CloneInitialSnapshot,
    TakeInitialSnapshot,
    CloseInitialSnapshotClone,
    MoveInitialOwner,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    PrepareApplied { position: usize, edit: usize, field: u8 },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    PrepareRedo { position: usize, edit: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct DrawingStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    arena_bootstrap_job: DrawingMutationArenaBootstrapJob,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<DrawingSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>>>,
    active: std::mem::ManuallyDrop<Option<DrawingOwnedRetirement>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    owner_catalog: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationOwnerCatalog>>,
    mutation_candidate: std::mem::ManuallyDrop<Option<DrawingMutationCandidateAuthority>>,
    prepared_history_id: std::mem::ManuallyDrop<Option<String>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    /// 🧬️ The paged clone of `envelope.vcs.genesis.snapshot()` that becomes the runtime's live fold —
    /// the envelope keeps its own initial snapshot, which `print_document_pack` / the cold from-scratch
    /// fold read back (moving it out left every loaded document's `.pack` empty — ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17).
    initial_clone: std::mem::ManuallyDrop<Option<DrawingSnapshot>>,
    initial_snapshot_clone: std::mem::ManuallyDrop<Option<<DrawingSnapshot as RetainedClone>::Cursor>>,
    initial_clone_authority: RetainedCloneBorrowAuthority,
    initial_clone_turn: usize,
    phase: DrawingStoreInitializationPhase,
    resume_phase: Option<DrawingStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl DrawingStoreInitializationAuthority {
    fn new(
        envelope: store::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
        owner_catalog: Result<store::ArtifactStoreInitializationOwnerCatalog, &'static str>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Self {
        let bootstrap_job = DrawingMutationArenaBootstrapJob::new(operation, generation);
        let (owner_catalog, arena_bootstrap_job, phase, fault) = match (owner_catalog, bootstrap_job) {
            (Ok(owner_catalog), Ok(arena_bootstrap_job)) => (Some(owner_catalog), arena_bootstrap_job, DrawingStoreInitializationPhase::InitializeArena, None),
            (Err(error), Ok(mut arena_bootstrap_job)) => {
                arena_bootstrap_job.terminal = true;
                (None, arena_bootstrap_job, DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec()))
            }
            (_, Err(error)) => (None, DrawingMutationArenaBootstrapJob::inactive(operation, generation), DrawingStoreInitializationPhase::RetireFault, Some(error.as_bytes().to_vec())),
        };
        Self {
            actor,
            operation,
            generation,
            arena_bootstrap_job,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            owner_catalog: std::mem::ManuallyDrop::new(owner_catalog),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            prepared_history_id: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            initial_clone: std::mem::ManuallyDrop::new(None),
            initial_snapshot_clone: std::mem::ManuallyDrop::new(None),
            initial_clone_authority: RetainedCloneBorrowAuthority::new(()),
            initial_clone_turn: 0,
            phase,
            resume_phase: None,
            cancel_requested: false,
            fault,
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = DrawingStoreInitializationPhase::RetireFault;
    }

    fn catalog_close_is_next(&self) -> bool {
        self.active.is_none() && self.prepared_history_id.is_none() && self.initial_snapshot_clone.is_none() && self.initial_clone.is_none() && self.mutation_candidate.is_none() && self.runtime.is_none() && self.owner_catalog.is_some()
    }

    fn pump_active(&mut self) -> Result<bool, semio_framework_value::ValueError> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, DRAWING_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= DRAWING_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing store initializer retirement exceeded its exact grant")),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing store initializer retirement reported a false terminal")),
        }
    }

    fn pump_terminal_retirement(&mut self, maximum_bytes: usize) -> Result<bool, semio_framework_value::ValueError> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(value) = self.prepared_history_id.take() {
            *self.active = Some(DrawingOwnedRetirement::new(DrawingRetirementOwner::HistoryId(value)));
            return Ok(false);
        }
        if let Some(clone) = self.initial_snapshot_clone.as_mut() {
            clone.begin_close();
            let grant = initial_snapshot_clone_grant(self.initial_clone_turn);
            self.initial_clone_turn = self.initial_clone_turn.wrapping_add(1);
            let step = clone.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) && clone.terminal_is_empty() { drop(self.initial_snapshot_clone.take()); }
            return Ok(false);
        }
        if let Some(value) = self.initial_clone.take() {
            *self.active = Some(DrawingOwnedRetirement::new(DrawingRetirementOwner::Snapshot(value)));
            return Ok(false);
        }
        if self.mutation_candidate.is_some() {
            let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut);
            let candidate = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained during close");
            return match candidate.close_step(current, DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if candidate.terminal_is_empty() => {
                    drop(self.mutation_candidate.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing mutation candidate reported a false terminal")),
                _ => Ok(false),
            };
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&DrawingSnapshotRetirementFactory, 1, DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing initialization runtime reported a false terminal")),
                _ => return Ok(false),
            }
        }
        if self.owner_catalog.is_some() {
            close_initialization_catalog(&mut self.owner_catalog, 1, maximum_bytes)?;
            return Ok(false);
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(drawing_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, DRAWING_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Drawing initialization envelope retirement reported a false terminal")),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.arena_bootstrap_job.terminal
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.owner_catalog.is_none()
            && self.mutation_candidate.is_none()
            && self.prepared_history_id.is_none()
            && self.initial_clone.is_none()
            && self.initial_snapshot_clone.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.arena_bootstrap_job.terminal = true;
            self.fail(b"drawing-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, DrawingStoreInitializationPhase::InitializeArena | DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::Cancelled) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_message().into_bytes());
            self.phase = DrawingStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if !matches!(self.phase, DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault | DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault | DrawingStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(1, DRAWING_OWNED_FIELD_BYTES) {
                    Ok(store::SnapshotRetirementStep::Complete) => {}
                    Ok(_) => { cx.consume_fuel(1); return semio_framework_job::StepOutcome::Yield; }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            DrawingStoreInitializationPhase::InitializeArena => {
                match self.arena_bootstrap_job.step(cx) {
                    DrawingMutationArenaBootstrapStep::Ready => self.phase = DrawingStoreInitializationPhase::ValidateEnvelope,
                    DrawingMutationArenaBootstrapStep::Pending { .. } | DrawingMutationArenaBootstrapStep::Blocked => {}
                    DrawingMutationArenaBootstrapStep::Cancelled => self.phase = DrawingStoreInitializationPhase::RetireCancelled,
                    DrawingMutationArenaBootstrapStep::Fault(error) => self.fail(error.as_bytes()),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let owner_catalog = self.owner_catalog.take().expect("pre-admitted Drawing owner catalog");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), self.actor.clone(), owner_catalog));
                self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"drawing-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::DRAWING_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-envelope-invalid");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditId { edit } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEdit { index: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if entry.id.is_empty() || entry.id.len() > DRAWING_OWNED_FIELD_BYTES || entry.actor.as_ref().is_some_and(|actor| actor.len() > DRAWING_OWNED_FIELD_BYTES) || entry.started_at.len() > DRAWING_OWNED_FIELD_BYTES {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                let entry = envelope.vcs.edits.get(edit).expect("Drawing edit remains retained during metadata validation");
                let Some(value) = entry.mutation_meta.get(meta) else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditId { edit: edit + 1 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                if value.mutation_id.as_ref().is_some_and(|id| id.0.len() > DRAWING_OWNED_FIELD_BYTES) {
                    self.fail(b"drawing-store.initializer-hostile-edit-field");
                } else {
                    self.phase = DrawingStoreInitializationPhase::ValidateEditMeta { edit, meta: meta + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Drawing envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = DrawingStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = DrawingStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"drawing-store.initializer-duplicate-edit"),
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::StartInitialClone => {
                *self.initial_snapshot_clone = Some(DrawingSnapshot::retained_clone_cursor());
                self.initial_clone_turn = 0;
                self.phase = DrawingStoreInitializationPhase::CloneInitialSnapshot;
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::CloneInitialSnapshot => {
                let grant = initial_snapshot_clone_grant(self.initial_clone_turn);
                self.initial_clone_turn = self.initial_clone_turn.wrapping_add(1);
                let source = self.envelope.as_ref().expect("retained Drawing clone genesis").vcs.genesis.snapshot();
                let clone = self.initial_snapshot_clone.as_mut().expect("retained native Drawing snapshot clone");
                match clone.advance(self.initial_clone_authority.borrow(source), grant) {
                    Ok(step) => {
                        let progress = step.progress();
                        cx.consume_fuel(progress.copied_items.max(1) as u64);
                        if matches!(step, RetainedCloneStep::Complete(_)) { self.phase = DrawingStoreInitializationPhase::TakeInitialSnapshot; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::TakeInitialSnapshot => {
                let clone = self.initial_snapshot_clone.as_mut().expect("completed Drawing snapshot clone");
                *self.initial_clone = clone.take();
                clone.begin_close();
                if self.initial_clone.is_some() { self.phase = DrawingStoreInitializationPhase::CloseInitialSnapshotClone; }
                else { self.fail(b"drawing-store.initializer-clone-false-terminal"); }
                cx.consume_fuel(size_of::<DrawingSnapshot>().max(1) as u64);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::CloseInitialSnapshotClone => {
                let grant = initial_snapshot_clone_grant(self.initial_clone_turn);
                self.initial_clone_turn = self.initial_clone_turn.wrapping_add(1);
                let clone = self.initial_snapshot_clone.as_mut().expect("closing native Drawing snapshot clone");
                match clone.close_granted(grant) {
                    Ok(step) => {
                        cx.consume_fuel(step.progress().copied_items.max(1) as u64);
                        if matches!(step, RetainedCloneStep::Complete(_)) && clone.terminal_is_empty() { drop(self.initial_snapshot_clone.take());self.phase = DrawingStoreInitializationPhase::MoveInitialOwner; }
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes());self.phase = DrawingStoreInitializationPhase::RetireFault; }
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::MoveInitialOwner => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained during initial owner move");
                let initial = self.initial_clone.take().expect("Drawing initial clone remains retained until the runtime adopts it");
                match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(DrawingSnapshotRetirementFactory)) {
                    Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                    Err(initial) => {
                        *self.active = Some(DrawingOwnedRetirement::new(DrawingRetirementOwner::Snapshot(initial)));
                        self.fail(b"initializer-owned-workspace-adoption");
                    }
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = DrawingStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = DrawingStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = DrawingStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = DrawingStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Drawing runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = DrawingStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = DrawingStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = DrawingStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Drawing runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"drawing-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"drawing-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    
                    self.phase = DrawingStoreInitializationPhase::StartInitialClone;
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                if self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).is_none_or(|entry| mutation >= entry.forwards.len()) {
                    self.phase = DrawingStoreInitializationPhase::PrepareApplied { position, edit, field: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                }
                let withdrawn = {
                    let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its forwards fold");
                    envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).is_none_or(|effective| effective.operation().is_none())
                };
                if withdrawn {
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                if self.mutation_candidate.is_none() {
                    match DrawingMutationCandidateAuthority::try_new(self.operation, self.generation) {
                        Ok(candidate) => *self.mutation_candidate = Some(candidate),
                        Err(DrawingMutationArenaBorrowError::NotReady | DrawingMutationArenaBorrowError::Contended) => return semio_framework_job::StepOutcome::Yield,
                        Err(error) => {
                            self.fail(error.as_str().as_bytes());
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    }
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained while its forwards fold");
                let effective = envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).expect("Drawing applied forward remains retained");
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Drawing runtime current snapshot remains retained");
                let stepped = self.mutation_candidate.as_mut().expect("Drawing mutation candidate remains retained").step(current, effective.operation().expect("Drawing effective forward was checked"), cx);
                drop(effective);
                let candidate_complete = match stepped {
                    Ok(complete) => complete,
                    Err(error) => {
                        self.fail(error.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if candidate_complete {
                    self.mutation_candidate.as_mut().expect("Drawing completed mutation candidate remains retained").take().expect("Drawing mutation overlay terminal commit witness remains exact");
                    drop(self.mutation_candidate.take());
                    self.phase = DrawingStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::PrepareApplied { position, edit, field } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                match field {
                    0 => {
                        *self.prepared_history_id = Some(clone_drawing_string(&entry.id).expect("validated Drawing applied id remains admitted"));
                        self.phase = DrawingStoreInitializationPhase::CommitApplied { position, edit };
                        cx.consume_fuel(entry.id.len().max(1) as u64);
                    }
                    _ => self.fail(b"drawing-store.initializer-applied-preparation"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::CommitApplied { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing applied id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained");
                if let Err(error) = runtime.push_applied_admitted(id, entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = DrawingStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = DrawingStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Drawing envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"drawing-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    self.phase = DrawingStoreInitializationPhase::PrepareRedo { position, edit: scan };
                } else {
                    self.fail(b"drawing-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::PrepareRedo { position, edit } => {
                let id = &self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained").id;
                *self.prepared_history_id = Some(clone_drawing_string(id).expect("validated Drawing redo id remains admitted"));
                self.phase = DrawingStoreInitializationPhase::CommitRedo { position, edit };
                cx.consume_fuel(id.len().max(1) as u64);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.prepared_history_id.take().expect("Drawing redo id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Drawing runtime remains retained").push_redo_admitted(id, entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = DrawingStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = DrawingStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            DrawingStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"drawing-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Drawing envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Drawing runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, drawing_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = DrawingStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            DrawingStoreInitializationPhase::RetireCancelled | DrawingStoreInitializationPhase::RetireFault => match (if self.catalog_close_is_next() { next_initialization_catalog_close_byte_demand(&self.owner_catalog) } else { Ok(DRAWING_OWNED_FIELD_BYTES) }).and_then(|bytes| self.pump_terminal_retirement(bytes)) {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == DrawingStoreInitializationPhase::RetireCancelled {
                        self.phase = DrawingStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = DrawingStoreInitializationPhase::Fault;
                        let source = self.fault.take().unwrap_or_else(|| b"drawing-store.initializer-fault".to_vec());
                        let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &source).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            DrawingStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            DrawingStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            DrawingStoreInitializationPhase::Fault => {
                let source = self.fault.as_deref().unwrap_or(b"drawing-store.initializer-fault");
                let detail = cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, source).unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, DrawingStoreInitializationPhase::Cancelled | DrawingStoreInitializationPhase::Fault) {
            self.phase = DrawingStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if self.catalog_close_is_next() {
            return close_initialization_catalog(&mut self.owner_catalog, maximum_items, maximum_bytes).map(|step| match step {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Blocked => semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 },
                store::SnapshotRetirementStep::Complete => semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 },
            }).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-catalog-close"), error.into_message()));
        }
        if maximum_items == 0 || maximum_bytes < DRAWING_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement(maximum_bytes) {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                drop(self.mutation_candidate.take());
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("Drawing initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<DrawingSnapshot, DrawingMutation>> {
        if self.phase != DrawingStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.mutation_candidate.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for DrawingStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner() || std::thread::panicking(), "Drawing store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn drawing_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<DrawingSnapshot, DrawingMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<DrawingSnapshot, DrawingMutation> {
    let owner_catalog = store::ArtifactStoreInitializationOwnerCatalog::try_new();
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(DrawingStoreInitializationAuthority::new(envelope, owner_catalog, operation, generation, actor)))
}
//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️RetainedMutationAuthorityTests
#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mutation-authority/🦀️.rs"]
mod retained_mutation_authority_tests;
//#endregion 🧪️RetainedMutationAuthorityTests

fn initial_snapshot_clone_grant(turn: usize) -> RetainedCloneGrant {
    match turn % 3 {
        0 => RetainedCloneGrant::one_capacity_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
        1 => RetainedCloneGrant::one_payload_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
        _ => RetainedCloneGrant::one_release_turn(DRAWING_OWNED_FIELD_BYTES, usize::MAX),
    }
}
