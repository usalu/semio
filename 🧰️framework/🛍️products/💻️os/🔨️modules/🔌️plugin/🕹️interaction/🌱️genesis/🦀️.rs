//! 🌱️ The framework interaction lane declares its actual empty native State, intrinsic Pack wire and retirement catalog.
use semio_framework_os_kernel as store;
use semio_framework_plugin_ownership::original_store_genesis::{OriginalStoreGenesisOwner,OriginalFixedStoreInitial,FixedStoreInitial};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::size_of;
struct EmptyInteraction;
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for EmptyInteraction{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;match path{[]=>Ok(V::Record(&[1])),[0]=>Ok(V::IntrinsicObject(4)),[0,index]if *index<4=>Ok(V::IntrinsicObject(0)),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"empty native interaction genesis path differs"))}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[0]{["activeGranularity","activeMode","hover","selection"].get(index).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"empty native interaction key differs"))}else{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"empty native interaction key path differs"))}}
}
/// 🧰️ Empty interaction admission has a real retirement catalog and no operation replay or publication authority.
pub struct InteractionCatalog;
impl store::os_store::document_store_owners::DocumentStoreOwnerProducer<store::InteractionState,crate::mutations::InteractionConfigMutation> for InteractionCatalog{
 fn source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<store::InteractionState,crate::mutations::InteractionConfigMutation>()}
 fn admit_sources(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<store::InteractionState,crate::mutations::InteractionConfigMutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<store::InteractionState,crate::mutations::InteractionConfigMutation>>{store::os_store::document_store_owners::bounded_artifact_store_owners(grant)}
}
/// 🕹️ The framework's actual empty interaction source agrees with its existing native Pack wire.
pub struct InteractionGenesis;
impl OriginalStoreGenesisOwner for InteractionGenesis{
 type State=store::InteractionState;type Mutation=crate::mutations::InteractionConfigMutation;type Catalog=InteractionCatalog;type Initial=FixedStoreInitial<Self>;
 const SCHEMA:&'static str="framework.interaction";const MAXIMUM_INITIAL_PACK_BYTES:usize=4096;
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["","","",label]}
 fn history_shape()->store::HistoryShape{store::HistoryShape::Config}
 fn initial_state_pack_source(state:&Self::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{if !state.active_granularity.is_empty()||!state.active_mode.is_empty()||!state.hover.is_empty()||!state.selection.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native empty interaction source retains authored empty maps"))}use semio_framework_dsl_record::{BorrowedRecordSpec,BorrowedFieldSpec,BorrowedShape,RecordLayout};const FIELDS:&[BorrowedFieldSpec]=&[BorrowedFieldSpec::new(1,"value",BorrowedShape::Value)];Ok((&EmptyInteraction,BorrowedRecordSpec{keyword:None,layout:RecordLayout::Lines,fields:FIELDS}))}
}
impl OriginalFixedStoreInitial for InteractionGenesis{type State=store::InteractionState;
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<Self::State>(),depth:1,..Default::default()})}
 fn initial_state(grant:RetainedCloneGrant)->Result<(Self::State,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original empty interaction source is unfunded"))}Ok((Self::State::default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
}
