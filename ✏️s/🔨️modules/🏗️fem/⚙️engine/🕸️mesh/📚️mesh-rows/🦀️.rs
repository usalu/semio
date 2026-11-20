//! 📚️ Cold and mounted row storage for the same deterministic meshing stages.

use super::*;
use semio_framework_value_persistent::list::{PagedList, PagedListReservationFailure, PagedListReservationOutcome};
use semio_s_fem_sparse::{SPARSE_INDEX_SPACE, close_paged_owner_step, close_vec_owner_step, paged_owner_retirement_demand};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum MeshRows<T> {
    Cold(Vec<T>),
    Mounted(PagedList<T, SPARSE_INDEX_SPACE>),
}

impl<T> Default for MeshRows<T> {
    fn default() -> Self { Self::Cold(Vec::new()) }
}

impl<T> From<Vec<T>> for MeshRows<T> {
    fn from(values: Vec<T>) -> Self { Self::Cold(values) }
}

impl<T> MeshRows<T> {
    pub(super) fn new(mounted: bool) -> Self {
        if mounted { Self::Mounted(PagedList::default()) } else { Self::default() }
    }

    pub(super) fn is_mounted(&self) -> bool { matches!(self, Self::Mounted(_)) }

    pub(super) fn len(&self) -> usize {
        match self { Self::Cold(values) => values.len(), Self::Mounted(values) => values.len() }
    }

    pub(super) fn is_empty(&self) -> bool { self.len() == 0 }

    pub(super) fn capacity(&self) -> usize {
        match self { Self::Cold(values) => values.capacity(), Self::Mounted(values) => values.capacity() }
    }

    pub(super) fn get(&self, index: usize) -> Option<&T> {
        match self { Self::Cold(values) => values.get(index), Self::Mounted(values) => values.get(index) }
    }

    pub(super) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self { Self::Cold(values) => values.get_mut(index), Self::Mounted(values) => values.get_mut(index) }
    }

    pub(super) fn reserve(&mut self, target: usize, context: &mut StepContext<'_>) -> Result<bool, ()> {
        match self {
            Self::Cold(values) => {
                if values.capacity() < target { values.try_reserve_exact(target - values.len()).map_err(|_| ())?; }
                Ok(true)
            }
            Self::Mounted(values) => {
                if values.capacity() >= target { return Ok(true); }
                reserve_page(values, values.whole_page_capacity(target), context)?;
                Ok(false)
            }
        }
    }

    pub(super) fn reserve_initial(&mut self, target: usize, context: &mut StepContext<'_>) -> Result<bool, ()> {
        let target = match self {
            Self::Cold(_) if target == usize::MAX => 0,
            Self::Cold(_) => target,
            Self::Mounted(values) => target.min(values.whole_page_capacity(1)),
        };
        self.reserve(target, context)
    }

    pub(super) fn push(&mut self, value: T) {
        match self {
            Self::Cold(values) => values.push(value),
            Self::Mounted(values) => assert!(values.push_reserved(value).is_ok(), "mesh write exceeded its admitted rows"),
        }
    }

    pub(super) fn pop(&mut self) -> Option<T> {
        match self { Self::Cold(values) => values.pop(), Self::Mounted(values) => values.pop() }
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
        match self {
            Self::Cold(values) => close_vec_owner_step(values, maximum_bytes),
            Self::Mounted(values) => close_paged_owner_step(values, maximum_bytes).map(|progress| progress.map(|(_, bytes)| (1, bytes))),
        }
    }

    pub(super) fn demand(&self, depth: usize) -> Result<Option<RetirementDemand>, ValueError> {
        match self {
            Self::Cold(values) => mesh_job_run::vector_retirement_demand(values, depth),
            Self::Mounted(values) if values.terminal_is_empty() => Ok(None),
            Self::Mounted(values) => paged_owner_retirement_demand(values).map(|mut demand| {
                if !values.is_empty() { demand.copy_bytes = size_of::<T>(); }
                Some(demand)
            }),
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        match self { Self::Cold(values) => values.capacity() == 0, Self::Mounted(values) => values.terminal_is_empty() }
    }

    pub(super) fn as_cold(&self) -> &Vec<T> {
        match self { Self::Cold(values) => values, Self::Mounted(_) => unreachable!("cold mesh rows required") }
    }

    pub(super) fn into_vec(self) -> Vec<T> {
        match self { Self::Cold(values) => values, Self::Mounted(_) => unreachable!("cold mesh rows required") }
    }

    #[cfg(test)]
    pub(super) fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len()).map(|index| &self[index])
    }

    #[cfg(test)]
    pub(super) fn as_ptr(&self) -> *const T {
        match self { Self::Cold(values) => values.as_ptr(), Self::Mounted(values) => values.get(0).map_or(std::ptr::null(), std::ptr::from_ref) }
    }

    #[cfg(test)]
    pub(super) fn allocated_bytes(&self) -> usize {
        match self { Self::Cold(values) => values.capacity() * size_of::<T>(), Self::Mounted(values) => values.allocated_bytes() }
    }
}

fn reserve_page<T>(values: &mut PagedList<T, SPARSE_INDEX_SPACE>, target: usize, context: &mut StepContext<'_>) -> Result<(), ()> {
    let demand = RetirementDemand {
        copy_bytes: values.next_capacity_copy_byte_demand(target).map_err(|_| ())?,
        capacity_bytes: values.next_capacity_allocation_bytes(target).map_err(|_| ())?.unwrap_or(0),
        depth: values.next_capacity_depth_demand(target).map_err(|_| ())?,
        ..RetirementDemand::default()
    };
    if !context.admit_work_step(demand).map_err(|_| ())? { return Ok(()); }
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: demand.depth };
    let Some((turn, work)) = context.with_original_effect(grant, |_, effect| values.reserve_capacity_one_funded(target, effect)).map_err(|_| ())? else { return Ok(()); };
    let result = match turn.result {
        Ok(PagedListReservationOutcome::Ready(progress)) => Ok(RetainedCloneStep::Progress(progress)),
        Ok(PagedListReservationOutcome::Denied) => Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())),
        Err(PagedListReservationFailure::Work(error)) => Err(error),
        Err(PagedListReservationFailure::Allocation { error, copied_bytes }) => Err(ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress {
            copied_items: usize::from(copied_bytes != 0 || error.allocated_bytes != 0),
            copied_bytes,
            retained_capacity_bytes: error.allocated_bytes,
            released_bytes: 0,
        })),
    };
    receive_original_retained_work(context, RetainedWorkOutcome { result, work }).map(|_| ()).map_err(|_| ())
}

impl<T: Copy> MeshRows<T> {
    pub(super) fn swap(&mut self, left: usize, right: usize) {
        let previous = self[left];
        self[left] = self[right];
        self[right] = previous;
    }

    pub(super) fn snapshot(&self) -> Vec<T> {
        (0..self.len()).map(|index| self[index]).collect()
    }
}

impl<T> Index<usize> for MeshRows<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T { self.get(index).expect("mesh row is present") }
}

impl<T> IndexMut<usize> for MeshRows<T> {
    fn index_mut(&mut self, index: usize) -> &mut T { self.get_mut(index).expect("mesh row is present") }
}

#[derive(Default)]
pub(super) struct MeshRows2 {
    pub(super) points: MeshRows<[f64; 2]>,
    pub(super) tris: MeshRows<[u32; 3]>,
}

impl MeshRows2 {
    pub(super) fn new(mounted: bool) -> Self {
        Self { points: MeshRows::new(mounted), tris: MeshRows::new(mounted) }
    }
}
