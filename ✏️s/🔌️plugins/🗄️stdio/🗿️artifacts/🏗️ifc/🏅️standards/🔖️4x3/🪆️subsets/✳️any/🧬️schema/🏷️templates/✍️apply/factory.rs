//! 🛠️ The addressed template action uses the native retained-command factory and publication declaration.
use super::*;
#[cfg(test)]
#[path = "chooser-tests.rs"]
mod chooser_tests;
use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_plugin::{
    ArtifactBoundedFirstStepProof, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane,
    InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory,
};

pub const PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: APPLY_TEMPLATE_ACTION, lanes: &[ArtifactToolPublicationLane::Artifact] }];
pub fn action() -> semio_plugin::ActionDefinition {
    use semio_plugin::{ActionArgDef, ActionArgOption, ActionDefinition, ActionKind};
    use semio_ui_locale::LocalizedLabel;
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
    ToolExecutionContract::bounded_first_step(TEMPLATE_MAXIMUM_RAW_BYTES, 4096, MAXIMUM_IDENTITIES as u64, TEMPLATE_MAXIMUM_RAW_BYTES, 7500)
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
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: TEMPLATE_MAXIMUM_RAW_BYTES, checkpoint_resume: false);
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
pub fn build(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Ifc4x3AnyEditor>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    if original.tool_id != APPLY_TEMPLATE_ACTION {
        return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Ifc4x3AnyEditor>(request, output, cx);
    }
    if !matches!(*original.command, Ifc4x3EditCommand::ApplyTemplate { .. }) {
        return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "Template tool received a different native command").into());
    }
    semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Ifc4x3AnyEditor::command_id, TEMPLATE_MAXIMUM_RAW_BYTES, TEMPLATE_WORK_CAPACITY.work_items(), false, |_| IfcTemplateWork::default())
}
