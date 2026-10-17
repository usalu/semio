//! 📄️ Numerical checkpoint pages: identity, mounted scalar slots, page cursors, owner advance and restore.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalCheckpointIdentity {
    pub operation: u64,
    pub revision: u64,
    pub generation: u64,
    pub seed: u64,
}

impl NumericalCheckpointIdentity {
    pub fn from_operation(operation: Operation) -> Self {
        Self { operation: operation.operation.0, revision: operation.base_revision.0, generation: operation.generation.0, seed: operation.seed }
    }

    pub fn matches(self, operation: Operation) -> bool {
        self == Self::from_operation(operation)
    }
}
pub const NUMERICAL_OWNER_PAGE_BYTES: usize = 16 * 1024;
pub const MOUNTED_SCALAR_SLOTS: usize = 768;

/// 📊️ A mounted scalar admission or update exceeds its available slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedScalarFault {
    Capacity { requested: usize, maximum: usize },
    Index { index: usize, len: usize },
}

/// 🧮 Fixed mounted scalar owner with one admission, write, update, or close action per turn.
pub struct MountedScalarSlots {
    pub values: [f64; MOUNTED_SCALAR_SLOTS],
    pub admitted: usize,
    pub len: usize,
}

impl MountedScalarSlots {
    pub fn new() -> Self {
        Self { values: [0.0; MOUNTED_SCALAR_SLOTS], admitted: 0, len: 0 }
    }

