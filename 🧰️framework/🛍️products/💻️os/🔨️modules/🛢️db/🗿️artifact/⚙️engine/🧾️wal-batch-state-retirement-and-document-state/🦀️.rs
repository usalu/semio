//! 🧾️ WAL batch, state retirement and document state: the WAL batch helpers, the state retirement reservations and `DocumentState`. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️State
pub async fn admit_wal_bytes(source: Vec<u8>, maximum: u64, control: &mut db_wal_log::WalCursorControl) -> Result<db_wal_log::WalBytes, DbError> {
    let source = if source.len() as u64 <= maximum && source.capacity() as u64 > maximum { source.into_boxed_slice().into_vec() } else { source };
    match db_wal_log::WalBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            control.grant()?;
            let _ = rejected.close_step()?;
            Err(rejected.into_error())
        }
    }
}

pub async fn push_wal_record(records: &mut db_wal_log::WalRecordBatch, record: db_wal_log::WalRecord, control: &mut db_wal_log::WalCursorControl) -> Result<(), DbError> {
    match records.push(record) {
        Ok(()) => Ok(()),
        Err(mut record) => {
            control.grant()?;
            let _ = record.close_step()?;
            drop(record);
            Err(DbError::LimitExceeded("db_artifact fixed wal record batch"))
        }
    }
}

pub async fn close_wal_record_batch(records: &mut db_wal_log::WalRecordBatch) -> Result<(), DbError> {
    let mut control = db_wal_log::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    while !records.terminal_is_empty() {
        control.grant()?;
        if !records.close_step()? && !records.terminal_is_empty() {
            return Err(DbError::Internal("WAL record batch retirement stalled".to_string()));
        }
        semio_framework_async::yield_once().await;
    }
    Ok(())
}

//#region 🔖️StateRetirement
pub(super) const ARTIFACT_STATE_RETIREMENT_SLOTS: usize = 64;

/// 🧹️ Persists one rejected staging graph and advances exactly one refusal,
/// source, slot, page, or text owner for each maintenance grant.
pub(super) struct ArtifactStateRetirementCursor {
    pub rejected: Option<db_state::StateEntryRejected>,
    staged: [Option<db_state::StateEntry>; 64],
    pub phase: u8,
    pub slot: u8,
    pub retirement: Option<ArtifactStateRetirementReservation>,
}

impl ArtifactStateRetirementCursor {
    pub fn rejected(retirement: ArtifactStateRetirementReservation, rejected: db_state::StateEntryRejected, staged: [Option<db_state::StateEntry>; 64]) -> Self {
        Self { rejected: Some(rejected), staged, phase: 0, slot: 0, retirement: Some(retirement) }
    }

    pub fn entry(retirement: ArtifactStateRetirementReservation, entry: db_state::StateEntry) -> Self {
        let mut staged = std::array::from_fn(|_| None);
        staged[0] = Some(entry);
        Self { rejected: None, staged, phase: 2, slot: 0, retirement: Some(retirement) }
    }

    fn staged(retirement: ArtifactStateRetirementReservation, staged: [Option<db_state::StateEntry>; 64]) -> Self {
        Self { rejected: None, staged, phase: 2, slot: 0, retirement: Some(retirement) }
    }

    pub(super) fn empty() -> Self {
        Self { rejected: None, staged: std::array::from_fn(|_| None), phase: 3, slot: 64, retirement: None }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1)?;
        control.grant()?;
        match self.phase {
            0 => {
                let rejected = self.rejected.as_mut().ok_or_else(|| DbError::Internal("artifact retirement lost refusal owner".to_string()))?;
                if !rejected.close_step()? {
                    self.phase = 1;
                }
                Ok(true)
            }
            1 => {
                let source = self.rejected.take().and_then(db_state::StateEntryRejected::into_source);
                drop(source);
                self.phase = 2;
                Ok(true)
            }
            2 => {
                let Some(entry) = self.staged.get_mut(self.slot as usize) else {
                    self.phase = 3;
                    release_artifact_state_retirement(&mut self.retirement);
                    return Ok(true);
                };
                if let Some(entry) = entry.as_mut() {
                    if entry.close_step()? {
                        return Ok(true);
                    }
                }
                *entry = None;
                self.slot += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == 3 && self.rejected.is_none() && self.staged.iter().all(Option::is_none)
    }
}

pub(super) static ARTIFACT_STATE_RETIREMENT: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);

