//! 🚫️ Decode rejections and erased retirement: the admitted and unadmitted decode rejections with their erased snapshot retirement and drop. See [`super`].

use super::*;

pub struct ArtifactEnvelopeDecodeRejected<P, Mutation> {
    pub record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>,
    pub fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>,
    pub field_registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub field_ticket: ArtifactEnvelopeFieldDecoderTicket,
    pub(super) field_returned: bool,
    pub(super) maximum_field_close_byte_demand: usize,
    pub(super) maximum_retained_field_close_bytes: usize,
    pub released_field_bytes: usize,
    pub diagnostic: OwnedSchemaDecodeDiagnostic,
}

impl<P, Mutation> ArtifactEnvelopeDecodeRejected<P, Mutation> {
    /// 📏️ Observes the leased field decoder as a child, then one fixed page; `None` is a contended lease.
    pub fn demands(&self, maximum_copy_bytes: usize) -> Result<Option<semio_framework_value::RetirementDemand>, ValueError> {
        if let Some(fields) = self.fields.as_ref() {
            let demand = match fields.with_owner(|owner| artifact_envelope_field_demands(owner, maximum_copy_bytes)) {
                Ok(demand) => demand.map_err(artifact_envelope_diagnostic_error)?,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(None),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_field_close_byte_demand {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field close demand exceeded its admitted maximum"));
            }
            return Ok(Some(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected field depth overflow"))?, ..demand }));
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(Some(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() }));
        }
        Ok(Some(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }))
    }

    fn demand_axis(&self, maximum_copy_bytes: usize, axis: impl FnOnce(semio_framework_value::RetirementDemand) -> usize) -> Result<usize, ValueError> {
        Ok(axis(self.demands(maximum_copy_bytes)?.unwrap_or(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })))
    }
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
        fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(self.demands(body)?.unwrap_or(semio_framework_value::RetirementDemand{depth:1,..Default::default()}))}

/// 🧱️ A retained decode page is a GRANULE: it is released whole or not at all, so a turn
    /// that cannot pay a full page buys nothing. The empty record husk still completes under
    /// any grant — its last page has already been paid for.
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.fields.is_some() {
            let terminal = match self.fields.as_ref().expect("rejected decode retains its field lease").with_owner(|owner| owner.terminal_is_empty()) {
                Ok(terminal) => terminal,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(RetainedCloneStep::Progress(empty)),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            if terminal {
                if !self.fields.as_mut().expect("rejected decode retains its field lease").return_now() {
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner was returned twice"));
                }
                drop(self.fields.take());
                self.field_returned = true;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let Some(demand) = self.demands(grant.maximum_copy_bytes)? else { return Ok(RetainedCloneStep::Progress(empty)) };
            if grant.maximum_depth < demand.depth {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected field owner exceeds admitted depth"));
            }
            if !artifact_retirement_grant_covers(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let (step, terminal) = match self.fields.as_ref().expect("rejected decode retains its field lease").with_owner(|owner| {
                let step = owner.close_step(child);
                (step, owner.terminal_is_empty())
            }) {
                Ok((Ok(step), terminal)) => (step, terminal),
                Ok((Err(diagnostic), _)) => return Err(artifact_envelope_diagnostic_error(diagnostic)),
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(RetainedCloneStep::Progress(empty)),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            let progress = step.progress();
            if !progress.fits(child) {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner exceeded its close grant"));
            }
            if matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner reported a false terminal"));
            }
            self.released_field_bytes = self
                .released_field_bytes
                .checked_add(progress.released_bytes)
                .filter(|total| *total <= self.maximum_retained_field_close_bytes)
                .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field close ledger exceeded its admitted aggregate"))?;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(record) = self.record.as_mut() {
            if record.terminal_is_empty() {
                drop(self.record.take());
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if grant.maximum_copy_bytes < record.next_close_copy_byte_demand() {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            if grant.maximum_depth < 2 {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected record exceeds admitted depth"));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = record.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "artifact envelope rejected record")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.record.is_none() && self.fields.is_none() && self.field_returned && self.field_registry.ticket_reclaimed(self.field_ticket)
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.demand_axis(maximum_body_bytes, |demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.depth) }
}

impl<P, Mutation> Drop for ArtifactEnvelopeDecodeRejected<P, Mutation> {
    fn drop(&mut self) {
        let _ = self.diagnostic;
        assert!(
            std::thread::panicking() || (self.record.is_none() && self.fields.is_none() && self.field_returned && self.field_registry.ticket_reclaimed(self.field_ticket)),
            "artifact envelope decode rejection reached Drop before every exact page owner was cursor-retired"
        );
    }
}

/// 🧹️ Exact close authority used when the shared field registry rejects admission
/// after the record pages and owner-supplied decoder have already been constructed.
pub struct ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    pub record: Option<OwnedSchemaRecordCursor>,
    pub fields: Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>,
    pub terminal: bool,
}

impl<P, Mutation> ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    pub fn new(record: OwnedSchemaRecordCursor, fields: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>) -> Self {
        Self { record: Some(record), fields: Some(fields), terminal: false }
    }

    /// 📏️ Observes the unadmitted decoder as a child, the separate release of its terminal shell, then one fixed page.
    pub fn demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(fields) = self.fields.as_ref() {
            if fields.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(fields.as_ref()), depth: 1, ..Default::default() }); }
            let demand = artifact_envelope_field_demands(fields.as_ref(), maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "unadmitted envelope field decoder depth overflow"))?, ..demand });
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "unadmitted envelope rejection exceeds admitted depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(fields) = self.fields.as_mut() {
            if fields.terminal_is_empty() {
                drop(self.fields.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
            let step = fields.close_step(child).map_err(artifact_envelope_diagnostic_error)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, fields.terminal_is_empty(), "unadmitted envelope field decoder")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(record) = self.record.as_mut() {
            if record.terminal_is_empty() {
                drop(self.record.take());
                self.terminal = true;
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let step = record.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "unadmitted envelope record")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.record.is_none() && self.fields.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.record.is_none() && self.fields.is_none()), "unadmitted envelope decode rejection reached Drop before every exact owner was cursor-retired");
    }
}
