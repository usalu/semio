//! 🔓️ raster owned field decode: the pack decoders, the macro-generated snapshot and mutation field decode authorities, the envelope owned-field catalog and the rejected conflict authority.

use super::*;

fn decode_raster_snapshot_pack(bytes: &[u8]) -> Result<RasterSnapshot, ()> {
    <RasterSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_raster_mutation_pack(bytes: &[u8]) -> Result<RasterMutation, ()> {
    RasterMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! raster_owned_field_close_capacity {
    (ArtifactEnvelopeSnapshotFieldAuthority) => {
        fn maximum_close_byte_demand(&self) -> usize {
            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority) => {};
}

macro_rules! raster_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
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
                store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
            }

            fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if matches!(self.state, $state::Decode(_)) {
                    return Ok(semio_framework_value::RetirementDemand { release_bytes: RASTER_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
                }
                let demand = match self.retirement.as_ref() {
                    Some(owner) => store::os_store::snapshot_read::artifact_retirement_box_demands(owner, maximum_copy_bytes),
                    None => store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value),
                };
                demand.map_err(|_| self.diagnostic(concat!("raster-envelope.", $kind, "-retirement-demand"), 0))
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
                if cx.operation() != self.operation || cx.generation() != self.generation {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-stale-authority"), token.start));
                }
                if cx.is_cancelled() {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-cancelled"), token.start));
                }
                if cx.should_yield() {
                    return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("raster-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("raster-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-cancelled"), token.start)),
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
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("raster-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            raster_owned_field_close_capacity!($authority_trait);

            fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(0).map(|demand| demand.copy_bytes)
            }

            fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(0).map(|demand| demand.release_bytes)
            }

            fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(0).map(|demand| demand.depth)
            }

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
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-retirement-depth"), 0));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: RASTER_OWNED_FIELD_BYTES, ..empty }));
                }
                let attempt = if self.retirement.is_some() {
                    store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant)
                } else if self.value.is_some() {
                    store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant)
                } else {
                    self.state = $state::Complete;
                    return Ok(RetainedCloneStep::Complete(empty));
                };
                let step = attempt.map_err(|error| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: concat!("raster-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path: self.path, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: error.retained_progress() })?;
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
                    concat!("Raster ", $kind, " decode reached Drop before publication or bounded retirement"),
                );
            }
        }
    };
}

raster_owned_field_authority!(
    RasterSnapshotDecodeState,
    RasterSnapshotDecodeAuthority,
    RasterSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_raster_snapshot_pack,
    "snapshot"
);

raster_owned_field_authority!(
    RasterMutationDecodeState,
    RasterMutationDecodeAuthority,
    RasterMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_raster_mutation_pack,
    "mutation"
);

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<RasterSnapshot, RasterMutation> for RasterEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<RasterSnapshot, RasterMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<RasterSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(RasterSnapshotRetirementFactory), std::sync::Arc::new(RasterMutationRetirementFactory), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<RasterSnapshot, RasterMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<RasterSnapshot>> {
        Box::new(RasterSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<RasterMutation>> {
        Box::new(RasterMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(RasterRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<RasterMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(RasterMutationRetirementFactory))
    }
}

pub fn raster_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<RasterSnapshot, RasterMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(RasterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(RasterSnapshotRetirementFactory), std::sync::Arc::new(RasterMutationRetirementFactory))
}

struct RasterRejectedConflictAuthority {
    pub(super) terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for RasterRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "raster-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterEnvelopeOwnedFieldCatalog;
