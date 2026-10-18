//! 📄️ Decode page, state and authority: the decode limits, the decode page and record, the decode state and the envelope decode authority with its interactive job and drop. See [`super`].

use super::*;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES: usize = 262_144;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES / ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES * 4;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeDecodePage {
    inner: OwnedSchemaDecodePage,
}

/// 📚️ Seals exact caller-pre-admitted pages into the sole 12-field envelope cursor.
pub fn artifact_envelope_decode_record(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: OwnedSchemaDecodePages) -> Result<OwnedSchemaRecordCursor, OwnedSchemaDecodePages> {
    let tokens = OwnedSchemaTokenCursor::try_new(operation, generation, pages)?;
    OwnedSchemaRecordCursor::try_new(artifact_envelope_owned_schema(), tokens).map_err(|(tokens, _)| tokens.into_pages())
}

impl ArtifactEnvelopeDecodePage {
    /// 📄️ Creates one definitionally shallow page from an already bounded producer buffer.
    pub fn try_from_array(bytes: [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES], len: usize) -> Result<Self, [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]> {
        Ok(Self { inner: OwnedSchemaDecodePage::try_from_array(bytes, len)? })
    }

    /// 📄️ Constructs the fixed page inside a producer callback whose extent was already accepted.
    pub fn from_preflighted_array(bytes: [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES], len: usize) -> Self {
        Self { inner: OwnedSchemaDecodePage::from_preflighted_array(bytes, len) }
    }

    /// 📥️ Moves this exact page into pre-reserved schema storage or returns it untouched.
    pub fn admit_into(self, pages: &mut OwnedSchemaDecodePages) -> Result<(), (OwnedSchemaDecodeAdmissionFault, Self)> {
        pages.admit_page(self.inner).map_err(|(fault, inner)| (fault, Self { inner }))
    }

    /// 📨️ Commits this page after the same mutable ingress authority ran its producer preflight.
    pub fn admit_preflighted_into(self, pages: &mut OwnedSchemaDecodePages) {
        pages.admit_preflighted_page(self.inner);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeDecodeState {
    Fields,
    ReleaseSuccess,
    ReleaseCancelled,
    ReleaseFault(OwnedSchemaDecodeDiagnostic),
    Transferred,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🧬️ Persistent fixed-page envelope decode job; terminal outcomes are withheld until every nested owner is empty.
pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation> {
    pub record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>,
    pub fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>,
    pub field_registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub field_ticket: ArtifactEnvelopeFieldDecoderTicket,
    pub(super) field_returned: bool,
    pub(super) maximum_field_close_byte_demand: usize,
    pub(super) maximum_retained_field_close_bytes: usize,
    pub released_field_bytes: usize,
    pending_field: Option<(u16, OwnedSchemaToken, bool)>,
    pub state: ArtifactEnvelopeDecodeState,
    pub fault: semio_framework_job::RetainedPayloadBuilder,
    fault_cursor: usize,
}

impl<P, Mutation> ArtifactEnvelopeDecodeAuthority<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// 🏗️ Admits the field decoder before binding the shallow job lease and sealed record cursor.
    pub fn try_new(
        record: OwnedSchemaRecordCursor,
        field_registry: &Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
        fields: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>,
    ) -> Result<Self, (OwnedSchemaRecordCursor, ArtifactEnvelopeFieldDecoderRegistryFault, Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>)> {
        let maximum_field_close_byte_demand = fields.maximum_close_byte_demand();
        let maximum_retained_field_close_bytes = fields.maximum_retained_close_bytes();
        if maximum_field_close_byte_demand > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_retained_field_close_bytes > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES
            || maximum_field_close_byte_demand > maximum_retained_field_close_bytes
        {
            return Err((record, ArtifactEnvelopeFieldDecoderRegistryFault::CloseByteCapacity, fields));
        }
        let fields = match field_registry.try_admit(fields) {
            Ok(fields) => fields,
            Err((fault, fields)) => return Err((record, fault, fields)),
        };
        let field_ticket = fields.ticket();
        Ok(Self {
            record: std::mem::ManuallyDrop::new(Some(record)),
            fields: std::mem::ManuallyDrop::new(Some(fields)),
            field_registry: Arc::clone(field_registry),
            field_ticket,
            field_returned: false,
            maximum_field_close_byte_demand,
            maximum_retained_field_close_bytes,
            released_field_bytes: 0,
            pending_field: None,
            state: ArtifactEnvelopeDecodeState::Fields,
            fault: semio_framework_job::RetainedPayloadBuilder::new(semio_framework_job::JobPayloadStream::Fault),
            fault_cursor: 0,
        })
    }

    pub fn begin_release(&mut self, state: ArtifactEnvelopeDecodeState) {
        self.state = state;
    }

    fn record_release_fault(&mut self, diagnostic: OwnedSchemaDecodeDiagnostic) {
        if !matches!(self.state, ArtifactEnvelopeDecodeState::ReleaseFault(_)) {
            self.state = ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic);
        }
    }

    pub fn diagnostic(code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn checked_field_close_demand(&self, demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_field_close_byte_demand {
            return Err(Self::diagnostic("artifact-envelope.decode-close-demand-over-admitted-maximum"));
        }
        Ok(demand)
    }

    fn record_released_field_bytes(&mut self, released_bytes: usize) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        let released_field_bytes = self.released_field_bytes.checked_add(released_bytes).filter(|total| *total <= self.maximum_retained_field_close_bytes).ok_or_else(|| Self::diagnostic("artifact-envelope.decode-close-byte-ledger-overflow"))?;
        self.released_field_bytes = released_field_bytes;
        Ok(())
    }

    fn release_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        cx.set_stage("artifact-envelope-decode-close");
        if cx.fuel_exhausted()||cx.deadline_exceeded(){return Ok(())}
        semio_framework_job::InteractiveJob::close_step(self,cx)?;
        Ok(())
    }