    pub fn admit_one(&mut self, target: usize) -> Result<bool, MountedScalarFault> {
        if target > MOUNTED_SCALAR_SLOTS {
            return Err(MountedScalarFault::Capacity { requested: target, maximum: MOUNTED_SCALAR_SLOTS });
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn push(&mut self, value: f64) -> Result<(), f64> {
        if self.len == self.admitted {
            return Err(value);
        }
        self.values[self.len] = value;
        self.len += 1;
        Ok(())
    }

    pub fn add_at(&mut self, index: usize, value: f64) -> Result<(), MountedScalarFault> {
        if index >= self.len {
            return Err(MountedScalarFault::Index { index, len: self.len });
        }
        self.values[index] += value;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        (index < self.len).then(|| self.values[index])
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.admitted == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.len != 0 {
            self.len -= 1;
            self.values[self.len] = 0.0;
            return false;
        }
        if self.admitted != 0 {
            self.admitted -= 1;
            return false;
        }
        true
    }
}

impl Default for MountedScalarSlots {
    fn default() -> Self {
        Self::new()
    }
}
pub const NUMERICAL_CHECKPOINT_VERSION: u16 = 1;
pub const NUMERICAL_CHECKPOINT_HEADER_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalPageCursor {
    pub field: u16,
    pub owner: usize,
    pub item: usize,
}

impl NumericalPageCursor {
    pub fn new() -> Self {
        Self { field: 0, owner: 0, item: 0 }
    }
}

pub fn numerical_page_header(magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor) -> [u8; NUMERICAL_CHECKPOINT_HEADER_BYTES] {
    let mut bytes = [0; NUMERICAL_CHECKPOINT_HEADER_BYTES];
    bytes[..8].copy_from_slice(magic);
    bytes[8..10].copy_from_slice(&NUMERICAL_CHECKPOINT_VERSION.to_le_bytes());
    bytes[10..12].copy_from_slice(&kind.to_le_bytes());
    bytes[12..14].copy_from_slice(&cursor.field.to_le_bytes());
    bytes[16..24].copy_from_slice(&(cursor.owner as u64).to_le_bytes());
    bytes[24..32].copy_from_slice(&(cursor.item as u64).to_le_bytes());
    bytes
}

pub fn advance_numerical_page_header(writer: &mut RetainedJobPayloadWriter, magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if writer.staged_page_len() == Some(0) {
        writer.write_staged(&numerical_page_header(magic, kind, cursor))?;
        return Ok(true);
    }
    Ok(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalCheckpointFault {
    Cancelled,
    Stale,
    Version,
    Truncated,
    Field,
    Envelope,
    Admission,
}

pub fn write_numerical_entry(writer: &mut RetainedJobPayloadWriter, bytes: &[u8]) -> Result<bool, JobPayloadAdmissionFault> {
    if writer.staged_page_remaining() < bytes.len() {
        writer.commit_staged_page()?;
        return Ok(false);
    }
    writer.write_staged(bytes)?;
    Ok(true)
}

pub fn advance_owner_length(writer: &mut RetainedJobPayloadWriter, length: usize, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if cursor.owner == 0 {
        if !write_numerical_entry(writer, &(length as u64).to_le_bytes())? {
            return Ok(false);
        }
        cursor.owner = 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u32_owner(writer: &mut RetainedJobPayloadWriter, values: &[u32], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u64_owner(writer: &mut RetainedJobPayloadWriter, values: &[usize], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &(*value as u64).to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u64_values(writer: &mut RetainedJobPayloadWriter, values: &[u64], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_f64_owner(writer: &mut RetainedJobPayloadWriter, values: &[f64], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_bits().to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_paged_u32_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<u32, N>, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_paged_f64_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<f64, N>, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_bits().to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_pair_owner(writer: &mut RetainedJobPayloadWriter, values: &[(u32, f64)], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some((index, value)) = values.get(cursor.item) {
        let mut bytes = [0; 12];
        bytes[..4].copy_from_slice(&index.to_le_bytes());
        bytes[4..].copy_from_slice(&value.to_bits().to_le_bytes());
        if !write_numerical_entry(writer, &bytes)? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_matrix_owner(writer: &mut RetainedJobPayloadWriter, matrix: &MatD, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if cursor.owner == 0 {
        let mut bytes = [0; 24];
        bytes[..8].copy_from_slice(&(matrix.rows as u64).to_le_bytes());
        bytes[8..16].copy_from_slice(&(matrix.cols as u64).to_le_bytes());
        bytes[16..].copy_from_slice(&(matrix.data.len() as u64).to_le_bytes());
        if !write_numerical_entry(writer, &bytes)? {
            return Ok(false);
        }
        cursor.owner = 1;
        return Ok(false);
    }
    if let Some(value) = matrix.data.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_bits().to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}
pub struct NumericalPageView<'a> {
    pub kind: u16,
    pub field: u16,
    pub owner: usize,
    pub item: usize,
    pub bytes: &'a [u8],
}

pub(super) fn read_checkpoint_u16(bytes: &[u8], offset: usize) -> Result<u16, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset.checked_add(2).ok_or(NumericalCheckpointFault::Truncated)?).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

pub(super) fn read_checkpoint_u32(bytes: &[u8], offset: usize) -> Result<u32, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset.checked_add(4).ok_or(NumericalCheckpointFault::Truncated)?).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

pub fn read_checkpoint_u64(bytes: &[u8], offset: usize) -> Result<u64, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset.checked_add(8).ok_or(NumericalCheckpointFault::Truncated)?).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u64::from_le_bytes([value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7]]))
}

pub fn read_checkpoint_usize(bytes: &[u8], offset: usize) -> Result<usize, NumericalCheckpointFault> {
    usize::try_from(read_checkpoint_u64(bytes, offset)?).map_err(|_| NumericalCheckpointFault::Envelope)
}

pub fn parse_numerical_page<'a>(bytes: &'a [u8], magic: &[u8; 8]) -> Result<NumericalPageView<'a>, NumericalCheckpointFault> {
    if bytes.len() < NUMERICAL_CHECKPOINT_HEADER_BYTES || bytes.get(..8) != Some(magic.as_slice()) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if read_checkpoint_u16(bytes, 8)? != NUMERICAL_CHECKPOINT_VERSION {
        return Err(NumericalCheckpointFault::Version);
    }
    Ok(NumericalPageView {
        kind: read_checkpoint_u16(bytes, 10)?,
        field: read_checkpoint_u16(bytes, 12)?,
        owner: read_checkpoint_usize(bytes, 16)?,
        item: read_checkpoint_usize(bytes, 24)?,
        bytes: &bytes[NUMERICAL_CHECKPOINT_HEADER_BYTES..],
    })
}

pub fn declared_owner_length(page: &NumericalPageView<'_>, maximum: usize) -> Result<usize, NumericalCheckpointFault> {
    let length = read_checkpoint_usize(page.bytes, 0)?;
    (length <= maximum).then_some(length).ok_or(NumericalCheckpointFault::Envelope)
}

pub fn validate_restored_owner<T>(owner: &Vec<T>) -> Result<(), NumericalCheckpointFault> {
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(NumericalCheckpointFault::Envelope)?;
    (bytes <= NUMERICAL_OWNER_PAGE_BYTES).then_some(()).ok_or(NumericalCheckpointFault::Envelope)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalOwnerShape {
    pub length: usize,
    pub capacity: usize,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Default, Debug)]
pub struct NumericalOwnerRestoreCursor {
    pub shape: Option<NumericalOwnerShape>,
    pub next_item: usize,
    pub page_item: usize,
    pub page_owner: usize,
    pub page_values: usize,
    pub page_open: bool,
}

impl NumericalOwnerRestoreCursor {
    pub fn begin_page(&mut self, page: &NumericalPageView<'_>, maximum: usize, width: usize, matrix: bool) -> Result<NumericalOwnerShape, NumericalCheckpointFault> {
        if self.page_open {
            if page.item != self.page_item || page.owner != self.page_owner { return Err(NumericalCheckpointFault::Field); }
            return self.shape.ok_or(NumericalCheckpointFault::Field);
        }
        let prefix = if page.owner == 0 { if matrix { 24 } else { 8 } } else { 0 };
        let shape = match self.shape {
            Some(shape) if page.owner == 1 && page.item == self.next_item => shape,
            Some(_) => return Err(NumericalCheckpointFault::Field),
            None if page.owner == 0 && page.item == 0 => {
                if matrix {
                    let rows = read_checkpoint_usize(page.bytes, 0)?;
                    let cols = read_checkpoint_usize(page.bytes, 8)?;
                    let capacity = rows.checked_mul(cols).ok_or(NumericalCheckpointFault::Envelope)?;
                    let length = read_checkpoint_usize(page.bytes, 16)?;
                    if capacity > maximum || length > capacity { return Err(NumericalCheckpointFault::Envelope); }
                    NumericalOwnerShape { length, capacity, rows, cols }
                } else {
                    let length = declared_owner_length(page, maximum)?;
                    NumericalOwnerShape { length, capacity: length, rows: 0, cols: 0 }
                }
            }
            None => return Err(NumericalCheckpointFault::Field),
        };
        let remaining = shape.length.checked_sub(self.next_item).ok_or(NumericalCheckpointFault::Field)?;
        let page_values = remaining.min((semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - NUMERICAL_CHECKPOINT_HEADER_BYTES - prefix) / width);
        if page.bytes.len() != prefix + page_values * width { return Err(NumericalCheckpointFault::Truncated); }
        self.shape = Some(shape);
        self.page_item = page.item;
        self.page_owner = page.owner;
        self.page_values = page_values;
        self.page_open = true;
        Ok(shape)
    }

    pub fn finish_page(&mut self) {
        if self.shape.is_some_and(|shape| self.next_item == shape.length) { *self = Self::default(); }
        else { self.page_open = false; }
    }
}

pub trait NumericalRestoreScalar: Sized {
    const WIDTH: usize;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault>;
}

impl NumericalRestoreScalar for u32 {
    const WIDTH: usize = 4;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { read_checkpoint_u32(bytes, offset) }
}

impl NumericalRestoreScalar for usize {
    const WIDTH: usize = 8;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { read_checkpoint_usize(bytes, offset) }
}

impl NumericalRestoreScalar for f64 {
    const WIDTH: usize = 8;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { Ok(f64::from_bits(read_checkpoint_u64(bytes, offset)?)) }
}

impl NumericalRestoreScalar for (u32, f64) {
    const WIDTH: usize = 12;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> {
        Ok((read_checkpoint_u32(bytes, offset)?, f64::from_bits(read_checkpoint_u64(bytes, offset.checked_add(4).ok_or(NumericalCheckpointFault::Truncated)?)?)))
    }
}

pub trait NumericalRestoreOwner<T> {
    fn restored_len(&self) -> usize;
    fn reserve_restore(&mut self, capacity: usize) -> Result<bool, NumericalCheckpointFault>;
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault>;
}

impl<T> NumericalRestoreOwner<T> for Vec<T> {
    fn restored_len(&self) -> usize { self.len() }
    fn reserve_restore(&mut self, capacity: usize) -> Result<bool, NumericalCheckpointFault> {
        let bytes = capacity.checked_mul(size_of::<T>()).ok_or(NumericalCheckpointFault::Envelope)?;
        if bytes > NUMERICAL_OWNER_PAGE_BYTES { return Err(NumericalCheckpointFault::Envelope); }
        if self.capacity() < capacity { self.try_reserve_exact(capacity - self.len()).map_err(|_| NumericalCheckpointFault::Admission)?; }
        validate_restored_owner(self)?;
        Ok(true)
    }
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault> {
        if self.len() == self.capacity() { return Err(NumericalCheckpointFault::Admission); }
        self.push(value);
        Ok(())
    }
}

impl<T, const N: usize> NumericalRestoreOwner<T> for PagedList<T, N> {
    fn restored_len(&self) -> usize { self.len() }
    fn reserve_restore(&mut self, capacity: usize) -> Result<bool, NumericalCheckpointFault> {
        if self.capacity() < capacity { self.reserve_capacity_one(capacity, SPARSE_PAGE_BYTES).map_err(|_| NumericalCheckpointFault::Admission)?; }
        Ok(self.capacity() >= capacity)
    }
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault> { self.push_reserved(value).map_err(|_| NumericalCheckpointFault::Admission) }
}

pub fn restore_numerical_owner<T: NumericalRestoreScalar>(
    owner: &mut impl NumericalRestoreOwner<T>,
    page: &NumericalPageView<'_>,
    maximum: usize,
    entry: &mut usize,
    cursor: &mut NumericalOwnerRestoreCursor,
    matrix: bool,
) -> Result<bool, NumericalCheckpointFault> {
    let shape = cursor.begin_page(page, maximum, T::WIDTH, matrix)?;
    if owner.restored_len() != cursor.next_item { return Err(NumericalCheckpointFault::Field); }
    if *entry == 0 {
        if owner.reserve_restore(shape.capacity)? { *entry = 1; }
        return Ok(false);
    }
    let item = entry.checked_sub(1).ok_or(NumericalCheckpointFault::Field)?;
    if item > cursor.page_values || cursor.page_item.checked_add(item) != Some(cursor.next_item) { return Err(NumericalCheckpointFault::Field); }
    if item < cursor.page_values {
        let prefix = if page.owner == 0 { if matrix { 24 } else { 8 } } else { 0 };
        owner.append_restore(T::decode(page.bytes, prefix + item * T::WIDTH)?)?;
        cursor.next_item += 1;
        *entry += 1;
        return Ok(false);
    }
    cursor.finish_page();
    Ok(true)
}

pub fn restore_u32_entry(owner: &mut Vec<u32>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_usize_entry(owner: &mut Vec<usize>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_f64_entry(owner: &mut Vec<f64>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_paged_u32_entry<const N: usize>(owner: &mut PagedList<u32, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_paged_f64_entry<const N: usize>(owner: &mut PagedList<f64, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_pair_entry(owner: &mut Vec<(u32, f64)>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}
