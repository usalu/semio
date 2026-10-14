//! ♻️ Shared bounded close receipt and original backing retirement contracts.
use semio_framework_job::{InteractiveJobCloseStep as CloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::mem::size_of;

pub fn prepared_close_gate(grant: RetainedCloneGrant, terminal: bool, release: Result<usize, ValueError>) -> Option<Result<CloseStep,ValueError>> {
    if terminal { return Some(Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })); }
    let bytes = match release { Ok(bytes) => bytes, Err(error) => return Some(Err(error)) };
    if grant.maximum_items == 0 || bytes > grant.maximum_release_bytes { return Some(Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() })); }
    if grant.maximum_depth == 0 { return Some(Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared close requires depth"))); }
    None
}

pub fn prepared_close_progress(released_bytes: usize) -> Result<CloseStep,ValueError> {
    Ok(CloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes } })
}

pub fn prepared_close_child(step: Result<CloseStep,ValueError>) -> Result<CloseStep,ValueError> {
    step.map(|step|match step { CloseStep::Complete { progress } => CloseStep::Pending { progress }, step => step })
}

pub fn prepared_vec_bytes<T>(value: &Vec<T>) -> Result<usize, ValueError> {
    value.capacity().checked_mul(size_of::<T>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "prepared backing extent overflow"))
}

pub fn prepared_owner_demands(terminal: bool, release: Result<usize, ValueError>) -> Result<RetainedCloneGrant, ValueError> {
    Ok(RetainedCloneGrant { maximum_items: usize::from(!terminal), maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: release?, maximum_depth: usize::from(!terminal) })
}

pub fn prepared_close_text(key: &mut String) -> usize {
    if key.pop().is_some() { return 0; }
    let bytes = key.capacity();
    *key = String::new();
    bytes
}

pub fn prepared_close_bytes(bytes: &mut Vec<u8>, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
    if !bytes.is_empty() {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared bytes close requires depth")); }
        let processed = bytes.len().min(grant.maximum_copy_bytes);
        bytes.truncate(bytes.len() - processed);
        return Ok(CloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: processed, ..Default::default() } });
    }
    let release = bytes.capacity();
    if grant.maximum_items == 0 || release > grant.maximum_release_bytes { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
    if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared byte backing close requires depth")); }
    drop(std::mem::take(bytes));
    prepared_close_progress(release)
}

