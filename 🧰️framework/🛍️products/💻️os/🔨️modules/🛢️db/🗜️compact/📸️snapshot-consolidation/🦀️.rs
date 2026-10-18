//! 📸️ Snapshot consolidation: the retained pages, their retirement and the snapshot consolidator. See [`super`].

use super::*;

//#region 🔖️SnapshotConsolidation
pub(super) const COMPACTION_RETAINED_PAGE_OWNERS: usize = 64;

pub(super) const COMPACTION_RETIREMENT_SLOTS: usize = 64;

/// 🧱️ Fixed, page-credit-witnessed snapshot consolidation owner set.
pub(super) struct CompactionRetainedPages {
    pub(super) pages: [Option<db_state::Page>; COMPACTION_RETAINED_PAGE_OWNERS],
    credits: [u8; COMPACTION_RETAINED_PAGE_OWNERS],
    pub(super) len: u8,
    pub(super) retirement: Option<CompactionRetirementReservation>,
}

impl CompactionRetainedPages {
    pub(super) fn new() -> Self {
        Self { pages: std::array::from_fn(|_| None), credits: [0; COMPACTION_RETAINED_PAGE_OWNERS], len: 0, retirement: None }
    }

    pub(super) fn contains(&self, hash: ContentHash) -> bool {
        self.pages[..self.len()].iter().flatten().any(|page| page.hash == hash)
    }

    pub(super) fn preflight_push(&mut self) -> Result<(), DbError> {
        if self.len() == COMPACTION_RETAINED_PAGE_OWNERS {
            return Err(DbError::LimitExceeded("db_compact fixed retained snapshot pages"));
        }
        if self.len == 0 && self.retirement.is_none() {
            self.retirement = Some(reserve_compaction_retirement().ok_or_else(|| DbError::Unavailable("compaction page retirement pressure refused admission".to_string()))?);
        }
        Ok(())
    }

    pub(super) fn push_preflighted(&mut self, page: db_state::Page) {
        let index = self.len();
        self.credits[index] = page.pages().page_count();
        self.pages[index] = Some(page);
        self.len += 1;
    }

    pub(super) fn try_push(&mut self, page: db_state::Page) -> Result<(), db_state::Page> {
        if self.preflight_push().is_err() {
            return Err(page);
        }
        self.push_preflighted(page);
        Ok(())
    }

    pub(super) fn len(&self) -> usize {
        self.len as usize
    }

    pub(super) fn slots(&self) -> &[Option<db_state::Page>] {
        &self.pages
    }

    pub(super) fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let page = self.pages[index].as_mut().ok_or_else(|| DbError::Internal("compaction close lost retained page".to_string()))?;
        if let Some(returned) = page.close_step()? {
            if returned != 0 {
                self.credits[index] = self.credits[index].checked_sub(1).ok_or_else(|| DbError::Internal("compaction page credit returned twice".to_string()))?;
            }
            return Ok(true);
        }
        if self.credits[index] != 0 || !page.terminal_is_empty() {
            return Err(DbError::Internal("compaction page reached a false empty witness".to_string()));
        }
        self.pages[index] = None;
        self.len -= 1;
        if self.len == 0 {
            release_compaction_retirement(&mut self.retirement);
        }
        Ok(true)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.credits.iter().all(|credit| *credit == 0) && self.pages.iter().all(Option::is_none)
    }
}

pub(super) static COMPACTION_PAGE_RETIREMENT: std::sync::Mutex<[Option<CompactionRetainedPages>; COMPACTION_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; COMPACTION_RETIREMENT_SLOTS]);

pub(super) static COMPACTION_PAGE_RETIREMENT_OVERFLOW: std::sync::Mutex<[Option<CompactionRetainedPages>; COMPACTION_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; COMPACTION_RETIREMENT_SLOTS]);

pub(super) static COMPACTION_PAGE_RETIREMENT_QUARANTINE: std::sync::Mutex<[Option<CompactionRetainedPages>; COMPACTION_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; COMPACTION_RETIREMENT_SLOTS]);

static COMPACTION_RETIREMENT_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];

pub(super) static COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy)]
struct CompactionRetirementReservation {
    pub(super) tier: u8,
    pub(super) index: u8,
}

fn reserve_compaction_retirement() -> Option<CompactionRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..COMPACTION_RETIREMENT_SLOTS as u8 {
            let bit = 1u64 << index;
            if COMPACTION_RETIREMENT_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(CompactionRetirementReservation { tier, index });
            }
            COMPACTION_RETIREMENT_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

