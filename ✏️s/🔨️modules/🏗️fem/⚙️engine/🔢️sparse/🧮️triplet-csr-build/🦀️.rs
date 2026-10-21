//! 🧮️ Funded triplet to CSR conversion: sequence-ordered heap sort, duplicate merge and row pointers, one admitted action per call.

use super::*;

const CSR_TRIPLET_QUOTE: &[u8] = b"fem.triplet-csr-quote";

/// 🧱️ One element-matrix contribution; its global assembly sequence orders duplicate coordinates so accumulation is deterministic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SparseTriplet {
    pub sequence: u64,
    pub row: u32,
    pub col: u32,
    pub value: f64,
}

impl SparseTriplet {
    fn key(&self) -> (u32, u32, u64) {
        (self.row, self.col, self.sequence)
    }
}

/// 🪜️ The phases of the funded triplet to CSR conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsrTripletBuildStage {
    ReserveRows,
    InitializeRows,
    Sort,
    CountUnique,
    ReserveIndices,
    ReserveValues,
    Merge,
    ReserveIndptr,
    Indptr,
    Complete,
}

/// 🧵️ Retained triplet to CSR conversion. Every heap-sort comparison and swap, duplicate merge, row count and output entry advances
/// under its own quoted grant, so a denied item, copy, capacity or depth axis leaves the build exactly as it was.
pub struct CsrTripletBuild {
    pub entries: PagedList<SparseTriplet, SPARSE_INDEX_SPACE>,
    pub stage: CsrTripletBuildStage,
    pub sort: HeapSortCursor,
    pub count_cursor: usize,
    pub unique_count: usize,
    pub count_key: Option<(u32, u32)>,
    pub merge_cursor: usize,
    pub row_cursor: usize,
    pub row_counts: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub indptr: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub indices: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub values: PagedList<f64, SPARSE_INDEX_SPACE>,
    pub last_key: Option<(u32, u32)>,
    pub matrix: Option<Csr>,
    pub n: usize,
}

impl CsrTripletBuild {
    /// 🏗️ Takes ownership of the unsorted triplets of an order-`n` matrix.
    pub fn new(n: usize, entries: PagedList<SparseTriplet, SPARSE_INDEX_SPACE>) -> Self {
        let sort = HeapSortCursor::new(entries.len());
        Self {
            entries,
            stage: CsrTripletBuildStage::ReserveRows,
            sort,
            count_cursor: 0,
            unique_count: 0,
            count_key: None,
            merge_cursor: 0,
            row_cursor: 0,
            row_counts: PagedList::default(),
            indptr: PagedList::default(),
            indices: PagedList::default(),
            values: PagedList::default(),
            last_key: None,
            matrix: None,
            n,
        }
    }

    /// 📦️ The physical backing every owner of this build retains, including the finished matrix until it is taken.
    pub fn allocated_bytes(&self) -> usize {
        self.entries.allocated_bytes() + self.row_counts.allocated_bytes() + self.indptr.allocated_bytes() + self.indices.allocated_bytes() + self.values.allocated_bytes() + self.matrix.as_ref().map_or(0, Csr::allocated_bytes)
    }

