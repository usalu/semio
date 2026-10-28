//! 🌱️ Empty persisted lanes declare a real empty record, canonical Pack and original fixed State birth.
use super::{NoConfig,NoConfigMutation};
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
use semio_framework_os_kernel as store;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
/// 🕳️ Config and draft differ by their declared Store identity and history shape.
pub struct EmptyLaneGenesisOwner<const DRAFT:bool>;
pub type NoConfigGenesisOwner=EmptyLaneGenesisOwner<false>;
pub type NoDraftGenesisOwner=EmptyLaneGenesisOwner<true>;
impl<const DRAFT:bool> OriginalFixedStoreInitial for EmptyLaneGenesisOwner<DRAFT>{
 type State=NoConfig;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn initial_state(grant:RetainedCloneGrant)->Result<(NoConfig,RetainedCloneProgress),ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"empty lane initial State requires its original grant"))}Ok((NoConfig{},RetainedCloneProgress{copied_items:1,..Default::default()}))}
}
impl<const DRAFT:bool> OriginalStoreGenesisOwner for EmptyLaneGenesisOwner<DRAFT>{
 type State=NoConfig;type Mutation=NoConfigMutation;type Catalog=store::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer;type Initial=FixedStoreInitial<Self>;
 const SCHEMA:&'static str=if DRAFT{"framework.empty.draft"}else{"framework.empty.config"};
 const MAXIMUM_INITIAL_PACK_BYTES:usize=1024;
 fn pack_envelope_id()->Option<&'static str>{Some(NoConfig::__DSL_ENVELOPE_ID)}
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["empty-",if DRAFT{"draft"}else{"config"},":",label]}
 fn history_shape()->store::HistoryShape{if DRAFT{store::HistoryShape::Document}else{store::HistoryShape::Config}}
 fn initial_state_pack_source(state:&NoConfig)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{Ok((state,<NoConfig as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}
}
