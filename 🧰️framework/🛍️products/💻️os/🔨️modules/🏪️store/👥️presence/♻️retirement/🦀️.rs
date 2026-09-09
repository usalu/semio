//! 🧹️ Retained ownership of a detached local presence root and its complete peer roster.

use super::*;

//#region 🧹️StoreRetirement
pub(super) fn advance_returned_local<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: Option<&Arc<dyn SnapshotRetirementFactory<P>>>,
    maximum_items: usize,
    maximum_bytes: usize,
) -> Result<SnapshotRetirementStep, String> {
    if maximum_items == 0 {
        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    }
    if let Some(owner) = active.as_mut() {
        return match owner.close_step(1, maximum_bytes)? {
            SnapshotRetirementStep::Complete if owner.terminal_is_empty() => {
                drop(active.take());
                Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            SnapshotRetirementStep::Complete => Err("presence returned local owner completed without its exact empty witness".into()),
            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("presence returned local owner exceeded its exact grant".into()),
            step => Ok(step),
        };
    }
    if !registry.has_returned() {
        return Ok(SnapshotRetirementStep::Complete);
    }
    let factory = factory.ok_or_else(|| "presence returned local read has no exact retirement factory".to_string())?;
    match registry.try_take_one_returned::<P>() {
        Ok(Some(root)) => {
            *active = Some(factory.retire(root));
            Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
        }
        Ok(None) => Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
        Err(reason) if reason == "snapshot read lease registry is busy" => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
        Err(reason) => Err(reason),
    }
}

pub struct PresenceStoreRetirement<P> {
    base_root: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    peers: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    active_local: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    active_peers: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    reads: std::mem::ManuallyDrop<Option<Arc<SnapshotReadLeaseRegistry>>>,
    active_returned: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    local_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    peer_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
}

impl<P: Send + Sync + 'static> PresenceStoreRetirement<P> {
    fn new(
        local: Arc<P>,
        peers: Arc<PresencePeersRoot<P>>,
        reads: Arc<SnapshotReadLeaseRegistry>,
        active_returned: Option<Box<dyn ErasedSnapshotRetirement>>,
        local_factory: Arc<dyn SnapshotRetirementFactory<P>>,
        peer_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    ) -> Self {
        Self {
            base_root: std::mem::ManuallyDrop::new(None),
            local: std::mem::ManuallyDrop::new(Some(local)),
            peers: std::mem::ManuallyDrop::new(Some(peers)),
            active_local: std::mem::ManuallyDrop::new(None),
            active_peers: std::mem::ManuallyDrop::new(None),
            reads: std::mem::ManuallyDrop::new(Some(reads)),
            active_returned: std::mem::ManuallyDrop::new(active_returned),
            local_factory: Some(local_factory),
            peer_factory,
        }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.base_root.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(reads) = self.reads.as_ref() {
            if self.active_returned.is_some() || reads.has_returned() {
                return advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref(), 1, maximum_bytes);
            }
        }
        if let Some(active) = self.active_local.as_mut() {
            let step = active.close_step(1, maximum_bytes)?;
            return match step {
                SnapshotRetirementStep::Complete => {
                    if !active.terminal_is_empty() {
                        return Err("presence local close reported Complete without its terminal-empty witness".into());
                    }
                    drop(self.active_local.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("presence local close exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(active) = self.active_peers.as_mut() {
            let step = active.close_step(1, maximum_bytes)?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !active.terminal_is_empty() {
                return Err("presence peer close reported Complete without its terminal-empty witness".into());
            }
            drop(self.active_peers.take());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(local) = self.local.take() {
            *self.active_local = Some(self.local_factory.as_ref().expect("detached local root retains its installed factory").retire(local));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(peers) = self.peers.take() {
            return match Arc::try_unwrap(peers) {
                Ok(peers) => {
                    if peers.is_empty() {
                        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    }
                    let retired = PresencePeersRetiredEntries { entries: std::mem::ManuallyDrop::new(peers.entries), len: peers.len };
                    *self.active_peers = Some(PresencePeersRetirement::new(retired, self.peer_factory.as_ref().expect("detached nonempty peer root retains its installed factory").clone()));
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(peers) => {
                    *self.peers = Some(peers);
                    Ok(SnapshotRetirementStep::Blocked)
                }
            };
        }
        if self.reads.as_ref().is_some_and(|reads| !reads.terminal_is_empty()) {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if self.reads.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.local_factory.take().is_some() || self.peer_factory.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }
}

impl<P> PresenceStoreRetirement<P> {
    pub fn terminal_is_empty(&self) -> bool {
        self.base_root.is_none()
            && self.local.is_none()
            && self.peers.is_none()
            && self.active_local.is_none()
            && self.active_peers.is_none()
            && self.active_returned.is_none()
            && self.reads.is_none()
            && self.local_factory.is_none()
            && self.peer_factory.is_none()
    }
}

impl<P> Drop for PresenceStoreRetirement<P> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.terminal_is_empty(), "presence store retirement requires its exact terminal-empty witness");
        }
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresenceStoreRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        PresenceStoreRetirement::close_step(self, maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        PresenceStoreRetirement::terminal_is_empty(self)
    }
}

impl<P: Send + Sync + 'static> PresencePeersCommit<P> {
    pub fn into_retirement(self) -> PresenceStoreRetirement<P> {
        PresenceStoreRetirement {
            base_root: std::mem::ManuallyDrop::new(Some(self.base_root)),
            local: std::mem::ManuallyDrop::new(None),
            peers: std::mem::ManuallyDrop::new(Some(self.root)),
            active_local: std::mem::ManuallyDrop::new(None),
            active_peers: std::mem::ManuallyDrop::new(self.retirement),
            reads: std::mem::ManuallyDrop::new(None),
            active_returned: std::mem::ManuallyDrop::new(None),
            local_factory: None,
            peer_factory: Some(self.factory),
        }
    }
}
//#endregion 🧹️StoreRetirement

//#region 🔌️OwnerTransfer
impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> PresenceStore<P, M> {
    /// 🧹️ Detaches exact roots once after the concrete domain validates its empty terminal value.
    pub fn begin_retirement(&mut self, terminal_local: Arc<P>, terminal_is_empty: fn(&P) -> bool) -> Result<PresenceStoreRetirement<P>, (&'static str, Arc<P>)> {
        if self.close_started || !terminal_is_empty(terminal_local.as_ref()) {
            return Err(("presence close requires a fresh store and an exact empty domain terminal", terminal_local));
        }
        let Some(local_factory) = self.local_retirement_factory.as_ref() else {
            return Err(("presence close requires its installed local-root retirement factory", terminal_local));
        };
        if !self.peers.is_empty() && self.peer_retirement_factory.is_none() {
            return Err(("presence close requires its installed peer retirement factory", terminal_local));
        }
        let local_factory = local_factory.clone();
        let peer_factory = self.peer_retirement_factory.clone();
        self.close_started = true;
        let local = std::mem::replace(&mut *self.local, terminal_local);
        let peers = std::mem::replace(&mut *self.peers, Arc::new(PresencePeersRoot::empty()));
        Ok(PresenceStoreRetirement::new(local, peers, Arc::clone(&self.local_reads), self.active_returned_local.take(), local_factory, peer_factory))
    }

    pub fn retirement_started(&self) -> bool {
        self.close_started
    }
}

impl<P, M> Drop for PresenceStore<P, M> {
    fn drop(&mut self) {
        let terminal = self.close_started && self.local_reads.terminal_is_empty() && self.active_returned_local.is_none();
        if !std::thread::panicking() {
            assert!(terminal, "presence store requires its exact detached terminal-empty owner before Drop");
        }
        if terminal {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.local);
                std::mem::ManuallyDrop::drop(&mut self.peers);
                std::mem::ManuallyDrop::drop(&mut self.local_reads);
            }
        }
    }
}
//#endregion 🔌️OwnerTransfer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️testkit/🧬️mutations/🦀️.rs"]
mod fixture_mutations;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
