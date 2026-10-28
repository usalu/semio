//! 🛠️ The addressed template action uses the native retained-command factory and publication declaration.
use super::*;
#[cfg(test)]
#[path = "chooser-tests.rs"]
mod chooser_tests;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    AppOperationContext, ArtifactBoundedFirstStepProof, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane,
    InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};

pub const PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: APPLY_TEMPLATE_ACTION, lanes: &[ArtifactToolPublicationLane::Artifact] }];
pub fn action() -> semio_framework_plugin::ActionDefinition {
    use semio_framework_plugin::{ActionArgDef, ActionArgOption, ActionDefinition, ActionKind};
    use semio_framework_ui_locale::LocalizedLabel;
    ActionDefinition::bounded_catalog(APPLY_TEMPLATE_ACTION, LocalizedLabel::native("Apply Released Template", "Veröffentlichtes Template anwenden"), ActionKind::Mutation).with_args(vec![
        ActionArgDef::select("template", LocalizedLabel::native("Template", "Template"), crate::templates::ifc4x3_template_catalog().templates.iter().map(|template| ActionArgOption::new(template.name, LocalizedLabel::data(template.name))).collect()).required(),
        ActionArgDef::text("target", LocalizedLabel::native("Target Entity", "Zielelement")).required(),
        ActionArgDef::any("entries", LocalizedLabel::native("Selected Entries", "Ausgewählte Einträge")).required(),
        ActionArgDef::any("identities", LocalizedLabel::native("Retained Identities", "Gespeicherte Identitäten")).required(),
        ActionArgDef::any("ownerHistory", LocalizedLabel::native("Owner History", "Eigentümerhistorie")),
        ActionArgDef::text("description", LocalizedLabel::native("Description", "Beschreibung")),
        ActionArgDef::text("methodOfMeasurement", LocalizedLabel::native("Method of Measurement", "Messmethode")),
        ActionArgDef::any("performance", LocalizedLabel::native("Performance History", "Leistungshistorie")),
    ])
}
pub struct IfcTemplateToolJobFactory {
    keys: Vec<ToolFactoryKey>,
}
impl IfcTemplateToolJobFactory {
    fn new(controller: &str) -> Self {
        Self { keys: vec![ToolFactoryKey::new(controller, APPLY_TEMPLATE_ACTION)] }
    }
}
pub fn execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(TEMPLATE_MAXIMUM_RAW_BYTES, 4096, MAXIMUM_IDENTITIES, TEMPLATE_MAXIMUM_RAW_BYTES, 7500)
}
impl ToolJobFactory for IfcTemplateToolJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Ifc4x3AnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Ifc4x3AnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        execution_contract()
    }
    fn create_job(&mut self, _: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > TEMPLATE_MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Template application rejects oversized wire or a checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for IfcTemplateToolJobFactory {
    type Owner = EditorApp<Ifc4x3AnyEditor>;
    const TOOL_IDS: &'static [&'static str] = &[APPLY_TEMPLATE_ACTION];
    const DOCUMENT_SCHEMA: &'static str = crate::editor::IFC4X3_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = PUBLICATION_CONTRACTS;
}
pub fn register(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Ifc4x3AnyEditor>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(IfcTemplateToolJobFactory::new(&controller))
}
pub fn proof() -> ArtifactBoundedFirstStepProof {
    ArtifactBoundedFirstStepProof::new::<EditorApp<Ifc4x3AnyEditor>>(
        "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/factory.rs",
        crate::editor::IFC4X3_CONTROLLER,
        "IfcTemplateToolJobFactory",
        APPLY_TEMPLATE_ACTION,
        crate::editor::IFC4X3_DOCUMENT_SCHEMA,
        execution_contract(),
    )
    .with_factory_type::<EditorApp<Ifc4x3AnyEditor>, IfcTemplateToolJobFactory>()
}
pub fn build(request: ArtifactOwnedToolJobRequest<EditorApp<Ifc4x3AnyEditor>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if request.tool_id != APPLY_TEMPLATE_ACTION {
        return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Ifc4x3AnyEditor>(request);
    }
    if !matches!(*request.command, Ifc4x3EditCommand::ApplyTemplate { .. }) {
        return Err(Fault::from("Template tool received a different native command"));
    }
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        retained: request.retained,
        authoring_seed: request.authoring_seed,
    };
    let payload = ArtifactRetainedCommandPayload::new(
        ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            snapshot_read: request.snapshot_read,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        Ifc4x3AnyEditor::command_id,
        TEMPLATE_MAXIMUM_RAW_BYTES,
        TEMPLATE_WORK_CAPACITY.work_items(),
        Box::new(IfcTemplateWork::default()),
    );
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
