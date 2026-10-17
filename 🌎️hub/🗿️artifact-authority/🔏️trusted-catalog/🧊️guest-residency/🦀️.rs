//! 🧊️ Compiled-guest residency: the codec budget, the memory-budgeted residency ledger with its slots and the footprint port.

use super::*;

/// ⛽️ Fuel and wall-clock ceiling for ONE guest codec call. A `codec` function is pure and bounded
/// (encode a snapshot, print a history log, reduce an op batch), so a call that needs more than this
/// is refused rather than allowed to hold a request thread. The fuel figure is the same order the
/// build-time `describe()` cap sits at, measured on a real debug-built component.
pub const GUEST_CODEC_BUDGET: semio_framework::kernel::Budget =
    semio_framework::kernel::Budget { fuel: 4_000_000_000, deadline_ms: 30_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };

/// 🧊️ Compiled values held within an operator-configured memory budget (`TrustedCatalogGuestResidencyV1`):
/// every value is charged its source's byte length, and every operation ([`OperationContext`]) that uses a
/// value counts as one use of it however many calls it makes. A newly compiled value that fits stays resident.
/// One that does not fit stays only when its uses before this operation outnumber those of every value it
/// would release — the least recently used values no call holds, taken in that order — and otherwise serves
/// the operation that compiled it (and any other that reaches it meanwhile) and is dropped with it. Use counts
/// are capped and halve together after a declared number of uses per registered value, so a round-robin over
/// more values than fit keeps a stable resident set instead of recompiling every value on every use, while a
/// workload that moved on displaces the values it left. Nothing is released for idleness, a value a running
/// call holds is never released, and a released value compiles again on its next use.
pub struct GuestResidencyLedgerV1<T> {
    pub budget_bytes: std::sync::atomic::AtomicU64,
    pub access_count_ceiling: u32,
    pub access_count_aging_per_guest: u64,
    clock: std::sync::atomic::AtomicU64,
    accesses_since_aging: std::sync::atomic::AtomicU64,
    pub hits: std::sync::atomic::AtomicU64,
    pub compiles: std::sync::atomic::AtomicU64,
    pub admitted: std::sync::atomic::AtomicU64,
    pub bypassed: std::sync::atomic::AtomicU64,
    pub released: std::sync::atomic::AtomicU64,
    pub compile_micros: std::sync::atomic::AtomicU64,
    slots: std::sync::Mutex<Vec<Arc<GuestResidencySlotV1<T>>>>,
}

/// 📏️ What a resident value holds beyond the bytes it was registered with, measured on the value itself once it exists
/// and again after every codec call that used it — a guest's state can grow with its first call.
pub trait GuestResidentFootprintV1 {
    /// 📐️ The bytes the value holds beyond its registration charge.
    fn footprint_bytes(&self) -> u64;
}

impl GuestResidentFootprintV1 for semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest {
    /// 🧊️ Beyond its parsed component (which its registration charges) a compiled guest holds the owned interpreter's codec
    /// origin once a codec call assembled it: the guest's linear memory right after its plugin bundle assembled.
    fn footprint_bytes(&self) -> u64 {
        self.codec_origin_bytes()
    }
}

/// 🧩️ What one slot holds: nothing, a resident value, or a value that serves only the calls holding it.
enum GuestResidentValueV1<T> {
    Absent,
    Resident(Arc<T>),
    Held(std::sync::Weak<T>),
}

struct GuestResidencySlotV1<T> {
    value: tokio::sync::Mutex<GuestResidentValueV1<T>>,
    charge_bytes: u64,
    footprint_bytes: std::sync::atomic::AtomicU64,
    last_used: std::sync::atomic::AtomicU64,
    access_count: std::sync::atomic::AtomicU32,
    last_operation: std::sync::atomic::AtomicU64,
    prior_uses: std::sync::atomic::AtomicU32,
}

impl<T> GuestResidencySlotV1<T> {
    fn counts_as_resident(&self) -> bool {
        self.value.try_lock().map_or(true, |value| match &*value {
            GuestResidentValueV1::Resident(_) => true,
            GuestResidentValueV1::Held(held) => held.strong_count() > 0,
            GuestResidentValueV1::Absent => false,
        })
    }