    fn quote_reserve<T>(owner: &PagedList<T, SPARSE_INDEX_SPACE>, target: usize) -> Result<RetirementDemand, &'static [u8]> {
        let refused = |_| CSR_TRIPLET_QUOTE;
        Ok(RetirementDemand {
            copy_bytes: size_of::<usize>() + owner.next_capacity_copy_byte_demand(target).map_err(refused)?,
            capacity_bytes: owner.next_capacity_allocation_bytes(target).map_err(refused)?.unwrap_or(0),
            release_bytes: 0,
            depth: owner.next_capacity_depth_demand(target).map_err(refused)?.max(1),
        })
    }

    fn quote_get<T>(owner: &PagedList<T, SPARSE_INDEX_SPACE>, index: usize) -> Result<usize, &'static [u8]> {
        owner.next_get_depth_demand(index).map_err(|_| CSR_TRIPLET_QUOTE)
    }

    fn quote_push<T>(owner: &PagedList<T, SPARSE_INDEX_SPACE>) -> Result<usize, &'static [u8]> {
        owner.next_push_depth_demand().map_err(|_| CSR_TRIPLET_QUOTE)
    }

    fn quote(copy_bytes: usize, depth: usize) -> Result<RetirementDemand, &'static [u8]> {
        Ok(RetirementDemand { copy_bytes: copy_bytes.max(1), depth: depth.max(1), ..Default::default() })
    }

    /// 💬️ Quotes the item, copy, capacity and depth demand of the next action without changing any owner.
    pub fn next_demands(&self) -> Result<RetirementDemand, &'static [u8]> {
        let entry = size_of::<SparseTriplet>();
        match self.stage {
            CsrTripletBuildStage::ReserveRows => Self::quote_reserve(&self.row_counts, self.n),
            CsrTripletBuildStage::InitializeRows if self.row_counts.len() < self.n => Self::quote(size_of::<u32>(), Self::quote_push(&self.row_counts)?),
            CsrTripletBuildStage::InitializeRows => Self::quote(size_of::<usize>(), 1),
            CsrTripletBuildStage::Sort => match self.sort.next() {
                HeapSortWork::Compare { left, right } | HeapSortWork::Swap { left, right } => Self::quote(size_of::<HeapSortCursor>() + 2 * entry, Self::quote_get(&self.entries, left)? + Self::quote_get(&self.entries, right)?),
                HeapSortWork::Advance | HeapSortWork::Complete => Self::quote(size_of::<HeapSortCursor>(), 1),
            },
            CsrTripletBuildStage::CountUnique if self.count_cursor < self.entries.len() => Self::quote(entry + 2 * size_of::<usize>(), Self::quote_get(&self.entries, self.count_cursor)?),
            CsrTripletBuildStage::CountUnique => Self::quote(size_of::<usize>(), 1),
            CsrTripletBuildStage::ReserveIndices => Self::quote_reserve(&self.indices, self.unique_count),
            CsrTripletBuildStage::ReserveValues => Self::quote_reserve(&self.values, self.unique_count),
            CsrTripletBuildStage::Merge => match self.entries.get(self.merge_cursor) {
                None => Self::quote(size_of::<usize>(), 1),
                Some(item) => {
                    let placement = if self.last_key == Some((item.row, item.col)) {
                        Self::quote_get(&self.values, self.values.len().saturating_sub(1))?
                    } else {
                        Self::quote_push(&self.indices)? + Self::quote_push(&self.values)? + Self::quote_get(&self.row_counts, item.row as usize)?
                    };
                    Self::quote(entry + size_of::<u32>() + size_of::<f64>(), placement + Self::quote_get(&self.entries, self.merge_cursor)?)
                }
            },
            CsrTripletBuildStage::ReserveIndptr => {
                let required = self.n.checked_add(1).ok_or(CSR_TRIPLET_QUOTE)?;
                if self.indptr.capacity() < required { Self::quote_reserve(&self.indptr, required) } else { Self::quote(size_of::<u32>(), Self::quote_push(&self.indptr)?) }
            }
            CsrTripletBuildStage::Indptr if self.row_cursor < self.row_counts.len() => Self::quote(3 * size_of::<u32>(), Self::quote_get(&self.row_counts, self.row_cursor)? + Self::quote_get(&self.indptr, self.indptr.len().saturating_sub(1))? + Self::quote_push(&self.indptr)?),
            CsrTripletBuildStage::Indptr => Self::quote(size_of::<Csr>(), 1),
            CsrTripletBuildStage::Complete => Ok(RetirementDemand::default()),
        }
    }

    fn reserve<T>(owner: &mut PagedList<T, SPARSE_INDEX_SPACE>, target: usize, refusal: &'static [u8]) -> Result<bool, &'static [u8]> {
        if owner.capacity() >= target {
            return Ok(true);
        }
        let progress = owner.reserve_capacity_one(target, SPARSE_PAGE_BYTES).map_err(|_| refusal)?;
        if progress.progressed { Ok(false) } else { Err(refusal) }
    }

    fn advance(&mut self) -> Result<bool, &'static [u8]> {
        let n = self.n;
        match self.stage {
            CsrTripletBuildStage::ReserveRows => {
                if Self::reserve(&mut self.row_counts, n, b"fem.triplet-csr-row-allocation")? {
                    self.stage = CsrTripletBuildStage::InitializeRows;
                }
            }
            CsrTripletBuildStage::InitializeRows => {
                if self.row_counts.len() < n {
                    self.row_counts.push_reserved(0).map_err(|_| b"fem.triplet-csr-row-placement" as &'static [u8])?;
                } else {
                    self.stage = CsrTripletBuildStage::Sort;
                }
            }
            CsrTripletBuildStage::Sort => match self.sort.next() {
                HeapSortWork::Complete => self.stage = CsrTripletBuildStage::CountUnique,
                HeapSortWork::Compare { left, right } => {
                    let left_key = self.entries.get(left).ok_or(b"fem.triplet-csr-left-entry" as &'static [u8])?.key();
                    let right_key = self.entries.get(right).ok_or(b"fem.triplet-csr-right-entry" as &'static [u8])?.key();
                    self.sort.advance(Some(left_key.cmp(&right_key))).map_err(|_| b"fem.triplet-csr-sort" as &'static [u8])?;
                }
                HeapSortWork::Swap { left, right } => {
                    if left >= self.entries.len() || right >= self.entries.len() {
                        return Err(b"fem.triplet-csr-sort-bound");
                    }
                    self.entries.swap(left, right);
                    self.sort.advance(None).map_err(|_| b"fem.triplet-csr-sort" as &'static [u8])?;
                }
                HeapSortWork::Advance => self.sort.advance(None).map_err(|_| b"fem.triplet-csr-sort" as &'static [u8])?,
            },
            CsrTripletBuildStage::CountUnique => {
                if let Some(item) = self.entries.get(self.count_cursor).copied() {
                    let key = (item.row, item.col);
                    if self.count_key != Some(key) {
                        self.unique_count = self.unique_count.checked_add(1).ok_or(b"fem.triplet-csr-unique-overflow" as &'static [u8])?;
                        self.count_key = Some(key);
                    }
                    self.count_cursor += 1;
                } else {
                    self.stage = CsrTripletBuildStage::ReserveIndices;
                }
            }
            CsrTripletBuildStage::ReserveIndices => {
                if Self::reserve(&mut self.indices, self.unique_count, b"fem.triplet-csr-index-allocation")? {
                    self.stage = CsrTripletBuildStage::ReserveValues;
                }
            }
            CsrTripletBuildStage::ReserveValues => {
                if Self::reserve(&mut self.values, self.unique_count, b"fem.triplet-csr-value-allocation")? {
                    self.stage = CsrTripletBuildStage::Merge;
                }
            }
            CsrTripletBuildStage::Merge => {
                if let Some(item) = self.entries.get(self.merge_cursor).copied() {
                    let key = (item.row, item.col);
                    if self.last_key == Some(key) {
                        let last = self.values.len().checked_sub(1).ok_or(b"fem.triplet-csr-missing-value" as &'static [u8])?;
                        *self.values.get_mut(last).ok_or(b"fem.triplet-csr-missing-value" as &'static [u8])? += item.value;
                    } else {
                        if !self.indices.has_reserved_slot() || !self.values.has_reserved_slot() {
                            return Err(b"fem.triplet-csr-output-capacity");
                        }
                        let count = self.row_counts.get_mut(item.row as usize).ok_or(b"fem.triplet-csr-row-bound" as &'static [u8])?;
                        *count = count.checked_add(1).ok_or(b"fem.triplet-csr-row-overflow" as &'static [u8])?;
                        self.indices.push_reserved(item.col).map_err(|_| b"fem.triplet-csr-index-placement" as &'static [u8])?;
                        self.values.push_reserved(item.value).map_err(|_| b"fem.triplet-csr-value-placement" as &'static [u8])?;
                        self.last_key = Some(key);
                    }
                    self.merge_cursor += 1;
                } else {
                    self.stage = CsrTripletBuildStage::ReserveIndptr;
                }
            }
            CsrTripletBuildStage::ReserveIndptr => {
                let required = n.checked_add(1).ok_or(b"fem.triplet-csr-indptr-overflow" as &'static [u8])?;
                if Self::reserve(&mut self.indptr, required, b"fem.triplet-csr-indptr-allocation")? {
                    self.indptr.push_reserved(0).map_err(|_| b"fem.triplet-csr-indptr-placement" as &'static [u8])?;
                    self.stage = CsrTripletBuildStage::Indptr;
                }
            }
            CsrTripletBuildStage::Indptr => {
                if let Some(count) = self.row_counts.get(self.row_cursor).copied() {
                    let previous = self.indptr.last().copied().ok_or(b"fem.triplet-csr-indptr-missing" as &'static [u8])?;
                    let next = previous.checked_add(count).ok_or(b"fem.triplet-csr-indptr-overflow" as &'static [u8])?;
                    self.indptr.push_reserved(next).map_err(|_| b"fem.triplet-csr-indptr-placement" as &'static [u8])?;
                    self.row_cursor += 1;
                } else {
                    self.matrix = Some(Csr::from_paged_parts(n, std::mem::take(&mut self.indptr), std::mem::take(&mut self.indices), std::mem::take(&mut self.values)));
                    self.stage = CsrTripletBuildStage::Complete;
                }
            }
            CsrTripletBuildStage::Complete => {}
        }
        Ok(self.stage == CsrTripletBuildStage::Complete)
    }

    /// 🎟️ Runs the next quoted action only when the grant covers every axis of its quote; a denied axis changes nothing and reports no progress.
    pub fn step_one_funded(&mut self, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), &'static [u8]> {
        if self.stage == CsrTripletBuildStage::Complete {
            return Ok((true, RetainedCloneProgress::default()));
        }
        let demand = self.next_demands()?;
        if !numerical_grant_covers(grant, demand) {
            return Ok((false, RetainedCloneProgress::default()));
        }
        let before = self.allocated_bytes();
        let complete = self.advance()?;
        Ok((complete, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: self.allocated_bytes().saturating_sub(before), released_bytes: 0 }))
    }

    /// 🧾️ Quotes the copy of the finished matrix out of this build, or `None` before it exists.
    pub fn take_demands(&self) -> Option<RetirementDemand> {
        (self.stage == CsrTripletBuildStage::Complete && self.matrix.is_some()).then(|| RetirementDemand { copy_bytes: size_of::<Csr>(), depth: 1, ..Default::default() })
    }

    /// 📤️ Transfers the finished matrix only under a grant that covers its quote.
    pub fn take_complete(&mut self, grant: RetainedCloneGrant) -> Option<(Csr, RetainedCloneProgress)> {
        let demand = self.take_demands()?;
        if !numerical_grant_covers(grant, demand) {
            return None;
        }
        self.matrix.take().map(|matrix| (matrix, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    /// 🧹️ Releases one owner item or physical backing per call, entries first and the untaken matrix last.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_paged_owner_step(&mut self.entries, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.row_counts, &mut self.indptr, &mut self.indices] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        match close_paged_owner_step(&mut self.values, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        if let Some(matrix) = self.matrix.as_mut() {
            let step = matrix.close_step(maximum_bytes);
            if !step.0 {
                return step;
            }
            self.matrix = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }
}
