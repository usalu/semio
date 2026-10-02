//! 🎚️ DAG mutation — `SetSlider`: the ABSOLUTE value, minimum or maximum of one slider node. A slider scrub's intent IS the
//! value it lands on, so one press commits one `set-slider` row and time travel edits that number.
use crate::diff::DagDiff;
use crate::mutations::{dag_label_number, DagMutation};
use crate::DagSnapshot;

//#region 🔖️Mutation
/// 🎛️ Which slider number a `set-slider` writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DagSliderField {
    #[default]
    Value,
    Min,
    Max,
}

impl DagSliderField {
    /// 🔤️ The wire spelling (`value` / `min` / `max`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Min => "min",
            Self::Max => "max",
        }
    }

    /// 🔎️ The field a wire spelling names, or nothing.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "value" => Some(Self::Value),
            "min" => Some(Self::Min),
            "max" => Some(Self::Max),
            _ => None,
        }
    }

    fn label(self) -> (&'static str, &'static str) {
        match self {
            Self::Value => ("value", "Wert"),
            Self::Min => ("minimum", "Minimum"),
            Self::Max => ("maximum", "Maximum"),
        }
    }
}

/// 🎚️ `set-slider` payload — FINAL-state absolute number of one slider field.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSlider {
    pub id: String,
    pub field: DagSliderField,
    pub value: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_slider(id: String, field: DagSliderField, value: f64) -> DagMutation {
    DagMutation::SetSlider(SetSlider { id, field, value })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for SetSlider {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "slider", kind: "set-slider", record: "SetSlider" };

    fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (value_en, value_de) = dag_label_number(self.value);
        let (field_en, field_de) = self.field.label();
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set slider \"{}\" {field_en} to {value_en}", self.id), &format!("{field_de} von Schieberegler \"{}\" auf {value_de} setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
