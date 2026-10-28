//! 🎟️ Window genesis declares its schema, identity and native source on the shared original Store birth.
use super::*;
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesis,OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
/// 🪟️ The Window owner supplies the same declared state and semantic catalog used by its native replay.
pub struct WindowGenesisOwner<O>(std::marker::PhantomData<fn()->O>);
impl<O:WindowConfigOwner> OriginalStoreGenesisOwner for WindowGenesisOwner<O>{
 type State=O::State;type Mutation=O::Mutation;type Catalog=WindowConfigInitializationOwners<O>;type Initial=FixedStoreInitial<Self>;
 fn pack_envelope_id()->Option<&'static str>{Some(<O::State as pack::snapshot::ArtifactDsl>::envelope_id())}
 const SCHEMA:&'static str=O::SCHEMA;const MAXIMUM_INITIAL_PACK_BYTES:usize=O::MAXIMUM_INITIAL_PACK_BYTES;
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["window-config:",O::WINDOW_KIND_ID,":",label]}
 fn history_shape()->protocol::HistoryShape{protocol::HistoryShape::Config}
 fn initial_state_pack_source(state:&O::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{O::initial_state_pack_source(state)}
}
impl<O:WindowConfigOwner> OriginalFixedStoreInitial for WindowGenesisOwner<O>{type State=O::State;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{O::initial_state_demands()}
 fn initial_state(grant:RetainedCloneGrant)->Result<(O::State,RetainedCloneProgress),ValueError>{O::initial_state(grant)}
}
/// 🌱️ Every Window partition uses the same original genesis, Pack, native replay and cancellation state machine.
pub type WindowConfigPartitionBirth<O>=OriginalStoreGenesis<WindowGenesisOwner<O>>;
