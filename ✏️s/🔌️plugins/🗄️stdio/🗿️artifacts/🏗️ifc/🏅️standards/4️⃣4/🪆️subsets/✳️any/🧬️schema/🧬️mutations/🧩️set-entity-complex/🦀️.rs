//! 🧩️ Replaces the additional type constituents of one IFC4 entity instance.

use super::*;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityComplex {
    pub id: u64,
    pub complex: Vec<IfcComplexType>,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetEntityComplex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-complex", kind: "set-entity-complex", record: "SetEntityComplex" };

    fn diff(&self, _base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
        protocol::MutationOutcome::new(diff::diff_set_entity_complex(self.id, self.complex.clone()))
    }

    fn inverse(&self, base: &IfcSnapshot) -> Result<Vec<IfcMutation>, semio_framework_value::ValueError> {
        Ok(base.entities.iter().find(|entity| entity.id == self.id).map(|entity| IfcMutation::SetEntityComplex(Self { id: self.id, complex: entity.complex.clone() })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set complex constituents", "Komplexe Bestandteile setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["entities".into(), self.id.to_string(), "complex".into()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔁️replaces/🦀️.rs"]
mod tests;
