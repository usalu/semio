//! 🎛️ Trinity Rewriting app — Parameters window (editable form over the RHS's declared parameters).

use crate::artifacts::jack::PropertyValue;
use crate::artifacts::rewriting::schema::{ParameterKind, Rhs};
use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::terminology::TrinityRewritingLabels;
use semio_framework_plugin::Label;
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren, InputKind, Trigger};

trait ParameterKindLabel {
    fn kind_label(&self) -> String;
}

impl ParameterKindLabel for crate::artifacts::rewriting::schema::ParameterSpec {
    fn kind_label(&self) -> String {
        match self.kind {
            ParameterKind::String => "string".into(),
            ParameterKind::Number => "number".into(),
            ParameterKind::Boolean => "boolean".into(),
        }
    }
}

pub(crate) fn render(state: &RewritingSnapshot, labels: &TrinityRewritingLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Ok(rhs) = pack::from_json_str::<Rhs>(&state.rhs_json) else {
        return semio_framework_plugin::built_text_node(Label::data("Invalid RHS")).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.parameters.invalid", "the fixed invalid-RHS label exceeds its UI bound"));
    };
    let mut children = semio_framework_plugin::UiFixedList::default();
    for param in &rhs.parameters {
        let value = state.parameter_bindings.get(&param.name).cloned().unwrap_or_else(|| param.default.clone());
        let display = match value {
            PropertyValue::String(text) => text,
            PropertyValue::Number(number) => number.to_string(),
            PropertyValue::Bool(flag) => flag.to_string(),
            _ => String::new(),
        };
        let input_kind = match param.kind {
            ParameterKind::Number => InputKind::Number,
            ParameterKind::Boolean | ParameterKind::String => InputKind::Text,
        };
        let input_id = format!("trinity-rewriting.param.{}.input", param.name);
        let input_value = semio_framework_plugin::UiText::try_from_string(display).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter value admission failed"))?;
        let placeholder = crate::editor::rewriting::ui_label(param.kind_label())?;
        let args = crate::editor::rewriting::ui_value_map([("name", crate::editor::rewriting::ui_value_text(&param.name)?)])?;
        let (action, args) = crate::editor::rewriting::rewriting_action("setParameter", Some(args))?;
        let input =
            semio_framework_ui_contract::input(input_kind).value(input_value).placeholder(placeholder).try_id(&input_id).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter input id admission failed"))?;
        let input = match args {
            Some(args) => input.try_on_with(Trigger::Change, action, args),
            None => input.try_on(Trigger::Change, action),
        }
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter input action admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter input admission failed"))?;
        let field_id = format!("trinity-rewriting.param.{}", param.name);
        let field = semio_framework_ui_contract::field(crate::editor::rewriting::ui_label(&param.name)?)
            .try_id(&field_id)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter field id admission failed"))?
            .try_child(input)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter field child admission failed"))?
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter field admission failed"))?;
        children.try_push(field).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter field list admission failed"))?;
    }
    if children.is_empty() {
        return semio_framework_plugin::built_text_node(Label::data("No parameters declared on RHS.")).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.parameters.empty", "the fixed empty-parameters label exceeds its UI bound"));
    }
    semio_framework_ui_contract::column()
        .try_id("trinity-rewriting.parameters")
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter column id admission failed"))?
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter column children admission failed"))?
        .accessibility_label(crate::editor::rewriting::ui_label(labels.parameters.as_str())?)
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "parameter column admission failed"))
}
