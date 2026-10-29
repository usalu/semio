//! 🧱️ The declared native constructor of the mounted instance: its assembly admission, the original App recipient and the opening factory.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
use semio_framework::kernel::ActorId;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_instance_stores::stores_components::ConstructorState;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::Fault;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use std::mem::ManuallyDrop;
use store::MemberFactory;
use semio_framework_plugin_mounted_owner::MountedOwnerPhaseV1;
use semio_framework_plugin_mounted_owner::MountedOwnerTurnV1;
use semio_framework_job::OriginalConstructionDemand;
use semio_framework_plugin_reactor_app::OriginalPluginAppConstruction;
use semio_framework_plugin_reactor_app::OriginalPluginAppFactory;
use semio_framework_plugin_reactor_app::OriginalPluginAppSource;
use semio_framework_job::OriginalSourceBinding;
use semio_framework_plugin_ownership::original_store_genesis::OriginalStoreInitialSource;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_window_kits::RegisteredDefinitionSource;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use store::ReferenceSpaceMember;
use semio_framework_job::StepContext;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_job::admit_original_construction;
use semio_framework_job::close_original_retained_step;
use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
use std::mem::size_of;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_instance_initialization::services::{OriginalInstanceLiveStores,OriginalInstanceServices,OriginalControllerClose,OriginalInstanceRegistration,OriginalInstanceRegistrationParts,invalid,construction};
use semio_framework_plugin_instance_initialization::stores::{OriginalArtifactAppBirth,OriginalArtifactStores,OriginalArtifactStoreSet};
#[path="🧺️sources/🦀️.rs"]
pub mod sources;
#[path="🏭️factory/🦀️.rs"]
pub mod factory;
#[path="📦️original/🦀️.rs"]
mod original;
pub use original::OriginalVcsApp;
pub use sources::{OriginalArtifactInstanceAdmission,OriginalArtifactInstanceSources};
