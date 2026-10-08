//! ♻️ Typed Drawing retirement with separately admitted payload, scaffold and backing work.

use super::*;

#[derive(semio_framework_value::RetireOwned)]
pub(super) enum DrawingRetirementOwner {
    Snapshot(DrawingSnapshot),
    Mutation(DrawingMutation),
    Layer(DrawingLayerNode),
    Fill(FillStyle),
    Stroke(StrokeStyle),
    String(semio_framework_value::paged::PagedUtf8<{usize::MAX}>),
    Segments(semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>),
    SegmentCollections(semio_framework_value::list::PagedList<semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>, {usize::MAX}>),
    HistoryId(String),
}

pub(super) struct DrawingOwnedRetirement {
    owner: DrawingDecodedFieldRetirement<DrawingRetirementOwner>,
}

impl DrawingOwnedRetirement {
    pub(super) fn new(owner: DrawingRetirementOwner) -> Self {
        Self { owner: DrawingDecodedFieldRetirement::try_new(owner).unwrap_or_else(|_| panic!("Drawing native owner lacks typed retirement authority")) }
    }

    pub(super) fn next_grant(&self) -> Result<RetainedCloneGrant, semio_framework_value::ValueError> { self.owner.next_grant() }

    pub(super) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> { self.owner.step_granted(grant) }
}

impl store::ErasedSnapshotRetirement for DrawingOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        let step = self.owner.step(maximum_items, maximum_bytes)?;
        match step {
            RetainedCloneStep::Progress(progress) => Ok(store::SnapshotRetirementStep::Pending { released_items: progress.copied_items, released_bytes: progress.released_bytes }),
            RetainedCloneStep::Complete(_) => Ok(store::SnapshotRetirementStep::Complete),
        }
    }

    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }

    fn next_close_byte_demand(&self) -> usize { self.owner.next_close_byte_demand().expect("Drawing typed retirement has an exact next demand") }
}
