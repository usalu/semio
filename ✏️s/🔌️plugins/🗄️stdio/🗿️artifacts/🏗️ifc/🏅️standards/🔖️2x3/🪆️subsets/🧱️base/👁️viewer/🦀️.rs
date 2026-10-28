//! 👁️ IFC 2x3 Any viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Ifc2x3AnyViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Ifc2x3AnyViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT reference the sibling editor module.

use crate::standards::v2x3::subsets::base::io::inspector;
use semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation;
use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use crate::viewer::ifc2x3_any::modes::view;
use crate::viewer::ifc2x3_any::modes::view::windows::main;
use semio_framework_plugin::app::InteractionView;
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
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Verified against this artifact's own `📸️snapshot/🦀️.rs`
/// `impl ArtifactAnalysis for …AnalyzerAnalysis { const DIALECT }` row (read, not guessed) — see
/// the packet report for the exact grep evidence per subset.
pub const IFC2X3_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId::ANY };
pub const IFC2X3_ANY_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Ifc2x3AnyViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Ifc2x3AnyViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Ifc2x3AnyViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Ifc2x3AnyViewer;

impl ArtifactViewer for Ifc2x3AnyViewer {
    type Snapshot = Ifc2x3Snapshot;
    type Mutation = Ifc2x3Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Ifc2x3AnyViewCommand;

    const DIALECT: Dialect = IFC2X3_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IFC2X3_ANY_DOCUMENT_SCHEMA;
    fn interaction_topology(doc:&ArtifactView<'_,Self::Snapshot>,_cfg:&ConfigView<'_,Self::Config>)->Result<protocol::InteractionTopology,semio_framework_value::ValueError>{Ok(inspector::topology(doc.snapshot))}

    fn reference_birth()->Option<fn()->Ifc2x3Snapshot>{Some(||{
        Ifc2x3Snapshot::default()
    })}

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        if body_key==inspector::BODY_KEY{return inspector::render(doc,_interaction,view_state,"s.stdio.ifc@2x3/*#viewer",false,cx).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into);}
        if body_key == main::BODY_KEY {
            return main::render_selected_in_context(doc.snapshot, cx,_interaction,inspector::DOMAIN,inspector::GRANULARITY).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into);
        }
        Self::render(body_key, doc, cfg, view_state).map(Some).map_err(Into::into)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_ifc2x3_any_viewer() -> semio_framework_plugin::AppDefinition {
    let mut definition=Viewer::builder(IFC2X3_ANY_DIALECT).document(["stdio", "ifc2x3"]).icon_id("box").mode_def(view::definition()).default_mode_id(view::IFC2X3_ANY_VIEW_MODE_ID).window_kind_def(main::definition()).window_kind_def(inspector::window_definition(false)).default_layout(view::layout()).build_definition();
    inspector::install_interaction(&mut definition);definition
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
