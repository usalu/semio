//! 🧬️ PNG publication preparation: every PNG mutation is prepared by the central cold apply over the mutation's own cold semantics.
//!
//! The former hand-driven pixel-region cursor targeted the retired `RetainedCloneEdit` cursor contract; its bounded-turn laws are to be restated against the current contract (see the png ticket notes) rather than carried as dead code.
use crate::{schema::mutations::PngMutation, PngSnapshot};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<PngSnapshot,PngMutation>>{Arc::new(RetainedClonePreparationFactory::new(Arc::new(PngPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("png retained publication depth"))}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct PngPublication;
impl RetainedCloneEdit<PngSnapshot,PngMutation> for PngPublication{
    type Cursor=PngPublicationCursor;
    fn preflight(&self,mutation:&PngMutation,_lane:store::os_store::store_schema::HistoryLane)->Result<store::os_store::one_item::ArtifactStoreOneItemFootprint,&'static str>{Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<PngSnapshot,_>(mutation,store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))}
    fn begin_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn snapshot_cursor_birth_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn begin(&self,grant:RetainedCloneGrant)->Result<(Self::Cursor,RetainedCloneProgress),ValueError>{let progress=self.begin_demand().admit(grant)?;Ok((PngPublicationCursor::new(),progress))}
}
