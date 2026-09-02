//! 👁️ S Home viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `HomeViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<HomeViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::home::{SHomeSnapshot, HOME_DIALECT, S_HOME_DOCUMENT_SCHEMA};
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use crate::viewer::home::modes::view;
use crate::viewer::home::modes::view::windows::main;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Fault, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, PluginAssemblyError, UiAssemblyResult, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the viewer's config now
/// carries the folded hub directory read model (`HomeConfig`, shared with the editor — see
/// `HomeViewer::Config` below), so this Home viewer session ALSO needs to receive folded directory
/// events when it is the currently-mounted session (contract §C6: the shell's directory lane folds into
/// whichever session is mounted, editor or viewer). `Noop` stays the `Default` variant
/// (`assert_viewer_never_mutates` requires `Command: Default` and only ever dispatches the default).
#[derive(Clone, Debug, PartialEq, Default)]
pub enum HomeViewCommand {
    #[default]
    Noop,
    /// 📇️ Mirrors the editor's `fold-directory-events` command — a config-only fold, never an
    /// artifact/draft mutation (structurally impossible here: `ViewEmit` has no such field).
    FoldDirectoryEvents { events_json: String },
}

impl protocol::OpBinary for HomeViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match self {
            HomeViewCommand::Noop => Ok(vec![0]),
            HomeViewCommand::FoldDirectoryEvents { events_json } => {
                let mut out = vec![1u8];
                out.extend_from_slice(events_json.as_bytes());
                Ok(out)
            }
        }
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        match bytes.first() {
            Some(1) => Ok(HomeViewCommand::FoldDirectoryEvents { events_json: String::from_utf8_lossy(&bytes[1..]).into_owned() }),
            _ => Ok(HomeViewCommand::Noop),
        }
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct HomeViewer;

impl ArtifactViewer for HomeViewer {
    type Snapshot = SHomeSnapshot;
    type Mutation = crate::artifacts::home::op::SHomeMutation;
    // 📇️ Shared with the editor (`crate::editor::home::config::HomeConfig`), not `NoConfig` — the
    // viewer renders the SAME hub-directory-fed table (`crate::home_space_rows`) and must therefore
    // read the SAME folded `directory_json`. `assert_viewer_never_mutates` only asserts the ARTIFACT/
    // draft store never advances (contract §2.5) — the config lane is fair game for both surfaces, and
    // a viewer emitting a `ConfigMutation` is not a document mutation.
    type Config = HomeConfig;
    type ConfigMutation = HomeConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = HomeViewCommand;

    const DIALECT: Dialect = HOME_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = S_HOME_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SHomeSnapshot {
        SHomeSnapshot::default()
    }

    /// 🪪️ Same app-schema descriptor as the editor (contract requires both surfaces sharing a dialect
    /// to also share a config schema, since it is registered per-document-schema, not per-role).
    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    /// 👁️ Structurally read-only: neither variant ever carries an artifact/draft mutation (`ViewEmit`
    /// has no such field to carry one in). `Noop` returns the empty emit; `FoldDirectoryEvents` folds
    /// each event into `HomeConfigMutation::FoldDirectoryEvent`, the SAME config-only writer the editor
    /// uses — never an optimistic mutation, never a document edit.
    fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        match command {
            HomeViewCommand::Noop => Ok(ViewEmit::default()),
            HomeViewCommand::FoldDirectoryEvents { events_json } => {
                let events: Vec<store::os_directory::DirectoryEvent> = pack::from_json_str(events_json).unwrap_or_default();
                let config_mutations = events.iter().filter_map(|event| Some(pack::to_json_string(event))).map(|event_json| HomeConfigMutation::FoldDirectoryEvent { event_json }).collect();
                Ok(ViewEmit::config(config_mutations))
            }
        }
    }

    /// 👁️ Renders the SAME overview table the editor's main window does, read-only: no create/delete/
    /// rename/share affordances, fed by `cfg.snapshot.directory()` — never the artifact document itself.
    fn render(body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> UiAssemblyResult<ComponentTree> {
        let root = match body_key {
            main::S_HOME_VIEW_BODY => main::render(&cfg.snapshot.directory(), &cfg.snapshot.locale)?,
            _ => semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| PluginAssemblyError::new("s.home.viewer.render.unknown-body", "unknown body key text admission failed"))?,
        };
        Ok(ComponentTree { root })
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_home_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(HOME_DIALECT).document(["semio", "s", "home"]).icon_id("home").mode_def(view::definition()).default_mode_id(view::S_HOME_VIEW_MODE).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_home_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_home_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, HOME_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<HomeViewer as ArtifactViewer>::DIALECT, HOME_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_the_main_body_key_for_the_default_snapshot() {
        let snapshot = SHomeSnapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = HomeConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let _node = <HomeViewer as ArtifactViewer>::render(main::S_HOME_VIEW_BODY, &doc, &cfg);
    }

    #[semio_framework_async_macros::async_test]
    async fn fold_directory_events_command_never_touches_the_document_store() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<HomeViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_and_viewer_share_one_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::home::HomeApp, HomeViewer>();
    }
}
//#endregion 🧪️Tests
