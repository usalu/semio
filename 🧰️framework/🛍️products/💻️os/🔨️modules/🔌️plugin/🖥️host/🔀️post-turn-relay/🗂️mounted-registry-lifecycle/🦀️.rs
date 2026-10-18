//! 🗂️ Mounted registry lifecycle: the registry constructor, the pool, the original wake checkout, collect, admit and close, the reservation, mount, detach, reaper and the detached takes. See [`super`].

use super::*;

//#region 🔀️PostTurnRelay
impl GuestRelayMountedRegistry {
    pub(crate) fn new(wake_authority:GuestRelayWakeAuthority) -> Self {
        Self::with_pool(plugin_host_worker_pool(),wake_authority)
    }

    pub(crate) fn with_pool(pool: WorkerPool,wake_authority:GuestRelayWakeAuthority) -> Self {
        Self {
            wake_authority,
            slots: std::iter::repeat_with(|| Mutex::new(GuestRelayMountedSlot::Empty)).take(GUEST_RELAY_MOUNTED_SLOTS).collect(),
            next_generation: std::sync::atomic::AtomicU64::new(1),
            pool,
            reaper_cursor: std::sync::atomic::AtomicUsize::new(0),
            reaper_epoch: std::sync::atomic::AtomicU64::new(0),
            reaper_active: AtomicBool::new(false),
            reaper_failed: AtomicBool::new(false),
            reaper_timer_pending: AtomicBool::new(false),
            #[cfg(any(test, feature = "testkit"))]
            reaper_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            reaper_timers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            reaper_wake_waits: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub(super) fn checkout_original_wake(&self,wake:&mut GuestRelayWakeCustody,index:usize,generation:u64)->Option<GuestRelayWakeTurnGrant>{
        let mut context=match self.wake_authority.receiver.checkout(index,generation){Ok(context)=>context?,Err(error)=>{wake.refusal=Some(error.kind);return None}};let policy=self.wake_authority.policy;
        context.grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:context.grant.maximum_items.min(policy.maximum_items),maximum_copy_bytes:context.grant.maximum_copy_bytes.min(policy.maximum_copy_bytes),maximum_capacity_bytes:context.grant.maximum_capacity_bytes.min(policy.maximum_capacity_bytes),maximum_release_bytes:context.grant.maximum_release_bytes.min(policy.maximum_release_bytes),maximum_depth:context.grant.maximum_depth.min(policy.maximum_depth)};Some(context)
    }

    pub(super) fn collect_original_wake_receipt(&self,wake:&mut GuestRelayWakeCustody)->bool{
        let Some(receipt)=wake.receipt.as_ref()else{return true};
        match self.wake_authority.receiver.collect(receipt){Ok(true)=>{wake.receipt=None;true},Ok(false)=>false,Err(error)=>{wake.refusal=Some(error.kind);false}}
    }

    pub(super) fn admit_original_session_wake<J:semio_framework_job::InteractiveJob+'static>(&self,owner:&semio_framework_job_worker::WorkerJobSession<J>,wake:&mut GuestRelayWakeCustody,index:usize,generation:u64,waker:&std::task::Waker)->bool{
        if !self.collect_original_wake_receipt(wake){return false}
        if wake.registered{return true}
        let Some(context)=self.checkout_original_wake(wake,index,generation)else{return false};let grant=context.grant;let epoch=context.epoch;
        if wake.original.is_some(){
            match owner.register_retained_wake(&mut wake.original,grant){
                Ok(Some(progress))=>{assert!(progress.fits(grant),"original mounted wake registration exceeded supplied authority");wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});wake.registered=true;waker.wake_by_ref();},
                Ok(None)|Err(_)=>{wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:Default::default()});}
            }
            self.collect_original_wake_receipt(wake);return false;
        }
        match (self.wake_authority.issuer)(waker,grant){
            Ok(Some((original,progress)))=>{assert!(progress.fits(grant),"original mounted wake issuer exceeded supplied authority");*wake.original=Some(original);wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});wake.refusal=None;wake.original.as_ref().unwrap().wake_by_ref();},
            Ok(None)=>{wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:Default::default()});},
            Err(error)=>{wake.refusal=Some(error.kind);let progress=error.retained_progress();wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});}
        }
        self.collect_original_wake_receipt(wake);false
    }

    fn close_original_wake(&self,wake:&mut GuestRelayWakeCustody,index:usize,generation:u64)->GuestRelayMountedReap{
        if !self.collect_original_wake_receipt(wake){return GuestRelayMountedReap::Blocked{wake_registered:false}}
        if wake.original.is_none(){return GuestRelayMountedReap::Idle};let Some(context)=self.checkout_original_wake(wake,index,generation)else{return GuestRelayMountedReap::Blocked{wake_registered:false}};let grant=context.grant;let epoch=context.epoch;let original=wake.original.as_mut().unwrap();
        let terminal=original.terminal_is_empty();let demand=if terminal{semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<Box<dyn semio_framework_job_worker::RetainedWorkerWake>>>(),release_bytes:original.frame_release_bytes(),depth:1,..Default::default()}}else{match original.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>{wake.refusal=Some(error.kind);wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:error.retained_progress()});self.collect_original_wake_receipt(wake);return GuestRelayMountedReap::Blocked{wake_registered:false}}}};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress:Default::default()});self.collect_original_wake_receipt(wake);return GuestRelayMountedReap::Blocked{wake_registered:false}}
        let progress=if terminal{drop(wake.original.take());semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}else{match original.close_step(grant){Ok(step)=>step.progress(),Err(error)=>{wake.refusal=Some(error.kind);error.retained_progress()}}};
        assert!(progress.fits(grant),"original mounted wake close exceeded supplied authority");wake.receipt=Some(GuestRelayWakeReceipt{epoch,slot:index,generation,grant,progress});let collected=self.collect_original_wake_receipt(wake);if !collected||progress==Default::default(){GuestRelayMountedReap::Blocked{wake_registered:false}}else{GuestRelayMountedReap::Progress}
    }

    pub(crate) fn reserve(&self) -> Option<(usize, u64)> {
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        for (index, slot) in self.slots.iter().enumerate() {
            let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
                return None;
            }
            if matches!(*slot, GuestRelayMountedSlot::Empty) {
                let generation = self
                    .next_generation
                    .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| match generation {
                        0 => None,
                        u64::MAX => Some(0),
                        generation => Some(generation + 1),
                    })
                    .ok()?;
                *slot = GuestRelayMountedSlot::Reserved { generation, output: GuestRelayMountedOutput::new() };
                return Some((index, generation));
            }
        }
        None
    }

    pub(super) fn mount(&self, index: usize, generation: u64, mut owner: GuestRelayMountedOwner) {
        let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let reserved = std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty);
        let GuestRelayMountedSlot::Reserved { generation: reserved_generation, output } = reserved else {
            debug_assert!(false, "mounted relay requires its exact reserved output owner");
            return;
        };
        if reserved_generation != generation {
            debug_assert!(false, "mounted relay generation must match its reserved output owner");
            *slot = GuestRelayMountedSlot::Reserved { generation: reserved_generation, output };
            return;
        }
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire){
            if let GuestRelayMountedOwner::Rejected(rejected)=&mut owner{Self::close_rejected(rejected);}
        }
        *slot =
            GuestRelayMountedSlot::Mounted(GuestRelayMountedSession { wake:GuestRelayWakeCustody::new(), generation, owner, checked_out: None, lifecycle_probe_checked_out: None, outcome_stage: GuestRelayOutcomeStage::Idle, outcome_kind: GuestRelayOutcomeKind::Continues, outcome_page: 0, output, terminal: None, lifecycle: GuestRelayMountedLifecycle::Running });
    }

    pub(crate) fn detach(self: &Arc<Self>, index: usize, generation: u64) {
        let detached = {
            let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { return };
            if session.generation != generation {
                return;
            }
            if let Some(owner) = session.checked_out.take() {
                owner.begin_close();
            } else if let Some(owner) = session.lifecycle_probe_checked_out.take() {
                owner.begin_close();
            } else {
                match &session.owner {
                    GuestRelayMountedOwner::Session(owner) => {
                        let _ = owner.begin_close();
                    }
                    GuestRelayMountedOwner::LifecycleProbe(owner) => {
                        let _ = owner.begin_close();
                    }
                    GuestRelayMountedOwner::Rejected(_) | GuestRelayMountedOwner::Empty => {}
                }
            }
            session.lifecycle = GuestRelayMountedLifecycle::DetachedForReap;
            true
        };
        if detached {
            self.reaper_epoch.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.ensure_reaper();
        }
    }

    pub(super) fn ensure_reaper(self: &Arc<Self>) {
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            self.fail_detached();
            return;
        }
        if self.reaper_active.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let registry = Arc::clone(self);
        let recovery = Arc::clone(self);
        GuestRelayPoolFuture::spawn_recoverable(self.pool.clone(), Lane::Maintenance, GuestRelayMountedReaper { registry }, move |failure| {
            recovery.reaper_active.store(false, std::sync::atomic::Ordering::Release);
            match failure {
                GuestRelayPoolFailure::FuturePanicked => recovery.ensure_reaper(),
                GuestRelayPoolFailure::Admission(_) => {
                    recovery.reaper_failed.store(true, std::sync::atomic::Ordering::Release);
                    recovery.fail_detached();
                }
            }
        });
    }

    fn fail_detached(&self) {
        self.reaper_failed.store(true,std::sync::atomic::Ordering::Release);
    }

    fn close_rejected(owner:&mut GuestRelayRefusedSource<GuestColdRelayJob>){
        owner.begin_close();
    }

    pub(super) fn reap_one(&self, waker: &std::task::Waker) -> GuestRelayMountedReap {
        let cursor = self.reaper_cursor.load(std::sync::atomic::Ordering::Acquire) % GUEST_RELAY_MOUNTED_SLOTS;
        for offset in 0..GUEST_RELAY_MOUNTED_SLOTS {
            let index = (cursor + offset) % GUEST_RELAY_MOUNTED_SLOTS;
            let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { continue };
            if !matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
                continue;
            }
            self.reaper_cursor.store((index + 1) % GUEST_RELAY_MOUNTED_SLOTS, std::sync::atomic::Ordering::Release);
            match self.close_original_wake(&mut session.wake,index,session.generation){GuestRelayMountedReap::Idle=>{},step=>return step}
            return match self.pump_close(session,index,Some(waker)) {
                Ok(GuestRelayMountedClose::Complete) => {
                    *slot = GuestRelayMountedSlot::Empty;
                    GuestRelayMountedReap::Progress
                }
                Ok(GuestRelayMountedClose::Pending) => GuestRelayMountedReap::Progress,
                Ok(GuestRelayMountedClose::Blocked { wake_registered }) => GuestRelayMountedReap::Blocked { wake_registered },
                Err(failure)=>{session.terminal=Some(GuestRelayMountedTerminal::HostFault(PluginHostError::OriginalTurn(failure)));GuestRelayMountedReap::Blocked{wake_registered:session.wake.registered}},
            };
        }
        GuestRelayMountedReap::Idle
    }

    pub(super) fn schedule_reaper_timer(self: &Arc<Self>, waker: &std::task::Waker) {
        if self.reaper_timer_pending.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        #[cfg(any(test, feature = "testkit"))]
        self.reaper_timers.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let registry = Arc::clone(self);
        let wake = waker.clone();
        self.pool.submit_at(
            self.pool.now_ms().saturating_add(GUEST_RELAY_REAPER_FALLBACK_MS),
            Lane::Maintenance,
            Box::new(move || {
                registry.reaper_timer_pending.store(false, std::sync::atomic::Ordering::Release);
                wake.wake();
            }),
        );
    }

    pub(super) fn park_blocked_reaper(self: &Arc<Self>, waker: &std::task::Waker, wake_registered: bool) {
        if wake_registered {
            #[cfg(any(test, feature = "testkit"))]
            self.reaper_wake_waits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        } else {
            self.schedule_reaper_timer(waker);
        }
    }

    pub(super) fn has_detached(&self) -> bool {
        self.slots.iter().any(|slot| {
            let slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap))
        })
    }

}
//#endregion 🔀️PostTurnRelay
