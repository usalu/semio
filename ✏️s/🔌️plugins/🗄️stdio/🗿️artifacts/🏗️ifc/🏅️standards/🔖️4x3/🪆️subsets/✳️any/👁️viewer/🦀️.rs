//! 👁️ IFC4x3 viewer: the read-only counterpart of the editor, rendering the representation-derived scene and the inspector. It never references the sibling editor module.

use semio_framework_2d_compute::EngineHandles;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, MeshWindowKit, ModeDefinition, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use semio_framework_ui_locale::{Label, LocalizedLabel};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation;

pub const IFC4X3_ANY_VIEWER_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4x3"), subset: SubsetId::ANY };
pub const IFC4X3_VIEWER_DOCUMENT_SCHEMA: &str = "s.stdio.ifc@4x3/*";
pub const IFC4X3_VIEWER_CONTROLLER: &str = "s.stdio.ifc@4x3/*#viewer";
pub const IFC4X3_VIEW_MODE_ID: &str = "view";

/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Ifc4x3ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Ifc4x3ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Ifc4x3ViewCommand::Noop)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Ifc4x3AnyViewer;

impl ArtifactViewer for Ifc4x3AnyViewer {
    type Snapshot = IfcSnapshot;
    type Mutation = IfcMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Ifc4x3ViewCommand;

    const DIALECT: Dialect = IFC4X3_ANY_VIEWER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IFC4X3_VIEWER_DOCUMENT_SCHEMA;

    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
        Ok(semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::inspector::topology(doc.snapshot))
    }

    fn reference_birth() -> Option<fn() -> Result<IfcSnapshot, Fault>> {
        Some(|| Ok(IfcSnapshot::default()))
    }

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
        interaction: &InteractionView<'_>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let inspector = semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::inspector::BODY_KEY;
        if body_key == inspector {
            return semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::inspector::render(doc, interaction, view_state, IFC4X3_VIEWER_CONTROLLER, false, cx).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into);
        }
        if body_key == MeshWindowKit::KIND_ID {
            return semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::geometry::render_selected_in_context(doc.snapshot, cx, interaction).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into);
        }
        Self::render(body_key, doc, cfg, view_state).map(Some).map_err(Into::into)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            body if body == MeshWindowKit::KIND_ID => semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::geometry::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

pub fn create_ifc4x3_any_viewer() -> semio_framework_plugin::AppDefinition {
    let mode = ModeDefinition { id: IFC4X3_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() };
    let inspector = semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::inspector::window_definition(false);
    let layout = semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::inspector::layout(MeshWindowKit::KIND_ID, false);
    let mut definition = Viewer::builder(IFC4X3_ANY_VIEWER_DIALECT).document(["stdio", "ifc4x3"]).icon_id("box").mode_def(mode).default_mode_id(IFC4X3_VIEW_MODE_ID).window_kind_def(MeshWindowKit::window_kind()).window_kind_def(inspector).default_layout(layout).build_definition();
    semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::inspector::install_interaction(&mut definition);
    definition
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_is_read_only_and_shares_the_editor_coordinate() {
        let definition = create_ifc4x3_any_viewer();
        assert_eq!(definition.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(definition.dialect, IFC4X3_ANY_VIEWER_DIALECT.into());
        assert_eq!(<Ifc4x3AnyViewer as ArtifactViewer>::DOCUMENT_SCHEMA, crate::editor::IFC4X3_DOCUMENT_SCHEMA);
        assert_eq!(<Ifc4x3AnyViewer as ArtifactViewer>::DIALECT, crate::editor::IFC4X3_ANY_DIALECT);
    }
}
