//! 🔓️ Drawing arena owned-field decode: the retirement factories, the decoded field retirement, the macro-generated owned-field authorities and the envelope owned-field catalog.

use super::*;

pub const DRAWING_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// 🏭️ Framework-owned retirement issuer for decoded Drawing snapshots.
pub type DrawingSnapshotRetirementFactory = semio_framework_value::retirement::OwnedValueRetirementFactory<DrawingSnapshot>;

/// 🏭️ Framework-owned retirement issuer for decoded Drawing mutations.
pub type DrawingMutationRetirementFactory = semio_framework_value::retirement::OwnedValueRetirementFactory<DrawingMutation>;

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
