//! 🎚️ Exact-window persisted-local configuration with independent event-stream authority.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;

use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use std::any::Any;
use crate::window_mutation::ErasedWindowMutationValue;
use semio_framework_value::{FactoryBoxedValue,FactoryBoxedPublication,retirement::controlled::ControlledRetirement};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[path="🧬️preview/🦀️.rs"]
mod semantic_preview;
use semantic_preview::WindowSemanticPreview;

pub use semio_framework_plugin_window_config_replace::{WindowConfigReplaceCursor,WindowConfigReplaceEdit,WindowConfigReplaceMutation};
#[path="🧬️preparation/✏️apply/🦀️.rs"]
mod apply_edit;
pub use apply_edit::{WindowConfigApplyCursor,WindowConfigApplyEdit,WindowConfigApplyMutation};

#[path = "📥️retained/🦀️.rs"]
pub mod retained;
#[path = "🗂️registry/🦀️.rs"]
pub mod registry;
#[path = "📬️mutation/🦀️.rs"]
pub mod window_mutation;
use registry::WindowRegistry;
pub use retained::{WindowConfigPackLoad, WindowConfigPackLoadDiagnostic, WindowConfigPackLoadPhase, WindowConfigPackLoadProgress, WindowConfigPackLoadStep};

#[path = "📐️owner-contract-and-mutation/🦀️.rs"]
mod owner_contract_and_mutation;
pub use owner_contract_and_mutation::{RejectedWindowConfigEmission, WindowConfigAuthorityIdentity, WindowConfigMutation, WindowConfigOwner, WindowConfigSnapshot, bounded_window_config_preparation_factory, bounded_window_config_preparation_source_birth, bounded_window_config_store_disposer, bounded_window_config_store_owners};

#[path = "🔏️pack-authority-and-publication/🦀️.rs"]
mod pack_authority_and_publication;
pub use pack_authority_and_publication::{ErasedWindowConfigPublication, TypedWindowConfigPublication, WindowConfigAuthority, WindowConfigPack};

#[path = "🗄️store-owners/🦀️.rs"]
mod store_owners;
pub use store_owners::{DirectIngress, TypedWindowConfigStoreOwner};
use store_owners::{ErasedWindowConfigStoreOwner, WindowConfigPartition};

#[path = "🧭️owner-registry/🦀️.rs"]
mod owner_registry;
pub use owner_registry::{WINDOW_CONFIG_PACK_LOAD_TURNS, WindowConfigOwnerRegistry};
use owner_registry::{close_window_actor, controlled_owner_demands, window_actor_demands};

#[cfg(test)]
#[path="🏠️catalog/🧪️tests/🦀️.rs"]
mod original_catalog_tests;
