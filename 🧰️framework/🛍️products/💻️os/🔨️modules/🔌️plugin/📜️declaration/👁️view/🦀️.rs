//! 👁️ The read views an app receives, its operation contexts and the empty state lanes.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path="staged-form/🦀️.rs"]
pub mod staged_form;

#[doc(hidden)]
#[path = "../🪆️child/🦀️.rs"]
pub mod child_view;
#[doc(hidden)]
#[path = "../🛠️tool-machine/🦀️.rs"]
pub mod gesture_contract;
#[doc(hidden)]
#[path = "../🎮️command/🦀️.rs"]
pub mod history_view;
#[doc(hidden)]
#[path = "../🎞️media/🦀️.rs"]
pub mod media_contract;
#[doc(hidden)]
#[path = "../⏯️tool-run/🦀️.rs"]
pub mod tool_run_contract;
#[doc(hidden)]
#[path = "../../👥️presence/♻️retirement/🦀️.rs"]
pub mod presence_retirement;
#[doc(hidden)]
#[path = "../../🎚️config/🚫️none/♻️retirement/🦀️.rs"]
pub mod no_config_retirement;
#[doc(hidden)]
#[path = "../../📝️draft/🚫️none/♻️retirement/🦀️.rs"]
pub mod no_draft_retirement;
#[doc(hidden)]
#[path = "../../🫧️transient/♻️retirement/🦀️.rs"]
pub mod transient_retirement;
#[doc(hidden)]
#[path = "🛬️lifted/👷️job/🦀️.rs"]
pub mod tool_contract;
use semio_framework::ActionKind;
use semio_framework::ActionDefinition;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
use semio_framework::Fault;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use std::collections::BTreeMap;
use pack::snapshot::ArtifactPack;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::child_view::ChildContentView;
use crate::history_view::HistoryView;
use semio_framework_plugin_interaction::interaction_topology::empty_domain_hover;
use semio_framework_plugin_interaction::interaction_topology::empty_domain_selection;
use semio_framework_plugin_interaction::interaction_topology::leftover_selected_ids_of;

use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use crate::tool_contract::ArtifactInstanceOperationOwnerHandle;

use crate::tool_run_contract::ToolRunView;
pub use crate::tool_run_contract::ToolRunPollView;
use semio_framework_plugin_interaction::UiAssemblyResult;

#[path = "🔭️artifact-view/🦀️.rs"]
mod artifact_view;
pub use artifact_view::{ArtifactPendingView, ArtifactView};

#[path = "🪪️operation-contexts/🦀️.rs"]
mod operation_contexts;
pub use operation_contexts::{
    artifact_render_publication_revision, render_operation_publication_revision,
    AppOperationContext, AppRenderOperationContext, MountedSnapshotReadPreparation,
};

#[path = "🔍️lane-views/🦀️.rs"]
mod lane_views;
pub use lane_views::{ConfigView, DraftView, InteractionView, PresencePeersView, PresenceView};

#[path = "🕳️empty-lanes/🦀️.rs"]
mod empty_lanes;
pub use empty_lanes::{
    NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation,
    NoStateDiff, NoTransient, NoTransientMutation,
};

#[path = "🫂️peer-presence/🦀️.rs"]
mod peer_presence;
pub use peer_presence::{
    cancellation_action_definition, InteractionHoverState, PeerMark, PeerPresence,
    PeerPresenceEntry, PeerPresenceRoot, PEER_PRESENCE_SLOTS,
};

#[cfg(test)]
#[path = "🧪️tests/🪪️entity-id/🦀️.rs"]
mod entity_id_tests;

#[cfg(test)]
#[path = "🧪️tests/🪪️render-revision/🦀️.rs"]
mod render_revision_tests;
