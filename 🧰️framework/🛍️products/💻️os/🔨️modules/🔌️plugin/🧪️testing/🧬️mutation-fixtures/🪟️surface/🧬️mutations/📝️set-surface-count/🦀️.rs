use super::super::{SurfaceDiff, SurfaceMutation, SurfaceSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetSurfaceCount {
    pub value: i32,
}
impl MutationKind<SurfaceSnapshot, SurfaceMutation> for SetSurfaceCount {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "surface-count", kind: "set-surface-count", record: "SetSurfaceCount" };


    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Set surface count to {}", self.value), &format!("Oberflächenanzahl auf {} setzen", self.value))
    }
}

impl semio_os_command::ColdMutationKind<SurfaceSnapshot, SurfaceMutation> for SetSurfaceCount {
    fn diff(&self, _: &SurfaceSnapshot) -> MutationOutcome<SurfaceDiff> {
        MutationOutcome::new(SurfaceDiff { count: Some(self.value) })
    }
    fn inverse(&self, base: &SurfaceSnapshot) -> Result<Vec<SurfaceMutation>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { value: base.count }.into()])())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧪️sum-law/🦀️.rs"]
mod sum_law_tests;
