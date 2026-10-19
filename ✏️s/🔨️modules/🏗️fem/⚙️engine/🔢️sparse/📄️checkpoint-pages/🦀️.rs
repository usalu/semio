//! 📄️ Numerical checkpoint pages: identity, mounted scalar slots, page cursors, owner advance and restore.

use super::*;
// #region 🔖️Ldlt




/// 🧱️ Packed sparse columns retain one offset stream and one original entry stream.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SparseColumns {
    pub offsets: PagedList<usize, SPARSE_INDEX_SPACE>,
    pub entries: PagedList<(u32, f64), SPARSE_INDEX_SPACE>,
}

impl SparseColumns {
    pub fn from_owned(columns: Vec<Vec<(u32, f64)>>) -> Self {
        let mut offsets = Vec::with_capacity(columns.len() + 1);
        let mut entries = Vec::new();
        offsets.push(0);
        for column in columns { entries.extend(column); offsets.push(entries.len()); }
        Self { offsets: paged_from_values(offsets), entries: paged_from_values(entries) }
    }
    pub fn len(&self) -> usize { self.offsets.len().saturating_sub(1) }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn column_len(&self, column: usize) -> Option<usize> { self.offsets.get(column.checked_add(1)?)?.checked_sub(*self.offsets.get(column)?) }
    pub fn entry(&self, column: usize, entry: usize) -> Option<(u32, f64)> {
        if entry >= self.column_len(column)? { return None; }
        self.entries.get(self.offsets.get(column)?.checked_add(entry)?).copied()
    }
    pub fn entry_depth_demand(&self, column: usize, entry: usize) -> Result<usize, ValueError> {
        let failure = |error: semio_framework_value_persistent::list::PagedListError| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason);
        let after = column.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "factor column offset overflow"))?;
        let outer = self.offsets.next_get_depth_demand(after).map_err(failure)?;
        if self.column_len(column).is_some_and(|length| entry >= length) { return outer.checked_mul(2).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "factor path depth overflow")); }
        let offset = self.offsets.get(column).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "factor column is missing"))?.checked_add(entry).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "factor entry offset overflow"))?;
        outer.checked_mul(2).and_then(|depth| depth.checked_add(self.entries.next_get_depth_demand(offset).ok()?)).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "factor path depth overflow"))
    }
    pub fn terminal_is_empty(&self) -> bool { self.offsets.terminal_is_empty() && self.entries.terminal_is_empty() }
    pub fn next_retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.entries.terminal_is_empty() { paged_owner_retirement_demand(&self.entries) } else { paged_owner_retirement_demand(&self.offsets) }
    }
    pub fn close_step(&mut self, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
        if let Some(step) = close_paged_owner_step(&mut self.entries, maximum_bytes)? { return Ok(Some(step)); }
        close_paged_owner_step(&mut self.offsets, maximum_bytes)
    }
}

/// 🔗️ Original row contributors preserve ascending factor-column arithmetic order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowContributor { pub column: u32, pub next: u64 }

#[derive(Clone, PartialEq, Default)]
pub struct RowContributors {
    pub heads: PagedList<u64, SPARSE_INDEX_SPACE>,
    pub tails: PagedList<u64, SPARSE_INDEX_SPACE>,
    pub links: PagedList<RowContributor, SPARSE_INDEX_SPACE>,
}

impl RowContributors {
    pub fn terminal_is_empty(&self) -> bool { self.heads.terminal_is_empty() && self.tails.terminal_is_empty() && self.links.terminal_is_empty() }
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.links.terminal_is_empty() { paged_owner_retirement_demand(&self.links) } else if !self.tails.terminal_is_empty() { paged_owner_retirement_demand(&self.tails) } else { paged_owner_retirement_demand(&self.heads) }
    }
    pub fn close_step(&mut self, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
        if let Some(step) = close_paged_owner_step(&mut self.links, maximum_bytes)? { return Ok(Some(step)); }
        if let Some(step) = close_paged_owner_step(&mut self.tails, maximum_bytes)? { return Ok(Some(step)); }
        close_paged_owner_step(&mut self.heads, maximum_bytes)
    }
}


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

fn write_numerical_bytes(target: &mut [std::mem::MaybeUninit<u8>], bytes: &[u8]) -> Result<(), JobPayloadAdmissionFault> {
    if target.len() != bytes.len() { return Err(JobPayloadAdmissionFault::StreamBytes); }
    for (slot, byte) in target.iter_mut().zip(bytes) { slot.write(*byte); }
    Ok(())
}

