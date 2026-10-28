//! ✏️ IFC4x3 editor using the shared retained command, publication and details infrastructure.

use crate::templates::application::{work::factory as template_factory, APPLY_TEMPLATE_ACTION};
use semio_framework_2d_compute::EngineHandles;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, DraftView, Emit, Fault, MeshWindowKit, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, WindowKit};
use semio_framework_ui_locale::{Label, LocalizedLabel};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcSnapshot, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};

pub const IFC4X3_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4x3"), subset: SubsetId::ANY };
pub const IFC4X3_DOCUMENT_SCHEMA: &str = "s.stdio.ifc@4x3/*";
pub const IFC4X3_CONTROLLER: &str = "s.stdio.ifc@4x3/*#editor";

#[cfg(test)]
use crate::details::{encode_global_id, fresh_global_id, next_entity_id};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
pub enum Ifc4x3EditCommand {
    EditSnapshot { event: editing::SnapshotEditEvent },
    ApplyTemplate { application: DslValue },
}

impl protocol::OpBinary for Ifc4x3EditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &[
        editing::SET_SNAPSHOT_VALUE_ACTION_ID,
        editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
        editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
        editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
        editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
        editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
        APPLY_TEMPLATE_ACTION,
    ];
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "Ifc4x3EditCommand", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "Ifc4x3EditCommand", offset: 0, detail: error.to_string() })
    }
}

#[derive(Default, Clone, Copy)]
pub struct Ifc4x3AnyEditor;

#[path = "render-work.rs"]
pub mod render_work;
#[cfg(test)]
#[path = "render-work-tests.rs"]
mod render_work_tests;

impl ArtifactEditor for Ifc4x3AnyEditor {
    type Snapshot = IfcSnapshot;
    type Mutation = IfcMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Ifc4x3EditCommand;
    type RenderWork = render_work::Ifc4x3RenderWork;
    fn uses_render_work(body_key: &str)->bool{body_key==editing::SNAPSHOT_DETAILS_BODY_KEY||body_key==semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY}
    fn begin_render_work(body_key: &str,cx: &mut semio_framework_job::StepContext<'_>)->Result<Option<Self::RenderWork>,semio_framework_job::OriginalTurnFailure>{Self::RenderWork::begin(body_key,cx)}
    fn receive_render_work_request(work:&mut Self::RenderWork,request:&mut Option<semio_framework_plugin::action_staging::StagedFormRequest>,view:&semio_framework_plugin::ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{work.receive_request(request,view,cx)}
    fn render_work_demand(work:&Self::RenderWork,maximum:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{work.demand(maximum)}
    fn render_with_work_context(work:&mut Self::RenderWork,_owner:&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,body_key:&str,source:&std::sync::Arc<Self::Snapshot>,doc:&ArtifactView<'_,Self::Snapshot>,_cfg:&ConfigView<'_,Self::Config>,view_state:&semio_framework_plugin::ViewModel,_transient:&semio_framework_plugin::TransientView<'_,Self::Transient>,_interaction:&InteractionView<'_>,cx:&mut semio_framework_job::StepContext<'_>)->semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree>{work.render(body_key,source,doc,view_state,cx)}
    const DIALECT: Dialect = IFC4X3_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = IFC4X3_DOCUMENT_SCHEMA;

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = editing::snapshot_edit_bounded_first_step_proofs::<Self>("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/🦀️.rs", IFC4X3_CONTROLLER, IFC4X3_DOCUMENT_SCHEMA);
        proofs.push(template_factory::proof());
        proofs
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        editing::register_snapshot_edit_tool_factory::<Self>(registry)?;
        template_factory::register(registry)
    }
    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        template_factory::build(request)
    }
    semio_s_artifact_stdio_contract::snapshot_editing_publication_authority!();

    fn command_id(command: &Self::Command) -> &'static str {
        match command {
            Ifc4x3EditCommand::EditSnapshot { event } => event.action_id(),
            Ifc4x3EditCommand::ApplyTemplate { .. } => APPLY_TEMPLATE_ACTION,
        }
    }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() {
            return Err(fault);
        }
        if action == APPLY_TEMPLATE_ACTION {
            return args.take().map(|application| Ifc4x3EditCommand::ApplyTemplate { application }).ok_or_else(|| Fault::from("Template application payload is missing"));
        }
        editing::snapshot_edit_event_from_action(action, args.as_ref())?.map(|event| Ifc4x3EditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("unsupported IFC4x3 action '{action}'")))
    }
    fn initial_snapshot() -> IfcSnapshot {
        let mut snapshot = IfcSnapshot::default();
        snapshot.header.file_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])];
        snapshot
    }
    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            Ifc4x3EditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            Ifc4x3EditCommand::ApplyTemplate { .. } => Err(Fault::from("Template applications require their retained native tool job")),
        }
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
        if body_key == MeshWindowKit::KIND_ID {
            return semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::geometry::render_in_context(doc.snapshot, cx).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into);
        }
        Self::render(body_key, doc, cfg, view_state).map(Some).map_err(Into::into)
    }
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        if body_key == MeshWindowKit::KIND_ID {
            return semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::geometry::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree);
        }
        if body_key != editing::SNAPSHOT_DETAILS_BODY_KEY {
            return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}")));
        }
        let revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
        editing::render_snapshot_details_provider_revisioned(
            &crate::details::Ifc4x3DetailsProvider::new(doc.snapshot),
            revision,
            view_state.locale,
            IFC4X3_CONTROLLER,
            &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY),
        )
        .map(semio_framework_plugin::built_to_component_tree)
    }
}