    /// 🧹️ Transfers an unstarted exact decode owner into a retained close authority.
    pub fn reject(self, diagnostic: OwnedSchemaDecodeDiagnostic) -> Result<ArtifactEnvelopeDecodeRejected<P, Mutation>, Self> {
        if !matches!(self.state, ArtifactEnvelopeDecodeState::Fields) || self.record.is_none() || self.fields.is_none() || self.field_returned || self.pending_field.is_some() || !self.fault.terminal_is_empty() {
            return Err(self);
        }
        let mut source = std::mem::ManuallyDrop::new(self);
        let Some(record) = source.record.take() else { unreachable!("validated rejected record owner") };
        let Some(fields) = source.fields.take() else { unreachable!("validated rejected field owner") };
        let field_registry = Arc::clone(&source.field_registry);
        let field_ticket = source.field_ticket;
        let maximum_field_close_byte_demand = source.maximum_field_close_byte_demand;
        let maximum_retained_field_close_bytes = source.maximum_retained_field_close_bytes;
        let released_field_bytes = source.released_field_bytes;
        source.state = ArtifactEnvelopeDecodeState::Transferred;
        unsafe { std::mem::ManuallyDrop::drop(&mut source) };
        Ok(ArtifactEnvelopeDecodeRejected {
            record: std::mem::ManuallyDrop::new(Some(record)),
            fields: std::mem::ManuallyDrop::new(Some(fields)),
            field_registry,
            field_ticket,
            field_returned: false,
            maximum_field_close_byte_demand,
            maximum_retained_field_close_bytes,
            released_field_bytes,
            diagnostic,
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.fault.terminal_is_empty()
            && self.record.is_none()
            && self.fields.is_none()
            && self.field_returned
            && self.field_registry.ticket_reclaimed(self.field_ticket)
            && matches!(self.state, ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_))
    }

    /// 📏️ Observes the next close turn: the leased field decoder as a child, then one fixed page, then the terminal handoff.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if !self.fault.terminal_is_empty() {
            let mut demand = self.fault.retirement_demands().map_err(|error| Self::diagnostic("artifact-envelope.fault-close-demand").with_native(error))?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.fault-close-depth"))?;
            return Ok(demand);
        }
        if let Some(fields) = self.fields.as_ref() {
            let demand = match fields.with_owner(|owner| artifact_envelope_field_demands(owner, maximum_copy_bytes)) {
                Ok(demand) => demand?,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }),
                Err(_) => return Err(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")),
            };
            let demand = self.checked_field_close_demand(demand)?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.decode-close-depth-overflow"))?, ..demand });
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }

    pub fn released_field_bytes(&self) -> usize {
        self.released_field_bytes
    }
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<semio_framework_job::InteractiveJobCloseStep,ValueError> {
        use semio_framework_job::InteractiveJobCloseStep as Close;
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Close::Pending { progress: empty }); }
        if matches!(self.state,ArtifactEnvelopeDecodeState::Fields){self.state=ArtifactEnvelopeDecodeState::ReleaseCancelled;}
        self.pending_field=None;
        if !self.fault.terminal_is_empty() {
            let demand = match self.close_demands(grant.maximum_copy_bytes) {
                Ok(demand) => demand,
                Err(diagnostic) => return Err(artifact_envelope_diagnostic_error(diagnostic)),
            };
            if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Ok(Close::Pending { progress: empty }); }
            let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.fault.close_step_granted(child_grant) {
                Ok(step) => Ok(Close::Pending { progress: step.progress() }),
                Err(error) => Err(error),
            };
        }
        if self.fields.is_some() {
            if grant.maximum_items == 0 {
                return Ok(Close::Pending { progress: empty });
            }
            let terminal = match self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| owner.terminal_is_empty()) {
                Ok(terminal) => terminal,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty));
                }
            };
            if terminal {
                if !self.fields.as_mut().expect("closing job holds its field decoder lease").return_now() {
                    return Ok(Close::Blocked);
                }
                drop(self.fields.take());
                self.field_returned = true;
                return Ok(Close::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } });
            }
            let demand = match self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| artifact_envelope_field_demands(owner, grant.maximum_copy_bytes)) {
                Ok(Ok(demand)) => demand,
                Ok(Err(diagnostic)) => {
                    self.record_release_fault(diagnostic);
                    return Err(artifact_envelope_diagnostic_error(diagnostic));
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty));
                }
            };
            let demand = match self.checked_field_close_demand(demand) {
                Ok(demand) => demand,
                Err(diagnostic) => {
                    self.record_release_fault(diagnostic);
                    return Err(artifact_envelope_diagnostic_error(diagnostic));
                }
            };
            if grant.maximum_depth <= demand.depth {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original decoder close refused").with_retained_progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            if !artifact_retirement_grant_covers(child, demand) {
                return Ok(Close::Pending { progress: empty });
            }
            let close = self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| {
                let step = owner.close_step(child);
                (step, owner.terminal_is_empty())
            });
            return match close {
                Ok((Ok(step), terminal)) => {
                    let progress = step.progress();
                    if !progress.fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
                        self.record_release_fault(Self::diagnostic("artifact-envelope.decode-close-over-budget"));
                        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(progress));
                    }
                    if let Err(diagnostic) = self.record_released_field_bytes(progress.released_bytes) {
                        self.record_release_fault(diagnostic);
                        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(progress));
                    }
                    Ok(Close::Pending { progress })
                }
                Ok((Err(diagnostic), _)) => {
                    let released=self.record_released_field_bytes(diagnostic.retained_progress.released_bytes);
                    self.record_release_fault(diagnostic);
                    if let Err(ledger)=released{self.record_release_fault(ledger.with_progress(diagnostic.retained_progress));}
                    Err(artifact_envelope_diagnostic_error(diagnostic))
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty))
                }
            };
        }
        if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
            if self.field_returned {
                let _ = self.field_registry.reclaim_returned_ticket_now(self.field_ticket);
            }
            if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
                return Ok(Close::Blocked);
            }
        }
        if let Some(record) = self.record.as_mut() {
            let nonterminal = !record.terminal_is_empty();
            if grant.maximum_items == 0 || nonterminal && grant.maximum_copy_bytes < record.next_close_copy_byte_demand() {
                return Ok(Close::Pending { progress: empty });
            }
            if !nonterminal {
                drop(self.record.take());
                return Ok(Close::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } });
            }
            if grant.maximum_depth < 2 {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original decoder close refused").with_retained_progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            match record.close_step(child) {
                Ok(step) if step.progress().fits(child) => return Ok(Close::Pending { progress: step.progress() }),
                Ok(step) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(step.progress())),
                Err(error) => return Err(error),
            }
        }
        let before = self.state;
        self.state = match self.state {
            ArtifactEnvelopeDecodeState::ReleaseSuccess | ArtifactEnvelopeDecodeState::Complete => ArtifactEnvelopeDecodeState::Complete,
            ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic) | ArtifactEnvelopeDecodeState::Fault(diagnostic) => ArtifactEnvelopeDecodeState::Fault(diagnostic),
            _ => ArtifactEnvelopeDecodeState::Cancelled,
        };
        if ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self) { Ok(Close::Complete { progress: RetainedCloneProgress { copied_items: usize::from(before != self.state), ..empty } }) } else { Ok(Close::Blocked) }
    }

    fn advance_decode(&mut self, cx: &mut semio_framework_job::StepContext<'_>) {
        match self.state {
            ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_) => return,
            ArtifactEnvelopeDecodeState::Fields => {}
            _ => unreachable!("release states returned before decode"),
        }
        if cx.is_cancelled() {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
            return ();
        }
        cx.set_stage("artifact-envelope-decode-fields");
        let Some(record) = self.record.as_mut() else {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.missing-record-owner")));
            return ();
        };
        let Some(fields) = self.fields.as_ref() else {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.missing-field-owner")));
            return ();
        };
        if let Some((field_id, token, terminal)) = self.pending_field {
            return match fields.with_owner(|owner| owner.accept_field_token(field_id, token, terminal, record, cx)) {
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) => (),
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)) => {
                    self.pending_field = None;
                    ()
                }
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)) => {
                    self.pending_field = None;
                    ()
                }
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.field-published-record-before-outer-terminal")));
                    ()
                }
                Ok(Err(diagnostic)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                    ()
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => (),
                Err(_) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")));
                    ()
                }
            };
        }
        match record.step(cx) {
            OwnedSchemaRecordStep::Pending => (),
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal } => {
                self.pending_field = Some((field_id, token, terminal));
                ()
            }
            OwnedSchemaRecordStep::Complete => match fields.with_owner(|owner| {
                let step = owner.finish_record(cx);
                let terminal = owner.terminal_is_empty();
                (step, terminal)
            }) {
                Ok((Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete), true)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseSuccess);
                    ()
                }
                Ok((Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete), false)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.record-complete-with-live-owner")));
                    ()
                }
                Ok((Ok(_), _)) => (),
                Ok((Err(diagnostic), _)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                    ()
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => (),
                Err(_) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")));
                    ()
                }
            },
            OwnedSchemaRecordStep::Cancelled => {
                self.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
                ()
            }
            OwnedSchemaRecordStep::Fault(diagnostic) => {
                self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                ()
            }
        }
    }

}

impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let before = cx.retained_progress();
        let state = self.state;
        if matches!(self.state,ArtifactEnvelopeDecodeState::ReleaseSuccess|ArtifactEnvelopeDecodeState::ReleaseCancelled|ArtifactEnvelopeDecodeState::ReleaseFault(_)){self.release_step(cx)?;}else{self.advance_decode(cx);}
        let outcome=match self.state {
            ArtifactEnvelopeDecodeState::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            ArtifactEnvelopeDecodeState::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            ArtifactEnvelopeDecodeState::Fault(diagnostic) => {
                if !self.fault.advance_initialization(cx)? { return Ok(None); }
                if !self.fault.append_original(cx, diagnostic.code.as_bytes(), &mut self.fault_cursor)? { return Ok(None); }
                if !self.fault.seal(cx)? { return Ok(None); }
                semio_framework_job::JobOutcomeBorrow::admit_fault(cx, self.fault.published().expect("sealed original fault remains inside decoder"))
            }
            _ if before == cx.retained_progress() && state == self.state => Ok(None),
            _ => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
        };
        outcome.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Complete if self.state == ArtifactEnvelopeDecodeState::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Cancelled if self.state == ArtifactEnvelopeDecodeState::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Fault if matches!(self.state, ArtifactEnvelopeDecodeState::Fault(_)) => descriptor.fault(self.fault.published().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original decoder fault payload is not published"))?),
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoder outcome descriptor lacks its original semantic result")),
        }
    }

    fn begin_close(&mut self) {
        if matches!(self.state, ArtifactEnvelopeDecodeState::Fields) {
            self.state = ArtifactEnvelopeDecodeState::ReleaseCancelled;
        }
        self.pending_field = None;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let original=cx.retained_grant();
        let grant=RetainedCloneGrant{maximum_items:original.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)),..original};
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let step=self.close_granted(grant).and_then(|step|step.admit(grant,ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self))).map_err(|error|{cx.consume_fuel(error.retained_progress().copied_items as u64);semio_framework_job::OriginalTurnFailure::debit_original(error,cx)})?;
        cx.consume_fuel(step.progress().copied_items as u64);
        cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        Ok(step)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self)
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeDecodeAuthority<P, Mutation> {
    fn drop(&mut self) {
        if matches!(self.state, ArtifactEnvelopeDecodeState::Transferred) {
            assert!(std::thread::panicking() || (self.record.is_none() && self.fields.is_none() && self.pending_field.is_none() && !self.field_returned), "transferred artifact envelope decode source retained an owner after exact rejection handoff");
            return;
        }
        let terminal = self.fault.terminal_is_empty()
            && self.record.is_none()
            && self.fields.is_none()
            && self.field_returned
            && self.field_registry.ticket_reclaimed(self.field_ticket)
            && matches!(self.state, ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_));
        assert!(std::thread::panicking() || (terminal), "artifact envelope decode authority reached Drop before terminal publication or retained close");
    }
}
