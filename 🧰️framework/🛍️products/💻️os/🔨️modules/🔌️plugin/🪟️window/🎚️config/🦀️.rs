//! 🎚️ Exact-window persisted-local configuration with independent event-stream authority.
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as pack;

use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use crate::window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[doc(hidden)]
#[path="🧬️preview/🦀️.rs"]
pub mod semantic_preview;
pub use semantic_preview::WindowSemanticPreview;

pub use semio_plugin_window_config_replace::{WindowConfigReplaceCursor,WindowConfigReplaceEdit,WindowConfigReplaceMutation};
#[path="🧬️preparation/✏️apply/🦀️.rs"]
mod apply_edit;
pub use apply_edit::{WindowConfigApplyCursor,WindowConfigApplyEdit,WindowConfigApplyMutation};

#[path = "📥️retained/🦀️.rs"]
pub mod retained;
pub use retained::WindowConfigCommandLoad;
use semio_plugin_window_config_registry as registry;
use semio_plugin_window_config_mutation as window_mutation;
use registry::WindowRegistry;
pub use retained::{WindowConfigPackLoad, WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress, WindowConfigPackLoadStep};

#[doc(hidden)]
#[path = "📐️owner-contract-and-mutation/🦀️.rs"]
pub mod owner_contract_and_mutation;
pub use owner_contract_and_mutation::{WindowConfigNativePairReceiving,WindowConfigHistorySource,window_config_native_pair_demands,admit_window_config_native_pair,RejectedWindowConfigEmission, WindowConfigAuthorityIdentity, WindowConfigBorrow, WindowConfigMutation, WindowConfigOwner, WindowConfigSnapshot, bounded_window_config_preparation_factory, bounded_window_config_preparation_source_birth, bounded_window_config_store_disposer,bounded_window_config_store_disposer_birth, bounded_window_config_store_owners};

#[doc(hidden)]
#[path = "🔏️pack-authority-and-publication/🦀️.rs"]
pub mod pack_authority_and_publication;
pub use pack_authority_and_publication::{ErasedWindowConfigPublication, TypedWindowConfigPublication, WindowConfigAuthority, WindowConfigPack,WindowConfigPackSource};

#[doc(hidden)]
#[path = "🗄️store-owners/🦀️.rs"]
pub mod store_owners;
pub use store_owners::{DirectIngress, TypedWindowConfigStoreOwner, WindowConfigInitializationOwners, WindowConfigPartitionBirth};
pub use store_owners::{ErasedWindowConfigStoreOwner, WindowConfigPartition, WindowConfigStore};

#[doc(hidden)]
#[path = "🧭️owner-registry/🦀️.rs"]
pub mod owner_registry;
pub use owner_registry::WindowConfigOwnerRegistry;
pub use owner_registry::{close_window_actor, controlled_owner_demands, window_actor_demands};

#[cfg(test)]
extern crate self as semio_plugin_window_config;
#[cfg(test)]
#[path="🏠️catalog/🧪️tests/🦀️.rs"]
mod original_catalog_tests;

#[cfg(test)]
#[path="🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🦀️.rs"]
mod original_genesis_tests;
#[cfg(test)]
#[global_allocator]
static ORIGINAL_WINDOW_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

pub use owner_registry::OriginalWindowOwnerRegistration;


