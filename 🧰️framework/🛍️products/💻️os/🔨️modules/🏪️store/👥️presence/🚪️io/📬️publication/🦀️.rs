//! 📬️ Original peer publication receives independently admitted inline transfers and native Arc births.
use super::*;
use semio_framework_value::{ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

fn admit(grant: RetainedCloneGrant, progress: RetainedCloneProgress, depth: usize) -> Result<(), ValueError> {
    if grant.maximum_items < progress.copied_items { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "peer publication exceeds original item grant")); }
    if grant.maximum_depth < depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "peer publication exceeds original structural depth")); }
    if !progress.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "peer publication exceeds original physical grant")); }
    Ok(())
}
fn receipt(capacity: usize) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 } }
fn missing() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "peer publication no longer retains its original owner") }

impl<P: Send + Sync + 'static> PresencePeersPublication<P> {
    pub(super) fn new(current: &Arc<PresencePeersRoot<P>>, factory: &Arc<dyn SnapshotRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), ValueError> {
        let progress = receipt(0);
        admit(grant, progress, 1)?;
        Ok((Self { base_root: std::mem::ManuallyDrop::new(Some(current.clone())), candidate: std::mem::ManuallyDrop::new(Some(current.clone_aliases())), created: std::mem::ManuallyDrop::new(PresencePeersRetiredEntries::empty()), retired: std::mem::ManuallyDrop::new(PresencePeersRetiredEntries::empty()), active: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory.clone())), factory_close: None, prune_cursor: 0, pruning_complete: false }, progress))
    }

    pub fn prune_one(&mut self, mut keep: impl FnMut(&str) -> bool, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), ValueError> {
        if self.pruning_complete { return Ok((false, Default::default())); }
        let candidate = self.candidate.as_ref().ok_or_else(missing)?;
        let progress = receipt(0);
        admit(grant, progress, 1)?;
        if self.prune_cursor >= candidate.len { self.pruning_complete = true; return Ok((false, progress)); }
        if candidate.entries[self.prune_cursor].as_ref().is_some_and(|entry| keep(&entry.actor)) { self.prune_cursor += 1; return Ok((true, progress)); }
        if self.retired.len == PRESENCE_PEER_SLOTS { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "peer publication displaced owner slots are saturated")); }
        let progress = receipt(0);
        admit(grant, progress, 1)?;
        let (next, retired) = candidate.remove_at(self.prune_cursor);
        drop(self.candidate.replace(next).expect("admitted original peer candidate"));
        let mut retired = PresencePeersRetiredEntries::one(retired);
        self.retired.append(&mut retired).expect("preflight retained exact peer slot");
        Ok((true, progress))
    }

    pub fn adopt(&mut self, actor: &mut Option<String>, presence: &mut Option<P>, received_at_ms: i64, original: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<RetainedCloneStep, ValueError> {
        if !self.pruning_complete { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "peer publication must finish pruning before adoption")); }
        let candidate = self.candidate.as_ref().ok_or_else(missing)?;
        let source = actor.as_ref().ok_or_else(missing)?;
        if presence.is_none() { return Err(missing()); }
        if source.is_empty() || source.len() > PRESENCE_PEER_ID_BYTES { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "peer actor exceeds its original UTF8 authority")); }
        let index = candidate.entries[..candidate.len].partition_point(|entry| entry.as_ref().is_some_and(|entry| entry.actor.as_str() < source.as_str()));
        let replaces = candidate.entries.get(index).and_then(Option::as_ref).is_some_and(|entry| entry.actor.as_str() == source.as_str());
        if self.created.len == PRESENCE_PEER_SLOTS || (replaces && self.retired.len == PRESENCE_PEER_SLOTS) || (!replaces && candidate.len == PRESENCE_PEER_SLOTS) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "peer publication original ownership slots are saturated")); }
        let capacity = presence_arc_bytes::<P>()?.checked_add(presence_arc_bytes::<PresencePeerEntry<P>>()?).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "peer Arc birth extent overflow"))?;
        let progress = receipt(capacity);
        admit(original.remaining_grant(), progress, 2)?;
        original.native().scoped_stage(|native| { native.begin_stage(0)?; native.charge(capacity) })?;
        let mut next = candidate.clone_aliases();
        let inserted = Arc::new(PresencePeerEntry::new(actor.take().expect("admitted original actor"), Arc::new(presence.take().expect("admitted original presence")), received_at_ms));
        let retired = if replaces { next.entries[index].replace(inserted.clone()) } else { for cursor in (index..next.len).rev() { next.entries[cursor + 1] = next.entries[cursor].take(); } next.entries[index] = Some(inserted.clone()); next.len += 1; None };
        drop(self.candidate.replace(next).expect("admitted original peer candidate"));
        let created_index = self.created.len;
        self.created.entries[created_index] = Some(inserted);
        self.created.len += 1;
        if let Some(retired) = retired { let retired_index = self.retired.len; self.retired.entries[retired_index] = Some(retired); self.retired.len += 1; }
        original.record_progress(progress)?;
        Ok(RetainedCloneStep::Progress(progress))
    }

    pub fn release_created_one(&mut self, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), ValueError> {
        if self.created.len == 0 { return Ok((false, Default::default())); }
        let index = self.created.len - 1;
        if self.created.entries[index].as_ref().is_none_or(|entry| Arc::strong_count(entry) <= 1) { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "created peer alias retains final physical ownership")); }
        let progress = receipt(0);
        admit(grant, progress, 1)?;
        self.created.len = index;
        drop(self.created.entries[index].take());
        Ok((true, progress))
    }

    pub fn take_commit(&mut self, original: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<(PresencePeersCommit<P>, RetainedCloneProgress), ValueError> {
        if !self.pruning_complete || self.created.len != 0 || self.active.is_some() { return Err(missing()); }
        if self.candidate.is_none() || self.base_root.is_none() || self.factory.is_none() || self.factory_close.is_some() { return Err(missing()); }
        let progress = receipt(presence_arc_bytes::<PresencePeersRoot<P>>()?);
        admit(original.remaining_grant(), progress, 1)?;
        original.native().scoped_stage(|native| { native.begin_stage(0)?; native.charge(progress.retained_capacity_bytes) })?;
        let root = Arc::new(self.candidate.take().expect("admitted original peer root"));
        let retired = std::mem::replace(&mut *self.retired, PresencePeersRetiredEntries::empty());
        let factory = self.factory.take().expect("admitted original peer factory");
        let retirement = (!retired.is_empty()).then(|| PresencePeersRetirement::new(retired, factory.clone()));
        let commit = PresencePeersCommit { base_root: std::mem::ManuallyDrop::new(self.base_root.take().expect("admitted original base root")), root: std::mem::ManuallyDrop::new(root), retirement: std::mem::ManuallyDrop::new(retirement), factory: std::mem::ManuallyDrop::new(factory), transferred: false };
        original.record_progress(progress)?;
        Ok((commit, progress))
    }
}