    fn charged(&self) -> bool {
        self.value.try_lock().map_or(true, |value| matches!(&*value, GuestResidentValueV1::Resident(_)))
    }

    /// 📏️ What the slot's value is charged: its registered bytes and its measured footprint.
    fn charge(&self) -> u64 {
        self.charge_bytes.saturating_add(self.footprint_bytes.load(std::sync::atomic::Ordering::Acquire))
    }
}

impl<T> GuestResidencyLedgerV1<T> {
    pub fn new(residency: TrustedCatalogGuestResidencyV1) -> Arc<Self> {
        let counter = || std::sync::atomic::AtomicU64::new(0);
        Arc::new(Self {
            budget_bytes: std::sync::atomic::AtomicU64::new(residency.resident_component_bytes),
            access_count_ceiling: residency.access_count_ceiling,
            access_count_aging_per_guest: residency.access_count_aging_per_guest.max(1),
            clock: counter(),
            accesses_since_aging: counter(),
            hits: counter(),
            compiles: counter(),
            admitted: counter(),
            bypassed: counter(),
            released: counter(),
            compile_micros: counter(),
            slots: std::sync::Mutex::new(Vec::new()),
        })
    }

    /// 🪪️ Registers one value charged `charge_bytes` once resident; answers its residency handle.
    pub fn register(self: &Arc<Self>, charge_bytes: u64) -> GuestResidencyV1<T> {
        let slot = Arc::new(GuestResidencySlotV1 {
            value: tokio::sync::Mutex::new(GuestResidentValueV1::Absent),
            charge_bytes,
            footprint_bytes: std::sync::atomic::AtomicU64::new(0),
            last_used: std::sync::atomic::AtomicU64::new(0),
            access_count: std::sync::atomic::AtomicU32::new(0),
            last_operation: std::sync::atomic::AtomicU64::new(0),
            prior_uses: std::sync::atomic::AtomicU32::new(0),
        });
        self.slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(Arc::clone(&slot));
        GuestResidencyV1 { ledger: Arc::clone(self), slot }
    }

