use crate::test_app_mutation_fixture::config::{TestConfig, TestConfigDiff};
use crate::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use protocol::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
fn required_nullable<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    <Option<String> as serde::Deserialize>::deserialize(deserializer)
}
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = store)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeTestConfigSelection {
    #[serde(deserialize_with = "required_nullable")]
    pub selected: Option<String>,
}
semio_framework_value::artifact_retire_struct!(ChangeTestConfigSelection { selected });
impl ChangeTestConfigSelection {
    pub const OPCODE: &'static str = "change-test-config-selection";
}
impl OpText for ChangeTestConfigSelection {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let value = line
            .strip_prefix("change-test-config-selection ")
            .ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected change-test-config-selection", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(Self {
            selected: serde_json::from_str(value)
                .map_err(|_| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "selection must be a JSON nullable string", semio_framework_diagnostic::TextSpan::at(1, 1)))?,
        })
    }
    fn print_op(&self) -> String {
        format!("{} {}", Self::OPCODE, serde_json::to_string(&self.selected).expect("nullable string serializes"))
    }
}
impl MutationKind<TestConfig, TestConfigMutation> for ChangeTestConfigSelection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "test-config-selection", kind: "change-test-config-selection", record: "ChangedTestConfigSelection" };
    fn diff(&self, _: &TestConfig) -> MutationOutcome<TestConfigDiff> {
        MutationOutcome::new(match &self.selected {
            None => TestConfigDiff::Clear,
            Some(value) => TestConfigDiff::Set(value.clone()),
        })
    }
    fn inverse(&self, base: &TestConfig) -> Result<Vec<TestConfigMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { selected: base.selected.clone() }.into()])())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &{
                match &self.selected {
                    None => "Clear test config selection".into(),
                    Some(value) => format!("Change test config selection to {value}"),
                }
            },
            &{
                match &self.selected {
                    None => "Testkonfigurationsauswahl leeren".into(),
                    Some(value) => format!("Testkonfigurationsauswahl auf {value} ändern"),
                }
            },
        )
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../🧪️tests/🎚️test-app-config-selection-unit/🦀️.rs"]
pub mod tests;
