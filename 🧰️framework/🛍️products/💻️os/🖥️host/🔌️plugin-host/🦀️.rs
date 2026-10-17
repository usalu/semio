//! 🔌️ Plugin host, studio document VCS store, backbone, and catalog.
// #region host

use crate::instance::{create_os_id, OsInstanceState};
use crate::registry::{os_app_registration, resolve_kernel_future, resolve_os_app_definition, PluginRegistry};
use crate::space;
use crate::workflow;
use protocol::Mutation;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework::{AppDefinition, PluginManifest, ProgramContributionEntry, ViewModel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::document_backbone_ref, os_store::document_materialize::materialize_document_snapshot, os_store::store_schema::ArtifactBackboneRef, os_store::store_schema::ArtifactCommand, os_store::store_schema::ArtifactEnvelope, ArtifactStore};
use ui_wgpu::wgpu::{ui_recovery_panel, UiNode};
use vcs::{ArtifactVcs, VcsError};

#[path = "🔁️host-lifecycle/🦀️.rs"]
mod host_lifecycle;
pub use host_lifecycle::*;

#[path = "🗃️backbone-document/🦀️.rs"]
mod backbone_document;
pub use backbone_document::*;

#[path = "🩹️workflow-reconcile/🦀️.rs"]
mod workflow_reconcile;
use workflow_reconcile::*;

#[path = "🏪️workflow-store/🦀️.rs"]
mod workflow_store;
pub use workflow_store::*;

#[path = "🔗️backbone-ports/🦀️.rs"]
mod backbone_ports;
pub use backbone_ports::*;

#[path = "📚️space-catalog/🦀️.rs"]
mod space_catalog;
pub use space_catalog::*;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "🧩️extension-install/🦀️.rs"]
mod extension_install;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use extension_install::*;

// 🫀️ Presence used to be a `presence:` backbone-URI polling hack (`OS_PRESENCE_URI_PREFIX` /
// `write_os_presence` / `read_os_presence_peers`) — deleted. Presence now flows through the semio_hub's
// duplex `PresencePeer`/`HubServerFrame::Presence` frames (`framework/core/rs`'s 🔖️HubProtocol
// region) via `framework/sync`'s `ArtifactHost::subscribe` yielding `ArtifactEvent::Presence`. 🎯️
// ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C8.4: the old
// `presence_peers_json`/`ViewModel.presence_peers_json` JSON-array bridge is DELETED — plugins
// now receive the roster through the object-safe app trait's `adopt_presence` (§C7.6, the ONLY
// plugin ingress for peers) via `AppCommand::Presence`, not a `ViewModel` field a native host
// pre-translates.

#[cfg(test)]
include!("../🧪️tests/🔬️host-unit/🦀️.rs");
// #endregion host