impl editing::SnapshotEditingEditor for Ifc4x3AnyEditor {
    fn snapshot_edit_work_capacity() -> semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity {
        crate::details::details_work_capacity()
    }
    fn snapshot_edit_work(tool: &'static str) -> Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> {
        crate::details::details_edit_work::<Self>(tool)
    }
    fn snapshot_edit_is_admitted(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot) -> bool {
        crate::details::details_edit_is_admitted(event, snapshot)
    }
    fn snapshot_edit_retained_extent(_: &editing::SnapshotEditEvent,_:&IfcSnapshot)->Option<usize>{Some(Self::snapshot_edit_work_capacity().work_items())}
    fn snapshot_edit_retained_path_is_admitted(event:&editing::SnapshotEditEvent,snapshot:&IfcSnapshot)->bool{crate::details::details_edit_retained_path_is_admitted(event,snapshot)}
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command {
            Ifc4x3EditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }
    fn snapshot_edit_command_from_original(event:&mut Option<editing::SnapshotEditEvent>)->Option<Self::Command>{event.take().map(|event|Ifc4x3EditCommand::EditSnapshot{event})}
    fn snapshot_edit_event_mut(command: &mut Self::Command) -> Option<&mut editing::SnapshotEditEvent> {
        match command {
            Ifc4x3EditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot) -> Result<Option<Vec<IfcMutation>>, Fault> {
        crate::details::details_edit_special(event, snapshot)
    }
    fn snapshot_edit_emit_in_context(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Emit<IfcMutation, NoConfigMutation, NoDraftMutation>, Fault> {
        if matches!(event, editing::SnapshotEditEvent::ReplaceSource { .. }) {
            return Self::snapshot_edit_emit(event, snapshot);
        }
        let artifact_mutations = crate::details::details_edit_special_controlled(event, snapshot, &mut || {
            if cx.is_cancelled() {
                return Err("details-cancelled".into());
            }
            if cx.should_yield() {
                return Err("details-work-limit".into());
            }
            cx.consume_fuel(1);
            Ok(())
        })?
        .unwrap_or_default();
        Ok(Emit { artifact_mutations, ..Default::default() })
    }
}

pub fn create_ifc4x3_any_editor() -> semio_framework_plugin::AppDefinition {
    let mode = semio_framework_plugin::ModeDefinition { id: "edit".into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() };
    let layout = editing::snapshot_details_split_layout(MeshWindowKit::KIND_ID, "Main");
    editing::snapshot_edit_actions_with(
        semio_framework_plugin::Editor::builder(IFC4X3_ANY_DIALECT)
            .document(["stdio", "ifc4x3"])
            .icon_id("box")
            .mode_def(mode)
            .default_mode_id("edit")
            .window_kind_def(MeshWindowKit::window_kind())
            .window_kind_def(editing::snapshot_details_window_definition())
            .default_layout(layout),
    )
    .action_with(template_factory::action())
    .action_interactive_job(APPLY_TEMPLATE_ACTION, semio_framework_plugin::InteractiveJobClassification::Migrated)
    .build_definition()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