struct NumericalBytesSource<'a>(&'a [u8]);
impl semio_framework_job_payload::JobPayloadStagedSource for NumericalBytesSource<'_> {
    fn byte_len(&self) -> usize { self.0.len() }
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(self.0.len()) }
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) }
    fn write_bytes(&self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(target, self.0) }
}

pub trait NumericalWireScalar: NumericalRestoreScalar + Copy {
    fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault>;
}
impl NumericalWireScalar for u32 { fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(target, &self.to_le_bytes()) } }
impl NumericalWireScalar for u64 { fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(target, &self.to_le_bytes()) } }
impl NumericalWireScalar for usize { fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(target, &(self as u64).to_le_bytes()) } }
impl NumericalWireScalar for f64 { fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(target, &self.to_bits().to_le_bytes()) } }
impl NumericalWireScalar for (u32, f64) { fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(&mut target[..4], &self.0.to_le_bytes())?; write_numerical_bytes(&mut target[4..], &self.1.to_bits().to_le_bytes()) } }
impl NumericalWireScalar for RowContributor { fn write_wire(self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { write_numerical_bytes(&mut target[..4], &self.column.to_le_bytes())?; write_numerical_bytes(&mut target[4..], &self.next.to_le_bytes()) } }

pub trait NumericalReadView<T: NumericalWireScalar> {
    fn length(&self) -> usize;
    fn read_scalar(&self, index: usize) -> Option<T>;
    fn read_depth(&self, index: usize) -> Result<usize, JobPayloadAdmissionFault>;
}
impl<T: NumericalWireScalar> NumericalReadView<T> for [T] {
    fn length(&self) -> usize { self.len() }
    fn read_scalar(&self, index: usize) -> Option<T> { self.get(index).copied() }
    fn read_depth(&self, _: usize) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) }
}
impl<T: NumericalWireScalar> NumericalReadView<T> for Vec<T> {
    fn length(&self) -> usize { self.len() }
    fn read_scalar(&self, index: usize) -> Option<T> { self.get(index).copied() }
    fn read_depth(&self, _: usize) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) }
}
impl<T: NumericalWireScalar, const N: usize> NumericalReadView<T> for [T; N] {
    fn length(&self) -> usize { self.len() }
    fn read_scalar(&self, index: usize) -> Option<T> { self.get(index).copied() }
    fn read_depth(&self, _: usize) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) }
}
impl<T: NumericalWireScalar, const N: usize> NumericalReadView<T> for PagedList<T, N> {
    fn length(&self) -> usize { self.len() }
    fn read_scalar(&self, index: usize) -> Option<T> { self.get(index).copied() }
    fn read_depth(&self, index: usize) -> Result<usize, JobPayloadAdmissionFault> { self.next_get_depth_demand(index).map_err(|_| JobPayloadAdmissionFault::StreamBytes) }
}

struct NumericalScalarSource<'a, T: NumericalWireScalar, S: NumericalReadView<T> + ?Sized> { values: &'a S, index: usize, marker: std::marker::PhantomData<T> }
impl<T: NumericalWireScalar, S: NumericalReadView<T> + ?Sized> semio_framework_job_payload::JobPayloadStagedSource for NumericalScalarSource<'_, T, S> {
    fn byte_len(&self) -> usize { T::WIDTH }
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(size_of::<T>() + T::WIDTH) }
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { self.values.read_depth(self.index) }
    fn write_bytes(&self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { self.values.read_scalar(self.index).ok_or(JobPayloadAdmissionFault::StreamBytes)?.write_wire(target) }
}