    pub(super) fn snapshot(&self) -> Vec<Arc<GuestResidencySlotV1<T>>> {
        self.slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    /// 🎚️ Applies a new budget and releases least recently used values no call holds until the resident
    /// charge fits it.
    pub(crate) fn configure(&self, budget_bytes: u64) {
        self.budget_bytes.store(budget_bytes, std::sync::atomic::Ordering::Release);
        let mut slots = self.snapshot();
        slots.sort_by_key(|slot| slot.last_used.load(std::sync::atomic::Ordering::Acquire));
        let mut charged: u64 = slots.iter().filter(|slot| slot.charged()).map(|slot| slot.charge()).sum();
        for slot in slots {
            if charged <= budget_bytes {
                break;
            }
            let Ok(mut value) = slot.value.try_lock() else { continue };
            if matches!(&*value, GuestResidentValueV1::Resident(resident) if Arc::strong_count(resident) == 1) {
                *value = GuestResidentValueV1::Absent;
                charged = charged.saturating_sub(slot.charge());
                self.released.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            }
        }
    }

    /// 🕰️ Records one call of `operation`: the slot becomes the most recently used, and the operation's first
    /// call counts one use (capped); every `accessCountAgingPerGuest × registered` uses all counts halve.
    /// Answers the slot's uses before this operation and whether this call was the operation's first.
    fn touch(&self, slot: &GuestResidencySlotV1<T>, operation: u64) -> (u32, bool) {
        let tick = self.clock.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        slot.last_used.store(tick, std::sync::atomic::Ordering::Release);
        if slot.last_operation.swap(operation, std::sync::atomic::Ordering::AcqRel) == operation {
            return (slot.prior_uses.load(std::sync::atomic::Ordering::Acquire), false);
        }
        let slots = self.snapshot();
        let window = self.access_count_aging_per_guest.saturating_mul(u64::try_from(slots.len()).unwrap_or(u64::MAX).max(1));
        let aged = self.accesses_since_aging.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |uses| Some(if uses + 1 >= window { 0 } else { uses + 1 })).is_ok_and(|uses| uses + 1 >= window);
        if aged {
            for slot in slots {
                let _ = slot.access_count.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| Some(count / 2));
            }
        }
        let ceiling = self.access_count_ceiling;
        let prior = slot.access_count.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| (count < ceiling).then_some(count + 1)).unwrap_or_else(|count| count);
        slot.prior_uses.store(prior, std::sync::atomic::Ordering::Release);
        (prior, true)
    }

    /// ⚖️ Whether `keep`'s compiled value stays resident: it fits, or its `prior` uses outnumber those of every
    /// least recently used unheld value it would release, which are then released. Never admits a value larger
    /// than the whole budget. A slot another call is compiling or acquiring right now counts as charged.
    fn admit(&self, keep: &Arc<GuestResidencySlotV1<T>>, prior: u32) -> bool {
        let budget = self.budget_bytes.load(std::sync::atomic::Ordering::Acquire);
        if keep.charge() > budget {
            return false;
        }
        let slots = self.snapshot();
        let others: Vec<&Arc<GuestResidencySlotV1<T>>> = slots.iter().filter(|slot| !Arc::ptr_eq(slot, keep)).collect();
        let charged: u64 = others.iter().filter(|slot| slot.charged()).map(|slot| slot.charge()).sum();
        let excess = charged.saturating_add(keep.charge()).saturating_sub(budget);
        if excess == 0 {
            return true;
        }
        let mut candidates: Vec<(&Arc<GuestResidencySlotV1<T>>, tokio::sync::MutexGuard<'_, GuestResidentValueV1<T>>)> = others
            .iter()
            .filter_map(|slot| slot.value.try_lock().ok().filter(|value| matches!(&**value, GuestResidentValueV1::Resident(resident) if Arc::strong_count(resident) == 1)).map(|value| (*slot, value)))
            .collect();
        candidates.sort_by_key(|(slot, _)| slot.last_used.load(std::sync::atomic::Ordering::Acquire));
        let (mut freed, mut victim_count, mut hottest_victim) = (0u64, 0usize, 0u32);
        for (slot, _) in &candidates {
            if freed >= excess {
                break;
            }
            freed = freed.saturating_add(slot.charge());
            victim_count += 1;
            hottest_victim = hottest_victim.max(slot.access_count.load(std::sync::atomic::Ordering::Acquire));
        }
        if freed < excess || prior <= hottest_victim {
            return false;
        }
        for (_, value) in candidates.iter_mut().take(victim_count) {
            **value = GuestResidentValueV1::Absent;
        }
        self.released.fetch_add(u64::try_from(victim_count).unwrap_or(u64::MAX), std::sync::atomic::Ordering::AcqRel);
        true
    }

    /// 📏️ The ledger's state now.
    pub fn state(&self) -> TrustedCatalogGuestResidencyStateV1 {
        let slots = self.snapshot();
        let resident: Vec<_> = slots.iter().filter(|slot| slot.counts_as_resident()).collect();
        let load = |counter: &std::sync::atomic::AtomicU64| counter.load(std::sync::atomic::Ordering::Acquire);
        TrustedCatalogGuestResidencyStateV1 {
            budget_bytes: load(&self.budget_bytes),
            registered_guests: u64::try_from(slots.len()).unwrap_or(u64::MAX),
            resident_guests: u64::try_from(resident.len()).unwrap_or(u64::MAX),
            resident_bytes: resident.iter().map(|slot| slot.charge()).sum(),
            footprint_bytes: resident.iter().map(|slot| slot.footprint_bytes.load(std::sync::atomic::Ordering::Acquire)).sum(),
            hits: load(&self.hits),
            compiles: load(&self.compiles),
            admitted: load(&self.admitted),
            bypassed: load(&self.bypassed),
            released: load(&self.released),
            compile_micros: load(&self.compile_micros),
        }
    }
}

/// 🔑️ One value's place in a [`GuestResidencyLedgerV1`].
pub struct GuestResidencyV1<T> {
    ledger: Arc<GuestResidencyLedgerV1<T>>,
    pub slot: Arc<GuestResidencySlotV1<T>>,
}

