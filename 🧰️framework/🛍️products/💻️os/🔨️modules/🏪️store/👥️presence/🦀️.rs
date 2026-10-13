//! 👥️ Presence peers: the bounded peer table and its root custody, peer entries, retirement of returned peer roots, and
//! the lane item receipt every ephemeral publication reports. The ephemeral lanes of the engine own one table; the
//! presence read, publication io and rejection leaves read it without naming the engine.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::ValueError;
use std::sync::Arc;
use super::snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
#[cfg(target_arch = "wasm32")]
use super::serialization::{ArtifactCodecApplyFuture};
use crate::os_pack::pack_rt;
#[cfg(target_arch = "wasm32")]
use super::codec_registry::{ComponentDocumentCodecFuture};
#[cfg(target_arch = "wasm32")]
use super::backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use super::backbone::local_storage_backbone_key;

/// 🧬️ Exact witness returned after one, and only one, lane item changes a live root.
/// 🌉️ Carries serde alongside `ToValue`/`FromValue`: `TypedOperationResultPage::try_serialize`
/// (`🔌️plugin/🦀️.rs`) is bound on `serde::Serialize`, and both fields are `u64`, so the derives add
/// no bound fan-out. `rename_all` is mirrored so the two encodings emit the same keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct LaneItemReceipt {
    pub generation_before: u64,
    pub generation_after: u64,
}

pub const PRESENCE_PEER_SLOTS: usize = 64;

pub const PRESENCE_PEER_ID_BYTES: usize = 256;

#[derive(Debug)]
pub(crate) struct PresencePeerEntry<P> {
    pub(crate) actor: std::mem::ManuallyDrop<String>,
    presence: std::mem::ManuallyDrop<Option<Arc<P>>>,
    received_at_ms: i64,
}

impl<P> PresencePeerEntry<P> {
    pub(crate) fn new(actor: String, presence: Arc<P>, received_at_ms: i64) -> Self {
        Self { actor: std::mem::ManuallyDrop::new(actor), presence: std::mem::ManuallyDrop::new(Some(presence)), received_at_ms }
    }
}

impl<P> Drop for PresencePeerEntry<P> {
    fn drop(&mut self) {
        let terminal = self.actor.capacity() == 0 && self.presence.is_none();
        if !std::thread::panicking() {
            assert!(std::thread::panicking() || (terminal), "presence peer entry requires exact final-owner retirement");
        }
        if terminal {
            unsafe {
                drop(std::mem::ManuallyDrop::take(&mut self.actor));
                drop(std::mem::ManuallyDrop::take(&mut self.presence));
            }
        }
    }
}

#[derive(Debug)]
pub struct PresencePeersRoot<P> {
    pub(crate) entries: [Option<Arc<PresencePeerEntry<P>>>; PRESENCE_PEER_SLOTS],
    pub(crate) len: usize,
}

impl<P> PresencePeersRoot<P> {
    pub(crate) fn clone_aliases(&self) -> Self {
        Self { entries: self.entries.clone(), len: self.len }
    }
}

impl<P> PresencePeersRoot<P> {
    pub fn empty() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn peers(&self) -> impl Iterator<Item = (&str, &P)> {
        self.entries[..self.len].iter().filter_map(|entry| entry.as_deref().and_then(|entry| entry.presence.as_deref().map(|presence| (entry.actor.as_str(), presence))))
    }

    fn upsert(&self, actor: String, presence: P, received_at_ms: i64) -> Result<(Self, Option<Arc<PresencePeerEntry<P>>>), String> {
        if actor.is_empty() || actor.len() > PRESENCE_PEER_ID_BYTES {
            return Err("presence peer actor is empty or exceeds its fixed byte authority".into());
        }
        let mut next = self.clone_aliases();
        let mut index = 0usize;
        while index < next.len && next.entries[index].as_ref().is_some_and(|entry| entry.actor.as_str() < actor.as_str()) {
            index += 1;
        }
        if next.entries.get(index).and_then(Option::as_ref).is_some_and(|entry| entry.actor.as_str() == actor.as_str()) {
            let retired = next.entries[index].replace(Arc::new(PresencePeerEntry::new(actor, Arc::new(presence), received_at_ms)));
            return Ok((next, retired));
        }
        if next.len == PRESENCE_PEER_SLOTS {
            return Err("presence peer root exceeds its fixed item authority".into());
        }
        for cursor in (index..next.len).rev() {
            next.entries[cursor + 1] = next.entries[cursor].take();
        }
        next.entries[index] = Some(Arc::new(PresencePeerEntry::new(actor, Arc::new(presence), received_at_ms)));
        next.len += 1;
        Ok((next, None))
    }