pub fn advance_numerical_page_header(writer: &mut RetainedJobPayloadWriter, magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    if writer.staged_page_len() == Some(0) {
        writer.write_staged_source_funded(context, &NumericalBytesSource(&numerical_page_header(magic, kind, cursor)), 0)?;
        return Ok(true);
    }
    Ok(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalCheckpointFault { Cancelled, Stale, Version, Truncated, Field, Envelope, Admission }

pub fn write_numerical_entry(writer: &mut RetainedJobPayloadWriter, bytes: &[u8], context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    if writer.staged_page_remaining() < bytes.len() { writer.commit_staged_page_funded(context, 0)?; return Ok(false); }
    writer.write_staged_source_funded(context, &NumericalBytesSource(bytes), size_of::<usize>())
}

pub fn advance_owner_length(writer: &mut RetainedJobPayloadWriter, length: usize, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    if cursor.owner == 0 {
        if write_numerical_entry(writer, &(length as u64).to_le_bytes(), context)? { cursor.owner = 1; }
        return Ok(false);
    }
    Ok(true)
}

fn advance_numerical_owner<T: NumericalWireScalar>(writer: &mut RetainedJobPayloadWriter, values: &(impl NumericalReadView<T> + ?Sized), cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    if !advance_owner_length(writer, values.length(), cursor, context)? { return Ok(false); }
    if cursor.item < values.length() {
        if writer.staged_page_remaining() < T::WIDTH { writer.commit_staged_page_funded(context, 0)?; return Ok(false); }
        let source = NumericalScalarSource { values, index: cursor.item, marker: std::marker::PhantomData };
        if writer.write_staged_source_funded(context, &source, size_of::<usize>())? { cursor.item += 1; }
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u32_owner(writer: &mut RetainedJobPayloadWriter, values: &[u32], cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_u64_owner(writer: &mut RetainedJobPayloadWriter, values: &(impl NumericalReadView<usize> + ?Sized), cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_u64_values(writer: &mut RetainedJobPayloadWriter, values: &[u64], cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_f64_owner(writer: &mut RetainedJobPayloadWriter, values: &[f64], cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_paged_u32_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<u32, N>, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_paged_f64_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<f64, N>, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_pair_owner(writer: &mut RetainedJobPayloadWriter, values: &[(u32, f64)], cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_paged_pair_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<(u32, f64), N>, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_paged_u64_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<u64, N>, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }
pub fn advance_contributor_owner(writer: &mut RetainedJobPayloadWriter, values: &PagedList<RowContributor, SPARSE_INDEX_SPACE>, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> { advance_numerical_owner(writer, values, cursor, context) }

pub trait NumericalMatrixView { fn rows(&self) -> usize; fn columns(&self) -> usize; fn length(&self) -> usize; fn scalar(&self, index: usize) -> Option<f64>; fn scalar_depth(&self, index: usize) -> Result<usize, JobPayloadAdmissionFault>; }
impl NumericalMatrixView for MatD { fn rows(&self) -> usize { self.rows } fn columns(&self) -> usize { self.cols } fn length(&self) -> usize { self.data.len() } fn scalar(&self, index: usize) -> Option<f64> { self.data.get(index).copied() } fn scalar_depth(&self, _: usize) -> Result<usize, JobPayloadAdmissionFault> { Ok(1) } }
impl NumericalMatrixView for semio_s_fem_algebra::PagedMatD { fn rows(&self) -> usize { self.rows } fn columns(&self) -> usize { self.cols } fn length(&self) -> usize { self.data.len() } fn scalar(&self, index: usize) -> Option<f64> { self.data.get(index).copied() } fn scalar_depth(&self, index: usize) -> Result<usize, JobPayloadAdmissionFault> { self.data.next_get_depth_demand(index).map_err(|_| JobPayloadAdmissionFault::StreamBytes) } }
struct NumericalMatrixScalar<'a, M: NumericalMatrixView> { matrix: &'a M, index: usize }
impl<M: NumericalMatrixView> semio_framework_job_payload::JobPayloadStagedSource for NumericalMatrixScalar<'_, M> {
    fn byte_len(&self) -> usize { 8 }
    fn copy_byte_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { Ok(16) }
    fn depth_demand(&self) -> Result<usize, JobPayloadAdmissionFault> { self.matrix.scalar_depth(self.index) }
    fn write_bytes(&self, target: &mut [std::mem::MaybeUninit<u8>]) -> Result<(), JobPayloadAdmissionFault> { self.matrix.scalar(self.index).ok_or(JobPayloadAdmissionFault::StreamBytes)?.write_wire(target) }
}
pub fn advance_matrix_owner(writer: &mut RetainedJobPayloadWriter, matrix: &impl NumericalMatrixView, cursor: &mut NumericalPageCursor, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    if cursor.owner == 0 {
        let mut bytes = [0; 24]; bytes[..8].copy_from_slice(&(matrix.rows() as u64).to_le_bytes()); bytes[8..16].copy_from_slice(&(matrix.columns() as u64).to_le_bytes()); bytes[16..].copy_from_slice(&(matrix.length() as u64).to_le_bytes());
        if write_numerical_entry(writer, &bytes, context)? { cursor.owner = 1; }
        return Ok(false);
    }
    if cursor.item < matrix.length() {
        if writer.staged_page_remaining() < 8 { writer.commit_staged_page_funded(context, 0)?; return Ok(false); }
        if writer.write_staged_source_funded(context, &NumericalMatrixScalar { matrix, index: cursor.item }, size_of::<usize>())? { cursor.item += 1; }
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
        let page_values = remaining.min((semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES - NUMERICAL_CHECKPOINT_HEADER_BYTES - prefix) / width);
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

impl NumericalRestoreScalar for u64 {
    const WIDTH: usize = 8;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { read_checkpoint_u64(bytes, offset) }
}

impl NumericalRestoreScalar for RowContributor {
    const WIDTH: usize = 12;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { Ok(Self { column: read_checkpoint_u32(bytes, offset)?, next: read_checkpoint_u64(bytes, offset.checked_add(4).ok_or(NumericalCheckpointFault::Truncated)?)? }) }
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
    fn reserve_restore_demand(&self, capacity: usize) -> Result<Option<RetirementDemand>, NumericalCheckpointFault>;
    fn reserve_restore(&mut self, capacity: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, NumericalCheckpointFault>;
    fn append_restore_depth(&self) -> Result<usize, NumericalCheckpointFault>;
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault>;
}

impl<T> NumericalRestoreOwner<T> for Vec<T> {
    fn restored_len(&self) -> usize { self.len() }
    fn reserve_restore_demand(&self, capacity: usize) -> Result<Option<RetirementDemand>, NumericalCheckpointFault> {
        let bytes = capacity.checked_mul(size_of::<T>()).ok_or(NumericalCheckpointFault::Envelope)?;
        if bytes > NUMERICAL_OWNER_PAGE_BYTES { return Err(NumericalCheckpointFault::Envelope); }
        if self.capacity() >= capacity { return Ok(None); }
        if self.capacity() != 0 { return Err(NumericalCheckpointFault::Envelope); }
        Ok(Some(RetirementDemand { copy_bytes: size_of::<Self>(), capacity_bytes: bytes, depth: 1, ..Default::default() }))
    }
    fn reserve_restore(&mut self, capacity: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, NumericalCheckpointFault> {
        let Some(demand) = self.reserve_restore_demand(capacity)? else { return Ok(Default::default()); };
        if !numerical_grant_covers(grant, demand) { return Ok(Default::default()); }
        self.try_reserve_exact(capacity - self.len()).map_err(|_| NumericalCheckpointFault::Admission)?;
        validate_restored_owner(self)?;
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: self.capacity() * size_of::<T>(), ..Default::default() })
    }
    fn append_restore_depth(&self) -> Result<usize, NumericalCheckpointFault> { Ok(1) }
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault> {
        if self.len() == self.capacity() { return Err(NumericalCheckpointFault::Admission); }
        self.push(value);
        Ok(())
    }
}

impl<T, const N: usize> NumericalRestoreOwner<T> for PagedList<T, N> {
    fn restored_len(&self) -> usize { self.len() }
    fn reserve_restore_demand(&self, capacity: usize) -> Result<Option<RetirementDemand>, NumericalCheckpointFault> {
        let Some(capacity_bytes) = self.next_capacity_allocation_bytes(capacity).map_err(|_| NumericalCheckpointFault::Admission)? else { return Ok(None); };
        Ok(Some(RetirementDemand { capacity_bytes, copy_bytes: self.next_capacity_copy_byte_demand(capacity).map_err(|_| NumericalCheckpointFault::Admission)?, depth: self.next_capacity_depth_demand(capacity).map_err(|_| NumericalCheckpointFault::Admission)?, ..Default::default() }))
    }
    fn reserve_restore(&mut self, capacity: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, NumericalCheckpointFault> {
        self.reserve_capacity_one_funded(capacity, grant).map_err(|_| NumericalCheckpointFault::Admission)
    }
    fn append_restore_depth(&self) -> Result<usize, NumericalCheckpointFault> { self.next_push_depth_demand().map_err(|_| NumericalCheckpointFault::Admission) }
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault> { self.push_reserved(value).map_err(|_| NumericalCheckpointFault::Admission) }
}

fn numerical_restore_owner_demand<T: NumericalRestoreScalar>(owner: &impl NumericalRestoreOwner<T>, page: &NumericalPageView<'_>, cursor: &NumericalOwnerRestoreCursor, entry: usize, matrix: bool) -> Result<RetirementDemand, NumericalCheckpointFault> {
    if !cursor.page_open {
        let prefix = if page.owner == 0 { if matrix { 24 } else { 8 } } else { 0 };
        return Ok(RetirementDemand { copy_bytes: NUMERICAL_CHECKPOINT_HEADER_BYTES + prefix + size_of::<NumericalOwnerRestoreCursor>() + if matrix && page.owner == 0 { 2 * size_of::<usize>() } else { 0 }, depth: 1, ..Default::default() });
    }
    let shape = cursor.shape.ok_or(NumericalCheckpointFault::Field)?;
    if entry == 0 {
        return Ok(owner.reserve_restore_demand(shape.capacity)?.unwrap_or(RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() }));
    }
    let item = entry.checked_sub(1).ok_or(NumericalCheckpointFault::Field)?;
    if item < cursor.page_values {
        Ok(RetirementDemand { copy_bytes: T::WIDTH + size_of::<T>() + 2 * size_of::<usize>(), depth: owner.append_restore_depth()?, ..Default::default() })
    } else {
        Ok(RetirementDemand { copy_bytes: size_of::<NumericalOwnerRestoreCursor>(), depth: 1, ..Default::default() })
    }
}

pub fn numerical_restore_admit(context: &mut StepContext<'_>, demand: RetirementDemand) -> Result<bool, NumericalCheckpointFault> {
    if context.is_cancelled() { return Err(NumericalCheckpointFault::Cancelled); }
    if !numerical_grant_covers(context.retained_grant(), demand) { return Ok(false); }
    context.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes }).map_err(|_| NumericalCheckpointFault::Admission)?;
    Ok(true)
}

pub fn numerical_restore_control(context: &mut StepContext<'_>, entry: usize, count: usize, installed_bytes: usize, depth: usize) -> Result<bool, NumericalCheckpointFault> {
    let copy_bytes = if entry == 0 { NUMERICAL_CHECKPOINT_HEADER_BYTES + 8 + size_of::<usize>() } else if entry <= count { 16 + size_of::<usize>() } else { count * 8 + installed_bytes + size_of::<usize>() };
    numerical_restore_admit(context, RetirementDemand { copy_bytes, depth: depth.max(1), ..Default::default() })
}

pub fn restore_numerical_owner<T: NumericalRestoreScalar>(
    owner: &mut impl NumericalRestoreOwner<T>,
    page: &NumericalPageView<'_>,
    maximum: usize,
    entry: &mut usize,
    cursor: &mut NumericalOwnerRestoreCursor,
    matrix: bool,
    context: &mut StepContext<'_>,
) -> Result<bool, NumericalCheckpointFault> {
    let demand = numerical_restore_owner_demand(owner, page, cursor, *entry, matrix)?;
    if context.is_cancelled() { return Err(NumericalCheckpointFault::Cancelled); }
    if !numerical_grant_covers(context.retained_grant(), demand) { return Ok(false); }
    if !cursor.page_open {
        cursor.begin_page(page, maximum, T::WIDTH, matrix)?;
        numerical_restore_admit(context, demand)?;
        return Ok(false);
    }
    let shape = cursor.shape.ok_or(NumericalCheckpointFault::Field)?;
    if page.item != cursor.page_item || page.owner != cursor.page_owner { return Err(NumericalCheckpointFault::Field); }
    if owner.restored_len() != cursor.next_item { return Err(NumericalCheckpointFault::Field); }
    if *entry == 0 {
        if owner.reserve_restore_demand(shape.capacity)?.is_some() {
            let progress = owner.reserve_restore(shape.capacity, context.retained_grant())?;
            context.consume_retained(progress).map_err(|_| NumericalCheckpointFault::Admission)?;
            return Ok(false);
        }
        numerical_restore_admit(context, demand)?;
        *entry = 1;
        return Ok(false);
    }
    let item = entry.checked_sub(1).ok_or(NumericalCheckpointFault::Field)?;
    if item > cursor.page_values || cursor.page_item.checked_add(item) != Some(cursor.next_item) { return Err(NumericalCheckpointFault::Field); }
    if item < cursor.page_values {
        let prefix = if page.owner == 0 { if matrix { 24 } else { 8 } } else { 0 };
        let value = T::decode(page.bytes, prefix + item * T::WIDTH)?;
        numerical_restore_admit(context, demand)?;
        owner.append_restore(value)?;
        cursor.next_item += 1;
        *entry += 1;
        return Ok(false);
    }
    numerical_restore_admit(context, demand)?;
    cursor.finish_page();
    Ok(true)
}

pub fn restore_u32_entry(owner: &mut Vec<u32>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false, context)
}

pub fn restore_usize_entry(owner: &mut Vec<usize>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false, context)
}

pub fn restore_f64_entry(owner: &mut Vec<f64>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false, context)
}

pub fn restore_paged_u32_entry<const N: usize>(owner: &mut PagedList<u32, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false, context)
}

pub fn restore_paged_f64_entry<const N: usize>(owner: &mut PagedList<f64, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false, context)
}

pub fn restore_pair_entry(owner: &mut Vec<(u32, f64)>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false, context)
}
