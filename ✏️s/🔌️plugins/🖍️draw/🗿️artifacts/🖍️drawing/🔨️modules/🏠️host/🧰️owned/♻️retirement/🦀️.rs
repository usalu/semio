//! ♻️ Typed Drawing retirement with separately admitted payload, scaffold and backing work.


#[derive(semio_framework_value::RetireOwned)]
pub enum DrawingRetirementOwner {
    Snapshot(DrawingSnapshot),
    Asset(DrawingImageAsset),
    Mutation(DrawingMutation),
    Layer(DrawingLayerNode),
    Fill(FillStyle),
    Stroke(StrokeStyle),
    String(semio_framework_value::paged::PagedUtf8<{usize::MAX}>),
    Segments(semio_framework_value_persistent::list::PagedList<PathSegment, {usize::MAX}>),
    SegmentCollections(semio_framework_value_persistent::list::PagedList<semio_framework_value_persistent::list::PagedList<PathSegment, {usize::MAX}>, {usize::MAX}>),
    HistoryId(String),
}
