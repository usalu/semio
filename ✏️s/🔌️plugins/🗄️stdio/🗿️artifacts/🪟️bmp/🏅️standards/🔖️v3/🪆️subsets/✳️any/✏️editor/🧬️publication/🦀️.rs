//! 🧬️ BMP publication preparation: every BMP mutation is prepared by the central cold apply over the mutation's own cold semantics.
//!
//! The former hand-driven pixel-region cursor targeted the retired `RetainedCloneEditCursor` contract; its bounded-turn laws are to be restated against the current contract rather than carried as dead code.
use crate::{schema::mutations::BmpMutation, BmpSnapshot};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<BmpSnapshot,BmpMutation>>{Arc::new(RetainedClonePreparationFactory::new(Arc::new(BmpPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("bmp retained publication depth"))}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct BmpPublication;
impl RetainedCloneEdit<BmpSnapshot,BmpMutation> for BmpPublication{
    type Cursor=BmpPublicationCursor;
    fn preflight(&self,mutation:&BmpMutation,_lane:store::os_store::store_schema::HistoryLane)->Result<store::os_store::one_item::ArtifactStoreOneItemFootprint,&'static str>{Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<BmpSnapshot,_>(mutation,store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))}
    fn begin_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn snapshot_cursor_birth_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn begin(&self,grant:RetainedCloneGrant)->Result<(Self::Cursor,RetainedCloneProgress),ValueError>{let progress=self.begin_demand().admit(grant)?;Ok((BmpPublicationCursor::new(),progress))}
}
