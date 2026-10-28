//! 🌱️ Concrete App declarations authorize four original genesis sources and their own retained controller closure.
use crate::ArtifactApp;
use semio_framework_plugin_ownership::original_store_genesis::OriginalStoreGenesisOwner;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use semio_framework_os_kernel as store;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{RetirementDemand,ValueError,retirement::RetireOwned};
/// 🧬️ Native instance admission requires four concrete authored genesis vocabularies and their exact catalogs.
pub trait OriginalArtifactAppBirth:ArtifactApp+RetireOwned{
 type DocumentGenesis:OriginalStoreGenesisOwner<State=Self::Snapshot,Mutation=Self::Mutation>;
 type ConfigGenesis:OriginalStoreGenesisOwner<State=Self::Config,Mutation=Self::ConfigMutation>;
 type DraftGenesis:OriginalStoreGenesisOwner<State=Self::Draft,Mutation=Self::DraftMutation>;
 type InteractionGenesis:OriginalStoreGenesisOwner<State=store::InteractionState,Mutation=InteractionConfigMutation>;
 fn controller_birth_demands(app_id:&str)->Result<RetirementDemand,ValueError>;
 fn admit_controller(app_id:&str,grant:semio_framework_value::RetainedCloneGrant)->Result<(Self,semio_framework_value::RetainedCloneProgress),ValueError> where Self:Sized;
 fn controller_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn controller_close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn controller_terminal_is_empty(&self)->bool;
}