fn release_compaction_retirement(reservation: &mut Option<CompactionRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        COMPACTION_RETIREMENT_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn compaction_vacant_retirement_slot(tier: usize, slots: &[Option<CompactionRetainedPages>]) -> Option<usize> {
    let reserved = COMPACTION_RETIREMENT_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_compaction_pages(owner: CompactionRetainedPages) {
    let reservation = owner.retirement.unwrap_or(CompactionRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

#[cfg(test)]
pub(super) fn retire_compaction_pages(owner: CompactionRetainedPages) -> Result<(), CompactionRetainedPages> {
    if owner.retirement.is_some() {
        install_reserved_compaction_pages(owner);
        return Ok(());
    }
    let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = compaction_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = compaction_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = compaction_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

pub fn compaction_page_maintenance_step() -> Result<bool, DbError> {
    let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = compaction_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("compaction quarantine retirement changed page owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = compaction_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("compaction overflow retirement changed page owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("compaction retirement changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for CompactionRetainedPages {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            release_compaction_retirement(&mut self.retirement);
            return;
        }
        install_reserved_compaction_pages(std::mem::replace(self, Self::new()));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
pub(super) enum CompactionCloseExit {
    Running,
    Closed,
    Fault,
}

/// 🛰️ Mounted close state that advances one retained page opportunity per poll.
#[cfg(test)]
pub(super) struct MountedCompactionPageClose<'owner> {
    pub(super) owner: &'owner mut CompactionRetainedPages,
    pub(super) exit: CompactionCloseExit,
}

#[cfg(test)]
impl<'owner> MountedCompactionPageClose<'owner> {
    pub(super) fn new(owner: &'owner mut CompactionRetainedPages) -> Self {
        Self { owner, exit: CompactionCloseExit::Running }
    }
}

#[cfg(test)]
impl Future for MountedCompactionPageClose<'_> {
    type Output = Result<CompactionCloseExit, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.owner.close_step() {
            Ok(true) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(false) => {
                self.exit = CompactionCloseExit::Closed;
                std::task::Poll::Ready(Ok(self.exit))
            }
            Err(error) => {
                self.exit = CompactionCloseExit::Fault;
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

/// 🌳️ Walks the snapshot chain from `through_generation` back to its full-baseline root,
/// returning the latest generation's own descriptor plus every page introduced anywhere in the
/// chain, deduplicated by content hash — `SnapshotConsolidator::consolidate`'s input.
#[cfg(test)]
async fn collect_chain_pages<S: db_io_core::SnapshotStorage>(
    manager: &db_snapshot::SnapshotManager<'_, S>,
    document: &ArtifactId,
    through_generation: u64,
    budget: &CompactionBudget,
) -> Result<(db_snapshot::SnapshotDescriptor, CompactionRetainedPages), DbError> {
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut cursor = manager.chain_cursor(document, through_generation, control);
    let latest_descriptor = cursor.latest_descriptor().await?;
    let mut descriptor = latest_descriptor.clone();
    let mut pages = CompactionRetainedPages::new();
    let mut generations_walked = 0u64;
    loop {
        generations_walked += 1;
        check_len(generations_walked, budget.max_snapshot_generations, "db_compact::snapshot_chain_depth")?;
        for hash in descriptor.new_pages.iter().copied() {
            if !pages.contains(hash) {
                pages.preflight_push()?;
                let bytes = cursor.read_page(hash).await?;
                let page = db_state::Page::try_from_pages(bytes).await?;
                pages.push_preflighted(page);
            }
        }
        match descriptor.parent_generation {
            Some(parent) => descriptor = cursor.descriptor(parent).await?,
            None => break,
        }
    }
    let _ = cursor.close_step()?;
    Ok((latest_descriptor, pages))
}

/// 🧑️‍💼️ Rolls up a document's incremental snapshot chain into a fresh, self-sufficient
/// full baseline — the responsibility `db_snapshot`'s own module doc explicitly defers to this
/// crate (see that crate's "Scope boundary" note).
#[cfg(test)]
pub struct SnapshotConsolidator<'storage, S: db_io_core::SnapshotStorage> {
    pub(super) manager: db_snapshot::SnapshotManager<'storage, S>,
}

#[cfg(test)]
impl<'storage, S: db_io_core::SnapshotStorage> SnapshotConsolidator<'storage, S> {
    pub async fn new(storage: &'storage S) -> SnapshotConsolidator<'storage, S> {
        SnapshotConsolidator { manager: db_snapshot::SnapshotManager::new(storage).await }
    }

    /// 🧵️ Publishes a new full-baseline generation carrying the union of every page from
    /// the chain's root through `through_generation` (deduplicated by content hash), with the
    /// latest generation's own frontier/provenance/`roots`. Returns the new generation number; the
    /// caller is responsible for `retain_from` afterward once satisfied nothing still needs an old
    /// generation (e.g. an in-flight replica read — this method itself never prunes).
    ///
    /// 🎯️ Scope boundary: this is a page-union roll-up, not a reachability GC — a page that became
    /// unreferenced somewhere along the chain is still carried into the new baseline (harmless:
    /// `roots` still resolves correctly, the baseline is just not maximally compact). Pruning truly
    /// unreachable pages needs the application-level page-tree structure this crate doesn't have
    /// (see module doc's "diff collapse" scope note for the same underlying constraint) and is left
    /// as this crate's deliberate extension seam.
    pub async fn consolidate(&self, document: &ArtifactId, through_generation: u64, budget: &CompactionBudget) -> Result<u64, DbError> {
        let (latest, mut pages) = collect_chain_pages(&self.manager, document, through_generation, budget).await?;
        let body = db_snapshot::SnapshotBody {
            head_seq: latest.head_seq,
            commit_seq: latest.commit_seq,
            epoch: latest.epoch,
            chain_hash: latest.chain_hash,
            protocol_version: latest.protocol_version,
            base_pack_hash: latest.base_pack_hash,
            roots: latest.roots,
            created_at_ms: latest.created_at_ms,
        };
        let publication = self.manager.publish_retained(document, db_snapshot::SnapshotOrigin::FullBaseline, pages.slots(), pages.len(), body).await;
        let close = MountedCompactionPageClose::new(&mut pages).await;
        match (publication, close) {
            (Ok(generation), Ok(CompactionCloseExit::Closed)) => Ok(generation),
            (Err(error), _) | (_, Err(error)) => Err(error),
            _ => Err(DbError::Internal("compaction close produced a false exit witness".to_string())),
        }
    }

    /// 🗑️ Forwards to `db_snapshot::SnapshotManager::retain_from` — `floor_generation` must
    /// itself be a full baseline (typically the generation `consolidate` just returned).
    pub async fn retain_from(&self, document: &ArtifactId, floor_generation: u64) -> Result<(), DbError> {
        self.manager.retain_from(document, floor_generation).await
    }
}
//#endregion 🔖️SnapshotConsolidation
