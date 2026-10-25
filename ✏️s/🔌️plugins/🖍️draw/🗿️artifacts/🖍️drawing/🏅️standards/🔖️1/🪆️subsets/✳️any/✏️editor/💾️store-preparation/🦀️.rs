//! 💾️ Drawing store preparation: the one item publication preparation factory, bounded by the drawing work items and otherwise the cold mutation preparation of the store.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory as _;

/// 🧮️ Work item bound shared by the bounded drawing commands and the store preparation.
pub const DRAWING_BOUNDED_WORK_ITEMS: usize = 4_096;

//#region 📬️StorePreparation
/// 📬️ The drawing document lane: the store's cold preparation behind the drawing work item bound.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DrawingArtifactStorePreparationFactory;

type DrawingCold = store::ColdMutationApplyFactory<DrawingSnapshot, DrawingMutation>;
type DrawingPreparationRequest = store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation, DrawingMutation>;
type DrawingPreparation = Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation>>;

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<DrawingMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<DrawingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        DrawingCold::default().begin_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &DrawingMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        DrawingCold::default().preflight(mutation, lane)
    }

    fn begin_demand(&self, mutation: &DrawingMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        DrawingCold::default().begin_demand(mutation, lane)
    }

    fn begin_copy_byte_demand(&self, mutation: &DrawingMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<usize, semio_framework_value::ValueError> {
        DrawingCold::default().begin_copy_byte_demand(mutation, lane)
    }

    fn begin(&self, request: DrawingPreparationRequest, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(DrawingPreparation, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, DrawingPreparationRequest)> {
        let items = request.base.get().layers.len().saturating_add(request.base.get().assets.len());
        if items > DRAWING_BOUNDED_WORK_ITEMS {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "drawing preparation exceeds its bounded work items"), request));
        }
        DrawingCold::default().begin(request, grant)
    }
}
//#endregion 📬️StorePreparation
