//! 🎟️ Retained paged writer steps share one storage bridge with their standalone entry points.
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, work::RetainedWorkLedger};
use semio_framework_value_persistent::list::PagedList;
use semio_s_artifact_draw_drawing_io::byte_writer::reserve;

pub(crate) enum Storage<'a, 'effect> {
    Standalone,
    Original(&'a mut RetainedWorkLedger, &'a mut RetainedAllocationEffect<'effect>),
}
impl Storage<'_, '_> {
    pub(crate) fn slot<T>(&mut self, list: &mut PagedList<T, {usize::MAX}>, message: &'static str) -> Result<bool, &'static str> {
        match self {
            Self::Standalone => reserve(list).map_err(|_| message),
            Self::Original(work, effect) => work.slot(list, effect).map_err(|_| message),
        }
    }
    pub(crate) fn push<T>(&mut self, list: &mut PagedList<T, {usize::MAX}>, value: T) -> Result<(), T> {
        match self {
            Self::Standalone => list.push_reserved(value),
            Self::Original(work, _) => work.push(list, value).map_err(|(_, value)| value),
        }
    }
    pub(crate) fn page(&mut self, page: &mut Vec<u8>, capacity: usize, message: &'static str) -> Result<bool, &'static str> {
        match self {
            Self::Standalone => {
                page.try_reserve_exact(capacity).map_err(|_| message)?;
                Ok(true)
            }
            Self::Original(work, effect) => work.contiguous(page, capacity, effect.actual()).map_err(|_| message),
        }
    }
    pub(crate) fn record_child(&mut self, progress: RetainedCloneProgress) -> Result<(), &'static str> {
        match self {
            Self::Standalone => Ok(()),
            Self::Original(work, _) => work.consume_child(progress),
        }
    }
    pub(crate) fn copy(&mut self, bytes: usize) -> bool {
        match self {
            Self::Standalone => true,
            Self::Original(work, _) => work.copy_bytes(bytes),
        }
    }
}
