//! 📜️ Window config contract: the owner trait with its bounded factories, the mutation and snapshot, and the pack, authority and publication they travel in.
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as pack;

use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework::Fault;
use std::any::Any;
use semio_plugin_window_config_mutation as window_mutation;
use window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[doc(hidden)]
#[path = "📐️owner-contract-and-mutation/🦀️.rs"]
pub mod owner_contract_and_mutation;
pub use owner_contract_and_mutation::{WindowConfigNativePairReceiving,WindowConfigHistorySource,window_config_native_pair_demands,admit_window_config_native_pair,RejectedWindowConfigEmission, WindowConfigAuthorityIdentity, WindowConfigBorrow, WindowConfigMutation, WindowConfigOwner, WindowConfigSnapshot, bounded_window_config_preparation_factory, bounded_window_config_preparation_source_birth, bounded_window_config_store_disposer,bounded_window_config_store_disposer_birth, bounded_window_config_store_owners};

#[doc(hidden)]
#[path = "🔏️pack-authority-and-publication/🦀️.rs"]
pub mod pack_authority_and_publication;
pub use pack_authority_and_publication::{ErasedWindowConfigPublication, TypedWindowConfigPublication, WindowConfigAuthority, WindowConfigPack,WindowConfigPackSource};