pub(super) static ARTIFACT_STATE_RETIREMENT_OVERFLOW: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);

pub(super) static ARTIFACT_STATE_RETIREMENT_QUARANTINE: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);

pub(super) static ARTIFACT_STATE_RETIREMENT_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];

pub(super) static ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy)]
struct ArtifactStateRetirementReservation {
    pub(super) tier: u8,
    pub index: u8,
}

pub(super) fn reserve_artifact_state_retirement() -> Option<ArtifactStateRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..ARTIFACT_STATE_RETIREMENT_SLOTS as u8 {
            let bit = 1u64 << index;
            if ARTIFACT_STATE_RETIREMENT_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(ArtifactStateRetirementReservation { tier, index });
            }
            ARTIFACT_STATE_RETIREMENT_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

pub(super) fn release_artifact_state_retirement(reservation: &mut Option<ArtifactStateRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        ARTIFACT_STATE_RETIREMENT_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn release_artifact_state_retirements(reservations: &mut [Option<ArtifactStateRetirementReservation>; 64]) {
    for reservation in reservations {
        release_artifact_state_retirement(reservation);
    }
}

fn artifact_state_vacant_retirement_slot(tier: usize, slots: &[Option<ArtifactStateRetirementCursor>]) -> Option<usize> {
    let reserved = ARTIFACT_STATE_RETIREMENT_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_artifact_state_owner(owner: ArtifactStateRetirementCursor) {
    let reservation = owner.retirement.unwrap_or(ArtifactStateRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

#[cfg(test)]
pub(super) fn retire_artifact_state_owner(owner: ArtifactStateRetirementCursor) -> Result<(), ArtifactStateRetirementCursor> {
    if owner.retirement.is_some() {
        install_reserved_artifact_state_owner(owner);
        return Ok(());
    }
    let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = artifact_state_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = artifact_state_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = artifact_state_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

/// 🧹️ Advances one parked state retirement — a replaced or removed value, or a refused
/// staging graph. Every state apply drives it until nothing is parked, so a replaced value's pages
/// and its I/O operation never outlive the write that replaced it; parked owners used to wait for a
/// maintenance pass nothing ran, until the process ran out of I/O operations and refused every write.
pub fn artifact_state_retirement_maintenance_step() -> Result<bool, DbError> {
    let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = artifact_state_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("artifact quarantine retirement changed owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = artifact_state_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("artifact overflow retirement changed owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("artifact retirement changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for ArtifactStateRetirementCursor {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            install_reserved_artifact_state_owner(std::mem::replace(self, Self::empty()));
        }
    }
}
//#endregion 🔖️StateRetirement

/// 🏗️ A document's materialized state: a flat `db_state::PMap` from path to raw value
/// bytes, plus a per-path last-writer map for `submit`'s local, path-granular conflict detection
/// (see `🔖️Conflict`'s doc on why this stays local rather than `db_conflict`-backed). `values` uses
/// `PMap` (not a mutable `HashMap`) specifically so `content_hash` — the `Frontier.chain_hash`
/// source — is a real content-addressed digest of the whole state, not an incidental byte count,
/// and so `PMap::iter` gives `snapshot_now`/`query` a cheap, complete enumeration.
pub(super) struct DocumentState {
    pub(super) values: db_state::RetainedStateMap,
    last_writer: db_state::PMap<String, protocol::MutationId>,
}

/// 🗺️ One envelope planned by `ArtifactEngine::plan_one` and not applied yet: its path entries with every value
/// already encoded as the state stores it (`None` deletes), the regions it touches and the conflicts it meets.
pub(super) struct PlannedEntries {
    pub entries: Vec<(String, Option<Vec<u8>>)>,
    pub touched: db_state::TouchedSet,
    pub conflicts: Vec<ConflictRecord>,
}

impl DocumentState {
    // 🚫️async: E1 pure constructor, `db_state::PMap::new` is sync — see R9
    pub fn new() -> DocumentState {
        DocumentState { values: db_state::RetainedStateMap::new(), last_writer: db_state::PMap::new() }
    }

    pub fn get(&self, path: &str) -> Option<&db_io_core::DbIoPages> {
        self.values.get(path)
    }

    pub(super) async fn content_hash(&self) -> Result<ContentHash, DbError> {
        let mut control = db_state::StateCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        self.values.content_hash(&mut control).await
    }

    /// 🧭️ What applying one envelope's flattened path-value entries would touch and conflict with, decided
    /// without applying anything: a path's last writer is `written`'s (an earlier envelope of the same batch) or else
    /// the state's own, and a conflict is a last writer that is neither `mutation_id` itself nor a declared
    /// `dependencies` member. `written` gains this envelope's paths.
    pub(super) fn plan_entries(&self, mutation_id: &protocol::MutationId, dependencies: &[protocol::MutationId], entries: &[(String, Option<DslValue>)], written: &mut HashMap<String, protocol::MutationId>) -> Result<(db_state::TouchedSet, Vec<ConflictRecord>), DbError> {
        check_len(entries.len() as u64, 64, "db_artifact::retained_state_mutations")?;
        let mut touched = db_state::TouchedSet::new();
        let mut conflicts = Vec::new();
        for (path, _) in entries {
            if let Some(previous_writer) = written.get(path).or_else(|| self.last_writer.get(path)) {
                if previous_writer != mutation_id && !dependencies.contains(previous_writer) {
                    conflicts.push(ConflictRecord { command_id: mutation_id.clone(), conflicting_with: previous_writer.clone(), path: path.clone() });
                }
            }
            touched.record(db_state::TouchedRegion::write(path.clone()));
            written.insert(path.clone(), mutation_id.clone());
        }
        Ok((touched, conflicts))
    }

    /// ✍️ Applies one planned envelope's entries ([`Self::plan_entries`]): every value, already encoded, set at
    /// its path, every `None` a deletion, and `mutation_id` recorded as each path's last writer.
    pub(super) async fn apply_entries(&mut self, mutation_id: &protocol::MutationId, entries: Vec<(String, Option<Vec<u8>>)>) -> Result<(), DbError> {
        while artifact_state_retirement_maintenance_step()? {}
        if entries.is_empty() {
            return Ok(());
        }
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let mut staged: [Option<db_state::StateEntry>; 64] = std::array::from_fn(|_| None);
        let mut retirements: [Option<ArtifactStateRetirementReservation>; 64] = [None; 64];
        for index in 0..entries.len() {
            let Some(reservation) = reserve_artifact_state_retirement() else {
                release_artifact_state_retirements(&mut retirements);
                return Err(DbError::Unavailable("artifact state retirement pressure refused admission".to_string()));
            };
            retirements[index] = Some(reservation);
        }
        let mut paths = Vec::with_capacity(entries.len());
        for (index, (path, value)) in entries.into_iter().enumerate() {
            if let Some(bytes) = value {
                match db_state::StateEntry::try_admit(&path, bytes, MAX_STATE_PAGE_VALUE_BYTES, &mut control).await {
                    Ok(entry) => staged[index] = Some(entry),
                    Err(rejected) => {
                        let error = format!("retained state admission failed: {}", rejected.error());
                        let error = if matches!(rejected.error(), DbError::Unavailable(_)) { DbError::Unavailable(error) } else { DbError::InvalidArgument(error) };
                        let retirement = retirements[index].take().ok_or_else(|| DbError::Internal("retained state refusal lost retirement preflight".to_string()))?;
                        release_artifact_state_retirements(&mut retirements);
                        install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::rejected(retirement, rejected, staged));
                        while artifact_state_retirement_maintenance_step()? {}
                        return Err(error);
                    }
                }
            }
            paths.push(path);
        }
        for (index, path) in paths.into_iter().enumerate() {
            let retirement = retirements[index].take().ok_or_else(|| DbError::Internal("retained state staging lost retirement admission".to_string()))?;
            match staged[index].take() {
                Some(entry) => {
                    let replaced = match self.values.insert(entry) {
                        Ok(replaced) => replaced,
                        Err(rejected) => {
                            staged[index] = Some(rejected);
                            release_artifact_state_retirements(&mut retirements);
                            install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::staged(retirement, staged));
                            return Err(DbError::LimitExceeded("retained state entries"));
                        }
                    };
                    match replaced {
                        Some(replaced) => install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::entry(retirement, replaced)),
                        None => release_artifact_state_retirement(&mut Some(retirement)),
                    }
                }
                None => match self.values.remove(&path) {
                    Some(removed) => install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::entry(retirement, removed)),
                    None => release_artifact_state_retirement(&mut Some(retirement)),
                },
            }
            self.last_writer = self.last_writer.insert(path, mutation_id.clone());
        }
        while artifact_state_retirement_maintenance_step()? {}
        Ok(())
    }
}
//#endregion 🔖️State
//#endregion 🔖️Ids
