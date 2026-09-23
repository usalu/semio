//! ✏️ `svg` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! SVG has no pixel buffer: `set-pixel-region` parses the vector DSL and emits direct prolog, attribute, and child mutations rather than editing pixels.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::svg_any::modes::edit;
use crate::editor::svg_any::modes::edit::windows::main;
use crate::standards::v1_1::subsets::base::schema::mutations::{
    InsertElementMutation, InsertElementPayload, RemoveElementMutation, RemoveElementPayload, SetAttributeMutation, SetAttributePayload, SetDeclarationMutation, SetDeclarationPayload, SetDoctypeMutation, SetDoctypePayload, SetElementNameMutation,
    SetElementNamePayload, SvgMutation,
};
use crate::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
use crate::{STDIO_SVG_DOCUMENT_SCHEMA, SVG_ANY_DIALECT};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum SvgAnyEditCommand {
    SetPixelRegion { source: String },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
}

impl protocol::OpBinary for SvgAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_any-edit-command", offset: 0, detail: error.to_string() })?;
        pack::from_json_str(text).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_any-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.svg.tool-command.v1";
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn svgAnyEditor_example_snapshot(example_id: &str) -> SvgSnapshot {
    if example_id == crate::examples::demo::ID { <SvgSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { SvgSnapshot::default() }
}
fn svgAnyEditor_command_id(command: &SvgAnyEditCommand) -> &'static str {
    match command { SvgAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, _ => "other" }
}
fn svgAnyEditor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<SvgAnyEditCommand, Fault> {
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(SvgAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn svgAnyEditor_retained_extent(command: &SvgAnyEditCommand, _snapshot: &SvgSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, SvgAnyEditCommand::SetActiveExample { .. }).then_some(1)
}
fn svgAnyEditor_retained_reduce(command: &SvgAnyEditCommand, _snapshot: &SvgSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SvgAnyEditor>>>, _operation: &AppOperationContext) -> Result<Emit<SvgMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        SvgAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&svgAnyEditor_example_snapshot(example_id), STDIO_SVG_DOCUMENT_SCHEMA)], description: Some(format!("Load example {example_id}")), ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct SvgAnyEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl SvgAnyEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for SvgAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SvgAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SvgAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for SvgAnyEditorExampleFactory {
    type Owner = EditorApp<SvgAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<SvgAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.svg@1.1/*#editor",
        artifact_schema: "stdio.svg",
        factory: "SvgAnyEditorExampleFactory",
        factory_type: SvgAnyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(SvgAnyEditorExampleFactory::new(registry.controller_id()))
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if !STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if svgAnyEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, svgAnyEditor_command_id, STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, svgAnyEditor_retained_reduce, svgAnyEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_SVG_DOCUMENT_SCHEMA, operation, generation))
    }
    fn command_id(command: &Self::Command) -> &'static str { svgAnyEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { svgAnyEditor_command_from_action(action, args) }
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SvgAnyEditor;

impl ArtifactEditor for SvgAnyEditor {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgAnyEditCommand;

    const DIALECT: Dialect = SVG_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Self::Snapshot {
        SvgSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SvgAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&svgAnyEditor_example_snapshot(example_id), STDIO_SVG_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            }),
            SvgAnyEditCommand::SetPixelRegion { source } => {
                let Ok(snapshot) = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(source) else { return Ok(Emit::default()) };
                if snapshot.doc.prolog != doc.snapshot.doc.prolog {
                    return Ok(Emit::default());
                }
                let (Some(XmlNode::Element { name: current_name, attrs: current_attrs, children: current_children }), Some(XmlNode::Element { name, attrs, children })) = (&doc.snapshot.doc.root, &snapshot.doc.root) else {
                    return Ok(Emit::default());
                };
                let mut mutations = vec![
                    SvgMutation::SetDeclaration(SetDeclarationMutation::Apply(SetDeclarationPayload { declaration: snapshot.doc.declaration.clone() })),
                    SvgMutation::SetDoctype(SetDoctypeMutation::Apply(SetDoctypePayload { doctype: snapshot.doc.doctype.clone() })),
                ];
                if current_name != name {
                    mutations.push(SvgMutation::SetElementName(SetElementNameMutation::Apply(SetElementNamePayload { path: Vec::new(), name: name.clone() })));
                }
                mutations.extend(
                    current_attrs
                        .iter()
                        .filter(|current| !attrs.iter().any(|target| target.name == current.name))
                        .map(|current| SvgMutation::SetAttribute(SetAttributeMutation::Apply(SetAttributePayload { path: Vec::new(), name: current.name.clone(), value: None }))),
                );
                mutations.extend(attrs.iter().map(|attribute| SvgMutation::SetAttribute(SetAttributeMutation::Apply(SetAttributePayload { path: Vec::new(), name: attribute.name.clone(), value: Some(attribute.value.clone()) }))));
                mutations.extend((0..current_children.len()).rev().map(|index| SvgMutation::RemoveElement(RemoveElementMutation::Apply(RemoveElementPayload { parent: Vec::new(), index }))));
                mutations.extend(children.iter().cloned().enumerate().map(|(index, node)| SvgMutation::InsertElement(InsertElementMutation::Apply(InsertElementPayload { parent: Vec::new(), index, node }))));
                Ok(Emit::mutations(mutations))
            }
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_svg_any_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SVG_ANY_DIALECT).document(["semio", "svg"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
