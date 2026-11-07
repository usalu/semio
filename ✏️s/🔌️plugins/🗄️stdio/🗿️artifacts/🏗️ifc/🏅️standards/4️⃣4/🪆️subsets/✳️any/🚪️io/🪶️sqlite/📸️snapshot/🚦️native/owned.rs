//! 🫴️ Literal IFC Pack ingress keeps the original source, receiving ledger and partial graph until funded closure.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, owned_graph::GraphReceiving};
use semio_framework_dsl_record::receiving::owned::{OwnedSlot, funds, metadata, receipt};
use semio_os_kernel::{ArtifactPackReceiving, NativeSnapshotDecodeOwner, OriginalArtifactPackReceiving, OriginalArtifactSnapshotReceiving, OriginalSnapshotRecordReceiving, NativeSnapshotFailure};
use semio_framework_value::{
    RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetireOwned, RetirementDemand, ValueError, ValueRefusalKind,
    retirement::{RetireOwned as RetireOwnedTrait, RetirementCursor, RetirementStep},
};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeCloseControl, NativeDecodeControl, OriginalReceivingPrimary};
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
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep> {
        if self.closed {
            return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 };
        }
        let demand = match self.close_demands(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return RetainedWorkOutcome { result: Err(error), work: 0 },
        };
        if !funds(grant, demand) {
            return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 };
        }
        self.closing = true;
        let turn = if !self.graph.terminal_is_empty() {
            let turn = self.graph.close(grant);
            RetainedWorkOutcome { work: turn.work, result: turn.result.map(|step| step.progress()) }
        } else if !self.record.terminal_is_empty() {
            self.record.close_step(grant)
        } else {
            self.closed = true;
            RetainedWorkOutcome { result: Ok(receipt(demand)), work: 1 }
        };
        let complete = OriginalArtifactSnapshotReceiving::terminal_is_empty(self);
        RetainedWorkOutcome { work: turn.work, result: turn.result.map(|progress| if complete { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }) }
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
    fn step(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, body: usize, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        if let Err(error)=self.control(native){return RetainedWorkOutcome{result:Err(error.into()),work:0}}
        let demand=match self.demands(source,body){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
        if self.closing||self.closed||self.phase>=3||!funds(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
        if self.phase==1&&!self.record.value.as_ref().unwrap().is_ready(){return self.record.value.as_mut().unwrap().step(source,native,body,grant)}
        if self.phase==2{return self.graph.step(native,body,grant)}
        let mut work=0;
        let requested=grant;let turn=native.receive_construction_step(demand,|native,granted|{let grant=requested.restricted_by(granted);
            let result=(||{
                native.checkpoint()?;
                if !native.prepare_work(1)?{return Ok(Default::default())}
                native.commit_work(1)?;work=1;
                match self.phase {
                    0=>{let Some((record,p))=OriginalSnapshotRecordReceiving::admit(source,STDIO_IFC_DOCUMENT_SCHEMA,self.limits.clone(),native,grant)?else{return Ok(Default::default())};*self.record.value=Some(record);self.phase=1;Ok(p)},
                    1=>{let p=self.record.value.as_mut().unwrap().take_into(&mut self.graph.frame.value,grant)?;if self.graph.frame.value.is_some(){self.phase=2}Ok(p)},
                    _=>unreachable!(),
                }
            })();
            RetainedWorkOutcome{result:result.map(RetainedCloneStep::Progress),work}
        });
        RetainedWorkOutcome{result:turn.result.map(|step|step.progress()).map_err(|failure|OriginalReceivingPrimary{primary:failure.primary,quote_refusal:failure.receipt_refusal,allocation_refusal:None,progress:failure.progress}),work:turn.work}
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
    fn close_step(&mut self, native: &mut dyn NativeDecodeCloseControl, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep> {
        let result=self.control(native).and_then(|()|self.close_demands(grant.maximum_copy_bytes));
        let demand=match result{Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if !funds(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
        if let Err(error)=native.charge(demand.capacity_bytes){return RetainedWorkOutcome{result:Err(error),work:0}}
        self.retire_step(grant)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closed && self.graph.terminal_is_empty() && self.record.terminal_is_empty()
    }
}
impl ArtifactPackReceiving for IfcSnapshot {
    fn receive_pack(source:&[u8],output:&mut Option<Self>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
        owner.receive_nested::<IfcOriginalPackReceiving,Self,NativeSnapshotFailure>(output,&mut |slot,produced,_marker,_unwind_failure,original|{
            let mut work=0;
            let result=(||{
                let grant=original.remaining_grant();
                if !funds(grant,Self::original_pack_birth_demand()){return Err(NativeSnapshotFailure::from(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original IFC Pack receiving birth exceeds caller grant")))}
                work=1;
                let Some((receiver,progress))=Self::admit_original_pack(source,store::mounted_pack_rt::PackLimits::default(),original.native(),grant)?else{return Err(NativeSnapshotFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"admitted original IFC receiver omitted its header")))};
                *slot=Some(receiver);original.record_progress(progress)?;
                let turn=original.drive_cursor::<IfcOriginalPackReceiving,Self,NativeSnapshotFailure>(slot,produced,&mut |receiver,produced,native,grant|{
                    if receiver.is_ready(){
                        let demand=match receiver.take_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
                        if !funds(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
                        let result=receiver.take_into(produced,grant).map_err(Into::into);
                        RetainedWorkOutcome{result,work:1}
                    }else{let turn=receiver.step(source,native,grant.maximum_copy_bytes.min(4096),grant);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
                });
                work=work.checked_add(turn.work).expect("finite IFC receiving work fits");turn.result
            })();
            RetainedWorkOutcome{result,work}
        })
    }
}
impl RetireOwnedTrait for IfcOriginalPackReceiving {
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
        let turn = self.retire_step(grant);
        match turn.result {
            Ok(step) => RetirementStep::Progress(step.progress(), turn.work),
            Err(error) => RetirementStep::Failure(error, turn.work),
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