    fn remove(&self, actor: &str) -> (Self, Option<Arc<PresencePeerEntry<P>>>) {
        let mut next = self.clone_aliases();
        let Some(index) = next.entries[..next.len].iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor.as_str() == actor)) else {
            return (next, None);
        };
        let retired = next.entries[index].take();
        for cursor in index..next.len.saturating_sub(1) {
            next.entries[cursor] = next.entries[cursor + 1].take();
        }
        next.len = next.len.saturating_sub(1);
        next.entries[next.len] = None;
        (next, retired)
    }

    pub(crate) fn remove_at(&self, index: usize) -> (Self, Option<Arc<PresencePeerEntry<P>>>) {
        if index >= self.len {
            return (self.clone_aliases(), None);
        }
        let mut next = self.clone_aliases();
        let retired = next.entries[index].take();
        for cursor in index..next.len.saturating_sub(1) {
            next.entries[cursor] = next.entries[cursor + 1].take();
        }
        next.len = next.len.saturating_sub(1);
        next.entries[next.len] = None;
        (next, retired)
    }

    fn retain_received(&self, oldest_allowed_ms: i64) -> (Self, PresencePeersRetiredEntries<P>) {
        let mut next = Self::empty();
        let mut retired = PresencePeersRetiredEntries::empty();
        for entry in self.entries[..self.len].iter().filter_map(Option::as_ref) {
            if entry.received_at_ms >= oldest_allowed_ms {
                next.entries[next.len] = Some(entry.clone());
                next.len += 1;
            } else {
                retired.entries[retired.len] = Some(entry.clone());
                retired.len += 1;
            }
        }
        (next, retired)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresencePeersCapacityError;

pub struct PresencePeersRetiredEntries<P> {
    pub(crate) entries: std::mem::ManuallyDrop<[Option<Arc<PresencePeerEntry<P>>>; PRESENCE_PEER_SLOTS]>,
    pub(crate) len: usize,
}

impl<P> PresencePeersRetiredEntries<P> {
    pub fn empty() -> Self {
        Self { entries: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)), len: 0 }
    }

    pub(crate) fn one(entry: Option<Arc<PresencePeerEntry<P>>>) -> Self {
        let mut retired = Self::empty();
        if let Some(entry) = entry {
            retired.entries[0] = Some(entry);
            retired.len = 1;
        }
        retired
    }

    pub fn append(&mut self, other: &mut Self) -> Result<(), PresencePeersCapacityError> {
        if self.len.saturating_add(other.len) > PRESENCE_PEER_SLOTS {
            return Err(PresencePeersCapacityError);
        }
        for index in 0..other.len {
            if let Some(entry) = other.entries[index].take() {
                self.entries[self.len] = Some(entry);
                self.len += 1;
            }
        }
        other.len = 0;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<P> Drop for PresencePeersRetiredEntries<P> {
    fn drop(&mut self) {
        debug_assert!(self.is_empty(), "nonterminal app-typed presence retirement entries reached Drop");
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PresencePeerRootCustody { BaseAlias, Displaced }

pub struct PresencePeersRetirement<P> {
    root_custody: PresencePeerRootCustody,
    pub(crate) root: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    owned_root: std::mem::ManuallyDrop<Option<PresencePeersRoot<P>>>,
    retired: std::mem::ManuallyDrop<PresencePeersRetiredEntries<P>>,
    waiting: std::mem::ManuallyDrop<Option<Arc<PresencePeerEntry<P>>>>,
    pub(crate) entry: std::mem::ManuallyDrop<Option<PresencePeerEntry<P>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(crate) factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    factory_close: Option<semio_framework_value::FactoryAuthority>,
    cursor: usize,
}

pub(crate) fn presence_arc_bytes<T>() -> Result<usize, ValueError> {
    std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<T>()).map(|(layout, _)| layout.pad_to_align().size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "presence Arc layout overflow"))
}

impl<P: Send + Sync + 'static> PresencePeersRetirement<P> {
    /// 🔗️ Retires one captured alias of a peer roster: a non-final alias releases its Arc handle only, the final one retires every entry through the issuer.
    pub fn of_alias(root: Arc<PresencePeersRoot<P>>, factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>) -> Self {
        Self::from_root(root, factory, PresencePeerRootCustody::BaseAlias)
    }
    pub(crate) fn new(retired: PresencePeersRetiredEntries<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { root_custody: PresencePeerRootCustody::Displaced, root: std::mem::ManuallyDrop::new(None), owned_root: std::mem::ManuallyDrop::new(None), retired: std::mem::ManuallyDrop::new(retired), waiting: std::mem::ManuallyDrop::new(None), entry: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(None), factory: Some(factory), factory_close: None, cursor: 0 }
    }
    pub(crate) fn from_root(root: Arc<PresencePeersRoot<P>>, factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>, root_custody: PresencePeerRootCustody) -> Self {
        Self { root_custody, root: std::mem::ManuallyDrop::new(Some(root)), owned_root: std::mem::ManuallyDrop::new(None), retired: std::mem::ManuallyDrop::new(PresencePeersRetiredEntries::empty()), waiting: std::mem::ManuallyDrop::new(None), entry: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(None), factory, factory_close: None, cursor: 0 }
    }
    pub fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence peer depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(artifact_retirement_box_demands(active, body)?); }
        if let Some(entry) = self.entry.as_ref() {
            if entry.actor.capacity() != 0 { return Ok(RetirementDemand { release_bytes: entry.actor.capacity(), depth: 1, ..Default::default() }); }
            if let Some(presence) = entry.presence.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence peer lacks original snapshot factory"))?.retirement_birth_bytes(presence), depth: 2, ..Default::default() }); }
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(_waiting) = self.waiting.as_ref() { return Ok(RetirementDemand { release_bytes: presence_arc_bytes::<PresencePeerEntry<P>>()?, depth: 1, ..Default::default() }); }
        if self.cursor < self.retired.len || self.retired.len != 0 { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(_root) = self.root.as_ref() { return Ok(RetirementDemand { release_bytes: presence_arc_bytes::<PresencePeersRoot<P>>()?, depth: 1, ..Default::default() }); }
        if self.owned_root.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.factory.is_some() { return Ok(RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.as_ref() { return nested(factory.demands(body)?); }
        Ok(Default::default())
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presence peers exceed admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(entry) = self.entry.as_mut() {
            if entry.actor.capacity() != 0 { *entry.actor = String::new(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })); }
            if let Some(original) = entry.presence.take() {
                return match self.factory.as_ref().expect("observed original presence factory").retire(original, child) { Ok((active, receipt)) => { *self.active = Some(active); if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "presence peer constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(receipt)) }, Err((error, original)) => { *entry.presence = Some(original); Err(error) } };
            }
            self.entry.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(waiting) = self.waiting.as_ref() {
            if Arc::weak_count(waiting) != 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let released_bytes = if let Some(entry) = Arc::into_inner(self.waiting.take().unwrap()) { *self.entry = Some(entry); demand.release_bytes } else { 0 };
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if self.cursor < self.retired.len { let index = self.cursor; self.cursor += 1; *self.waiting = self.retired.entries[index].take(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if self.retired.len != 0 { self.retired.len = 0; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if let Some(root) = self.root.as_ref() {
            if self.root_custody == PresencePeerRootCustody::Displaced && Arc::strong_count(root) != 1 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if Arc::weak_count(root) != 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if self.root_custody == PresencePeerRootCustody::BaseAlias {
                let released_bytes = if let Some(root) = Arc::into_inner(self.root.take().unwrap()) { *self.owned_root = Some(root); demand.release_bytes } else { 0 };
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
            }
            return match Arc::try_unwrap(self.root.take().unwrap()) { Ok(root) => { *self.owned_root = Some(root); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })) }, Err(original) => { *self.root = Some(original); Ok(RetainedCloneStep::Progress(Default::default())) } };
        }
        if let Some(root) = self.owned_root.as_mut() { if root.len != 0 { root.len -= 1; *self.waiting = root.entries[root.len].take(); } else { self.owned_root.take(); } return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if let Some(factory) = self.factory.take() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
        if let Some(factory) = self.factory_close.as_mut() { let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "presence peer factory")?; if factory.terminal_is_empty() { self.factory_close = None; } return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl<P> PresencePeersRetirement<P> {
    pub fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.owned_root.is_none() && self.retired.len == 0 && self.waiting.is_none() && self.entry.is_none() && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none() }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresencePeersRetirement<P> {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { PresencePeersRetirement::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { PresencePeersRetirement::terminal_is_empty(self) }
}

impl<P> Drop for PresencePeersRetirement<P> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "incomplete presence-peer retirement reached Drop without bounded disposal"); }
}
