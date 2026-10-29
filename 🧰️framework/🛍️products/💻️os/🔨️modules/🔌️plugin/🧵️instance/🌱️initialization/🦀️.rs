//! 🌱️ Original instance initialization: the four native Stores, the ephemeral lanes, the framework service owners and the registration source that precede the mounted instance.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
use semio_framework::kernel::ActorId;
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use std::collections::BTreeMap;
use semio_framework_plugin_instance_ladders::ladders_components::CloseCursors;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use std::mem::ManuallyDrop;
use store::MemberFactory;
use store::Mutation;
use crate::ephemeral::OriginalEmptyEphemeralSource;
use crate::ephemeral::OriginalEmptyEphemeralStores;
use semio_framework_job::OriginalSourceBinding;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_plugin_window_kits::RegisteredDefinitionSource;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use store::ReferenceSpaceMember;
use semio_framework_job::StepContext;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_value::ValueError;
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_job::admit_original_construction;
use semio_framework_job::close_original_retained_step;
use std::mem::size_of;
#[path="🏪️stores/🦀️.rs"]
pub mod stores;
#[path="🫧️ephemeral/🦀️.rs"]
pub mod ephemeral;
#[path="🧾️services/🦀️.rs"]
pub mod services;
use services::{OriginalInstanceLiveStores,OriginalInstanceServices,OriginalInstanceServiceSources,OriginalInstanceRegistration,OriginalInstanceRegistrationParts,invalid,construction};
use stores::{OriginalArtifactAppBirth,OriginalArtifactStores};
#[path="🎟️registration/🦀️.rs"]
pub mod registration;
