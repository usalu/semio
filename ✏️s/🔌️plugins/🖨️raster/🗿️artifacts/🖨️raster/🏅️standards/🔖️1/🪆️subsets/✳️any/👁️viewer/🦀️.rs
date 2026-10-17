//! 👁️ Raster viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `RasterViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<RasterViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use crate::modes::view;
use crate::modes::view::windows::{composite, navigator};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_model::{RASTER_DIALECT, RASTER_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_2d::compute::EngineHandles;

//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct RasterViewer;

impl ArtifactViewer for RasterViewer {
    type Snapshot = RasterSnapshot;
    type Mutation = semio_s_artifact_raster_raster_mutations::RasterMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = RasterViewCommand;

    const DIALECT: Dialect = RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields.
    /// Without it every live envelope load faults with `viewer did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store. `assets` declares no child slot
    /// (see `🧬️schema/📸️snapshot`), so the projection is honestly empty.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("raster.child-projection"), error.to_string()))
    }

    /// 🔐️ The artifact's own document-store owner catalogue, identical to the sibling editor's: a viewer holds the same
    /// snapshot and must retire its owned values the same way, never through the framework's generic bounded owners.
    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_artifact_raster_raster_host::raster_document_store_owners_source_demands()
    }

    fn build_document_store_owners(
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(semio_s_artifact_raster_raster_host::raster_document_store_owners_admission(grant))
    }

    /// 📄️ Boots on the constant empty shell `empty_raster_snapshot()`, like the editor: the store's
    /// construction derive-`Clone`s and `encode_pack`s the initial snapshot, and raster admits only
    /// the empty shell to both — see `RasterPlayApp::initial_snapshot`. A viewer instance receives its
    /// document through the archive/pack load path (raster's own paged clone authority).
    fn initial_snapshot() -> RasterSnapshot {
        semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `RasterViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is a pure
    /// addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            composite::RASTER_VIEW_BODY_COMPOSITE => composite::render(doc.snapshot)?,
            navigator::RASTER_VIEW_BODY_NAVIGATOR => navigator::render(doc.snapshot)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster viewer unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod config {
    #[path = "🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod modes {
    #[path = "🎭️modes/👁️view/🦀️.rs"]
    pub mod view;
}
#[path = "."]
pub mod presence {
    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}
//#endregion 🪢️TaxonomyMounts

#[path = "🧱️command-and-manifest/🦀️.rs"]
mod command_and_manifest;
pub use command_and_manifest::*;
