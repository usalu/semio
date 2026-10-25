//! 🧾️ Limits and rows: the query limits, the retained row sets with their retirement reservations and maintenance step. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🛡️ Query-side ceilings, checked via `check_len` before allocating the next row
/// or byte, mirroring the family-wide "validate before allocating" invariant. `max_result_bytes`
/// defaults to `DbLimits::default().max_query_bytes` — the same budget the mailbox layer
/// already reserves for one query's wire payload, kept as a single source of truth.
#[derive(Clone, Debug)]
pub struct QueryLimits {
    pub max_scan_rows: u64,
    pub max_result_rows: u64,
    pub max_result_bytes: u64,
}

impl Default for QueryLimits {
    fn default() -> Self {
        QueryLimits { max_scan_rows: 1_000_000, max_result_rows: 10_000, max_result_bytes: DbLimits::default().max_query_bytes }
    }
}

/// 📐️ A conservative byte-size estimate of `value` — sums scalar widths and container
/// element/key bytes. Only used to enforce `QueryLimits::max_result_bytes`, so slight
/// under/over-estimation (e.g. `Value` enum tag overhead is ignored) is acceptable; being cheap and
/// allocation-free is what matters.
// 🚫️async: E1 pure, self-recursive accessor consumed by a sync Iterator::map — see R9
pub(super) fn value_byte_estimate(value: &Value) -> u64 {
    match value {
        Value::Null | Value::Bool(_) => 1,
        Value::Int(_) | Value::Float(_) => 8,
        Value::Text(text) => text.len() as u64,
        Value::Bytes(bytes) => bytes.len() as u64,
        Value::List(items) => items.iter().map(value_byte_estimate).sum(),
        Value::Map(map) => map.iter().map(|(key, val)| key.len() as u64 + value_byte_estimate(val)).sum(),
    }
}
//#endregion 🔖️Limits

//#region 🔖️QuerySource
/// 🆔️ An opaque per-document row identifier. `u64` (not a `String`) to match `db_index`'s
/// own `doc_ref` convention for full-text/touched-region postings (see `db_index::FullTextIndex`'s
/// doc) — a `FullTextLookup`'s postings and a `QuerySource`'s row ids are meant to be the same
/// space, so pushdown candidates resolve back into `QuerySource::get` directly.
/// 🧪️ `Serialize`/`Deserialize` — same reason as `Value`'s: `RowId` is `QueryResultField::Key`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct RowId(pub u64);

#[derive(Debug)]
pub struct QueryRow {
    pub id: RowId,
    pub value: Value,
}

impl QueryRow {
    pub fn new(id: RowId, value: Value) -> Self {
        Self { id, value }
    }

    pub fn id(&self) -> RowId {
        self.id
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn into_parts(self) -> (RowId, Value) {
        (self.id, self.value)
    }
}

pub(super) const QUERY_ROW_SLOTS: usize = 64;

pub(super) const QUERY_RETIRED_ROW_SETS: usize = 64;

#[derive(Debug)]
pub struct QueryRows {
    pub slots: [Option<QueryRow>; QUERY_ROW_SLOTS],
    pub len: u8,
    pub retirement: Option<QueryRowsRetirementReservation>,
}

impl QueryRows {
    pub fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), len: 0, retirement: None }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 🛂️ Reserves the exact retirement authority before a caller acquires its next row.
    pub fn preflight_push(&mut self) -> Result<(), DbError> {
        if self.len() == QUERY_ROW_SLOTS {
            return Err(DbError::LimitExceeded("query row slots"));
        }
        if self.len == 0 && self.retirement.is_none() {
            self.retirement = Some(reserve_query_rows_retirement().ok_or_else(|| DbError::Unavailable("query row retirement pressure refused admission".to_string()))?);
        }
        Ok(())
    }

    /// 📥️ Installs one row after `preflight_push` preserved its exact refusal boundary.
    pub fn push_preflighted(&mut self, row: QueryRow) {
        let index = self.len();
        self.slots[index] = Some(row);
        self.len += 1;
    }

    pub fn push(&mut self, row: QueryRow) -> Result<(), QueryRow> {
        if self.preflight_push().is_err() {
            return Err(row);
        }
        self.push_preflighted(row);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &QueryRow> {
        self.slots[..self.len()].iter().flatten()
    }

    pub fn get(&self, index: usize) -> Option<&QueryRow> {
        self.slots.get(index).and_then(Option::as_ref)
    }

    pub(super) fn sort_by(&mut self, mut compare: impl FnMut(&QueryRow, &QueryRow) -> Ordering) {
        for index in 1..self.len() {
            let mut cursor = index;
            while cursor > 0 {
                let order = compare(self.slots[cursor - 1].as_ref().expect("live query row slot"), self.slots[cursor].as_ref().expect("live query row slot"));
                if order != Ordering::Greater {
                    break;
                }
                self.slots.swap(cursor - 1, cursor);
                cursor -= 1;
            }
        }
    }

    pub fn take(&mut self, index: usize) -> Option<QueryRow> {
        if index >= self.len() {
            return None;
        }
        let row = self.slots[index].take();
        for cursor in index + 1..self.len() {
            self.slots[cursor - 1] = self.slots[cursor].take();
        }
        self.len -= 1;
        if self.len == 0 {
            release_query_rows_retirement(&mut self.retirement);
        }
        row
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let row = self.slots[index].as_mut().ok_or_else(|| DbError::Internal("query close lost row".to_string()))?;
        if value_close_step(&mut row.value)? {
            return Ok(true);
        }
        self.slots[index] = None;
        self.len -= 1;
        if self.len == 0 {
            release_query_rows_retirement(&mut self.retirement);
        }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

impl Default for QueryRows {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) static QUERY_RETIRED_ROWS: std::sync::Mutex<[Option<QueryRows>; QUERY_RETIRED_ROW_SETS]> = std::sync::Mutex::new([const { None }; QUERY_RETIRED_ROW_SETS]);

pub(super) static QUERY_RETIRED_ROWS_OVERFLOW: std::sync::Mutex<[Option<QueryRows>; QUERY_RETIRED_ROW_SETS]> = std::sync::Mutex::new([const { None }; QUERY_RETIRED_ROW_SETS]);

pub(super) static QUERY_RETIRED_ROWS_QUARANTINE: std::sync::Mutex<[Option<QueryRows>; QUERY_RETIRED_ROW_SETS]> = std::sync::Mutex::new([const { None }; QUERY_RETIRED_ROW_SETS]);

static QUERY_RETIRED_ROWS_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];

pub(super) static QUERY_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, Debug)]
pub(super) struct QueryRowsRetirementReservation {
    pub(super) tier: u8,
    pub(super) index: u8,
}

