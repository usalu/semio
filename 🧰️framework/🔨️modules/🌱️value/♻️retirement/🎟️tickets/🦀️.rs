//! 🎟️ Original erased child closure demand and funded frame settlement.
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
pub fn factory_ticket_demands<T:ErasedSnapshotRetirement+?Sized>(ticket:&Box<T>,copy:usize)->Result<RetirementDemand,ValueError> {
    if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:size_of_val(ticket.as_ref()),depth:1,..Default::default()});}
    ticket.next_demand(copy)
}
pub fn close_factory_ticket<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
    let empty=RetainedCloneProgress::default();
    let Some(ticket)=slot.as_mut()else{return Ok(RetainedCloneStep::Complete(empty));};
    if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
    if ticket.terminal_is_empty() {
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"terminal ticket requires admitted depth"));}
        let bytes=size_of_val(ticket.as_ref());
        if grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(empty));}
        drop(slot.take());
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
    }
    let step=ticket.close_step(grant)?;
    let step=crate::retained_clone::admit_retained_clone_close(grant,step,ticket.terminal_is_empty(),"factory ticket payload")?;
    Ok(RetainedCloneStep::Progress(step.progress()))
}

/// 🧷️ Native erased children retain the actual original cleanup loan through terminal frame release.
pub fn close_factory_ticket_original<T:ErasedSnapshotRetirement+?Sized>(slot:&mut Option<Box<T>>,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let Some(ticket)=slot.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};if ticket.terminal_is_empty(){return close_factory_ticket(slot,grant)}let step=ticket.close_original_step(original,grant)?;let step=crate::retained_clone::admit_retained_clone_close(grant,step,ticket.terminal_is_empty(),"original factory ticket payload")?;Ok(RetainedCloneStep::Progress(step.progress()))}
