//! 🫴️ Literal IFC Pack ingress keeps the original source, receiving ledger and partial graph until funded closure.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, owned_graph::GraphReceiving};
use semio_framework_dsl_record::receiving::owned::{OwnedSlot, funds, metadata, receipt};
use semio_framework_os_kernel::{ArtifactPackReceiving, NativeSnapshotDecodeOwner, OriginalArtifactPackReceiving, OriginalArtifactSnapshotReceiving, OriginalSnapshotRecordReceiving};
use semio_framework_value::{
    RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetireOwned, RetirementDemand, ValueError, ValueRefusalKind,
    retirement::{RetirementCursor, RetirementStep},
};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeCloseControl, NativeDecodeControl};
use std::mem::size_of;
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "original IFC Pack receiving changed its source or original receiving owner")
}
/// 🧳️ Every typed parser and graph frontier stays resident in the source-owned original receiver.
pub struct IfcOriginalPackReceiving {
    source: (usize, usize),
    control: (u64, Option<u64>),
    limits: store::mounted_pack_rt::PackLimits,
    record: OwnedSlot<OriginalSnapshotRecordReceiving<Frame>>,
    graph: GraphReceiving,
    phase: u8,
    closing: bool,
    closed: bool,
}
impl IfcOriginalPackReceiving {
    #[cfg(test)]
    pub(super) fn graph_phase(&self) -> Option<bool> {
        (self.phase == 2).then(|| self.graph.is_scanning_binary())
    }
    fn source(&self, source: &[u8]) -> Result<(), ValueError> {
        if self.source != (source.as_ptr() as usize, source.len()) {
            return Err(invalid());
        }
        Ok(())
    }
    fn control(&self, native: &dyn NativeDecodeCloseControl) -> Result<(), ValueError> {
        if self.control != (native.ledger_identity(), native.retirement_recipient_identity()) {
            return Err(invalid());
        }
        Ok(())
    }
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closed {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !funds(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.closing = true;
        let progress = if !self.graph.terminal_is_empty() {
            self.graph.close(grant)?.progress()
        } else if !self.record.terminal_is_empty() {
            self.record.close_step(grant)?
        } else {
            self.closed = true;
            receipt(demand)
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}
impl OriginalArtifactPackReceiving for IfcSnapshot {
    type Receiving = IfcOriginalPackReceiving;
    fn original_pack_birth_demand() -> RetirementDemand {
        metadata::<IfcOriginalPackReceiving>()
    }
    fn admit_original_pack(source: &[u8], limits: store::mounted_pack_rt::PackLimits, native: &NativeDecodeControl<'_>, grant: RetainedCloneGrant) -> Result<Option<(Self::Receiving, RetainedCloneProgress)>, ValueError> {
        let demand = Self::original_pack_birth_demand();
        if !funds(grant, demand) {
            return Ok(None);
        }
        if source.len() as u64 > limits.max_file_len {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "original IFC Pack exceeds source extent admission"));
        }
        Ok(Some((
            IfcOriginalPackReceiving {
                source: (source.as_ptr() as usize, source.len()),
                control: (native.ledger_identity(), native.retirement_recipient_identity()),
                limits,
                record: Default::default(),
                graph: Default::default(),
                phase: 0,
                closing: false,
                closed: false,
            },
            receipt(demand),
        )))
    }
}
impl OriginalArtifactSnapshotReceiving<IfcSnapshot> for IfcOriginalPackReceiving {
    fn demands(&self, source: &[u8], body: usize) -> Result<RetirementDemand, ValueError> {
        self.source(source)?;
        if self.closing || self.closed {
            return self.close_demands(body);
        }
        match self.phase {
            0 => Ok(metadata::<OriginalSnapshotRecordReceiving<Frame>>()),
            1 => {
                let record = self.record.value.as_ref().ok_or_else(invalid)?;
                if record.is_ready() { Ok(metadata::<Frame>()) } else { record.demands(source, body) }
            }
            2 => self.graph.demands(body),
            _ => Ok(Default::default()),
        }
    }
    fn step(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        self.control(native)?;
        let demand = self.demands(source, body)?;
        if self.closing || self.closed || self.phase >= 3 || !funds(grant, demand) {
            return Ok(Default::default());
        }
        native.checkpoint()?;
        match self.phase {
            0 => {
                let Some((record, p)) = OriginalSnapshotRecordReceiving::admit(source, STDIO_IFC_DOCUMENT_SCHEMA, self.limits.clone(), native, grant)? else {
                    return Ok(Default::default());
                };
                *self.record.value = Some(record);
                self.phase = 1;
                Ok(p)
            }
            1 => {
                let record = self.record.value.as_mut().unwrap();
                if record.is_ready() {
                    let p = record.take_into(&mut self.graph.frame.value, grant)?;
                    if self.graph.frame.value.is_some() {
                        self.phase = 2;
                    }
                    Ok(p)
                } else {
                    record.step(source, native, body, grant)
                }
            }
            2 => self.graph.step(native, body, grant),
            _ => Ok(Default::default()),
        }
    }
    fn is_ready(&self) -> bool {
        !self.closing && !self.closed && self.phase == 2 && self.graph.is_ready()
    }
    fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.graph.take_demand())
    }
    fn take_into(&mut self, output: &mut Option<IfcSnapshot>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let p = self.graph.take_into(output, grant)?;
        if output.is_some() {
            self.phase = 3;
        }
        Ok(p)
    }
    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closed {
            return Ok(Default::default());
        }
        if !self.graph.terminal_is_empty() {
            return self.graph.close_demands(body);
        }
        if !self.record.terminal_is_empty() {
            return self.record.close_demands(body);
        }
        Ok(metadata::<u8>())
    }
    fn close_step(&mut self, native: &mut dyn NativeDecodeCloseControl, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.control(native)?;
        let d = self.close_demands(grant.maximum_copy_bytes)?;
        if !funds(grant, d) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        native.charge(d.capacity_bytes)?;
        self.retire_step(grant)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closed && self.graph.terminal_is_empty() && self.record.terminal_is_empty()
    }
}
impl ArtifactPackReceiving for IfcSnapshot {
    fn receive_pack(source: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        owner.receive_nested::<IfcOriginalPackReceiving, Self>(|slot, original| {
            let grant = original.remaining_grant();
            let Some((rx, p)) = Self::admit_original_pack(source, store::mounted_pack_rt::PackLimits::default(), original.native(), grant)? else {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original IFC Pack receiving birth exceeds caller grant"));
            };
            *slot = Some(rx);
            original.record_progress(p)?;
            original.drive_cursor(slot, |rx, native, grant| {
                let result = if rx.is_ready() {
                    let mut out = None;
                    rx.take_into(&mut out, grant).map(|p| (out, p))
                } else {
                    rx.step(source, native, grant.maximum_copy_bytes.min(4096), grant).map(|p| (None, p))
                };
                match result {
                    Ok((out, p)) => (Ok(out), p),
                    Err(e) => {
                        let p = e.retained_progress();
                        (Err(e), p)
                    }
                }
            })
        })
    }
}
impl RetireOwned for IfcOriginalPackReceiving {
    fn retirement(mut self) -> Box<dyn RetirementCursor> {
        self.closing = true;
        Box::new(self)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl RetirementCursor for IfcOriginalPackReceiving {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(body)
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        match self.retire_step(grant) {
            Ok(step) => RetirementStep::Progress(step.progress()),
            Err(error) => RetirementStep::Failure(error),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        OriginalArtifactSnapshotReceiving::terminal_is_empty(self)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
impl Drop for IfcOriginalPackReceiving {
    fn drop(&mut self) {
        assert!(OriginalArtifactSnapshotReceiving::terminal_is_empty(self) || std::thread::panicking(), "original IFC Pack receiving requires funded closure");
    }
}