pub(super) fn reserve_query_rows_retirement() -> Option<QueryRowsRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..QUERY_RETIRED_ROW_SETS as u8 {
            let bit = 1u64 << index;
            if QUERY_RETIRED_ROWS_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(QueryRowsRetirementReservation { tier, index });
            }
            QUERY_RETIRED_ROWS_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

fn release_query_rows_retirement(reservation: &mut Option<QueryRowsRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        QUERY_RETIRED_ROWS_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn query_rows_vacant_retirement_slot(tier: usize, slots: &[Option<QueryRows>]) -> Option<usize> {
    let reserved = QUERY_RETIRED_ROWS_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_query_rows(owner: QueryRows) {
    let reservation = owner.retirement.unwrap_or(QueryRowsRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

#[cfg(test)]
pub(super) fn retire_query_rows(owner: QueryRows) -> Result<(), QueryRows> {
    if owner.retirement.is_some() {
        install_reserved_query_rows(owner);
        return Ok(());
    }
    let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = query_rows_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = query_rows_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = query_rows_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

pub(super) fn retire_query_row_at(rows: &mut QueryRows, index: usize) -> Result<(), DbError> {
    let retirement = reserve_query_rows_retirement().ok_or_else(|| DbError::Unavailable("query row retirement pressure refused removal".to_string()))?;
    let row = rows.take(index).ok_or_else(|| DbError::Internal("query row retirement lost preflighted row".to_string()))?;
    let mut retired = QueryRows::new();
    retired.retirement = Some(retirement);
    retired.slots[0] = Some(row);
    retired.len = 1;
    install_reserved_query_rows(retired);
    Ok(())
}

pub(super) fn retire_query_row_with_reservation(row: QueryRow, retirement: QueryRowsRetirementReservation) {
    let mut rows = QueryRows::new();
    rows.retirement = Some(retirement);
    rows.slots[0] = Some(row);
    rows.len = 1;
    install_reserved_query_rows(rows);
}

pub fn query_rows_maintenance_step() -> Result<bool, DbError> {
    let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = query_rows_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("query quarantine retirement changed row owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = query_rows_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("query overflow retirement changed row owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("query retired row set changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for QueryRows {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            release_query_rows_retirement(&mut self.retirement);
            return;
        }
        install_reserved_query_rows(std::mem::replace(self, Self::new()));
    }
}

fn value_close_step(value: &mut Value) -> Result<bool, DbError> {
    match value {
        Value::Bytes(bytes) => Ok(bytes.close_step()?.is_some()),
        Value::List(items) => {
            let Some(item) = items.last_mut() else { return Ok(false) };
            if value_close_step(item)? {
                return Ok(true);
            }
            items.pop();
            Ok(true)
        }
        Value::Map(map) => {
            let Some((key, mut item)) = map.pop_last() else { return Ok(false) };
            if value_close_step(&mut item)? {
                map.insert(key, item);
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}
//#endregion 🔖️QuerySource