impl<T: GuestResidentFootprintV1 + Send + Sync + 'static> GuestResidencyV1<T> {
    /// 🔑️ The resident or held value for one call of `context`'s operation, compiled by `compile` when there is
    /// none (one compile at a time). A value that is not admitted stays with the operation that compiled it — and
    /// with any other operation that reaches it meanwhile — and is dropped with the last of them; a held value
    /// is admitted as soon as its uses outnumber the values it would release.
    pub async fn acquire<F, Fut>(&self, context: &OperationContext<'_>, compile: F) -> Result<Arc<T>, AuthorityError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AuthorityError>>,
    {
        let mut value = self.slot.value.lock().await;
        let (prior, first_call) = self.ledger.touch(&self.slot, context.serial());
        let present = match &*value {
            GuestResidentValueV1::Resident(resident) => Some((Arc::clone(resident), true)),
            GuestResidentValueV1::Held(held) => held.upgrade().map(|held| (held, false)),
            GuestResidentValueV1::Absent => None,
        };
        if let Some((present, resident)) = present {
            self.ledger.hits.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            if !resident {
                if self.ledger.admit(&self.slot, prior) {
                    self.ledger.admitted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    *value = GuestResidentValueV1::Resident(Arc::clone(&present));
                } else if first_call {
                    context.retain(Arc::clone(&present) as Arc<dyn std::any::Any + Send + Sync>);
                }
            }
            return Ok(present);
        }
        let started = std::time::Instant::now();
        let compiled = Arc::new(compile().await?);
        self.ledger.compile_micros.fetch_add(u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX), std::sync::atomic::Ordering::AcqRel);
        self.ledger.compiles.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.slot.footprint_bytes.store(compiled.footprint_bytes(), std::sync::atomic::Ordering::Release);
        *value = if self.ledger.admit(&self.slot, prior) {
            self.ledger.admitted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            GuestResidentValueV1::Resident(Arc::clone(&compiled))
        } else {
            self.ledger.bypassed.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            context.retain(Arc::clone(&compiled) as Arc<dyn std::any::Any + Send + Sync>);
            GuestResidentValueV1::Held(Arc::downgrade(&compiled))
        };
        Ok(compiled)
    }

    /// 📏️ Charges `value` what it holds now ([`GuestResidentFootprintV1`]) after a call of `context`'s operation used it.
    /// A resident value that grew past what the budget has left stays resident only when its uses before this operation
    /// outnumber those of every least recently used unheld value it would release — which are then released — and
    /// otherwise serves the operations holding it (this one among them) and is dropped with the last of them. A held value
    /// is charged its growth when it is next admitted; a value the slot no longer holds changes nothing.
    pub async fn remeasure(&self, value: &Arc<T>, context: &OperationContext<'_>) {
        let footprint = value.footprint_bytes();
        if self.slot.footprint_bytes.load(std::sync::atomic::Ordering::Acquire) == footprint {
            return;
        }
        let mut held = self.slot.value.lock().await;
        let resident = match &*held {
            GuestResidentValueV1::Resident(resident) if Arc::ptr_eq(resident, value) => true,
            GuestResidentValueV1::Held(weak) if std::ptr::eq(weak.as_ptr(), Arc::as_ptr(value)) => false,
            GuestResidentValueV1::Resident(_) | GuestResidentValueV1::Held(_) | GuestResidentValueV1::Absent => return,
        };
        self.slot.footprint_bytes.store(footprint, std::sync::atomic::Ordering::Release);
        if !resident || self.ledger.admit(&self.slot, self.slot.prior_uses.load(std::sync::atomic::Ordering::Acquire)) {
            return;
        }
        *held = GuestResidentValueV1::Held(Arc::downgrade(value));
        context.retain(Arc::clone(value) as Arc<dyn std::any::Any + Send + Sync>);
        self.ledger.released.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

impl<T> GuestResidencyV1<T> {
    /// 📏️ Whether a compiled value stays resident now (not merely held by a running operation).
    #[cfg(any(test, feature = "testkit"))]
    pub fn is_resident(&self) -> bool {
        self.slot.charged()
    }
}

#[cfg(any(test, feature = "testkit"))]
impl GuestResidentFootprintV1 for Vec<u8> {
    /// 📏️ A byte-charged test value holds only the bytes it was registered with.
    fn footprint_bytes(&self) -> u64 {
        0
    }
}
